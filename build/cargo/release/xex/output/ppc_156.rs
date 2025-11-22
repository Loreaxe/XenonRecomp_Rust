pub fn sub_82D61FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D61FA8 size=72
    let mut pc: u32 = 0x82D61FA8;
    'dispatch: loop {
        match pc {
            0x82D61FA8 => {
    //   block [0x82D61FA8..0x82D61FF0)
	// 82D61FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D61FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D61FB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D61FB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D61FB8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D61FBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D61FC0: 396B5124  addi r11, r11, 0x5124
	ctx.r[11].s64 = ctx.r[11].s64 + 20772;
	// 82D61FC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D61FC8: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82D61FCC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D61FD0: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D61FD4: 4B504F6D  bl 0x82266f40
	ctx.lr = 0x82D61FD8;
	sub_82266F40(ctx, base);
	// 82D61FD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D61FDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D61FE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D61FE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D61FE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D61FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D61FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D61FF0 size=128
    let mut pc: u32 = 0x82D61FF0;
    'dispatch: loop {
        match pc {
            0x82D61FF0 => {
    //   block [0x82D61FF0..0x82D62070)
	// 82D61FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D61FF4: 4BF47419  bl 0x82ca940c
	ctx.lr = 0x82D61FF8;
	sub_82CA93D0(ctx, base);
	// 82D61FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D61FFC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D62000: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D62004: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 82D62008: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82D6200C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D62010: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D62014: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D62018: 4BFFFF01  bl 0x82d61f18
	ctx.lr = 0x82D6201C;
	sub_82D61F18(ctx, base);
	// 82D6201C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62020: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D62024: 409A0040  bne cr6, 0x82d62064
	if !ctx.cr[6].eq {
	pc = 0x82D62064; continue 'dispatch;
	}
	// 82D62028: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D6202C: 4BFF372D  bl 0x82d55758
	ctx.lr = 0x82D62030;
	sub_82D55758(ctx, base);
	// 82D62030: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D62034: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82D62038: 409AFFD0  bne cr6, 0x82d62008
	if !ctx.cr[6].eq {
	pc = 0x82D62008; continue 'dispatch;
	}
	// 82D6203C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D62040: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D62044: 4BFF3AAD  bl 0x82d55af0
	ctx.lr = 0x82D62048;
	sub_82D55AF0(ctx, base);
	// 82D62048: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6204C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82D62050: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82D62054: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82D62058: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 82D6205C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D62060: 4BF473FC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82D62064: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D62068: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D6206C: 4BF473F0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D62070 size=104
    let mut pc: u32 = 0x82D62070;
    'dispatch: loop {
        match pc {
            0x82D62070 => {
    //   block [0x82D62070..0x82D620D8)
	// 82D62070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D62074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D62078: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D6207C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D62080: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62084: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D62088: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 82D6208C: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82D62090: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D62094: 4BFF31B5  bl 0x82d55248
	ctx.lr = 0x82D62098;
	sub_82D55248(ctx, base);
	// 82D62098: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D6209C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D620A0: 396B5124  addi r11, r11, 0x5124
	ctx.r[11].s64 = ctx.r[11].s64 + 20772;
	// 82D620A4: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82D620A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82D620AC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82D620B0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D620B4: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82D620B8: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82D620BC: 4B504E85  bl 0x82266f40
	ctx.lr = 0x82D620C0;
	sub_82266F40(ctx, base);
	// 82D620C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D620C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D620C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D620CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D620D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D620D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D620D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D620D8 size=104
    let mut pc: u32 = 0x82D620D8;
    'dispatch: loop {
        match pc {
            0x82D620D8 => {
    //   block [0x82D620D8..0x82D62140)
	// 82D620D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D620DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D620E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D620E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D620E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D620EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D620F0: 3BDF0008  addi r30, r31, 8
	ctx.r[30].s64 = ctx.r[31].s64 + 8;
	// 82D620F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D620F8: 4BFFE539  bl 0x82d60630
	ctx.lr = 0x82D620FC;
	sub_82D60630(ctx, base);
	// 82D620FC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D62100: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82D62104: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D62108: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D6210C: 40990008  ble cr6, 0x82d62114
	if !ctx.cr[6].gt {
	pc = 0x82D62114; continue 'dispatch;
	}
	// 82D62110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D62114: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82D62118: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D6211C: 419A000C  beq cr6, 0x82d62128
	if ctx.cr[6].eq {
	pc = 0x82D62128; continue 'dispatch;
	}
	// 82D62120: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D62124: 4BFFE655  bl 0x82d60778
	ctx.lr = 0x82D62128;
	sub_82D60778(ctx, base);
	// 82D62128: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D6212C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D62130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D62134: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D62138: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D6213C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D62140 size=124
    let mut pc: u32 = 0x82D62140;
    'dispatch: loop {
        match pc {
            0x82D62140 => {
    //   block [0x82D62140..0x82D621BC)
	// 82D62140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D62144: 4BF472C5  bl 0x82ca9408
	ctx.lr = 0x82D62148;
	sub_82CA93D0(ctx, base);
	// 82D62148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6214C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D62150: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82D62154: 3BFE0008  addi r31, r30, 8
	ctx.r[31].s64 = ctx.r[30].s64 + 8;
	// 82D62158: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82D6215C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D62160: 4BFFE4D1  bl 0x82d60630
	ctx.lr = 0x82D62164;
	sub_82D60630(ctx, base);
	// 82D62164: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D62168: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D6216C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D62170: 40990008  ble cr6, 0x82d62178
	if !ctx.cr[6].gt {
	pc = 0x82D62178; continue 'dispatch;
	}
	// 82D62174: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D62178: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82D6217C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D62180: 419A0024  beq cr6, 0x82d621a4
	if ctx.cr[6].eq {
	pc = 0x82D621A4; continue 'dispatch;
	}
	// 82D62184: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D62188: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6218C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82D62190: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D62194: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D62198: 7F8B512E  stwx r28, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u32) };
	// 82D6219C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D621A0: 4BF472B8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82D621A4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82D621A8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D621AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D621B0: 4B503A51  bl 0x82265c00
	ctx.lr = 0x82D621B4;
	sub_82265C00(ctx, base);
	// 82D621B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D621B8: 4BF472A0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D621C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D621C0 size=140
    let mut pc: u32 = 0x82D621C0;
    'dispatch: loop {
        match pc {
            0x82D621C0 => {
    //   block [0x82D621C0..0x82D6224C)
	// 82D621C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D621C4: 4BF47245  bl 0x82ca9408
	ctx.lr = 0x82D621C8;
	sub_82CA93D0(ctx, base);
	// 82D621C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D621CC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D621D0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D621D4: 3BFE0008  addi r31, r30, 8
	ctx.r[31].s64 = ctx.r[30].s64 + 8;
	// 82D621D8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82D621DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D621E0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82D621E4: 4BFFE44D  bl 0x82d60630
	ctx.lr = 0x82D621E8;
	sub_82D60630(ctx, base);
	// 82D621E8: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D621EC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D621F0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D621F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D621F8: 40990008  ble cr6, 0x82d62200
	if !ctx.cr[6].gt {
	pc = 0x82D62200; continue 'dispatch;
	}
	// 82D621FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D62200: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82D62204: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D62208: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D6220C: 419A0030  beq cr6, 0x82d6223c
	if ctx.cr[6].eq {
	pc = 0x82D6223C; continue 'dispatch;
	}
	// 82D62210: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D62214: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62218: 995D0000  stb r10, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82D6221C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D62220: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82D62224: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D62228: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D6222C: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82D62230: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D62234: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D62238: 4BF47220  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82D6223C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D62240: 997D0000  stb r11, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D62244: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D62248: 4BF47210  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D62250 size=144
    let mut pc: u32 = 0x82D62250;
    'dispatch: loop {
        match pc {
            0x82D62250 => {
    //   block [0x82D62250..0x82D622E0)
	// 82D62250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D62254: 4BF471B5  bl 0x82ca9408
	ctx.lr = 0x82D62258;
	sub_82CA93D0(ctx, base);
	// 82D62258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6225C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D62260: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82D62264: 3BDD0008  addi r30, r29, 8
	ctx.r[30].s64 = ctx.r[29].s64 + 8;
	// 82D62268: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82D6226C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D62270: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D62274: 4BFFE3BD  bl 0x82d60630
	ctx.lr = 0x82D62278;
	sub_82D60630(ctx, base);
	// 82D62278: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6227C: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D62280: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D62284: 40990008  ble cr6, 0x82d6228c
	if !ctx.cr[6].gt {
	pc = 0x82D6228C; continue 'dispatch;
	}
	// 82D62288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D6228C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82D62290: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D62294: 409A002C  bne cr6, 0x82d622c0
	if !ctx.cr[6].eq {
	pc = 0x82D622C0; continue 'dispatch;
	}
	// 82D62298: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D6229C: 4BFF34BD  bl 0x82d55758
	ctx.lr = 0x82D622A0;
	sub_82D55758(ctx, base);
	// 82D622A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D622A4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D622A8: 409AFFC4  bne cr6, 0x82d6226c
	if !ctx.cr[6].eq {
	pc = 0x82D6226C; continue 'dispatch;
	}
	// 82D622AC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82D622B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D622B4: 4BFFFD3D  bl 0x82d61ff0
	ctx.lr = 0x82D622B8;
	sub_82D61FF0(ctx, base);
	// 82D622B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D622BC: 4BF4719C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82D622C0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D622C4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D622C8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82D622CC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D622D0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D622D4: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D622D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D622DC: 4BF4717C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D622E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D622E0 size=116
    let mut pc: u32 = 0x82D622E0;
    'dispatch: loop {
        match pc {
            0x82D622E0 => {
    //   block [0x82D622E0..0x82D62354)
	// 82D622E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D622E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D622E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D622EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D622F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D622F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D622F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D622FC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82D62300: 4B504C09  bl 0x82266f08
	ctx.lr = 0x82D62304;
	sub_82266F08(ctx, base);
	// 82D62304: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D62308: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D6230C: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D62310: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D62314: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D62318: 419A0020  beq cr6, 0x82d62338
	if ctx.cr[6].eq {
	pc = 0x82D62338; continue 'dispatch;
	}
	// 82D6231C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62320: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D62324: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82D62328: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6232C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D62330: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D62334: 4BFF2F95  bl 0x82d552c8
	ctx.lr = 0x82D62338;
	sub_82D552C8(ctx, base);
	// 82D62338: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D6233C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D62340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D62344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D62348: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D6234C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D62350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D62358 size=20
    let mut pc: u32 = 0x82D62358;
    'dispatch: loop {
        match pc {
            0x82D62358 => {
    //   block [0x82D62358..0x82D6236C)
	// 82D62358: A1650012  lhz r11, 0x12(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(18 as u32) ) } as u64;
	// 82D6235C: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82D62360: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82D62364: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D62368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D62370 size=96
    let mut pc: u32 = 0x82D62370;
    'dispatch: loop {
        match pc {
            0x82D62370 => {
    //   block [0x82D62370..0x82D623D0)
	// 82D62370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D62374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D62378: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D6237C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D62380: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D62384: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D62388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D6238C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D62390: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82D62394: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82D62398: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D6239C: 4BFF367D  bl 0x82d55a18
	ctx.lr = 0x82D623A0;
	sub_82D55A18(ctx, base);
	// 82D623A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D623A4: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82D623A8: 419A0010  beq cr6, 0x82d623b8
	if ctx.cr[6].eq {
	pc = 0x82D623B8; continue 'dispatch;
	}
	// 82D623AC: A1630012  lhz r11, 0x12(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(18 as u32) ) } as u64;
	// 82D623B0: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82D623B4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D623B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D623BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D623C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D623C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D623C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D623CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D623D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D623D0 size=20
    let mut pc: u32 = 0x82D623D0;
    'dispatch: loop {
        match pc {
            0x82D623D0 => {
    //   block [0x82D623D0..0x82D623E4)
	// 82D623D0: A1650012  lhz r11, 0x12(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(18 as u32) ) } as u64;
	// 82D623D4: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82D623D8: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82D623DC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D623E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D623E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D623E8 size=104
    let mut pc: u32 = 0x82D623E8;
    'dispatch: loop {
        match pc {
            0x82D623E8 => {
    //   block [0x82D623E8..0x82D62450)
	// 82D623E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D623EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D623F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D623F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D623F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D623FC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82D62400: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D62404: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D62408: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82D6240C: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D62410: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62414: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D62418: 4BFF3601  bl 0x82d55a18
	ctx.lr = 0x82D6241C;
	sub_82D55A18(ctx, base);
	// 82D6241C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D62420: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82D62424: 419A0010  beq cr6, 0x82d62434
	if ctx.cr[6].eq {
	pc = 0x82D62434; continue 'dispatch;
	}
	// 82D62428: A1630012  lhz r11, 0x12(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(18 as u32) ) } as u64;
	// 82D6242C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82D62430: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D62434: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D62438: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D6243C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D62440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D62444: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D62448: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D6244C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D62450 size=100
    let mut pc: u32 = 0x82D62450;
    'dispatch: loop {
        match pc {
            0x82D62450 => {
    //   block [0x82D62450..0x82D624B4)
	// 82D62450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D62454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D62458: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D6245C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D62460: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D62464: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D62468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D6246C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D62470: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82D62474: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82D62478: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D6247C: 4BFF359D  bl 0x82d55a18
	ctx.lr = 0x82D62480;
	sub_82D55A18(ctx, base);
	// 82D62480: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D62484: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82D62488: 419A0010  beq cr6, 0x82d62498
	if ctx.cr[6].eq {
	pc = 0x82D62498; continue 'dispatch;
	}
	// 82D6248C: A1630012  lhz r11, 0x12(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(18 as u32) ) } as u64;
	// 82D62490: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82D62494: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D62498: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D6249C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D624A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D624A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D624A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D624AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D624B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D624B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D624B8 size=24
    let mut pc: u32 = 0x82D624B8;
    'dispatch: loop {
        match pc {
            0x82D624B8 => {
    //   block [0x82D624B8..0x82D624D0)
	// 82D624B8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D624BC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82D624C0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82D624C4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82D624C8: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D624CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D624D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D624D0 size=8
    let mut pc: u32 = 0x82D624D0;
    'dispatch: loop {
        match pc {
            0x82D624D0 => {
    //   block [0x82D624D0..0x82D624D8)
	// 82D624D0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D624D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D624D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D624D8 size=8
    let mut pc: u32 = 0x82D624D8;
    'dispatch: loop {
        match pc {
            0x82D624D8 => {
    //   block [0x82D624D8..0x82D624E0)
	// 82D624D8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D624DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D624E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D624E0 size=16
    let mut pc: u32 = 0x82D624E0;
    'dispatch: loop {
        match pc {
            0x82D624E0 => {
    //   block [0x82D624E0..0x82D624F0)
	// 82D624E0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D624E4: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D624E8: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D624EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D624F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D624F0 size=16
    let mut pc: u32 = 0x82D624F0;
    'dispatch: loop {
        match pc {
            0x82D624F0 => {
    //   block [0x82D624F0..0x82D62500)
	// 82D624F0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D624F4: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D624F8: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D624FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D62500 size=12
    let mut pc: u32 = 0x82D62500;
    'dispatch: loop {
        match pc {
            0x82D62500 => {
    //   block [0x82D62500..0x82D6250C)
	// 82D62500: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62504: 7C6B2214  add r3, r11, r4
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82D62508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D62510 size=16
    let mut pc: u32 = 0x82D62510;
    'dispatch: loop {
        match pc {
            0x82D62510 => {
    //   block [0x82D62510..0x82D62520)
	// 82D62510: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62514: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D62518: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D6251C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D62520 size=16
    let mut pc: u32 = 0x82D62520;
    'dispatch: loop {
        match pc {
            0x82D62520 => {
    //   block [0x82D62520..0x82D62530)
	// 82D62520: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62524: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D62528: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D6252C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D62530 size=16
    let mut pc: u32 = 0x82D62530;
    'dispatch: loop {
        match pc {
            0x82D62530 => {
    //   block [0x82D62530..0x82D62540)
	// 82D62530: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62534: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D62538: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D6253C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D62540 size=16
    let mut pc: u32 = 0x82D62540;
    'dispatch: loop {
        match pc {
            0x82D62540 => {
    //   block [0x82D62540..0x82D62550)
	// 82D62540: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62544: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D62548: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D6254C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D62550 size=16
    let mut pc: u32 = 0x82D62550;
    'dispatch: loop {
        match pc {
            0x82D62550 => {
    //   block [0x82D62550..0x82D62560)
	// 82D62550: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62554: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D62558: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D6255C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D62560 size=16
    let mut pc: u32 = 0x82D62560;
    'dispatch: loop {
        match pc {
            0x82D62560 => {
    //   block [0x82D62560..0x82D62570)
	// 82D62560: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62564: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D62568: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D6256C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D62570 size=12
    let mut pc: u32 = 0x82D62570;
    'dispatch: loop {
        match pc {
            0x82D62570 => {
    //   block [0x82D62570..0x82D6257C)
	// 82D62570: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62574: 7C6B2214  add r3, r11, r4
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82D62578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D62580 size=12
    let mut pc: u32 = 0x82D62580;
    'dispatch: loop {
        match pc {
            0x82D62580 => {
    //   block [0x82D62580..0x82D6258C)
	// 82D62580: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62584: 7C6B2214  add r3, r11, r4
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82D62588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D62590 size=16
    let mut pc: u32 = 0x82D62590;
    'dispatch: loop {
        match pc {
            0x82D62590 => {
    //   block [0x82D62590..0x82D625A0)
	// 82D62590: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62594: 548B2036  slwi r11, r4, 4
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D62598: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D6259C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D625A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D625A0 size=24
    let mut pc: u32 = 0x82D625A0;
    'dispatch: loop {
        match pc {
            0x82D625A0 => {
    //   block [0x82D625A0..0x82D625B8)
	// 82D625A0: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D625A4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D625A8: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82D625AC: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D625B0: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D625B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D625B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D625B8 size=16
    let mut pc: u32 = 0x82D625B8;
    'dispatch: loop {
        match pc {
            0x82D625B8 => {
    //   block [0x82D625B8..0x82D625C8)
	// 82D625B8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D625BC: 548B3032  slwi r11, r4, 6
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D625C0: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D625C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D625C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D625C8 size=24
    let mut pc: u32 = 0x82D625C8;
    'dispatch: loop {
        match pc {
            0x82D625C8 => {
    //   block [0x82D625C8..0x82D625E0)
	// 82D625C8: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D625CC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D625D0: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82D625D4: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D625D8: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D625DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D625E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D625E0 size=16
    let mut pc: u32 = 0x82D625E0;
    'dispatch: loop {
        match pc {
            0x82D625E0 => {
    //   block [0x82D625E0..0x82D625F0)
	// 82D625E0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D625E4: 548B1838  slwi r11, r4, 3
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D625E8: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D625EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D625F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D625F0 size=24
    let mut pc: u32 = 0x82D625F0;
    'dispatch: loop {
        match pc {
            0x82D625F0 => {
    //   block [0x82D625F0..0x82D62608)
	// 82D625F0: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D625F4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D625F8: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82D625FC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D62600: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D62604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D62608 size=16
    let mut pc: u32 = 0x82D62608;
    'dispatch: loop {
        match pc {
            0x82D62608 => {
    //   block [0x82D62608..0x82D62618)
	// 82D62608: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6260C: 548B1838  slwi r11, r4, 3
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D62610: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D62614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D62618 size=8
    let mut pc: u32 = 0x82D62618;
    'dispatch: loop {
        match pc {
            0x82D62618 => {
    //   block [0x82D62618..0x82D62620)
	// 82D62618: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6261C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D62620 size=88
    let mut pc: u32 = 0x82D62620;
    'dispatch: loop {
        match pc {
            0x82D62620 => {
    //   block [0x82D62620..0x82D62678)
	// 82D62620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D62624: 4BF46DE9  bl 0x82ca940c
	ctx.lr = 0x82D62628;
	sub_82CA93D0(ctx, base);
	// 82D62628: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6262C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D62630: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D62634: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82D62638: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6263C: 4BFFA71D  bl 0x82d5cd58
	ctx.lr = 0x82D62640;
	sub_82D5CD58(ctx, base);
	// 82D62640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D62644: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62648: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D6264C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D62650: 4BFF33C9  bl 0x82d55a18
	ctx.lr = 0x82D62654;
	sub_82D55A18(ctx, base);
	// 82D62654: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D62658: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82D6265C: 419A0010  beq cr6, 0x82d6266c
	if ctx.cr[6].eq {
	pc = 0x82D6266C; continue 'dispatch;
	}
	// 82D62660: A1630012  lhz r11, 0x12(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(18 as u32) ) } as u64;
	// 82D62664: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82D62668: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D6266C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D62670: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D62674: 4BF46DE8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D62678 size=12
    let mut pc: u32 = 0x82D62678;
    'dispatch: loop {
        match pc {
            0x82D62678 => {
    //   block [0x82D62678..0x82D62684)
	// 82D62678: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82D6267C: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82D62680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D62688 size=20
    let mut pc: u32 = 0x82D62688;
    'dispatch: loop {
        match pc {
            0x82D62688 => {
    //   block [0x82D62688..0x82D6269C)
	// 82D62688: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6268C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D62690: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D62694: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D62698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D626A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D626A0 size=80
    let mut pc: u32 = 0x82D626A0;
    'dispatch: loop {
        match pc {
            0x82D626A0 => {
    //   block [0x82D626A0..0x82D626F0)
	// 82D626A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D626A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D626A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D626AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D626B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D626B4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D626B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D626BC: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D626C0: 4BFFA699  bl 0x82d5cd58
	ctx.lr = 0x82D626C4;
	sub_82D5CD58(ctx, base);
	// 82D626C4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D626C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D626CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D626D0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D626D4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D626D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D626DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D626E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D626E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D626E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D626EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D626F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D626F0 size=4
    let mut pc: u32 = 0x82D626F0;
    'dispatch: loop {
        match pc {
            0x82D626F0 => {
    //   block [0x82D626F0..0x82D626F4)
	// 82D626F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D626F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D626F8 size=32
    let mut pc: u32 = 0x82D626F8;
    'dispatch: loop {
        match pc {
            0x82D626F8 => {
    //   block [0x82D626F8..0x82D62718)
	// 82D626F8: 81640020  lwz r11, 0x20(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D626FC: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82D62700: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82D62704: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82D62708: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82D6270C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82D62710: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D62714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D62718 size=64
    let mut pc: u32 = 0x82D62718;
    'dispatch: loop {
        match pc {
            0x82D62718 => {
    //   block [0x82D62718..0x82D62758)
	// 82D62718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6271C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D62720: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D62724: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D62728: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D6272C: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D62730: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82D62734: 419A0010  beq cr6, 0x82d62744
	if ctx.cr[6].eq {
	pc = 0x82D62744; continue 'dispatch;
	}
	// 82D62738: 4BF9DEA1  bl 0x82d005d8
	ctx.lr = 0x82D6273C;
	sub_82D005D8(ctx, base);
	// 82D6273C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82D62740: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82D62744: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D62748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D6274C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D62750: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D62754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D62758 size=92
    let mut pc: u32 = 0x82D62758;
    'dispatch: loop {
        match pc {
            0x82D62758 => {
    //   block [0x82D62758..0x82D627B4)
	// 82D62758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6275C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D62760: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D62764: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D62768: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D6276C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62770: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D62774: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D62778: 4E800421  bctrl
	ctx.lr = 0x82D6277C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D6277C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82D62780: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D62784: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82D62788: 4BF9DE39  bl 0x82d005c0
	ctx.lr = 0x82D6278C;
	sub_82D005C0(ctx, base);
	// 82D6278C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82D62790: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 82D62794: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82D62798: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82D6279C: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82D627A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D627A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D627A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D627AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D627B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D627B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D627B8 size=100
    let mut pc: u32 = 0x82D627B8;
    'dispatch: loop {
        match pc {
            0x82D627B8 => {
    //   block [0x82D627B8..0x82D6281C)
	// 82D627B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D627BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D627C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D627C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D627C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D627CC: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D627D0: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82D627D4: 419A0030  beq cr6, 0x82d62804
	if ctx.cr[6].eq {
	pc = 0x82D62804; continue 'dispatch;
	}
	// 82D627D8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82D627DC: 4BF9DEBD  bl 0x82d00698
	ctx.lr = 0x82D627E0;
	sub_82D00698(ctx, base);
	// 82D627E0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D627E4: 4099000C  ble cr6, 0x82d627f0
	if !ctx.cr[6].gt {
	pc = 0x82D627F0; continue 'dispatch;
	}
	// 82D627E8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82D627EC: 409A001C  bne cr6, 0x82d62808
	if !ctx.cr[6].eq {
	pc = 0x82D62808; continue 'dispatch;
	}
	// 82D627F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D627F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D627F8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D627FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D62800: 4E800421  bctrl
	ctx.lr = 0x82D62804;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D62804: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D62808: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D6280C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D62810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D62814: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D62818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D62820 size=100
    let mut pc: u32 = 0x82D62820;
    'dispatch: loop {
        match pc {
            0x82D62820 => {
    //   block [0x82D62820..0x82D62884)
	// 82D62820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D62824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D62828: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D6282C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D62830: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D62834: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D62838: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82D6283C: 419A0030  beq cr6, 0x82d6286c
	if ctx.cr[6].eq {
	pc = 0x82D6286C; continue 'dispatch;
	}
	// 82D62840: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82D62844: 4BF9DEE5  bl 0x82d00728
	ctx.lr = 0x82D62848;
	sub_82D00728(ctx, base);
	// 82D62848: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D6284C: 4099000C  ble cr6, 0x82d62858
	if !ctx.cr[6].gt {
	pc = 0x82D62858; continue 'dispatch;
	}
	// 82D62850: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82D62854: 409A001C  bne cr6, 0x82d62870
	if !ctx.cr[6].eq {
	pc = 0x82D62870; continue 'dispatch;
	}
	// 82D62858: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6285C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D62860: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D62864: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D62868: 4E800421  bctrl
	ctx.lr = 0x82D6286C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D6286C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D62870: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D62874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D62878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D6287C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D62880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D62888 size=24
    let mut pc: u32 = 0x82D62888;
    'dispatch: loop {
        match pc {
            0x82D62888 => {
    //   block [0x82D62888..0x82D628A0)
	// 82D62888: 3964FFD0  addi r11, r4, -0x30
	ctx.r[11].s64 = ctx.r[4].s64 + -48;
	// 82D6288C: 216B0009  subfic r11, r11, 9
	ctx.xer.ca = ctx.r[11].u32 <= 9 as u32;
	ctx.r[11].s64 = (9 as i64) - ctx.r[11].s64;
	// 82D62890: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82D62894: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D62898: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D6289C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D628A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D628A0 size=8
    let mut pc: u32 = 0x82D628A0;
    'dispatch: loop {
        match pc {
            0x82D628A0 => {
    //   block [0x82D628A0..0x82D628A8)
	// 82D628A0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D628A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D628A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D628A8 size=220
    let mut pc: u32 = 0x82D628A8;
    'dispatch: loop {
        match pc {
            0x82D628A8 => {
    //   block [0x82D628A8..0x82D62984)
	// 82D628A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D628AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D628B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D628B4: 9421FBF0  stwu r1, -0x410(r1)
	ea = ctx.r[1].u32.wrapping_add(-1040 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D628B8: 3FE08333  lis r31, -0x7ccd
	ctx.r[31].s64 = -2093809664;
	// 82D628BC: 897F7770  lbz r11, 0x7770(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(30576 as u32) ) } as u64;
	// 82D628C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D628C4: 409A00AC  bne cr6, 0x82d62970
	if !ctx.cr[6].eq {
	pc = 0x82D62970; continue 'dispatch;
	}
	// 82D628C8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82D628CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D628D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D628D4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D628D8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82D628DC: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82D628E0: 994B000C  stb r10, 0xc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 82D628E4: 3960000D  li r11, 0xd
	ctx.r[11].s64 = 13;
	// 82D628E8: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82D628EC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D628F0: 99610051  stb r11, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[11].u8 ) };
	// 82D628F4: 4BF9DF95  bl 0x82d00888
	ctx.lr = 0x82D628F8;
	sub_82D00888(ctx, base);
	// 82D628F8: 38810270  addi r4, r1, 0x270
	ctx.r[4].s64 = ctx.r[1].s64 + 624;
	// 82D628FC: 38600202  li r3, 0x202
	ctx.r[3].s64 = 514;
	// 82D62900: 4BF9DCA9  bl 0x82d005a8
	ctx.lr = 0x82D62904;
	sub_82D005A8(ctx, base);
	// 82D62904: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82D62908: 409A0060  bne cr6, 0x82d62968
	if !ctx.cr[6].eq {
	pc = 0x82D62968; continue 'dispatch;
	}
	// 82D6290C: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82D62910: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82D62914: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D62918: 4BFF50D1  bl 0x82d579e8
	ctx.lr = 0x82D6291C;
	sub_82D579E8(ctx, base);
	// 82D6291C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D62920: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D62924: 388B515C  addi r4, r11, 0x515c
	ctx.r[4].s64 = ctx.r[11].s64 + 20828;
	// 82D62928: 4BFF54C9  bl 0x82d57df0
	ctx.lr = 0x82D6292C;
	sub_82D57DF0(ctx, base);
	// 82D6292C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D62930: 3CA03218  lis r5, 0x3218
	ctx.r[5].s64 = 840433664;
	// 82D62934: 39000048  li r8, 0x48
	ctx.r[8].s64 = 72;
	// 82D62938: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82D6293C: 60A525F8  ori r5, r5, 0x25f8
	ctx.r[5].u64 = ctx.r[5].u64 | 9720;
	// 82D62940: 806B7630  lwz r3, 0x7630(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30256 as u32) ) } as u64;
	// 82D62944: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D62948: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82D6294C: 38EB5134  addi r7, r11, 0x5134
	ctx.r[7].s64 = ctx.r[11].s64 + 20788;
	// 82D62950: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62954: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D62958: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D6295C: 4E800421  bctrl
	ctx.lr = 0x82D62960;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D62960: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D62964: 4BFF5ACD  bl 0x82d58430
	ctx.lr = 0x82D62968;
	sub_82D58430(ctx, base);
	// 82D62968: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D6296C: 997F7770  stb r11, 0x7770(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(30576 as u32), ctx.r[11].u8 ) };
	// 82D62970: 38210410  addi r1, r1, 0x410
	ctx.r[1].s64 = ctx.r[1].s64 + 1040;
	// 82D62974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D62978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D6297C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D62980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D62988 size=96
    let mut pc: u32 = 0x82D62988;
    'dispatch: loop {
        match pc {
            0x82D62988 => {
    //   block [0x82D62988..0x82D629E8)
	// 82D62988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6298C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D62990: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D62994: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D62998: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D6299C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D629A0: 396B2698  addi r11, r11, 0x2698
	ctx.r[11].s64 = ctx.r[11].s64 + 9880;
	// 82D629A4: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D629A8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82D629AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D629B0: 419A0010  beq cr6, 0x82d629c0
	if ctx.cr[6].eq {
	pc = 0x82D629C0; continue 'dispatch;
	}
	// 82D629B4: 4BF9DC25  bl 0x82d005d8
	ctx.lr = 0x82D629B8;
	sub_82D005D8(ctx, base);
	// 82D629B8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82D629BC: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82D629C0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D629C4: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D629C8: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82D629CC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D629D0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D629D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D629D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D629DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D629E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D629E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D629E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D629E8 size=316
    let mut pc: u32 = 0x82D629E8;
    'dispatch: loop {
        match pc {
            0x82D629E8 => {
    //   block [0x82D629E8..0x82D62B24)
	// 82D629E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D629EC: 4BF46A21  bl 0x82ca940c
	ctx.lr = 0x82D629F0;
	sub_82CA93D0(ctx, base);
	// 82D629F0: 9421FD70  stwu r1, -0x290(r1)
	ea = ctx.r[1].u32.wrapping_add(-656 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D629F4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D629F8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D629FC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D62A00: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82D62A04: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D62A08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D62A0C: 4BFF6335  bl 0x82d58d40
	ctx.lr = 0x82D62A10;
	sub_82D58D40(ctx, base);
	// 82D62A10: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62A14: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82D62A18: B3C10052  sth r30, 0x52(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[30].u16 ) };
	// 82D62A1C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82D62A20: 396BFFD0  addi r11, r11, -0x30
	ctx.r[11].s64 = ctx.r[11].s64 + -48;
	// 82D62A24: 216B0009  subfic r11, r11, 9
	ctx.xer.ca = ctx.r[11].u32 <= 9 as u32;
	ctx.r[11].s64 = (9 as i64) - ctx.r[11].s64;
	// 82D62A28: B1410050  sth r10, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u16 ) };
	// 82D62A2C: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82D62A30: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D62A34: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82D62A38: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D62A3C: 419A0014  beq cr6, 0x82d62a50
	if ctx.cr[6].eq {
	pc = 0x82D62A50; continue 'dispatch;
	}
	// 82D62A40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D62A44: 4BF9DD75  bl 0x82d007b8
	ctx.lr = 0x82D62A48;
	sub_82D007B8(ctx, base);
	// 82D62A48: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82D62A4C: 4800005C  b 0x82d62aa8
	pc = 0x82D62AA8; continue 'dispatch;
	// 82D62A50: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82D62A54: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82D62A58: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D62A5C: 4BFF4F8D  bl 0x82d579e8
	ctx.lr = 0x82D62A60;
	sub_82D579E8(ctx, base);
	// 82D62A60: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D62A64: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D62A68: 388B5184  addi r4, r11, 0x5184
	ctx.r[4].s64 = ctx.r[11].s64 + 20868;
	// 82D62A6C: 4BFF5385  bl 0x82d57df0
	ctx.lr = 0x82D62A70;
	sub_82D57DF0(ctx, base);
	// 82D62A70: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D62A74: 390000C3  li r8, 0xc3
	ctx.r[8].s64 = 195;
	// 82D62A78: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82D62A7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82D62A80: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82D62A84: 806B7630  lwz r3, 0x7630(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30256 as u32) ) } as u64;
	// 82D62A88: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D62A8C: 38EB5134  addi r7, r11, 0x5134
	ctx.r[7].s64 = ctx.r[11].s64 + 20788;
	// 82D62A90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62A94: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D62A98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D62A9C: 4E800421  bctrl
	ctx.lr = 0x82D62AA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D62AA0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D62AA4: 4BFF598D  bl 0x82d58430
	ctx.lr = 0x82D62AA8;
	sub_82D58430(ctx, base);
	// 82D62AA8: 807D0020  lwz r3, 0x20(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D62AAC: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82D62AB0: 409A0034  bne cr6, 0x82d62ae4
	if !ctx.cr[6].eq {
	pc = 0x82D62AE4; continue 'dispatch;
	}
	// 82D62AB4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62AB8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D62ABC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D62AC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D62AC4: 4E800421  bctrl
	ctx.lr = 0x82D62AC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D62AC8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82D62ACC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D62AD0: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82D62AD4: 4BF9DAED  bl 0x82d005c0
	ctx.lr = 0x82D62AD8;
	sub_82D005C0(ctx, base);
	// 82D62AD8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82D62ADC: 907D0020  stw r3, 0x20(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 82D62AE0: 419A002C  beq cr6, 0x82d62b0c
	if ctx.cr[6].eq {
	pc = 0x82D62B0C; continue 'dispatch;
	}
	// 82D62AE4: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82D62AE8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D62AEC: 4BF9DB4D  bl 0x82d00638
	ctx.lr = 0x82D62AF0;
	sub_82D00638(ctx, base);
	// 82D62AF0: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82D62AF4: 409A0024  bne cr6, 0x82d62b18
	if !ctx.cr[6].eq {
	pc = 0x82D62B18; continue 'dispatch;
	}
	// 82D62AF8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62AFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D62B00: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D62B04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D62B08: 4E800421  bctrl
	ctx.lr = 0x82D62B0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D62B0C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D62B10: 38210290  addi r1, r1, 0x290
	ctx.r[1].s64 = ctx.r[1].s64 + 656;
	// 82D62B14: 4BF46948  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82D62B18: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D62B1C: 38210290  addi r1, r1, 0x290
	ctx.r[1].s64 = ctx.r[1].s64 + 656;
	// 82D62B20: 4BF4693C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D62B28 size=124
    let mut pc: u32 = 0x82D62B28;
    'dispatch: loop {
        match pc {
            0x82D62B28 => {
    //   block [0x82D62B28..0x82D62BA4)
	// 82D62B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D62B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D62B30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D62B34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D62B38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D62B3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D62B40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D62B44: 4BFFEB3D  bl 0x82d61680
	ctx.lr = 0x82D62B48;
	sub_82D61680(ctx, base);
	// 82D62B48: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D62B4C: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82D62B50: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82D62B54: 396B2698  addi r11, r11, 0x2698
	ctx.r[11].s64 = ctx.r[11].s64 + 9880;
	// 82D62B58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D62B5C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D62B60: 409A002C  bne cr6, 0x82d62b8c
	if !ctx.cr[6].eq {
	pc = 0x82D62B8C; continue 'dispatch;
	}
	// 82D62B64: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D62B68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D62B6C: 4E800421  bctrl
	ctx.lr = 0x82D62B70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D62B70: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82D62B74: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D62B78: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82D62B7C: 4BF9DA45  bl 0x82d005c0
	ctx.lr = 0x82D62B80;
	sub_82D005C0(ctx, base);
	// 82D62B80: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D62B84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D62B88: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82D62B8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D62B90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D62B94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D62B98: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D62B9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D62BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D62BA8 size=616
    let mut pc: u32 = 0x82D62BA8;
    'dispatch: loop {
        match pc {
            0x82D62BA8 => {
    //   block [0x82D62BA8..0x82D62E10)
	// 82D62BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D62BAC: 4BF46855  bl 0x82ca9400
	ctx.lr = 0x82D62BB0;
	sub_82CA93D0(ctx, base);
	// 82D62BB0: 9421FD10  stwu r1, -0x2f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-752 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D62BB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D62BB8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82D62BBC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62BC0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D62BC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D62BC8: 4E800421  bctrl
	ctx.lr = 0x82D62BCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D62BCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82D62BD0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D62BD4: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82D62BD8: 4BF9D9E9  bl 0x82d005c0
	ctx.lr = 0x82D62BDC;
	sub_82D005C0(ctx, base);
	// 82D62BDC: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82D62BE0: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 82D62BE4: 419A0064  beq cr6, 0x82d62c48
	if ctx.cr[6].eq {
	pc = 0x82D62C48; continue 'dispatch;
	}
	// 82D62BE8: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82D62BEC: B38100A2  sth r28, 0xa2(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(162 as u32), ctx.r[28].u16 ) };
	// 82D62BF0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82D62BF4: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82D62BF8: 3C800000  lis r4, 0
	ctx.r[4].s64 = 0;
	// 82D62BFC: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82D62C00: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 82D62C04: B16100A0  sth r11, 0xa0(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[11].u16 ) };
	// 82D62C08: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82D62C0C: 93A100A4  stw r29, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[29].u32 ) };
	// 82D62C10: 6084FFFF  ori r4, r4, 0xffff
	ctx.r[4].u64 = ctx.r[4].u64 | 65535;
	// 82D62C14: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82D62C18: 4BF9D9E9  bl 0x82d00600
	ctx.lr = 0x82D62C1C;
	sub_82D00600(ctx, base);
	// 82D62C1C: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82D62C20: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 82D62C24: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D62C28: 4BF9D9F9  bl 0x82d00620
	ctx.lr = 0x82D62C2C;
	sub_82D00620(ctx, base);
	// 82D62C2C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82D62C30: 409A0024  bne cr6, 0x82d62c54
	if !ctx.cr[6].eq {
	pc = 0x82D62C54; continue 'dispatch;
	}
	// 82D62C34: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62C38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D62C3C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D62C40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D62C44: 4E800421  bctrl
	ctx.lr = 0x82D62C48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D62C48: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D62C4C: 382102F0  addi r1, r1, 0x2f0
	ctx.r[1].s64 = ctx.r[1].s64 + 752;
	// 82D62C50: 4BF46800  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 82D62C54: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82D62C58: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D62C5C: 4BF9D9F5  bl 0x82d00650
	ctx.lr = 0x82D62C60;
	sub_82D00650(ctx, base);
	// 82D62C60: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82D62C64: 419AFFD0  beq cr6, 0x82d62c34
	if ctx.cr[6].eq {
	pc = 0x82D62C34; continue 'dispatch;
	}
	// 82D62C68: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62C6C: 3B600004  li r27, 4
	ctx.r[27].s64 = 4;
	// 82D62C70: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82D62C74: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D62C78: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82D62C7C: 4BFF25CD  bl 0x82d55248
	ctx.lr = 0x82D62C80;
	sub_82D55248(ctx, base);
	// 82D62C80: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82D62C84: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82D62C88: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82D62C8C: 9BA30000  stb r29, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 82D62C90: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D62C94: 4BF9DCDD  bl 0x82d00970
	ctx.lr = 0x82D62C98;
	sub_82D00970(ctx, base);
	// 82D62C98: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D62C9C: 419AFFF4  beq cr6, 0x82d62c90
	if ctx.cr[6].eq {
	pc = 0x82D62C90; continue 'dispatch;
	}
	// 82D62CA0: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82D62CA4: 409A0038  bne cr6, 0x82d62cdc
	if !ctx.cr[6].eq {
	pc = 0x82D62CDC; continue 'dispatch;
	}
	// 82D62CA8: 93A10060  stw r29, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 82D62CAC: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82D62CB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D62CB4: 419A0058  beq cr6, 0x82d62d0c
	if ctx.cr[6].eq {
	pc = 0x82D62D0C; continue 'dispatch;
	}
	// 82D62CB8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D62CBC: 89010067  lbz r8, 0x67(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(103 as u32) ) } as u64;
	// 82D62CC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D62CC4: 88E10066  lbz r7, 0x66(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(102 as u32) ) } as u64;
	// 82D62CC8: 388B51C8  addi r4, r11, 0x51c8
	ctx.r[4].s64 = ctx.r[11].s64 + 20936;
	// 82D62CCC: 88C10065  lbz r6, 0x65(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(101 as u32) ) } as u64;
	// 82D62CD0: 88A10064  lbz r5, 0x64(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82D62CD4: 4BFF61BD  bl 0x82d58e90
	ctx.lr = 0x82D62CD8;
	sub_82D58E90(ctx, base);
	// 82D62CD8: 4800008C  b 0x82d62d64
	pc = 0x82D62D64; continue 'dispatch;
	// 82D62CDC: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82D62CE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D62CE4: 419AFFC8  beq cr6, 0x82d62cac
	if ctx.cr[6].eq {
	pc = 0x82D62CAC; continue 'dispatch;
	}
	// 82D62CE8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D62CEC: 89010063  lbz r8, 0x63(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(99 as u32) ) } as u64;
	// 82D62CF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D62CF4: 88E10062  lbz r7, 0x62(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(98 as u32) ) } as u64;
	// 82D62CF8: 388B51C8  addi r4, r11, 0x51c8
	ctx.r[4].s64 = ctx.r[11].s64 + 20936;
	// 82D62CFC: 88C10061  lbz r6, 0x61(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(97 as u32) ) } as u64;
	// 82D62D00: 88A10060  lbz r5, 0x60(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82D62D04: 4BFF618D  bl 0x82d58e90
	ctx.lr = 0x82D62D08;
	sub_82D58E90(ctx, base);
	// 82D62D08: 4800005C  b 0x82d62d64
	pc = 0x82D62D64; continue 'dispatch;
	// 82D62D0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82D62D10: 3BCB16F8  addi r30, r11, 0x16f8
	ctx.r[30].s64 = ctx.r[11].s64 + 5880;
	// 82D62D14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D62D18: 4BFF5F11  bl 0x82d58c28
	ctx.lr = 0x82D62D1C;
	sub_82D58C28(ctx, base);
	// 82D62D1C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D62D20: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 82D62D24: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D62D28: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D62D2C: 40980024  bge cr6, 0x82d62d50
	if !ctx.cr[6].lt {
	pc = 0x82D62D50; continue 'dispatch;
	}
	// 82D62D30: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D62D34: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D62D38: 41980008  blt cr6, 0x82d62d40
	if ctx.cr[6].lt {
	pc = 0x82D62D40; continue 'dispatch;
	}
	// 82D62D3C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82D62D40: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D62D44: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D62D48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D62D4C: 4BFF41C5  bl 0x82d56f10
	ctx.lr = 0x82D62D50;
	sub_82D56F10(ctx, base);
	// 82D62D50: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D62D54: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D62D58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D62D5C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82D62D60: 4BFF5FD1  bl 0x82d58d30
	ctx.lr = 0x82D62D64;
	sub_82D58D30(ctx, base);
	// 82D62D64: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82D62D68: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 82D62D6C: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 82D62D70: 4BFF4C79  bl 0x82d579e8
	ctx.lr = 0x82D62D74;
	sub_82D579E8(ctx, base);
	// 82D62D74: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D62D78: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 82D62D7C: 388B51B4  addi r4, r11, 0x51b4
	ctx.r[4].s64 = ctx.r[11].s64 + 20916;
	// 82D62D80: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D62D84: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 82D62D88: 3BEB51AC  addi r31, r11, 0x51ac
	ctx.r[31].s64 = ctx.r[11].s64 + 20908;
	// 82D62D8C: 4BFF5065  bl 0x82d57df0
	ctx.lr = 0x82D62D90;
	sub_82D57DF0(ctx, base);
	// 82D62D90: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D62D94: 4BFF57B5  bl 0x82d58548
	ctx.lr = 0x82D62D98;
	sub_82D58548(ctx, base);
	// 82D62D98: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D62D9C: 4BFF5055  bl 0x82d57df0
	ctx.lr = 0x82D62DA0;
	sub_82D57DF0(ctx, base);
	// 82D62DA0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82D62DA4: 4BFF5185  bl 0x82d57f28
	ctx.lr = 0x82D62DA8;
	sub_82D57F28(ctx, base);
	// 82D62DA8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D62DAC: 3900016C  li r8, 0x16c
	ctx.r[8].s64 = 364;
	// 82D62DB0: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82D62DB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82D62DB8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D62DBC: 806B7630  lwz r3, 0x7630(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30256 as u32) ) } as u64;
	// 82D62DC0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D62DC4: 38EB5134  addi r7, r11, 0x5134
	ctx.r[7].s64 = ctx.r[11].s64 + 20788;
	// 82D62DC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62DCC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D62DD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D62DD4: 4E800421  bctrl
	ctx.lr = 0x82D62DD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D62DD8: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 82D62DDC: 4BFF5655  bl 0x82d58430
	ctx.lr = 0x82D62DE0;
	sub_82D58430(ctx, base);
	// 82D62DE0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D62DE4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D62DE8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D62DEC: 409A0018  bne cr6, 0x82d62e04
	if !ctx.cr[6].eq {
	pc = 0x82D62E04; continue 'dispatch;
	}
	// 82D62DF0: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D62DF4: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82D62DF8: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D62DFC: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D62E00: 4BFF24C9  bl 0x82d552c8
	ctx.lr = 0x82D62E04;
	sub_82D552C8(ctx, base);
	// 82D62E04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D62E08: 382102F0  addi r1, r1, 0x2f0
	ctx.r[1].s64 = ctx.r[1].s64 + 752;
	// 82D62E0C: 4BF46644  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D62E10 size=468
    let mut pc: u32 = 0x82D62E10;
    'dispatch: loop {
        match pc {
            0x82D62E10 => {
    //   block [0x82D62E10..0x82D62FE4)
	// 82D62E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D62E14: 4BF465F1  bl 0x82ca9404
	ctx.lr = 0x82D62E18;
	sub_82CA93D0(ctx, base);
	// 82D62E18: 9421FB20  stwu r1, -0x4e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-1248 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D62E1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D62E20: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D62E24: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82D62E28: 419A0134  beq cr6, 0x82d62f5c
	if ctx.cr[6].eq {
	pc = 0x82D62F5C; continue 'dispatch;
	}
	// 82D62E2C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82D62E30: 91610094  stw r11, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 82D62E34: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82D62E38: 916101A4  stw r11, 0x1a4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(420 as u32), ctx.r[11].u32 ) };
	// 82D62E3C: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 82D62E40: 38E10068  addi r7, r1, 0x68
	ctx.r[7].s64 = ctx.r[1].s64 + 104;
	// 82D62E44: 38C101A0  addi r6, r1, 0x1a0
	ctx.r[6].s64 = ctx.r[1].s64 + 416;
	// 82D62E48: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82D62E4C: 93C10090  stw r30, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[30].u32 ) };
	// 82D62E50: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82D62E54: 93C101A0  stw r30, 0x1a0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(416 as u32), ctx.r[30].u32 ) };
	// 82D62E58: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 82D62E5C: 9361006C  stw r27, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[27].u32 ) };
	// 82D62E60: 4B562DA9  bl 0x822c5c08
	ctx.lr = 0x82D62E64;
	sub_822C5C08(ctx, base);
	// 82D62E64: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D62E68: 409900F4  ble cr6, 0x82d62f5c
	if !ctx.cr[6].gt {
	pc = 0x82D62F5C; continue 'dispatch;
	}
	// 82D62E6C: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82D62E70: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D62E74: 4BF9D955  bl 0x82d007c8
	ctx.lr = 0x82D62E78;
	sub_82D007C8(ctx, base);
	// 82D62E78: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D62E7C: 419A00E0  beq cr6, 0x82d62f5c
	if ctx.cr[6].eq {
	pc = 0x82D62F5C; continue 'dispatch;
	}
	// 82D62E80: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82D62E84: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D62E88: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82D62E8C: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82D62E90: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82D62E94: 4BF9D7CD  bl 0x82d00660
	ctx.lr = 0x82D62E98;
	sub_82D00660(ctx, base);
	// 82D62E98: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62E9C: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 82D62EA0: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82D62EA4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D62EA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D62EAC: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82D62EB0: 4BFF2399  bl 0x82d55248
	ctx.lr = 0x82D62EB4;
	sub_82D55248(ctx, base);
	// 82D62EB4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D62EB8: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82D62EBC: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82D62EC0: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82D62EC4: 388B51D4  addi r4, r11, 0x51d4
	ctx.r[4].s64 = ctx.r[11].s64 + 20948;
	// 82D62EC8: 9B630000  stb r27, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u8 ) };
	// 82D62ECC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D62ED0: A0C10082  lhz r6, 0x82(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(130 as u32) ) } as u64;
	// 82D62ED4: 80A10084  lwz r5, 0x84(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82D62ED8: 4BFF5FB9  bl 0x82d58e90
	ctx.lr = 0x82D62EDC;
	sub_82D58E90(ctx, base);
	// 82D62EDC: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82D62EE0: 388102B0  addi r4, r1, 0x2b0
	ctx.r[4].s64 = ctx.r[1].s64 + 688;
	// 82D62EE4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82D62EE8: 4BFF4B01  bl 0x82d579e8
	ctx.lr = 0x82D62EEC;
	sub_82D579E8(ctx, base);
	// 82D62EEC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D62EF0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82D62EF4: 4BFF5655  bl 0x82d58548
	ctx.lr = 0x82D62EF8;
	sub_82D58548(ctx, base);
	// 82D62EF8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D62EFC: 39000194  li r8, 0x194
	ctx.r[8].s64 = 404;
	// 82D62F00: 38C102B0  addi r6, r1, 0x2b0
	ctx.r[6].s64 = ctx.r[1].s64 + 688;
	// 82D62F04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82D62F08: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D62F0C: 806B7630  lwz r3, 0x7630(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30256 as u32) ) } as u64;
	// 82D62F10: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D62F14: 38EB5134  addi r7, r11, 0x5134
	ctx.r[7].s64 = ctx.r[11].s64 + 20788;
	// 82D62F18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D62F1C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D62F20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D62F24: 4E800421  bctrl
	ctx.lr = 0x82D62F28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D62F28: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82D62F2C: 4BFF5505  bl 0x82d58430
	ctx.lr = 0x82D62F30;
	sub_82D58430(ctx, base);
	// 82D62F30: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82D62F34: 409A0034  bne cr6, 0x82d62f68
	if !ctx.cr[6].eq {
	pc = 0x82D62F68; continue 'dispatch;
	}
	// 82D62F38: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D62F3C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D62F40: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D62F44: 409A0018  bne cr6, 0x82d62f5c
	if !ctx.cr[6].eq {
	pc = 0x82D62F5C; continue 'dispatch;
	}
	// 82D62F48: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D62F4C: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82D62F50: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D62F54: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D62F58: 4BFF2371  bl 0x82d552c8
	ctx.lr = 0x82D62F5C;
	sub_82D552C8(ctx, base);
	// 82D62F5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D62F60: 382104E0  addi r1, r1, 0x4e0
	ctx.r[1].s64 = ctx.r[1].s64 + 1248;
	// 82D62F64: 4BF464F0  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82D62F68: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82D62F6C: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82D62F70: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 82D62F74: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D62F78: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82D62F7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D62F80: 4BF9D681  bl 0x82d00600
	ctx.lr = 0x82D62F84;
	sub_82D00600(ctx, base);
	// 82D62F84: 38A00014  li r5, 0x14
	ctx.r[5].s64 = 20;
	// 82D62F88: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 82D62F8C: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82D62F90: 4BFF22B9  bl 0x82d55248
	ctx.lr = 0x82D62F94;
	sub_82D55248(ctx, base);
	// 82D62F94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D62F98: 39600024  li r11, 0x24
	ctx.r[11].s64 = 36;
	// 82D62F9C: B17E0004  sth r11, 4(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82D62FA0: 4BFFE6E1  bl 0x82d61680
	ctx.lr = 0x82D62FA4;
	sub_82D61680(ctx, base);
	// 82D62FA4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D62FA8: 93FE0020  stw r31, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 82D62FAC: 396B2698  addi r11, r11, 0x2698
	ctx.r[11].s64 = ctx.r[11].s64 + 9880;
	// 82D62FB0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D62FB4: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D62FB8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D62FBC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D62FC0: 409A0018  bne cr6, 0x82d62fd8
	if !ctx.cr[6].eq {
	pc = 0x82D62FD8; continue 'dispatch;
	}
	// 82D62FC4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D62FC8: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82D62FCC: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D62FD0: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D62FD4: 4BFF22F5  bl 0x82d552c8
	ctx.lr = 0x82D62FD8;
	sub_82D552C8(ctx, base);
	// 82D62FD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D62FDC: 382104E0  addi r1, r1, 0x4e0
	ctx.r[1].s64 = ctx.r[1].s64 + 1248;
	// 82D62FE0: 4BF46474  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D62FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D62FE8 size=152
    let mut pc: u32 = 0x82D62FE8;
    'dispatch: loop {
        match pc {
            0x82D62FE8 => {
    //   block [0x82D62FE8..0x82D63080)
	// 82D62FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D62FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D62FF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D62FF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D62FF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D62FFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D63000: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D63004: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D63008: 396B2698  addi r11, r11, 0x2698
	ctx.r[11].s64 = ctx.r[11].s64 + 9880;
	// 82D6300C: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D63010: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82D63014: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D63018: 419A0010  beq cr6, 0x82d63028
	if ctx.cr[6].eq {
	pc = 0x82D63028; continue 'dispatch;
	}
	// 82D6301C: 4BF9D5BD  bl 0x82d005d8
	ctx.lr = 0x82D63020;
	sub_82D005D8(ctx, base);
	// 82D63020: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82D63024: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82D63028: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D6302C: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D63030: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D63034: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D63038: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82D6303C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D63040: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D63044: 419A0020  beq cr6, 0x82d63064
	if ctx.cr[6].eq {
	pc = 0x82D63064; continue 'dispatch;
	}
	// 82D63048: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6304C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D63050: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 82D63054: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D63058: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D6305C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D63060: 4BFF2269  bl 0x82d552c8
	ctx.lr = 0x82D63064;
	sub_82D552C8(ctx, base);
	// 82D63064: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D63068: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D6306C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D63070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D63074: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D63078: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D6307C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63080 size=4
    let mut pc: u32 = 0x82D63080;
    'dispatch: loop {
        match pc {
            0x82D63080 => {
    //   block [0x82D63080..0x82D63084)
	// 82D63080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63088 size=4
    let mut pc: u32 = 0x82D63088;
    'dispatch: loop {
        match pc {
            0x82D63088 => {
    //   block [0x82D63088..0x82D6308C)
	// 82D63088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63090 size=4
    let mut pc: u32 = 0x82D63090;
    'dispatch: loop {
        match pc {
            0x82D63090 => {
    //   block [0x82D63090..0x82D63094)
	// 82D63090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63098 size=4
    let mut pc: u32 = 0x82D63098;
    'dispatch: loop {
        match pc {
            0x82D63098 => {
    //   block [0x82D63098..0x82D6309C)
	// 82D63098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D630A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D630A0 size=4
    let mut pc: u32 = 0x82D630A0;
    'dispatch: loop {
        match pc {
            0x82D630A0 => {
    //   block [0x82D630A0..0x82D630A4)
	// 82D630A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D630A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D630A8 size=4
    let mut pc: u32 = 0x82D630A8;
    'dispatch: loop {
        match pc {
            0x82D630A8 => {
    //   block [0x82D630A8..0x82D630AC)
	// 82D630A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D630B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D630B0 size=4
    let mut pc: u32 = 0x82D630B0;
    'dispatch: loop {
        match pc {
            0x82D630B0 => {
    //   block [0x82D630B0..0x82D630B4)
	// 82D630B0: 48000008  b 0x82d630b8
	sub_82D630B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D630B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D630B8 size=176
    let mut pc: u32 = 0x82D630B8;
    'dispatch: loop {
        match pc {
            0x82D630B8 => {
    //   block [0x82D630B8..0x82D63168)
	// 82D630B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D630BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D630C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D630C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D630C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D630CC: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D630D0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D630D4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D630D8: 409A0020  bne cr6, 0x82d630f8
	if !ctx.cr[6].eq {
	pc = 0x82D630F8; continue 'dispatch;
	}
	// 82D630DC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D630E0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D630E4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D630E8: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D630EC: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82D630F0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D630F4: 4BFF21D5  bl 0x82d552c8
	ctx.lr = 0x82D630F8;
	sub_82D552C8(ctx, base);
	// 82D630F8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D630FC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D63100: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D63104: 409A0024  bne cr6, 0x82d63128
	if !ctx.cr[6].eq {
	pc = 0x82D63128; continue 'dispatch;
	}
	// 82D63108: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6310C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D63110: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D63114: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D63118: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D6311C: 1CAB0070  mulli r5, r11, 0x70
	ctx.r[5].s64 = ctx.r[11].s64 * 112;
	// 82D63120: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D63124: 4BFF21A5  bl 0x82d552c8
	ctx.lr = 0x82D63128;
	sub_82D552C8(ctx, base);
	// 82D63128: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6312C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D63130: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D63134: 409A0020  bne cr6, 0x82d63154
	if !ctx.cr[6].eq {
	pc = 0x82D63154; continue 'dispatch;
	}
	// 82D63138: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6313C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D63140: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D63144: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D63148: 55653032  slwi r5, r11, 6
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6314C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D63150: 4BFF2179  bl 0x82d552c8
	ctx.lr = 0x82D63154;
	sub_82D552C8(ctx, base);
	// 82D63154: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D63158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D6315C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D63160: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D63164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63168 size=4
    let mut pc: u32 = 0x82D63168;
    'dispatch: loop {
        match pc {
            0x82D63168 => {
    //   block [0x82D63168..0x82D6316C)
	// 82D63168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63170 size=4
    let mut pc: u32 = 0x82D63170;
    'dispatch: loop {
        match pc {
            0x82D63170 => {
    //   block [0x82D63170..0x82D63174)
	// 82D63170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63178 size=4
    let mut pc: u32 = 0x82D63178;
    'dispatch: loop {
        match pc {
            0x82D63178 => {
    //   block [0x82D63178..0x82D6317C)
	// 82D63178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63180 size=20
    let mut pc: u32 = 0x82D63180;
    'dispatch: loop {
        match pc {
            0x82D63180 => {
    //   block [0x82D63180..0x82D63194)
	// 82D63180: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63184: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D63188: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6318C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D63190: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63198 size=8
    let mut pc: u32 = 0x82D63198;
    'dispatch: loop {
        match pc {
            0x82D63198 => {
    //   block [0x82D63198..0x82D631A0)
	// 82D63198: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D6319C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D631A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D631A0 size=24
    let mut pc: u32 = 0x82D631A0;
    'dispatch: loop {
        match pc {
            0x82D631A0 => {
    //   block [0x82D631A0..0x82D631B8)
	// 82D631A0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D631A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D631A8: 396B59F4  addi r11, r11, 0x59f4
	ctx.r[11].s64 = ctx.r[11].s64 + 23028;
	// 82D631AC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D631B0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D631B4: 48004884  b 0x82d67a38
	sub_82D67A38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D631B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D631B8 size=4
    let mut pc: u32 = 0x82D631B8;
    'dispatch: loop {
        match pc {
            0x82D631B8 => {
    //   block [0x82D631B8..0x82D631BC)
	// 82D631B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D631C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D631C0 size=12
    let mut pc: u32 = 0x82D631C0;
    'dispatch: loop {
        match pc {
            0x82D631C0 => {
    //   block [0x82D631C0..0x82D631CC)
	// 82D631C0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D631C4: 386B59F4  addi r3, r11, 0x59f4
	ctx.r[3].s64 = ctx.r[11].s64 + 23028;
	// 82D631C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D631D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D631D0 size=20
    let mut pc: u32 = 0x82D631D0;
    'dispatch: loop {
        match pc {
            0x82D631D0 => {
    //   block [0x82D631D0..0x82D631E4)
	// 82D631D0: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D631D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D631D8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D631DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D631E0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D631E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D631E8 size=20
    let mut pc: u32 = 0x82D631E8;
    'dispatch: loop {
        match pc {
            0x82D631E8 => {
    //   block [0x82D631E8..0x82D631FC)
	// 82D631E8: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D631EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D631F0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D631F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D631F8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D63200 size=100
    let mut pc: u32 = 0x82D63200;
    'dispatch: loop {
        match pc {
            0x82D63200 => {
    //   block [0x82D63200..0x82D63264)
	// 82D63200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D63204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D63208: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D6320C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D63210: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D63214: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D63218: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D6321C: 48004CED  bl 0x82d67f08
	ctx.lr = 0x82D63220;
	sub_82D67F08(ctx, base);
	// 82D63220: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D63224: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D63228: 419A0020  beq cr6, 0x82d63248
	if ctx.cr[6].eq {
	pc = 0x82D63248; continue 'dispatch;
	}
	// 82D6322C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63230: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D63234: 38C00037  li r6, 0x37
	ctx.r[6].s64 = 55;
	// 82D63238: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6323C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D63240: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D63244: 4BFF2085  bl 0x82d552c8
	ctx.lr = 0x82D63248;
	sub_82D552C8(ctx, base);
	// 82D63248: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D6324C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D63250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D63254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D63258: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D6325C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D63260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D63268 size=100
    let mut pc: u32 = 0x82D63268;
    'dispatch: loop {
        match pc {
            0x82D63268 => {
    //   block [0x82D63268..0x82D632CC)
	// 82D63268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6326C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D63270: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D63274: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D63278: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6327C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D63280: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D63284: 48005DD5  bl 0x82d69058
	ctx.lr = 0x82D63288;
	sub_82D69058(ctx, base);
	// 82D63288: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D6328C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D63290: 419A0020  beq cr6, 0x82d632b0
	if ctx.cr[6].eq {
	pc = 0x82D632B0; continue 'dispatch;
	}
	// 82D63294: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63298: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D6329C: 38C00039  li r6, 0x39
	ctx.r[6].s64 = 57;
	// 82D632A0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D632A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D632A8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D632AC: 4BFF201D  bl 0x82d552c8
	ctx.lr = 0x82D632B0;
	sub_82D552C8(ctx, base);
	// 82D632B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D632B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D632B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D632BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D632C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D632C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D632C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D632D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D632D0 size=8
    let mut pc: u32 = 0x82D632D0;
    'dispatch: loop {
        match pc {
            0x82D632D0 => {
    //   block [0x82D632D0..0x82D632D8)
	// 82D632D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D632D4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D632D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D632D8 size=24
    let mut pc: u32 = 0x82D632D8;
    'dispatch: loop {
        match pc {
            0x82D632D8 => {
    //   block [0x82D632D8..0x82D632F0)
	// 82D632D8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D632DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D632E0: 396B5B34  addi r11, r11, 0x5b34
	ctx.r[11].s64 = ctx.r[11].s64 + 23348;
	// 82D632E4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D632E8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D632EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D632F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D632F0 size=20
    let mut pc: u32 = 0x82D632F0;
    'dispatch: loop {
        match pc {
            0x82D632F0 => {
    //   block [0x82D632F0..0x82D63304)
	// 82D632F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D632F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D632F8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D632FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D63300: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63308 size=12
    let mut pc: u32 = 0x82D63308;
    'dispatch: loop {
        match pc {
            0x82D63308 => {
    //   block [0x82D63308..0x82D63314)
	// 82D63308: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D6330C: 386B5B34  addi r3, r11, 0x5b34
	ctx.r[3].s64 = ctx.r[11].s64 + 23348;
	// 82D63310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63318 size=4
    let mut pc: u32 = 0x82D63318;
    'dispatch: loop {
        match pc {
            0x82D63318 => {
    //   block [0x82D63318..0x82D6331C)
	// 82D63318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63320 size=4
    let mut pc: u32 = 0x82D63320;
    'dispatch: loop {
        match pc {
            0x82D63320 => {
    //   block [0x82D63320..0x82D63324)
	// 82D63320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63328 size=20
    let mut pc: u32 = 0x82D63328;
    'dispatch: loop {
        match pc {
            0x82D63328 => {
    //   block [0x82D63328..0x82D6333C)
	// 82D63328: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6332C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D63330: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63334: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D63338: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63340 size=8
    let mut pc: u32 = 0x82D63340;
    'dispatch: loop {
        match pc {
            0x82D63340 => {
    //   block [0x82D63340..0x82D63348)
	// 82D63340: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D63344: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63348 size=24
    let mut pc: u32 = 0x82D63348;
    'dispatch: loop {
        match pc {
            0x82D63348 => {
    //   block [0x82D63348..0x82D63360)
	// 82D63348: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D6334C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D63350: 396B5D64  addi r11, r11, 0x5d64
	ctx.r[11].s64 = ctx.r[11].s64 + 23908;
	// 82D63354: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D63358: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D6335C: 48006E04  b 0x82d6a160
	sub_82D6A160(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63360 size=4
    let mut pc: u32 = 0x82D63360;
    'dispatch: loop {
        match pc {
            0x82D63360 => {
    //   block [0x82D63360..0x82D63364)
	// 82D63360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63368 size=12
    let mut pc: u32 = 0x82D63368;
    'dispatch: loop {
        match pc {
            0x82D63368 => {
    //   block [0x82D63368..0x82D63374)
	// 82D63368: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D6336C: 386B5D64  addi r3, r11, 0x5d64
	ctx.r[3].s64 = ctx.r[11].s64 + 23908;
	// 82D63370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D63378 size=100
    let mut pc: u32 = 0x82D63378;
    'dispatch: loop {
        match pc {
            0x82D63378 => {
    //   block [0x82D63378..0x82D633DC)
	// 82D63378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6337C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D63380: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D63384: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D63388: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6338C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D63390: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D63394: 4800723D  bl 0x82d6a5d0
	ctx.lr = 0x82D63398;
	sub_82D6A5D0(ctx, base);
	// 82D63398: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D6339C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D633A0: 419A0020  beq cr6, 0x82d633c0
	if ctx.cr[6].eq {
	pc = 0x82D633C0; continue 'dispatch;
	}
	// 82D633A4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D633A8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D633AC: 38C00037  li r6, 0x37
	ctx.r[6].s64 = 55;
	// 82D633B0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D633B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D633B8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D633BC: 4BFF1F0D  bl 0x82d552c8
	ctx.lr = 0x82D633C0;
	sub_82D552C8(ctx, base);
	// 82D633C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D633C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D633C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D633CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D633D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D633D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D633D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D633E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D633E0 size=20
    let mut pc: u32 = 0x82D633E0;
    'dispatch: loop {
        match pc {
            0x82D633E0 => {
    //   block [0x82D633E0..0x82D633F4)
	// 82D633E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D633E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D633E8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D633EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D633F0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D633F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D633F8 size=8
    let mut pc: u32 = 0x82D633F8;
    'dispatch: loop {
        match pc {
            0x82D633F8 => {
    //   block [0x82D633F8..0x82D63400)
	// 82D633F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D633FC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63400 size=24
    let mut pc: u32 = 0x82D63400;
    'dispatch: loop {
        match pc {
            0x82D63400 => {
    //   block [0x82D63400..0x82D63418)
	// 82D63400: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D63404: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D63408: 396B5E14  addi r11, r11, 0x5e14
	ctx.r[11].s64 = ctx.r[11].s64 + 24084;
	// 82D6340C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D63410: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D63414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63418 size=12
    let mut pc: u32 = 0x82D63418;
    'dispatch: loop {
        match pc {
            0x82D63418 => {
    //   block [0x82D63418..0x82D63424)
	// 82D63418: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D6341C: 386B5E14  addi r3, r11, 0x5e14
	ctx.r[3].s64 = ctx.r[11].s64 + 24084;
	// 82D63420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D63428 size=96
    let mut pc: u32 = 0x82D63428;
    'dispatch: loop {
        match pc {
            0x82D63428 => {
    //   block [0x82D63428..0x82D63488)
	// 82D63428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6342C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D63430: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D63434: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D63438: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D6343C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D63440: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D63444: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82D63448: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D6344C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D63450: 419A0020  beq cr6, 0x82d63470
	if ctx.cr[6].eq {
	pc = 0x82D63470; continue 'dispatch;
	}
	// 82D63454: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63458: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D6345C: 38C00036  li r6, 0x36
	ctx.r[6].s64 = 54;
	// 82D63460: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D63464: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D63468: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D6346C: 4BFF1E5D  bl 0x82d552c8
	ctx.lr = 0x82D63470;
	sub_82D552C8(ctx, base);
	// 82D63470: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D63474: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D63478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D6347C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D63480: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D63484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63488 size=20
    let mut pc: u32 = 0x82D63488;
    'dispatch: loop {
        match pc {
            0x82D63488 => {
    //   block [0x82D63488..0x82D6349C)
	// 82D63488: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6348C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D63490: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63494: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D63498: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D634A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D634A0 size=8
    let mut pc: u32 = 0x82D634A0;
    'dispatch: loop {
        match pc {
            0x82D634A0 => {
    //   block [0x82D634A0..0x82D634A8)
	// 82D634A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D634A4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D634A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D634A8 size=24
    let mut pc: u32 = 0x82D634A8;
    'dispatch: loop {
        match pc {
            0x82D634A8 => {
    //   block [0x82D634A8..0x82D634C0)
	// 82D634A8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D634AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D634B0: 396B5EEC  addi r11, r11, 0x5eec
	ctx.r[11].s64 = ctx.r[11].s64 + 24300;
	// 82D634B4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D634B8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D634BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D634C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D634C0 size=12
    let mut pc: u32 = 0x82D634C0;
    'dispatch: loop {
        match pc {
            0x82D634C0 => {
    //   block [0x82D634C0..0x82D634CC)
	// 82D634C0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D634C4: 386B5EEC  addi r3, r11, 0x5eec
	ctx.r[3].s64 = ctx.r[11].s64 + 24300;
	// 82D634C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D634D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D634D0 size=100
    let mut pc: u32 = 0x82D634D0;
    'dispatch: loop {
        match pc {
            0x82D634D0 => {
    //   block [0x82D634D0..0x82D63534)
	// 82D634D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D634D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D634D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D634DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D634E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D634E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D634E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D634EC: 48002D25  bl 0x82d66210
	ctx.lr = 0x82D634F0;
	sub_82D66210(ctx, base);
	// 82D634F0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D634F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D634F8: 419A0020  beq cr6, 0x82d63518
	if ctx.cr[6].eq {
	pc = 0x82D63518; continue 'dispatch;
	}
	// 82D634FC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63500: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D63504: 38C0003A  li r6, 0x3a
	ctx.r[6].s64 = 58;
	// 82D63508: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6350C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D63510: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D63514: 4BFF1DB5  bl 0x82d552c8
	ctx.lr = 0x82D63518;
	sub_82D552C8(ctx, base);
	// 82D63518: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D6351C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D63520: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D63524: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D63528: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D6352C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D63530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63538 size=4
    let mut pc: u32 = 0x82D63538;
    'dispatch: loop {
        match pc {
            0x82D63538 => {
    //   block [0x82D63538..0x82D6353C)
	// 82D63538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63540 size=4
    let mut pc: u32 = 0x82D63540;
    'dispatch: loop {
        match pc {
            0x82D63540 => {
    //   block [0x82D63540..0x82D63544)
	// 82D63540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63548 size=4
    let mut pc: u32 = 0x82D63548;
    'dispatch: loop {
        match pc {
            0x82D63548 => {
    //   block [0x82D63548..0x82D6354C)
	// 82D63548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63550 size=4
    let mut pc: u32 = 0x82D63550;
    'dispatch: loop {
        match pc {
            0x82D63550 => {
    //   block [0x82D63550..0x82D63554)
	// 82D63550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63558 size=4
    let mut pc: u32 = 0x82D63558;
    'dispatch: loop {
        match pc {
            0x82D63558 => {
    //   block [0x82D63558..0x82D6355C)
	// 82D63558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63560 size=4
    let mut pc: u32 = 0x82D63560;
    'dispatch: loop {
        match pc {
            0x82D63560 => {
    //   block [0x82D63560..0x82D63564)
	// 82D63560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63568 size=4
    let mut pc: u32 = 0x82D63568;
    'dispatch: loop {
        match pc {
            0x82D63568 => {
    //   block [0x82D63568..0x82D6356C)
	// 82D63568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63570 size=12
    let mut pc: u32 = 0x82D63570;
    'dispatch: loop {
        match pc {
            0x82D63570 => {
    //   block [0x82D63570..0x82D6357C)
	// 82D63570: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D63574: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D63578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63580 size=60
    let mut pc: u32 = 0x82D63580;
    'dispatch: loop {
        match pc {
            0x82D63580 => {
    //   block [0x82D63580..0x82D635BC)
	// 82D63580: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82D63588: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82D6358C: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D63590: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D63594: 4099001C  ble cr6, 0x82d635b0
	if !ctx.cr[6].gt {
	pc = 0x82D635B0; continue 'dispatch;
	}
	// 82D63598: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82D6359C: 80E3001C  lwz r7, 0x1c(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D635A0: 7D2759AE  stbx r9, r7, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 82D635A4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D635A8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D635AC: 4198FFF0  blt cr6, 0x82d6359c
	if ctx.cr[6].lt {
	pc = 0x82D6359C; continue 'dispatch;
	}
	// 82D635B0: 99030029  stb r8, 0x29(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(41 as u32), ctx.r[8].u8 ) };
	// 82D635B4: 99030028  stb r8, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[8].u8 ) };
	// 82D635B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D635C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D635C0 size=132
    let mut pc: u32 = 0x82D635C0;
    'dispatch: loop {
        match pc {
            0x82D635C0 => {
    //   block [0x82D635C0..0x82D63644)
	// 82D635C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D635C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D635C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D635CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D635D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D635D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D635D8: 80840000  lwz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D635DC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D635E0: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D635E4: 83EB0010  lwz r31, 0x10(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D635E8: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D635EC: 7D7F5A14  add r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82D635F0: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D635F4: 4BFF573D  bl 0x82d58d30
	ctx.lr = 0x82D635F8;
	sub_82D58D30(ctx, base);
	// 82D635F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D635FC: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82D63600: 4099001C  ble cr6, 0x82d6361c
	if !ctx.cr[6].gt {
	pc = 0x82D6361C; continue 'dispatch;
	}
	// 82D63604: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82D63608: 813E001C  lwz r9, 0x1c(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D6360C: 7D4B49AE  stbx r10, r11, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u8) };
	// 82D63610: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D63614: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D63618: 4198FFF0  blt cr6, 0x82d63608
	if ctx.cr[6].lt {
	pc = 0x82D63608; continue 'dispatch;
	}
	// 82D6361C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D63620: 997E0029  stb r11, 0x29(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(41 as u32), ctx.r[11].u8 ) };
	// 82D63624: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D63628: 997E0028  stb r11, 0x28(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 82D6362C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D63630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D63634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D63638: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D6363C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D63640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D63648 size=128
    let mut pc: u32 = 0x82D63648;
    'dispatch: loop {
        match pc {
            0x82D63648 => {
    //   block [0x82D63648..0x82D636C8)
	// 82D63648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6364C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D63650: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D63654: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D63658: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6365C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D63660: 80840000  lwz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63664: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63668: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6366C: 83EB0010  lwz r31, 0x10(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D63670: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D63674: 7D7F5A14  add r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82D63678: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6367C: 4BFF56B5  bl 0x82d58d30
	ctx.lr = 0x82D63680;
	sub_82D58D30(ctx, base);
	// 82D63680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D63684: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D63688: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82D6368C: 40990018  ble cr6, 0x82d636a4
	if !ctx.cr[6].gt {
	pc = 0x82D636A4; continue 'dispatch;
	}
	// 82D63690: 813E001C  lwz r9, 0x1c(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D63694: 7D4B49AE  stbx r10, r11, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u8) };
	// 82D63698: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D6369C: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D636A0: 4198FFF0  blt cr6, 0x82d63690
	if ctx.cr[6].lt {
	pc = 0x82D63690; continue 'dispatch;
	}
	// 82D636A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D636A8: 997E0029  stb r11, 0x29(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(41 as u32), ctx.r[11].u8 ) };
	// 82D636AC: 995E0028  stb r10, 0x28(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[10].u8 ) };
	// 82D636B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D636B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D636B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D636BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D636C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D636C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D636C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D636C8 size=72
    let mut pc: u32 = 0x82D636C8;
    'dispatch: loop {
        match pc {
            0x82D636C8 => {
    //   block [0x82D636C8..0x82D63710)
	// 82D636C8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82D636CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D636D0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D636D4: 81290010  lwz r9, 0x10(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D636D8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D636DC: 4099001C  ble cr6, 0x82d636f8
	if !ctx.cr[6].gt {
	pc = 0x82D636F8; continue 'dispatch;
	}
	// 82D636E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82D636E4: 80EA001C  lwz r7, 0x1c(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D636E8: 7D0759AE  stbx r8, r7, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32), ctx.r[8].u8) };
	// 82D636EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D636F0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D636F4: 4198FFF0  blt cr6, 0x82d636e4
	if ctx.cr[6].lt {
	pc = 0x82D636E4; continue 'dispatch;
	}
	// 82D636F8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82D636FC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D63700: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 82D63704: 992A0028  stb r9, 0x28(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(40 as u32), ctx.r[9].u8 ) };
	// 82D63708: 996A0029  stb r11, 0x29(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(41 as u32), ctx.r[11].u8 ) };
	// 82D6370C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63710 size=68
    let mut pc: u32 = 0x82D63710;
    'dispatch: loop {
        match pc {
            0x82D63710 => {
    //   block [0x82D63710..0x82D63754)
	// 82D63710: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82D63714: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D63718: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82D6371C: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63720: 81290010  lwz r9, 0x10(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D63724: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D63728: 40990018  ble cr6, 0x82d63740
	if !ctx.cr[6].gt {
	pc = 0x82D63740; continue 'dispatch;
	}
	// 82D6372C: 80EA001C  lwz r7, 0x1c(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D63730: 7D0759AE  stbx r8, r7, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32), ctx.r[8].u8) };
	// 82D63734: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D63738: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D6373C: 4198FFF0  blt cr6, 0x82d6372c
	if ctx.cr[6].lt {
	pc = 0x82D6372C; continue 'dispatch;
	}
	// 82D63740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D63744: 386A0010  addi r3, r10, 0x10
	ctx.r[3].s64 = ctx.r[10].s64 + 16;
	// 82D63748: 996A0029  stb r11, 0x29(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(41 as u32), ctx.r[11].u8 ) };
	// 82D6374C: 990A0028  stb r8, 0x28(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(40 as u32), ctx.r[8].u8 ) };
	// 82D63750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63758 size=196
    let mut pc: u32 = 0x82D63758;
    'dispatch: loop {
        match pc {
            0x82D63758 => {
    //   block [0x82D63758..0x82D6381C)
	// 82D63758: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6375C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82D63760: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 82D63764: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D63768: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D6376C: 40990034  ble cr6, 0x82d637a0
	if !ctx.cr[6].gt {
	pc = 0x82D637A0; continue 'dispatch;
	}
	// 82D63770: 8104001C  lwz r8, 0x1c(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D63774: 7D4858AE  lbzx r10, r8, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D63778: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82D6377C: 554607FE  clrlwi r6, r10, 0x1f
	ctx.r[6].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82D63780: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82D63784: 419A0010  beq cr6, 0x82d63794
	if ctx.cr[6].eq {
	pc = 0x82D63794; continue 'dispatch;
	}
	// 82D63788: 554A07BC  rlwinm r10, r10, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82D6378C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D63790: 409A008C  bne cr6, 0x82d6381c
	if !ctx.cr[6].eq {
		sub_82D6381C(ctx, base);
		return;
	}
	// 82D63794: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D63798: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D6379C: 4198FFD8  blt cr6, 0x82d63774
	if ctx.cr[6].lt {
	pc = 0x82D63774; continue 'dispatch;
	}
	// 82D637A0: 89640029  lbz r11, 0x29(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(41 as u32) ) } as u64;
	// 82D637A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D637A8: 419A0030  beq cr6, 0x82d637d8
	if ctx.cr[6].eq {
	pc = 0x82D637D8; continue 'dispatch;
	}
	// 82D637AC: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 82D637B0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D637B4: 40990024  ble cr6, 0x82d637d8
	if !ctx.cr[6].gt {
	pc = 0x82D637D8; continue 'dispatch;
	}
	// 82D637B8: 8144001C  lwz r10, 0x1c(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D637BC: 7D0A58AE  lbzx r8, r10, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D637C0: 550807FE  clrlwi r8, r8, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x00000001u64;
	// 82D637C4: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82D637C8: 409A0054  bne cr6, 0x82d6381c
	if !ctx.cr[6].eq {
		sub_82D6381C(ctx, base);
		return;
	}
	// 82D637CC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D637D0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D637D4: 4198FFE8  blt cr6, 0x82d637bc
	if ctx.cr[6].lt {
	pc = 0x82D637BC; continue 'dispatch;
	}
	// 82D637D8: 89640028  lbz r11, 0x28(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D637DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D637E0: 419A0030  beq cr6, 0x82d63810
	if ctx.cr[6].eq {
	pc = 0x82D63810; continue 'dispatch;
	}
	// 82D637E4: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 82D637E8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D637EC: 40990024  ble cr6, 0x82d63810
	if !ctx.cr[6].gt {
	pc = 0x82D63810; continue 'dispatch;
	}
	// 82D637F0: 8144001C  lwz r10, 0x1c(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D637F4: 7D0A58AE  lbzx r8, r10, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D637F8: 550807BC  rlwinm r8, r8, 0, 0x1e, 0x1e
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82D637FC: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82D63800: 409A001C  bne cr6, 0x82d6381c
	if !ctx.cr[6].eq {
		sub_82D6381C(ctx, base);
		return;
	}
	// 82D63804: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D63808: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D6380C: 4198FFE8  blt cr6, 0x82d637f4
	if ctx.cr[6].lt {
	pc = 0x82D637F4; continue 'dispatch;
	}
	// 82D63810: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D63814: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D63818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6381C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D6381C size=8
    let mut pc: u32 = 0x82D6381C;
    'dispatch: loop {
        match pc {
            0x82D6381C => {
    //   block [0x82D6381C..0x82D63824)
	// 82D6381C: 98E30000  stb r7, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 82D63820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D63828 size=140
    let mut pc: u32 = 0x82D63828;
    'dispatch: loop {
        match pc {
            0x82D63828 => {
    //   block [0x82D63828..0x82D638B4)
	// 82D63828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6382C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D63830: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D63834: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D63838: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D6383C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63840: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D63844: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D63848: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D6384C: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D63850: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82D63854: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D63858: 4BFF54D9  bl 0x82d58d30
	ctx.lr = 0x82D6385C;
	sub_82D58D30(ctx, base);
	// 82D6385C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D63864: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D63868: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D6386C: 40990024  ble cr6, 0x82d63890
	if !ctx.cr[6].gt {
	pc = 0x82D63890; continue 'dispatch;
	}
	// 82D63870: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82D63874: 813F001C  lwz r9, 0x1c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D63878: 7D4959AE  stbx r10, r9, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u8) };
	// 82D6387C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D63880: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63884: 81290010  lwz r9, 0x10(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D63888: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D6388C: 4198FFE8  blt cr6, 0x82d63874
	if ctx.cr[6].lt {
	pc = 0x82D63874; continue 'dispatch;
	}
	// 82D63890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D63894: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D63898: 995F0028  stb r10, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u8 ) };
	// 82D6389C: 997F0029  stb r11, 0x29(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(41 as u32), ctx.r[11].u8 ) };
	// 82D638A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D638A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D638A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D638AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D638B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D638B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D638B8 size=296
    let mut pc: u32 = 0x82D638B8;
    'dispatch: loop {
        match pc {
            0x82D638B8 => {
    //   block [0x82D638B8..0x82D639E0)
	// 82D638B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D638BC: 4BF45B41  bl 0x82ca93fc
	ctx.lr = 0x82D638C0;
	sub_82CA93D0(ctx, base);
	// 82D638C0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D638C4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82D638C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D638CC: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82D638D0: 3BBC0004  addi r29, r28, 4
	ctx.r[29].s64 = ctx.r[28].s64 + 4;
	// 82D638D4: 3BFC0010  addi r31, r28, 0x10
	ctx.r[31].s64 = ctx.r[28].s64 + 16;
	// 82D638D8: 3B7C001C  addi r27, r28, 0x1c
	ctx.r[27].s64 = ctx.r[28].s64 + 28;
	// 82D638DC: 90BC0000  stw r5, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82D638E0: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82D638E4: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82D638E8: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D638EC: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D638F0: 915D0008  stw r10, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82D638F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D638F8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D638FC: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82D63900: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D63904: 917B0004  stw r11, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D63908: 915B0008  stw r10, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82D6390C: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63910: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D63914: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D63918: 83CA0010  lwz r30, 0x10(r10)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6391C: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82D63920: 40980024  bge cr6, 0x82d63944
	if !ctx.cr[6].lt {
	pc = 0x82D63944; continue 'dispatch;
	}
	// 82D63924: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D63928: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D6392C: 41980008  blt cr6, 0x82d63934
	if ctx.cr[6].lt {
	pc = 0x82D63934; continue 'dispatch;
	}
	// 82D63930: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82D63934: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 82D63938: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D6393C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D63940: 4BFF35D1  bl 0x82d56f10
	ctx.lr = 0x82D63944;
	sub_82D56F10(ctx, base);
	// 82D63944: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82D63948: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D6394C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D63950: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82D63954: 40980024  bge cr6, 0x82d63978
	if !ctx.cr[6].lt {
	pc = 0x82D63978; continue 'dispatch;
	}
	// 82D63958: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D6395C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D63960: 41980008  blt cr6, 0x82d63968
	if ctx.cr[6].lt {
	pc = 0x82D63968; continue 'dispatch;
	}
	// 82D63964: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82D63968: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 82D6396C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D63970: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D63974: 4BFF359D  bl 0x82d56f10
	ctx.lr = 0x82D63978;
	sub_82D56F10(ctx, base);
	// 82D63978: 93DD0004  stw r30, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82D6397C: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D63980: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D63984: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82D63988: 40980024  bge cr6, 0x82d639ac
	if !ctx.cr[6].lt {
	pc = 0x82D639AC; continue 'dispatch;
	}
	// 82D6398C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D63990: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D63994: 41980008  blt cr6, 0x82d6399c
	if ctx.cr[6].lt {
	pc = 0x82D6399C; continue 'dispatch;
	}
	// 82D63998: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82D6399C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D639A0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D639A4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82D639A8: 4BFF3569  bl 0x82d56f10
	ctx.lr = 0x82D639AC;
	sub_82D56F10(ctx, base);
	// 82D639AC: 2F1A0001  cmpwi cr6, r26, 1
	ctx.cr[6].compare_i32(ctx.r[26].s32, 1, &mut ctx.xer);
	// 82D639B0: 93DB0004  stw r30, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82D639B4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82D639B8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82D639BC: 409A0014  bne cr6, 0x82d639d0
	if !ctx.cr[6].eq {
	pc = 0x82D639D0; continue 'dispatch;
	}
	// 82D639C0: 4BFFFC01  bl 0x82d635c0
	ctx.lr = 0x82D639C4;
	sub_82D635C0(ctx, base);
	// 82D639C4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82D639C8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D639CC: 4BF45A80  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
	// 82D639D0: 4BFFFC79  bl 0x82d63648
	ctx.lr = 0x82D639D4;
	sub_82D63648(ctx, base);
	// 82D639D4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82D639D8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D639DC: 4BF45A70  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D639E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D639E0 size=580
    let mut pc: u32 = 0x82D639E0;
    'dispatch: loop {
        match pc {
            0x82D639E0 => {
    //   block [0x82D639E0..0x82D63C24)
	// 82D639E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D639E4: 4BF45A1D  bl 0x82ca9400
	ctx.lr = 0x82D639E8;
	sub_82CA93D0(ctx, base);
	// 82D639E8: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D639EC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82D639F0: 3BA3001C  addi r29, r3, 0x1c
	ctx.r[29].s64 = ctx.r[3].s64 + 28;
	// 82D639F4: 7D445214  add r10, r4, r10
	ctx.r[10].u64 = ctx.r[4].u64 + ctx.r[10].u64;
	// 82D639F8: 3BE00020  li r31, 0x20
	ctx.r[31].s64 = 32;
	// 82D639FC: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63A00: 552907BC  rlwinm r9, r9, 0, 0x1e, 0x1e
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82D63A04: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D63A08: 419A0098  beq cr6, 0x82d63aa0
	if ctx.cr[6].eq {
	pc = 0x82D63AA0; continue 'dispatch;
	}
	// 82D63A0C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63A10: 5568083C  slwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D63A14: 81290004  lwz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D63A18: 7D29422E  lhzx r9, r9, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82D63A1C: 7D290734  extsh r9, r9
	ctx.r[9].s64 = ctx.r[9].s16 as i64;
	// 82D63A20: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 82D63A24: 419A0030  beq cr6, 0x82d63a54
	if ctx.cr[6].eq {
	pc = 0x82D63A54; continue 'dispatch;
	}
	// 82D63A28: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82D63A2C: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63A30: 61290008  ori r9, r9, 8
	ctx.r[9].u64 = ctx.r[9].u64 | 8;
	// 82D63A34: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82D63A38: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63A3C: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D63A40: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63A44: 552907BC  rlwinm r9, r9, 0, 0x1e, 0x1e
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82D63A48: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D63A4C: 409AFFC0  bne cr6, 0x82d63a0c
	if !ctx.cr[6].eq {
	pc = 0x82D63A0C; continue 'dispatch;
	}
	// 82D63A50: 48000050  b 0x82d63aa0
	pc = 0x82D63AA0; continue 'dispatch;
	// 82D63A54: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D63A58: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D63A5C: 81030010  lwz r8, 0x10(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D63A60: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82D63A64: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82D63A68: 55292036  slwi r9, r9, 4
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D63A6C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82D63A70: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D63C28 size=468
    let mut pc: u32 = 0x82D63C28;
    'dispatch: loop {
        match pc {
            0x82D63C28 => {
    //   block [0x82D63C28..0x82D63DFC)
	// 82D63C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D63C2C: 4BF457BD  bl 0x82ca93e8
	ctx.lr = 0x82D63C30;
	sub_82CA93D0(ctx, base);
	// 82D63C30: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D63C34: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82D63C38: 3A9C0029  addi r20, r28, 0x29
	ctx.r[20].s64 = ctx.r[28].s64 + 41;
	// 82D63C3C: 897C0029  lbz r11, 0x29(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(41 as u32) ) } as u64;
	// 82D63C40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D63C44: 409A01B0  bne cr6, 0x82d63df4
	if !ctx.cr[6].eq {
	pc = 0x82D63DF4; continue 'dispatch;
	}
	// 82D63C48: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63C4C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82D63C50: 82AB0010  lwz r21, 0x10(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D63C54: 2F150000  cmpwi cr6, r21, 0
	ctx.cr[6].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 82D63C58: 40990194  ble cr6, 0x82d63dec
	if !ctx.cr[6].gt {
	pc = 0x82D63DEC; continue 'dispatch;
	}
	// 82D63C5C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D63C60: 3ADC001C  addi r22, r28, 0x1c
	ctx.r[22].s64 = ctx.r[28].s64 + 28;
	// 82D63C64: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82D63C68: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82D63C6C: 3B4B4CB0  addi r26, r11, 0x4cb0
	ctx.r[26].s64 = ctx.r[11].s64 + 19632;
	// 82D63C70: 3B600020  li r27, 0x20
	ctx.r[27].s64 = 32;
	// 82D63C74: 3AE00010  li r23, 0x10
	ctx.r[23].s64 = 16;
	// 82D63C78: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63C7C: 7D6BC8AE  lbzx r11, r11, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82D63C80: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82D63C84: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D63C88: 419A0150  beq cr6, 0x82d63dd8
	if ctx.cr[6].eq {
	pc = 0x82D63DD8; continue 'dispatch;
	}
	// 82D63C8C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63C90: 815C0010  lwz r10, 0x10(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D63C94: 7FFD5214  add r31, r29, r10
	ctx.r[31].u64 = ctx.r[29].u64 + ctx.r[10].u64;
	// 82D63C98: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D63C9C: 7D6BC22E  lhzx r11, r11, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82D63CA0: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82D63CA4: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82D63CA8: 419A00FC  beq cr6, 0x82d63da4
	if ctx.cr[6].eq {
	pc = 0x82D63DA4; continue 'dispatch;
	}
	// 82D63CAC: 5568083C  slwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D63CB0: 813C0004  lwz r9, 4(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D63CB4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D63CB8: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82D63CBC: 7FC9EA14  add r30, r9, r29
	ctx.r[30].u64 = ctx.r[9].u64 + ctx.r[29].u64;
	// 82D63CC0: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D63CC4: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D63CC8: 4BFF7411  bl 0x82d5b0d8
	ctx.lr = 0x82D63CCC;
	sub_82D5B0D8(ctx, base);
	// 82D63CCC: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D63E00 size=436
    let mut pc: u32 = 0x82D63E00;
    'dispatch: loop {
        match pc {
            0x82D63E00 => {
    //   block [0x82D63E00..0x82D63FB4)
	// 82D63E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D63E04: 4BF455F5  bl 0x82ca93f8
	ctx.lr = 0x82D63E08;
	sub_82CA93D0(ctx, base);
	// 82D63E08: 89630028  lbz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D63E0C: 3B230028  addi r25, r3, 0x28
	ctx.r[25].s64 = ctx.r[3].s64 + 40;
	// 82D63E10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D63E14: 409A019C  bne cr6, 0x82d63fb0
	if !ctx.cr[6].eq {
	pc = 0x82D63FB0; continue 'dispatch;
	}
	// 82D63E18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63E1C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82D63E20: 834B0010  lwz r26, 0x10(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D63E24: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82D63E28: 40990180  ble cr6, 0x82d63fa8
	if !ctx.cr[6].gt {
	pc = 0x82D63FA8; continue 'dispatch;
	}
	// 82D63E2C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D63E30: 3B63001C  addi r27, r3, 0x1c
	ctx.r[27].s64 = ctx.r[3].s64 + 28;
	// 82D63E34: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D63E38: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82D63E3C: 3BEB4CB0  addi r31, r11, 0x4cb0
	ctx.r[31].s64 = ctx.r[11].s64 + 19632;
	// 82D63E40: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82D63E44: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 82D63E48: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63E4C: 7D6BF0AE  lbzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82D63E50: 556B07BC  rlwinm r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D63E54: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D63E58: 419A013C  beq cr6, 0x82d63f94
	if ctx.cr[6].eq {
	pc = 0x82D63F94; continue 'dispatch;
	}
	// 82D63E5C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63E60: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D63E64: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82D63E68: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D63E6C: 7D4AEA2E  lhzx r10, r10, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82D63E70: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 82D63E74: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82D63E78: 419A00E8  beq cr6, 0x82d63f60
	if ctx.cr[6].eq {
	pc = 0x82D63F60; continue 'dispatch;
	}
	// 82D63E7C: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D63E80: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D63FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D63FB8 size=108
    let mut pc: u32 = 0x82D63FB8;
    'dispatch: loop {
        match pc {
            0x82D63FB8 => {
    //   block [0x82D63FB8..0x82D64024)
	// 82D63FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D63FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D63FC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D63FC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D63FC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D63FCC: 4BFFFC5D  bl 0x82d63c28
	ctx.lr = 0x82D63FD0;
	sub_82D63C28(ctx, base);
	// 82D63FD0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D63FD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D63FD8: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D63FDC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D63FE0: 4099001C  ble cr6, 0x82d63ffc
	if !ctx.cr[6].gt {
	pc = 0x82D63FFC; continue 'dispatch;
	}
	// 82D63FE4: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82D63FE8: 811F001C  lwz r8, 0x1c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D63FEC: 7D2859AE  stbx r9, r8, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 82D63FF0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D63FF4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D63FF8: 4198FFF0  blt cr6, 0x82d63fe8
	if ctx.cr[6].lt {
	pc = 0x82D63FE8; continue 'dispatch;
	}
	// 82D63FFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D64000: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D64004: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82D64008: 995F0028  stb r10, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u8 ) };
	// 82D6400C: 997F0029  stb r11, 0x29(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(41 as u32), ctx.r[11].u8 ) };
	// 82D64010: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D64014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D64018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D6401C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D64020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D64028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D64028 size=104
    let mut pc: u32 = 0x82D64028;
    'dispatch: loop {
        match pc {
            0x82D64028 => {
    //   block [0x82D64028..0x82D64090)
	// 82D64028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6402C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D64030: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D64034: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D64038: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D6403C: 4BFFFDC5  bl 0x82d63e00
	ctx.lr = 0x82D64040;
	sub_82D63E00(ctx, base);
	// 82D64040: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D64044: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D64048: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82D6404C: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D64050: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D64054: 40990018  ble cr6, 0x82d6406c
	if !ctx.cr[6].gt {
	pc = 0x82D6406C; continue 'dispatch;
	}
	// 82D64058: 811F001C  lwz r8, 0x1c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D6405C: 7D2859AE  stbx r9, r8, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 82D64060: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D64064: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D64068: 4198FFF0  blt cr6, 0x82d64058
	if ctx.cr[6].lt {
	pc = 0x82D64058; continue 'dispatch;
	}
	// 82D6406C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D64070: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82D64074: 997F0029  stb r11, 0x29(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(41 as u32), ctx.r[11].u8 ) };
	// 82D64078: 993F0028  stb r9, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[9].u8 ) };
	// 82D6407C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D64080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D64084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D64088: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D6408C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D64090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D64090 size=276
    let mut pc: u32 = 0x82D64090;
    'dispatch: loop {
        match pc {
            0x82D64090 => {
    //   block [0x82D64090..0x82D641A4)
	// 82D64090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D64094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D64098: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D6409C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D640A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D640A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D640A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D640AC: 83CB0010  lwz r30, 0x10(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D640B0: 4BFFFB79  bl 0x82d63c28
	ctx.lr = 0x82D640B4;
	sub_82D63C28(ctx, base);
	// 82D640B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D640B8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82D640BC: 4099009C  ble cr6, 0x82d64158
	if !ctx.cr[6].gt {
	pc = 0x82D64158; continue 'dispatch;
	}
	// 82D640C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82D640C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82D640C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82D640CC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D640D0: 80CA000C  lwz r6, 0xc(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D640D4: 7CC6382E  lwzx r6, r6, r7
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82D640D8: 88C60004  lbz r6, 4(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D640DC: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82D640E0: 419A0018  beq cr6, 0x82d640f8
	if ctx.cr[6].eq {
	pc = 0x82D640F8; continue 'dispatch;
	}
	// 82D640E4: 814A0014  lwz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D640E8: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D641A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D641A8 size=428
    let mut pc: u32 = 0x82D641A8;
    'dispatch: loop {
        match pc {
            0x82D641A8 => {
    //   block [0x82D641A8..0x82D64354)
	// 82D641A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D641AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D641B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D641B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D641B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D641BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D641C0: 83EB0010  lwz r31, 0x10(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D641C4: 4BFFFC3D  bl 0x82d63e00
	ctx.lr = 0x82D641C8;
	sub_82D63E00(ctx, base);
	// 82D641C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82D641CC: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82D641D0: 40990138  ble cr6, 0x82d64308
	if !ctx.cr[6].gt {
	pc = 0x82D64308; continue 'dispatch;
	}
	// 82D641D4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D641D8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82D641DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82D641E0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82D641E4: 388B4CB0  addi r4, r11, 0x4cb0
	ctx.r[4].s64 = ctx.r[11].s64 + 19632;
	// 82D641E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D641EC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D641F0: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D641F4: 7FCA282E  lwzx r30, r10, r5
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 82D641F8: 7D48322E  lhzx r10, r8, r6
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82D641FC: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 82D64200: 891E0004  lbz r8, 4(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D64204: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82D64208: 419A00C0  beq cr6, 0x82d642c8
	if ctx.cr[6].eq {
	pc = 0x82D642C8; continue 'dispatch;
	}
	// 82D6420C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D64210: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82D64214: 7D0B4A14  add r8, r11, r9
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82D64218: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6421C: 419A0080  beq cr6, 0x82d6429c
	if ctx.cr[6].eq {
	pc = 0x82D6429C; continue 'dispatch;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D64358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D64358 size=160
    let mut pc: u32 = 0x82D64358;
    'dispatch: loop {
        match pc {
            0x82D64358 => {
    //   block [0x82D64358..0x82D643F8)
	// 82D64358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6435C: 4BF450B1  bl 0x82ca940c
	ctx.lr = 0x82D64360;
	sub_82CA93D0(ctx, base);
	// 82D64360: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D64364: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D64368: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D6436C: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 82D64370: C00B0C64  lfs f0, 0xc64(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D64374: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D64378: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D643F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D643F8 size=48
    let mut pc: u32 = 0x82D643F8;
    'dispatch: loop {
        match pc {
            0x82D643F8 => {
    //   block [0x82D643F8..0x82D64428)
	// 82D643F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D643FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D64400: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D64404: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D64408: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D6440C: 4BFFF81D  bl 0x82d63c28
	ctx.lr = 0x82D64410;
	sub_82D63C28(ctx, base);
	// 82D64410: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82D64414: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D64418: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D6441C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D64420: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D64424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D64428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D64428 size=36
    let mut pc: u32 = 0x82D64428;
    'dispatch: loop {
        match pc {
            0x82D64428 => {
    //   block [0x82D64428..0x82D6444C)
	// 82D64428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6442C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D64430: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D64434: 4BFFF9CD  bl 0x82d63e00
	ctx.lr = 0x82D64438;
	sub_82D63E00(ctx, base);
	// 82D64438: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82D6443C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D64440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D64444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D64448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D64450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D64450 size=1068
    let mut pc: u32 = 0x82D64450;
    'dispatch: loop {
        match pc {
            0x82D64450 => {
    //   block [0x82D64450..0x82D6487C)
	// 82D64450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D64454: 4BF44F8D  bl 0x82ca93e0
	ctx.lr = 0x82D64458;
	sub_82CA93D0(ctx, base);
	// 82D64458: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6445C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D64460: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82D64464: 3B1F001C  addi r24, r31, 0x1c
	ctx.r[24].s64 = ctx.r[31].s64 + 28;
	// 82D64468: 3AB90001  addi r21, r25, 1
	ctx.r[21].s64 = ctx.r[25].s64 + 1;
	// 82D6446C: 3B400020  li r26, 0x20
	ctx.r[26].s64 = 32;
	// 82D64470: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D64474: 7EBCAB78  mr r28, r21
	ctx.r[28].u64 = ctx.r[21].u64;
	// 82D64478: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D6447C: 3A800010  li r20, 0x10
	ctx.r[20].s64 = 16;
	// 82D64480: 3A400000  li r18, 0
	ctx.r[18].s64 = 0;
	// 82D64484: 82CA0010  lwz r22, 0x10(r10)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D64488: 7D4BC8AE  lbzx r10, r11, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82D6448C: 7F1CB000  cmpw cr6, r28, r22
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[22].s32, &mut ctx.xer);
	// 82D64490: 614A0004  ori r10, r10, 4
	ctx.r[10].u64 = ctx.r[10].u64 | 4;
	// 82D64494: 7D4BC9AE  stbx r10, r11, r25
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[25].u32), ctx.r[10].u8) };
	// 82D64498: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D6449C: 3A6B4CB0  addi r19, r11, 0x4cb0
	ctx.r[19].s64 = ctx.r[11].s64 + 19632;
	// 82D644A0: 409801D0  bge cr6, 0x82d64670
	if !ctx.cr[6].lt {
	pc = 0x82D64670; continue 'dispatch;
	}
	// 82D644A4: 578B083C  slwi r11, r28, 1
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D644A8: 5797083C  slwi r23, r28, 1
	ctx.r[23].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[23].u64 = ctx.r[23].u32 as u64;
	// 82D644AC: 7D7C5A14  add r11, r28, r11
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[11].u64;
	// 82D644B0: 557B2036  slwi r27, r11, 4
	ctx.r[27].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82D644B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D644B8: 81380000  lwz r9, 0(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D644BC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D644C0: 7D4BBA2E  lhzx r10, r11, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82D644C4: 7D4B0734  extsh r11, r10
	ctx.r[11].s64 = ctx.r[10].s16 as i64;
	// 82D644C8: 7D6B48AE  lbzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82D644CC: 556B077A  rlwinm r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D644D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D644D4: 419A0188  beq cr6, 0x82d6465c
	if ctx.cr[6].eq {
	pc = 0x82D6465C; continue 'dispatch;
	}
	// 82D644D8: 7D69E0AE  lbzx r11, r9, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82D644DC: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82D644E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D644E4: 419A0164  beq cr6, 0x82d64648
	if ctx.cr[6].eq {
	pc = 0x82D64648; continue 'dispatch;
	}
	// 82D644E8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D644EC: 7D440734  extsh r4, r10
	ctx.r[4].s64 = ctx.r[10].s16 as i64;
	// 82D644F0: 7FDB5A14  add r30, r27, r11
	ctx.r[30].u64 = ctx.r[27].u64 + ctx.r[11].u64;
	// 82D644F4: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 82D644F8: 419A011C  beq cr6, 0x82d64614
	if ctx.cr[6].eq {
	pc = 0x82D64614; continue 'dispatch;
	}
	// 82D644FC: 7D4448AE  lbzx r10, r4, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82D64500: 554A07BC  rlwinm r10, r10, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82D64504: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D64508: 409A0018  bne cr6, 0x82d64520
	if !ctx.cr[6].eq {
	pc = 0x82D64520; continue 'dispatch;
	}
	// 82D6450C: 548A083C  slwi r10, r4, 1
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D64510: 7D445214  add r10, r4, r10
	ctx.r[10].u64 = ctx.r[4].u64 + ctx.r[10].u64;
	// 82D64514: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D64518: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82D6451C: 48000010  b 0x82d6452c
	pc = 0x82D6452C; continue 'dispatch;
	// 82D64520: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D64524: 4BFFF4BD  bl 0x82d639e0
	ctx.lr = 0x82D64528;
	sub_82D639E0(ctx, base);
	// 82D64528: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82D6452C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D64530: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D64534: 7FABDA14  add r29, r11, r27
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82D64538: 4BFF6BA1  bl 0x82d5b0d8
	ctx.lr = 0x82D6453C;
	sub_82D5B0D8(ctx, base);
	// 82D6453C: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D64880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D64880 size=816
    let mut pc: u32 = 0x82D64880;
    'dispatch: loop {
        match pc {
            0x82D64880 => {
    //   block [0x82D64880..0x82D64BB0)
	// 82D64880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D64884: 4BF44B59  bl 0x82ca93dc
	ctx.lr = 0x82D64888;
	sub_82CA93D0(ctx, base);
	// 82D64888: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6488C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D64890: 7C942378  mr r20, r4
	ctx.r[20].u64 = ctx.r[4].u64;
	// 82D64894: 3A200001  li r17, 1
	ctx.r[17].s64 = 1;
	// 82D64898: 3A400000  li r18, 0
	ctx.r[18].s64 = 0;
	// 82D6489C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82D648A0: 409A0070  bne cr6, 0x82d64910
	if !ctx.cr[6].eq {
	pc = 0x82D64910; continue 'dispatch;
	}
	// 82D648A4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D648A8: 38940001  addi r4, r20, 1
	ctx.r[4].s64 = ctx.r[20].s64 + 1;
	// 82D648AC: 83AB0010  lwz r29, 0x10(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D648B0: 7F04E800  cmpw cr6, r4, r29
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82D648B4: 409802B4  bge cr6, 0x82d64b68
	if !ctx.cr[6].lt {
	pc = 0x82D64B68; continue 'dispatch;
	}
	// 82D648B8: 549F083C  slwi r31, r4, 1
	ctx.r[31].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82D648BC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D648C0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D648C4: 7D6BFA2E  lhzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82D648C8: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82D648CC: 7F0BA000  cmpw cr6, r11, r20
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[20].s32, &mut ctx.xer);
	// 82D648D0: 409A002C  bne cr6, 0x82d648fc
	if !ctx.cr[6].eq {
	pc = 0x82D648FC; continue 'dispatch;
	}
	// 82D648D4: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D648D8: 7D6B20AE  lbzx r11, r11, r4
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82D648DC: 556B07BC  rlwinm r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D648E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D648E4: 419A000C  beq cr6, 0x82d648f0
	if ctx.cr[6].eq {
	pc = 0x82D648F0; continue 'dispatch;
	}
	// 82D648E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D648EC: 4BFFF0F5  bl 0x82d639e0
	ctx.lr = 0x82D648F0;
	sub_82D639E0(ctx, base);
	// 82D648F0: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D648F4: 7E2B21AE  stbx r17, r11, r4
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), ctx.r[17].u8) };
	// 82D648F8: 9A5E0029  stb r18, 0x29(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(41 as u32), ctx.r[18].u8 ) };
	// 82D648FC: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82D64900: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82D64904: 7F04E800  cmpw cr6, r4, r29
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82D64908: 4198FFB4  blt cr6, 0x82d648bc
	if ctx.cr[6].lt {
	pc = 0x82D648BC; continue 'dispatch;
	}
	// 82D6490C: 4800025C  b 0x82d64b68
	pc = 0x82D64B68; continue 'dispatch;
	// 82D64910: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D64914: 3A740001  addi r19, r20, 1
	ctx.r[19].s64 = ctx.r[20].s64 + 1;
	// 82D64918: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D6491C: 3B3E001C  addi r25, r30, 0x1c
	ctx.r[25].s64 = ctx.r[30].s64 + 28;
	// 82D64920: 7E7B9B78  mr r27, r19
	ctx.r[27].u64 = ctx.r[19].u64;
	// 82D64924: 82AA0010  lwz r21, 0x10(r10)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D64928: 7D4BA0AE  lbzx r10, r11, r20
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 82D6492C: 7F1BA800  cmpw cr6, r27, r21
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[21].s32, &mut ctx.xer);
	// 82D64930: 614A0004  ori r10, r10, 4
	ctx.r[10].u64 = ctx.r[10].u64 | 4;
	// 82D64934: 7D4BA1AE  stbx r10, r11, r20
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[20].u32), ctx.r[10].u8) };
	// 82D64938: 409801E0  bge cr6, 0x82d64b18
	if !ctx.cr[6].lt {
	pc = 0x82D64B18; continue 'dispatch;
	}
	// 82D6493C: 576B083C  slwi r11, r27, 1
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D64940: 5778083C  slwi r24, r27, 1
	ctx.r[24].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 82D64944: 7D7B5A14  add r11, r27, r11
	ctx.r[11].u64 = ctx.r[27].u64 + ctx.r[11].u64;
	// 82D64948: 3B400020  li r26, 0x20
	ctx.r[26].s64 = 32;
	// 82D6494C: 557C2036  slwi r28, r11, 4
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82D64950: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D64954: 3AC00010  li r22, 0x10
	ctx.r[22].s64 = 16;
	// 82D64958: 3AEB4CB0  addi r23, r11, 0x4cb0
	ctx.r[23].s64 = ctx.r[11].s64 + 19632;
	// 82D6495C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D64960: 81390000  lwz r9, 0(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D64964: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D64968: 7D4BC22E  lhzx r10, r11, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82D6496C: 7D4B0734  extsh r11, r10
	ctx.r[11].s64 = ctx.r[10].s16 as i64;
	// 82D64970: 7D6B48AE  lbzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82D64974: 556B077A  rlwinm r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D64978: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D6497C: 419A0188  beq cr6, 0x82d64b04
	if ctx.cr[6].eq {
	pc = 0x82D64B04; continue 'dispatch;
	}
	// 82D64980: 7D69D8AE  lbzx r11, r9, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82D64984: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82D64988: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D6498C: 419A0164  beq cr6, 0x82d64af0
	if ctx.cr[6].eq {
	pc = 0x82D64AF0; continue 'dispatch;
	}
	// 82D64990: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D64994: 7D440734  extsh r4, r10
	ctx.r[4].s64 = ctx.r[10].s16 as i64;
	// 82D64998: 7FEBE214  add r31, r11, r28
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82D6499C: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 82D649A0: 419A011C  beq cr6, 0x82d64abc
	if ctx.cr[6].eq {
	pc = 0x82D64ABC; continue 'dispatch;
	}
	// 82D649A4: 7D4920AE  lbzx r10, r9, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82D649A8: 554A07BC  rlwinm r10, r10, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82D649AC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D649B0: 409A0018  bne cr6, 0x82d649c8
	if !ctx.cr[6].eq {
	pc = 0x82D649C8; continue 'dispatch;
	}
	// 82D649B4: 548A083C  slwi r10, r4, 1
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D649B8: 7D445214  add r10, r4, r10
	ctx.r[10].u64 = ctx.r[4].u64 + ctx.r[10].u64;
	// 82D649BC: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D649C0: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82D649C4: 48000010  b 0x82d649d4
	pc = 0x82D649D4; continue 'dispatch;
	// 82D649C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D649CC: 4BFFF015  bl 0x82d639e0
	ctx.lr = 0x82D649D0;
	sub_82D639E0(ctx, base);
	// 82D649D0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82D649D4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D649D8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D649DC: 7FABE214  add r29, r11, r28
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82D649E0: 4BFF66F9  bl 0x82d5b0d8
	ctx.lr = 0x82D649E4;
	sub_82D5B0D8(ctx, base);
	// 82D649E4: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D64BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D64BB0 size=12
    let mut pc: u32 = 0x82D64BB0;
    'dispatch: loop {
        match pc {
            0x82D64BB0 => {
    //   block [0x82D64BB0..0x82D64BBC)
	// 82D64BB0: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D64BB4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D64BB8: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D64BBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D64BBC size=80
    let mut pc: u32 = 0x82D64BBC;
    'dispatch: loop {
        match pc {
            0x82D64BBC => {
    //   block [0x82D64BBC..0x82D64C0C)
	// 82D64BBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D64BC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D64BC4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82D64BC8: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D64BCC: 80C3000C  lwz r6, 0xc(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D64BD0: 7CA8522E  lhzx r5, r8, r10
	ctx.r[5].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D64BD4: 7D0B302E  lwzx r8, r11, r6
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82D64BD8: 7CA60734  extsh r6, r5
	ctx.r[6].s64 = ctx.r[5].s16 as i64;
	// 82D64BDC: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82D64BE0: 41990010  bgt cr6, 0x82d64bf0
	if ctx.cr[6].gt {
	pc = 0x82D64BF0; continue 'dispatch;
	}
	// 82D64BE4: 7C860774  extsb r6, r4
	ctx.r[6].s64 = ctx.r[4].s8 as i64;
	// 82D64BE8: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82D64BEC: 409A0008  bne cr6, 0x82d64bf4
	if !ctx.cr[6].eq {
	pc = 0x82D64BF4; continue 'dispatch;
	}
	// 82D64BF0: 98E80004  stb r7, 4(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[7].u8 ) };
	// 82D64BF4: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82D64BF8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D64BFC: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82D64C00: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D64C04: 409AFFC4  bne cr6, 0x82d64bc8
	if !ctx.cr[6].eq {
	pc = 0x82D64BC8; continue 'dispatch;
	}
	// 82D64C08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D64C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D64C10 size=140
    let mut pc: u32 = 0x82D64C10;
    'dispatch: loop {
        match pc {
            0x82D64C10 => {
    //   block [0x82D64C10..0x82D64C9C)
	// 82D64C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D64C14: 4BF447ED  bl 0x82ca9400
	ctx.lr = 0x82D64C18;
	sub_82CA93D0(ctx, base);
	// 82D64C18: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D64C1C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82D64C20: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D64C24: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82D64C28: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D64C2C: 837C0010  lwz r27, 0x10(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D64C30: 409A000C  bne cr6, 0x82d64c3c
	if !ctx.cr[6].eq {
	pc = 0x82D64C3C; continue 'dispatch;
	}
	// 82D64C34: 3D6082D6  lis r11, -0x7d2a
	ctx.r[11].s64 = -2099904512;
	// 82D64C38: 3BCB8A10  addi r30, r11, -0x75f0
	ctx.r[30].s64 = ctx.r[11].s64 + -30192;
	// 82D64C3C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82D64C40: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82D64C44: 40990040  ble cr6, 0x82d64c84
	if !ctx.cr[6].gt {
	pc = 0x82D64C84; continue 'dispatch;
	}
	// 82D64C48: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82D64C4C: 817C000C  lwz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D64C50: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D64C54: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82D64C58: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D64C5C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D64C60: 7FC903A6  mtctr r30
	ctx.ctr.u64 = ctx.r[30].u64;
	// 82D64C64: 4E800421  bctrl
	ctx.lr = 0x82D64C68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D64C68: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D64C6C: 419A0024  beq cr6, 0x82d64c90
	if ctx.cr[6].eq {
	pc = 0x82D64C90; continue 'dispatch;
	}
	// 82D64C70: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 82D64C74: 7D7D0734  extsh r29, r11
	ctx.r[29].s64 = ctx.r[11].s16 as i64;
	// 82D64C78: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82D64C7C: 7F1FD800  cmpw cr6, r31, r27
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82D64C80: 4198FFCC  blt cr6, 0x82d64c4c
	if ctx.cr[6].lt {
	pc = 0x82D64C4C; continue 'dispatch;
	}
	// 82D64C84: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82D64C88: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D64C8C: 4BF447C4  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 82D64C90: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D64C94: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D64C98: 4BF447B8  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D64CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D64CA0 size=12
    let mut pc: u32 = 0x82D64CA0;
    'dispatch: loop {
        match pc {
            0x82D64CA0 => {
    //   block [0x82D64CA0..0x82D64CAC)
	// 82D64CA0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D64CA4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D64CA8: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D64CAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D64CAC size=40
    let mut pc: u32 = 0x82D64CAC;
    'dispatch: loop {
        match pc {
            0x82D64CAC => {
    //   block [0x82D64CAC..0x82D64CD4)
	// 82D64CAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82D64CB0: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82D64CB4: 8103000C  lwz r8, 0xc(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D64CB8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D64CBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D64CC0: 7D0A402E  lwzx r8, r10, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82D64CC4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82D64CC8: 99280004  stb r9, 4(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u8 ) };
	// 82D64CCC: 409AFFE8  bne cr6, 0x82d64cb4
	if !ctx.cr[6].eq {
	pc = 0x82D64CB4; continue 'dispatch;
	}
	// 82D64CD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D64CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D64CD8 size=76
    let mut pc: u32 = 0x82D64CD8;
    'dispatch: loop {
        match pc {
            0x82D64CD8 => {
    //   block [0x82D64CD8..0x82D64D24)
	// 82D64CD8: 81050008  lwz r8, 8(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D64CDC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D64CE0: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82D64CE4: 40990034  ble cr6, 0x82d64d18
	if !ctx.cr[6].gt {
	pc = 0x82D64D18; continue 'dispatch;
	}
	// 82D64CE8: 81450004  lwz r10, 4(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D64CEC: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D64CF0: 7D290734  extsh r9, r9
	ctx.r[9].s64 = ctx.r[9].s16 as i64;
	// 82D64CF4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D64CF8: 4198002C  blt cr6, 0x82d64d24
	if ctx.cr[6].lt {
		sub_82D64D24(ctx, base);
		return;
	}
	// 82D64CFC: 80E40010  lwz r7, 0x10(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D64D00: 7F093800  cmpw cr6, r9, r7
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82D64D04: 40980020  bge cr6, 0x82d64d24
	if !ctx.cr[6].lt {
		sub_82D64D24(ctx, base);
		return;
	}
	// 82D64D08: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D64D0C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82D64D10: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82D64D14: 4198FFD8  blt cr6, 0x82d64cec
	if ctx.cr[6].lt {
	pc = 0x82D64CEC; continue 'dispatch;
	}
	// 82D64D18: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D64D1C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D64D20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D64D24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D64D24 size=12
    let mut pc: u32 = 0x82D64D24;
    'dispatch: loop {
        match pc {
            0x82D64D24 => {
    //   block [0x82D64D24..0x82D64D30)
	// 82D64D24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D64D28: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D64D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D64D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D64D30 size=12
    let mut pc: u32 = 0x82D64D30;
    'dispatch: loop {
        match pc {
            0x82D64D30 => {
    //   block [0x82D64D30..0x82D64D3C)
	// 82D64D30: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D64D34: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D64D38: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D64D3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D64D3C size=64
    let mut pc: u32 = 0x82D64D3C;
    'dispatch: loop {
        match pc {
            0x82D64D3C => {
    //   block [0x82D64D3C..0x82D64D7C)
	// 82D64D3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D64D40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D64D44: 8103000C  lwz r8, 0xc(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D64D48: 7D0A402E  lwzx r8, r10, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82D64D4C: 89080004  lbz r8, 4(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D64D50: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82D64D54: 419A0010  beq cr6, 0x82d64d64
	if ctx.cr[6].eq {
	pc = 0x82D64D64; continue 'dispatch;
	}
	// 82D64D58: 81030014  lwz r8, 0x14(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D64D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D64D80 size=16
    let mut pc: u32 = 0x82D64D80;
    'dispatch: loop {
        match pc {
            0x82D64D80 => {
    //   block [0x82D64D80..0x82D64D90)
	// 82D64D80: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D64D84: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82D64D88: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D64D8C: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D64D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D64D90 size=60
    let mut pc: u32 = 0x82D64D90;
    'dispatch: loop {
        match pc {
            0x82D64D90 => {
    //   block [0x82D64D90..0x82D64DCC)
	// 82D64D90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D64D94: 7D0B2050  subf r8, r11, r4
	ctx.r[8].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 82D64D98: 80E3000C  lwz r7, 0xc(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D64D9C: 7CEA382E  lwzx r7, r10, r7
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82D64DA0: 88E70004  lbz r7, 4(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D64DA4: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82D64DA8: 419A000C  beq cr6, 0x82d64db4
	if ctx.cr[6].eq {
	pc = 0x82D64DB4; continue 'dispatch;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D64DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D64DD0 size=88
    let mut pc: u32 = 0x82D64DD0;
    'dispatch: loop {
        match pc {
            0x82D64DD0 => {
    //   block [0x82D64DD0..0x82D64E28)
	// 82D64DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D64DD4: 4BF44631  bl 0x82ca9404
	ctx.lr = 0x82D64DD8;
	sub_82CA93D0(ctx, base);
	// 82D64DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D64DDC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D64DE0: 419A0040  beq cr6, 0x82d64e20
	if ctx.cr[6].eq {
	pc = 0x82D64E20; continue 'dispatch;
	}
	// 82D64DE4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82D64DE8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82D64DEC: 7F853850  subf r28, r5, r7
	ctx.r[28].s64 = ctx.r[7].s64 - ctx.r[5].s64;
	// 82D64DF0: 7F652050  subf r27, r5, r4
	ctx.r[27].s64 = ctx.r[4].s64 - ctx.r[5].s64;
	// 82D64DF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D64DF8: 7CDCFA14  add r6, r28, r31
	ctx.r[6].u64 = ctx.r[28].u64 + ctx.r[31].u64;
	// 82D64DFC: C03D0000  lfs f1, 0(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82D64E00: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D64E04: 7C7BFA14  add r3, r27, r31
	ctx.r[3].u64 = ctx.r[27].u64 + ctx.r[31].u64;
	// 82D64E08: 48001061  bl 0x82d65e68
	ctx.lr = 0x82D64E0C;
	sub_82D65E68(ctx, base);
	// 82D64E0C: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82D64E10: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82D64E14: 3BFF0030  addi r31, r31, 0x30
	ctx.r[31].s64 = ctx.r[31].s64 + 48;
	// 82D64E18: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D64E1C: 409AFFDC  bne cr6, 0x82d64df8
	if !ctx.cr[6].eq {
	pc = 0x82D64DF8; continue 'dispatch;
	}
	// 82D64E20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D64E24: 4BF44630  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D64E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D64E28 size=92
    let mut pc: u32 = 0x82D64E28;
    'dispatch: loop {
        match pc {
            0x82D64E28 => {
    //   block [0x82D64E28..0x82D64E84)
	// 82D64E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D64E2C: 4BF445DD  bl 0x82ca9408
	ctx.lr = 0x82D64E30;
	sub_82CA93D0(ctx, base);
	// 82D64E30: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82D64E34: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D64E38: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82D64E3C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D64E40: 419A0038  beq cr6, 0x82d64e78
	if ctx.cr[6].eq {
	pc = 0x82D64E78; continue 'dispatch;
	}
	// 82D64E44: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82D64E48: 7FA53850  subf r29, r5, r7
	ctx.r[29].s64 = ctx.r[7].s64 - ctx.r[5].s64;
	// 82D64E4C: 7F852050  subf r28, r5, r4
	ctx.r[28].s64 = ctx.r[4].s64 - ctx.r[5].s64;
	// 82D64E50: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D64E54: 7CDDFA14  add r6, r29, r31
	ctx.r[6].u64 = ctx.r[29].u64 + ctx.r[31].u64;
	// 82D64E58: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82D64E5C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D64E60: 7C7CFA14  add r3, r28, r31
	ctx.r[3].u64 = ctx.r[28].u64 + ctx.r[31].u64;
	// 82D64E64: 48001005  bl 0x82d65e68
	ctx.lr = 0x82D64E68;
	sub_82D65E68(ctx, base);
	// 82D64E68: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82D64E6C: 3BFF0030  addi r31, r31, 0x30
	ctx.r[31].s64 = ctx.r[31].s64 + 48;
	// 82D64E70: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D64E74: 409AFFE0  bne cr6, 0x82d64e54
	if !ctx.cr[6].eq {
	pc = 0x82D64E54; continue 'dispatch;
	}
	// 82D64E78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D64E7C: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82D64E80: 4BF445D8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D64E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D64E88 size=116
    let mut pc: u32 = 0x82D64E88;
    'dispatch: loop {
        match pc {
            0x82D64E88 => {
    //   block [0x82D64E88..0x82D64EFC)
	// 82D64E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D64E8C: 4BF44579  bl 0x82ca9404
	ctx.lr = 0x82D64E90;
	sub_82CA93D0(ctx, base);
	// 82D64E90: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82D64E94: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D64E98: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82D64E9C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82D64EA0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82D64EA4: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82D64EA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D64EAC: 419A0044  beq cr6, 0x82d64ef0
	if ctx.cr[6].eq {
	pc = 0x82D64EF0; continue 'dispatch;
	}
	// 82D64EB0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D64EB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D64EB8: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D64EBC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82D64EC0: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82D64EC4: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D64EC8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D64ECC: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D64ED0: 7CCBDA14  add r6, r11, r27
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82D64ED4: 7C8BE214  add r4, r11, r28
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82D64ED8: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82D64EDC: 48000F8D  bl 0x82d65e68
	ctx.lr = 0x82D64EE0;
	sub_82D65E68(ctx, base);
	// 82D64EE0: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82D64EE4: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 82D64EE8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D64EEC: 409AFFCC  bne cr6, 0x82d64eb8
	if !ctx.cr[6].eq {
	pc = 0x82D64EB8; continue 'dispatch;
	}
	// 82D64EF0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D64EF4: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82D64EF8: 4BF4455C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D64F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D64F00 size=52
    let mut pc: u32 = 0x82D64F00;
    'dispatch: loop {
        match pc {
            0x82D64F00 => {
    //   block [0x82D64F00..0x82D64F34)
	// 82D64F00: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D64F04: 2F05FFFF  cmpwi cr6, r5, -1
	ctx.cr[6].compare_i32(ctx.r[5].s32, -1, &mut ctx.xer);
	// 82D64F08: C00B0C14  lfs f0, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D64F0C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D64F10: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82D64F14: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 82D64F18: D001FFF8  stfs f0, -8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82D64F1C: C00B0C18  lfs f0, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D64F20: D001FFFC  stfs f0, -4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82D64F24: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D64F34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D64F34 size=64
    let mut pc: u32 = 0x82D64F34;
    'dispatch: loop {
        match pc {
            0x82D64F34 => {
    //   block [0x82D64F34..0x82D64F74)
	// 82D64F34: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82D64F38: 54AB083C  slwi r11, r5, 1
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D64F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D64F78 size=12
    let mut pc: u32 = 0x82D64F78;
    'dispatch: loop {
        match pc {
            0x82D64F78 => {
    //   block [0x82D64F78..0x82D64F84)
	// 82D64F78: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82D64F7C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D64F80: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D64F84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D64F84 size=92
    let mut pc: u32 = 0x82D64F84;
    'dispatch: loop {
        match pc {
            0x82D64F84 => {
    //   block [0x82D64F84..0x82D64FE0)
	// 82D64F84: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82D64F88: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82D64F8C: C0090C14  lfs f0, 0xc14(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D64FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D64FE0 size=724
    let mut pc: u32 = 0x82D64FE0;
    'dispatch: loop {
        match pc {
            0x82D64FE0 => {
    //   block [0x82D64FE0..0x82D652B4)
	// 82D64FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D64FE4: 4BF44409  bl 0x82ca93ec
	ctx.lr = 0x82D64FE8;
	sub_82CA93D0(ctx, base);
	// 82D64FE8: 9421FD10  stwu r1, -0x2f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-752 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D64FEC: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 82D64FF0: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82D64FF4: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82D64FF8: 7CB52B78  mr r21, r5
	ctx.r[21].u64 = ctx.r[5].u64;
	// 82D64FFC: 81760004  lwz r11, 4(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D65000: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D65004: 419A0248  beq cr6, 0x82d6524c
	if ctx.cr[6].eq {
	pc = 0x82D6524C; continue 'dispatch;
	}
	// 82D65008: 81380010  lwz r9, 0x10(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6500C: 81760008  lwz r11, 8(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D65010: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D65014: 409A0238  bne cr6, 0x82d6524c
	if !ctx.cr[6].eq {
	pc = 0x82D6524C; continue 'dispatch;
	}
	// 82D65018: 8178001C  lwz r11, 0x1c(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D6501C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D65020: 419A006C  beq cr6, 0x82d6508c
	if ctx.cr[6].eq {
	pc = 0x82D6508C; continue 'dispatch;
	}
	// 82D65024: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82D65028: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82D6502C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D65030: 4BFF29B9  bl 0x82d579e8
	ctx.lr = 0x82D65034;
	sub_82D579E8(ctx, base);
	// 82D65034: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D65038: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D6503C: 388B6388  addi r4, r11, 0x6388
	ctx.r[4].s64 = ctx.r[11].s64 + 25480;
	// 82D65040: 4BFF2DB1  bl 0x82d57df0
	ctx.lr = 0x82D65044;
	sub_82D57DF0(ctx, base);
	// 82D65044: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D65048: 3CA0654E  lis r5, 0x654e
	ctx.r[5].s64 = 1699610624;
	// 82D6504C: 3900016E  li r8, 0x16e
	ctx.r[8].s64 = 366;
	// 82D65050: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
	// 82D65054: 60A53437  ori r5, r5, 0x3437
	ctx.r[5].u64 = ctx.r[5].u64 | 13367;
	// 82D65058: 806B7630  lwz r3, 0x7630(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30256 as u32) ) } as u64;
	// 82D6505C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D65060: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82D65064: 38EB636C  addi r7, r11, 0x636c
	ctx.r[7].s64 = ctx.r[11].s64 + 25452;
	// 82D65068: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6506C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D65070: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D65074: 4E800421  bctrl
	ctx.lr = 0x82D65078;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D65078: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D6507C: 4BFF33B5  bl 0x82d58430
	ctx.lr = 0x82D65080;
	sub_82D58430(ctx, base);
	// 82D65080: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D65084: 382102F0  addi r1, r1, 0x2f0
	ctx.r[1].s64 = ctx.r[1].s64 + 752;
	// 82D65088: 4BF443B4  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
	// 82D6508C: 817A0010  lwz r11, 0x10(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D65090: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82D65094: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D65098: 40990028  ble cr6, 0x82d650c0
	if !ctx.cr[6].gt {
	pc = 0x82D650C0; continue 'dispatch;
	}
	// 82D6509C: 815A000C  lwz r10, 0xc(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D650A0: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D650A4: 81080000  lwz r8, 0(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D650A8: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82D650AC: 419A00C0  beq cr6, 0x82d6516c
	if ctx.cr[6].eq {
	pc = 0x82D6516C; continue 'dispatch;
	}
	// 82D650B0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82D650B4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82D650B8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D650BC: 4198FFE4  blt cr6, 0x82d650a0
	if ctx.cr[6].lt {
	pc = 0x82D650A0; continue 'dispatch;
	}
	// 82D650C0: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82D650C4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D650C8: 40990098  ble cr6, 0x82d65160
	if !ctx.cr[6].gt {
	pc = 0x82D65160; continue 'dispatch;
	}
	// 82D650CC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82D650D0: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82D650D4: 81580018  lwz r10, 0x18(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D650D8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82D650DC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82D650E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D650E4: 7D4AC82E  lwzx r10, r10, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82D650E8: 83AA0000  lwz r29, 0(r10)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D650EC: 40990050  ble cr6, 0x82d6513c
	if !ctx.cr[6].gt {
	pc = 0x82D6513C; continue 'dispatch;
	}
	// 82D650F0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82D650F4: 817A000C  lwz r11, 0xc(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D650F8: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D650FC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D65100: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D65104: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D65108: 7EA903A6  mtctr r21
	ctx.ctr.u64 = ctx.r[21].u64;
	// 82D6510C: 4E800421  bctrl
	ctx.lr = 0x82D65110;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D65110: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D65114: 409A0010  bne cr6, 0x82d65124
	if !ctx.cr[6].eq {
	pc = 0x82D65124; continue 'dispatch;
	}
	// 82D65118: 81760004  lwz r11, 4(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6511C: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82D65120: 7FDB5B2E  sthx r30, r27, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[11].u32), ctx.r[30].u16) };
	// 82D65124: 395F0001  addi r10, r31, 1
	ctx.r[10].s64 = ctx.r[31].s64 + 1;
	// 82D65128: 817A0010  lwz r11, 0x10(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6512C: 7D5E0734  extsh r30, r10
	ctx.r[30].s64 = ctx.r[10].s16 as i64;
	// 82D65130: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82D65134: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D65138: 4198FFBC  blt cr6, 0x82d650f4
	if ctx.cr[6].lt {
	pc = 0x82D650F4; continue 'dispatch;
	}
	// 82D6513C: 578A063E  clrlwi r10, r28, 0x18
	ctx.r[10].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 82D65140: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D65144: 419A0098  beq cr6, 0x82d651dc
	if ctx.cr[6].eq {
	pc = 0x82D651DC; continue 'dispatch;
	}
	// 82D65148: 81580010  lwz r10, 0x10(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6514C: 3AF70001  addi r23, r23, 1
	ctx.r[23].s64 = ctx.r[23].s64 + 1;
	// 82D65150: 3B390004  addi r25, r25, 4
	ctx.r[25].s64 = ctx.r[25].s64 + 4;
	// 82D65154: 3B7B0002  addi r27, r27, 2
	ctx.r[27].s64 = ctx.r[27].s64 + 2;
	// 82D65158: 7F175000  cmpw cr6, r23, r10
	ctx.cr[6].compare_i32(ctx.r[23].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D6515C: 4198FF78  blt cr6, 0x82d650d4
	if ctx.cr[6].lt {
	pc = 0x82D650D4; continue 'dispatch;
	}
	// 82D65160: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D65164: 382102F0  addi r1, r1, 0x2f0
	ctx.r[1].s64 = ctx.r[1].s64 + 752;
	// 82D65168: 4BF442D4  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
	// 82D6516C: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82D65170: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82D65174: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82D65178: 4BFF2871  bl 0x82d579e8
	ctx.lr = 0x82D6517C;
	sub_82D579E8(ctx, base);
	// 82D6517C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D65180: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82D65184: 388B6354  addi r4, r11, 0x6354
	ctx.r[4].s64 = ctx.r[11].s64 + 25428;
	// 82D65188: 4BFF2C69  bl 0x82d57df0
	ctx.lr = 0x82D6518C;
	sub_82D57DF0(ctx, base);
	// 82D6518C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D65190: 4BFF2D99  bl 0x82d57f28
	ctx.lr = 0x82D65194;
	sub_82D57F28(ctx, base);
	// 82D65194: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D65198: 3CA0654E  lis r5, 0x654e
	ctx.r[5].s64 = 1699610624;
	// 82D6519C: 39000178  li r8, 0x178
	ctx.r[8].s64 = 376;
	// 82D651A0: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
	// 82D651A4: 60A53437  ori r5, r5, 0x3437
	ctx.r[5].u64 = ctx.r[5].u64 | 13367;
	// 82D651A8: 806B7630  lwz r3, 0x7630(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30256 as u32) ) } as u64;
	// 82D651AC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D651B0: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82D651B4: 38EB636C  addi r7, r11, 0x636c
	ctx.r[7].s64 = ctx.r[11].s64 + 25452;
	// 82D651B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D651BC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D651C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D651C4: 4E800421  bctrl
	ctx.lr = 0x82D651C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D651C8: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82D651CC: 4BFF3265  bl 0x82d58430
	ctx.lr = 0x82D651D0;
	sub_82D58430(ctx, base);
	// 82D651D0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D651D4: 382102F0  addi r1, r1, 0x2f0
	ctx.r[1].s64 = ctx.r[1].s64 + 752;
	// 82D651D8: 4BF44264  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
	// 82D651DC: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82D651E0: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82D651E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D651E8: 4BFF2801  bl 0x82d579e8
	ctx.lr = 0x82D651EC;
	sub_82D579E8(ctx, base);
	// 82D651EC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D651F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D651F4: 388B6340  addi r4, r11, 0x6340
	ctx.r[4].s64 = ctx.r[11].s64 + 25408;
	// 82D651F8: 4BFF2BF9  bl 0x82d57df0
	ctx.lr = 0x82D651FC;
	sub_82D57DF0(ctx, base);
	// 82D651FC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D65200: 4BFF2BF1  bl 0x82d57df0
	ctx.lr = 0x82D65204;
	sub_82D57DF0(ctx, base);
	// 82D65204: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D65208: 3CA0654E  lis r5, 0x654e
	ctx.r[5].s64 = 1699610624;
	// 82D6520C: 3900018E  li r8, 0x18e
	ctx.r[8].s64 = 398;
	// 82D65210: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
	// 82D65214: 60A53438  ori r5, r5, 0x3438
	ctx.r[5].u64 = ctx.r[5].u64 | 13368;
	// 82D65218: 806B7630  lwz r3, 0x7630(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30256 as u32) ) } as u64;
	// 82D6521C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D65220: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82D65224: 38EB636C  addi r7, r11, 0x636c
	ctx.r[7].s64 = ctx.r[11].s64 + 25452;
	// 82D65228: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6522C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D65230: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D65234: 4E800421  bctrl
	ctx.lr = 0x82D65238;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D65238: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D6523C: 4BFF31F5  bl 0x82d58430
	ctx.lr = 0x82D65240;
	sub_82D58430(ctx, base);
	// 82D65240: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D65244: 382102F0  addi r1, r1, 0x2f0
	ctx.r[1].s64 = ctx.r[1].s64 + 752;
	// 82D65248: 4BF441F4  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
	// 82D6524C: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82D65250: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82D65254: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82D65258: 4BFF2791  bl 0x82d579e8
	ctx.lr = 0x82D6525C;
	sub_82D579E8(ctx, base);
	// 82D6525C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D65260: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82D65264: 388B6324  addi r4, r11, 0x6324
	ctx.r[4].s64 = ctx.r[11].s64 + 25380;
	// 82D65268: 4BFF2B89  bl 0x82d57df0
	ctx.lr = 0x82D6526C;
	sub_82D57DF0(ctx, base);
	// 82D6526C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D65270: 3CA0654E  lis r5, 0x654e
	ctx.r[5].s64 = 1699610624;
	// 82D65274: 39000167  li r8, 0x167
	ctx.r[8].s64 = 359;
	// 82D65278: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
	// 82D6527C: 60A53435  ori r5, r5, 0x3435
	ctx.r[5].u64 = ctx.r[5].u64 | 13365;
	// 82D65280: 806B7630  lwz r3, 0x7630(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30256 as u32) ) } as u64;
	// 82D65284: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D65288: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82D6528C: 38EB636C  addi r7, r11, 0x636c
	ctx.r[7].s64 = ctx.r[11].s64 + 25452;
	// 82D65290: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D65294: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D65298: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D6529C: 4E800421  bctrl
	ctx.lr = 0x82D652A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D652A0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82D652A4: 4BFF318D  bl 0x82d58430
	ctx.lr = 0x82D652A8;
	sub_82D58430(ctx, base);
	// 82D652A8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D652AC: 382102F0  addi r1, r1, 0x2f0
	ctx.r[1].s64 = ctx.r[1].s64 + 752;
	// 82D652B0: 4BF4418C  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D652B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D652B8 size=328
    let mut pc: u32 = 0x82D652B8;
    'dispatch: loop {
        match pc {
            0x82D652B8 => {
    //   block [0x82D652B8..0x82D65400)
	// 82D652B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D652BC: 4BF4413D  bl 0x82ca93f8
	ctx.lr = 0x82D652C0;
	sub_82CA93D0(ctx, base);
	// 82D652C0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D652C4: 40990138  ble cr6, 0x82d653fc
	if !ctx.cr[6].gt {
	pc = 0x82D653FC; continue 'dispatch;
	}
	// 82D652C8: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D652CC: 39460010  addi r10, r6, 0x10
	ctx.r[10].s64 = ctx.r[6].s64 + 16;
	// 82D652D0: 7FE73050  subf r31, r7, r6
	ctx.r[31].s64 = ctx.r[6].s64 - ctx.r[7].s64;
	// 82D652D4: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82D652D8: 3967001C  addi r11, r7, 0x1c
	ctx.r[11].s64 = ctx.r[7].s64 + 28;
	// 82D652DC: 38694CB0  addi r3, r9, 0x4cb0
	ctx.r[3].s64 = ctx.r[9].s64 + 19632;
	// 82D652E0: 3B20FFF0  li r25, -0x10
	ctx.r[25].s64 = -16;
	// 82D652E4: 3B40FFE4  li r26, -0x1c
	ctx.r[26].s64 = -28;
	// 82D652E8: 3B60FFF4  li r27, -0xc
	ctx.r[27].s64 = -12;
	// 82D652EC: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 82D652F0: 3BA00020  li r29, 0x20
	ctx.r[29].s64 = 32;
	// 82D652F4: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 82D652F8: A1240000  lhz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D652FC: 7D290734  extsh r9, r9
	ctx.r[9].s64 = ctx.r[9].s16 as i64;
	// 82D65300: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 82D65304: 419A0018  beq cr6, 0x82d6531c
	if ctx.cr[6].eq {
	pc = 0x82D6531C; continue 'dispatch;
	}
	// 82D65308: 5528083C  slwi r8, r9, 1
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D6530C: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82D65310: 55292036  slwi r9, r9, 4
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D65314: 7D093A14  add r8, r9, r7
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 82D65318: 48000008  b 0x82d65320
	pc = 0x82D65320; continue 'dispatch;
	// 82D6531C: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
	// 82D65320: 39280010  addi r9, r8, 0x10
	ctx.r[9].s64 = ctx.r[8].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D65400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D65400 size=348
    let mut pc: u32 = 0x82D65400;
    'dispatch: loop {
        match pc {
            0x82D65400 => {
    //   block [0x82D65400..0x82D6555C)
	// 82D65400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D65404: 4BF43FE1  bl 0x82ca93e4
	ctx.lr = 0x82D65408;
	sub_82CA93D0(ctx, base);
	// 82D65408: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6540C: 7CB32B78  mr r19, r5
	ctx.r[19].u64 = ctx.r[5].u64;
	// 82D65410: 7CD43378  mr r20, r6
	ctx.r[20].u64 = ctx.r[6].u64;
	// 82D65414: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D65418: 4099013C  ble cr6, 0x82d65554
	if !ctx.cr[6].gt {
	pc = 0x82D65554; continue 'dispatch;
	}
	// 82D6541C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D65420: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82D65424: 3BD40010  addi r30, r20, 0x10
	ctx.r[30].s64 = ctx.r[20].s64 + 16;
	// 82D65428: 3BE7001C  addi r31, r7, 0x1c
	ctx.r[31].s64 = ctx.r[7].s64 + 28;
	// 82D6542C: 7F47A050  subf r26, r7, r20
	ctx.r[26].s64 = ctx.r[20].s64 - ctx.r[7].s64;
	// 82D65430: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D65434: 3B6B4CB0  addi r27, r11, 0x4cb0
	ctx.r[27].s64 = ctx.r[11].s64 + 19632;
	// 82D65438: 3AA0FFF0  li r21, -0x10
	ctx.r[21].s64 = -16;
	// 82D6543C: 3AC0FFE4  li r22, -0x1c
	ctx.r[22].s64 = -28;
	// 82D65440: 3AE0FFF4  li r23, -0xc
	ctx.r[23].s64 = -12;
	// 82D65444: 3B000010  li r24, 0x10
	ctx.r[24].s64 = 16;
	// 82D65448: 3B200004  li r25, 4
	ctx.r[25].s64 = 4;
	// 82D6544C: A17C0000  lhz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D65450: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82D65454: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82D65458: 419A0018  beq cr6, 0x82d65470
	if ctx.cr[6].eq {
	pc = 0x82D65470; continue 'dispatch;
	}
	// 82D6545C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D65460: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D65464: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D65468: 7C8BA214  add r4, r11, r20
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[20].u64;
	// 82D6546C: 48000008  b 0x82d65474
	pc = 0x82D65474; continue 'dispatch;
	// 82D65470: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 82D65474: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D65478: 4BFF5C61  bl 0x82d5b0d8
	ctx.lr = 0x82D6547C;
	sub_82D5B0D8(ctx, base);
	// 82D6547C: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D65560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D65560 size=24
    let mut pc: u32 = 0x82D65560;
    'dispatch: loop {
        match pc {
            0x82D65560 => {
    //   block [0x82D65560..0x82D65578)
	// 82D65560: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82D65564: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 82D65568: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82D6556C: 38AA7380  addi r5, r10, 0x7380
	ctx.r[5].s64 = ctx.r[10].s64 + 29568;
	// 82D65570: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 82D65574: 4BFFFD44  b 0x82d652b8
	sub_82D652B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D65578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D65578 size=24
    let mut pc: u32 = 0x82D65578;
    'dispatch: loop {
        match pc {
            0x82D65578 => {
    //   block [0x82D65578..0x82D65590)
	// 82D65578: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82D6557C: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 82D65580: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82D65584: 38AA7380  addi r5, r10, 0x7380
	ctx.r[5].s64 = ctx.r[10].s64 + 29568;
	// 82D65588: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 82D6558C: 4BFFFE74  b 0x82d65400
	sub_82D65400(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D65590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D65590 size=288
    let mut pc: u32 = 0x82D65590;
    'dispatch: loop {
        match pc {
            0x82D65590 => {
    //   block [0x82D65590..0x82D656B0)
	// 82D65590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D65594: 4BF43E6D  bl 0x82ca9400
	ctx.lr = 0x82D65598;
	sub_82CA93D0(ctx, base);
	// 82D65598: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82D6559C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D655A0: 4099010C  ble cr6, 0x82d656ac
	if !ctx.cr[6].gt {
	pc = 0x82D656AC; continue 'dispatch;
	}
	// 82D655A4: 39040010  addi r8, r4, 0x10
	ctx.r[8].s64 = ctx.r[4].s64 + 16;
	// 82D655A8: 3CE08202  lis r7, -0x7dfe
	ctx.r[7].s64 = -2113798144;
	// 82D655AC: 3966001C  addi r11, r6, 0x1c
	ctx.r[11].s64 = ctx.r[6].s64 + 28;
	// 82D655B0: 39450010  addi r10, r5, 0x10
	ctx.r[10].s64 = ctx.r[5].s64 + 16;
	// 82D655B4: 7CC62850  subf r6, r6, r5
	ctx.r[6].s64 = ctx.r[5].s64 - ctx.r[6].s64;
	// 82D655B8: 3BE40020  addi r31, r4, 0x20
	ctx.r[31].s64 = ctx.r[4].s64 + 32;
	// 82D655BC: 38A8000C  addi r5, r8, 0xc
	ctx.r[5].s64 = ctx.r[8].s64 + 12;
	// 82D655C0: 38E74CB0  addi r7, r7, 0x4cb0
	ctx.r[7].s64 = ctx.r[7].s64 + 19632;
	// 82D655C4: 3B60FFF0  li r27, -0x10
	ctx.r[27].s64 = -16;
	// 82D655C8: 3B80FFE4  li r28, -0x1c
	ctx.r[28].s64 = -28;
	// 82D655CC: 3BA0FFF4  li r29, -0xc
	ctx.r[29].s64 = -12;
	// 82D655D0: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 82D655D4: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D656B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D656B0 size=300
    let mut pc: u32 = 0x82D656B0;
    'dispatch: loop {
        match pc {
            0x82D656B0 => {
    //   block [0x82D656B0..0x82D657DC)
	// 82D656B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D656B4: 4BF43D39  bl 0x82ca93ec
	ctx.lr = 0x82D656B8;
	sub_82CA93D0(ctx, base);
	// 82D656B8: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D656BC: 7C952378  mr r21, r4
	ctx.r[21].u64 = ctx.r[4].u64;
	// 82D656C0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D656C4: 40990110  ble cr6, 0x82d657d4
	if !ctx.cr[6].gt {
	pc = 0x82D657D4; continue 'dispatch;
	}
	// 82D656C8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D656CC: 3BC50010  addi r30, r5, 0x10
	ctx.r[30].s64 = ctx.r[5].s64 + 16;
	// 82D656D0: 3BE6001C  addi r31, r6, 0x1c
	ctx.r[31].s64 = ctx.r[6].s64 + 28;
	// 82D656D4: 7F662850  subf r27, r6, r5
	ctx.r[27].s64 = ctx.r[5].s64 - ctx.r[6].s64;
	// 82D656D8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D656DC: 3B8B4CB0  addi r28, r11, 0x4cb0
	ctx.r[28].s64 = ctx.r[11].s64 + 19632;
	// 82D656E0: 3AC0FFF0  li r22, -0x10
	ctx.r[22].s64 = -16;
	// 82D656E4: 3AE0FFE4  li r23, -0x1c
	ctx.r[23].s64 = -28;
	// 82D656E8: 3B00FFF4  li r24, -0xc
	ctx.r[24].s64 = -12;
	// 82D656EC: 3B200010  li r25, 0x10
	ctx.r[25].s64 = 16;
	// 82D656F0: 3B400004  li r26, 4
	ctx.r[26].s64 = 4;
	// 82D656F4: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 82D656F8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D656FC: 4BFF59DD  bl 0x82d5b0d8
	ctx.lr = 0x82D65700;
	sub_82D5B0D8(ctx, base);
	// 82D65700: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D657E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D657E0 size=248
    let mut pc: u32 = 0x82D657E0;
    'dispatch: loop {
        match pc {
            0x82D657E0 => {
    //   block [0x82D657E0..0x82D658D8)
	// 82D657E0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82D657E4: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D657E8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D657EC: 409900E4  ble cr6, 0x82d658d0
	if !ctx.cr[6].gt {
	pc = 0x82D658D0; continue 'dispatch;
	}
	// 82D657F0: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82D657F4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D657F8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82D657FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82D65800: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82D65804: 38AB4CB0  addi r5, r11, 0x4cb0
	ctx.r[5].s64 = ctx.r[11].s64 + 19632;
	// 82D65808: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6580C: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D65810: 7D0B302E  lwzx r8, r11, r6
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82D65814: 7D67522E  lhzx r11, r7, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D65818: 89480004  lbz r10, 4(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6581C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D65820: 419A0098  beq cr6, 0x82d658b8
	if ctx.cr[6].eq {
	pc = 0x82D658B8; continue 'dispatch;
	}
	// 82D65824: 7D6A0734  extsh r10, r11
	ctx.r[10].s64 = ctx.r[11].s16 as i64;
	// 82D65828: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D6582C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82D65830: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82D65834: 419A007C  beq cr6, 0x82d658b0
	if ctx.cr[6].eq {
	pc = 0x82D658B0; continue 'dispatch;
	}
	// 82D65838: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D658D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D658D8 size=252
    let mut pc: u32 = 0x82D658D8;
    'dispatch: loop {
        match pc {
            0x82D658D8 => {
    //   block [0x82D658D8..0x82D659D4)
	// 82D658D8: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82D658DC: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82D658E0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D658E4: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82D658E8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D658EC: 409900DC  ble cr6, 0x82d659c8
	if !ctx.cr[6].gt {
	pc = 0x82D659C8; continue 'dispatch;
	}
	// 82D658F0: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82D658F4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D658F8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D658FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82D65900: 7CE92850  subf r7, r9, r5
	ctx.r[7].s64 = ctx.r[5].s64 - ctx.r[9].s64;
	// 82D65904: 3BEB4CB0  addi r31, r11, 0x4cb0
	ctx.r[31].s64 = ctx.r[11].s64 + 19632;
	// 82D65908: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6590C: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D65910: 7D0B202E  lwzx r8, r11, r4
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82D65914: 7D66522E  lhzx r11, r6, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D65918: 89480004  lbz r10, 4(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6591C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D65920: 419A0090  beq cr6, 0x82d659b0
	if ctx.cr[6].eq {
	pc = 0x82D659B0; continue 'dispatch;
	}
	// 82D65924: 7D6A0734  extsh r10, r11
	ctx.r[10].s64 = ctx.r[11].s16 as i64;
	// 82D65928: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82D6592C: 419A007C  beq cr6, 0x82d659a8
	if ctx.cr[6].eq {
	pc = 0x82D659A8; continue 'dispatch;
	}
	// 82D65930: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D659D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D659D8 size=164
    let mut pc: u32 = 0x82D659D8;
    'dispatch: loop {
        match pc {
            0x82D659D8 => {
    //   block [0x82D659D8..0x82D65A7C)
	// 82D659D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D659DC: 4BF43A25  bl 0x82ca9400
	ctx.lr = 0x82D659E0;
	sub_82CA93D0(ctx, base);
	// 82D659E0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D659E4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82D659E8: B0A100B6  sth r5, 0xb6(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(182 as u32), ctx.r[5].u16 ) };
	// 82D659EC: 7CDF0734  extsh r31, r6
	ctx.r[31].s64 = ctx.r[6].s16 as i64;
	// 82D659F0: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82D659F4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82D659F8: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82D659FC: B3C10050  sth r30, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u16 ) };
	// 82D65A00: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82D65A04: 40990040  ble cr6, 0x82d65a44
	if !ctx.cr[6].gt {
	pc = 0x82D65A44; continue 'dispatch;
	}
	// 82D65A08: 7CBB0734  extsh r27, r5
	ctx.r[27].s64 = ctx.r[5].s16 as i64;
	// 82D65A0C: 7F1FD800  cmpw cr6, r31, r27
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82D65A10: 419A0034  beq cr6, 0x82d65a44
	if ctx.cr[6].eq {
	pc = 0x82D65A44; continue 'dispatch;
	}
	// 82D65A14: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82D65A18: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82D65A1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D65A20: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D65A24: 4B539C85  bl 0x8229f6a8
	ctx.lr = 0x82D65A28;
	sub_8229F6A8(ctx, base);
	// 82D65A28: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D65A2C: 57EA083C  slwi r10, r31, 1
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D65A30: 7FCA5A2E  lhzx r30, r10, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D65A34: 7FDF0734  extsh r31, r30
	ctx.r[31].s64 = ctx.r[30].s16 as i64;
	// 82D65A38: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82D65A3C: B3C10050  sth r30, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u16 ) };
	// 82D65A40: 4199FFCC  bgt cr6, 0x82d65a0c
	if ctx.cr[6].gt {
	pc = 0x82D65A0C; continue 'dispatch;
	}
	// 82D65A44: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82D65A48: 38A100B6  addi r5, r1, 0xb6
	ctx.r[5].s64 = ctx.r[1].s64 + 182;
	// 82D65A4C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D65A50: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D65A54: 4B539C55  bl 0x8229f6a8
	ctx.lr = 0x82D65A58;
	sub_8229F6A8(ctx, base);
	// 82D65A58: 7FCB0734  extsh r11, r30
	ctx.r[11].s64 = ctx.r[30].s16 as i64;
	// 82D65A5C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82D65A60: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82D65A64: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82D65A68: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82D65A6C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82D65A70: 997A0000  stb r11, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D65A74: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D65A78: 4BF439D8  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D65A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D65A80 size=484
    let mut pc: u32 = 0x82D65A80;
    'dispatch: loop {
        match pc {
            0x82D65A80 => {
    //   block [0x82D65A80..0x82D65C64)
	// 82D65A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D65A84: 4BF43971  bl 0x82ca93f4
	ctx.lr = 0x82D65A88;
	sub_82CA93D0(ctx, base);
	// 82D65A88: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D65A8C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82D65A90: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D65A94: 3B000004  li r24, 4
	ctx.r[24].s64 = 4;
	// 82D65A98: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82D65A9C: 3FA08000  lis r29, -0x8000
	ctx.r[29].s64 = -2147483648;
	// 82D65AA0: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82D65AA4: 83DC0010  lwz r30, 0x10(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D65AA8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82D65AAC: 7D78B82E  lwzx r11, r24, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82D65AB0: 395E0010  addi r10, r30, 0x10
	ctx.r[10].s64 = ctx.r[30].s64 + 16;
	// 82D65AB4: 93410050  stw r26, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 82D65AB8: 93410054  stw r26, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[26].u32 ) };
	// 82D65ABC: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 82D65AC0: 55440036  rlwinm r4, r10, 0, 0, 0x1b
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82D65AC4: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D65AC8: 810B002C  lwz r8, 0x2c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D65ACC: 7D2A2214  add r9, r10, r4
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82D65AD0: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82D65AD4: 4199000C  bgt cr6, 0x82d65ae0
	if ctx.cr[6].gt {
	pc = 0x82D65AE0; continue 'dispatch;
	}
	// 82D65AD8: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82D65ADC: 4800001C  b 0x82d65af8
	pc = 0x82D65AF8; continue 'dispatch;
	// 82D65AE0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D65AE4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82D65AE8: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D65AEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D65AF0: 4E800421  bctrl
	ctx.lr = 0x82D65AF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D65AF4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82D65AF8: 7FCBEB78  or r11, r30, r29
	ctx.r[11].u64 = ctx.r[30].u64 | ctx.r[29].u64;
	// 82D65AFC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82D65B00: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82D65B04: 83FC0010  lwz r31, 0x10(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D65B08: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D65B0C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82D65B10: 7F1F5000  cmpw cr6, r31, r10
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D65B14: 40990054  ble cr6, 0x82d65b68
	if !ctx.cr[6].gt {
	pc = 0x82D65B68; continue 'dispatch;
	}
	// 82D65B18: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D65B1C: 7D5E5378  mr r30, r10
	ctx.r[30].u64 = ctx.r[10].u64;
	// 82D65B20: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D65B24: 40980024  bge cr6, 0x82d65b48
	if !ctx.cr[6].lt {
	pc = 0x82D65B48; continue 'dispatch;
	}
	// 82D65B28: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D65B2C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D65B30: 41980008  blt cr6, 0x82d65b38
	if ctx.cr[6].lt {
	pc = 0x82D65B38; continue 'dispatch;
	}
	// 82D65B34: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82D65B38: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D65B3C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D65B40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D65B44: 4BFF13CD  bl 0x82d56f10
	ctx.lr = 0x82D65B48;
	sub_82D56F10(ctx, base);
	// 82D65B48: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82D65B4C: 7F1EF800  cmpw cr6, r30, r31
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D65B50: 40980018  bge cr6, 0x82d65b68
	if !ctx.cr[6].lt {
	pc = 0x82D65B68; continue 'dispatch;
	}
	// 82D65B54: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D65B58: 7F4B51AE  stbx r26, r11, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[26].u8) };
	// 82D65B5C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D65B60: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D65B64: 4198FFF0  blt cr6, 0x82d65b54
	if ctx.cr[6].lt {
	pc = 0x82D65B54; continue 'dispatch;
	}
	// 82D65B68: 7F2B0734  extsh r11, r25
	ctx.r[11].s64 = ctx.r[25].s16 as i64;
	// 82D65B6C: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D65B70: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82D65B74: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82D65B78: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82D65B7C: 7D5E0734  extsh r30, r10
	ctx.r[30].s64 = ctx.r[10].s16 as i64;
	// 82D65B80: 7FAB49AE  stbx r29, r11, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[29].u8) };
	// 82D65B84: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82D65B88: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D65B8C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D65B90: 40980080  bge cr6, 0x82d65c10
	if !ctx.cr[6].lt {
	pc = 0x82D65C10; continue 'dispatch;
	}
	// 82D65B94: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D65B98: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D65B9C: 57E9083C  slwi r9, r31, 1
	ctx.r[9].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D65BA0: 7D49522E  lhzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D65BA4: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 82D65BA8: 7D4A58AE  lbzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D65BAC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D65BB0: 419A0048  beq cr6, 0x82d65bf8
	if ctx.cr[6].eq {
	pc = 0x82D65BF8; continue 'dispatch;
	}
	// 82D65BB4: 7FBF59AE  stbx r29, r31, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u8) };
	// 82D65BB8: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D65BBC: 815B0004  lwz r10, 4(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D65BC0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D65BC4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D65BC8: 409A0010  bne cr6, 0x82d65bd8
	if !ctx.cr[6].eq {
	pc = 0x82D65BD8; continue 'dispatch;
	}
	// 82D65BCC: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82D65BD0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82D65BD4: 4BFF13C5  bl 0x82d56f98
	ctx.lr = 0x82D65BD8;
	sub_82D56F98(ctx, base);
	// 82D65BD8: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D65BDC: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D65BE0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D65BE4: 7FCB532E  sthx r30, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u16) };
	// 82D65BE8: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D65BEC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D65BF0: 917B0004  stw r11, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D65BF4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D65BF8: 395F0001  addi r10, r31, 1
	ctx.r[10].s64 = ctx.r[31].s64 + 1;
	// 82D65BFC: 813C0010  lwz r9, 0x10(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D65C00: 7D5E0734  extsh r30, r10
	ctx.r[30].s64 = ctx.r[10].s16 as i64;
	// 82D65C04: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82D65C08: 7F1F4800  cmpw cr6, r31, r9
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D65C0C: 4198FF8C  blt cr6, 0x82d65b98
	if ctx.cr[6].lt {
	pc = 0x82D65B98; continue 'dispatch;
	}
	// 82D65C10: 7C78B82E  lwzx r3, r24, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82D65C14: 8081005C  lwz r4, 0x5c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82D65C18: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D65C1C: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82D65C20: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82D65C24: 409A0014  bne cr6, 0x82d65c38
	if !ctx.cr[6].eq {
	pc = 0x82D65C38; continue 'dispatch;
	}
	// 82D65C28: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D65C2C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D65C30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D65C34: 4E800421  bctrl
	ctx.lr = 0x82D65C38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D65C38: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D65C3C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D65C40: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D65C44: 409A0018  bne cr6, 0x82d65c5c
	if !ctx.cr[6].eq {
	pc = 0x82D65C5C; continue 'dispatch;
	}
	// 82D65C48: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D65C4C: 7C78B82E  lwzx r3, r24, r23
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82D65C50: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D65C54: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D65C58: 4BFEF671  bl 0x82d552c8
	ctx.lr = 0x82D65C5C;
	sub_82D552C8(ctx, base);
	// 82D65C5C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82D65C60: 4BF437E4  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D65C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D65C68 size=508
    let mut pc: u32 = 0x82D65C68;
    'dispatch: loop {
        match pc {
            0x82D65C68 => {
    //   block [0x82D65C68..0x82D65E64)
	// 82D65C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D65C6C: 4BF4378D  bl 0x82ca93f8
	ctx.lr = 0x82D65C70;
	sub_82CA93D0(ctx, base);
	// 82D65C70: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D65C74: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82D65C78: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D65C7C: 3B200004  li r25, 4
	ctx.r[25].s64 = 4;
	// 82D65C80: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82D65C84: 3FA08000  lis r29, -0x8000
	ctx.r[29].s64 = -2147483648;
	// 82D65C88: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82D65C8C: 83DC0010  lwz r30, 0x10(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D65C90: 7D79C02E  lwzx r11, r25, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82D65C94: 395E0010  addi r10, r30, 0x10
	ctx.r[10].s64 = ctx.r[30].s64 + 16;
	// 82D65C98: 93610050  stw r27, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 82D65C9C: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82D65CA0: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 82D65CA4: 55440036  rlwinm r4, r10, 0, 0, 0x1b
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82D65CA8: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D65CAC: 810B002C  lwz r8, 0x2c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D65CB0: 7D2A2214  add r9, r10, r4
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82D65CB4: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82D65CB8: 4199000C  bgt cr6, 0x82d65cc4
	if ctx.cr[6].gt {
	pc = 0x82D65CC4; continue 'dispatch;
	}
	// 82D65CBC: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82D65CC0: 4800001C  b 0x82d65cdc
	pc = 0x82D65CDC; continue 'dispatch;
	// 82D65CC4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D65CC8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82D65CCC: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D65CD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D65CD4: 4E800421  bctrl
	ctx.lr = 0x82D65CD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D65CD8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82D65CDC: 7FCBEB78  or r11, r30, r29
	ctx.r[11].u64 = ctx.r[30].u64 | ctx.r[29].u64;
	// 82D65CE0: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82D65CE4: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82D65CE8: 83FC0010  lwz r31, 0x10(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D65CEC: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D65CF0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82D65CF4: 7F1F5000  cmpw cr6, r31, r10
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D65CF8: 40990054  ble cr6, 0x82d65d4c
	if !ctx.cr[6].gt {
	pc = 0x82D65D4C; continue 'dispatch;
	}
	// 82D65CFC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D65D00: 7D5E5378  mr r30, r10
	ctx.r[30].u64 = ctx.r[10].u64;
	// 82D65D04: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D65D08: 40980024  bge cr6, 0x82d65d2c
	if !ctx.cr[6].lt {
	pc = 0x82D65D2C; continue 'dispatch;
	}
	// 82D65D0C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D65D10: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D65D14: 41980008  blt cr6, 0x82d65d1c
	if ctx.cr[6].lt {
	pc = 0x82D65D1C; continue 'dispatch;
	}
	// 82D65D18: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82D65D1C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D65D20: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D65D24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D65D28: 4BFF11E9  bl 0x82d56f10
	ctx.lr = 0x82D65D2C;
	sub_82D56F10(ctx, base);
	// 82D65D2C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82D65D30: 7F1EF800  cmpw cr6, r30, r31
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D65D34: 40980018  bge cr6, 0x82d65d4c
	if !ctx.cr[6].lt {
	pc = 0x82D65D4C; continue 'dispatch;
	}
	// 82D65D38: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D65D3C: 7F6B51AE  stbx r27, r11, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u8) };
	// 82D65D40: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D65D44: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D65D48: 4198FFF0  blt cr6, 0x82d65d38
	if ctx.cr[6].lt {
	pc = 0x82D65D38; continue 'dispatch;
	}
	// 82D65D4C: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D65D50: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 82D65D54: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82D65D58: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D65D5C: 4099003C  ble cr6, 0x82d65d98
	if !ctx.cr[6].gt {
	pc = 0x82D65D98; continue 'dispatch;
	}
	// 82D65D60: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 82D65D64: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82D65D68: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D65D6C: 7D6B522E  lhzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D65D70: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82D65D74: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82D65D78: 419A000C  beq cr6, 0x82d65d84
	if ctx.cr[6].eq {
	pc = 0x82D65D84; continue 'dispatch;
	}
	// 82D65D7C: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D65D80: 7D0B39AE  stbx r8, r11, r7
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), ctx.r[8].u8) };
	// 82D65D84: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D65D88: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82D65D8C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82D65D90: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D65D94: 4198FFD4  blt cr6, 0x82d65d68
	if ctx.cr[6].lt {
	pc = 0x82D65D68; continue 'dispatch;
	}
	// 82D65D98: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D65D9C: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 82D65DA0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D65DA4: 4099006C  ble cr6, 0x82d65e10
	if !ctx.cr[6].gt {
	pc = 0x82D65E10; continue 'dispatch;
	}
	// 82D65DA8: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 82D65DAC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D65DB0: 7D7E58AE  lbzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D65DB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D65DB8: 409A0040  bne cr6, 0x82d65df8
	if !ctx.cr[6].eq {
	pc = 0x82D65DF8; continue 'dispatch;
	}
	// 82D65DBC: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D65DC0: 815A0004  lwz r10, 4(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D65DC4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D65DC8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D65DCC: 409A0010  bne cr6, 0x82d65ddc
	if !ctx.cr[6].eq {
	pc = 0x82D65DDC; continue 'dispatch;
	}
	// 82D65DD0: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82D65DD4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82D65DD8: 4BFF11C1  bl 0x82d56f98
	ctx.lr = 0x82D65DDC;
	sub_82D56F98(ctx, base);
	// 82D65DDC: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D65DE0: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D65DE4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D65DE8: 7FEB532E  sthx r31, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u16) };
	// 82D65DEC: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D65DF0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D65DF4: 917A0004  stw r11, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D65DF8: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82D65DFC: 815C0010  lwz r10, 0x10(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D65E00: 7D7F0734  extsh r31, r11
	ctx.r[31].s64 = ctx.r[11].s16 as i64;
	// 82D65E04: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82D65E08: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D65E0C: 4198FFA0  blt cr6, 0x82d65dac
	if ctx.cr[6].lt {
	pc = 0x82D65DAC; continue 'dispatch;
	}
	// 82D65E10: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82D65E14: 8081005C  lwz r4, 0x5c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82D65E18: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D65E1C: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82D65E20: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82D65E24: 409A0014  bne cr6, 0x82d65e38
	if !ctx.cr[6].eq {
	pc = 0x82D65E38; continue 'dispatch;
	}
	// 82D65E28: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D65E2C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D65E30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D65E34: 4E800421  bctrl
	ctx.lr = 0x82D65E38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D65E38: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D65E3C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D65E40: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D65E44: 409A0018  bne cr6, 0x82d65e5c
	if !ctx.cr[6].eq {
	pc = 0x82D65E5C; continue 'dispatch;
	}
	// 82D65E48: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D65E4C: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82D65E50: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D65E54: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D65E58: 4BFEF471  bl 0x82d552c8
	ctx.lr = 0x82D65E5C;
	sub_82D552C8(ctx, base);
	// 82D65E5C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82D65E60: 4BF435E8  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D65E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D65E68 size=164
    let mut pc: u32 = 0x82D65E68;
    'dispatch: loop {
        match pc {
            0x82D65E68 => {
    //   block [0x82D65E68..0x82D65F0C)
	// 82D65E68: 39010024  addi r8, r1, 0x24
	ctx.r[8].s64 = ctx.r[1].s64 + 36;
	// 82D65E6C: D0210024  stfs f1, 0x24(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82D65E70: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D65F0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D65F0C size=120
    let mut pc: u32 = 0x82D65F0C;
    'dispatch: loop {
        match pc {
            0x82D65F0C => {
    //   block [0x82D65F0C..0x82D65F84)
	// 82D65F0C: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D65F84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D65F84 size=64
    let mut pc: u32 = 0x82D65F84;
    'dispatch: loop {
        match pc {
            0x82D65F84 => {
    //   block [0x82D65F84..0x82D65FC4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D65FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D65FC8 size=580
    let mut pc: u32 = 0x82D65FC8;
    'dispatch: loop {
        match pc {
            0x82D65FC8 => {
    //   block [0x82D65FC8..0x82D6620C)
	// 82D65FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D65FCC: 4BF43415  bl 0x82ca93e0
	ctx.lr = 0x82D65FD0;
	sub_82CA93D0(ctx, base);
	// 82D65FD0: DBC1FF78  stfd f30, -0x88(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[30].u64 ) };
	// 82D65FD4: DBE1FF80  stfd f31, -0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), ctx.f[31].u64 ) };
	// 82D65FD8: 9421FCE0  stwu r1, -0x320(r1)
	ea = ctx.r[1].u32.wrapping_add(-800 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D65FDC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D65FE0: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 82D65FE4: 3A5D000C  addi r18, r29, 0xc
	ctx.r[18].s64 = ctx.r[29].s64 + 12;
	// 82D65FE8: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D65FEC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D65FF0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D65FF4: 3AAB7630  addi r21, r11, 0x7630
	ctx.r[21].s64 = ctx.r[11].s64 + 30256;
	// 82D65FF8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D65FFC: 3A8B6448  addi r20, r11, 0x6448
	ctx.r[20].s64 = ctx.r[11].s64 + 25672;
	// 82D66000: 40990158  ble cr6, 0x82d66158
	if !ctx.cr[6].gt {
	pc = 0x82D66158; continue 'dispatch;
	}
	// 82D66004: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D66008: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82D6600C: 3B8B6444  addi r28, r11, 0x6444
	ctx.r[28].s64 = ctx.r[11].s64 + 25668;
	// 82D66010: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D66014: 3AFD0008  addi r23, r29, 8
	ctx.r[23].s64 = ctx.r[29].s64 + 8;
	// 82D66018: 3B6B6424  addi r27, r11, 0x6424
	ctx.r[27].s64 = ctx.r[11].s64 + 25636;
	// 82D6601C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D66020: C3CA0BE8  lfs f30, 0xbe8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3048 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82D66024: 3ADD0004  addi r22, r29, 4
	ctx.r[22].s64 = ctx.r[29].s64 + 4;
	// 82D66028: 3B4B641C  addi r26, r11, 0x641c
	ctx.r[26].s64 = ctx.r[11].s64 + 25628;
	// 82D6602C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D66030: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82D66034: 3B2B6410  addi r25, r11, 0x6410
	ctx.r[25].s64 = ctx.r[11].s64 + 25616;
	// 82D66038: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6603C: 81560000  lwz r10, 0(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D66040: 7D785A14  add r11, r24, r11
	ctx.r[11].u64 = ctx.r[24].u64 + ctx.r[11].u64;
	// 82D66044: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D66048: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6604C: A10B0002  lhz r8, 2(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82D66050: A0EB0000  lhz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D66054: 7D080734  extsh r8, r8
	ctx.r[8].s64 = ctx.r[8].s16 as i64;
	// 82D66058: 8129000C  lwz r9, 0xc(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6605C: 7CE70734  extsh r7, r7
	ctx.r[7].s64 = ctx.r[7].s16 as i64;
	// 82D66060: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D66064: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D66068: 7FE8502E  lwzx r31, r8, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D6606C: 7FC7482E  lwzx r30, r7, r9
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82D66070: 895F0004  lbz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D66074: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D66078: 419A00CC  beq cr6, 0x82d66144
	if ctx.cr[6].eq {
	pc = 0x82D66144; continue 'dispatch;
	}
	// 82D6607C: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82D66080: 39410054  addi r10, r1, 0x54
	ctx.r[10].s64 = ctx.r[1].s64 + 84;
	// 82D66084: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D66210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D66210 size=72
    let mut pc: u32 = 0x82D66210;
    'dispatch: loop {
        match pc {
            0x82D66210 => {
    //   block [0x82D66210..0x82D66258)
	// 82D66210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D66214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D66218: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D6621C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D66220: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D66224: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D66228: 396B5EEC  addi r11, r11, 0x5eec
	ctx.r[11].s64 = ctx.r[11].s64 + 24300;
	// 82D6622C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82D66230: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D66234: 4BFFCE85  bl 0x82d630b8
	ctx.lr = 0x82D66238;
	sub_82D630B8(ctx, base);
	// 82D66238: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D6623C: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D66240: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D66244: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D66248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D6624C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D66250: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D66254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D66258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D66258 size=2548
    let mut pc: u32 = 0x82D66258;
    'dispatch: loop {
        match pc {
            0x82D66258 => {
    //   block [0x82D66258..0x82D66C4C)
	// 82D66258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6625C: 4BF43175  bl 0x82ca93d0
	ctx.lr = 0x82D66260;
	sub_82CA93D0(ctx, base);
	// 82D66260: DBE1FF60  stfd f31, -0xa0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82D66264: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D66268: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6626C: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82D66270: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82D66274: 7F5E5A14  add r26, r30, r11
	ctx.r[26].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82D66278: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82D6627C: 7C952378  mr r21, r4
	ctx.r[21].u64 = ctx.r[4].u64;
	// 82D66280: 7CB42B78  mr r20, r5
	ctx.r[20].u64 = ctx.r[5].u64;
	// 82D66284: 938101D4  stw r28, 0x1d4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(468 as u32), ctx.r[28].u32 ) };
	// 82D66288: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82D6628C: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D66290: 932101F4  stw r25, 0x1f4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(500 as u32), ctx.r[25].u32 ) };
	// 82D66294: 93410078  stw r26, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[26].u32 ) };
	// 82D66298: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6629C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D662A0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D662A4: 4098002C  bge cr6, 0x82d662d0
	if !ctx.cr[6].lt {
	pc = 0x82D662D0; continue 'dispatch;
	}
	// 82D662A8: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D662AC: 3D008202  lis r8, -0x7dfe
	ctx.r[8].s64 = -2113798144;
	// 82D662B0: 392964C0  addi r9, r9, 0x64c0
	ctx.r[9].s64 = ctx.r[9].s64 + 25792;
	// 82D662B4: 390864B8  addi r8, r8, 0x64b8
	ctx.r[8].s64 = ctx.r[8].s64 + 25784;
	// 82D662B8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D662BC: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82D662C0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D662C4: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82D662C8: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D662CC: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D662D0: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D662D4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D662D8: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D662DC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D662E0: 40980020  bge cr6, 0x82d66300
	if !ctx.cr[6].lt {
	pc = 0x82D66300; continue 'dispatch;
	}
	// 82D662E4: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D662E8: 392964A4  addi r9, r9, 0x64a4
	ctx.r[9].s64 = ctx.r[9].s64 + 25764;
	// 82D662EC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D662F0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D662F4: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82D662F8: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D662FC: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D66300: 80FC0014  lwz r7, 0x14(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D66304: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D66308: 39E00000  li r15, 0
	ctx.r[15].s64 = 0;
	// 82D6630C: 3BE00020  li r31, 0x20
	ctx.r[31].s64 = 32;
	// 82D66310: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82D66314: 3AEB4CB0  addi r23, r11, 0x4cb0
	ctx.r[23].s64 = ctx.r[11].s64 + 19632;
	// 82D66318: 40990120  ble cr6, 0x82d66438
	if !ctx.cr[6].gt {
	pc = 0x82D66438; continue 'dispatch;
	}
	// 82D6631C: 387C0010  addi r3, r28, 0x10
	ctx.r[3].s64 = ctx.r[28].s64 + 16;
	// 82D66320: 7DE67B78  mr r6, r15
	ctx.r[6].u64 = ctx.r[15].u64;
	// 82D66324: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D66C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D66C50 size=100
    let mut pc: u32 = 0x82D66C50;
    'dispatch: loop {
        match pc {
            0x82D66C50 => {
    //   block [0x82D66C50..0x82D66CB4)
	// 82D66C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D66C54: 4BF427B5  bl 0x82ca9408
	ctx.lr = 0x82D66C58;
	sub_82CA93D0(ctx, base);
	// 82D66C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D66C5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D66C60: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82D66C64: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82D66C68: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82D66C6C: 4BFFD7BD  bl 0x82d64428
	ctx.lr = 0x82D66C70;
	sub_82D64428(ctx, base);
	// 82D66C70: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D66C74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D66C78: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D66C7C: 4BFFD77D  bl 0x82d643f8
	ctx.lr = 0x82D66C80;
	sub_82D643F8(ctx, base);
	// 82D66C80: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D66C84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D66C88: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D66C8C: 4BFFD39D  bl 0x82d64028
	ctx.lr = 0x82D66C90;
	sub_82D64028(ctx, base);
	// 82D66C90: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D66C94: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82D66C98: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D66C9C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82D66CA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D66CA4: 80CB0000  lwz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D66CA8: 4BFFF5B1  bl 0x82d66258
	ctx.lr = 0x82D66CAC;
	sub_82D66258(ctx, base);
	// 82D66CAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D66CB0: 4BF427A8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D66CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D66CB8 size=128
    let mut pc: u32 = 0x82D66CB8;
    'dispatch: loop {
        match pc {
            0x82D66CB8 => {
    //   block [0x82D66CB8..0x82D66D38)
	// 82D66CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D66CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D66CC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D66CC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D66CC8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D66CCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D66CD0: 392B5EEC  addi r9, r11, 0x5eec
	ctx.r[9].s64 = ctx.r[11].s64 + 24300;
	// 82D66CD4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82D66CD8: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82D66CDC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D66CE0: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82D66CE4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D66CE8: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82D66CEC: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D66CF0: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82D66CF4: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82D66CF8: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82D66CFC: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82D66D00: 9143001C  stw r10, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82D66D04: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82D66D08: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82D66D0C: 91430028  stw r10, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82D66D10: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D66D14: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D66D18: 9903002C  stb r8, 0x2c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[8].u8 ) };
	// 82D66D1C: 4800001D  bl 0x82d66d38
	ctx.lr = 0x82D66D20;
	sub_82D66D38(ctx, base);
	// 82D66D20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D66D24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D66D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D66D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D66D30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D66D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D66D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D66D38 size=736
    let mut pc: u32 = 0x82D66D38;
    'dispatch: loop {
        match pc {
            0x82D66D38 => {
    //   block [0x82D66D38..0x82D67018)
	// 82D66D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D66D3C: 4BF426C5  bl 0x82ca9400
	ctx.lr = 0x82D66D40;
	sub_82CA93D0(ctx, base);
	// 82D66D40: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D66D44: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82D66D48: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82D66D4C: 3BDA0008  addi r30, r26, 8
	ctx.r[30].s64 = ctx.r[26].s64 + 8;
	// 82D66D50: 3BFB0008  addi r31, r27, 8
	ctx.r[31].s64 = ctx.r[27].s64 + 8;
	// 82D66D54: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D66D58: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D66D5C: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D66D60: 917B0004  stw r11, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D66D64: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D66D68: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D66D6C: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D66D70: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D66D74: 40980060  bge cr6, 0x82d66dd4
	if !ctx.cr[6].lt {
	pc = 0x82D66DD4; continue 'dispatch;
	}
	// 82D66D78: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D66D7C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D66D80: 409A0020  bne cr6, 0x82d66da0
	if !ctx.cr[6].eq {
	pc = 0x82D66DA0; continue 'dispatch;
	}
	// 82D66D84: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D66D88: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D66D8C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D66D90: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D66D94: 55453032  slwi r5, r10, 6
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D66D98: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D66D9C: 4BFEE52D  bl 0x82d552c8
	ctx.lr = 0x82D66DA0;
	sub_82D552C8(ctx, base);
	// 82D66DA0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D66DA4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D66DA8: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D66DAC: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82D66DB0: 55243032  slwi r4, r9, 6
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(6);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82D66DB4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D66DB8: 4BFEE491  bl 0x82d55248
	ctx.lr = 0x82D66DBC;
	sub_82D55248(ctx, base);
	// 82D66DBC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D66DC0: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82D66DC4: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D66DC8: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D66DCC: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82D66DD0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D66DD4: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D66DD8: 3B80FFF0  li r28, -0x10
	ctx.r[28].s64 = -16;
	// 82D66DDC: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D66DE0: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 82D66DE4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D66DE8: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D66DEC: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D66DF0: 40990054  ble cr6, 0x82d66e44
	if !ctx.cr[6].gt {
	pc = 0x82D66E44; continue 'dispatch;
	}
	// 82D66DF4: 39680020  addi r11, r8, 0x20
	ctx.r[11].s64 = ctx.r[8].s64 + 32;
	// 82D66DF8: 39470002  addi r10, r7, 2
	ctx.r[10].s64 = ctx.r[7].s64 + 2;
	// 82D66DFC: 7D083850  subf r8, r8, r7
	ctx.r[8].s64 = ctx.r[7].s64 - ctx.r[8].s64;
	// 82D66E00: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 82D66E04: 38E0002E  li r7, 0x2e
	ctx.r[7].s64 = 46;
	// 82D66E08: A0AAFFFE  lhz r5, -2(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-2 as u32) ) } as u64;
	// 82D66E0C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82D66E10: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D66E14: B0ABFFE0  sth r5, -0x20(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-32 as u32), ctx.r[5].u16 ) };
	// 82D66E18: A0AA0000  lhz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D66E1C: B0ABFFE2  sth r5, -0x1e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-30 as u32), ctx.r[5].u16 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D67018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D67018 size=1508
    let mut pc: u32 = 0x82D67018;
    'dispatch: loop {
        match pc {
            0x82D67018 => {
    //   block [0x82D67018..0x82D675FC)
	// 82D67018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6701C: 4BF423E5  bl 0x82ca9400
	ctx.lr = 0x82D67020;
	sub_82CA93D0(ctx, base);
	// 82D67020: DBA1FFB0  stfd f29, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[29].u64 ) };
	// 82D67024: DBC1FFB8  stfd f30, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[30].u64 ) };
	// 82D67028: DBE1FFC0  stfd f31, -0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82D6702C: 3980FF80  li r12, -0x80
	ctx.r[12].s64 = -128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D67600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D67600 size=184
    let mut pc: u32 = 0x82D67600;
    'dispatch: loop {
        match pc {
            0x82D67600 => {
    //   block [0x82D67600..0x82D676B8)
	// 82D67600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D67604: 4BF41E09  bl 0x82ca940c
	ctx.lr = 0x82D67608;
	sub_82CA93D0(ctx, base);
	// 82D67608: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82D6760C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D67610: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D67614: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82D67618: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D6761C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82D67620: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D67624: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D67628: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D6762C: 4E800421  bctrl
	ctx.lr = 0x82D67630;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D67630: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 82D67634: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82D67638: C01F000C  lfs f0, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6763C: 79680020  clrldi r8, r11, 0x20
	ctx.r[8].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82D67640: EC1F0024  fdivs f0, f31, f0
	ctx.f[0].f64 = ((ctx.f[31].f64 / ctx.f[0].f64) as f32) as f64;
	// 82D67644: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82D67648: F9010050  std r8, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u64 ) };
	// 82D6764C: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82D67650: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82D67654: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82D67658: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82D6765C: FDA0065E  fctidz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82D67660: 7DA04FAE  stfiwx f13, 0, r9
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32, tmp.u32) };
	// 82D67664: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D67668: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82D6766C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D67670: 40990020  ble cr6, 0x82d67690
	if !ctx.cr[6].gt {
	pc = 0x82D67690; continue 'dispatch;
	}
	// 82D67674: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D67678: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D6767C: C00B0C14  lfs f0, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67680: D01D0000  stfs f0, 0(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D67684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D67688: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82D6768C: 4BF41DD0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82D67690: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82D67694: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82D67698: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82D6769C: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82D676A0: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82D676A4: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82D676A8: D01D0000  stfs f0, 0(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D676AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D676B0: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82D676B4: 4BF41DA8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D676B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D676B8 size=84
    let mut pc: u32 = 0x82D676B8;
    'dispatch: loop {
        match pc {
            0x82D676B8 => {
    //   block [0x82D676B8..0x82D6770C)
	// 82D676B8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D676BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D676C0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82D676C4: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82D676C8: C1AB0BF8  lfs f13, 0xbf8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3064 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D676CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D676D0: D1A30008  stfs f13, 8(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82D676D4: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82D676D8: B1230000  sth r9, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 82D676DC: B1030002  sth r8, 2(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 82D676E0: C18B0C18  lfs f12, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82D676E4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D676E8: D1830010  stfs f12, 0x10(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82D676EC: C00B0BE8  lfs f0, 0xbe8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3048 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D676F0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82D676F4: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82D676F8: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82D676FC: C16BBE14  lfs f11, -0x41ec(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16876 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82D67700: D1630018  stfs f11, 0x18(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82D67704: 9943000C  stb r10, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 82D67708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D67710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D67710 size=8
    let mut pc: u32 = 0x82D67710;
    'dispatch: loop {
        match pc {
            0x82D67710 => {
    //   block [0x82D67710..0x82D67718)
	// 82D67710: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D67714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D67718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D67718 size=24
    let mut pc: u32 = 0x82D67718;
    'dispatch: loop {
        match pc {
            0x82D67718 => {
    //   block [0x82D67718..0x82D67730)
	// 82D67718: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6771C: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D67720: 0CCB0000  twi 6, r11, 0
	// 82D67724: 7D645B96  divwu r11, r4, r11
	ctx.r[11].u32 = ctx.r[4].u32 / ctx.r[11].u32;
	// 82D67728: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D6772C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D67730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D67730 size=96
    let mut pc: u32 = 0x82D67730;
    'dispatch: loop {
        match pc {
            0x82D67730 => {
    //   block [0x82D67730..0x82D67790)
	// 82D67730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D67734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D67738: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D6773C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D67740: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82D67744: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82D67748: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D6774C: 4BFFFEB5  bl 0x82d67600
	ctx.lr = 0x82D67750;
	sub_82D67600(ctx, base);
	// 82D67750: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D67754: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D67758: 0CCB0000  twi 6, r11, 0
	// 82D6775C: 7D2A5B96  divwu r9, r10, r11
	ctx.r[9].u32 = ctx.r[10].u32 / ctx.r[11].u32;
	// 82D67760: 7D2959D6  mullw r9, r9, r11
	ctx.r[9].s64 = (ctx.r[9].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82D67764: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82D67768: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D6776C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82D67770: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82D67774: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82D67778: 386B0003  addi r3, r11, 3
	ctx.r[3].s64 = ctx.r[11].s64 + 3;
	// 82D6777C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D67780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D67784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D67788: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D6778C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D67790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D67790 size=76
    let mut pc: u32 = 0x82D67790;
    'dispatch: loop {
        match pc {
            0x82D67790 => {
    //   block [0x82D67790..0x82D677DC)
	// 82D67790: 81630054  lwz r11, 0x54(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D67794: 5489103A  slwi r9, r4, 2
	ctx.r[9].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D67798: 81430044  lwz r10, 0x44(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 82D6779C: 8103004C  lwz r8, 0x4c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82D677A0: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82D677A4: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82D677A8: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82D677AC: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D677B0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82D677B4: 5569073E  clrlwi r9, r11, 0x1c
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82D677B8: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D677BC: 99250008  stb r9, 8(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[9].u8 ) };
	// 82D677C0: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D677C4: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82D677C8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D677CC: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D677D0: 419A000C  beq cr6, 0x82d677dc
	if ctx.cr[6].eq {
		sub_82D677DC(ctx, base);
		return;
	}
	// 82D677D4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D677D8: 48000008  b 0x82d677e0
	sub_82D677DC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D677DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D677DC size=44
    let mut pc: u32 = 0x82D677DC;
    'dispatch: loop {
        match pc {
            0x82D677DC => {
    //   block [0x82D677DC..0x82D67808)
	// 82D677DC: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D677E0: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D677E4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82D677E8: 556A003E  slwi r10, r11, 0
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D677EC: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D677F0: 552B063E  clrlwi r11, r9, 0x18
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82D677F4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D677F8: 396B000F  addi r11, r11, 0xf
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	// 82D677FC: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D67800: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D67804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D67808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D67808 size=148
    let mut pc: u32 = 0x82D67808;
    'dispatch: loop {
        match pc {
            0x82D67808 => {
    //   block [0x82D67808..0x82D6789C)
	// 82D67808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6780C: 4BF41C01  bl 0x82ca940c
	ctx.lr = 0x82D67810;
	sub_82CA93D0(ctx, base);
	// 82D67810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D67814: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D67818: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82D6781C: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82D67820: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82D67824: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D67828: 4BFFFDD9  bl 0x82d67600
	ctx.lr = 0x82D6782C;
	sub_82D67600(ctx, base);
	// 82D6782C: 39600060  li r11, 0x60
	ctx.r[11].s64 = 96;
	// 82D67830: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82D67834: 38BE0018  addi r5, r30, 0x18
	ctx.r[5].s64 = ctx.r[30].s64 + 24;
	// 82D67838: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D6783C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D67840: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D67844: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D67848: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82D6784C: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82D67850: 396B000F  addi r11, r11, 0xf
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	// 82D67854: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D67858: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82D6785C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D67860: 7C8A5B96  divwu r4, r10, r11
	ctx.r[4].u32 = ctx.r[10].u32 / ctx.r[11].u32;
	// 82D67864: 0CCB0000  twi 6, r11, 0
	// 82D67868: 4BFFFF29  bl 0x82d67790
	ctx.lr = 0x82D6786C;
	sub_82D67790(ctx, base);
	// 82D6786C: 2F1D0004  cmpwi cr6, r29, 4
	ctx.cr[6].compare_i32(ctx.r[29].s32, 4, &mut ctx.xer);
	// 82D67870: 409A0024  bne cr6, 0x82d67894
	if !ctx.cr[6].eq {
	pc = 0x82D67894; continue 'dispatch;
	}
	// 82D67874: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82D67878: 39440001  addi r10, r4, 1
	ctx.r[10].s64 = ctx.r[4].s64 + 1;
	// 82D6787C: 38BE0024  addi r5, r30, 0x24
	ctx.r[5].s64 = ctx.r[30].s64 + 36;
	// 82D67880: 7D2A5B96  divwu r9, r10, r11
	ctx.r[9].u32 = ctx.r[10].u32 / ctx.r[11].u32;
	// 82D67884: 0CCB0000  twi 6, r11, 0
	// 82D67888: 7D6959D6  mullw r11, r9, r11
	ctx.r[11].s64 = (ctx.r[9].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82D6788C: 7C8B5050  subf r4, r11, r10
	ctx.r[4].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82D67890: 4BFFFF01  bl 0x82d67790
	ctx.lr = 0x82D67894;
	sub_82D67790(ctx, base);
	// 82D67894: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D67898: 4BF41BC4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D678A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D678A0 size=248
    let mut pc: u32 = 0x82D678A0;
    'dispatch: loop {
        match pc {
            0x82D678A0 => {
    //   block [0x82D678A0..0x82D67998)
	// 82D678A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D678A4: 4BF41B59  bl 0x82ca93fc
	ctx.lr = 0x82D678A8;
	sub_82CA93D0(ctx, base);
	// 82D678A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D678AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D678B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82D678B4: 833F0048  lwz r25, 0x48(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82D678B8: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82D678BC: 419A00D8  beq cr6, 0x82d67994
	if ctx.cr[6].eq {
	pc = 0x82D67994; continue 'dispatch;
	}
	// 82D678C0: 839F004C  lwz r28, 0x4c(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82D678C4: 3B79FFFF  addi r27, r25, -1
	ctx.r[27].s64 = ctx.r[25].s64 + -1;
	// 82D678C8: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82D678CC: 3BB9FFFF  addi r29, r25, -1
	ctx.r[29].s64 = ctx.r[25].s64 + -1;
	// 82D678D0: 83DF0054  lwz r30, 0x54(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D678D4: 395C000F  addi r10, r28, 0xf
	ctx.r[10].s64 = ctx.r[28].s64 + 15;
	// 82D678D8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82D678DC: 7C8BF214  add r4, r11, r30
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82D678E0: 554B0036  rlwinm r11, r10, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82D678E4: 3B4B0060  addi r26, r11, 0x60
	ctx.r[26].s64 = ctx.r[11].s64 + 96;
	// 82D678E8: 7D66202E  lwzx r11, r6, r4
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82D678EC: 7F07E800  cmpw cr6, r7, r29
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82D678F0: 7D262214  add r9, r6, r4
	ctx.r[9].u64 = ctx.r[6].u64 + ctx.r[4].u64;
	// 82D678F4: 7D4BF214  add r10, r11, r30
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82D678F8: 7D4AE214  add r10, r10, r28
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[28].u64;
	// 82D678FC: 5548073E  clrlwi r8, r10, 0x1c
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x0000000Fu64;
	// 82D67900: 419A000C  beq cr6, 0x82d6790c
	if ctx.cr[6].eq {
	pc = 0x82D6790C; continue 'dispatch;
	}
	// 82D67904: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D67908: 48000008  b 0x82d67910
	pc = 0x82D67910; continue 'dispatch;
	// 82D6790C: 815F0050  lwz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D67910: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82D67914: 550B063E  clrlwi r11, r8, 0x18
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 82D67918: 7F07D840  cmplw cr6, r7, r27
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82D6791C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D67920: 396B000F  addi r11, r11, 0xf
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	// 82D67924: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D67928: 7CABD214  add r5, r11, r26
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82D6792C: 4098004C  bge cr6, 0x82d67978
	if !ctx.cr[6].lt {
	pc = 0x82D67978; continue 'dispatch;
	}
	// 82D67930: 7D462214  add r10, r6, r4
	ctx.r[10].u64 = ctx.r[6].u64 + ctx.r[4].u64;
	// 82D67934: 811F004C  lwz r8, 0x4c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82D67938: 39670001  addi r11, r7, 1
	ctx.r[11].s64 = ctx.r[7].s64 + 1;
	// 82D6793C: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82D67940: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D67944: 7D2BF214  add r9, r11, r30
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82D67948: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82D6794C: 5529073E  clrlwi r9, r9, 0x1c
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000000Fu64;
	// 82D67950: 419A000C  beq cr6, 0x82d6795c
	if ctx.cr[6].eq {
	pc = 0x82D6795C; continue 'dispatch;
	}
	// 82D67954: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D67958: 48000008  b 0x82d67960
	pc = 0x82D67960; continue 'dispatch;
	// 82D6795C: 815F0050  lwz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D67960: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82D67964: 552A063E  clrlwi r10, r9, 0x18
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82D67968: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82D6796C: 396B000F  addi r11, r11, 0xf
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	// 82D67970: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D67974: 7CAB2A14  add r5, r11, r5
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82D67978: 7F051800  cmpw cr6, r5, r3
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82D6797C: 40990008  ble cr6, 0x82d67984
	if !ctx.cr[6].gt {
	pc = 0x82D67984; continue 'dispatch;
	}
	// 82D67980: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82D67984: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82D67988: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 82D6798C: 7F07C840  cmplw cr6, r7, r25
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[25].u32, &mut ctx.xer);
	// 82D67990: 4198FF58  blt cr6, 0x82d678e8
	if ctx.cr[6].lt {
	pc = 0x82D678E8; continue 'dispatch;
	}
	// 82D67994: 4BF41AB8  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D67998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D67998 size=32
    let mut pc: u32 = 0x82D67998;
    'dispatch: loop {
        match pc {
            0x82D67998 => {
    //   block [0x82D67998..0x82D679B8)
	// 82D67998: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6799C: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82D679A0: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82D679A4: 80A30010  lwz r5, 0x10(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D679A8: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 82D679AC: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D679B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D679B4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D679B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D679B8 size=44
    let mut pc: u32 = 0x82D679B8;
    'dispatch: loop {
        match pc {
            0x82D679B8 => {
    //   block [0x82D679B8..0x82D679E4)
	// 82D679B8: D021FFF0  stfs f1, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82D679BC: 8961FFF3  lbz r11, -0xd(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-13 as u32) ) } as u64;
	// 82D679C0: 9961FFF4  stb r11, -0xc(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), ctx.r[11].u8 ) };
	// 82D679C4: 8961FFF2  lbz r11, -0xe(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-14 as u32) ) } as u64;
	// 82D679C8: 9961FFF5  stb r11, -0xb(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-11 as u32), ctx.r[11].u8 ) };
	// 82D679CC: 8961FFF1  lbz r11, -0xf(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-15 as u32) ) } as u64;
	// 82D679D0: 9961FFF6  stb r11, -0xa(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-10 as u32), ctx.r[11].u8 ) };
	// 82D679D4: 8961FFF0  lbz r11, -0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82D679D8: 9961FFF7  stb r11, -9(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-9 as u32), ctx.r[11].u8 ) };
	// 82D679DC: C021FFF4  lfs f1, -0xc(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82D679E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D679E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D679E8 size=44
    let mut pc: u32 = 0x82D679E8;
    'dispatch: loop {
        match pc {
            0x82D679E8 => {
    //   block [0x82D679E8..0x82D67A14)
	// 82D679E8: 90610014  stw r3, 0x14(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82D679EC: 89610017  lbz r11, 0x17(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(23 as u32) ) } as u64;
	// 82D679F0: 9961FFF0  stb r11, -0x10(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u8 ) };
	// 82D679F4: 89610016  lbz r11, 0x16(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(22 as u32) ) } as u64;
	// 82D679F8: 9961FFF1  stb r11, -0xf(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-15 as u32), ctx.r[11].u8 ) };
	// 82D679FC: 89610015  lbz r11, 0x15(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(21 as u32) ) } as u64;
	// 82D67A00: 9961FFF2  stb r11, -0xe(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-14 as u32), ctx.r[11].u8 ) };
	// 82D67A04: 89610014  lbz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D67A08: 9961FFF3  stb r11, -0xd(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-13 as u32), ctx.r[11].u8 ) };
	// 82D67A0C: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82D67A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D67A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D67A18 size=28
    let mut pc: u32 = 0x82D67A18;
    'dispatch: loop {
        match pc {
            0x82D67A18 => {
    //   block [0x82D67A18..0x82D67A34)
	// 82D67A18: B0610016  sth r3, 0x16(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(22 as u32), ctx.r[3].u16 ) };
	// 82D67A1C: 89610017  lbz r11, 0x17(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(23 as u32) ) } as u64;
	// 82D67A20: 9961FFF0  stb r11, -0x10(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u8 ) };
	// 82D67A24: 89610016  lbz r11, 0x16(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(22 as u32) ) } as u64;
	// 82D67A28: 9961FFF1  stb r11, -0xf(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-15 as u32), ctx.r[11].u8 ) };
	// 82D67A2C: A061FFF0  lhz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82D67A30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D67A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D67A38 size=1228
    let mut pc: u32 = 0x82D67A38;
    'dispatch: loop {
        match pc {
            0x82D67A38 => {
    //   block [0x82D67A38..0x82D67F04)
	// 82D67A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D67A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D67A40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D67A44: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D67A48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D67A4C: 815F003C  lwz r10, 0x3c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82D67A50: 2B0A0004  cmplwi cr6, r10, 4
	ctx.cr[6].compare_u32(ctx.r[10].u32, 4 as u32, &mut ctx.xer);
	// 82D67A54: 409A0020  bne cr6, 0x82d67a74
	if !ctx.cr[6].eq {
	pc = 0x82D67A74; continue 'dispatch;
	}
	// 82D67A58: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D67A5C: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D67A60: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D67A64: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82D67A68: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82D67A6C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82D67A70: 48000008  b 0x82d67a78
	pc = 0x82D67A78; continue 'dispatch;
	// 82D67A74: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D67A78: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82D67A7C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D67A80: 419A0470  beq cr6, 0x82d67ef0
	if ctx.cr[6].eq {
	pc = 0x82D67EF0; continue 'dispatch;
	}
	// 82D67A84: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D67A88: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D67A8C: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82D67A90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D67A94: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D67A98: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D67A9C: 40990038  ble cr6, 0x82d67ad4
	if !ctx.cr[6].gt {
	pc = 0x82D67AD4; continue 'dispatch;
	}
	// 82D67AA0: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D67AA4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D67AA8: B1210050  sth r9, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u16 ) };
	// 82D67AAC: 89210051  lbz r9, 0x51(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 82D67AB0: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 82D67AB4: 89210050  lbz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D67AB8: 99210055  stb r9, 0x55(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(85 as u32), ctx.r[9].u8 ) };
	// 82D67ABC: A1210054  lhz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D67AC0: B12B0000  sth r9, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 82D67AC4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82D67AC8: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D67ACC: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D67AD0: 4198FFD0  blt cr6, 0x82d67aa0
	if ctx.cr[6].lt {
	pc = 0x82D67AA0; continue 'dispatch;
	}
	// 82D67AD4: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82D67AD8: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D67ADC: 48004255  bl 0x82d6bd30
	ctx.lr = 0x82D67AE0;
	sub_82D6BD30(ctx, base);
	// 82D67AE0: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82D67AE4: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82D67AE8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82D67AEC: 81210060  lwz r9, 0x60(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82D67AF0: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D67AF4: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82D67AF8: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82D67AFC: 813F0054  lwz r9, 0x54(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D67B00: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82D67B04: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82D67B08: 419800E4  blt cr6, 0x82d67bec
	if ctx.cr[6].lt {
	pc = 0x82D67BEC; continue 'dispatch;
	}
	// 82D67B0C: 392AFFFC  addi r9, r10, -4
	ctx.r[9].s64 = ctx.r[10].s64 + -4;
	// 82D67B10: 5529F0BE  srwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D67B14: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82D67B18: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D67B1C: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67B20: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82D67B24: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82D67B28: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67B2C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D67B30: 88E10057  lbz r7, 0x57(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 82D67B34: 98E10050  stb r7, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u8 ) };
	// 82D67B38: 88E10056  lbz r7, 0x56(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 82D67B3C: 98E10051  stb r7, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[7].u8 ) };
	// 82D67B40: 88E10055  lbz r7, 0x55(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 82D67B44: 98E10052  stb r7, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[7].u8 ) };
	// 82D67B48: 88E10054  lbz r7, 0x54(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D67B4C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82D67B50: 98E10053  stb r7, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[7].u8 ) };
	// 82D67B54: 88E10057  lbz r7, 0x57(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 82D67B58: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67B5C: 98E10050  stb r7, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u8 ) };
	// 82D67B60: 88E10056  lbz r7, 0x56(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 82D67B64: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D67B68: C00B0008  lfs f0, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67B6C: 98E10051  stb r7, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[7].u8 ) };
	// 82D67B70: 88E10055  lbz r7, 0x55(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 82D67B74: 98E10052  stb r7, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[7].u8 ) };
	// 82D67B78: 88E10054  lbz r7, 0x54(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D67B7C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82D67B80: 98E10053  stb r7, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[7].u8 ) };
	// 82D67B84: 88E10057  lbz r7, 0x57(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 82D67B88: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67B8C: 98E10050  stb r7, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u8 ) };
	// 82D67B90: 88E10056  lbz r7, 0x56(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 82D67B94: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D67B98: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67B9C: 98E10051  stb r7, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[7].u8 ) };
	// 82D67BA0: 88E10055  lbz r7, 0x55(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 82D67BA4: 98E10052  stb r7, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[7].u8 ) };
	// 82D67BA8: 88E10054  lbz r7, 0x54(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D67BAC: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82D67BB0: 98E10053  stb r7, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[7].u8 ) };
	// 82D67BB4: 88E10057  lbz r7, 0x57(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 82D67BB8: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67BBC: 98E10050  stb r7, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u8 ) };
	// 82D67BC0: 88E10056  lbz r7, 0x56(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 82D67BC4: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82D67BC8: 98E10051  stb r7, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[7].u8 ) };
	// 82D67BCC: 88E10055  lbz r7, 0x55(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 82D67BD0: 98E10052  stb r7, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[7].u8 ) };
	// 82D67BD4: 88E10054  lbz r7, 0x54(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D67BD8: 98E10053  stb r7, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[7].u8 ) };
	// 82D67BDC: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67BE0: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82D67BE4: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82D67BE8: 409AFF34  bne cr6, 0x82d67b1c
	if !ctx.cr[6].eq {
	pc = 0x82D67B1C; continue 'dispatch;
	}
	// 82D67BEC: 7F085000  cmpw cr6, r8, r10
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D67BF0: 40980048  bge cr6, 0x82d67c38
	if !ctx.cr[6].lt {
	pc = 0x82D67C38; continue 'dispatch;
	}
	// 82D67BF4: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 82D67BF8: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67BFC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D67C00: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82D67C04: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D67C08: 89210057  lbz r9, 0x57(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 82D67C0C: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 82D67C10: 89210056  lbz r9, 0x56(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 82D67C14: 99210051  stb r9, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[9].u8 ) };
	// 82D67C18: 89210055  lbz r9, 0x55(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 82D67C1C: 99210052  stb r9, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[9].u8 ) };
	// 82D67C20: 89210054  lbz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D67C24: 99210053  stb r9, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[9].u8 ) };
	// 82D67C28: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67C2C: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D67C30: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D67C34: 409AFFC4  bne cr6, 0x82d67bf8
	if !ctx.cr[6].eq {
	pc = 0x82D67BF8; continue 'dispatch;
	}
	// 82D67C38: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82D67C3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82D67C40: 8141007C  lwz r10, 0x7c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82D67C44: 81010078  lwz r8, 0x78(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82D67C48: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D67C4C: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D67C50: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82D67C54: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82D67C58: 811F0034  lwz r8, 0x34(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82D67C5C: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82D67C60: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82D67C64: 2F090004  cmpwi cr6, r9, 4
	ctx.cr[6].compare_i32(ctx.r[9].s32, 4, &mut ctx.xer);
	// 82D67C68: 419801A8  blt cr6, 0x82d67e10
	if ctx.cr[6].lt {
	pc = 0x82D67E10; continue 'dispatch;
	}
	// 82D67C6C: 3909FFFC  addi r8, r9, -4
	ctx.r[8].s64 = ctx.r[9].s64 + -4;
	// 82D67C70: 5508F0BE  srwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D67C74: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82D67C78: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D67C7C: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67C80: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82D67C84: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82D67C88: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82D67C8C: 88C10057  lbz r6, 0x57(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 82D67C90: 98C10050  stb r6, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u8 ) };
	// 82D67C94: 88C10056  lbz r6, 0x56(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 82D67C98: 98C10051  stb r6, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[6].u8 ) };
	// 82D67C9C: 88C10055  lbz r6, 0x55(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 82D67CA0: 98C10052  stb r6, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[6].u8 ) };
	// 82D67CA4: 88C10054  lbz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D67CA8: 98C10053  stb r6, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[6].u8 ) };
	// 82D67CAC: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67CB0: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D67CB4: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67CB8: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82D67CBC: 88C1005B  lbz r6, 0x5b(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(91 as u32) ) } as u64;
	// 82D67CC0: 98C1005C  stb r6, 0x5c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u8 ) };
	// 82D67CC4: 88C1005A  lbz r6, 0x5a(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 82D67CC8: 98C1005D  stb r6, 0x5d(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(93 as u32), ctx.r[6].u8 ) };
	// 82D67CCC: 88C10059  lbz r6, 0x59(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(89 as u32) ) } as u64;
	// 82D67CD0: 98C1005E  stb r6, 0x5e(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(94 as u32), ctx.r[6].u8 ) };
	// 82D67CD4: 88C10058  lbz r6, 0x58(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D67CD8: 98C1005F  stb r6, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[6].u8 ) };
	// 82D67CDC: C001005C  lfs f0, 0x5c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67CE0: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D67CE4: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67CE8: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82D67CEC: 88C10057  lbz r6, 0x57(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 82D67CF0: 98C10050  stb r6, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u8 ) };
	// 82D67CF4: 88C10056  lbz r6, 0x56(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 82D67CF8: 98C10051  stb r6, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[6].u8 ) };
	// 82D67CFC: 88C10055  lbz r6, 0x55(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 82D67D00: 98C10052  stb r6, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[6].u8 ) };
	// 82D67D04: 88C10054  lbz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D67D08: 98C10053  stb r6, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[6].u8 ) };
	// 82D67D0C: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67D10: D00A0004  stfs f0, 4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D67D14: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67D18: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82D67D1C: 88C1005B  lbz r6, 0x5b(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(91 as u32) ) } as u64;
	// 82D67D20: 98C1005C  stb r6, 0x5c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u8 ) };
	// 82D67D24: 88C1005A  lbz r6, 0x5a(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 82D67D28: 98C1005D  stb r6, 0x5d(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(93 as u32), ctx.r[6].u8 ) };
	// 82D67D2C: 88C10059  lbz r6, 0x59(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(89 as u32) ) } as u64;
	// 82D67D30: 98C1005E  stb r6, 0x5e(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(94 as u32), ctx.r[6].u8 ) };
	// 82D67D34: 88C10058  lbz r6, 0x58(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D67D38: 98C1005F  stb r6, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[6].u8 ) };
	// 82D67D3C: C001005C  lfs f0, 0x5c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67D40: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D67D44: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67D48: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82D67D4C: 88C10057  lbz r6, 0x57(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 82D67D50: 98C10050  stb r6, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u8 ) };
	// 82D67D54: 88C10056  lbz r6, 0x56(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 82D67D58: 98C10051  stb r6, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[6].u8 ) };
	// 82D67D5C: 88C10055  lbz r6, 0x55(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 82D67D60: 98C10052  stb r6, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[6].u8 ) };
	// 82D67D64: 88C10054  lbz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D67D68: 98C10053  stb r6, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[6].u8 ) };
	// 82D67D6C: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67D70: D00A0008  stfs f0, 8(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82D67D74: C00B0008  lfs f0, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67D78: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82D67D7C: 88C1005B  lbz r6, 0x5b(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(91 as u32) ) } as u64;
	// 82D67D80: 98C1005C  stb r6, 0x5c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u8 ) };
	// 82D67D84: 88C1005A  lbz r6, 0x5a(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 82D67D88: 98C1005D  stb r6, 0x5d(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(93 as u32), ctx.r[6].u8 ) };
	// 82D67D8C: 88C10059  lbz r6, 0x59(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(89 as u32) ) } as u64;
	// 82D67D90: 98C1005E  stb r6, 0x5e(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(94 as u32), ctx.r[6].u8 ) };
	// 82D67D94: 88C10058  lbz r6, 0x58(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D67D98: 98C1005F  stb r6, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[6].u8 ) };
	// 82D67D9C: C001005C  lfs f0, 0x5c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67DA0: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82D67DA4: C00A000C  lfs f0, 0xc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67DA8: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82D67DAC: 88C10057  lbz r6, 0x57(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 82D67DB0: 98C10050  stb r6, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u8 ) };
	// 82D67DB4: 88C10056  lbz r6, 0x56(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 82D67DB8: 98C10051  stb r6, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[6].u8 ) };
	// 82D67DBC: 88C10055  lbz r6, 0x55(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 82D67DC0: 98C10052  stb r6, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[6].u8 ) };
	// 82D67DC4: 88C10054  lbz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D67DC8: 98C10053  stb r6, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[6].u8 ) };
	// 82D67DCC: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67DD0: D00A000C  stfs f0, 0xc(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82D67DD4: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82D67DD8: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67DDC: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82D67DE0: 88C1005B  lbz r6, 0x5b(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(91 as u32) ) } as u64;
	// 82D67DE4: 98C1005C  stb r6, 0x5c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u8 ) };
	// 82D67DE8: 88C1005A  lbz r6, 0x5a(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 82D67DEC: 98C1005D  stb r6, 0x5d(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(93 as u32), ctx.r[6].u8 ) };
	// 82D67DF0: 88C10059  lbz r6, 0x59(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(89 as u32) ) } as u64;
	// 82D67DF4: 98C1005E  stb r6, 0x5e(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(94 as u32), ctx.r[6].u8 ) };
	// 82D67DF8: 88C10058  lbz r6, 0x58(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D67DFC: 98C1005F  stb r6, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[6].u8 ) };
	// 82D67E00: C001005C  lfs f0, 0x5c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67E04: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82D67E08: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82D67E0C: 409AFE70  bne cr6, 0x82d67c7c
	if !ctx.cr[6].eq {
	pc = 0x82D67C7C; continue 'dispatch;
	}
	// 82D67E10: 7F074840  cmplw cr6, r7, r9
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D67E14: 4098007C  bge cr6, 0x82d67e90
	if !ctx.cr[6].lt {
	pc = 0x82D67E90; continue 'dispatch;
	}
	// 82D67E18: 7D274850  subf r9, r7, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	// 82D67E1C: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67E20: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82D67E24: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82D67E28: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D67E2C: 89010057  lbz r8, 0x57(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 82D67E30: 99010050  stb r8, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u8 ) };
	// 82D67E34: 89010056  lbz r8, 0x56(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 82D67E38: 99010051  stb r8, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[8].u8 ) };
	// 82D67E3C: 89010055  lbz r8, 0x55(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 82D67E40: 99010052  stb r8, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[8].u8 ) };
	// 82D67E44: 89010054  lbz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D67E48: 99010053  stb r8, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[8].u8 ) };
	// 82D67E4C: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67E50: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D67E54: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82D67E58: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67E5C: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82D67E60: 8901005B  lbz r8, 0x5b(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(91 as u32) ) } as u64;
	// 82D67E64: 9901005C  stb r8, 0x5c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u8 ) };
	// 82D67E68: 8901005A  lbz r8, 0x5a(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 82D67E6C: 9901005D  stb r8, 0x5d(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(93 as u32), ctx.r[8].u8 ) };
	// 82D67E70: 89010059  lbz r8, 0x59(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(89 as u32) ) } as u64;
	// 82D67E74: 9901005E  stb r8, 0x5e(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(94 as u32), ctx.r[8].u8 ) };
	// 82D67E78: 89010058  lbz r8, 0x58(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D67E7C: 9901005F  stb r8, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[8].u8 ) };
	// 82D67E80: C001005C  lfs f0, 0x5c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D67E84: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D67E88: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D67E8C: 409AFF90  bne cr6, 0x82d67e1c
	if !ctx.cr[6].eq {
	pc = 0x82D67E1C; continue 'dispatch;
	}
	// 82D67E90: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82D67E94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D67E98: 813F0054  lwz r9, 0x54(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D67E9C: 811F0048  lwz r8, 0x48(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82D67EA0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82D67EA4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82D67EA8: 40990048  ble cr6, 0x82d67ef0
	if !ctx.cr[6].gt {
	pc = 0x82D67EF0; continue 'dispatch;
	}
	// 82D67EAC: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D67EB0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D67EB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82D67EB8: 8921005F  lbz r9, 0x5f(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(95 as u32) ) } as u64;
	// 82D67EBC: 99210058  stb r9, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u8 ) };
	// 82D67EC0: 8921005E  lbz r9, 0x5e(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(94 as u32) ) } as u64;
	// 82D67EC4: 99210059  stb r9, 0x59(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(89 as u32), ctx.r[9].u8 ) };
	// 82D67EC8: 8921005D  lbz r9, 0x5d(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(93 as u32) ) } as u64;
	// 82D67ECC: 9921005A  stb r9, 0x5a(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(90 as u32), ctx.r[9].u8 ) };
	// 82D67ED0: 8921005C  lbz r9, 0x5c(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82D67ED4: 9921005B  stb r9, 0x5b(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(91 as u32), ctx.r[9].u8 ) };
	// 82D67ED8: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D67EDC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D67EE0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D67EE4: 813F0048  lwz r9, 0x48(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82D67EE8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D67EEC: 4198FFC0  blt cr6, 0x82d67eac
	if ctx.cr[6].lt {
	pc = 0x82D67EAC; continue 'dispatch;
	}
	// 82D67EF0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82D67EF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D67EF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D67EFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D67F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D67F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D67F08 size=104
    let mut pc: u32 = 0x82D67F08;
    'dispatch: loop {
        match pc {
            0x82D67F08 => {
    //   block [0x82D67F08..0x82D67F70)
	// 82D67F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D67F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D67F10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D67F14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D67F18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D67F1C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D67F20: 396B59F4  addi r11, r11, 0x59f4
	ctx.r[11].s64 = ctx.r[11].s64 + 23028;
	// 82D67F24: A15F0004  lhz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D67F28: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D67F2C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D67F30: 419A0020  beq cr6, 0x82d67f50
	if ctx.cr[6].eq {
	pc = 0x82D67F50; continue 'dispatch;
	}
	// 82D67F34: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D67F38: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D67F3C: 38C00037  li r6, 0x37
	ctx.r[6].s64 = 55;
	// 82D67F40: 80BF0058  lwz r5, 0x58(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D67F44: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D67F48: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D67F4C: 4BFED37D  bl 0x82d552c8
	ctx.lr = 0x82D67F50;
	sub_82D552C8(ctx, base);
	// 82D67F50: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D67F54: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D67F58: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D67F5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D67F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D67F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D67F68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D67F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D67F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D67F70 size=448
    let mut pc: u32 = 0x82D67F70;
    'dispatch: loop {
        match pc {
            0x82D67F70 => {
    //   block [0x82D67F70..0x82D68130)
	// 82D67F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D67F74: 4BF4146D  bl 0x82ca93e0
	ctx.lr = 0x82D67F78;
	sub_82CA93D0(ctx, base);
	// 82D67F78: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D67F7C: F8810108  std r4, 0x108(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(264 as u32), ctx.r[4].u64 ) };
	// 82D67F80: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82D67F84: F8610100  std r3, 0x100(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(256 as u32), ctx.r[3].u64 ) };
	// 82D67F88: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82D67F8C: F8A10110  std r5, 0x110(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(272 as u32), ctx.r[5].u64 ) };
	// 82D67F90: F8C10118  std r6, 0x118(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(280 as u32), ctx.r[6].u64 ) };
	// 82D67F94: 89610108  lbz r11, 0x108(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(264 as u32) ) } as u64;
	// 82D67F98: 8A410109  lbz r18, 0x109(r1)
	ctx.r[18].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(265 as u32) ) } as u64;
	// 82D67F9C: 83C10104  lwz r30, 0x104(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 82D67FA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D67FA4: 99610059  stb r11, 0x59(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(89 as u32), ctx.r[11].u8 ) };
	// 82D67FA8: 9A410058  stb r18, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[18].u8 ) };
	// 82D67FAC: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82D67FB0: 48090F29  bl 0x82df8ed8
	ctx.lr = 0x82D67FB4;
	sub_82DF8ED8(ctx, base);
	// 82D67FB4: 826D0000  lwz r19, 0(r13)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D67FB8: 3A800004  li r20, 4
	ctx.r[20].s64 = 4;
	// 82D67FBC: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82D67FC0: 55640036  rlwinm r4, r11, 0, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D67FC4: 7C74982E  lwzx r3, r20, r19
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 82D67FC8: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D67FCC: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D67FD0: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82D67FD4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D67FD8: 41990010  bgt cr6, 0x82d67fe8
	if ctx.cr[6].gt {
	pc = 0x82D67FE8; continue 'dispatch;
	}
	// 82D67FDC: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	// 82D67FE0: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82D67FE4: 48000018  b 0x82d67ffc
	pc = 0x82D67FFC; continue 'dispatch;
	// 82D67FE8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D67FEC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D67FF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D67FF4: 4E800421  bctrl
	ctx.lr = 0x82D67FF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D67FF8: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82D67FFC: 83010100  lwz r24, 0x100(r1)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(256 as u32) ) } as u64;
	// 82D68000: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82D68004: 83610118  lwz r27, 0x118(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(280 as u32) ) } as u64;
	// 82D68008: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82D6800C: 419A00F4  beq cr6, 0x82d68100
	if ctx.cr[6].eq {
	pc = 0x82D68100; continue 'dispatch;
	}
	// 82D68010: 83810110  lwz r28, 0x110(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(272 as u32) ) } as u64;
	// 82D68014: 57D9103A  slwi r25, r30, 2
	ctx.r[25].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 82D68018: 8161010C  lwz r11, 0x10c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(268 as u32) ) } as u64;
	// 82D6801C: 3AE00001  li r23, 1
	ctx.r[23].s64 = 1;
	// 82D68020: 82C10114  lwz r22, 0x114(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(276 as u32) ) } as u64;
	// 82D68024: 7EBC5850  subf r21, r28, r11
	ctx.r[21].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 82D68028: 7C15E42E  lfsx f0, r21, r28
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[28].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6802C: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 82D68030: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82D68034: C01C0000  lfs f0, 0(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D68038: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82D6803C: 419A0010  beq cr6, 0x82d6804c
	if ctx.cr[6].eq {
	pc = 0x82D6804C; continue 'dispatch;
	}
	// 82D68040: 7D76E8AE  lbzx r11, r22, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82D68044: 99610058  stb r11, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u8 ) };
	// 82D68048: 48000008  b 0x82d68050
	pc = 0x82D68050; continue 'dispatch;
	// 82D6804C: 9A410058  stb r18, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[18].u8 ) };
	// 82D68050: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82D68054: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D68058: 48090E81  bl 0x82df8ed8
	ctx.lr = 0x82D6805C;
	sub_82DF8ED8(ctx, base);
	// 82D6805C: 89610058  lbz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D68060: C0010060  lfs f0, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D68064: 3921006C  addi r9, r1, 0x6c
	ctx.r[9].s64 = ctx.r[1].s64 + 108;
	// 82D68068: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82D6806C: C1A1005C  lfs f13, 0x5c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D68070: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82D68074: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82D68078: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82D6807C: 7EEA5830  slw r10, r23, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[23].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82D68080: 7D4B07B4  extsw r11, r10
	ctx.r[11].s64 = ctx.r[10].s32 as i64;
	// 82D68084: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82D68088: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82D6808C: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82D68090: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82D68094: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82D68098: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82D6809C: 7C004FAE  stfiwx f0, 0, r9
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32, tmp.u32) };
	// 82D680A0: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82D680A4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82D680A8: 409A000C  bne cr6, 0x82d680b4
	if !ctx.cr[6].eq {
	pc = 0x82D680B4; continue 'dispatch;
	}
	// 82D680AC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D680B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82D680B4: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82D680B8: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 82D680BC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82D680C0: 48090A59  bl 0x82df8b18
	ctx.lr = 0x82D680C4;
	sub_82DF8B18(ctx, base);
	// 82D680C4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D680C8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82D680CC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D680D0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82D680D4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82D680D8: 7F6BDA14  add r27, r11, r27
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82D680DC: 480904F5  bl 0x82df85d0
	ctx.lr = 0x82D680E0;
	sub_82DF85D0(ctx, base);
	// 82D680E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D680E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D680E8: 4808FC11  bl 0x82df7cf8
	ctx.lr = 0x82D680EC;
	sub_82DF7CF8(ctx, base);
	// 82D680EC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82D680F0: 7FF9FA14  add r31, r25, r31
	ctx.r[31].u64 = ctx.r[25].u64 + ctx.r[31].u64;
	// 82D680F4: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82D680F8: 7F1DC040  cmplw cr6, r29, r24
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82D680FC: 4198FF2C  blt cr6, 0x82d68028
	if ctx.cr[6].lt {
	pc = 0x82D68028; continue 'dispatch;
	}
	// 82D68100: 7C74982E  lwzx r3, r20, r19
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 82D68104: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D68108: 93430020  stw r26, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[26].u32 ) };
	// 82D6810C: 7F1A5840  cmplw cr6, r26, r11
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82D68110: 409A0018  bne cr6, 0x82d68128
	if !ctx.cr[6].eq {
	pc = 0x82D68128; continue 'dispatch;
	}
	// 82D68114: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D68118: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82D6811C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D68120: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D68124: 4E800421  bctrl
	ctx.lr = 0x82D68128;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D68128: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82D6812C: 4BF41304  b 0x82ca9430
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D68130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D68130 size=1528
    let mut pc: u32 = 0x82D68130;
    'dispatch: loop {
        match pc {
            0x82D68130 => {
    //   block [0x82D68130..0x82D68728)
	// 82D68130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D68134: 4BF412A5  bl 0x82ca93d8
	ctx.lr = 0x82D68138;
	sub_82CA93D0(ctx, base);
	// 82D68138: DBC1FF68  stfd f30, -0x98(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-152 as u32), ctx.f[30].u64 ) };
	// 82D6813C: DBE1FF70  stfd f31, -0x90(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), ctx.f[31].u64 ) };
	// 82D68140: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D68144: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82D68148: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82D6814C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82D68150: 7CB22B78  mr r18, r5
	ctx.r[18].u64 = ctx.r[5].u64;
	// 82D68154: 7CD03378  mr r16, r6
	ctx.r[16].u64 = ctx.r[6].u64;
	// 82D68158: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 82D6815C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D68160: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D68164: 419A0020  beq cr6, 0x82d68184
	if ctx.cr[6].eq {
	pc = 0x82D68184; continue 'dispatch;
	}
	// 82D68168: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82D6816C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82D68170: 394A59F4  addi r10, r10, 0x59f4
	ctx.r[10].s64 = ctx.r[10].s64 + 23028;
	// 82D68174: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82D68178: B12B0006  sth r9, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82D6817C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D68180: 48000008  b 0x82d68188
	pc = 0x82D68188; continue 'dispatch;
	// 82D68184: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82D68188: 82CD0000  lwz r22, 0(r13)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6818C: 3A200008  li r17, 8
	ctx.r[17].s64 = 8;
	// 82D68190: 7D56882E  lwzx r10, r22, r17
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 82D68194: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D68198: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6819C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D681A0: 4098002C  bge cr6, 0x82d681cc
	if !ctx.cr[6].lt {
	pc = 0x82D681CC; continue 'dispatch;
	}
	// 82D681A4: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D681A8: 3D008202  lis r8, -0x7dfe
	ctx.r[8].s64 = -2113798144;
	// 82D681AC: 39296504  addi r9, r9, 0x6504
	ctx.r[9].s64 = ctx.r[9].s64 + 25860;
	// 82D681B0: 390864F8  addi r8, r8, 0x64f8
	ctx.r[8].s64 = ctx.r[8].s64 + 25848;
	// 82D681B4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D681B8: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82D681BC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D681C0: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82D681C4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D681C8: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D681CC: 83DC000C  lwz r30, 0xc(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D681D0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82D681D4: 815D003C  lwz r10, 0x3c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 82D681D8: 7E449378  mr r4, r18
	ctx.r[4].u64 = ctx.r[18].u64;
	// 82D681DC: 817D0040  lwz r11, 0x40(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(64 as u32) ) } as u64;
	// 82D681E0: 7E8AF214  add r20, r10, r30
	ctx.r[20].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82D681E4: 7E6BF214  add r19, r11, r30
	ctx.r[19].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82D681E8: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 82D681EC: 48003A95  bl 0x82d6bc80
	ctx.lr = 0x82D681F0;
	sub_82D6BC80(ctx, base);
	// 82D681F0: 8121007C  lwz r9, 0x7c(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82D681F4: 81410080  lwz r10, 0x80(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82D681F8: 7D76882E  lwzx r11, r22, r17
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 82D681FC: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82D68200: 81210078  lwz r9, 0x78(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82D68204: 7FEA4A14  add r31, r10, r9
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82D68208: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6820C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D68210: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D68214: 40980020  bge cr6, 0x82d68234
	if !ctx.cr[6].lt {
	pc = 0x82D68234; continue 'dispatch;
	}
	// 82D68218: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D6821C: 392964E4  addi r9, r9, 0x64e4
	ctx.r[9].s64 = ctx.r[9].s64 + 25828;
	// 82D68220: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D68224: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D68228: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82D6822C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D68230: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D68234: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D68238: 815D0024  lwz r10, 0x24(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6823C: 3AA00004  li r21, 4
	ctx.r[21].s64 = 4;
	// 82D68240: C00B0C14  lfs f0, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D68244: 7D6AF9D6  mullw r11, r10, r31
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[31].s32 as i64);
	// 82D68248: 7C76A82E  lwzx r3, r22, r21
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82D6824C: EFE0F028  fsubs f31, f0, f30
	ctx.f[31].f64 = (((ctx.f[0].f64 - ctx.f[30].f64) as f32) as f64);
	// 82D68250: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D68254: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D68258: 55641036  rlwinm r4, r11, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D6825C: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D68260: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82D68264: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D68268: 41990010  bgt cr6, 0x82d68278
	if ctx.cr[6].gt {
	pc = 0x82D68278; continue 'dispatch;
	}
	// 82D6826C: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 82D68270: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82D68274: 48000018  b 0x82d6828c
	pc = 0x82D6828C; continue 'dispatch;
	// 82D68278: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6827C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D68280: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D68284: 4E800421  bctrl
	ctx.lr = 0x82D68288;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D68288: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82D6828C: 7C76A82E  lwzx r3, r22, r21
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82D68290: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82D68294: 55641036  rlwinm r4, r11, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D68298: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D6829C: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D682A0: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82D682A4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D682A8: 41990010  bgt cr6, 0x82d682b8
	if ctx.cr[6].gt {
	pc = 0x82D682B8; continue 'dispatch;
	}
	// 82D682AC: 7D775B78  mr r23, r11
	ctx.r[23].u64 = ctx.r[11].u64;
	// 82D682B0: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82D682B4: 48000018  b 0x82d682cc
	pc = 0x82D682CC; continue 'dispatch;
	// 82D682B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D682BC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D682C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D682C4: 4E800421  bctrl
	ctx.lr = 0x82D682C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D682C8: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82D682CC: 815D0034  lwz r10, 0x34(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 82D682D0: 2F190003  cmpwi cr6, r25, 3
	ctx.cr[6].compare_i32(ctx.r[25].s32, 3, &mut ctx.xer);
	// 82D682D4: 817D0038  lwz r11, 0x38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 82D682D8: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82D682DC: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82D682E0: 813D0030  lwz r9, 0x30(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 82D682E4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82D682E8: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 82D682EC: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 82D682F0: 91410070  stw r10, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 82D682F4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82D682F8: 895D0029  lbz r10, 0x29(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(41 as u32) ) } as u64;
	// 82D682FC: 897D0028  lbz r11, 0x28(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D68300: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82D68304: 813D0024  lwz r9, 0x24(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D68308: 99410068  stb r10, 0x68(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u8 ) };
	// 82D6830C: 99610069  stb r11, 0x69(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(105 as u32), ctx.r[11].u8 ) };
	// 82D68310: 897C0020  lbz r11, 0x20(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D68314: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D68318: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82D6831C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D68320: 796607C6  sldi r6, r11, 0x20
	ctx.r[6].u64 = ctx.r[11].u64.wrapping_shl(32);
	ctx.r[6].u32 = ctx.r[6].u64 as u32;
	// 82D68324: 409A0120  bne cr6, 0x82d68444
	if !ctx.cr[6].eq {
	pc = 0x82D68444; continue 'dispatch;
	}
	// 82D68328: E8610060  ld r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82D6832C: E8810068  ld r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 82D68330: E8A10070  ld r5, 0x70(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 82D68334: 4BFFFC3D  bl 0x82d67f70
	ctx.lr = 0x82D68338;
	sub_82D67F70(ctx, base);
	// 82D68338: 815D0024  lwz r10, 0x24(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6833C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82D68340: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 82D68344: 7D7A5396  divwu r11, r26, r10
	ctx.r[11].u32 = ctx.r[26].u32 / ctx.r[10].u32;
	// 82D68348: 0CCA0000  twi 6, r10, 0
	// 82D6834C: 7D6B51D6  mullw r11, r11, r10
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82D68350: 7CCBD050  subf r6, r11, r26
	ctx.r[6].s64 = ctx.r[26].s64 - ctx.r[11].s64;
	// 82D68354: 419800A4  blt cr6, 0x82d683f8
	if ctx.cr[6].lt {
	pc = 0x82D683F8; continue 'dispatch;
	}
	// 82D68358: 397FFFFC  addi r11, r31, -4
	ctx.r[11].s64 = ctx.r[31].s64 + -4;
	// 82D6835C: 54C9103A  slwi r9, r6, 2
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D68360: 556BF0BE  srwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D68364: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D68368: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 82D6836C: 7D29DA14  add r9, r9, r27
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[27].u64;
	// 82D68370: 39770008  addi r11, r23, 8
	ctx.r[11].s64 = ctx.r[23].s64 + 8;
	// 82D68374: 54E5103A  slwi r5, r7, 2
	ctx.r[5].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D68378: C1A90000  lfs f13, 0(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D6837C: 7CCA3214  add r6, r10, r6
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82D68380: EDBF0372  fmuls f13, f31, f13
	ctx.f[13].f64 = (((ctx.f[31].f64 * ctx.f[13].f64) as f32) as f64);
	// 82D68384: C0090004  lfs f0, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D68388: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82D6838C: 7CCA3214  add r6, r10, r6
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82D68390: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 82D68394: 7CCA3214  add r6, r10, r6
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82D68398: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82D6839C: 7CCA3214  add r6, r10, r6
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82D683A0: EC006FBA  fmadds f0, f0, f30, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[30].f64 + ctx.f[13].f64) as f32) as f64);
	// 82D683A4: C1A90000  lfs f13, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D683A8: EDBF0372  fmuls f13, f31, f13
	ctx.f[13].f64 = (((ctx.f[31].f64 * ctx.f[13].f64) as f32) as f64);
	// 82D683AC: D00BFFF8  stfs f0, -8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82D683B0: C0090004  lfs f0, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D683B4: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82D683B8: EC006FBA  fmadds f0, f0, f30, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[30].f64 + ctx.f[13].f64) as f32) as f64);
	// 82D683BC: C1A90000  lfs f13, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D683C0: EDBF0372  fmuls f13, f31, f13
	ctx.f[13].f64 = (((ctx.f[31].f64 * ctx.f[13].f64) as f32) as f64);
	// 82D683C4: D00BFFFC  stfs f0, -4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82D683C8: C0090004  lfs f0, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D683CC: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82D683D0: EC006FBA  fmadds f0, f0, f30, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[30].f64 + ctx.f[13].f64) as f32) as f64);
	// 82D683D4: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D683D8: C0090000  lfs f0, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D683DC: EC1F0032  fmuls f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D683E0: C1A90004  lfs f13, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D683E4: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82D683E8: EC0D07BA  fmadds f0, f13, f30, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D683EC: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D683F0: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82D683F4: 409AFF84  bne cr6, 0x82d68378
	if !ctx.cr[6].eq {
	pc = 0x82D68378; continue 'dispatch;
	}
	// 82D683F8: 7F05F840  cmplw cr6, r5, r31
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82D683FC: 40980250  bge cr6, 0x82d6864c
	if !ctx.cr[6].lt {
	pc = 0x82D6864C; continue 'dispatch;
	}
	// 82D68400: 54A9103A  slwi r9, r5, 2
	ctx.r[9].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D68404: 54CB103A  slwi r11, r6, 2
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D68408: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D6840C: 7D29BA14  add r9, r9, r23
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[23].u64;
	// 82D68410: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82D68414: 7D45F850  subf r10, r5, r31
	ctx.r[10].s64 = ctx.r[31].s64 - ctx.r[5].s64;
	// 82D68418: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6841C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D68420: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82D68424: C1AB0004  lfs f13, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D68428: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82D6842C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D68430: EC0D07BA  fmadds f0, f13, f30, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D68434: D0090000  stfs f0, 0(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D68438: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82D6843C: 409AFFDC  bne cr6, 0x82d68418
	if !ctx.cr[6].eq {
	pc = 0x82D68418; continue 'dispatch;
	}
	// 82D68440: 4800020C  b 0x82d6864c
	pc = 0x82D6864C; continue 'dispatch;
	// 82D68444: EBC10060  ld r30, 0x60(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82D68448: EB210068  ld r25, 0x68(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 82D6844C: EB010070  ld r24, 0x70(r1)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 82D68450: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D68454: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82D68458: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 82D6845C: 4BFFFB15  bl 0x82d67f70
	ctx.lr = 0x82D68460;
	sub_82D67F70(ctx, base);
	// 82D68460: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D68464: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D68468: 7D5A5B96  divwu r10, r26, r11
	ctx.r[10].u32 = ctx.r[26].u32 / ctx.r[11].u32;
	// 82D6846C: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 82D68470: 7D4A59D6  mullw r10, r10, r11
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82D68474: 0CCB0000  twi 6, r11, 0
	// 82D68478: 7CEAD050  subf r7, r10, r26
	ctx.r[7].s64 = ctx.r[26].s64 - ctx.r[10].s64;
	// 82D6847C: 41980084  blt cr6, 0x82d68500
	if ctx.cr[6].lt {
	pc = 0x82D68500; continue 'dispatch;
	}
	// 82D68480: 395FFFFC  addi r10, r31, -4
	ctx.r[10].s64 = ctx.r[31].s64 + -4;
	// 82D68484: 54E6103A  slwi r6, r7, 2
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D68488: 554AF0BE  srwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D6848C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D68490: 390A0001  addi r8, r10, 1
	ctx.r[8].s64 = ctx.r[10].s64 + 1;
	// 82D68494: 7CC6DA14  add r6, r6, r27
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[27].u64;
	// 82D68498: 39570008  addi r10, r23, 8
	ctx.r[10].s64 = ctx.r[23].s64 + 8;
	// 82D6849C: 5504103A  slwi r4, r8, 2
	ctx.r[4].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82D684A0: C0060000  lfs f0, 0(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D684A4: 7CC64A14  add r6, r6, r9
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[9].u64;
	// 82D684A8: 7CAB3A14  add r5, r11, r7
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82D684AC: EC1F0032  fmuls f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D684B0: 7CE64A14  add r7, r6, r9
	ctx.r[7].u64 = ctx.r[6].u64 + ctx.r[9].u64;
	// 82D684B4: D00AFFF8  stfs f0, -8(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82D684B8: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82D684BC: C0060000  lfs f0, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D684C0: 7CCB2A14  add r6, r11, r5
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82D684C4: EC1F0032  fmuls f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D684C8: D00AFFFC  stfs f0, -4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82D684CC: C0070000  lfs f0, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D684D0: 7CE74A14  add r7, r7, r9
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 82D684D4: EC1F0032  fmuls f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D684D8: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D684DC: 7CAB3214  add r5, r11, r6
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 82D684E0: 7CC74A14  add r6, r7, r9
	ctx.r[6].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 82D684E4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82D684E8: C0070000  lfs f0, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D684EC: 7CEB2A14  add r7, r11, r5
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82D684F0: EC1F0032  fmuls f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D684F4: D00A0004  stfs f0, 4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D684F8: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82D684FC: 409AFFA4  bne cr6, 0x82d684a0
	if !ctx.cr[6].eq {
	pc = 0x82D684A0; continue 'dispatch;
	}
	// 82D68500: 7F04F840  cmplw cr6, r4, r31
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82D68504: 4098003C  bge cr6, 0x82d68540
	if !ctx.cr[6].lt {
	pc = 0x82D68540; continue 'dispatch;
	}
	// 82D68508: 5489103A  slwi r9, r4, 2
	ctx.r[9].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D6850C: 54EA103A  slwi r10, r7, 2
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D68510: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D68514: 7D29BA14  add r9, r9, r23
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[23].u64;
	// 82D68518: 7D4ADA14  add r10, r10, r27
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[27].u64;
	// 82D6851C: 7D64F850  subf r11, r4, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[4].s64;
	// 82D68520: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D68524: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D68528: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82D6852C: D0090000  stfs f0, 0(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D68530: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82D68534: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82D68538: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D6853C: 409AFFE4  bne cr6, 0x82d68520
	if !ctx.cr[6].eq {
	pc = 0x82D68520; continue 'dispatch;
	}
	// 82D68540: 897C002C  lbz r11, 0x2c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D68544: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82D68548: 815C0024  lwz r10, 0x24(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6854C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D68550: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82D68554: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D68558: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 82D6855C: 796607C6  sldi r6, r11, 0x20
	ctx.r[6].u64 = ctx.r[11].u64.wrapping_shl(32);
	ctx.r[6].u32 = ctx.r[6].u64 as u32;
	// 82D68560: 4BFFFA11  bl 0x82d67f70
	ctx.lr = 0x82D68564;
	sub_82D67F70(ctx, base);
	// 82D68564: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82D68568: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82D6856C: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 82D68570: 41980094  blt cr6, 0x82d68604
	if ctx.cr[6].lt {
	pc = 0x82D68604; continue 'dispatch;
	}
	// 82D68574: 397FFFFC  addi r11, r31, -4
	ctx.r[11].s64 = ctx.r[31].s64 + -4;
	// 82D68578: 815D0024  lwz r10, 0x24(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6857C: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82D68580: 5569F0BE  srwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D68584: 39770008  addi r11, r23, 8
	ctx.r[11].s64 = ctx.r[23].s64 + 8;
	// 82D68588: 39090001  addi r8, r9, 1
	ctx.r[8].s64 = ctx.r[9].s64 + 1;
	// 82D6858C: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D68590: 5505103A  slwi r5, r8, 2
	ctx.r[5].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D68594: C0070000  lfs f0, 0(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D68598: 7CE74A14  add r7, r7, r9
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 82D6859C: C1ABFFF8  lfs f13, -8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D685A0: 7CCA3214  add r6, r10, r6
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82D685A4: EC006FBA  fmadds f0, f0, f30, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[30].f64 + ctx.f[13].f64) as f32) as f64);
	// 82D685A8: D00BFFF8  stfs f0, -8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82D685AC: C18BFFFC  lfs f12, -4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82D685B0: 7CCA3214  add r6, r10, r6
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82D685B4: C16B0000  lfs f11, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82D685B8: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82D685BC: C0070000  lfs f0, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D685C0: 7CE74A14  add r7, r7, r9
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 82D685C4: EC0067BA  fmadds f0, f0, f30, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[30].f64 + ctx.f[12].f64) as f32) as f64);
	// 82D685C8: D00BFFFC  stfs f0, -4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82D685CC: C14B0004  lfs f10, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82D685D0: 7CCA3214  add r6, r10, r6
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82D685D4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82D685D8: 7CCA3214  add r6, r10, r6
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82D685DC: C0070000  lfs f0, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D685E0: 7CE74A14  add r7, r7, r9
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 82D685E4: EC005FBA  fmadds f0, f0, f30, f11
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[30].f64 + ctx.f[11].f64) as f32) as f64);
	// 82D685E8: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D685EC: C0070000  lfs f0, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D685F0: 7CE74A14  add r7, r7, r9
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 82D685F4: EC0057BA  fmadds f0, f0, f30, f10
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[30].f64 + ctx.f[10].f64) as f32) as f64);
	// 82D685F8: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D685FC: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82D68600: 409AFF94  bne cr6, 0x82d68594
	if !ctx.cr[6].eq {
	pc = 0x82D68594; continue 'dispatch;
	}
	// 82D68604: 7F05F840  cmplw cr6, r5, r31
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82D68608: 40980044  bge cr6, 0x82d6864c
	if !ctx.cr[6].lt {
	pc = 0x82D6864C; continue 'dispatch;
	}
	// 82D6860C: 813D0024  lwz r9, 0x24(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D68610: 54CA103A  slwi r10, r6, 2
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D68614: 54AB103A  slwi r11, r5, 2
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D68618: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D6861C: 7D2ADA14  add r9, r10, r27
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[27].u64;
	// 82D68620: 7D6BBA14  add r11, r11, r23
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 82D68624: 7D45F850  subf r10, r5, r31
	ctx.r[10].s64 = ctx.r[31].s64 - ctx.r[5].s64;
	// 82D68628: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6862C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D68630: C1A90000  lfs f13, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D68634: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82D68638: EC0D07BA  fmadds f0, f13, f30, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D6863C: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D68640: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D68644: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D68648: 409AFFE0  bne cr6, 0x82d68628
	if !ctx.cr[6].eq {
	pc = 0x82D68628; continue 'dispatch;
	}
	// 82D6864C: 7D56882E  lwzx r10, r22, r17
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 82D68650: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D68654: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D68658: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D6865C: 40980020  bge cr6, 0x82d6867c
	if !ctx.cr[6].lt {
	pc = 0x82D6867C; continue 'dispatch;
	}
	// 82D68660: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D68664: 392964D0  addi r9, r9, 0x64d0
	ctx.r[9].s64 = ctx.r[9].s64 + 25808;
	// 82D68668: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D6866C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D68670: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82D68674: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D68678: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D6867C: 7E058378  mr r5, r16
	ctx.r[5].u64 = ctx.r[16].u64;
	// 82D68680: 92410050  stw r18, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[18].u32 ) };
	// 82D68684: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82D68688: 92610058  stw r19, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[19].u32 ) };
	// 82D6868C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D68690: 92810054  stw r20, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[20].u32 ) };
	// 82D68694: 48003FFD  bl 0x82d6c690
	ctx.lr = 0x82D68698;
	sub_82D6C690(ctx, base);
	// 82D68698: 7C76A82E  lwzx r3, r22, r21
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82D6869C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D686A0: 92E30020  stw r23, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[23].u32 ) };
	// 82D686A4: 7F175840  cmplw cr6, r23, r11
	ctx.cr[6].compare_u32(ctx.r[23].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82D686A8: 409A0018  bne cr6, 0x82d686c0
	if !ctx.cr[6].eq {
	pc = 0x82D686C0; continue 'dispatch;
	}
	// 82D686AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D686B0: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82D686B4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D686B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D686BC: 4E800421  bctrl
	ctx.lr = 0x82D686C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D686C0: 7C76A82E  lwzx r3, r22, r21
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82D686C4: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D686C8: 93630020  stw r27, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[27].u32 ) };
	// 82D686CC: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82D686D0: 409A0018  bne cr6, 0x82d686e8
	if !ctx.cr[6].eq {
	pc = 0x82D686E8; continue 'dispatch;
	}
	// 82D686D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D686D8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82D686DC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D686E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D686E4: 4E800421  bctrl
	ctx.lr = 0x82D686E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D686E8: 7D56882E  lwzx r10, r22, r17
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 82D686EC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D686F0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D686F4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D686F8: 40980020  bge cr6, 0x82d68718
	if !ctx.cr[6].lt {
	pc = 0x82D68718; continue 'dispatch;
	}
	// 82D686FC: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D68700: 39296468  addi r9, r9, 0x6468
	ctx.r[9].s64 = ctx.r[9].s64 + 25704;
	// 82D68704: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D68708: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D6870C: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82D68710: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D68714: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D68718: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 82D6871C: CBC1FF68  lfd f30, -0x98(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-152 as u32) ) };
	// 82D68720: CBE1FF70  lfd f31, -0x90(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-144 as u32) ) };
	// 82D68724: 4BF40D04  b 0x82ca9428
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D68728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D68728 size=252
    let mut pc: u32 = 0x82D68728;
    'dispatch: loop {
        match pc {
            0x82D68728 => {
    //   block [0x82D68728..0x82D68824)
	// 82D68728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6872C: 4BF40CDD  bl 0x82ca9408
	ctx.lr = 0x82D68730;
	sub_82CA93D0(ctx, base);
	// 82D68730: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D68734: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D68738: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 82D6873C: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 82D68740: 7D7EE82E  lwzx r11, r30, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82D68744: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D68748: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6874C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D68750: 40980020  bge cr6, 0x82d68770
	if !ctx.cr[6].lt {
	pc = 0x82D68770; continue 'dispatch;
	}
	// 82D68754: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D68758: 39296518  addi r9, r9, 0x6518
	ctx.r[9].s64 = ctx.r[9].s64 + 25880;
	// 82D6875C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D68760: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D68764: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82D68768: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D6876C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D68770: 81430024  lwz r10, 0x24(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D68774: 80A30044  lwz r5, 0x44(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 82D68778: 7C845396  divwu r4, r4, r10
	ctx.r[4].u32 = ctx.r[4].u32 / ctx.r[10].u32;
	// 82D6877C: 81630054  lwz r11, 0x54(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D68780: 0CCA0000  twi 6, r10, 0
	// 82D68784: 90C10050  stw r6, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u32 ) };
	// 82D68788: 80C30034  lwz r6, 0x34(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82D6878C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82D68790: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D68794: 8123004C  lwz r9, 0x4c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82D68798: 7CC65A14  add r6, r6, r11
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[11].u64;
	// 82D6879C: 7D4A2A14  add r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 82D687A0: 83E30030  lwz r31, 0x30(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82D687A4: 81030038  lwz r8, 0x38(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82D687A8: 8B830029  lbz r28, 0x29(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(41 as u32) ) } as u64;
	// 82D687AC: 7C9F5A14  add r4, r31, r11
	ctx.r[4].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82D687B0: 88630028  lbz r3, 0x28(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D687B4: 7D085A14  add r8, r8, r11
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82D687B8: 90C10060  stw r6, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 82D687BC: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D687C0: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82D687C4: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82D687C8: 98610059  stb r3, 0x59(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(89 as u32), ctx.r[3].u8 ) };
	// 82D687CC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82D687D0: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 82D687D4: 9B810058  stb r28, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[28].u8 ) };
	// 82D687D8: 796607C6  sldi r6, r11, 0x20
	ctx.r[6].u64 = ctx.r[11].u64.wrapping_shl(32);
	ctx.r[6].u32 = ctx.r[6].u64 as u32;
	// 82D687DC: E8610050  ld r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82D687E0: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82D687E4: E8A10060  ld r5, 0x60(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82D687E8: 4BFFF789  bl 0x82d67f70
	ctx.lr = 0x82D687EC;
	sub_82D67F70(ctx, base);
	// 82D687EC: 7D7EE82E  lwzx r11, r30, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82D687F0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D687F4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D687F8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D687FC: 40980020  bge cr6, 0x82d6881c
	if !ctx.cr[6].lt {
	pc = 0x82D6881C; continue 'dispatch;
	}
	// 82D68800: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D68804: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82D68808: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D6880C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D68810: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82D68814: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D68818: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D6881C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82D68820: 4BF40C38  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D68828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D68828 size=332
    let mut pc: u32 = 0x82D68828;
    'dispatch: loop {
        match pc {
            0x82D68828 => {
    //   block [0x82D68828..0x82D68974)
	// 82D68828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6882C: 4BF40BC9  bl 0x82ca93f4
	ctx.lr = 0x82D68830;
	sub_82CA93D0(ctx, base);
	// 82D68830: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D68834: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D68838: 3B000008  li r24, 8
	ctx.r[24].s64 = 8;
	// 82D6883C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82D68840: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82D68844: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82D68848: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82D6884C: 7D78B82E  lwzx r11, r24, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82D68850: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D68854: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D68858: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D6885C: 40980020  bge cr6, 0x82d6887c
	if !ctx.cr[6].lt {
	pc = 0x82D6887C; continue 'dispatch;
	}
	// 82D68860: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D68864: 39296530  addi r9, r9, 0x6530
	ctx.r[9].s64 = ctx.r[9].s64 + 25904;
	// 82D68868: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D6886C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D68870: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82D68874: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D68878: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D6887C: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D68880: 817C0024  lwz r11, 0x24(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D68884: 813C0054  lwz r9, 0x54(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D68888: 7D5A5B96  divwu r10, r26, r11
	ctx.r[10].u32 = ctx.r[26].u32 / ctx.r[11].u32;
	// 82D6888C: 0CCB0000  twi 6, r11, 0
	// 82D68890: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D68894: 7D6BC9D6  mullw r11, r11, r25
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[25].s32 as i64);
	// 82D68898: 8108000C  lwz r8, 0xc(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6889C: 7FCA4A14  add r30, r10, r9
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82D688A0: 557D103A  slwi r29, r11, 2
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82D688A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D688A8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82D688AC: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82D688B0: 4E800421  bctrl
	ctx.lr = 0x82D688B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D688B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D688B8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D688BC: 409A007C  bne cr6, 0x82d68938
	if !ctx.cr[6].eq {
	pc = 0x82D68938; continue 'dispatch;
	}
	// 82D688C0: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D688C4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82D688C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D688CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D688D0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D688D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D688D8: 4E800421  bctrl
	ctx.lr = 0x82D688DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D688DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D688E0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D688E4: 409A0040  bne cr6, 0x82d68924
	if !ctx.cr[6].eq {
	pc = 0x82D68924; continue 'dispatch;
	}
	// 82D688E8: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82D688EC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D688F0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D688F4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D688F8: 40980020  bge cr6, 0x82d68918
	if !ctx.cr[6].lt {
	pc = 0x82D68918; continue 'dispatch;
	}
	// 82D688FC: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D68900: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82D68904: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D68908: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D6890C: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82D68910: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D68914: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D68918: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D6891C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82D68920: 4BF40B24  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
	// 82D68924: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82D68928: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D6892C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82D68930: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82D68934: 4BFFFDF5  bl 0x82d68728
	ctx.lr = 0x82D68938;
	sub_82D68728(ctx, base);
	// 82D68938: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82D6893C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D68940: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D68944: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D68948: 40980020  bge cr6, 0x82d68968
	if !ctx.cr[6].lt {
	pc = 0x82D68968; continue 'dispatch;
	}
	// 82D6894C: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D68950: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82D68954: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D68958: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D6895C: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82D68960: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D68964: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D68968: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D6896C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82D68970: 4BF40AD4  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D68978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D68978 size=1620
    let mut pc: u32 = 0x82D68978;
    'dispatch: loop {
        match pc {
            0x82D68978 => {
    //   block [0x82D68978..0x82D68FCC)
	// 82D68978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6897C: 4BF40A61  bl 0x82ca93dc
	ctx.lr = 0x82D68980;
	sub_82CA93D0(ctx, base);
	// 82D68980: DBC1FF70  stfd f30, -0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), ctx.f[30].u64 ) };
	// 82D68984: DBE1FF78  stfd f31, -0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[31].u64 ) };
	// 82D68988: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6898C: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D68990: 3AC00008  li r22, 8
	ctx.r[22].s64 = 8;
	// 82D68994: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82D68998: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82D6899C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D689A0: 7CB32B78  mr r19, r5
	ctx.r[19].u64 = ctx.r[5].u64;
	// 82D689A4: 7CD13378  mr r17, r6
	ctx.r[17].u64 = ctx.r[6].u64;
	// 82D689A8: 7D57B02E  lwzx r10, r23, r22
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82D689AC: 93210174  stw r25, 0x174(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(372 as u32), ctx.r[25].u32 ) };
	// 82D689B0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D689B4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D689B8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D689BC: 4098002C  bge cr6, 0x82d689e8
	if !ctx.cr[6].lt {
	pc = 0x82D689E8; continue 'dispatch;
	}
	// 82D689C0: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D689C4: 3D008202  lis r8, -0x7dfe
	ctx.r[8].s64 = -2113798144;
	// 82D689C8: 39296568  addi r9, r9, 0x6568
	ctx.r[9].s64 = ctx.r[9].s64 + 25960;
	// 82D689CC: 390864F8  addi r8, r8, 0x64f8
	ctx.r[8].s64 = ctx.r[8].s64 + 25848;
	// 82D689D0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D689D4: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82D689D8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D689DC: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82D689E0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D689E4: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D689E8: 835F002C  lwz r26, 0x2c(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D689EC: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82D689F0: 815F0054  lwz r10, 0x54(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D689F4: 7F5BD378  mr r27, r26
	ctx.r[27].u64 = ctx.r[26].u64;
	// 82D689F8: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D689FC: 7E4B5214  add r18, r11, r10
	ctx.r[18].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D68A00: 7F134840  cmplw cr6, r19, r9
	ctx.cr[6].compare_u32(ctx.r[19].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D68A04: 419A0028  beq cr6, 0x82d68a2c
	if ctx.cr[6].eq {
	pc = 0x82D68A2C; continue 'dispatch;
	}
	// 82D68A08: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82D68A0C: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 82D68A10: 7E439378  mr r3, r18
	ctx.r[3].u64 = ctx.r[18].u64;
	// 82D68A14: 4800326D  bl 0x82d6bc80
	ctx.lr = 0x82D68A18;
	sub_82D6BC80(ctx, base);
	// 82D68A18: 8141008C  lwz r10, 0x8c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82D68A1C: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82D68A20: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D68A24: 81410088  lwz r10, 0x88(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82D68A28: 7F6B5214  add r27, r11, r10
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D68A2C: 7D77B02E  lwzx r11, r23, r22
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82D68A30: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D68A34: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D68A38: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D68A3C: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D68A40: 3BA96558  addi r29, r9, 0x6558
	ctx.r[29].s64 = ctx.r[9].s64 + 25944;
	// 82D68A44: 40980018  bge cr6, 0x82d68a5c
	if !ctx.cr[6].lt {
	pc = 0x82D68A5C; continue 'dispatch;
	}
	// 82D68A48: 93AA0000  stw r29, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82D68A4C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D68A50: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82D68A54: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D68A58: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D68A5C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82D68A60: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82D68A64: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82D68A68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D68A6C: 4BFFEB95  bl 0x82d67600
	ctx.lr = 0x82D68A70;
	sub_82D67600(ctx, base);
	// 82D68A70: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D68A74: 3A800004  li r20, 4
	ctx.r[20].s64 = 4;
	// 82D68A78: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D68A7C: C3C10050  lfs f30, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82D68A80: C00B0C14  lfs f0, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D68A84: 7D6AD9D6  mullw r11, r10, r27
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[27].s32 as i64);
	// 82D68A88: 7C77A02E  lwzx r3, r23, r20
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 82D68A8C: EFE0F028  fsubs f31, f0, f30
	ctx.f[31].f64 = (((ctx.f[0].f64 - ctx.f[30].f64) as f32) as f64);
	// 82D68A90: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D68A94: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D68A98: 55641036  rlwinm r4, r11, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D68A9C: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D68AA0: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82D68AA4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D68AA8: 41990010  bgt cr6, 0x82d68ab8
	if ctx.cr[6].gt {
	pc = 0x82D68AB8; continue 'dispatch;
	}
	// 82D68AAC: 7D755B78  mr r21, r11
	ctx.r[21].u64 = ctx.r[11].u64;
	// 82D68AB0: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82D68AB4: 48000018  b 0x82d68acc
	pc = 0x82D68ACC; continue 'dispatch;
	// 82D68AB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D68ABC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D68AC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D68AC4: 4E800421  bctrl
	ctx.lr = 0x82D68AC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D68AC8: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 82D68ACC: 7C77A02E  lwzx r3, r23, r20
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 82D68AD0: 397B0004  addi r11, r27, 4
	ctx.r[11].s64 = ctx.r[27].s64 + 4;
	// 82D68AD4: 55641036  rlwinm r4, r11, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D68AD8: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D68ADC: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D68AE0: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82D68AE4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D68AE8: 41990010  bgt cr6, 0x82d68af8
	if ctx.cr[6].gt {
	pc = 0x82D68AF8; continue 'dispatch;
	}
	// 82D68AEC: 7D785B78  mr r24, r11
	ctx.r[24].u64 = ctx.r[11].u64;
	// 82D68AF0: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82D68AF4: 48000018  b 0x82d68b0c
	pc = 0x82D68B0C; continue 'dispatch;
	// 82D68AF8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D68AFC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D68B00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D68B04: 4E800421  bctrl
	ctx.lr = 0x82D68B08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D68B08: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82D68B0C: 83C10054  lwz r30, 0x54(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D68B10: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82D68B14: 419A0020  beq cr6, 0x82d68b34
	if ctx.cr[6].eq {
	pc = 0x82D68B34; continue 'dispatch;
	}
	// 82D68B18: 38C10174  addi r6, r1, 0x174
	ctx.r[6].s64 = ctx.r[1].s64 + 372;
	// 82D68B1C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82D68B20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D68B24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D68B28: 4BFFFD01  bl 0x82d68828
	ctx.lr = 0x82D68B2C;
	sub_82D68828(ctx, base);
	// 82D68B2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D68B30: 409A001C  bne cr6, 0x82d68b4c
	if !ctx.cr[6].eq {
	pc = 0x82D68B4C; continue 'dispatch;
	}
	// 82D68B34: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82D68B38: 7EA5AB78  mr r5, r21
	ctx.r[5].u64 = ctx.r[21].u64;
	// 82D68B3C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D68B40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D68B44: 4BFFFBE5  bl 0x82d68728
	ctx.lr = 0x82D68B48;
	sub_82D68728(ctx, base);
	// 82D68B48: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82D68B4C: 7D77B02E  lwzx r11, r23, r22
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82D68B50: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D68B54: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D68B58: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D68B5C: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D68B60: 3B89654C  addi r28, r9, 0x654c
	ctx.r[28].s64 = ctx.r[9].s64 + 25932;
	// 82D68B64: 40980018  bge cr6, 0x82d68b7c
	if !ctx.cr[6].lt {
	pc = 0x82D68B7C; continue 'dispatch;
	}
	// 82D68B68: 938A0000  stw r28, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82D68B6C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D68B70: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82D68B74: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D68B78: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D68B7C: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D68B80: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82D68B84: 7D7E5396  divwu r11, r30, r10
	ctx.r[11].u32 = ctx.r[30].u32 / ctx.r[10].u32;
	// 82D68B88: 0CCA0000  twi 6, r10, 0
	// 82D68B8C: 7D6B51D6  mullw r11, r11, r10
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82D68B90: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 82D68B94: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82D68B98: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D68B9C: 419A010C  beq cr6, 0x82d68ca8
	if ctx.cr[6].eq {
	pc = 0x82D68CA8; continue 'dispatch;
	}
	// 82D68BA0: 2F1B0004  cmpwi cr6, r27, 4
	ctx.cr[6].compare_i32(ctx.r[27].s32, 4, &mut ctx.xer);
	// 82D68BA4: 419800B8  blt cr6, 0x82d68c5c
	if ctx.cr[6].lt {
	pc = 0x82D68C5C; continue 'dispatch;
	}
	// 82D68BA8: 393BFFFC  addi r9, r27, -4
	ctx.r[9].s64 = ctx.r[27].s64 + -4;
	// 82D68BAC: 39580008  addi r10, r24, 8
	ctx.r[10].s64 = ctx.r[24].s64 + 8;
	// 82D68BB0: 5529F0BE  srwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D68BB4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82D68BB8: 5527103A  slwi r7, r9, 2
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D68BBC: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D68BC0: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82D68BC4: 7D081A14  add r8, r8, r3
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[3].u64;
	// 82D68BC8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D68BCC: C0080000  lfs f0, 0(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D68BD0: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82D68BD4: C1A80004  lfs f13, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D68BD8: EC0D07BA  fmadds f0, f13, f30, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D68BDC: D00AFFF8  stfs f0, -8(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82D68BE0: 811F0024  lwz r8, 0x24(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D68BE4: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82D68BE8: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D68BEC: 7D081A14  add r8, r8, r3
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[3].u64;
	// 82D68BF0: C0080000  lfs f0, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D68BF4: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82D68BF8: C1A80004  lfs f13, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D68BFC: EC0D07BA  fmadds f0, f13, f30, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D68C00: D00AFFFC  stfs f0, -4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82D68C04: 811F0024  lwz r8, 0x24(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D68C08: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82D68C0C: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D68C10: 7D081A14  add r8, r8, r3
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[3].u64;
	// 82D68C14: C0080000  lfs f0, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D68C18: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82D68C1C: C1A80004  lfs f13, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D68C20: EC0D07BA  fmadds f0, f13, f30, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D68C24: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D68C28: 811F0024  lwz r8, 0x24(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D68C2C: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82D68C30: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D68C34: 7D081A14  add r8, r8, r3
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[3].u64;
	// 82D68C38: C0080000  lfs f0, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D68C3C: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82D68C40: C1A80004  lfs f13, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D68C44: EC0D07BA  fmadds f0, f13, f30, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D68C48: D00A0004  stfs f0, 4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D68C4C: 811F0024  lwz r8, 0x24(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D68C50: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82D68C54: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82D68C58: 409AFF64  bne cr6, 0x82d68bbc
	if !ctx.cr[6].eq {
	pc = 0x82D68BBC; continue 'dispatch;
	}
	// 82D68C5C: 7F07D840  cmplw cr6, r7, r27
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82D68C60: 40980284  bge cr6, 0x82d68ee4
	if !ctx.cr[6].lt {
	pc = 0x82D68EE4; continue 'dispatch;
	}
	// 82D68C64: 54E9103A  slwi r9, r7, 2
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D68C68: 7D47D850  subf r10, r7, r27
	ctx.r[10].s64 = ctx.r[27].s64 - ctx.r[7].s64;
	// 82D68C6C: 7D29C214  add r9, r9, r24
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[24].u64;
	// 82D68C70: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D68C74: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D68C78: 7D081A14  add r8, r8, r3
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[3].u64;
	// 82D68C7C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D68C80: C0080000  lfs f0, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D68C84: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82D68C88: C1A80004  lfs f13, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D68C8C: EC0D07BA  fmadds f0, f13, f30, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D68C90: D0090000  stfs f0, 0(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D68C94: 811F0024  lwz r8, 0x24(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D68C98: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82D68C9C: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82D68CA0: 409AFFD0  bne cr6, 0x82d68c70
	if !ctx.cr[6].eq {
	pc = 0x82D68C70; continue 'dispatch;
	}
	// 82D68CA4: 48000240  b 0x82d68ee4
	pc = 0x82D68EE4; continue 'dispatch;
	// 82D68CA8: 2F1B0004  cmpwi cr6, r27, 4
	ctx.cr[6].compare_i32(ctx.r[27].s32, 4, &mut ctx.xer);
	// 82D68CAC: 41980088  blt cr6, 0x82d68d34
	if ctx.cr[6].lt {
	pc = 0x82D68D34; continue 'dispatch;
	}
	// 82D68CB0: 393BFFFC  addi r9, r27, -4
	ctx.r[9].s64 = ctx.r[27].s64 + -4;
	// 82D68CB4: 39580008  addi r10, r24, 8
	ctx.r[10].s64 = ctx.r[24].s64 + 8;
	// 82D68CB8: 5529F0BE  srwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D68CBC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82D68CC0: 5527103A  slwi r7, r9, 2
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D68CC4: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D68CC8: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82D68CCC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D68CD0: 7C081C2E  lfsx f0, r8, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D68CD4: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82D68CD8: D00AFFF8  stfs f0, -8(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82D68CDC: 811F0024  lwz r8, 0x24(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D68CE0: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82D68CE4: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D68CE8: 7C081C2E  lfsx f0, r8, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D68CEC: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82D68CF0: D00AFFFC  stfs f0, -4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82D68CF4: 811F0024  lwz r8, 0x24(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D68CF8: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82D68CFC: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D68D00: 7C081C2E  lfsx f0, r8, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D68D04: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82D68D08: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D68D0C: 811F0024  lwz r8, 0x24(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D68D10: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82D68D14: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D68D18: 7C081C2E  lfsx f0, r8, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D68D1C: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82D68D20: D00A0004  stfs f0, 4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D68D24: 811F0024  lwz r8, 0x24(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D68D28: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82D68D2C: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82D68D30: 409AFF94  bne cr6, 0x82d68cc4
	if !ctx.cr[6].eq {
	pc = 0x82D68CC4; continue 'dispatch;
	}
	// 82D68D34: 7F07D840  cmplw cr6, r7, r27
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82D68D38: 40980038  bge cr6, 0x82d68d70
	if !ctx.cr[6].lt {
	pc = 0x82D68D70; continue 'dispatch;
	}
	// 82D68D3C: 54E9103A  slwi r9, r7, 2
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D68D40: 7D47D850  subf r10, r7, r27
	ctx.r[10].s64 = ctx.r[27].s64 - ctx.r[7].s64;
	// 82D68D44: 7D29C214  add r9, r9, r24
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[24].u64;
	// 82D68D48: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D68D4C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D68D50: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D68D54: 7C081C2E  lfsx f0, r8, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D68D58: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82D68D5C: D0090000  stfs f0, 0(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D68D60: 811F0024  lwz r8, 0x24(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D68D64: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82D68D68: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82D68D6C: 409AFFDC  bne cr6, 0x82d68d48
	if !ctx.cr[6].eq {
	pc = 0x82D68D48; continue 'dispatch;
	}
	// 82D68D70: 7D57B02E  lwzx r10, r23, r22
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82D68D74: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D68D78: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D68D7C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D68D80: 40980018  bge cr6, 0x82d68d98
	if !ctx.cr[6].lt {
	pc = 0x82D68D98; continue 'dispatch;
	}
	// 82D68D84: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82D68D88: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D68D8C: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82D68D90: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D68D94: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D68D98: 3BBE0001  addi r29, r30, 1
	ctx.r[29].s64 = ctx.r[30].s64 + 1;
	// 82D68D9C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82D68DA0: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82D68DA4: 419A0020  beq cr6, 0x82d68dc4
	if ctx.cr[6].eq {
	pc = 0x82D68DC4; continue 'dispatch;
	}
	// 82D68DA8: 38C10174  addi r6, r1, 0x174
	ctx.r[6].s64 = ctx.r[1].s64 + 372;
	// 82D68DAC: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82D68DB0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D68DB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D68DB8: 4BFFFA71  bl 0x82d68828
	ctx.lr = 0x82D68DBC;
	sub_82D68828(ctx, base);
	// 82D68DBC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D68DC0: 409A001C  bne cr6, 0x82d68ddc
	if !ctx.cr[6].eq {
	pc = 0x82D68DDC; continue 'dispatch;
	}
	// 82D68DC4: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82D68DC8: 7EA5AB78  mr r5, r21
	ctx.r[5].u64 = ctx.r[21].u64;
	// 82D68DCC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D68DD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D68DD4: 4BFFF955  bl 0x82d68728
	ctx.lr = 0x82D68DD8;
	sub_82D68728(ctx, base);
	// 82D68DD8: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82D68DDC: 7D57B02E  lwzx r10, r23, r22
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82D68DE0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D68DE4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D68DE8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D68DEC: 40980018  bge cr6, 0x82d68e04
	if !ctx.cr[6].lt {
	pc = 0x82D68E04; continue 'dispatch;
	}
	// 82D68DF0: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82D68DF4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D68DF8: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82D68DFC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D68E00: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D68E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82D68E08: 2F1B0004  cmpwi cr6, r27, 4
	ctx.cr[6].compare_i32(ctx.r[27].s32, 4, &mut ctx.xer);
	// 82D68E0C: 41980098  blt cr6, 0x82d68ea4
	if ctx.cr[6].lt {
	pc = 0x82D68EA4; continue 'dispatch;
	}
	// 82D68E10: 395BFFFC  addi r10, r27, -4
	ctx.r[10].s64 = ctx.r[27].s64 + -4;
	// 82D68E14: 39780008  addi r11, r24, 8
	ctx.r[11].s64 = ctx.r[24].s64 + 8;
	// 82D68E18: 554AF0BE  srwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D68E1C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D68E20: 5547103A  slwi r7, r10, 2
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D68E24: 57C9103A  slwi r9, r30, 2
	ctx.r[9].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D68E28: C00BFFF8  lfs f0, -8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D68E2C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D68E30: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D68E34: 7DA91C2E  lfsx f13, r9, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D68E38: EC0D07BA  fmadds f0, f13, f30, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D68E3C: D00BFFF8  stfs f0, -8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82D68E40: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D68E44: C00BFFFC  lfs f0, -4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D68E48: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 82D68E4C: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D68E50: 7DA81C2E  lfsx f13, r8, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D68E54: EC0D07BA  fmadds f0, f13, f30, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D68E58: D00BFFFC  stfs f0, -4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82D68E5C: 811F0024  lwz r8, 0x24(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D68E60: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D68E64: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82D68E68: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D68E6C: 7DA81C2E  lfsx f13, r8, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D68E70: EC0D07BA  fmadds f0, f13, f30, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D68E74: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D68E78: 811F0024  lwz r8, 0x24(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D68E7C: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D68E80: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82D68E84: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D68E88: 7DA81C2E  lfsx f13, r8, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D68E8C: EC0D07BA  fmadds f0, f13, f30, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D68E90: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D68E94: 811F0024  lwz r8, 0x24(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D68E98: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82D68E9C: 7FC84A14  add r30, r8, r9
	ctx.r[30].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82D68EA0: 409AFF84  bne cr6, 0x82d68e24
	if !ctx.cr[6].eq {
	pc = 0x82D68E24; continue 'dispatch;
	}
	// 82D68EA4: 7F07D840  cmplw cr6, r7, r27
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82D68EA8: 4098003C  bge cr6, 0x82d68ee4
	if !ctx.cr[6].lt {
	pc = 0x82D68EE4; continue 'dispatch;
	}
	// 82D68EAC: 54EB103A  slwi r11, r7, 2
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D68EB0: 7D47D850  subf r10, r7, r27
	ctx.r[10].s64 = ctx.r[27].s64 - ctx.r[7].s64;
	// 82D68EB4: 7D6BC214  add r11, r11, r24
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 82D68EB8: 57C9103A  slwi r9, r30, 2
	ctx.r[9].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D68EBC: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D68EC0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D68EC4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D68EC8: 7DA91C2E  lfsx f13, r9, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D68ECC: EC0D07BA  fmadds f0, f13, f30, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D68ED0: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D68ED4: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D68ED8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D68EDC: 7FC9F214  add r30, r9, r30
	ctx.r[30].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 82D68EE0: 409AFFD8  bne cr6, 0x82d68eb8
	if !ctx.cr[6].eq {
	pc = 0x82D68EB8; continue 'dispatch;
	}
	// 82D68EE4: 7D57B02E  lwzx r10, r23, r22
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82D68EE8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D68EEC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D68EF0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D68EF4: 40980020  bge cr6, 0x82d68f14
	if !ctx.cr[6].lt {
	pc = 0x82D68F14; continue 'dispatch;
	}
	// 82D68EF8: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D68EFC: 3929653C  addi r9, r9, 0x653c
	ctx.r[9].s64 = ctx.r[9].s64 + 25916;
	// 82D68F00: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D68F04: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D68F08: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82D68F0C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D68F10: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D68F14: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82D68F18: 7E258B78  mr r5, r17
	ctx.r[5].u64 = ctx.r[17].u64;
	// 82D68F1C: 815F0054  lwz r10, 0x54(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D68F20: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82D68F24: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82D68F28: 92610058  stw r19, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[19].u32 ) };
	// 82D68F2C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D68F30: 9241005C  stw r18, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[18].u32 ) };
	// 82D68F34: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82D68F38: 48003759  bl 0x82d6c690
	ctx.lr = 0x82D68F3C;
	sub_82D6C690(ctx, base);
	// 82D68F3C: 7D77B02E  lwzx r11, r23, r22
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82D68F40: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D68F44: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D68F48: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D68F4C: 40980020  bge cr6, 0x82d68f6c
	if !ctx.cr[6].lt {
	pc = 0x82D68F6C; continue 'dispatch;
	}
	// 82D68F50: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D68F54: 39296468  addi r9, r9, 0x6468
	ctx.r[9].s64 = ctx.r[9].s64 + 25704;
	// 82D68F58: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D68F5C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D68F60: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82D68F64: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D68F68: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D68F6C: 7C77A02E  lwzx r3, r23, r20
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 82D68F70: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D68F74: 93030020  stw r24, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[24].u32 ) };
	// 82D68F78: 7F185840  cmplw cr6, r24, r11
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82D68F7C: 409A0018  bne cr6, 0x82d68f94
	if !ctx.cr[6].eq {
	pc = 0x82D68F94; continue 'dispatch;
	}
	// 82D68F80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D68F84: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82D68F88: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D68F8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D68F90: 4E800421  bctrl
	ctx.lr = 0x82D68F94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D68F94: 7C77A02E  lwzx r3, r23, r20
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 82D68F98: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D68F9C: 92A30020  stw r21, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[21].u32 ) };
	// 82D68FA0: 7F155840  cmplw cr6, r21, r11
	ctx.cr[6].compare_u32(ctx.r[21].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82D68FA4: 409A0018  bne cr6, 0x82d68fbc
	if !ctx.cr[6].eq {
	pc = 0x82D68FBC; continue 'dispatch;
	}
	// 82D68FA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D68FAC: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 82D68FB0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D68FB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D68FB8: 4E800421  bctrl
	ctx.lr = 0x82D68FBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D68FBC: 38210140  addi r1, r1, 0x140
	ctx.r[1].s64 = ctx.r[1].s64 + 320;
	// 82D68FC0: CBC1FF70  lfd f30, -0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-144 as u32) ) };
	// 82D68FC4: CBE1FF78  lfd f31, -0x88(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-136 as u32) ) };
	// 82D68FC8: 4BF40464  b 0x82ca942c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D68FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D68FD0 size=8
    let mut pc: u32 = 0x82D68FD0;
    'dispatch: loop {
        match pc {
            0x82D68FD0 => {
    //   block [0x82D68FD0..0x82D68FD8)
	// 82D68FD0: C0230030  lfs f1, 0x30(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82D68FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D68FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D68FD8 size=128
    let mut pc: u32 = 0x82D68FD8;
    'dispatch: loop {
        match pc {
            0x82D68FD8 => {
    //   block [0x82D68FD8..0x82D69058)
	// 82D68FD8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D68FDC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82D68FE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82D68FE4: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82D68FE8: 38E00030  li r7, 0x30
	ctx.r[7].s64 = 48;
	// 82D68FEC: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D68FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D68FF4: C1AA0C14  lfs f13, 0xc14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D68FF8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82D68FFC: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82D69000: D001FFE0  stfs f0, -0x20(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 82D69004: D001FFE4  stfs f0, -0x1c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-28 as u32), tmp.u32 ) };
	// 82D69008: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D6900C: D1A1FFE8  stfs f13, -0x18(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), tmp.u32 ) };
	// 82D69010: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D69014: D001FFEC  stfs f0, -0x14(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-20 as u32), tmp.u32 ) };
	// 82D69018: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82D6901C: D1A1FFF0  stfs f13, -0x10(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82D69020: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82D69024: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 82D69028: 99230011  stb r9, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[9].u8 ) };
	// 82D6902C: D001FFF8  stfs f0, -8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82D69030: 99630012  stb r11, 0x12(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(18 as u32), ctx.r[11].u8 ) };
	// 82D69034: D001FFFC  stfs f0, -4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82D69038: 99630013  stb r11, 0x13(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(19 as u32), ctx.r[11].u8 ) };
	// 82D6903C: 3961FFE0  addi r11, r1, -0x20
	ctx.r[11].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D69058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D69058 size=108
    let mut pc: u32 = 0x82D69058;
    'dispatch: loop {
        match pc {
            0x82D69058 => {
    //   block [0x82D69058..0x82D690C4)
	// 82D69058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6905C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D69060: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D69064: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D69068: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D6906C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D69070: 396B5B34  addi r11, r11, 0x5b34
	ctx.r[11].s64 = ctx.r[11].s64 + 23348;
	// 82D69074: A15F0004  lhz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D69078: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D6907C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D69080: 419A0024  beq cr6, 0x82d690a4
	if ctx.cr[6].eq {
	pc = 0x82D690A4; continue 'dispatch;
	}
	// 82D69084: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D69088: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D6908C: 813F0038  lwz r9, 0x38(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82D69090: 38C00039  li r6, 0x39
	ctx.r[6].s64 = 57;
	// 82D69094: 809F0034  lwz r4, 0x34(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82D69098: 55252036  slwi r5, r9, 4
	ctx.r[5].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6909C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D690A0: 4BFEC229  bl 0x82d552c8
	ctx.lr = 0x82D690A4;
	sub_82D552C8(ctx, base);
	// 82D690A4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D690A8: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D690AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D690B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D690B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D690B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D690BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D690C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D690C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D690C8 size=352
    let mut pc: u32 = 0x82D690C8;
    'dispatch: loop {
        match pc {
            0x82D690C8 => {
    //   block [0x82D690C8..0x82D69228)
	// 82D690C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D690CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D690D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D690D4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D690D8: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82D690DC: C0030030  lfs f0, 0x30(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D690E0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82D690E4: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82D690E8: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82D690EC: 7D4A07B4  extsw r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82D690F0: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82D690F4: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82D690F8: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82D690FC: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82D69100: EDA06824  fdivs f13, f0, f13
	ctx.f[13].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82D69104: 4198001C  blt cr6, 0x82d69120
	if ctx.cr[6].lt {
	pc = 0x82D69120; continue 'dispatch;
	}
	// 82D69108: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82D6910C: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D69110: 3920FFF0  li r9, -0x10
	ctx.r[9].s64 = -16;
	// 82D69114: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D69228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D69228 size=348
    let mut pc: u32 = 0x82D69228;
    'dispatch: loop {
        match pc {
            0x82D69228 => {
    //   block [0x82D69228..0x82D69384)
	// 82D69228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6922C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D69230: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D69234: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D69238: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6923C: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82D69240: 38830020  addi r4, r3, 0x20
	ctx.r[4].s64 = ctx.r[3].s64 + 32;
	// 82D69244: 39630030  addi r11, r3, 0x30
	ctx.r[11].s64 = ctx.r[3].s64 + 48;
	// 82D69248: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82D6924C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D69388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D69388 size=976
    let mut pc: u32 = 0x82D69388;
    'dispatch: loop {
        match pc {
            0x82D69388 => {
    //   block [0x82D69388..0x82D69758)
	// 82D69388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6938C: 4BF40069  bl 0x82ca93f4
	ctx.lr = 0x82D69390;
	sub_82CA93D0(ctx, base);
	// 82D69390: DBA1FF98  stfd f29, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[29].u64 ) };
	// 82D69394: DBC1FFA0  stfd f30, -0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[30].u64 ) };
	// 82D69398: DBE1FFA8  stfd f31, -0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 82D6939C: 3981FF90  addi r12, r1, -0x70
	ctx.r[12].s64 = ctx.r[1].s64 + -112;
	// 82D693A0: 4829D62D  bl 0x830069cc
	ctx.lr = 0x82D693A4;
	sub_83006760(ctx, base);
	// 82D693A4: 9421FC20  stwu r1, -0x3e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-992 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D693A8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D693AC: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82D693B0: 396B5B34  addi r11, r11, 0x5b34
	ctx.r[11].s64 = ctx.r[11].s64 + 23348;
	// 82D693B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D693B8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D693BC: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D693C0: B1590006  sth r10, 6(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D693C4: 831F000C  lwz r24, 0xc(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D693C8: 2F18FFFF  cmpwi cr6, r24, -1
	ctx.cr[6].compare_i32(ctx.r[24].s32, -1, &mut ctx.xer);
	// 82D693CC: 409A0008  bne cr6, 0x82d693d4
	if !ctx.cr[6].eq {
	pc = 0x82D693D4; continue 'dispatch;
	}
	// 82D693D0: 831F0004  lwz r24, 4(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D693D4: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82D693D8: 3BBF0020  addi r29, r31, 0x20
	ctx.r[29].s64 = ctx.r[31].s64 + 32;
	// 82D693DC: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82D693E0: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 82D693E4: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D69758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D69758 size=912
    let mut pc: u32 = 0x82D69758;
    'dispatch: loop {
        match pc {
            0x82D69758 => {
    //   block [0x82D69758..0x82D69AE8)
	// 82D69758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6975C: 4BF3FCAD  bl 0x82ca9408
	ctx.lr = 0x82D69760;
	sub_82CA93D0(ctx, base);
	// 82D69760: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82D69764: 9421FE20  stwu r1, -0x1e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-480 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D69768: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D6976C: FFE01090  fmr f31, f2
	ctx.f[31].f64 = ctx.f[2].f64;
	// 82D69770: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82D69774: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82D69778: 38A10180  addi r5, r1, 0x180
	ctx.r[5].s64 = ctx.r[1].s64 + 384;
	// 82D6977C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D69780: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D69784: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D69788: 4E800421  bctrl
	ctx.lr = 0x82D6978C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D6978C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D69790: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82D69794: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82D69798: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D6979C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D697A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D697A4: 4E800421  bctrl
	ctx.lr = 0x82D697A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D697A8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D697AC: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82D697B0: 3B8B4CB0  addi r28, r11, 0x4cb0
	ctx.r[28].s64 = ctx.r[11].s64 + 19632;
	// 82D697B4: 419A024C  beq cr6, 0x82d69a00
	if ctx.cr[6].eq {
	pc = 0x82D69A00; continue 'dispatch;
	}
	// 82D697B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D697BC: C03F0030  lfs f1, 0x30(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82D697C0: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82D697C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D697C8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D697CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D697D0: 4E800421  bctrl
	ctx.lr = 0x82D697D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D697D4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82D697D8: 7D7E00D0  neg r11, r30
	ctx.r[11].s64 = -ctx.r[30].s64;
	// 82D697DC: 41980008  blt cr6, 0x82d697e4
	if ctx.cr[6].lt {
	pc = 0x82D697E4; continue 'dispatch;
	}
	// 82D697E0: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82D697E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D697E8: 419A0218  beq cr6, 0x82d69a00
	if ctx.cr[6].eq {
	pc = 0x82D69A00; continue 'dispatch;
	}
	// 82D697EC: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D69AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D69AE8 size=112
    let mut pc: u32 = 0x82D69AE8;
    'dispatch: loop {
        match pc {
            0x82D69AE8 => {
    //   block [0x82D69AE8..0x82D69B58)
	// 82D69AE8: 3946FFFF  addi r10, r6, -1
	ctx.r[10].s64 = ctx.r[6].s64 + -1;
	// 82D69AEC: EC011024  fdivs f0, f1, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ((ctx.f[1].f64 / ctx.f[2].f64) as f32) as f64;
	// 82D69AF0: 3961FFD0  addi r11, r1, -0x30
	ctx.r[11].s64 = ctx.r[1].s64 + -48;
	// 82D69AF4: 79490020  clrldi r9, r10, 0x20
	ctx.r[9].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82D69AF8: 390AFFFF  addi r8, r10, -1
	ctx.r[8].s64 = ctx.r[10].s64 + -1;
	// 82D69AFC: F921FFD8  std r9, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.r[9].u64 ) };
	// 82D69B00: C9A1FFD8  lfd f13, -0x28(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82D69B04: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82D69B08: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82D69B0C: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82D69B10: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82D69B14: FDA0065E  fctidz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82D69B18: 7DA05FAE  stfiwx f13, 0, r11
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 82D69B1C: 8161FFD0  lwz r11, -0x30(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) } as u64;
	// 82D69B20: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82D69B24: 40990034  ble cr6, 0x82d69b58
	if !ctx.cr[6].gt {
		sub_82D69B58(ctx, base);
		return;
	}
	// 82D69B28: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D69B2C: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82D69B30: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82D69B34: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D69B38: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D69B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D69B58 size=240
    let mut pc: u32 = 0x82D69B58;
    'dispatch: loop {
        match pc {
            0x82D69B58 => {
    //   block [0x82D69B58..0x82D69C48)
	// 82D69B58: 79680020  clrldi r8, r11, 0x20
	ctx.r[8].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82D69B5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82D69B60: 38E1FFD0  addi r7, r1, -0x30
	ctx.r[7].s64 = ctx.r[1].s64 + -48;
	// 82D69B64: 394A4C40  addi r10, r10, 0x4c40
	ctx.r[10].s64 = ctx.r[10].s64 + 19520;
	// 82D69B68: 38C1FFD0  addi r6, r1, -0x30
	ctx.r[6].s64 = ctx.r[1].s64 + -48;
	// 82D69B6C: F901FFD8  std r8, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.r[8].u64 ) };
	// 82D69B70: C9A1FFD8  lfd f13, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82D69B74: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82D69B78: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82D69B7C: 3881FFD0  addi r4, r1, -0x30
	ctx.r[4].s64 = ctx.r[1].s64 + -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D69C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D69C48 size=108
    let mut pc: u32 = 0x82D69C48;
    'dispatch: loop {
        match pc {
            0x82D69C48 => {
    //   block [0x82D69C48..0x82D69CB4)
	// 82D69C48: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D69CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D69CB8 size=60
    let mut pc: u32 = 0x82D69CB8;
    'dispatch: loop {
        match pc {
            0x82D69CB8 => {
    //   block [0x82D69CB8..0x82D69CF4)
	// 82D69CB8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D69CBC: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82D69CC0: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82D69CC4: C1AB0C18  lfs f13, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D69CC8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D69CCC: D1A30004  stfs f13, 4(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D69CD0: B1430000  sth r10, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 82D69CD4: B1230002  sth r9, 2(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 82D69CD8: C00B0BE8  lfs f0, 0xbe8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3048 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D69CDC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82D69CE0: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82D69CE4: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82D69CE8: C18BBE14  lfs f12, -0x41ec(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16876 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82D69CEC: D183000C  stfs f12, 0xc(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82D69CF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D69CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D69CF8 size=8
    let mut pc: u32 = 0x82D69CF8;
    'dispatch: loop {
        match pc {
            0x82D69CF8 => {
    //   block [0x82D69CF8..0x82D69D00)
	// 82D69CF8: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D69CFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D69D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D69D00 size=24
    let mut pc: u32 = 0x82D69D00;
    'dispatch: loop {
        match pc {
            0x82D69D00 => {
    //   block [0x82D69D00..0x82D69D18)
	// 82D69D00: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D69D04: 8143005C  lwz r10, 0x5c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) } as u64;
	// 82D69D08: 0CCB0000  twi 6, r11, 0
	// 82D69D0C: 7D645B96  divwu r11, r4, r11
	ctx.r[11].u32 = ctx.r[4].u32 / ctx.r[11].u32;
	// 82D69D10: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D69D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D69D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D69D18 size=96
    let mut pc: u32 = 0x82D69D18;
    'dispatch: loop {
        match pc {
            0x82D69D18 => {
    //   block [0x82D69D18..0x82D69D78)
	// 82D69D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D69D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D69D20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D69D24: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D69D28: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82D69D2C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82D69D30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D69D34: 4BFFD8CD  bl 0x82d67600
	ctx.lr = 0x82D69D38;
	sub_82D67600(ctx, base);
	// 82D69D38: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D69D3C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D69D40: 0CCB0000  twi 6, r11, 0
	// 82D69D44: 7D2A5B96  divwu r9, r10, r11
	ctx.r[9].u32 = ctx.r[10].u32 / ctx.r[11].u32;
	// 82D69D48: 7D2959D6  mullw r9, r9, r11
	ctx.r[9].s64 = (ctx.r[9].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82D69D4C: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82D69D50: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D69D54: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82D69D58: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82D69D5C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82D69D60: 386B0003  addi r3, r11, 3
	ctx.r[3].s64 = ctx.r[11].s64 + 3;
	// 82D69D64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D69D68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D69D6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D69D70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D69D74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D69D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D69D78 size=60
    let mut pc: u32 = 0x82D69D78;
    'dispatch: loop {
        match pc {
            0x82D69D78 => {
    //   block [0x82D69D78..0x82D69DB4)
	// 82D69D78: 81630054  lwz r11, 0x54(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D69D7C: 3905FFFF  addi r8, r5, -1
	ctx.r[8].s64 = ctx.r[5].s64 + -1;
	// 82D69D80: 8123005C  lwz r9, 0x5c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) } as u64;
	// 82D69D84: 7D6459D6  mullw r11, r4, r11
	ctx.r[11].s64 = (ctx.r[4].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82D69D88: 8143003C  lwz r10, 0x3c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 82D69D8C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82D69D90: 7F044000  cmpw cr6, r4, r8
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82D69D94: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D69D98: 556A073E  clrlwi r10, r11, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82D69D9C: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D69DA0: 99460008  stb r10, 8(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 82D69DA4: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D69DA8: 409A000C  bne cr6, 0x82d69db4
	if !ctx.cr[6].eq {
		sub_82D69DB4(ctx, base);
		return;
	}
	// 82D69DAC: 81230058  lwz r9, 0x58(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D69DB0: 48000008  b 0x82d69db8
	sub_82D69DB4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D69DB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D69DB4 size=28
    let mut pc: u32 = 0x82D69DB4;
    'dispatch: loop {
        match pc {
            0x82D69DB4 => {
    //   block [0x82D69DB4..0x82D69DD0)
	// 82D69DB4: 81230054  lwz r9, 0x54(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D69DB8: 554B063E  clrlwi r11, r10, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82D69DBC: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82D69DC0: 396B000F  addi r11, r11, 0xf
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	// 82D69DC4: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D69DC8: 91660004  stw r11, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D69DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D69DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D69DD0 size=328
    let mut pc: u32 = 0x82D69DD0;
    'dispatch: loop {
        match pc {
            0x82D69DD0 => {
    //   block [0x82D69DD0..0x82D69F18)
	// 82D69DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D69DD4: 4BF3F639  bl 0x82ca940c
	ctx.lr = 0x82D69DD8;
	sub_82CA93D0(ctx, base);
	// 82D69DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D69DDC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D69DE0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82D69DE4: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82D69DE8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82D69DEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D69DF0: 4BFFD811  bl 0x82d67600
	ctx.lr = 0x82D69DF4;
	sub_82D67600(ctx, base);
	// 82D69DF4: 39600070  li r11, 0x70
	ctx.r[11].s64 = 112;
	// 82D69DF8: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82D69DFC: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D69E00: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D69E04: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82D69E08: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82D69E0C: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82D69E10: 396B000F  addi r11, r11, 0xf
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	// 82D69E14: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D69E18: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82D69E1C: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D69E20: 80BF0054  lwz r5, 0x54(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D69E24: 7D4B07B4  extsw r11, r10
	ctx.r[11].s64 = ctx.r[10].s32 as i64;
	// 82D69E28: 80DF005C  lwz r6, 0x5c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82D69E2C: 0CCA0000  twi 6, r10, 0
	// 82D69E30: 811F003C  lwz r8, 0x3c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82D69E34: 7D495396  divwu r10, r9, r10
	ctx.r[10].u32 = ctx.r[9].u32 / ctx.r[10].u32;
	// 82D69E38: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D69E3C: 0CCB0000  twi 6, r11, 0
	// 82D69E40: 7CE95A14  add r7, r9, r11
	ctx.r[7].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82D69E44: 7D2A29D6  mullw r9, r10, r5
	ctx.r[9].s64 = (ctx.r[10].s32 as i64) * (ctx.r[5].s32 as i64);
	// 82D69E48: 7D293214  add r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[6].u64;
	// 82D69E4C: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 82D69E50: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82D69E54: 7D075BD6  divw r8, r7, r11
	ctx.r[8].s32 = ctx.r[7].s32 / ctx.r[11].s32;
	// 82D69E58: 54E7083E  rotlwi r7, r7, 1
	ctx.r[7].u64 = ((ctx.r[7].u32).rotate_left(1)) as u64;
	// 82D69E5C: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 82D69E60: 7D6B3878  andc r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 & !ctx.r[7].u64;
	// 82D69E64: 38E8FFFF  addi r7, r8, -1
	ctx.r[7].s64 = ctx.r[8].s64 + -1;
	// 82D69E68: 0CABFFFF  twi 5, r11, -1
	// 82D69E6C: 552B073E  clrlwi r11, r9, 0x1c
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x0000000Fu64;
	// 82D69E70: 55290036  rlwinm r9, r9, 0, 0, 0x1b
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82D69E74: 7F0A3800  cmpw cr6, r10, r7
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82D69E78: 997E0020  stb r11, 0x20(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u8 ) };
	// 82D69E7C: 913E0018  stw r9, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 82D69E80: 409A000C  bne cr6, 0x82d69e8c
	if !ctx.cr[6].eq {
	pc = 0x82D69E8C; continue 'dispatch;
	}
	// 82D69E84: 813F0058  lwz r9, 0x58(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D69E88: 48000008  b 0x82d69e90
	pc = 0x82D69E90; continue 'dispatch;
	// 82D69E8C: 813F0054  lwz r9, 0x54(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D69E90: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82D69E94: 2F1D0004  cmpwi cr6, r29, 4
	ctx.cr[6].compare_i32(ctx.r[29].s32, 4, &mut ctx.xer);
	// 82D69E98: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82D69E9C: 396B000F  addi r11, r11, 0xf
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	// 82D69EA0: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D69EA4: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82D69EA8: 409A0068  bne cr6, 0x82d69f10
	if !ctx.cr[6].eq {
	pc = 0x82D69F10; continue 'dispatch;
	}
	// 82D69EAC: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 82D69EB0: 815F0054  lwz r10, 0x54(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D69EB4: 0CC80000  twi 6, r8, 0
	// 82D69EB8: 80DF005C  lwz r6, 0x5c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82D69EBC: 7CAB4396  divwu r5, r11, r8
	ctx.r[5].u32 = ctx.r[11].u32 / ctx.r[8].u32;
	// 82D69EC0: 813F003C  lwz r9, 0x3c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82D69EC4: 7D0541D6  mullw r8, r5, r8
	ctx.r[8].s64 = (ctx.r[5].s32 as i64) * (ctx.r[8].s32 as i64);
	// 82D69EC8: 7D685850  subf r11, r8, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 82D69ECC: 7D4B51D6  mullw r10, r11, r10
	ctx.r[10].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82D69ED0: 7D4A3214  add r10, r10, r6
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82D69ED4: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82D69ED8: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82D69EDC: 556A073E  clrlwi r10, r11, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82D69EE0: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D69EE4: 995E002C  stb r10, 0x2c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[10].u8 ) };
	// 82D69EE8: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82D69EEC: 409A000C  bne cr6, 0x82d69ef8
	if !ctx.cr[6].eq {
	pc = 0x82D69EF8; continue 'dispatch;
	}
	// 82D69EF0: 813F0058  lwz r9, 0x58(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D69EF4: 48000008  b 0x82d69efc
	pc = 0x82D69EFC; continue 'dispatch;
	// 82D69EF8: 813F0054  lwz r9, 0x54(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D69EFC: 554B063E  clrlwi r11, r10, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82D69F00: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82D69F04: 396B000F  addi r11, r11, 0xf
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	// 82D69F08: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D69F0C: 917E0028  stw r11, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82D69F10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D69F14: 4BF3F548  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D69F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D69F18 size=252
    let mut pc: u32 = 0x82D69F18;
    'dispatch: loop {
        match pc {
            0x82D69F18 => {
    //   block [0x82D69F18..0x82D6A014)
	// 82D69F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D69F1C: 4BF3F4ED  bl 0x82ca9408
	ctx.lr = 0x82D69F20;
	sub_82CA93D0(ctx, base);
	// 82D69F20: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82D69F24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D69F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D69F2C: 81670024  lwz r11, 0x24(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D69F30: 81270020  lwz r9, 0x20(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D69F34: 0CCB0000  twi 6, r11, 0
	// 82D69F38: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82D69F3C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82D69F40: 7C895BD6  divw r4, r9, r11
	ctx.r[4].s32 = ctx.r[9].s32 / ctx.r[11].s32;
	// 82D69F44: 5529083E  rotlwi r9, r9, 1
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(1)) as u64;
	// 82D69F48: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82D69F4C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82D69F50: 7D6B4878  andc r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & !ctx.r[9].u64;
	// 82D69F54: 0CABFFFF  twi 5, r11, -1
	// 82D69F58: 419A00B8  beq cr6, 0x82d6a010
	if ctx.cr[6].eq {
	pc = 0x82D6A010; continue 'dispatch;
	}
	// 82D69F5C: 8167003C  lwz r11, 0x3c(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(60 as u32) ) } as u64;
	// 82D69F60: 3BC4FFFF  addi r30, r4, -1
	ctx.r[30].s64 = ctx.r[4].s64 + -1;
	// 82D69F64: 80A70054  lwz r5, 0x54(r7)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D69F68: 3BE4FFFF  addi r31, r4, -1
	ctx.r[31].s64 = ctx.r[4].s64 + -1;
	// 82D69F6C: 8127005C  lwz r9, 0x5c(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(92 as u32) ) } as u64;
	// 82D69F70: 390B000F  addi r8, r11, 0xf
	ctx.r[8].s64 = ctx.r[11].s64 + 15;
	// 82D69F74: 7F855850  subf r28, r5, r11
	ctx.r[28].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 82D69F78: 7CC92A14  add r6, r9, r5
	ctx.r[6].u64 = ctx.r[9].u64 + ctx.r[5].u64;
	// 82D69F7C: 55090036  rlwinm r9, r8, 0, 0, 0x1b
	ctx.r[9].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82D69F80: 3BA90070  addi r29, r9, 0x70
	ctx.r[29].s64 = ctx.r[9].s64 + 112;
	// 82D69F84: 7D7C3214  add r11, r28, r6
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[6].u64;
	// 82D69F88: 7F0AF800  cmpw cr6, r10, r31
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D69F8C: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82D69F90: 409A000C  bne cr6, 0x82d69f9c
	if !ctx.cr[6].eq {
	pc = 0x82D69F9C; continue 'dispatch;
	}
	// 82D69F94: 81270058  lwz r9, 0x58(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D69F98: 48000008  b 0x82d69fa0
	pc = 0x82D69FA0; continue 'dispatch;
	// 82D69F9C: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 82D69FA0: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82D69FA4: 7F0AF040  cmplw cr6, r10, r30
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82D69FA8: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82D69FAC: 396B000F  addi r11, r11, 0xf
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	// 82D69FB0: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D69FB4: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82D69FB8: 4098003C  bge cr6, 0x82d69ff4
	if !ctx.cr[6].lt {
	pc = 0x82D69FF4; continue 'dispatch;
	}
	// 82D69FBC: 8127003C  lwz r9, 0x3c(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(60 as u32) ) } as u64;
	// 82D69FC0: 390A0001  addi r8, r10, 1
	ctx.r[8].s64 = ctx.r[10].s64 + 1;
	// 82D69FC4: 7D293214  add r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[6].u64;
	// 82D69FC8: 7F08F800  cmpw cr6, r8, r31
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D69FCC: 5529073E  clrlwi r9, r9, 0x1c
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000000Fu64;
	// 82D69FD0: 409A000C  bne cr6, 0x82d69fdc
	if !ctx.cr[6].eq {
	pc = 0x82D69FDC; continue 'dispatch;
	}
	// 82D69FD4: 81070058  lwz r8, 0x58(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D69FD8: 48000008  b 0x82d69fe0
	pc = 0x82D69FE0; continue 'dispatch;
	// 82D69FDC: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
	// 82D69FE0: 5529063E  clrlwi r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82D69FE4: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82D69FE8: 3929000F  addi r9, r9, 0xf
	ctx.r[9].s64 = ctx.r[9].s64 + 15;
	// 82D69FEC: 55290036  rlwinm r9, r9, 0, 0, 0x1b
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82D69FF0: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82D69FF4: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82D69FF8: 40990008  ble cr6, 0x82d6a000
	if !ctx.cr[6].gt {
	pc = 0x82D6A000; continue 'dispatch;
	}
	// 82D69FFC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82D6A000: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D6A004: 7CC62A14  add r6, r6, r5
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[5].u64;
	// 82D6A008: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82D6A00C: 4198FF78  blt cr6, 0x82d69f84
	if ctx.cr[6].lt {
	pc = 0x82D69F84; continue 'dispatch;
	}
	// 82D6A010: 4BF3F448  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6A018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D6A018 size=212
    let mut pc: u32 = 0x82D6A018;
    'dispatch: loop {
        match pc {
            0x82D6A018 => {
    //   block [0x82D6A018..0x82D6A0EC)
	// 82D6A018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6A01C: 4BF3F3D1  bl 0x82ca93ec
	ctx.lr = 0x82D6A020;
	sub_82CA93D0(ctx, base);
	// 82D6A020: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6A024: F88100D8  std r4, 0xd8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(216 as u32), ctx.r[4].u64 ) };
	// 82D6A028: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82D6A02C: F86100D0  std r3, 0xd0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), ctx.r[3].u64 ) };
	// 82D6A030: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82D6A034: F8A100E0  std r5, 0xe0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[5].u64 ) };
	// 82D6A038: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82D6A03C: F8C100E8  std r6, 0xe8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), ctx.r[6].u64 ) };
	// 82D6A040: 896100DC  lbz r11, 0xdc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(220 as u32) ) } as u64;
	// 82D6A044: 830100D0  lwz r24, 0xd0(r1)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(208 as u32) ) } as u64;
	// 82D6A048: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82D6A04C: 99610051  stb r11, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[11].u8 ) };
	// 82D6A050: 419A0094  beq cr6, 0x82d6a0e4
	if ctx.cr[6].eq {
	pc = 0x82D6A0E4; continue 'dispatch;
	}
	// 82D6A054: 816100D4  lwz r11, 0xd4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82D6A058: 83A100E4  lwz r29, 0xe4(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(228 as u32) ) } as u64;
	// 82D6A05C: 5579103A  slwi r25, r11, 2
	ctx.r[25].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 82D6A060: 816100E0  lwz r11, 0xe0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(224 as u32) ) } as u64;
	// 82D6A064: 834100EC  lwz r26, 0xec(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(236 as u32) ) } as u64;
	// 82D6A068: 82E100E8  lwz r23, 0xe8(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(232 as u32) ) } as u64;
	// 82D6A06C: 7EDD5850  subf r22, r29, r11
	ctx.r[22].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82D6A070: 8AA100DD  lbz r21, 0xdd(r1)
	ctx.r[21].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(221 as u32) ) } as u64;
	// 82D6A074: 838100D8  lwz r28, 0xd8(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(216 as u32) ) } as u64;
	// 82D6A078: 7C16EC2E  lfsx f0, r22, r29
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[29].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A07C: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 82D6A080: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82D6A084: C01D0000  lfs f0, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A088: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82D6A08C: 419A0010  beq cr6, 0x82d6a09c
	if ctx.cr[6].eq {
	pc = 0x82D6A09C; continue 'dispatch;
	}
	// 82D6A090: 7D77F0AE  lbzx r11, r23, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82D6A094: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82D6A098: 48000008  b 0x82d6a0a0
	pc = 0x82D6A0A0; continue 'dispatch;
	// 82D6A09C: 9AA10050  stb r21, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[21].u8 ) };
	// 82D6A0A0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82D6A0A4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D6A0A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D6A0AC: 7C7BD214  add r3, r27, r26
	ctx.r[3].u64 = ctx.r[27].u64 + ctx.r[26].u64;
	// 82D6A0B0: 4808E521  bl 0x82df85d0
	ctx.lr = 0x82D6A0B4;
	sub_82DF85D0(ctx, base);
	// 82D6A0B4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82D6A0B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D6A0BC: 4808F49D  bl 0x82df9558
	ctx.lr = 0x82D6A0C0;
	sub_82DF9558(ctx, base);
	// 82D6A0C0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D6A0C4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82D6A0C8: 7FF9FA14  add r31, r25, r31
	ctx.r[31].u64 = ctx.r[25].u64 + ctx.r[31].u64;
	// 82D6A0CC: 4808EE0D  bl 0x82df8ed8
	ctx.lr = 0x82D6A0D0;
	sub_82DF8ED8(ctx, base);
	// 82D6A0D0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82D6A0D4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82D6A0D8: 7F63DA14  add r27, r3, r27
	ctx.r[27].u64 = ctx.r[3].u64 + ctx.r[27].u64;
	// 82D6A0DC: 7F1EC040  cmplw cr6, r30, r24
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82D6A0E0: 4198FF98  blt cr6, 0x82d6a078
	if ctx.cr[6].lt {
	pc = 0x82D6A078; continue 'dispatch;
	}
	// 82D6A0E4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82D6A0E8: 4BF3F354  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6A0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D6A0F0 size=32
    let mut pc: u32 = 0x82D6A0F0;
    'dispatch: loop {
        match pc {
            0x82D6A0F0 => {
    //   block [0x82D6A0F0..0x82D6A110)
	// 82D6A0F0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6A0F4: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82D6A0F8: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82D6A0FC: 80A30010  lwz r5, 0x10(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6A100: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 82D6A104: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6A108: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D6A10C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6A110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D6A110 size=44
    let mut pc: u32 = 0x82D6A110;
    'dispatch: loop {
        match pc {
            0x82D6A110 => {
    //   block [0x82D6A110..0x82D6A13C)
	// 82D6A110: D021FFF0  stfs f1, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82D6A114: 8961FFF3  lbz r11, -0xd(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-13 as u32) ) } as u64;
	// 82D6A118: 9961FFF4  stb r11, -0xc(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), ctx.r[11].u8 ) };
	// 82D6A11C: 8961FFF2  lbz r11, -0xe(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-14 as u32) ) } as u64;
	// 82D6A120: 9961FFF5  stb r11, -0xb(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-11 as u32), ctx.r[11].u8 ) };
	// 82D6A124: 8961FFF1  lbz r11, -0xf(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-15 as u32) ) } as u64;
	// 82D6A128: 9961FFF6  stb r11, -0xa(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-10 as u32), ctx.r[11].u8 ) };
	// 82D6A12C: 8961FFF0  lbz r11, -0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82D6A130: 9961FFF7  stb r11, -9(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-9 as u32), ctx.r[11].u8 ) };
	// 82D6A134: C021FFF4  lfs f1, -0xc(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82D6A138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6A140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D6A140 size=28
    let mut pc: u32 = 0x82D6A140;
    'dispatch: loop {
        match pc {
            0x82D6A140 => {
    //   block [0x82D6A140..0x82D6A15C)
	// 82D6A140: B0610016  sth r3, 0x16(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(22 as u32), ctx.r[3].u16 ) };
	// 82D6A144: 89610017  lbz r11, 0x17(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(23 as u32) ) } as u64;
	// 82D6A148: 9961FFF0  stb r11, -0x10(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u8 ) };
	// 82D6A14C: 89610016  lbz r11, 0x16(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(22 as u32) ) } as u64;
	// 82D6A150: 9961FFF1  stb r11, -0xf(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-15 as u32), ctx.r[11].u8 ) };
	// 82D6A154: A061FFF0  lhz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82D6A158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6A160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D6A160 size=1132
    let mut pc: u32 = 0x82D6A160;
    'dispatch: loop {
        match pc {
            0x82D6A160 => {
    //   block [0x82D6A160..0x82D6A5CC)
	// 82D6A160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6A164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D6A168: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D6A16C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6A170: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D6A174: 815F0044  lwz r10, 0x44(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82D6A178: 2B0A0004  cmplwi cr6, r10, 4
	ctx.cr[6].compare_u32(ctx.r[10].u32, 4 as u32, &mut ctx.xer);
	// 82D6A17C: 409A0020  bne cr6, 0x82d6a19c
	if !ctx.cr[6].eq {
	pc = 0x82D6A19C; continue 'dispatch;
	}
	// 82D6A180: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82D6A184: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6A188: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D6A18C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82D6A190: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82D6A194: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82D6A198: 48000008  b 0x82d6a1a0
	pc = 0x82D6A1A0; continue 'dispatch;
	// 82D6A19C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D6A1A0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82D6A1A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D6A1A8: 419A0410  beq cr6, 0x82d6a5b8
	if ctx.cr[6].eq {
	pc = 0x82D6A5B8; continue 'dispatch;
	}
	// 82D6A1AC: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82D6A1B0: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6A1B4: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82D6A1B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D6A1BC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D6A1C0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D6A1C4: 40990038  ble cr6, 0x82d6a1fc
	if !ctx.cr[6].gt {
	pc = 0x82D6A1FC; continue 'dispatch;
	}
	// 82D6A1C8: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6A1CC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D6A1D0: B1210050  sth r9, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u16 ) };
	// 82D6A1D4: 89210051  lbz r9, 0x51(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 82D6A1D8: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 82D6A1DC: 89210050  lbz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D6A1E0: 99210055  stb r9, 0x55(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(85 as u32), ctx.r[9].u8 ) };
	// 82D6A1E4: A1210054  lhz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D6A1E8: B12B0000  sth r9, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 82D6A1EC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82D6A1F0: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6A1F4: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D6A1F8: 4198FFD0  blt cr6, 0x82d6a1c8
	if ctx.cr[6].lt {
	pc = 0x82D6A1C8; continue 'dispatch;
	}
	// 82D6A1FC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82D6A200: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6A204: 48001B2D  bl 0x82d6bd30
	ctx.lr = 0x82D6A208;
	sub_82D6BD30(ctx, base);
	// 82D6A208: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82D6A20C: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82D6A210: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82D6A214: 81210060  lwz r9, 0x60(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82D6A218: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D6A21C: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82D6A220: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82D6A224: 813F005C  lwz r9, 0x5c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82D6A228: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82D6A22C: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82D6A230: 419800E4  blt cr6, 0x82d6a314
	if ctx.cr[6].lt {
	pc = 0x82D6A314; continue 'dispatch;
	}
	// 82D6A234: 392AFFFC  addi r9, r10, -4
	ctx.r[9].s64 = ctx.r[10].s64 + -4;
	// 82D6A238: 5529F0BE  srwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D6A23C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82D6A240: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D6A244: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A248: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82D6A24C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82D6A250: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A254: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D6A258: 88E10057  lbz r7, 0x57(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 82D6A25C: 98E10050  stb r7, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u8 ) };
	// 82D6A260: 88E10056  lbz r7, 0x56(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 82D6A264: 98E10051  stb r7, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[7].u8 ) };
	// 82D6A268: 88E10055  lbz r7, 0x55(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 82D6A26C: 98E10052  stb r7, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[7].u8 ) };
	// 82D6A270: 88E10054  lbz r7, 0x54(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D6A274: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82D6A278: 98E10053  stb r7, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[7].u8 ) };
	// 82D6A27C: 88E10057  lbz r7, 0x57(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 82D6A280: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A284: 98E10050  stb r7, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u8 ) };
	// 82D6A288: 88E10056  lbz r7, 0x56(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 82D6A28C: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D6A290: C00B0008  lfs f0, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A294: 98E10051  stb r7, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[7].u8 ) };
	// 82D6A298: 88E10055  lbz r7, 0x55(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 82D6A29C: 98E10052  stb r7, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[7].u8 ) };
	// 82D6A2A0: 88E10054  lbz r7, 0x54(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D6A2A4: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82D6A2A8: 98E10053  stb r7, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[7].u8 ) };
	// 82D6A2AC: 88E10057  lbz r7, 0x57(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 82D6A2B0: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A2B4: 98E10050  stb r7, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u8 ) };
	// 82D6A2B8: 88E10056  lbz r7, 0x56(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 82D6A2BC: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D6A2C0: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A2C4: 98E10051  stb r7, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[7].u8 ) };
	// 82D6A2C8: 88E10055  lbz r7, 0x55(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 82D6A2CC: 98E10052  stb r7, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[7].u8 ) };
	// 82D6A2D0: 88E10054  lbz r7, 0x54(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D6A2D4: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82D6A2D8: 98E10053  stb r7, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[7].u8 ) };
	// 82D6A2DC: 88E10057  lbz r7, 0x57(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 82D6A2E0: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A2E4: 98E10050  stb r7, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u8 ) };
	// 82D6A2E8: 88E10056  lbz r7, 0x56(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 82D6A2EC: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82D6A2F0: 98E10051  stb r7, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[7].u8 ) };
	// 82D6A2F4: 88E10055  lbz r7, 0x55(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 82D6A2F8: 98E10052  stb r7, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[7].u8 ) };
	// 82D6A2FC: 88E10054  lbz r7, 0x54(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D6A300: 98E10053  stb r7, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[7].u8 ) };
	// 82D6A304: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A308: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82D6A30C: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82D6A310: 409AFF34  bne cr6, 0x82d6a244
	if !ctx.cr[6].eq {
	pc = 0x82D6A244; continue 'dispatch;
	}
	// 82D6A314: 7F085000  cmpw cr6, r8, r10
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D6A318: 40980048  bge cr6, 0x82d6a360
	if !ctx.cr[6].lt {
	pc = 0x82D6A360; continue 'dispatch;
	}
	// 82D6A31C: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 82D6A320: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A324: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D6A328: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82D6A32C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D6A330: 89210057  lbz r9, 0x57(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 82D6A334: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 82D6A338: 89210056  lbz r9, 0x56(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 82D6A33C: 99210051  stb r9, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[9].u8 ) };
	// 82D6A340: 89210055  lbz r9, 0x55(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 82D6A344: 99210052  stb r9, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[9].u8 ) };
	// 82D6A348: 89210054  lbz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D6A34C: 99210053  stb r9, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[9].u8 ) };
	// 82D6A350: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A354: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D6A358: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D6A35C: 409AFFC4  bne cr6, 0x82d6a320
	if !ctx.cr[6].eq {
	pc = 0x82D6A320; continue 'dispatch;
	}
	// 82D6A360: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82D6A364: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82D6A368: 8141007C  lwz r10, 0x7c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82D6A36C: 81010078  lwz r8, 0x78(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82D6A370: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D6A374: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82D6A378: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82D6A37C: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82D6A380: 811F0034  lwz r8, 0x34(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82D6A384: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82D6A388: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82D6A38C: 2F090004  cmpwi cr6, r9, 4
	ctx.cr[6].compare_i32(ctx.r[9].s32, 4, &mut ctx.xer);
	// 82D6A390: 419801A8  blt cr6, 0x82d6a538
	if ctx.cr[6].lt {
	pc = 0x82D6A538; continue 'dispatch;
	}
	// 82D6A394: 3909FFFC  addi r8, r9, -4
	ctx.r[8].s64 = ctx.r[9].s64 + -4;
	// 82D6A398: 5508F0BE  srwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D6A39C: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82D6A3A0: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D6A3A4: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A3A8: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82D6A3AC: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82D6A3B0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82D6A3B4: 88C10057  lbz r6, 0x57(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 82D6A3B8: 98C10050  stb r6, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u8 ) };
	// 82D6A3BC: 88C10056  lbz r6, 0x56(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 82D6A3C0: 98C10051  stb r6, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[6].u8 ) };
	// 82D6A3C4: 88C10055  lbz r6, 0x55(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 82D6A3C8: 98C10052  stb r6, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[6].u8 ) };
	// 82D6A3CC: 88C10054  lbz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D6A3D0: 98C10053  stb r6, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[6].u8 ) };
	// 82D6A3D4: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A3D8: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D6A3DC: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A3E0: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82D6A3E4: 88C1005B  lbz r6, 0x5b(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(91 as u32) ) } as u64;
	// 82D6A3E8: 98C1005C  stb r6, 0x5c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u8 ) };
	// 82D6A3EC: 88C1005A  lbz r6, 0x5a(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 82D6A3F0: 98C1005D  stb r6, 0x5d(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(93 as u32), ctx.r[6].u8 ) };
	// 82D6A3F4: 88C10059  lbz r6, 0x59(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(89 as u32) ) } as u64;
	// 82D6A3F8: 98C1005E  stb r6, 0x5e(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(94 as u32), ctx.r[6].u8 ) };
	// 82D6A3FC: 88C10058  lbz r6, 0x58(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D6A400: 98C1005F  stb r6, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[6].u8 ) };
	// 82D6A404: C001005C  lfs f0, 0x5c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A408: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D6A40C: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A410: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82D6A414: 88C10057  lbz r6, 0x57(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 82D6A418: 98C10050  stb r6, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u8 ) };
	// 82D6A41C: 88C10056  lbz r6, 0x56(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 82D6A420: 98C10051  stb r6, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[6].u8 ) };
	// 82D6A424: 88C10055  lbz r6, 0x55(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 82D6A428: 98C10052  stb r6, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[6].u8 ) };
	// 82D6A42C: 88C10054  lbz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D6A430: 98C10053  stb r6, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[6].u8 ) };
	// 82D6A434: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A438: D00A0004  stfs f0, 4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D6A43C: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A440: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82D6A444: 88C1005B  lbz r6, 0x5b(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(91 as u32) ) } as u64;
	// 82D6A448: 98C1005C  stb r6, 0x5c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u8 ) };
	// 82D6A44C: 88C1005A  lbz r6, 0x5a(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 82D6A450: 98C1005D  stb r6, 0x5d(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(93 as u32), ctx.r[6].u8 ) };
	// 82D6A454: 88C10059  lbz r6, 0x59(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(89 as u32) ) } as u64;
	// 82D6A458: 98C1005E  stb r6, 0x5e(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(94 as u32), ctx.r[6].u8 ) };
	// 82D6A45C: 88C10058  lbz r6, 0x58(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D6A460: 98C1005F  stb r6, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[6].u8 ) };
	// 82D6A464: C001005C  lfs f0, 0x5c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A468: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D6A46C: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A470: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82D6A474: 88C10057  lbz r6, 0x57(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 82D6A478: 98C10050  stb r6, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u8 ) };
	// 82D6A47C: 88C10056  lbz r6, 0x56(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 82D6A480: 98C10051  stb r6, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[6].u8 ) };
	// 82D6A484: 88C10055  lbz r6, 0x55(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 82D6A488: 98C10052  stb r6, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[6].u8 ) };
	// 82D6A48C: 88C10054  lbz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D6A490: 98C10053  stb r6, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[6].u8 ) };
	// 82D6A494: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A498: D00A0008  stfs f0, 8(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82D6A49C: C00B0008  lfs f0, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A4A0: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82D6A4A4: 88C1005B  lbz r6, 0x5b(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(91 as u32) ) } as u64;
	// 82D6A4A8: 98C1005C  stb r6, 0x5c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u8 ) };
	// 82D6A4AC: 88C1005A  lbz r6, 0x5a(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 82D6A4B0: 98C1005D  stb r6, 0x5d(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(93 as u32), ctx.r[6].u8 ) };
	// 82D6A4B4: 88C10059  lbz r6, 0x59(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(89 as u32) ) } as u64;
	// 82D6A4B8: 98C1005E  stb r6, 0x5e(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(94 as u32), ctx.r[6].u8 ) };
	// 82D6A4BC: 88C10058  lbz r6, 0x58(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D6A4C0: 98C1005F  stb r6, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[6].u8 ) };
	// 82D6A4C4: C001005C  lfs f0, 0x5c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A4C8: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82D6A4CC: C00A000C  lfs f0, 0xc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A4D0: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82D6A4D4: 88C10057  lbz r6, 0x57(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 82D6A4D8: 98C10050  stb r6, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u8 ) };
	// 82D6A4DC: 88C10056  lbz r6, 0x56(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 82D6A4E0: 98C10051  stb r6, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[6].u8 ) };
	// 82D6A4E4: 88C10055  lbz r6, 0x55(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 82D6A4E8: 98C10052  stb r6, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[6].u8 ) };
	// 82D6A4EC: 88C10054  lbz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D6A4F0: 98C10053  stb r6, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[6].u8 ) };
	// 82D6A4F4: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A4F8: D00A000C  stfs f0, 0xc(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82D6A4FC: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82D6A500: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A504: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82D6A508: 88C1005B  lbz r6, 0x5b(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(91 as u32) ) } as u64;
	// 82D6A50C: 98C1005C  stb r6, 0x5c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u8 ) };
	// 82D6A510: 88C1005A  lbz r6, 0x5a(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 82D6A514: 98C1005D  stb r6, 0x5d(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(93 as u32), ctx.r[6].u8 ) };
	// 82D6A518: 88C10059  lbz r6, 0x59(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(89 as u32) ) } as u64;
	// 82D6A51C: 98C1005E  stb r6, 0x5e(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(94 as u32), ctx.r[6].u8 ) };
	// 82D6A520: 88C10058  lbz r6, 0x58(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D6A524: 98C1005F  stb r6, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[6].u8 ) };
	// 82D6A528: C001005C  lfs f0, 0x5c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A52C: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82D6A530: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82D6A534: 409AFE70  bne cr6, 0x82d6a3a4
	if !ctx.cr[6].eq {
	pc = 0x82D6A3A4; continue 'dispatch;
	}
	// 82D6A538: 7F074840  cmplw cr6, r7, r9
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D6A53C: 4098007C  bge cr6, 0x82d6a5b8
	if !ctx.cr[6].lt {
	pc = 0x82D6A5B8; continue 'dispatch;
	}
	// 82D6A540: 7D274850  subf r9, r7, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	// 82D6A544: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A548: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82D6A54C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82D6A550: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D6A554: 89010057  lbz r8, 0x57(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 82D6A558: 99010050  stb r8, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u8 ) };
	// 82D6A55C: 89010056  lbz r8, 0x56(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 82D6A560: 99010051  stb r8, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[8].u8 ) };
	// 82D6A564: 89010055  lbz r8, 0x55(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 82D6A568: 99010052  stb r8, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[8].u8 ) };
	// 82D6A56C: 89010054  lbz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D6A570: 99010053  stb r8, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[8].u8 ) };
	// 82D6A574: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A578: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D6A57C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82D6A580: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A584: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82D6A588: 8901005B  lbz r8, 0x5b(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(91 as u32) ) } as u64;
	// 82D6A58C: 9901005C  stb r8, 0x5c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u8 ) };
	// 82D6A590: 8901005A  lbz r8, 0x5a(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 82D6A594: 9901005D  stb r8, 0x5d(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(93 as u32), ctx.r[8].u8 ) };
	// 82D6A598: 89010059  lbz r8, 0x59(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(89 as u32) ) } as u64;
	// 82D6A59C: 9901005E  stb r8, 0x5e(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(94 as u32), ctx.r[8].u8 ) };
	// 82D6A5A0: 89010058  lbz r8, 0x58(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D6A5A4: 9901005F  stb r8, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[8].u8 ) };
	// 82D6A5A8: C001005C  lfs f0, 0x5c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A5AC: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D6A5B0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D6A5B4: 409AFF90  bne cr6, 0x82d6a544
	if !ctx.cr[6].eq {
	pc = 0x82D6A544; continue 'dispatch;
	}
	// 82D6A5B8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82D6A5BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D6A5C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D6A5C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D6A5C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6A5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D6A5D0 size=104
    let mut pc: u32 = 0x82D6A5D0;
    'dispatch: loop {
        match pc {
            0x82D6A5D0 => {
    //   block [0x82D6A5D0..0x82D6A638)
	// 82D6A5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6A5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D6A5D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D6A5DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6A5E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D6A5E4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D6A5E8: 396B5D64  addi r11, r11, 0x5d64
	ctx.r[11].s64 = ctx.r[11].s64 + 23908;
	// 82D6A5EC: A15F0004  lhz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6A5F0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D6A5F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D6A5F8: 419A0020  beq cr6, 0x82d6a618
	if ctx.cr[6].eq {
	pc = 0x82D6A618; continue 'dispatch;
	}
	// 82D6A5FC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6A600: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D6A604: 38C00037  li r6, 0x37
	ctx.r[6].s64 = 55;
	// 82D6A608: 80BF0060  lwz r5, 0x60(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82D6A60C: 809F005C  lwz r4, 0x5c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82D6A610: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D6A614: 4BFEACB5  bl 0x82d552c8
	ctx.lr = 0x82D6A618;
	sub_82D552C8(ctx, base);
	// 82D6A618: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D6A61C: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D6A620: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D6A624: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D6A628: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D6A62C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D6A630: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D6A634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6A638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D6A638 size=320
    let mut pc: u32 = 0x82D6A638;
    'dispatch: loop {
        match pc {
            0x82D6A638 => {
    //   block [0x82D6A638..0x82D6A778)
	// 82D6A638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6A63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D6A640: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D6A644: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D6A648: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6A64C: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6A650: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 82D6A654: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 82D6A658: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82D6A65C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6A660: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6A664: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D6A668: 40980020  bge cr6, 0x82d6a688
	if !ctx.cr[6].lt {
	pc = 0x82D6A688; continue 'dispatch;
	}
	// 82D6A66C: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D6A670: 392966A4  addi r9, r9, 0x66a4
	ctx.r[9].s64 = ctx.r[9].s64 + 26276;
	// 82D6A674: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D6A678: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D6A67C: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82D6A680: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D6A684: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D6A688: 81430024  lwz r10, 0x24(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6A68C: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D6A690: 5549003E  slwi r9, r10, 0
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D6A694: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D6A698: 7D044B96  divwu r8, r4, r9
	ctx.r[8].u32 = ctx.r[4].u32 / ctx.r[9].u32;
	// 82D6A69C: 0CC90000  twi 6, r9, 0
	// 82D6A6A0: 4099000C  ble cr6, 0x82d6a6ac
	if !ctx.cr[6].gt {
	pc = 0x82D6A6AC; continue 'dispatch;
	}
	// 82D6A6A4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82D6A6A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82D6A6AC: 7C8941D6  mullw r4, r9, r8
	ctx.r[4].s64 = (ctx.r[9].s32 as i64) * (ctx.r[8].s32 as i64);
	// 82D6A6B0: 7CA45214  add r5, r4, r10
	ctx.r[5].u64 = ctx.r[4].u64 + ctx.r[10].u64;
	// 82D6A6B4: 7F055800  cmpw cr6, r5, r11
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D6A6B8: 40990008  ble cr6, 0x82d6a6c0
	if !ctx.cr[6].gt {
	pc = 0x82D6A6C0; continue 'dispatch;
	}
	// 82D6A6BC: 7D445850  subf r10, r4, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82D6A6C0: 90C10050  stw r6, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u32 ) };
	// 82D6A6C4: 80C30054  lwz r6, 0x54(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D6A6C8: 88A30029  lbz r5, 0x29(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(41 as u32) ) } as u64;
	// 82D6A6CC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82D6A6D0: 7D4641D6  mullw r10, r6, r8
	ctx.r[10].s64 = (ctx.r[6].s32 as i64) * (ctx.r[8].s32 as i64);
	// 82D6A6D4: 8083003C  lwz r4, 0x3c(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 82D6A6D8: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82D6A6DC: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82D6A6E0: 8163005C  lwz r11, 0x5c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) } as u64;
	// 82D6A6E4: 80C30030  lwz r6, 0x30(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82D6A6E8: 81030034  lwz r8, 0x34(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82D6A6EC: 81230038  lwz r9, 0x38(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82D6A6F0: 98A1005C  stb r5, 0x5c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[5].u8 ) };
	// 82D6A6F4: 88A30028  lbz r5, 0x28(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D6A6F8: E8610050  ld r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82D6A6FC: 7CC65A14  add r6, r6, r11
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[11].u64;
	// 82D6A700: 7D085A14  add r8, r8, r11
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82D6A704: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82D6A708: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82D6A70C: 98A1005D  stb r5, 0x5d(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(93 as u32), ctx.r[5].u8 ) };
	// 82D6A710: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82D6A714: 90C10060  stw r6, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 82D6A718: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 82D6A71C: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 82D6A720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82D6A724: E8A10060  ld r5, 0x60(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82D6A728: E8C10068  ld r6, 0x68(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 82D6A72C: 4BFFF8ED  bl 0x82d6a018
	ctx.lr = 0x82D6A730;
	sub_82D6A018(ctx, base);
	// 82D6A730: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82D6A734: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6A738: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6A73C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D6A740: 40980020  bge cr6, 0x82d6a760
	if !ctx.cr[6].lt {
	pc = 0x82D6A760; continue 'dispatch;
	}
	// 82D6A744: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D6A748: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82D6A74C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D6A750: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D6A754: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82D6A758: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D6A75C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D6A760: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D6A764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D6A768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D6A76C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D6A770: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D6A774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6A778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D6A778 size=332
    let mut pc: u32 = 0x82D6A778;
    'dispatch: loop {
        match pc {
            0x82D6A778 => {
    //   block [0x82D6A778..0x82D6A8C4)
	// 82D6A778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6A77C: 4BF3EC79  bl 0x82ca93f4
	ctx.lr = 0x82D6A780;
	sub_82CA93D0(ctx, base);
	// 82D6A780: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6A784: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6A788: 3B000008  li r24, 8
	ctx.r[24].s64 = 8;
	// 82D6A78C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82D6A790: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82D6A794: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82D6A798: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82D6A79C: 7D78B82E  lwzx r11, r24, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82D6A7A0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6A7A4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6A7A8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D6A7AC: 40980020  bge cr6, 0x82d6a7cc
	if !ctx.cr[6].lt {
	pc = 0x82D6A7CC; continue 'dispatch;
	}
	// 82D6A7B0: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D6A7B4: 392966BC  addi r9, r9, 0x66bc
	ctx.r[9].s64 = ctx.r[9].s64 + 26300;
	// 82D6A7B8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D6A7BC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D6A7C0: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82D6A7C4: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D6A7C8: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D6A7CC: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6A7D0: 817C0024  lwz r11, 0x24(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6A7D4: 813C005C  lwz r9, 0x5c(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(92 as u32) ) } as u64;
	// 82D6A7D8: 7D5A5B96  divwu r10, r26, r11
	ctx.r[10].u32 = ctx.r[26].u32 / ctx.r[11].u32;
	// 82D6A7DC: 0CCB0000  twi 6, r11, 0
	// 82D6A7E0: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6A7E4: 7D6BC9D6  mullw r11, r11, r25
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[25].s32 as i64);
	// 82D6A7E8: 8108000C  lwz r8, 0xc(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6A7EC: 7FCA4A14  add r30, r10, r9
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82D6A7F0: 557D103A  slwi r29, r11, 2
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82D6A7F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D6A7F8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82D6A7FC: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82D6A800: 4E800421  bctrl
	ctx.lr = 0x82D6A804;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D6A804: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D6A808: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D6A80C: 409A007C  bne cr6, 0x82d6a888
	if !ctx.cr[6].eq {
	pc = 0x82D6A888; continue 'dispatch;
	}
	// 82D6A810: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6A814: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82D6A818: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D6A81C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6A820: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6A824: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D6A828: 4E800421  bctrl
	ctx.lr = 0x82D6A82C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D6A82C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D6A830: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D6A834: 409A0040  bne cr6, 0x82d6a874
	if !ctx.cr[6].eq {
	pc = 0x82D6A874; continue 'dispatch;
	}
	// 82D6A838: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82D6A83C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6A840: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6A844: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D6A848: 40980020  bge cr6, 0x82d6a868
	if !ctx.cr[6].lt {
	pc = 0x82D6A868; continue 'dispatch;
	}
	// 82D6A84C: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D6A850: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82D6A854: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D6A858: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D6A85C: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82D6A860: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D6A864: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D6A868: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D6A86C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82D6A870: 4BF3EBD4  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
	// 82D6A874: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82D6A878: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D6A87C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82D6A880: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82D6A884: 4BFFFDB5  bl 0x82d6a638
	ctx.lr = 0x82D6A888;
	sub_82D6A638(ctx, base);
	// 82D6A888: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82D6A88C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6A890: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6A894: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D6A898: 40980020  bge cr6, 0x82d6a8b8
	if !ctx.cr[6].lt {
	pc = 0x82D6A8B8; continue 'dispatch;
	}
	// 82D6A89C: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D6A8A0: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82D6A8A4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D6A8A8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D6A8AC: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82D6A8B0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D6A8B4: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D6A8B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D6A8BC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82D6A8C0: 4BF3EB84  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6A8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D6A8C8 size=1636
    let mut pc: u32 = 0x82D6A8C8;
    'dispatch: loop {
        match pc {
            0x82D6A8C8 => {
    //   block [0x82D6A8C8..0x82D6AF2C)
	// 82D6A8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6A8CC: 4BF3EB11  bl 0x82ca93dc
	ctx.lr = 0x82D6A8D0;
	sub_82CA93D0(ctx, base);
	// 82D6A8D0: DBC1FF70  stfd f30, -0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), ctx.f[30].u64 ) };
	// 82D6A8D4: DBE1FF78  stfd f31, -0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[31].u64 ) };
	// 82D6A8D8: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6A8DC: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82D6A8E0: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82D6A8E4: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82D6A8E8: 7CB32B78  mr r19, r5
	ctx.r[19].u64 = ctx.r[5].u64;
	// 82D6A8EC: 7CD13378  mr r17, r6
	ctx.r[17].u64 = ctx.r[6].u64;
	// 82D6A8F0: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82D6A8F4: 83FC0000  lwz r31, 0(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6A8F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D6A8FC: 419A001C  beq cr6, 0x82d6a918
	if ctx.cr[6].eq {
	pc = 0x82D6A918; continue 'dispatch;
	}
	// 82D6A900: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D6A904: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D6A908: 396B5D64  addi r11, r11, 0x5d64
	ctx.r[11].s64 = ctx.r[11].s64 + 23908;
	// 82D6A90C: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D6A910: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D6A914: 48000008  b 0x82d6a91c
	pc = 0x82D6A91C; continue 'dispatch;
	// 82D6A918: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82D6A91C: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6A920: 3A400008  li r18, 8
	ctx.r[18].s64 = 8;
	// 82D6A924: 7D57902E  lwzx r10, r23, r18
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 82D6A928: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6A92C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6A930: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D6A934: 4098002C  bge cr6, 0x82d6a960
	if !ctx.cr[6].lt {
	pc = 0x82D6A960; continue 'dispatch;
	}
	// 82D6A938: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D6A93C: 3D008202  lis r8, -0x7dfe
	ctx.r[8].s64 = -2113798144;
	// 82D6A940: 392966FC  addi r9, r9, 0x66fc
	ctx.r[9].s64 = ctx.r[9].s64 + 26364;
	// 82D6A944: 390866F0  addi r8, r8, 0x66f0
	ctx.r[8].s64 = ctx.r[8].s64 + 26352;
	// 82D6A948: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D6A94C: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82D6A950: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D6A954: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82D6A958: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D6A95C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D6A960: 83BC000C  lwz r29, 0xc(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6A964: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82D6A968: 815F0044  lwz r10, 0x44(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82D6A96C: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 82D6A970: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82D6A974: 7EAAEA14  add r21, r10, r29
	ctx.r[21].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 82D6A978: 7E8BEA14  add r20, r11, r29
	ctx.r[20].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82D6A97C: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82D6A980: 48001301  bl 0x82d6bc80
	ctx.lr = 0x82D6A984;
	sub_82D6BC80(ctx, base);
	// 82D6A984: 8121007C  lwz r9, 0x7c(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82D6A988: 81410080  lwz r10, 0x80(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82D6A98C: 7D77902E  lwzx r11, r23, r18
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 82D6A990: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82D6A994: 81210078  lwz r9, 0x78(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82D6A998: 7FCA4A14  add r30, r10, r9
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82D6A99C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6A9A0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6A9A4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D6A9A8: 40980020  bge cr6, 0x82d6a9c8
	if !ctx.cr[6].lt {
	pc = 0x82D6A9C8; continue 'dispatch;
	}
	// 82D6A9AC: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D6A9B0: 392966DC  addi r9, r9, 0x66dc
	ctx.r[9].s64 = ctx.r[9].s64 + 26332;
	// 82D6A9B4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D6A9B8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D6A9BC: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82D6A9C0: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D6A9C4: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D6A9C8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D6A9CC: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6A9D0: 3AC00004  li r22, 4
	ctx.r[22].s64 = 4;
	// 82D6A9D4: C00B0C14  lfs f0, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6A9D8: 7D6AF1D6  mullw r11, r10, r30
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82D6A9DC: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82D6A9E0: EFE0F028  fsubs f31, f0, f30
	ctx.f[31].f64 = (((ctx.f[0].f64 - ctx.f[30].f64) as f32) as f64);
	// 82D6A9E4: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D6A9E8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D6A9EC: 55641036  rlwinm r4, r11, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D6A9F0: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D6A9F4: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82D6A9F8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D6A9FC: 41990010  bgt cr6, 0x82d6aa0c
	if ctx.cr[6].gt {
	pc = 0x82D6AA0C; continue 'dispatch;
	}
	// 82D6AA00: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 82D6AA04: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82D6AA08: 48000018  b 0x82d6aa20
	pc = 0x82D6AA20; continue 'dispatch;
	// 82D6AA0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6AA10: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D6AA14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D6AA18: 4E800421  bctrl
	ctx.lr = 0x82D6AA1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D6AA1C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82D6AA20: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82D6AA24: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 82D6AA28: 55641036  rlwinm r4, r11, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D6AA2C: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D6AA30: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D6AA34: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82D6AA38: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D6AA3C: 41990010  bgt cr6, 0x82d6aa4c
	if ctx.cr[6].gt {
	pc = 0x82D6AA4C; continue 'dispatch;
	}
	// 82D6AA40: 7D795B78  mr r25, r11
	ctx.r[25].u64 = ctx.r[11].u64;
	// 82D6AA44: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82D6AA48: 48000018  b 0x82d6aa60
	pc = 0x82D6AA60; continue 'dispatch;
	// 82D6AA4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6AA50: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D6AA54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D6AA58: 4E800421  bctrl
	ctx.lr = 0x82D6AA5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D6AA5C: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82D6AA60: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6AA64: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D6AA68: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D6AA6C: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82D6AA70: 4199002C  bgt cr6, 0x82d6aa9c
	if ctx.cr[6].gt {
	pc = 0x82D6AA9C; continue 'dispatch;
	}
	// 82D6AA74: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D6AA78: 811F0024  lwz r8, 0x24(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6AA7C: 7D385396  divwu r9, r24, r10
	ctx.r[9].u32 = ctx.r[24].u32 / ctx.r[10].u32;
	// 82D6AA80: 0CCA0000  twi 6, r10, 0
	// 82D6AA84: 38E90001  addi r7, r9, 1
	ctx.r[7].s64 = ctx.r[9].s64 + 1;
	// 82D6AA88: 7D0741D6  mullw r8, r7, r8
	ctx.r[8].s64 = (ctx.r[7].s32 as i64) * (ctx.r[8].s32 as i64);
	// 82D6AA8C: 7F085800  cmpw cr6, r8, r11
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D6AA90: 40990010  ble cr6, 0x82d6aaa0
	if !ctx.cr[6].gt {
	pc = 0x82D6AAA0; continue 'dispatch;
	}
	// 82D6AA94: 7D4A49D6  mullw r10, r10, r9
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82D6AA98: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82D6AA9C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82D6AAA0: 815F0034  lwz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82D6AAA4: 2F1A0003  cmpwi cr6, r26, 3
	ctx.cr[6].compare_i32(ctx.r[26].s32, 3, &mut ctx.xer);
	// 82D6AAA8: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82D6AAAC: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82D6AAB0: 7D4AEA14  add r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 82D6AAB4: 813F0030  lwz r9, 0x30(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82D6AAB8: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82D6AABC: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82D6AAC0: 7D29EA14  add r9, r9, r29
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[29].u64;
	// 82D6AAC4: 91410074  stw r10, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 82D6AAC8: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82D6AACC: 895F0029  lbz r10, 0x29(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(41 as u32) ) } as u64;
	// 82D6AAD0: 897F0028  lbz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D6AAD4: 91210070  stw r9, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 82D6AAD8: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6AADC: 9941006C  stb r10, 0x6c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u8 ) };
	// 82D6AAE0: 9961006D  stb r11, 0x6d(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(109 as u32), ctx.r[11].u8 ) };
	// 82D6AAE4: 897C0020  lbz r11, 0x20(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D6AAE8: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D6AAEC: E8810068  ld r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 82D6AAF0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D6AAF4: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82D6AAF8: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82D6AAFC: E8C10078  ld r6, 0x78(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 82D6AB00: 409A011C  bne cr6, 0x82d6ac1c
	if !ctx.cr[6].eq {
	pc = 0x82D6AC1C; continue 'dispatch;
	}
	// 82D6AB04: E8610060  ld r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82D6AB08: E8A10070  ld r5, 0x70(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 82D6AB0C: 4BFFF50D  bl 0x82d6a018
	ctx.lr = 0x82D6AB10;
	sub_82D6A018(ctx, base);
	// 82D6AB10: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6AB14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82D6AB18: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 82D6AB1C: 7D785396  divwu r11, r24, r10
	ctx.r[11].u32 = ctx.r[24].u32 / ctx.r[10].u32;
	// 82D6AB20: 0CCA0000  twi 6, r10, 0
	// 82D6AB24: 7D6B51D6  mullw r11, r11, r10
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82D6AB28: 7CCBC050  subf r6, r11, r24
	ctx.r[6].s64 = ctx.r[24].s64 - ctx.r[11].s64;
	// 82D6AB2C: 419800A4  blt cr6, 0x82d6abd0
	if ctx.cr[6].lt {
	pc = 0x82D6ABD0; continue 'dispatch;
	}
	// 82D6AB30: 397EFFFC  addi r11, r30, -4
	ctx.r[11].s64 = ctx.r[30].s64 + -4;
	// 82D6AB34: 54C9103A  slwi r9, r6, 2
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D6AB38: 556BF0BE  srwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D6AB3C: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D6AB40: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 82D6AB44: 7D29DA14  add r9, r9, r27
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[27].u64;
	// 82D6AB48: 39790008  addi r11, r25, 8
	ctx.r[11].s64 = ctx.r[25].s64 + 8;
	// 82D6AB4C: 54E5103A  slwi r5, r7, 2
	ctx.r[5].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6AB50: C1A90000  lfs f13, 0(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D6AB54: 7CCA3214  add r6, r10, r6
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82D6AB58: EDBF0372  fmuls f13, f31, f13
	ctx.f[13].f64 = (((ctx.f[31].f64 * ctx.f[13].f64) as f32) as f64);
	// 82D6AB5C: C0090004  lfs f0, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6AB60: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82D6AB64: 7CCA3214  add r6, r10, r6
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82D6AB68: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 82D6AB6C: 7CCA3214  add r6, r10, r6
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82D6AB70: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82D6AB74: 7CCA3214  add r6, r10, r6
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82D6AB78: EC006FBA  fmadds f0, f0, f30, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[30].f64 + ctx.f[13].f64) as f32) as f64);
	// 82D6AB7C: C1A90000  lfs f13, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D6AB80: EDBF0372  fmuls f13, f31, f13
	ctx.f[13].f64 = (((ctx.f[31].f64 * ctx.f[13].f64) as f32) as f64);
	// 82D6AB84: D00BFFF8  stfs f0, -8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82D6AB88: C0090004  lfs f0, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6AB8C: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82D6AB90: EC006FBA  fmadds f0, f0, f30, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[30].f64 + ctx.f[13].f64) as f32) as f64);
	// 82D6AB94: C1A90000  lfs f13, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D6AB98: EDBF0372  fmuls f13, f31, f13
	ctx.f[13].f64 = (((ctx.f[31].f64 * ctx.f[13].f64) as f32) as f64);
	// 82D6AB9C: D00BFFFC  stfs f0, -4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82D6ABA0: C0090004  lfs f0, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6ABA4: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82D6ABA8: EC006FBA  fmadds f0, f0, f30, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[30].f64 + ctx.f[13].f64) as f32) as f64);
	// 82D6ABAC: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D6ABB0: C0090000  lfs f0, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6ABB4: EC1F0032  fmuls f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D6ABB8: C1A90004  lfs f13, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D6ABBC: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82D6ABC0: EC0D07BA  fmadds f0, f13, f30, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D6ABC4: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D6ABC8: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82D6ABCC: 409AFF84  bne cr6, 0x82d6ab50
	if !ctx.cr[6].eq {
	pc = 0x82D6AB50; continue 'dispatch;
	}
	// 82D6ABD0: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82D6ABD4: 4098027C  bge cr6, 0x82d6ae50
	if !ctx.cr[6].lt {
	pc = 0x82D6AE50; continue 'dispatch;
	}
	// 82D6ABD8: 54A9103A  slwi r9, r5, 2
	ctx.r[9].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D6ABDC: 54CB103A  slwi r11, r6, 2
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D6ABE0: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D6ABE4: 7D29CA14  add r9, r9, r25
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[25].u64;
	// 82D6ABE8: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82D6ABEC: 7D45F050  subf r10, r5, r30
	ctx.r[10].s64 = ctx.r[30].s64 - ctx.r[5].s64;
	// 82D6ABF0: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6ABF4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D6ABF8: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82D6ABFC: C1AB0004  lfs f13, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D6AC00: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82D6AC04: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D6AC08: EC0D07BA  fmadds f0, f13, f30, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D6AC0C: D0090000  stfs f0, 0(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D6AC10: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82D6AC14: 409AFFDC  bne cr6, 0x82d6abf0
	if !ctx.cr[6].eq {
	pc = 0x82D6ABF0; continue 'dispatch;
	}
	// 82D6AC18: 48000238  b 0x82d6ae50
	pc = 0x82D6AE50; continue 'dispatch;
	// 82D6AC1C: EBA10060  ld r29, 0x60(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82D6AC20: EB410070  ld r26, 0x70(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 82D6AC24: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D6AC28: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82D6AC2C: 4BFFF3ED  bl 0x82d6a018
	ctx.lr = 0x82D6AC30;
	sub_82D6A018(ctx, base);
	// 82D6AC30: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6AC34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82D6AC38: 7D585B96  divwu r10, r24, r11
	ctx.r[10].u32 = ctx.r[24].u32 / ctx.r[11].u32;
	// 82D6AC3C: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 82D6AC40: 7D4A59D6  mullw r10, r10, r11
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82D6AC44: 0CCB0000  twi 6, r11, 0
	// 82D6AC48: 7CCAC050  subf r6, r10, r24
	ctx.r[6].s64 = ctx.r[24].s64 - ctx.r[10].s64;
	// 82D6AC4C: 41980084  blt cr6, 0x82d6acd0
	if ctx.cr[6].lt {
	pc = 0x82D6ACD0; continue 'dispatch;
	}
	// 82D6AC50: 395EFFFC  addi r10, r30, -4
	ctx.r[10].s64 = ctx.r[30].s64 + -4;
	// 82D6AC54: 54C7103A  slwi r7, r6, 2
	ctx.r[7].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D6AC58: 554AF0BE  srwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D6AC5C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D6AC60: 390A0001  addi r8, r10, 1
	ctx.r[8].s64 = ctx.r[10].s64 + 1;
	// 82D6AC64: 7CE7DA14  add r7, r7, r27
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[27].u64;
	// 82D6AC68: 39590008  addi r10, r25, 8
	ctx.r[10].s64 = ctx.r[25].s64 + 8;
	// 82D6AC6C: 5505103A  slwi r5, r8, 2
	ctx.r[5].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6AC70: C0070000  lfs f0, 0(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6AC74: 7CE74A14  add r7, r7, r9
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 82D6AC78: EC1F0032  fmuls f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D6AC7C: D00AFFF8  stfs f0, -8(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82D6AC80: 7CCB3214  add r6, r11, r6
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 82D6AC84: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82D6AC88: 7CCB3214  add r6, r11, r6
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 82D6AC8C: C0070000  lfs f0, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6AC90: 7CE74A14  add r7, r7, r9
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 82D6AC94: EC1F0032  fmuls f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D6AC98: D00AFFFC  stfs f0, -4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82D6AC9C: 7CCB3214  add r6, r11, r6
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 82D6ACA0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82D6ACA4: 7CCB3214  add r6, r11, r6
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 82D6ACA8: C0070000  lfs f0, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6ACAC: 7CE74A14  add r7, r7, r9
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 82D6ACB0: EC1F0032  fmuls f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D6ACB4: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D6ACB8: C0070000  lfs f0, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6ACBC: 7CE74A14  add r7, r7, r9
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 82D6ACC0: EC1F0032  fmuls f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D6ACC4: D00A0004  stfs f0, 4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D6ACC8: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82D6ACCC: 409AFFA4  bne cr6, 0x82d6ac70
	if !ctx.cr[6].eq {
	pc = 0x82D6AC70; continue 'dispatch;
	}
	// 82D6ACD0: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82D6ACD4: 4098003C  bge cr6, 0x82d6ad10
	if !ctx.cr[6].lt {
	pc = 0x82D6AD10; continue 'dispatch;
	}
	// 82D6ACD8: 54A9103A  slwi r9, r5, 2
	ctx.r[9].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D6ACDC: 54CA103A  slwi r10, r6, 2
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D6ACE0: 7D09CA14  add r8, r9, r25
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[25].u64;
	// 82D6ACE4: 7D2ADA14  add r9, r10, r27
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[27].u64;
	// 82D6ACE8: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D6ACEC: 7D45F050  subf r10, r5, r30
	ctx.r[10].s64 = ctx.r[30].s64 - ctx.r[5].s64;
	// 82D6ACF0: C0090000  lfs f0, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6ACF4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D6ACF8: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82D6ACFC: D0080000  stfs f0, 0(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D6AD00: 7D293A14  add r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 82D6AD04: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82D6AD08: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D6AD0C: 409AFFE4  bne cr6, 0x82d6acf0
	if !ctx.cr[6].eq {
	pc = 0x82D6ACF0; continue 'dispatch;
	}
	// 82D6AD10: 891C002C  lbz r8, 0x2c(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D6AD14: 39380001  addi r9, r24, 1
	ctx.r[9].s64 = ctx.r[24].s64 + 1;
	// 82D6AD18: 80FC0024  lwz r7, 0x24(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6AD1C: 0CCB0000  twi 6, r11, 0
	// 82D6AD20: 7D295B96  divwu r9, r9, r11
	ctx.r[9].u32 = ctx.r[9].u32 / ctx.r[11].u32;
	// 82D6AD24: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6AD28: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82D6AD2C: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D6AD30: 9101007C  stw r8, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[8].u32 ) };
	// 82D6AD34: 39090001  addi r8, r9, 1
	ctx.r[8].s64 = ctx.r[9].s64 + 1;
	// 82D6AD38: 7D0831D6  mullw r8, r8, r6
	ctx.r[8].s64 = (ctx.r[8].s32 as i64) * (ctx.r[6].s32 as i64);
	// 82D6AD3C: 7F085000  cmpw cr6, r8, r10
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D6AD40: 40990010  ble cr6, 0x82d6ad50
	if !ctx.cr[6].gt {
	pc = 0x82D6AD50; continue 'dispatch;
	}
	// 82D6AD44: 7D6B49D6  mullw r11, r11, r9
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82D6AD48: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82D6AD4C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82D6AD50: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82D6AD54: E8810068  ld r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 82D6AD58: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D6AD5C: E8C10078  ld r6, 0x78(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 82D6AD60: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82D6AD64: 4BFFF2B5  bl 0x82d6a018
	ctx.lr = 0x82D6AD68;
	sub_82D6A018(ctx, base);
	// 82D6AD68: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82D6AD6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82D6AD70: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 82D6AD74: 41980094  blt cr6, 0x82d6ae08
	if ctx.cr[6].lt {
	pc = 0x82D6AE08; continue 'dispatch;
	}
	// 82D6AD78: 397EFFFC  addi r11, r30, -4
	ctx.r[11].s64 = ctx.r[30].s64 + -4;
	// 82D6AD7C: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6AD80: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82D6AD84: 5569F0BE  srwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D6AD88: 39790008  addi r11, r25, 8
	ctx.r[11].s64 = ctx.r[25].s64 + 8;
	// 82D6AD8C: 39090001  addi r8, r9, 1
	ctx.r[8].s64 = ctx.r[9].s64 + 1;
	// 82D6AD90: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D6AD94: 5505103A  slwi r5, r8, 2
	ctx.r[5].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6AD98: C0070000  lfs f0, 0(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6AD9C: 7CE74A14  add r7, r7, r9
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 82D6ADA0: C1ABFFF8  lfs f13, -8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D6ADA4: 7CCA3214  add r6, r10, r6
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82D6ADA8: EC006FBA  fmadds f0, f0, f30, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[30].f64 + ctx.f[13].f64) as f32) as f64);
	// 82D6ADAC: D00BFFF8  stfs f0, -8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82D6ADB0: C18BFFFC  lfs f12, -4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82D6ADB4: 7CCA3214  add r6, r10, r6
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82D6ADB8: C16B0000  lfs f11, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82D6ADBC: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82D6ADC0: C0070000  lfs f0, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6ADC4: 7CE74A14  add r7, r7, r9
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 82D6ADC8: EC0067BA  fmadds f0, f0, f30, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[30].f64 + ctx.f[12].f64) as f32) as f64);
	// 82D6ADCC: D00BFFFC  stfs f0, -4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82D6ADD0: C14B0004  lfs f10, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82D6ADD4: 7CCA3214  add r6, r10, r6
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82D6ADD8: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82D6ADDC: 7CCA3214  add r6, r10, r6
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82D6ADE0: C0070000  lfs f0, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6ADE4: 7CE74A14  add r7, r7, r9
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 82D6ADE8: EC005FBA  fmadds f0, f0, f30, f11
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[30].f64 + ctx.f[11].f64) as f32) as f64);
	// 82D6ADEC: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D6ADF0: C0070000  lfs f0, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6ADF4: 7CE74A14  add r7, r7, r9
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 82D6ADF8: EC0057BA  fmadds f0, f0, f30, f10
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[30].f64 + ctx.f[10].f64) as f32) as f64);
	// 82D6ADFC: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D6AE00: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82D6AE04: 409AFF94  bne cr6, 0x82d6ad98
	if !ctx.cr[6].eq {
	pc = 0x82D6AD98; continue 'dispatch;
	}
	// 82D6AE08: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82D6AE0C: 40980044  bge cr6, 0x82d6ae50
	if !ctx.cr[6].lt {
	pc = 0x82D6AE50; continue 'dispatch;
	}
	// 82D6AE10: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6AE14: 54CA103A  slwi r10, r6, 2
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D6AE18: 54AB103A  slwi r11, r5, 2
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D6AE1C: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D6AE20: 7D2ADA14  add r9, r10, r27
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[27].u64;
	// 82D6AE24: 7D6BCA14  add r11, r11, r25
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 82D6AE28: 7D45F050  subf r10, r5, r30
	ctx.r[10].s64 = ctx.r[30].s64 - ctx.r[5].s64;
	// 82D6AE2C: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6AE30: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D6AE34: C1A90000  lfs f13, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D6AE38: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82D6AE3C: EC0D07BA  fmadds f0, f13, f30, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D6AE40: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D6AE44: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D6AE48: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D6AE4C: 409AFFE0  bne cr6, 0x82d6ae2c
	if !ctx.cr[6].eq {
	pc = 0x82D6AE2C; continue 'dispatch;
	}
	// 82D6AE50: 7D57902E  lwzx r10, r23, r18
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 82D6AE54: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6AE58: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6AE5C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D6AE60: 40980020  bge cr6, 0x82d6ae80
	if !ctx.cr[6].lt {
	pc = 0x82D6AE80; continue 'dispatch;
	}
	// 82D6AE64: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D6AE68: 392966C8  addi r9, r9, 0x66c8
	ctx.r[9].s64 = ctx.r[9].s64 + 26312;
	// 82D6AE6C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D6AE70: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D6AE74: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82D6AE78: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D6AE7C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D6AE80: 7E258B78  mr r5, r17
	ctx.r[5].u64 = ctx.r[17].u64;
	// 82D6AE84: 92610050  stw r19, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[19].u32 ) };
	// 82D6AE88: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82D6AE8C: 92810058  stw r20, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[20].u32 ) };
	// 82D6AE90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D6AE94: 92A10054  stw r21, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[21].u32 ) };
	// 82D6AE98: 480017F9  bl 0x82d6c690
	ctx.lr = 0x82D6AE9C;
	sub_82D6C690(ctx, base);
	// 82D6AE9C: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82D6AEA0: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D6AEA4: 93230020  stw r25, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[25].u32 ) };
	// 82D6AEA8: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82D6AEAC: 409A0018  bne cr6, 0x82d6aec4
	if !ctx.cr[6].eq {
	pc = 0x82D6AEC4; continue 'dispatch;
	}
	// 82D6AEB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6AEB4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82D6AEB8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D6AEBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D6AEC0: 4E800421  bctrl
	ctx.lr = 0x82D6AEC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D6AEC4: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82D6AEC8: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D6AECC: 93630020  stw r27, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[27].u32 ) };
	// 82D6AED0: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82D6AED4: 409A0018  bne cr6, 0x82d6aeec
	if !ctx.cr[6].eq {
	pc = 0x82D6AEEC; continue 'dispatch;
	}
	// 82D6AED8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6AEDC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82D6AEE0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D6AEE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D6AEE8: 4E800421  bctrl
	ctx.lr = 0x82D6AEEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D6AEEC: 7D57902E  lwzx r10, r23, r18
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 82D6AEF0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6AEF4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6AEF8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D6AEFC: 40980020  bge cr6, 0x82d6af1c
	if !ctx.cr[6].lt {
	pc = 0x82D6AF1C; continue 'dispatch;
	}
	// 82D6AF00: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D6AF04: 39296468  addi r9, r9, 0x6468
	ctx.r[9].s64 = ctx.r[9].s64 + 25704;
	// 82D6AF08: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D6AF0C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D6AF10: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82D6AF14: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D6AF18: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D6AF1C: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 82D6AF20: CBC1FF70  lfd f30, -0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-144 as u32) ) };
	// 82D6AF24: CBE1FF78  lfd f31, -0x88(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-136 as u32) ) };
	// 82D6AF28: 4BF3E504  b 0x82ca942c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6AF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D6AF30 size=1940
    let mut pc: u32 = 0x82D6AF30;
    'dispatch: loop {
        match pc {
            0x82D6AF30 => {
    //   block [0x82D6AF30..0x82D6B6C4)
	// 82D6AF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6AF34: 4BF3E4AD  bl 0x82ca93e0
	ctx.lr = 0x82D6AF38;
	sub_82CA93D0(ctx, base);
	// 82D6AF38: DBC1FF78  stfd f30, -0x88(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[30].u64 ) };
	// 82D6AF3C: DBE1FF80  stfd f31, -0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), ctx.f[31].u64 ) };
	// 82D6AF40: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6AF44: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6AF48: 3AA00008  li r21, 8
	ctx.r[21].s64 = 8;
	// 82D6AF4C: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82D6AF50: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82D6AF54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D6AF58: 7CB42B78  mr r20, r5
	ctx.r[20].u64 = ctx.r[5].u64;
	// 82D6AF5C: 7CD23378  mr r18, r6
	ctx.r[18].u64 = ctx.r[6].u64;
	// 82D6AF60: 7D57A82E  lwzx r10, r23, r21
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82D6AF64: 93010184  stw r24, 0x184(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(388 as u32), ctx.r[24].u32 ) };
	// 82D6AF68: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6AF6C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6AF70: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D6AF74: 4098002C  bge cr6, 0x82d6afa0
	if !ctx.cr[6].lt {
	pc = 0x82D6AFA0; continue 'dispatch;
	}
	// 82D6AF78: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D6AF7C: 3D008202  lis r8, -0x7dfe
	ctx.r[8].s64 = -2113798144;
	// 82D6AF80: 3929673C  addi r9, r9, 0x673c
	ctx.r[9].s64 = ctx.r[9].s64 + 26428;
	// 82D6AF84: 390866F0  addi r8, r8, 0x66f0
	ctx.r[8].s64 = ctx.r[8].s64 + 26352;
	// 82D6AF88: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D6AF8C: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82D6AF90: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D6AF94: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82D6AF98: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D6AF9C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D6AFA0: 833F002C  lwz r25, 0x2c(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D6AFA4: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82D6AFA8: 815F005C  lwz r10, 0x5c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82D6AFAC: 7F3BCB78  mr r27, r25
	ctx.r[27].u64 = ctx.r[25].u64;
	// 82D6AFB0: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6AFB4: 7E6B5214  add r19, r11, r10
	ctx.r[19].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D6AFB8: 7F144840  cmplw cr6, r20, r9
	ctx.cr[6].compare_u32(ctx.r[20].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D6AFBC: 419A0028  beq cr6, 0x82d6afe4
	if ctx.cr[6].eq {
	pc = 0x82D6AFE4; continue 'dispatch;
	}
	// 82D6AFC0: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82D6AFC4: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 82D6AFC8: 7E639B78  mr r3, r19
	ctx.r[3].u64 = ctx.r[19].u64;
	// 82D6AFCC: 48000CB5  bl 0x82d6bc80
	ctx.lr = 0x82D6AFD0;
	sub_82D6BC80(ctx, base);
	// 82D6AFD0: 814100AC  lwz r10, 0xac(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82D6AFD4: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82D6AFD8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D6AFDC: 814100A8  lwz r10, 0xa8(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(168 as u32) ) } as u64;
	// 82D6AFE0: 7F6B5214  add r27, r11, r10
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D6AFE4: 7D77A82E  lwzx r11, r23, r21
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82D6AFE8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6AFEC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6AFF0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D6AFF4: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D6AFF8: 3B49672C  addi r26, r9, 0x672c
	ctx.r[26].s64 = ctx.r[9].s64 + 26412;
	// 82D6AFFC: 40980018  bge cr6, 0x82d6b014
	if !ctx.cr[6].lt {
	pc = 0x82D6B014; continue 'dispatch;
	}
	// 82D6B000: 934A0000  stw r26, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82D6B004: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D6B008: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82D6B00C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D6B010: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D6B014: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82D6B018: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82D6B01C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82D6B020: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D6B024: 4BFFC5DD  bl 0x82d67600
	ctx.lr = 0x82D6B028;
	sub_82D67600(ctx, base);
	// 82D6B028: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D6B02C: 3AC00004  li r22, 4
	ctx.r[22].s64 = 4;
	// 82D6B030: C3C10050  lfs f30, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82D6B034: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82D6B038: 3F808000  lis r28, -0x8000
	ctx.r[28].s64 = -2147483648;
	// 82D6B03C: 395B0004  addi r10, r27, 4
	ctx.r[10].s64 = ctx.r[27].s64 + 4;
	// 82D6B040: C00B0C14  lfs f0, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6B044: 7D77B02E  lwzx r11, r23, r22
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82D6B048: 55441036  rlwinm r4, r10, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82D6B04C: 93C10070  stw r30, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[30].u32 ) };
	// 82D6B050: EFE0F028  fsubs f31, f0, f30
	ctx.f[31].f64 = (((ctx.f[0].f64 - ctx.f[30].f64) as f32) as f64);
	// 82D6B054: 93C10074  stw r30, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 82D6B058: 93810078  stw r28, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[28].u32 ) };
	// 82D6B05C: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D6B060: 810B002C  lwz r8, 0x2c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D6B064: 7D2A2214  add r9, r10, r4
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82D6B068: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82D6B06C: 4199000C  bgt cr6, 0x82d6b078
	if ctx.cr[6].gt {
	pc = 0x82D6B078; continue 'dispatch;
	}
	// 82D6B070: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82D6B074: 4800001C  b 0x82d6b090
	pc = 0x82D6B090; continue 'dispatch;
	// 82D6B078: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6B07C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82D6B080: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D6B084: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D6B088: 4E800421  bctrl
	ctx.lr = 0x82D6B08C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D6B08C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82D6B090: 7F6BE378  or r11, r27, r28
	ctx.r[11].u64 = ctx.r[27].u64 | ctx.r[28].u64;
	// 82D6B094: 91410070  stw r10, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 82D6B098: 9141007C  stw r10, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 82D6B09C: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82D6B0A0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D6B0A4: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82D6B0A8: 40980024  bge cr6, 0x82d6b0cc
	if !ctx.cr[6].lt {
	pc = 0x82D6B0CC; continue 'dispatch;
	}
	// 82D6B0AC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D6B0B0: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D6B0B4: 41980008  blt cr6, 0x82d6b0bc
	if ctx.cr[6].lt {
	pc = 0x82D6B0BC; continue 'dispatch;
	}
	// 82D6B0B8: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82D6B0BC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82D6B0C0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D6B0C4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82D6B0C8: 4BFEBE49  bl 0x82d56f10
	ctx.lr = 0x82D6B0CC;
	sub_82D56F10(ctx, base);
	// 82D6B0CC: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6B0D0: 7D77B02E  lwzx r11, r23, r22
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82D6B0D4: 7FAAD9D6  mullw r29, r10, r27
	ctx.r[29].s64 = (ctx.r[10].s32 as i64) * (ctx.r[27].s32 as i64);
	// 82D6B0D8: 93610074  stw r27, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[27].u32 ) };
	// 82D6B0DC: 393D0004  addi r9, r29, 4
	ctx.r[9].s64 = ctx.r[29].s64 + 4;
	// 82D6B0E0: 93C10080  stw r30, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[30].u32 ) };
	// 82D6B0E4: 93C10084  stw r30, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82D6B0E8: 93810088  stw r28, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[28].u32 ) };
	// 82D6B0EC: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D6B0F0: 810B002C  lwz r8, 0x2c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D6B0F4: 55241036  rlwinm r4, r9, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[9].u32 as u64 & 0x3FFFFFFFu64;
	// 82D6B0F8: 7D2A2214  add r9, r10, r4
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82D6B0FC: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82D6B100: 4199000C  bgt cr6, 0x82d6b10c
	if ctx.cr[6].gt {
	pc = 0x82D6B10C; continue 'dispatch;
	}
	// 82D6B104: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82D6B108: 4800001C  b 0x82d6b124
	pc = 0x82D6B124; continue 'dispatch;
	// 82D6B10C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6B110: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82D6B114: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D6B118: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D6B11C: 4E800421  bctrl
	ctx.lr = 0x82D6B120;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D6B120: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82D6B124: 7FABE378  or r11, r29, r28
	ctx.r[11].u64 = ctx.r[29].u64 | ctx.r[28].u64;
	// 82D6B128: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6B12C: 91410080  stw r10, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 82D6B130: 7FA9D9D6  mullw r29, r9, r27
	ctx.r[29].s64 = (ctx.r[9].s32 as i64) * (ctx.r[27].s32 as i64);
	// 82D6B134: 9141008C  stw r10, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[10].u32 ) };
	// 82D6B138: 91610088  stw r11, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 82D6B13C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D6B140: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82D6B144: 40980024  bge cr6, 0x82d6b168
	if !ctx.cr[6].lt {
	pc = 0x82D6B168; continue 'dispatch;
	}
	// 82D6B148: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D6B14C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D6B150: 41980008  blt cr6, 0x82d6b158
	if ctx.cr[6].lt {
	pc = 0x82D6B158; continue 'dispatch;
	}
	// 82D6B154: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82D6B158: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82D6B15C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D6B160: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82D6B164: 4BFEBDAD  bl 0x82d56f10
	ctx.lr = 0x82D6B168;
	sub_82D56F10(ctx, base);
	// 82D6B168: 93A10084  stw r29, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[29].u32 ) };
	// 82D6B16C: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82D6B170: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D6B174: 419A0020  beq cr6, 0x82d6b194
	if ctx.cr[6].eq {
	pc = 0x82D6B194; continue 'dispatch;
	}
	// 82D6B178: 38C10184  addi r6, r1, 0x184
	ctx.r[6].s64 = ctx.r[1].s64 + 388;
	// 82D6B17C: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82D6B180: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D6B184: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D6B188: 4BFFF5F1  bl 0x82d6a778
	ctx.lr = 0x82D6B18C;
	sub_82D6A778(ctx, base);
	// 82D6B18C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D6B190: 409A001C  bne cr6, 0x82d6b1ac
	if !ctx.cr[6].eq {
	pc = 0x82D6B1AC; continue 'dispatch;
	}
	// 82D6B194: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82D6B198: 80A10080  lwz r5, 0x80(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82D6B19C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D6B1A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D6B1A4: 4BFFF495  bl 0x82d6a638
	ctx.lr = 0x82D6B1A8;
	sub_82D6A638(ctx, base);
	// 82D6B1A8: 80610080  lwz r3, 0x80(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82D6B1AC: 7D57A82E  lwzx r10, r23, r21
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82D6B1B0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6B1B4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6B1B8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D6B1BC: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D6B1C0: 3B896720  addi r28, r9, 0x6720
	ctx.r[28].s64 = ctx.r[9].s64 + 26400;
	// 82D6B1C4: 40980018  bge cr6, 0x82d6b1dc
	if !ctx.cr[6].lt {
	pc = 0x82D6B1DC; continue 'dispatch;
	}
	// 82D6B1C8: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82D6B1CC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D6B1D0: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82D6B1D4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D6B1D8: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D6B1DC: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6B1E0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D6B1E4: 7D7D5396  divwu r11, r29, r10
	ctx.r[11].u32 = ctx.r[29].u32 / ctx.r[10].u32;
	// 82D6B1E8: 0CCA0000  twi 6, r10, 0
	// 82D6B1EC: 7D6B51D6  mullw r11, r11, r10
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82D6B1F0: 7D6BE850  subf r11, r11, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 82D6B1F4: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82D6B1F8: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D6B1FC: 419A0128  beq cr6, 0x82d6b324
	if ctx.cr[6].eq {
	pc = 0x82D6B324; continue 'dispatch;
	}
	// 82D6B200: 2F1B0004  cmpwi cr6, r27, 4
	ctx.cr[6].compare_i32(ctx.r[27].s32, 4, &mut ctx.xer);
	// 82D6B204: 419800D4  blt cr6, 0x82d6b2d8
	if ctx.cr[6].lt {
	pc = 0x82D6B2D8; continue 'dispatch;
	}
	// 82D6B208: 393BFFFC  addi r9, r27, -4
	ctx.r[9].s64 = ctx.r[27].s64 + -4;
	// 82D6B20C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82D6B210: 5529F0BE  srwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D6B214: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82D6B218: 5525103A  slwi r5, r9, 2
	ctx.r[5].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6B21C: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D6B220: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82D6B224: 7CE71A14  add r7, r7, r3
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[3].u64;
	// 82D6B228: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82D6B22C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D6B230: C0070000  lfs f0, 0(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6B234: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82D6B238: C1A70004  lfs f13, 4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D6B23C: 80E10070  lwz r7, 0x70(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D6B240: EC0D07BA  fmadds f0, f13, f30, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D6B244: 7C0A3D2E  stfsx f0, r10, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 82D6B248: 80FF0024  lwz r7, 0x24(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6B24C: 80C10070  lwz r6, 0x70(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D6B250: 7D675A14  add r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82D6B254: 7CCA3214  add r6, r10, r6
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82D6B258: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D6B25C: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82D6B260: 7CE71A14  add r7, r7, r3
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[3].u64;
	// 82D6B264: C0070000  lfs f0, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6B268: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82D6B26C: C1A70004  lfs f13, 4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D6B270: EC0D07BA  fmadds f0, f13, f30, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D6B274: D0060004  stfs f0, 4(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D6B278: 80FF0024  lwz r7, 0x24(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6B27C: 80C10070  lwz r6, 0x70(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D6B280: 7D675A14  add r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82D6B284: 7CC83214  add r6, r8, r6
	ctx.r[6].u64 = ctx.r[8].u64 + ctx.r[6].u64;
	// 82D6B288: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D6B28C: 7CE71A14  add r7, r7, r3
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[3].u64;
	// 82D6B290: C0070000  lfs f0, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6B294: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82D6B298: C1A70004  lfs f13, 4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D6B29C: EC0D07BA  fmadds f0, f13, f30, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D6B2A0: D006FFFC  stfs f0, -4(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82D6B2A4: 80FF0024  lwz r7, 0x24(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6B2A8: 7D675A14  add r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82D6B2AC: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D6B2B0: 7CE71A14  add r7, r7, r3
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[3].u64;
	// 82D6B2B4: C0070000  lfs f0, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6B2B8: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82D6B2BC: C1A70004  lfs f13, 4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D6B2C0: 80E10070  lwz r7, 0x70(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D6B2C4: EC0D07BA  fmadds f0, f13, f30, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D6B2C8: 7C083D2E  stfsx f0, r8, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 82D6B2CC: 811F0024  lwz r8, 0x24(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6B2D0: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82D6B2D4: 409AFF48  bne cr6, 0x82d6b21c
	if !ctx.cr[6].eq {
	pc = 0x82D6B21C; continue 'dispatch;
	}
	// 82D6B2D8: 7F05D840  cmplw cr6, r5, r27
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82D6B2DC: 409802B8  bge cr6, 0x82d6b594
	if !ctx.cr[6].lt {
	pc = 0x82D6B594; continue 'dispatch;
	}
	// 82D6B2E0: 54A9103A  slwi r9, r5, 2
	ctx.r[9].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D6B2E4: 7D45D850  subf r10, r5, r27
	ctx.r[10].s64 = ctx.r[27].s64 - ctx.r[5].s64;
	// 82D6B2E8: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D6B2EC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D6B2F0: 7D081A14  add r8, r8, r3
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[3].u64;
	// 82D6B2F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D6B2F8: C0080000  lfs f0, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6B2FC: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82D6B300: C1A80004  lfs f13, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D6B304: 81010070  lwz r8, 0x70(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D6B308: EC0D07BA  fmadds f0, f13, f30, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D6B30C: 7C09452E  stfsx f0, r9, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 82D6B310: 811F0024  lwz r8, 0x24(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6B314: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82D6B318: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82D6B31C: 409AFFCC  bne cr6, 0x82d6b2e8
	if !ctx.cr[6].eq {
	pc = 0x82D6B2E8; continue 'dispatch;
	}
	// 82D6B320: 48000274  b 0x82d6b594
	pc = 0x82D6B594; continue 'dispatch;
	// 82D6B324: 2F1B0004  cmpwi cr6, r27, 4
	ctx.cr[6].compare_i32(ctx.r[27].s32, 4, &mut ctx.xer);
	// 82D6B328: 419800A4  blt cr6, 0x82d6b3cc
	if ctx.cr[6].lt {
	pc = 0x82D6B3CC; continue 'dispatch;
	}
	// 82D6B32C: 393BFFFC  addi r9, r27, -4
	ctx.r[9].s64 = ctx.r[27].s64 + -4;
	// 82D6B330: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82D6B334: 5529F0BE  srwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D6B338: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82D6B33C: 5525103A  slwi r5, r9, 2
	ctx.r[5].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6B340: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D6B344: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82D6B348: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82D6B34C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D6B350: 7C071C2E  lfsx f0, r7, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6B354: 80E10070  lwz r7, 0x70(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D6B358: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82D6B35C: 7C0A3D2E  stfsx f0, r10, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 82D6B360: 80FF0024  lwz r7, 0x24(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6B364: 80C10070  lwz r6, 0x70(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D6B368: 7D675A14  add r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82D6B36C: 7CCA3214  add r6, r10, r6
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82D6B370: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D6B374: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82D6B378: 7C071C2E  lfsx f0, r7, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6B37C: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82D6B380: D0060004  stfs f0, 4(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D6B384: 80FF0024  lwz r7, 0x24(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6B388: 80C10070  lwz r6, 0x70(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D6B38C: 7D675A14  add r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82D6B390: 7CC83214  add r6, r8, r6
	ctx.r[6].u64 = ctx.r[8].u64 + ctx.r[6].u64;
	// 82D6B394: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D6B398: 7C071C2E  lfsx f0, r7, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6B39C: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82D6B3A0: D006FFFC  stfs f0, -4(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82D6B3A4: 80FF0024  lwz r7, 0x24(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6B3A8: 7D675A14  add r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82D6B3AC: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D6B3B0: 7C071C2E  lfsx f0, r7, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6B3B4: 80E10070  lwz r7, 0x70(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D6B3B8: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82D6B3BC: 7C083D2E  stfsx f0, r8, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 82D6B3C0: 811F0024  lwz r8, 0x24(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6B3C4: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82D6B3C8: 409AFF78  bne cr6, 0x82d6b340
	if !ctx.cr[6].eq {
	pc = 0x82D6B340; continue 'dispatch;
	}
	// 82D6B3CC: 7F05D840  cmplw cr6, r5, r27
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82D6B3D0: 40980038  bge cr6, 0x82d6b408
	if !ctx.cr[6].lt {
	pc = 0x82D6B408; continue 'dispatch;
	}
	// 82D6B3D4: 54A9103A  slwi r9, r5, 2
	ctx.r[9].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D6B3D8: 7D45D850  subf r10, r5, r27
	ctx.r[10].s64 = ctx.r[27].s64 - ctx.r[5].s64;
	// 82D6B3DC: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D6B3E0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D6B3E4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D6B3E8: 7C081C2E  lfsx f0, r8, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6B3EC: 81010070  lwz r8, 0x70(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D6B3F0: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82D6B3F4: 7C09452E  stfsx f0, r9, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 82D6B3F8: 811F0024  lwz r8, 0x24(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6B3FC: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82D6B400: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82D6B404: 409AFFD8  bne cr6, 0x82d6b3dc
	if !ctx.cr[6].eq {
	pc = 0x82D6B3DC; continue 'dispatch;
	}
	// 82D6B408: 7D57A82E  lwzx r10, r23, r21
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82D6B40C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6B410: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6B414: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D6B418: 40980018  bge cr6, 0x82d6b430
	if !ctx.cr[6].lt {
	pc = 0x82D6B430; continue 'dispatch;
	}
	// 82D6B41C: 934B0000  stw r26, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82D6B420: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D6B424: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82D6B428: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D6B42C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D6B430: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82D6B434: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82D6B438: 419A0020  beq cr6, 0x82d6b458
	if ctx.cr[6].eq {
	pc = 0x82D6B458; continue 'dispatch;
	}
	// 82D6B43C: 38C10184  addi r6, r1, 0x184
	ctx.r[6].s64 = ctx.r[1].s64 + 388;
	// 82D6B440: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82D6B444: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D6B448: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D6B44C: 4BFFF32D  bl 0x82d6a778
	ctx.lr = 0x82D6B450;
	sub_82D6A778(ctx, base);
	// 82D6B450: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D6B454: 409A001C  bne cr6, 0x82d6b470
	if !ctx.cr[6].eq {
	pc = 0x82D6B470; continue 'dispatch;
	}
	// 82D6B458: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82D6B45C: 80A10080  lwz r5, 0x80(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82D6B460: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D6B464: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D6B468: 4BFFF1D1  bl 0x82d6a638
	ctx.lr = 0x82D6B46C;
	sub_82D6A638(ctx, base);
	// 82D6B46C: 80610080  lwz r3, 0x80(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82D6B470: 7D57A82E  lwzx r10, r23, r21
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82D6B474: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6B478: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6B47C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D6B480: 40980018  bge cr6, 0x82d6b498
	if !ctx.cr[6].lt {
	pc = 0x82D6B498; continue 'dispatch;
	}
	// 82D6B484: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82D6B488: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D6B48C: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82D6B490: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D6B494: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D6B498: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D6B49C: 2F1B0004  cmpwi cr6, r27, 4
	ctx.cr[6].compare_i32(ctx.r[27].s32, 4, &mut ctx.xer);
	// 82D6B4A0: 419800B4  blt cr6, 0x82d6b554
	if ctx.cr[6].lt {
	pc = 0x82D6B554; continue 'dispatch;
	}
	// 82D6B4A4: 395BFFFC  addi r10, r27, -4
	ctx.r[10].s64 = ctx.r[27].s64 + -4;
	// 82D6B4A8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82D6B4AC: 554AF0BE  srwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D6B4B0: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 82D6B4B4: 5525103A  slwi r5, r9, 2
	ctx.r[5].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6B4B8: 81010070  lwz r8, 0x70(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D6B4BC: 57C7103A  slwi r7, r30, 2
	ctx.r[7].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D6B4C0: 394B000C  addi r10, r11, 0xc
	ctx.r[10].s64 = ctx.r[11].s64 + 12;
	// 82D6B4C4: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82D6B4C8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D6B4CC: 7C0B442E  lfsx f0, r11, r8
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6B4D0: 7DA71C2E  lfsx f13, r7, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D6B4D4: EC0D07BA  fmadds f0, f13, f30, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D6B4D8: 7C0B452E  stfsx f0, r11, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 82D6B4DC: 80FF0024  lwz r7, 0x24(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6B4E0: 81010070  lwz r8, 0x70(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D6B4E4: 7CE7F214  add r7, r7, r30
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[30].u64;
	// 82D6B4E8: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82D6B4EC: 54E6103A  slwi r6, r7, 2
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D6B4F0: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82D6B4F4: C0080004  lfs f0, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6B4F8: 7DA61C2E  lfsx f13, r6, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D6B4FC: EC0D07BA  fmadds f0, f13, f30, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D6B500: D0080004  stfs f0, 4(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D6B504: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6B508: 81010070  lwz r8, 0x70(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D6B50C: 7CE63A14  add r7, r6, r7
	ctx.r[7].u64 = ctx.r[6].u64 + ctx.r[7].u64;
	// 82D6B510: 7D0A4214  add r8, r10, r8
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82D6B514: 54E6103A  slwi r6, r7, 2
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D6B518: C008FFFC  lfs f0, -4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6B51C: 7DA61C2E  lfsx f13, r6, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D6B520: EC0D07BA  fmadds f0, f13, f30, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D6B524: D008FFFC  stfs f0, -4(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82D6B528: 811F0024  lwz r8, 0x24(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6B52C: 80C10070  lwz r6, 0x70(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D6B530: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82D6B534: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D6B538: 7C0A342E  lfsx f0, r10, r6
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[6].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6B53C: 7DA71C2E  lfsx f13, r7, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D6B540: EC0D07BA  fmadds f0, f13, f30, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D6B544: 7C0A352E  stfsx f0, r10, r6
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 82D6B548: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6B54C: 7FCA4214  add r30, r10, r8
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82D6B550: 409AFF68  bne cr6, 0x82d6b4b8
	if !ctx.cr[6].eq {
	pc = 0x82D6B4B8; continue 'dispatch;
	}
	// 82D6B554: 7F05D840  cmplw cr6, r5, r27
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82D6B558: 4098003C  bge cr6, 0x82d6b594
	if !ctx.cr[6].lt {
	pc = 0x82D6B594; continue 'dispatch;
	}
	// 82D6B55C: 54AB103A  slwi r11, r5, 2
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D6B560: 7D45D850  subf r10, r5, r27
	ctx.r[10].s64 = ctx.r[27].s64 - ctx.r[5].s64;
	// 82D6B564: 81210070  lwz r9, 0x70(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D6B568: 57C8103A  slwi r8, r30, 2
	ctx.r[8].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D6B56C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D6B570: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D6B574: 7C0B4C2E  lfsx f0, r11, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6B578: 7DA81C2E  lfsx f13, r8, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D6B57C: EC0D07BA  fmadds f0, f13, f30, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 82D6B580: 7C0B4D2E  stfsx f0, r11, r9
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), tmp.u32) };
	// 82D6B584: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6B588: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D6B58C: 7FC9F214  add r30, r9, r30
	ctx.r[30].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 82D6B590: 409AFFD4  bne cr6, 0x82d6b564
	if !ctx.cr[6].eq {
	pc = 0x82D6B564; continue 'dispatch;
	}
	// 82D6B594: 7D57A82E  lwzx r10, r23, r21
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82D6B598: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6B59C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6B5A0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D6B5A4: 40980020  bge cr6, 0x82d6b5c4
	if !ctx.cr[6].lt {
	pc = 0x82D6B5C4; continue 'dispatch;
	}
	// 82D6B5A8: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D6B5AC: 39296710  addi r9, r9, 0x6710
	ctx.r[9].s64 = ctx.r[9].s64 + 26384;
	// 82D6B5B0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D6B5B4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D6B5B8: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82D6B5BC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D6B5C0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D6B5C4: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82D6B5C8: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 82D6B5CC: 815F005C  lwz r10, 0x5c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82D6B5D0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82D6B5D4: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D6B5D8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D6B5DC: 92810058  stw r20, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[20].u32 ) };
	// 82D6B5E0: 9261005C  stw r19, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[19].u32 ) };
	// 82D6B5E4: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82D6B5E8: 480010A9  bl 0x82d6c690
	ctx.lr = 0x82D6B5EC;
	sub_82D6C690(ctx, base);
	// 82D6B5EC: 7D77A82E  lwzx r11, r23, r21
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82D6B5F0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6B5F4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6B5F8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D6B5FC: 40980020  bge cr6, 0x82d6b61c
	if !ctx.cr[6].lt {
	pc = 0x82D6B61C; continue 'dispatch;
	}
	// 82D6B600: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D6B604: 39296468  addi r9, r9, 0x6468
	ctx.r[9].s64 = ctx.r[9].s64 + 25704;
	// 82D6B608: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D6B60C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D6B610: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82D6B614: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D6B618: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D6B61C: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82D6B620: 8081008C  lwz r4, 0x8c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82D6B624: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D6B628: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82D6B62C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82D6B630: 409A0014  bne cr6, 0x82d6b644
	if !ctx.cr[6].eq {
	pc = 0x82D6B644; continue 'dispatch;
	}
	// 82D6B634: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6B638: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D6B63C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D6B640: 4E800421  bctrl
	ctx.lr = 0x82D6B644;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D6B644: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82D6B648: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D6B64C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D6B650: 409A0018  bne cr6, 0x82d6b668
	if !ctx.cr[6].eq {
	pc = 0x82D6B668; continue 'dispatch;
	}
	// 82D6B654: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D6B658: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82D6B65C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6B660: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82D6B664: 4BFE9C65  bl 0x82d552c8
	ctx.lr = 0x82D6B668;
	sub_82D552C8(ctx, base);
	// 82D6B668: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82D6B66C: 8081007C  lwz r4, 0x7c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82D6B670: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D6B674: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82D6B678: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82D6B67C: 409A0014  bne cr6, 0x82d6b690
	if !ctx.cr[6].eq {
	pc = 0x82D6B690; continue 'dispatch;
	}
	// 82D6B680: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6B684: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D6B688: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D6B68C: 4E800421  bctrl
	ctx.lr = 0x82D6B690;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D6B690: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82D6B694: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D6B698: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D6B69C: 409A0018  bne cr6, 0x82d6b6b4
	if !ctx.cr[6].eq {
	pc = 0x82D6B6B4; continue 'dispatch;
	}
	// 82D6B6A0: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D6B6A4: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82D6B6A8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6B6AC: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D6B6B0: 4BFE9C19  bl 0x82d552c8
	ctx.lr = 0x82D6B6B4;
	sub_82D552C8(ctx, base);
	// 82D6B6B4: 38210150  addi r1, r1, 0x150
	ctx.r[1].s64 = ctx.r[1].s64 + 336;
	// 82D6B6B8: CBC1FF78  lfd f30, -0x88(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-136 as u32) ) };
	// 82D6B6BC: CBE1FF80  lfd f31, -0x80(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-128 as u32) ) };
	// 82D6B6C0: 4BF3DD70  b 0x82ca9430
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6B6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D6B6C8 size=36
    let mut pc: u32 = 0x82D6B6C8;
    'dispatch: loop {
        match pc {
            0x82D6B6C8 => {
    //   block [0x82D6B6C8..0x82D6B6EC)
	// 82D6B6C8: 81430024  lwz r10, 0x24(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6B6CC: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6B6D0: 554B083E  rotlwi r11, r10, 1
	ctx.r[11].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 82D6B6D4: 7C6A4BD6  divw r3, r10, r9
	ctx.r[3].s32 = ctx.r[10].s32 / ctx.r[9].s32;
	// 82D6B6D8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D6B6DC: 0CC90000  twi 6, r9, 0
	// 82D6B6E0: 7D2B5878  andc r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 & !ctx.r[11].u64;
	// 82D6B6E4: 0CABFFFF  twi 5, r11, -1
	// 82D6B6E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6B6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D6B6F0 size=32
    let mut pc: u32 = 0x82D6B6F0;
    'dispatch: loop {
        match pc {
            0x82D6B6F0 => {
    //   block [0x82D6B6F0..0x82D6B710)
	// 82D6B6F0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6B6F4: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82D6B6F8: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82D6B6FC: 80A30010  lwz r5, 0x10(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6B700: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 82D6B704: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6B708: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D6B70C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6B710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D6B710 size=8
    let mut pc: u32 = 0x82D6B710;
    'dispatch: loop {
        match pc {
            0x82D6B710 => {
    //   block [0x82D6B710..0x82D6B718)
	// 82D6B710: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82D6B714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6B718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D6B718 size=132
    let mut pc: u32 = 0x82D6B718;
    'dispatch: loop {
        match pc {
            0x82D6B718 => {
    //   block [0x82D6B718..0x82D6B79C)
	// 82D6B718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6B71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D6B720: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D6B724: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D6B728: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6B72C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D6B730: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D6B734: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 82D6B738: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82D6B73C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82D6B740: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82D6B744: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D6B748: 4BFFBEB9  bl 0x82d67600
	ctx.lr = 0x82D6B74C;
	sub_82D67600(ctx, base);
	// 82D6B74C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6B750: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D6B754: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D6B758: 7D6B49D6  mullw r11, r11, r9
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82D6B75C: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D6B760: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82D6B764: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D6B768: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D6B76C: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82D6B770: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6B774: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D6B778: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D6B77C: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D6B780: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82D6B784: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D6B788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D6B78C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D6B790: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D6B794: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D6B798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6B7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D6B7A0 size=24
    let mut pc: u32 = 0x82D6B7A0;
    'dispatch: loop {
        match pc {
            0x82D6B7A0 => {
    //   block [0x82D6B7A0..0x82D6B7B8)
	// 82D6B7A0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6B7A4: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D6B7A8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D6B7AC: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D6B7B0: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 82D6B7B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6B7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D6B7B8 size=424
    let mut pc: u32 = 0x82D6B7B8;
    'dispatch: loop {
        match pc {
            0x82D6B7B8 => {
    //   block [0x82D6B7B8..0x82D6B960)
	// 82D6B7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6B7BC: 4BF3DC51  bl 0x82ca940c
	ctx.lr = 0x82D6B7C0;
	sub_82CA93D0(ctx, base);
	// 82D6B7C0: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6B7C4: D021001C  stfs f1, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82D6B7C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D6B7CC: 419A001C  beq cr6, 0x82d6b7e8
	if ctx.cr[6].eq {
	pc = 0x82D6B7E8; continue 'dispatch;
	}
	// 82D6B7D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82D6B7D4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82D6B7D8: 394A5E14  addi r10, r10, 0x5e14
	ctx.r[10].s64 = ctx.r[10].s64 + 24084;
	// 82D6B7DC: B12B0006  sth r9, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82D6B7E0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D6B7E4: 48000008  b 0x82d6b7ec
	pc = 0x82D6B7EC; continue 'dispatch;
	// 82D6B7E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D6B7EC: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6B7F0: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82D6B7F4: 8167000C  lwz r11, 0xc(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6B7F8: 5549083C  slwi r9, r10, 1
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D6B7FC: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82D6B800: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D6B804: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82D6B808: 419A0154  beq cr6, 0x82d6b95c
	if ctx.cr[6].eq {
	pc = 0x82D6B95C; continue 'dispatch;
	}
	// 82D6B80C: 3D008202  lis r8, -0x7dfe
	ctx.r[8].s64 = -2113798144;
	// 82D6B810: 39260010  addi r9, r6, 0x10
	ctx.r[9].s64 = ctx.r[6].s64 + 16;
	// 82D6B814: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82D6B818: 3CA08200  lis r5, -0x7e00
	ctx.r[5].s64 = -2113929216;
	// 82D6B81C: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 82D6B820: 3BC84C30  addi r30, r8, 0x4c30
	ctx.r[30].s64 = ctx.r[8].s64 + 19504;
	// 82D6B824: 3D008202  lis r8, -0x7dfe
	ctx.r[8].s64 = -2113798144;
	// 82D6B828: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82D6B82C: 38E84C40  addi r7, r8, 0x4c40
	ctx.r[7].s64 = ctx.r[8].s64 + 19520;
	// 82D6B830: C1A50C18  lfs f13, 0xc18(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(3096 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D6B834: C0060C14  lfs f0, 0xc14(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6B838: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82D6B83C: 3BE0FFF0  li r31, -0x10
	ctx.r[31].s64 = -16;
	// 82D6B840: 38C1001C  addi r6, r1, 0x1c
	ctx.r[6].s64 = ctx.r[1].s64 + 28;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6B960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D6B960 size=420
    let mut pc: u32 = 0x82D6B960;
    'dispatch: loop {
        match pc {
            0x82D6B960 => {
    //   block [0x82D6B960..0x82D6BB04)
	// 82D6B960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6B964: 4BF3DAA1  bl 0x82ca9404
	ctx.lr = 0x82D6B968;
	sub_82CA93D0(ctx, base);
	// 82D6B968: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82D6B96C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6B970: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6B974: 3B800008  li r28, 8
	ctx.r[28].s64 = 8;
	// 82D6B978: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D6B97C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82D6B980: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D6B984: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82D6B988: 7D5CD82E  lwzx r10, r28, r27
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82D6B98C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6B990: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6B994: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D6B998: 40980020  bge cr6, 0x82d6b9b8
	if !ctx.cr[6].lt {
	pc = 0x82D6B9B8; continue 'dispatch;
	}
	// 82D6B99C: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82D6B9A0: 3929674C  addi r9, r9, 0x674c
	ctx.r[9].s64 = ctx.r[9].s64 + 26444;
	// 82D6B9A4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D6B9A8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82D6B9AC: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82D6B9B0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82D6B9B4: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D6B9B8: C01F000C  lfs f0, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6B9BC: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82D6B9C0: 419800AC  blt cr6, 0x82d6ba6c
	if ctx.cr[6].lt {
	pc = 0x82D6BA6C; continue 'dispatch;
	}
	// 82D6B9C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82D6B9C8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D6B9CC: 419A0064  beq cr6, 0x82d6ba30
	if ctx.cr[6].eq {
	pc = 0x82D6BA30; continue 'dispatch;
	}
	// 82D6B9D0: 397D0020  addi r11, r29, 0x20
	ctx.r[11].s64 = ctx.r[29].s64 + 32;
	// 82D6B9D4: 3860FFE0  li r3, -0x20
	ctx.r[3].s64 = -32;
	// 82D6B9D8: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82D6B9DC: 38A0FFF0  li r5, -0x10
	ctx.r[5].s64 = -16;
	// 82D6B9E0: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82D6B9E4: 80FF0010  lwz r7, 0x10(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6B9E8: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6B9EC: 811F0020  lwz r8, 0x20(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D6B9F0: 7D475050  subf r10, r7, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	// 82D6B9F4: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82D6B9F8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82D6B9FC: 5547083C  slwi r7, r10, 1
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D6BA00: 7F09F040  cmplw cr6, r9, r30
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82D6BA04: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82D6BA08: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D6BA0C: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6BB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D6BB08 size=372
    let mut pc: u32 = 0x82D6BB08;
    'dispatch: loop {
        match pc {
            0x82D6BB08 => {
    //   block [0x82D6BB08..0x82D6BC7C)
	// 82D6BB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6BB0C: 4BF3D8ED  bl 0x82ca93f8
	ctx.lr = 0x82D6BB10;
	sub_82CA93D0(ctx, base);
	// 82D6BB10: 81030010  lwz r8, 0x10(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6BB14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D6BB18: 81230024  lwz r9, 0x24(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6BB1C: 3B230024  addi r25, r3, 0x24
	ctx.r[25].s64 = ctx.r[3].s64 + 36;
	// 82D6BB20: 0CC80000  twi 6, r8, 0
	// 82D6BB24: 552B083E  rotlwi r11, r9, 1
	ctx.r[11].u64 = ((ctx.r[9].u32).rotate_left(1)) as u64;
	// 82D6BB28: 38C30010  addi r6, r3, 0x10
	ctx.r[6].s64 = ctx.r[3].s64 + 16;
	// 82D6BB2C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D6BB30: 7D0B5878  andc r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 & !ctx.r[11].u64;
	// 82D6BB34: 0CABFFFF  twi 5, r11, -1
	// 82D6BB38: 7D2943D7  divw. r9, r9, r8
	ctx.r[9].s32 = ctx.r[9].s32 / ctx.r[8].s32;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D6BB3C: 4081013C  ble 0x82d6bc78
	if !ctx.cr[0].gt {
	pc = 0x82D6BC78; continue 'dispatch;
	}
	// 82D6BB40: 3BE50010  addi r31, r5, 0x10
	ctx.r[31].s64 = ctx.r[5].s64 + 16;
	// 82D6BB44: 3BA30020  addi r29, r3, 0x20
	ctx.r[29].s64 = ctx.r[3].s64 + 32;
	// 82D6BB48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D6BB4C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D6BB50: 3B850020  addi r28, r5, 0x20
	ctx.r[28].s64 = ctx.r[5].s64 + 32;
	// 82D6BB54: 3B63000C  addi r27, r3, 0xc
	ctx.r[27].s64 = ctx.r[3].s64 + 12;
	// 82D6BB58: 3BCB4CB0  addi r30, r11, 0x4cb0
	ctx.r[30].s64 = ctx.r[11].s64 + 19632;
	// 82D6BB5C: 3B400010  li r26, 0x10
	ctx.r[26].s64 = 16;
	// 82D6BB60: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82D6BB64: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6BC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D6BC80 size=172
    let mut pc: u32 = 0x82D6BC80;
    'dispatch: loop {
        match pc {
            0x82D6BC80 => {
    //   block [0x82D6BC80..0x82D6BD2C)
	// 82D6BC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6BC84: 4BF3D785  bl 0x82ca9408
	ctx.lr = 0x82D6BC88;
	sub_82CA93D0(ctx, base);
	// 82D6BC88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D6BC8C: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82D6BC90: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82D6BC94: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D6BC98: 91650018  stw r11, 0x18(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82D6BC9C: 9165001C  stw r11, 0x1c(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82D6BCA0: 91650020  stw r11, 0x20(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82D6BCA4: 419A0084  beq cr6, 0x82d6bd28
	if ctx.cr[6].eq {
	pc = 0x82D6BD28; continue 'dispatch;
	}
	// 82D6BCA8: A1690000  lhz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6BCAC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D6BCB0: 80650018  lwz r3, 0x18(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D6BCB4: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 82D6BCB8: 556BD1BE  srwi r11, r11, 6
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D6BCBC: 80C5001C  lwz r6, 0x1c(r5)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D6BCC0: 81050020  lwz r8, 0x20(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D6BCC4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D6BCC8: 557FDFFE  rlwinm r31, r11, 0x1b, 0x1f, 0x1f
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82D6BCCC: 5567D7FE  rlwinm r7, r11, 0x1a, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82D6BCD0: 557DFFFE  rlwinm r29, r11, 0x1f, 0x1f, 0x1f
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82D6BCD4: 7CE7FA14  add r7, r7, r31
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 82D6BCD8: 557EE7FE  rlwinm r30, r11, 0x1c, 0x1f, 0x1f
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82D6BCDC: 5564F7FE  rlwinm r4, r11, 0x1e, 0x1f, 0x1f
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82D6BCE0: 557FBFFE  rlwinm r31, r11, 0x17, 0x1f, 0x1f
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x000001FFu64;
	// 82D6BCE4: 557CC7FE  rlwinm r28, r11, 0x18, 0x1f, 0x1f
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82D6BCE8: 7C84EA14  add r4, r4, r29
	ctx.r[4].u64 = ctx.r[4].u64 + ctx.r[29].u64;
	// 82D6BCEC: 7CE7F214  add r7, r7, r30
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[30].u64;
	// 82D6BCF0: 557D07FE  clrlwi r29, r11, 0x1f
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82D6BCF4: 557EEFFE  rlwinm r30, r11, 0x1d, 0x1f, 0x1f
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82D6BCF8: 556BCFFE  rlwinm r11, r11, 0x19, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 82D6BCFC: 7FFFE214  add r31, r31, r28
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[28].u64;
	// 82D6BD00: 7C84EA14  add r4, r4, r29
	ctx.r[4].u64 = ctx.r[4].u64 + ctx.r[29].u64;
	// 82D6BD04: 7CE7F214  add r7, r7, r30
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[30].u64;
	// 82D6BD08: 7D7F5A14  add r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82D6BD0C: 7C841A14  add r4, r4, r3
	ctx.r[4].u64 = ctx.r[4].u64 + ctx.r[3].u64;
	// 82D6BD10: 7CE73214  add r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 82D6BD14: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82D6BD18: 90850018  stw r4, 0x18(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 82D6BD1C: 90E5001C  stw r7, 0x1c(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(28 as u32), ctx.r[7].u32 ) };
	// 82D6BD20: 91650020  stw r11, 0x20(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82D6BD24: 409AFF84  bne cr6, 0x82d6bca8
	if !ctx.cr[6].eq {
	pc = 0x82D6BCA8; continue 'dispatch;
	}
	// 82D6BD28: 4BF3D730  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6BD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D6BD30 size=352
    let mut pc: u32 = 0x82D6BD30;
    'dispatch: loop {
        match pc {
            0x82D6BD30 => {
    //   block [0x82D6BD30..0x82D6BE90)
	// 82D6BD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6BD34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D6BD38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D6BD3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D6BD40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6BD44: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82D6BD48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D6BD4C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D6BD50: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D6BD54: C00A0C18  lfs f0, 0xc18(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6BD58: D0050030  stfs f0, 0x30(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82D6BD5C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D6BD60: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D6BD64: 91650008  stw r11, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D6BD68: 9165000C  stw r11, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82D6BD6C: 91650010  stw r11, 0x10(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82D6BD70: 91650014  stw r11, 0x14(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82D6BD74: 419A0070  beq cr6, 0x82d6bde4
	if ctx.cr[6].eq {
	pc = 0x82D6BDE4; continue 'dispatch;
	}
	// 82D6BD78: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82D6BD7C: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 82D6BD80: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6BD84: 556B07BE  clrlwi r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82D6BD88: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82D6BD8C: 409A0010  bne cr6, 0x82d6bd9c
	if !ctx.cr[6].eq {
	pc = 0x82D6BD9C; continue 'dispatch;
	}
	// 82D6BD90: 8165000C  lwz r11, 0xc(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6BD94: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 82D6BD98: 9165000C  stw r11, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82D6BD9C: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6BDA0: 556B073A  rlwinm r11, r11, 0, 0x1c, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D6BDA4: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 82D6BDA8: 409A0010  bne cr6, 0x82d6bdb8
	if !ctx.cr[6].eq {
	pc = 0x82D6BDB8; continue 'dispatch;
	}
	// 82D6BDAC: 81650010  lwz r11, 0x10(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6BDB0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D6BDB4: 91650010  stw r11, 0x10(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82D6BDB8: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6BDBC: 556B06B6  rlwinm r11, r11, 0, 0x1a, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D6BDC0: 2B0B0020  cmplwi cr6, r11, 0x20
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32 as u32, &mut ctx.xer);
	// 82D6BDC4: 409A0010  bne cr6, 0x82d6bdd4
	if !ctx.cr[6].eq {
	pc = 0x82D6BDD4; continue 'dispatch;
	}
	// 82D6BDC8: 81650014  lwz r11, 0x14(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D6BDCC: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 82D6BDD0: 91650014  stw r11, 0x14(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82D6BDD4: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82D6BDD8: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82D6BDDC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D6BDE0: 409AFFA0  bne cr6, 0x82d6bd80
	if !ctx.cr[6].eq {
	pc = 0x82D6BD80; continue 'dispatch;
	}
	// 82D6BDE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D6BDE8: 4BFFFE99  bl 0x82d6bc80
	ctx.lr = 0x82D6BDEC;
	sub_82D6BC80(ctx, base);
	// 82D6BDEC: 80E50010  lwz r7, 0x10(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6BDF0: 57E8103A  slwi r8, r31, 2
	ctx.r[8].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D6BDF4: 8165001C  lwz r11, 0x1c(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D6BDF8: 81450020  lwz r10, 0x20(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D6BDFC: 7C674050  subf r3, r7, r8
	ctx.r[3].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 82D6BE00: 81250018  lwz r9, 0x18(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D6BE04: 57E7103A  slwi r7, r31, 2
	ctx.r[7].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D6BE08: 80C5000C  lwz r6, 0xc(r5)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6BE0C: 7D0A5A14  add r8, r10, r11
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82D6BE10: 80850014  lwz r4, 0x14(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D6BE14: 7CFF3A14  add r7, r31, r7
	ctx.r[7].u64 = ctx.r[31].u64 + ctx.r[7].u64;
	// 82D6BE18: 7D084A14  add r8, r8, r9
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82D6BE1C: 54E7083C  slwi r7, r7, 1
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D6BE20: 791E0020  clrldi r30, r8, 0x20
	ctx.r[30].u64 = ctx.r[8].u64 & 0x00000000FFFFFFFFu64;
	// 82D6BE24: 78E70020  clrldi r7, r7, 0x20
	ctx.r[7].u64 = ctx.r[7].u64 & 0x00000000FFFFFFFFu64;
	// 82D6BE28: 57E8083C  slwi r8, r31, 1
	ctx.r[8].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D6BE2C: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 82D6BE30: 7D7F4214  add r11, r31, r8
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[8].u64;
	// 82D6BE34: FBC10050  std r30, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u64 ) };
	// 82D6BE38: F8E10058  std r7, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u64 ) };
	// 82D6BE3C: 7D295850  subf r9, r9, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82D6BE40: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82D6BE44: 7D464850  subf r10, r6, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[6].s64;
	// 82D6BE48: 90650004  stw r3, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82D6BE4C: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82D6BE50: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D6BE54: 91650008  stw r11, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D6BE58: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82D6BE5C: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82D6BE60: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82D6BE64: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82D6BE68: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82D6BE6C: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82D6BE70: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82D6BE74: D0050030  stfs f0, 0x30(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82D6BE78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D6BE7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D6BE80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D6BE84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D6BE88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D6BE8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6BE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D6BE90 size=2048
    let mut pc: u32 = 0x82D6BE90;
    'dispatch: loop {
        match pc {
            0x82D6BE90 => {
    //   block [0x82D6BE90..0x82D6C690)
	// 82D6BE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6BE94: 4BF3D53D  bl 0x82ca93d0
	ctx.lr = 0x82D6BE98;
	sub_82CA93D0(ctx, base);
	// 82D6BE98: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D6BE9C: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6BEA0: 7D334B78  mr r19, r9
	ctx.r[19].u64 = ctx.r[9].u64;
	// 82D6BEA4: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82D6BEA8: 41990008  bgt cr6, 0x82d6beb0
	if ctx.cr[6].gt {
	pc = 0x82D6BEB0; continue 'dispatch;
	}
	// 82D6BEAC: 7D735B78  mr r19, r11
	ctx.r[19].u64 = ctx.r[11].u64;
	// 82D6BEB0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6BEB4: 3A200000  li r17, 0
	ctx.r[17].s64 = 0;
	// 82D6BEB8: 7E2E8B78  mr r14, r17
	ctx.r[14].u64 = ctx.r[17].u64;
	// 82D6BEBC: 7E3A8B78  mr r26, r17
	ctx.r[26].u64 = ctx.r[17].u64;
	// 82D6BEC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D6BEC4: 419A07C8  beq cr6, 0x82d6c68c
	if ctx.cr[6].eq {
	pc = 0x82D6C68C; continue 'dispatch;
	}
	// 82D6BEC8: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D6BECC: 9221FF60  stw r17, -0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.r[17].u32 ) };
	// 82D6BED0: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D6BED4: 5550103A  slwi r16, r10, 2
	ctx.r[16].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[16].u64 = ctx.r[16].u32 as u64;
	// 82D6BED8: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6BEDC: 8101FF60  lwz r8, -0xa0(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-160 as u32) ) } as u64;
	// 82D6BEE0: 7D4A422E  lhzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82D6BEE4: 7D4F5378  mr r15, r10
	ctx.r[15].u64 = ctx.r[10].u64;
	// 82D6BEE8: 5552D1BE  srwi r18, r10, 6
	ctx.r[18].u32 = ctx.r[10].u32.wrapping_shr(6);
	ctx.r[18].u64 = ctx.r[18].u32 as u64;
	// 82D6BEEC: 55EA07BE  clrlwi r10, r15, 0x1e
	ctx.r[10].u64 = ctx.r[15].u32 as u64 & 0x00000003u64;
	// 82D6BEF0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D6BEF4: 409A0260  bne cr6, 0x82d6c154
	if !ctx.cr[6].eq {
	pc = 0x82D6C154; continue 'dispatch;
	}
	// 82D6BEF8: 7D4E99D6  mullw r10, r14, r19
	ctx.r[10].s64 = (ctx.r[14].s32 as i64) * (ctx.r[19].s32 as i64);
	// 82D6BEFC: 7D0A4A14  add r8, r10, r9
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82D6BF00: 5546103A  slwi r6, r10, 2
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D6BF04: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D6BF08: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82D6BF0C: 5654043E  clrlwi r20, r18, 0x10
	ctx.r[20].u64 = ctx.r[18].u32 as u64 & 0x0000FFFFu64;
	// 82D6BF10: 3AA00002  li r21, 2
	ctx.r[21].s64 = 2;
	// 82D6BF14: 5676103A  slwi r22, r19, 2
	ctx.r[22].u32 = ctx.r[19].u32.wrapping_shl(2);
	ctx.r[22].u64 = ctx.r[22].u32 as u64;
	// 82D6BF18: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 82D6BF1C: 7F062214  add r24, r6, r4
	ctx.r[24].u64 = ctx.r[6].u64 + ctx.r[4].u64;
	// 82D6BF20: 7EE82214  add r23, r8, r4
	ctx.r[23].u64 = ctx.r[8].u64 + ctx.r[4].u64;
	// 82D6BF24: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D6BF28: 7D4AA830  slw r10, r10, r21
	if (ctx.r[21].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) << ((ctx.r[21].u8 & 0x1F) as u32)) as u64;
	}
	// 82D6BF2C: 7D4AA038  and r10, r10, r20
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[20].u64;
	// 82D6BF30: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D6BF34: 419A0210  beq cr6, 0x82d6c144
	if ctx.cr[6].eq {
	pc = 0x82D6C144; continue 'dispatch;
	}
	// 82D6BF38: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82D6BF3C: 2F090004  cmpwi cr6, r9, 4
	ctx.cr[6].compare_i32(ctx.r[9].s32, 4, &mut ctx.xer);
	// 82D6BF40: 419800C0  blt cr6, 0x82d6c000
	if ctx.cr[6].lt {
	pc = 0x82D6C000; continue 'dispatch;
	}
	// 82D6BF44: 7CB13A14  add r5, r17, r7
	ctx.r[5].u64 = ctx.r[17].u64 + ctx.r[7].u64;
	// 82D6BF48: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D6BF4C: 54BF103A  slwi r31, r5, 2
	ctx.r[31].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82D6BF50: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6BF54: 7CCAD214  add r6, r10, r26
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[26].u64;
	// 82D6BF58: 7CAB2A14  add r5, r11, r5
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82D6BF5C: 54DC083C  slwi r28, r6, 1
	ctx.r[28].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82D6BF60: 7CA5D214  add r5, r5, r26
	ctx.r[5].u64 = ctx.r[5].u64 + ctx.r[26].u64;
	// 82D6BF64: 557D083C  slwi r29, r11, 1
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82D6BF68: 54BB083C  slwi r27, r5, 1
	ctx.r[27].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82D6BF6C: 7CC6E214  add r6, r6, r28
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[28].u64;
	// 82D6BF70: 7CA5DA14  add r5, r5, r27
	ctx.r[5].u64 = ctx.r[5].u64 + ctx.r[27].u64;
	// 82D6BF74: 3949FFFC  addi r10, r9, -4
	ctx.r[10].s64 = ctx.r[9].s64 + -4;
	// 82D6BF78: 7F8BEA14  add r28, r11, r29
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82D6BF7C: 554AF0BE  srwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D6BF80: 54DD103A  slwi r29, r6, 2
	ctx.r[29].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82D6BF84: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6BF88: 390A0001  addi r8, r10, 1
	ctx.r[8].s64 = ctx.r[10].s64 + 1;
	// 82D6BF8C: 7FD03A14  add r30, r16, r7
	ctx.r[30].u64 = ctx.r[16].u64 + ctx.r[7].u64;
	// 82D6BF90: 7FBD3A14  add r29, r29, r7
	ctx.r[29].u64 = ctx.r[29].u64 + ctx.r[7].u64;
	// 82D6BF94: 7CA53A14  add r5, r5, r7
	ctx.r[5].u64 = ctx.r[5].u64 + ctx.r[7].u64;
	// 82D6BF98: 57863032  slwi r6, r28, 6
	ctx.r[6].u32 = ctx.r[28].u32.wrapping_shl(6);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D6BF9C: 7F0AC378  mr r10, r24
	ctx.r[10].u64 = ctx.r[24].u64;
	// 82D6BFA0: 57DE103A  slwi r30, r30, 2
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82D6BFA4: 551C103A  slwi r28, r8, 2
	ctx.r[28].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82D6BFA8: 57BD103A  slwi r29, r29, 2
	ctx.r[29].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82D6BFAC: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6BFB0: 83630000  lwz r27, 0(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6BFB4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82D6BFB8: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82D6BFBC: 7C1FDC2E  lfsx f0, r31, r27
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[27].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6BFC0: 7FFF3214  add r31, r31, r6
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[6].u64;
	// 82D6BFC4: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D6BFC8: 83630000  lwz r27, 0(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6BFCC: 7C1EDC2E  lfsx f0, r30, r27
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[27].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6BFD0: 7FDE3214  add r30, r30, r6
	ctx.r[30].u64 = ctx.r[30].u64 + ctx.r[6].u64;
	// 82D6BFD4: D00A0004  stfs f0, 4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D6BFD8: 83630000  lwz r27, 0(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6BFDC: 7C1DDC2E  lfsx f0, r29, r27
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[27].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6BFE0: 7FBD3214  add r29, r29, r6
	ctx.r[29].u64 = ctx.r[29].u64 + ctx.r[6].u64;
	// 82D6BFE4: D00A0008  stfs f0, 8(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82D6BFE8: 83630000  lwz r27, 0(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6BFEC: 7C05DC2E  lfsx f0, r5, r27
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[27].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6BFF0: 7CA53214  add r5, r5, r6
	ctx.r[5].u64 = ctx.r[5].u64 + ctx.r[6].u64;
	// 82D6BFF4: D00A000C  stfs f0, 0xc(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82D6BFF8: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82D6BFFC: 409AFFB4  bne cr6, 0x82d6bfb0
	if !ctx.cr[6].eq {
	pc = 0x82D6BFB0; continue 'dispatch;
	}
	// 82D6C000: 7F1C4840  cmplw cr6, r28, r9
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D6C004: 4098005C  bge cr6, 0x82d6c060
	if !ctx.cr[6].lt {
	pc = 0x82D6C060; continue 'dispatch;
	}
	// 82D6C008: 7D5C59D6  mullw r10, r28, r11
	ctx.r[10].s64 = (ctx.r[28].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82D6C00C: 7D4AD214  add r10, r10, r26
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[26].u64;
	// 82D6C010: 7CD9E214  add r6, r25, r28
	ctx.r[6].u64 = ctx.r[25].u64 + ctx.r[28].u64;
	// 82D6C014: 5545083C  slwi r5, r10, 1
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C018: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D6C01C: 7D4A2A14  add r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 82D6C020: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C024: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D6C028: 7CAB2A14  add r5, r11, r5
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82D6C02C: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82D6C030: 7D1C4850  subf r8, r28, r9
	ctx.r[8].s64 = ctx.r[9].s64 - ctx.r[28].s64;
	// 82D6C034: 7CC62214  add r6, r6, r4
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[4].u64;
	// 82D6C038: 54A52036  slwi r5, r5, 4
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C03C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D6C040: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C044: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82D6C048: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82D6C04C: 7C0AFC2E  lfsx f0, r10, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C050: 7D455214  add r10, r5, r10
	ctx.r[10].u64 = ctx.r[5].u64 + ctx.r[10].u64;
	// 82D6C054: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D6C058: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 82D6C05C: 409AFFE4  bne cr6, 0x82d6c040
	if !ctx.cr[6].eq {
	pc = 0x82D6C040; continue 'dispatch;
	}
	// 82D6C060: 3949FFFF  addi r10, r9, -1
	ctx.r[10].s64 = ctx.r[9].s64 + -1;
	// 82D6C064: 7D099850  subf r8, r9, r19
	ctx.r[8].s64 = ctx.r[19].s64 - ctx.r[9].s64;
	// 82D6C068: 7D4A59D6  mullw r10, r10, r11
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82D6C06C: 7CCAD214  add r6, r10, r26
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[26].u64;
	// 82D6C070: 7D3F4B78  mr r31, r9
	ctx.r[31].u64 = ctx.r[9].u64;
	// 82D6C074: 2F080004  cmpwi cr6, r8, 4
	ctx.cr[6].compare_i32(ctx.r[8].s32, 4, &mut ctx.xer);
	// 82D6C078: 41980074  blt cr6, 0x82d6c0ec
	if ctx.cr[6].lt {
	pc = 0x82D6C0EC; continue 'dispatch;
	}
	// 82D6C07C: 7D099850  subf r8, r9, r19
	ctx.r[8].s64 = ctx.r[19].s64 - ctx.r[9].s64;
	// 82D6C080: 54C5083C  slwi r5, r6, 1
	ctx.r[5].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C084: 3908FFFC  addi r8, r8, -4
	ctx.r[8].s64 = ctx.r[8].s64 + -4;
	// 82D6C088: 7CA62A14  add r5, r6, r5
	ctx.r[5].u64 = ctx.r[6].u64 + ctx.r[5].u64;
	// 82D6C08C: 5508F0BE  srwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D6C090: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C094: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82D6C098: 7CA53A14  add r5, r5, r7
	ctx.r[5].u64 = ctx.r[5].u64 + ctx.r[7].u64;
	// 82D6C09C: 551F103A  slwi r31, r8, 2
	ctx.r[31].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82D6C0A0: 7EEABB78  mr r10, r23
	ctx.r[10].u64 = ctx.r[23].u64;
	// 82D6C0A4: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C0A8: 7FFF4A14  add r31, r31, r9
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[9].u64;
	// 82D6C0AC: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C0B0: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82D6C0B4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82D6C0B8: 7C05F42E  lfsx f0, r5, r30
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[30].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C0BC: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D6C0C0: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C0C4: 7C05F42E  lfsx f0, r5, r30
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[30].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C0C8: D00A0004  stfs f0, 4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D6C0CC: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C0D0: 7C05F42E  lfsx f0, r5, r30
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[30].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C0D4: D00A0008  stfs f0, 8(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82D6C0D8: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C0DC: 7C05F42E  lfsx f0, r5, r30
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[30].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C0E0: D00A000C  stfs f0, 0xc(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82D6C0E4: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82D6C0E8: 409AFFC4  bne cr6, 0x82d6c0ac
	if !ctx.cr[6].eq {
	pc = 0x82D6C0AC; continue 'dispatch;
	}
	// 82D6C0EC: 7F1F9840  cmplw cr6, r31, r19
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[19].u32, &mut ctx.xer);
	// 82D6C0F0: 40980044  bge cr6, 0x82d6c134
	if !ctx.cr[6].lt {
	pc = 0x82D6C134; continue 'dispatch;
	}
	// 82D6C0F4: 54C5083C  slwi r5, r6, 1
	ctx.r[5].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C0F8: 7D19FA14  add r8, r25, r31
	ctx.r[8].u64 = ctx.r[25].u64 + ctx.r[31].u64;
	// 82D6C0FC: 7CC62A14  add r6, r6, r5
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[5].u64;
	// 82D6C100: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D6C104: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D6C108: 7D5F9850  subf r10, r31, r19
	ctx.r[10].s64 = ctx.r[19].s64 - ctx.r[31].s64;
	// 82D6C10C: 7CC63A14  add r6, r6, r7
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[7].u64;
	// 82D6C110: 7D082214  add r8, r8, r4
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[4].u64;
	// 82D6C114: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D6C118: 80A30000  lwz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C11C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D6C120: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D6C124: 7C062C2E  lfsx f0, r6, r5
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C128: D0080000  stfs f0, 0(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D6C12C: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82D6C130: 409AFFE8  bne cr6, 0x82d6c118
	if !ctx.cr[6].eq {
	pc = 0x82D6C118; continue 'dispatch;
	}
	// 82D6C134: 39CE0001  addi r14, r14, 1
	ctx.r[14].s64 = ctx.r[14].s64 + 1;
	// 82D6C138: 7F399A14  add r25, r25, r19
	ctx.r[25].u64 = ctx.r[25].u64 + ctx.r[19].u64;
	// 82D6C13C: 7F16C214  add r24, r22, r24
	ctx.r[24].u64 = ctx.r[22].u64 + ctx.r[24].u64;
	// 82D6C140: 7EF6BA14  add r23, r22, r23
	ctx.r[23].u64 = ctx.r[22].u64 + ctx.r[23].u64;
	// 82D6C144: 3AB5FFFF  addi r21, r21, -1
	ctx.r[21].s64 = ctx.r[21].s64 + -1;
	// 82D6C148: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82D6C14C: 2F15FFFF  cmpwi cr6, r21, -1
	ctx.cr[6].compare_i32(ctx.r[21].s32, -1, &mut ctx.xer);
	// 82D6C150: 4199FDD4  bgt cr6, 0x82d6bf24
	if ctx.cr[6].gt {
	pc = 0x82D6BF24; continue 'dispatch;
	}
	// 82D6C154: 55EA073A  rlwinm r10, r15, 0, 0x1c, 0x1d
	ctx.r[10].u64 = ctx.r[15].u32 as u64 & 0xFFFFFFFFu64;
	// 82D6C158: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D6C15C: 409A0284  bne cr6, 0x82d6c3e0
	if !ctx.cr[6].eq {
	pc = 0x82D6C3E0; continue 'dispatch;
	}
	// 82D6C160: 7D4E99D6  mullw r10, r14, r19
	ctx.r[10].s64 = (ctx.r[14].s32 as i64) * (ctx.r[19].s32 as i64);
	// 82D6C164: 7CEA4A14  add r7, r10, r9
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82D6C168: 38CA0002  addi r6, r10, 2
	ctx.r[6].s64 = ctx.r[10].s64 + 2;
	// 82D6C16C: 38E70002  addi r7, r7, 2
	ctx.r[7].s64 = ctx.r[7].s64 + 2;
	// 82D6C170: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D6C174: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D6C178: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82D6C17C: 5654043E  clrlwi r20, r18, 0x10
	ctx.r[20].u64 = ctx.r[18].u32 as u64 & 0x0000FFFFu64;
	// 82D6C180: 3AA00006  li r21, 6
	ctx.r[21].s64 = 6;
	// 82D6C184: 5676103A  slwi r22, r19, 2
	ctx.r[22].u32 = ctx.r[19].u32.wrapping_shl(2);
	ctx.r[22].u64 = ctx.r[22].u32 as u64;
	// 82D6C188: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 82D6C18C: 7F062214  add r24, r6, r4
	ctx.r[24].u64 = ctx.r[6].u64 + ctx.r[4].u64;
	// 82D6C190: 7EE72214  add r23, r7, r4
	ctx.r[23].u64 = ctx.r[7].u64 + ctx.r[4].u64;
	// 82D6C194: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D6C198: 7D4AA830  slw r10, r10, r21
	if (ctx.r[21].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) << ((ctx.r[21].u8 & 0x1F) as u32)) as u64;
	}
	// 82D6C19C: 7D4AA038  and r10, r10, r20
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[20].u64;
	// 82D6C1A0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D6C1A4: 419A022C  beq cr6, 0x82d6c3d0
	if ctx.cr[6].eq {
	pc = 0x82D6C3D0; continue 'dispatch;
	}
	// 82D6C1A8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82D6C1AC: 2F090004  cmpwi cr6, r9, 4
	ctx.cr[6].compare_i32(ctx.r[9].s32, 4, &mut ctx.xer);
	// 82D6C1B0: 419800D0  blt cr6, 0x82d6c280
	if ctx.cr[6].lt {
	pc = 0x82D6C280; continue 'dispatch;
	}
	// 82D6C1B4: 7CB04214  add r5, r16, r8
	ctx.r[5].u64 = ctx.r[16].u64 + ctx.r[8].u64;
	// 82D6C1B8: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D6C1BC: 38A50004  addi r5, r5, 4
	ctx.r[5].s64 = ctx.r[5].s64 + 4;
	// 82D6C1C0: 7CCAD214  add r6, r10, r26
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[26].u64;
	// 82D6C1C4: 54BE103A  slwi r30, r5, 2
	ctx.r[30].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82D6C1C8: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C1CC: 54DC083C  slwi r28, r6, 1
	ctx.r[28].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82D6C1D0: 7CAB2A14  add r5, r11, r5
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82D6C1D4: 557D083C  slwi r29, r11, 1
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82D6C1D8: 7CA5D214  add r5, r5, r26
	ctx.r[5].u64 = ctx.r[5].u64 + ctx.r[26].u64;
	// 82D6C1DC: 7CC6E214  add r6, r6, r28
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[28].u64;
	// 82D6C1E0: 54BB083C  slwi r27, r5, 1
	ctx.r[27].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82D6C1E4: 7F8BEA14  add r28, r11, r29
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82D6C1E8: 7CA5DA14  add r5, r5, r27
	ctx.r[5].u64 = ctx.r[5].u64 + ctx.r[27].u64;
	// 82D6C1EC: 54DD103A  slwi r29, r6, 2
	ctx.r[29].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82D6C1F0: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C1F4: 3949FFFC  addi r10, r9, -4
	ctx.r[10].s64 = ctx.r[9].s64 + -4;
	// 82D6C1F8: 7FBD4214  add r29, r29, r8
	ctx.r[29].u64 = ctx.r[29].u64 + ctx.r[8].u64;
	// 82D6C1FC: 554AF0BE  srwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D6C200: 7CA54214  add r5, r5, r8
	ctx.r[5].u64 = ctx.r[5].u64 + ctx.r[8].u64;
	// 82D6C204: 7FF14214  add r31, r17, r8
	ctx.r[31].u64 = ctx.r[17].u64 + ctx.r[8].u64;
	// 82D6C208: 38EA0001  addi r7, r10, 1
	ctx.r[7].s64 = ctx.r[10].s64 + 1;
	// 82D6C20C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82D6C210: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82D6C214: 38A50004  addi r5, r5, 4
	ctx.r[5].s64 = ctx.r[5].s64 + 4;
	// 82D6C218: 57863032  slwi r6, r28, 6
	ctx.r[6].u32 = ctx.r[28].u32.wrapping_shl(6);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D6C21C: 7F0AC378  mr r10, r24
	ctx.r[10].u64 = ctx.r[24].u64;
	// 82D6C220: 57FF103A  slwi r31, r31, 2
	ctx.r[31].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82D6C224: 54FC103A  slwi r28, r7, 2
	ctx.r[28].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82D6C228: 57BD103A  slwi r29, r29, 2
	ctx.r[29].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82D6C22C: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C230: 83630000  lwz r27, 0(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C234: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 82D6C238: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82D6C23C: 7C1FDC2E  lfsx f0, r31, r27
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[27].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C240: 7FFF3214  add r31, r31, r6
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[6].u64;
	// 82D6C244: D00AFFF8  stfs f0, -8(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82D6C248: 83630000  lwz r27, 0(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C24C: 7C1EDC2E  lfsx f0, r30, r27
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[27].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C250: 7FDE3214  add r30, r30, r6
	ctx.r[30].u64 = ctx.r[30].u64 + ctx.r[6].u64;
	// 82D6C254: D00AFFFC  stfs f0, -4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82D6C258: 83630000  lwz r27, 0(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C25C: 7C1DDC2E  lfsx f0, r29, r27
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[27].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C260: 7FBD3214  add r29, r29, r6
	ctx.r[29].u64 = ctx.r[29].u64 + ctx.r[6].u64;
	// 82D6C264: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D6C268: 83630000  lwz r27, 0(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C26C: 7C05DC2E  lfsx f0, r5, r27
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[27].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C270: 7CA53214  add r5, r5, r6
	ctx.r[5].u64 = ctx.r[5].u64 + ctx.r[6].u64;
	// 82D6C274: D00A0004  stfs f0, 4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D6C278: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82D6C27C: 409AFFB4  bne cr6, 0x82d6c230
	if !ctx.cr[6].eq {
	pc = 0x82D6C230; continue 'dispatch;
	}
	// 82D6C280: 7F1C4840  cmplw cr6, r28, r9
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D6C284: 40980060  bge cr6, 0x82d6c2e4
	if !ctx.cr[6].lt {
	pc = 0x82D6C2E4; continue 'dispatch;
	}
	// 82D6C288: 7D5C59D6  mullw r10, r28, r11
	ctx.r[10].s64 = (ctx.r[28].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82D6C28C: 7D4AD214  add r10, r10, r26
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[26].u64;
	// 82D6C290: 7CD9E214  add r6, r25, r28
	ctx.r[6].u64 = ctx.r[25].u64 + ctx.r[28].u64;
	// 82D6C294: 5545083C  slwi r5, r10, 1
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C298: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D6C29C: 7D4A2A14  add r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 82D6C2A0: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C2A4: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D6C2A8: 7CAB2A14  add r5, r11, r5
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82D6C2AC: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82D6C2B0: 7CFC4850  subf r7, r28, r9
	ctx.r[7].s64 = ctx.r[9].s64 - ctx.r[28].s64;
	// 82D6C2B4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82D6C2B8: 7CC62214  add r6, r6, r4
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[4].u64;
	// 82D6C2BC: 54A52036  slwi r5, r5, 4
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C2C0: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D6C2C4: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C2C8: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 82D6C2CC: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82D6C2D0: 7C1F542E  lfsx f0, r31, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C2D4: 7D455214  add r10, r5, r10
	ctx.r[10].u64 = ctx.r[5].u64 + ctx.r[10].u64;
	// 82D6C2D8: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D6C2DC: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 82D6C2E0: 409AFFE4  bne cr6, 0x82d6c2c4
	if !ctx.cr[6].eq {
	pc = 0x82D6C2C4; continue 'dispatch;
	}
	// 82D6C2E4: 3949FFFF  addi r10, r9, -1
	ctx.r[10].s64 = ctx.r[9].s64 + -1;
	// 82D6C2E8: 7CE99850  subf r7, r9, r19
	ctx.r[7].s64 = ctx.r[19].s64 - ctx.r[9].s64;
	// 82D6C2EC: 7D4A59D6  mullw r10, r10, r11
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82D6C2F0: 7CCAD214  add r6, r10, r26
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[26].u64;
	// 82D6C2F4: 7D3F4B78  mr r31, r9
	ctx.r[31].u64 = ctx.r[9].u64;
	// 82D6C2F8: 2F070004  cmpwi cr6, r7, 4
	ctx.cr[6].compare_i32(ctx.r[7].s32, 4, &mut ctx.xer);
	// 82D6C2FC: 41980078  blt cr6, 0x82d6c374
	if ctx.cr[6].lt {
	pc = 0x82D6C374; continue 'dispatch;
	}
	// 82D6C300: 7CE99850  subf r7, r9, r19
	ctx.r[7].s64 = ctx.r[19].s64 - ctx.r[9].s64;
	// 82D6C304: 54C5083C  slwi r5, r6, 1
	ctx.r[5].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C308: 38E7FFFC  addi r7, r7, -4
	ctx.r[7].s64 = ctx.r[7].s64 + -4;
	// 82D6C30C: 7CA62A14  add r5, r6, r5
	ctx.r[5].u64 = ctx.r[6].u64 + ctx.r[5].u64;
	// 82D6C310: 54E7F0BE  srwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shr(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D6C314: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C318: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82D6C31C: 7FE54214  add r31, r5, r8
	ctx.r[31].u64 = ctx.r[5].u64 + ctx.r[8].u64;
	// 82D6C320: 54E5103A  slwi r5, r7, 2
	ctx.r[5].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C324: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82D6C328: 7FE54A14  add r31, r5, r9
	ctx.r[31].u64 = ctx.r[5].u64 + ctx.r[9].u64;
	// 82D6C32C: 7EEABB78  mr r10, r23
	ctx.r[10].u64 = ctx.r[23].u64;
	// 82D6C330: 57C5103A  slwi r5, r30, 2
	ctx.r[5].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C334: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C338: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 82D6C33C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82D6C340: 7C05F42E  lfsx f0, r5, r30
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[30].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C344: D00AFFF8  stfs f0, -8(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82D6C348: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C34C: 7C05F42E  lfsx f0, r5, r30
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[30].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C350: D00AFFFC  stfs f0, -4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82D6C354: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C358: 7C05F42E  lfsx f0, r5, r30
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[30].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C35C: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D6C360: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C364: 7C05F42E  lfsx f0, r5, r30
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[30].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C368: D00A0004  stfs f0, 4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D6C36C: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82D6C370: 409AFFC4  bne cr6, 0x82d6c334
	if !ctx.cr[6].eq {
	pc = 0x82D6C334; continue 'dispatch;
	}
	// 82D6C374: 7F1F9840  cmplw cr6, r31, r19
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[19].u32, &mut ctx.xer);
	// 82D6C378: 40980048  bge cr6, 0x82d6c3c0
	if !ctx.cr[6].lt {
	pc = 0x82D6C3C0; continue 'dispatch;
	}
	// 82D6C37C: 54C5083C  slwi r5, r6, 1
	ctx.r[5].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C380: 7CF9FA14  add r7, r25, r31
	ctx.r[7].u64 = ctx.r[25].u64 + ctx.r[31].u64;
	// 82D6C384: 7CC62A14  add r6, r6, r5
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[5].u64;
	// 82D6C388: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D6C38C: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D6C390: 7D5F9850  subf r10, r31, r19
	ctx.r[10].s64 = ctx.r[19].s64 - ctx.r[31].s64;
	// 82D6C394: 7CC64214  add r6, r6, r8
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[8].u64;
	// 82D6C398: 7CE72214  add r7, r7, r4
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[4].u64;
	// 82D6C39C: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 82D6C3A0: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D6C3A4: 80A30000  lwz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C3A8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D6C3AC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D6C3B0: 7C062C2E  lfsx f0, r6, r5
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C3B4: D0070000  stfs f0, 0(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D6C3B8: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 82D6C3BC: 409AFFE8  bne cr6, 0x82d6c3a4
	if !ctx.cr[6].eq {
	pc = 0x82D6C3A4; continue 'dispatch;
	}
	// 82D6C3C0: 39CE0001  addi r14, r14, 1
	ctx.r[14].s64 = ctx.r[14].s64 + 1;
	// 82D6C3C4: 7F399A14  add r25, r25, r19
	ctx.r[25].u64 = ctx.r[25].u64 + ctx.r[19].u64;
	// 82D6C3C8: 7F16C214  add r24, r22, r24
	ctx.r[24].u64 = ctx.r[22].u64 + ctx.r[24].u64;
	// 82D6C3CC: 7EF6BA14  add r23, r22, r23
	ctx.r[23].u64 = ctx.r[22].u64 + ctx.r[23].u64;
	// 82D6C3D0: 3AB5FFFF  addi r21, r21, -1
	ctx.r[21].s64 = ctx.r[21].s64 + -1;
	// 82D6C3D4: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82D6C3D8: 2F150002  cmpwi cr6, r21, 2
	ctx.cr[6].compare_i32(ctx.r[21].s32, 2, &mut ctx.xer);
	// 82D6C3DC: 4199FDB8  bgt cr6, 0x82d6c194
	if ctx.cr[6].gt {
	pc = 0x82D6C194; continue 'dispatch;
	}
	// 82D6C3E0: 55EA06B6  rlwinm r10, r15, 0, 0x1a, 0x1b
	ctx.r[10].u64 = ctx.r[15].u32 as u64 & 0xFFFFFFFFu64;
	// 82D6C3E4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D6C3E8: 409A0284  bne cr6, 0x82d6c66c
	if !ctx.cr[6].eq {
	pc = 0x82D6C66C; continue 'dispatch;
	}
	// 82D6C3EC: 7D4E99D6  mullw r10, r14, r19
	ctx.r[10].s64 = (ctx.r[14].s32 as i64) * (ctx.r[19].s32 as i64);
	// 82D6C3F0: 7CEA4A14  add r7, r10, r9
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82D6C3F4: 38CA0002  addi r6, r10, 2
	ctx.r[6].s64 = ctx.r[10].s64 + 2;
	// 82D6C3F8: 38E70002  addi r7, r7, 2
	ctx.r[7].s64 = ctx.r[7].s64 + 2;
	// 82D6C3FC: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D6C400: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D6C404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82D6C408: 5654043E  clrlwi r20, r18, 0x10
	ctx.r[20].u64 = ctx.r[18].u32 as u64 & 0x0000FFFFu64;
	// 82D6C40C: 3AA00009  li r21, 9
	ctx.r[21].s64 = 9;
	// 82D6C410: 5676103A  slwi r22, r19, 2
	ctx.r[22].u32 = ctx.r[19].u32.wrapping_shl(2);
	ctx.r[22].u64 = ctx.r[22].u32 as u64;
	// 82D6C414: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 82D6C418: 7F062214  add r24, r6, r4
	ctx.r[24].u64 = ctx.r[6].u64 + ctx.r[4].u64;
	// 82D6C41C: 7EE72214  add r23, r7, r4
	ctx.r[23].u64 = ctx.r[7].u64 + ctx.r[4].u64;
	// 82D6C420: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D6C424: 7D4AA830  slw r10, r10, r21
	if (ctx.r[21].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) << ((ctx.r[21].u8 & 0x1F) as u32)) as u64;
	}
	// 82D6C428: 7D4AA038  and r10, r10, r20
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[20].u64;
	// 82D6C42C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D6C430: 419A022C  beq cr6, 0x82d6c65c
	if ctx.cr[6].eq {
	pc = 0x82D6C65C; continue 'dispatch;
	}
	// 82D6C434: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82D6C438: 2F090004  cmpwi cr6, r9, 4
	ctx.cr[6].compare_i32(ctx.r[9].s32, 4, &mut ctx.xer);
	// 82D6C43C: 419800D0  blt cr6, 0x82d6c50c
	if ctx.cr[6].lt {
	pc = 0x82D6C50C; continue 'dispatch;
	}
	// 82D6C440: 7CB04214  add r5, r16, r8
	ctx.r[5].u64 = ctx.r[16].u64 + ctx.r[8].u64;
	// 82D6C444: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D6C448: 38A50008  addi r5, r5, 8
	ctx.r[5].s64 = ctx.r[5].s64 + 8;
	// 82D6C44C: 7CCAD214  add r6, r10, r26
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[26].u64;
	// 82D6C450: 54BE103A  slwi r30, r5, 2
	ctx.r[30].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82D6C454: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C458: 54DC083C  slwi r28, r6, 1
	ctx.r[28].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82D6C45C: 7CAB2A14  add r5, r11, r5
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82D6C460: 557D083C  slwi r29, r11, 1
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82D6C464: 7CA5D214  add r5, r5, r26
	ctx.r[5].u64 = ctx.r[5].u64 + ctx.r[26].u64;
	// 82D6C468: 7CC6E214  add r6, r6, r28
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[28].u64;
	// 82D6C46C: 54BB083C  slwi r27, r5, 1
	ctx.r[27].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82D6C470: 7F8BEA14  add r28, r11, r29
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82D6C474: 7CA5DA14  add r5, r5, r27
	ctx.r[5].u64 = ctx.r[5].u64 + ctx.r[27].u64;
	// 82D6C478: 54DD103A  slwi r29, r6, 2
	ctx.r[29].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82D6C47C: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C480: 3949FFFC  addi r10, r9, -4
	ctx.r[10].s64 = ctx.r[9].s64 + -4;
	// 82D6C484: 7FBD4214  add r29, r29, r8
	ctx.r[29].u64 = ctx.r[29].u64 + ctx.r[8].u64;
	// 82D6C488: 554AF0BE  srwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D6C48C: 7CA54214  add r5, r5, r8
	ctx.r[5].u64 = ctx.r[5].u64 + ctx.r[8].u64;
	// 82D6C490: 7FF14214  add r31, r17, r8
	ctx.r[31].u64 = ctx.r[17].u64 + ctx.r[8].u64;
	// 82D6C494: 38EA0001  addi r7, r10, 1
	ctx.r[7].s64 = ctx.r[10].s64 + 1;
	// 82D6C498: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82D6C49C: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 82D6C4A0: 38A50008  addi r5, r5, 8
	ctx.r[5].s64 = ctx.r[5].s64 + 8;
	// 82D6C4A4: 57863032  slwi r6, r28, 6
	ctx.r[6].u32 = ctx.r[28].u32.wrapping_shl(6);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D6C4A8: 7F0AC378  mr r10, r24
	ctx.r[10].u64 = ctx.r[24].u64;
	// 82D6C4AC: 57FF103A  slwi r31, r31, 2
	ctx.r[31].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82D6C4B0: 54FC103A  slwi r28, r7, 2
	ctx.r[28].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82D6C4B4: 57BD103A  slwi r29, r29, 2
	ctx.r[29].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82D6C4B8: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C4BC: 83630000  lwz r27, 0(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C4C0: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 82D6C4C4: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82D6C4C8: 7C1FDC2E  lfsx f0, r31, r27
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[27].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C4CC: 7FFF3214  add r31, r31, r6
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[6].u64;
	// 82D6C4D0: D00AFFF8  stfs f0, -8(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82D6C4D4: 83630000  lwz r27, 0(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C4D8: 7C1EDC2E  lfsx f0, r30, r27
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[27].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C4DC: 7FDE3214  add r30, r30, r6
	ctx.r[30].u64 = ctx.r[30].u64 + ctx.r[6].u64;
	// 82D6C4E0: D00AFFFC  stfs f0, -4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82D6C4E4: 83630000  lwz r27, 0(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C4E8: 7C1DDC2E  lfsx f0, r29, r27
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[27].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C4EC: 7FBD3214  add r29, r29, r6
	ctx.r[29].u64 = ctx.r[29].u64 + ctx.r[6].u64;
	// 82D6C4F0: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D6C4F4: 83630000  lwz r27, 0(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C4F8: 7C05DC2E  lfsx f0, r5, r27
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[27].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C4FC: 7CA53214  add r5, r5, r6
	ctx.r[5].u64 = ctx.r[5].u64 + ctx.r[6].u64;
	// 82D6C500: D00A0004  stfs f0, 4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D6C504: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82D6C508: 409AFFB4  bne cr6, 0x82d6c4bc
	if !ctx.cr[6].eq {
	pc = 0x82D6C4BC; continue 'dispatch;
	}
	// 82D6C50C: 7F1C4840  cmplw cr6, r28, r9
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D6C510: 40980060  bge cr6, 0x82d6c570
	if !ctx.cr[6].lt {
	pc = 0x82D6C570; continue 'dispatch;
	}
	// 82D6C514: 7D5C59D6  mullw r10, r28, r11
	ctx.r[10].s64 = (ctx.r[28].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82D6C518: 7D4AD214  add r10, r10, r26
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[26].u64;
	// 82D6C51C: 7CD9E214  add r6, r25, r28
	ctx.r[6].u64 = ctx.r[25].u64 + ctx.r[28].u64;
	// 82D6C520: 5545083C  slwi r5, r10, 1
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C524: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D6C528: 7D4A2A14  add r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 82D6C52C: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C530: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D6C534: 7CAB2A14  add r5, r11, r5
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82D6C538: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82D6C53C: 7CFC4850  subf r7, r28, r9
	ctx.r[7].s64 = ctx.r[9].s64 - ctx.r[28].s64;
	// 82D6C540: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82D6C544: 7CC62214  add r6, r6, r4
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[4].u64;
	// 82D6C548: 54A52036  slwi r5, r5, 4
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C54C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D6C550: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C554: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 82D6C558: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82D6C55C: 7C0AFC2E  lfsx f0, r10, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C560: 7D4A2A14  add r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 82D6C564: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D6C568: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 82D6C56C: 409AFFE4  bne cr6, 0x82d6c550
	if !ctx.cr[6].eq {
	pc = 0x82D6C550; continue 'dispatch;
	}
	// 82D6C570: 3949FFFF  addi r10, r9, -1
	ctx.r[10].s64 = ctx.r[9].s64 + -1;
	// 82D6C574: 7CE99850  subf r7, r9, r19
	ctx.r[7].s64 = ctx.r[19].s64 - ctx.r[9].s64;
	// 82D6C578: 7D4A59D6  mullw r10, r10, r11
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82D6C57C: 7CCAD214  add r6, r10, r26
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[26].u64;
	// 82D6C580: 7D3F4B78  mr r31, r9
	ctx.r[31].u64 = ctx.r[9].u64;
	// 82D6C584: 2F070004  cmpwi cr6, r7, 4
	ctx.cr[6].compare_i32(ctx.r[7].s32, 4, &mut ctx.xer);
	// 82D6C588: 41980078  blt cr6, 0x82d6c600
	if ctx.cr[6].lt {
	pc = 0x82D6C600; continue 'dispatch;
	}
	// 82D6C58C: 7CE99850  subf r7, r9, r19
	ctx.r[7].s64 = ctx.r[19].s64 - ctx.r[9].s64;
	// 82D6C590: 54C5083C  slwi r5, r6, 1
	ctx.r[5].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C594: 38E7FFFC  addi r7, r7, -4
	ctx.r[7].s64 = ctx.r[7].s64 + -4;
	// 82D6C598: 7CA62A14  add r5, r6, r5
	ctx.r[5].u64 = ctx.r[6].u64 + ctx.r[5].u64;
	// 82D6C59C: 54E7F0BE  srwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shr(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D6C5A0: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C5A4: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82D6C5A8: 7FE54214  add r31, r5, r8
	ctx.r[31].u64 = ctx.r[5].u64 + ctx.r[8].u64;
	// 82D6C5AC: 54E5103A  slwi r5, r7, 2
	ctx.r[5].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C5B0: 3BDF0008  addi r30, r31, 8
	ctx.r[30].s64 = ctx.r[31].s64 + 8;
	// 82D6C5B4: 7FE54A14  add r31, r5, r9
	ctx.r[31].u64 = ctx.r[5].u64 + ctx.r[9].u64;
	// 82D6C5B8: 7EEABB78  mr r10, r23
	ctx.r[10].u64 = ctx.r[23].u64;
	// 82D6C5BC: 57C5103A  slwi r5, r30, 2
	ctx.r[5].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C5C0: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C5C4: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 82D6C5C8: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82D6C5CC: 7C05F42E  lfsx f0, r5, r30
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[30].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C5D0: D00AFFF8  stfs f0, -8(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82D6C5D4: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C5D8: 7C05F42E  lfsx f0, r5, r30
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[30].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C5DC: D00AFFFC  stfs f0, -4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82D6C5E0: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C5E4: 7C05F42E  lfsx f0, r5, r30
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[30].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C5E8: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D6C5EC: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C5F0: 7C05F42E  lfsx f0, r5, r30
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[30].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C5F4: D00A0004  stfs f0, 4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D6C5F8: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82D6C5FC: 409AFFC4  bne cr6, 0x82d6c5c0
	if !ctx.cr[6].eq {
	pc = 0x82D6C5C0; continue 'dispatch;
	}
	// 82D6C600: 7F1F9840  cmplw cr6, r31, r19
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[19].u32, &mut ctx.xer);
	// 82D6C604: 40980048  bge cr6, 0x82d6c64c
	if !ctx.cr[6].lt {
	pc = 0x82D6C64C; continue 'dispatch;
	}
	// 82D6C608: 54C5083C  slwi r5, r6, 1
	ctx.r[5].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6C60C: 7CF9FA14  add r7, r25, r31
	ctx.r[7].u64 = ctx.r[25].u64 + ctx.r[31].u64;
	// 82D6C610: 7CC62A14  add r6, r6, r5
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[5].u64;
	// 82D6C614: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82D6C618: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D6C61C: 7D5F9850  subf r10, r31, r19
	ctx.r[10].s64 = ctx.r[19].s64 - ctx.r[31].s64;
	// 82D6C620: 7CC64214  add r6, r6, r8
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[8].u64;
	// 82D6C624: 7CE72214  add r7, r7, r4
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[4].u64;
	// 82D6C628: 38C60008  addi r6, r6, 8
	ctx.r[6].s64 = ctx.r[6].s64 + 8;
	// 82D6C62C: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D6C630: 80A30000  lwz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C634: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D6C638: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D6C63C: 7C062C2E  lfsx f0, r6, r5
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C640: D0070000  stfs f0, 0(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D6C644: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 82D6C648: 409AFFE8  bne cr6, 0x82d6c630
	if !ctx.cr[6].eq {
	pc = 0x82D6C630; continue 'dispatch;
	}
	// 82D6C64C: 39CE0001  addi r14, r14, 1
	ctx.r[14].s64 = ctx.r[14].s64 + 1;
	// 82D6C650: 7F399A14  add r25, r25, r19
	ctx.r[25].u64 = ctx.r[25].u64 + ctx.r[19].u64;
	// 82D6C654: 7F16C214  add r24, r22, r24
	ctx.r[24].u64 = ctx.r[22].u64 + ctx.r[24].u64;
	// 82D6C658: 7EF6BA14  add r23, r22, r23
	ctx.r[23].u64 = ctx.r[22].u64 + ctx.r[23].u64;
	// 82D6C65C: 3AB5FFFF  addi r21, r21, -1
	ctx.r[21].s64 = ctx.r[21].s64 + -1;
	// 82D6C660: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82D6C664: 2F150006  cmpwi cr6, r21, 6
	ctx.cr[6].compare_i32(ctx.r[21].s32, 6, &mut ctx.xer);
	// 82D6C668: 4199FDB8  bgt cr6, 0x82d6c420
	if ctx.cr[6].gt {
	pc = 0x82D6C420; continue 'dispatch;
	}
	// 82D6C66C: 8141FF60  lwz r10, -0xa0(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-160 as u32) ) } as u64;
	// 82D6C670: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82D6C674: 3A31000C  addi r17, r17, 0xc
	ctx.r[17].s64 = ctx.r[17].s64 + 12;
	// 82D6C678: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82D6C67C: 3A10000C  addi r16, r16, 0xc
	ctx.r[16].s64 = ctx.r[16].s64 + 12;
	// 82D6C680: 7F1A5840  cmplw cr6, r26, r11
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82D6C684: 9141FF60  stw r10, -0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.r[10].u32 ) };
	// 82D6C688: 4198F850  blt cr6, 0x82d6bed8
	if ctx.cr[6].lt {
	pc = 0x82D6BED8; continue 'dispatch;
	}
	// 82D6C68C: 4BF3CD94  b 0x82ca9420
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6C690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D6C690 size=648
    let mut pc: u32 = 0x82D6C690;
    'dispatch: loop {
        match pc {
            0x82D6C690 => {
    //   block [0x82D6C690..0x82D6C918)
	// 82D6C690: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82D6C694: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82D6C698: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C69C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82D6C6A0: 80E30008  lwz r7, 8(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D6C6A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D6C6A8: 40990264  ble cr6, 0x82d6c90c
	if !ctx.cr[6].gt {
	pc = 0x82D6C90C; continue 'dispatch;
	}
	// 82D6C6AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82D6C6B0: 39650008  addi r11, r5, 8
	ctx.r[11].s64 = ctx.r[5].s64 + 8;
	// 82D6C6B4: 38AA4C80  addi r5, r10, 0x4c80
	ctx.r[5].s64 = ctx.r[10].s64 + 19584;
	// 82D6C6B8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82D6C6BC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82D6C6C0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82D6C6C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82D6C6C8: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 82D6C6CC: C1080C18  lfs f8, 0xc18(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3096 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82D6C6D0: C1490C14  lfs f10, 0xc14(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3092 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82D6C6D4: C12A0C4C  lfs f9, 0xc4c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3148 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82D6C6D8: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6C6DC: 7D0A322E  lhzx r8, r10, r6
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82D6C6E0: 550A07BE  clrlwi r10, r8, 0x1e
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x00000003u64;
	// 82D6C6E4: 5509D1BE  srwi r9, r8, 6
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shr(6);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D6C6E8: 2B0A0002  cmplwi cr6, r10, 2
	ctx.cr[6].compare_u32(ctx.r[10].u32, 2 as u32, &mut ctx.xer);
	// 82D6C6EC: 419A0078  beq cr6, 0x82d6c764
	if ctx.cr[6].eq {
	pc = 0x82D6C764; continue 'dispatch;
	}
	// 82D6C6F0: 552A077A  rlwinm r10, r9, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82D6C6F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D6C6F8: 419A0010  beq cr6, 0x82d6c708
	if ctx.cr[6].eq {
	pc = 0x82D6C708; continue 'dispatch;
	}
	// 82D6C6FC: C0040000  lfs f0, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C700: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 82D6C704: 4800000C  b 0x82d6c710
	pc = 0x82D6C710; continue 'dispatch;
	// 82D6C708: C0070000  lfs f0, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C70C: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 82D6C710: 552A07BC  rlwinm r10, r9, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82D6C714: D00BFFF8  stfs f0, -8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82D6C718: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D6C71C: 419A0010  beq cr6, 0x82d6c72c
	if ctx.cr[6].eq {
	pc = 0x82D6C72C; continue 'dispatch;
	}
	// 82D6C720: C0040000  lfs f0, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C724: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 82D6C728: 4800000C  b 0x82d6c734
	pc = 0x82D6C734; continue 'dispatch;
	// 82D6C72C: C0070000  lfs f0, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C730: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 82D6C734: 552A07FE  clrlwi r10, r9, 0x1f
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 82D6C738: D00BFFFC  stfs f0, -4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82D6C73C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D6C740: 419A0014  beq cr6, 0x82d6c754
	if ctx.cr[6].eq {
	pc = 0x82D6C754; continue 'dispatch;
	}
	// 82D6C744: C0040000  lfs f0, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C748: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 82D6C74C: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D6C750: 48000024  b 0x82d6c774
	pc = 0x82D6C774; continue 'dispatch;
	// 82D6C754: C0070000  lfs f0, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6C758: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 82D6C75C: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D6C760: 48000014  b 0x82d6c774
	pc = 0x82D6C774; continue 'dispatch;
	// 82D6C764: 394BFFF8  addi r10, r11, -8
	ctx.r[10].s64 = ctx.r[11].s64 + -8;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6C918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D6C918 size=4824
    let mut pc: u32 = 0x82D6C918;
    'dispatch: loop {
        match pc {
            0x82D6C918 => {
    //   block [0x82D6C918..0x82D6DBF0)
	// 82D6C918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6C91C: 4BF3CAB5  bl 0x82ca93d0
	ctx.lr = 0x82D6C920;
	sub_82CA93D0(ctx, base);
	// 82D6C920: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 82D6C924: 4BF413B1  bl 0x82cadcd4
	ctx.lr = 0x82D6C928;
	sub_82CADCA0(ctx, base);
	// 82D6C928: 9421FA40  stwu r1, -0x5c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-1472 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6C92C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82D6C930: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C934: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82D6C938: 90A105E4  stw r5, 0x5e4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1508 as u32), ctx.r[5].u32 ) };
	// 82D6C93C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82D6C940: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D6C944: 3FC08000  lis r30, -0x8000
	ctx.r[30].s64 = -2147483648;
	// 82D6C948: 83FA0004  lwz r31, 4(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6C94C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82D6C950: 3B1A0004  addi r24, r26, 4
	ctx.r[24].s64 = ctx.r[26].s64 + 4;
	// 82D6C954: 57EA083C  slwi r10, r31, 1
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D6C958: 93610060  stw r27, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[27].u32 ) };
	// 82D6C95C: 91610088  stw r11, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 82D6C960: 7D3F5214  add r9, r31, r10
	ctx.r[9].u64 = ctx.r[31].u64 + ctx.r[10].u64;
	// 82D6C964: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C968: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82D6C96C: 55292036  slwi r9, r9, 4
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D6C970: 93C10068  stw r30, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 82D6C974: 93A105DC  stw r29, 0x5dc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1500 as u32), ctx.r[29].u32 ) };
	// 82D6C978: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 82D6C97C: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D6C980: 55240036  rlwinm r4, r9, 0, 0, 0x1b
	ctx.r[4].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82D6C984: 810B002C  lwz r8, 0x2c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D6C988: 7D2A2214  add r9, r10, r4
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82D6C98C: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82D6C990: 4199000C  bgt cr6, 0x82d6c99c
	if ctx.cr[6].gt {
	pc = 0x82D6C99C; continue 'dispatch;
	}
	// 82D6C994: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82D6C998: 4800001C  b 0x82d6c9b4
	pc = 0x82D6C9B4; continue 'dispatch;
	// 82D6C99C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C9A0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82D6C9A4: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D6C9A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D6C9AC: 4E800421  bctrl
	ctx.lr = 0x82D6C9B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D6C9B0: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82D6C9B4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82D6C9B8: 83D80000  lwz r30, 0(r24)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6C9BC: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82D6C9C0: 7FEB5B78  or r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 | ctx.r[11].u64;
	// 82D6C9C4: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 82D6C9C8: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82D6C9CC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D6C9D0: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82D6C9D4: 40980024  bge cr6, 0x82d6c9f8
	if !ctx.cr[6].lt {
	pc = 0x82D6C9F8; continue 'dispatch;
	}
	// 82D6C9D8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D6C9DC: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D6C9E0: 41980008  blt cr6, 0x82d6c9e8
	if ctx.cr[6].lt {
	pc = 0x82D6C9E8; continue 'dispatch;
	}
	// 82D6C9E4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82D6C9E8: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 82D6C9EC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D6C9F0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D6C9F4: 4BFEA51D  bl 0x82d56f10
	ctx.lr = 0x82D6C9F8;
	sub_82D56F10(ctx, base);
	// 82D6C9F8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D6C9FC: 80D80000  lwz r6, 0(r24)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6CA00: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82D6CA04: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82D6CA08: C38B0C64  lfs f28, 0xc64(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3172 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 82D6CA0C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D6CA10: D3810090  stfs f28, 0x90(r1)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 82D6CA14: C36B0BE4  lfs f27, 0xbe4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3044 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 82D6CA18: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
	// 82D6CA1C: D3610100  stfs f27, 0x100(r1)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(256 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6DBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D6DBF0 size=788
    let mut pc: u32 = 0x82D6DBF0;
    'dispatch: loop {
        match pc {
            0x82D6DBF0 => {
    //   block [0x82D6DBF0..0x82D6DF04)
	// 82D6DBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6DBF4: 4BF3B811  bl 0x82ca9404
	ctx.lr = 0x82D6DBF8;
	sub_82CA93D0(ctx, base);
	// 82D6DBF8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6DBFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D6DC00: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82D6DC04: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82D6DC08: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82D6DC0C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6DC10: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6DC14: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82D6DC18: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82D6DC1C: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82D6DC20: 93C10068  stw r30, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 82D6DC24: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D6DC28: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82D6DC2C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D6DC30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82D6DC34: 93C1006C  stw r30, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 82D6DC38: 93C10074  stw r30, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 82D6DC3C: 93C10078  stw r30, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[30].u32 ) };
	// 82D6DC40: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82D6DC44: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82D6DC48: 93C10080  stw r30, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[30].u32 ) };
	// 82D6DC4C: 93C10084  stw r30, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82D6DC50: 93C1008C  stw r30, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[30].u32 ) };
	// 82D6DC54: 93C10090  stw r30, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[30].u32 ) };
	// 82D6DC58: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82D6DC5C: 91410070  stw r10, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 82D6DC60: 9141007C  stw r10, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 82D6DC64: 91410088  stw r10, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82D6DC68: 91410094  stw r10, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 82D6DC6C: 4099001C  ble cr6, 0x82d6dc88
	if !ctx.cr[6].gt {
	pc = 0x82D6DC88; continue 'dispatch;
	}
	// 82D6DC70: 40980008  bge cr6, 0x82d6dc78
	if !ctx.cr[6].lt {
	pc = 0x82D6DC78; continue 'dispatch;
	}
	// 82D6DC74: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82D6DC78: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82D6DC7C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D6DC80: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82D6DC84: 4BFE928D  bl 0x82d56f10
	ctx.lr = 0x82D6DC88;
	sub_82D56F10(ctx, base);
	// 82D6DC88: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D6DC8C: 93A10060  stw r29, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 82D6DC90: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D6DC94: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82D6DC98: 40980020  bge cr6, 0x82d6dcb8
	if !ctx.cr[6].lt {
	pc = 0x82D6DCB8; continue 'dispatch;
	}
	// 82D6DC9C: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82D6DCA0: 7F1D2000  cmpw cr6, r29, r4
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82D6DCA4: 41980008  blt cr6, 0x82d6dcac
	if ctx.cr[6].lt {
	pc = 0x82D6DCAC; continue 'dispatch;
	}
	// 82D6DCA8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D6DCAC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82D6DCB0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82D6DCB4: 4BFE925D  bl 0x82d56f10
	ctx.lr = 0x82D6DCB8;
	sub_82D56F10(ctx, base);
	// 82D6DCB8: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82D6DCBC: 93A1006C  stw r29, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[29].u32 ) };
	// 82D6DCC0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D6DCC4: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82D6DCC8: 40980020  bge cr6, 0x82d6dce8
	if !ctx.cr[6].lt {
	pc = 0x82D6DCE8; continue 'dispatch;
	}
	// 82D6DCCC: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82D6DCD0: 7F1D2000  cmpw cr6, r29, r4
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82D6DCD4: 41980008  blt cr6, 0x82d6dcdc
	if ctx.cr[6].lt {
	pc = 0x82D6DCDC; continue 'dispatch;
	}
	// 82D6DCD8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D6DCDC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82D6DCE0: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 82D6DCE4: 4BFE922D  bl 0x82d56f10
	ctx.lr = 0x82D6DCE8;
	sub_82D56F10(ctx, base);
	// 82D6DCE8: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82D6DCEC: 93A10078  stw r29, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[29].u32 ) };
	// 82D6DCF0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D6DCF4: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82D6DCF8: 40980024  bge cr6, 0x82d6dd1c
	if !ctx.cr[6].lt {
	pc = 0x82D6DD1C; continue 'dispatch;
	}
	// 82D6DCFC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D6DD00: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D6DD04: 41980008  blt cr6, 0x82d6dd0c
	if ctx.cr[6].lt {
	pc = 0x82D6DD0C; continue 'dispatch;
	}
	// 82D6DD08: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82D6DD0C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82D6DD10: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D6DD14: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82D6DD18: 4BFE91F9  bl 0x82d56f10
	ctx.lr = 0x82D6DD1C;
	sub_82D56F10(ctx, base);
	// 82D6DD1C: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82D6DD20: 93A10084  stw r29, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[29].u32 ) };
	// 82D6DD24: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D6DD28: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82D6DD2C: 40980024  bge cr6, 0x82d6dd50
	if !ctx.cr[6].lt {
	pc = 0x82D6DD50; continue 'dispatch;
	}
	// 82D6DD30: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D6DD34: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D6DD38: 41980008  blt cr6, 0x82d6dd40
	if ctx.cr[6].lt {
	pc = 0x82D6DD40; continue 'dispatch;
	}
	// 82D6DD3C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82D6DD40: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D6DD44: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D6DD48: 3861008C  addi r3, r1, 0x8c
	ctx.r[3].s64 = ctx.r[1].s64 + 140;
	// 82D6DD4C: 4BFE91C5  bl 0x82d56f10
	ctx.lr = 0x82D6DD50;
	sub_82D56F10(ctx, base);
	// 82D6DD50: 93A10090  stw r29, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[29].u32 ) };
	// 82D6DD54: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82D6DD58: 2F1D0004  cmpwi cr6, r29, 4
	ctx.cr[6].compare_i32(ctx.r[29].s32, 4, &mut ctx.xer);
	// 82D6DD5C: 41980130  blt cr6, 0x82d6de8c
	if ctx.cr[6].lt {
	pc = 0x82D6DE8C; continue 'dispatch;
	}
	// 82D6DD60: 38FDFFFD  addi r7, r29, -3
	ctx.r[7].s64 = ctx.r[29].s64 + -3;
	// 82D6DD64: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82D6DD68: 8101005C  lwz r8, 0x5c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82D6DD6C: C01F000C  lfs f0, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6DD70: 392B000C  addi r9, r11, 0xc
	ctx.r[9].s64 = ctx.r[11].s64 + 12;
	// 82D6DD74: 7C0B452E  stfsx f0, r11, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 82D6DD78: 81010068  lwz r8, 0x68(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82D6DD7C: C01F0010  lfs f0, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6DD80: 7C0B452E  stfsx f0, r11, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 82D6DD84: 81010074  lwz r8, 0x74(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82D6DD88: C01F0014  lfs f0, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6DD8C: 7C0B452E  stfsx f0, r11, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 82D6DD90: 81010080  lwz r8, 0x80(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82D6DD94: C01F0018  lfs f0, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6DD98: 7C0B452E  stfsx f0, r11, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 82D6DD9C: 891F001C  lbz r8, 0x1c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D6DDA0: 80C1008C  lwz r6, 0x8c(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82D6DDA4: 7D0651AE  stbx r8, r6, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u8) };
	// 82D6DDA8: C01F000C  lfs f0, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6DDAC: 8101005C  lwz r8, 0x5c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82D6DDB0: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82D6DDB4: D0080004  stfs f0, 4(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D6DDB8: 81010068  lwz r8, 0x68(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82D6DDBC: C01F0010  lfs f0, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6DDC0: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82D6DDC4: D0080004  stfs f0, 4(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D6DDC8: 81010074  lwz r8, 0x74(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82D6DDCC: C01F0014  lfs f0, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6DDD0: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82D6DDD4: D0080004  stfs f0, 4(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D6DDD8: 81010080  lwz r8, 0x80(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82D6DDDC: C01F0018  lfs f0, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6DDE0: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82D6DDE4: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82D6DDE8: D0080004  stfs f0, 4(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D6DDEC: 8101008C  lwz r8, 0x8c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82D6DDF0: 88DF001C  lbz r6, 0x1c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D6DDF4: 7D085214  add r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82D6DDF8: 98C80001  stb r6, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[6].u8 ) };
	// 82D6DDFC: 3909FFFC  addi r8, r9, -4
	ctx.r[8].s64 = ctx.r[9].s64 + -4;
	// 82D6DE00: 80C1005C  lwz r6, 0x5c(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82D6DE04: C01F000C  lfs f0, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6DE08: 7C08352E  stfsx f0, r8, r6
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 82D6DE0C: 80C10068  lwz r6, 0x68(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82D6DE10: C01F0010  lfs f0, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6DE14: 7C08352E  stfsx f0, r8, r6
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 82D6DE18: 80C10074  lwz r6, 0x74(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82D6DE1C: C01F0014  lfs f0, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6DE20: 7C08352E  stfsx f0, r8, r6
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 82D6DE24: 80C10080  lwz r6, 0x80(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82D6DE28: C01F0018  lfs f0, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6DE2C: 7C08352E  stfsx f0, r8, r6
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 82D6DE30: 8101008C  lwz r8, 0x8c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82D6DE34: 88DF001C  lbz r6, 0x1c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D6DE38: 7D085214  add r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82D6DE3C: 98C80002  stb r6, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[6].u8 ) };
	// 82D6DE40: C01F000C  lfs f0, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6DE44: 8101005C  lwz r8, 0x5c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82D6DE48: 7C09452E  stfsx f0, r9, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 82D6DE4C: 81010068  lwz r8, 0x68(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82D6DE50: C01F0010  lfs f0, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6DE54: 7C09452E  stfsx f0, r9, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 82D6DE58: 81010074  lwz r8, 0x74(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82D6DE5C: C01F0014  lfs f0, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6DE60: 7C09452E  stfsx f0, r9, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 82D6DE64: 81010080  lwz r8, 0x80(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82D6DE68: C01F0018  lfs f0, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6DE6C: 7C09452E  stfsx f0, r9, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 82D6DE70: 8121008C  lwz r9, 0x8c(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82D6DE74: 891F001C  lbz r8, 0x1c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D6DE78: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82D6DE7C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82D6DE80: 7F0A3800  cmpw cr6, r10, r7
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82D6DE84: 99090003  stb r8, 3(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(3 as u32), ctx.r[8].u8 ) };
	// 82D6DE88: 4198FEE0  blt cr6, 0x82d6dd68
	if ctx.cr[6].lt {
	pc = 0x82D6DD68; continue 'dispatch;
	}
	// 82D6DE8C: 7F0AE800  cmpw cr6, r10, r29
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82D6DE90: 40980054  bge cr6, 0x82d6dee4
	if !ctx.cr[6].lt {
	pc = 0x82D6DEE4; continue 'dispatch;
	}
	// 82D6DE94: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D6DE98: 8121005C  lwz r9, 0x5c(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82D6DE9C: C01F000C  lfs f0, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6DEA0: 7C0B4D2E  stfsx f0, r11, r9
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), tmp.u32) };
	// 82D6DEA4: 81210068  lwz r9, 0x68(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82D6DEA8: C01F0010  lfs f0, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6DEAC: 7C0B4D2E  stfsx f0, r11, r9
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), tmp.u32) };
	// 82D6DEB0: 81210074  lwz r9, 0x74(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82D6DEB4: C01F0014  lfs f0, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6DEB8: 7C0B4D2E  stfsx f0, r11, r9
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), tmp.u32) };
	// 82D6DEBC: 81210080  lwz r9, 0x80(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82D6DEC0: C01F0018  lfs f0, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D6DEC4: 7C0B4D2E  stfsx f0, r11, r9
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), tmp.u32) };
	// 82D6DEC8: 893F001C  lbz r9, 0x1c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D6DECC: 8101008C  lwz r8, 0x8c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82D6DED0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D6DED4: 7D2851AE  stbx r9, r8, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u8) };
	// 82D6DED8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D6DEDC: 7F0AE800  cmpw cr6, r10, r29
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82D6DEE0: 4198FFB8  blt cr6, 0x82d6de98
	if ctx.cr[6].lt {
	pc = 0x82D6DE98; continue 'dispatch;
	}
	// 82D6DEE4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82D6DEE8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82D6DEEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D6DEF0: 4BFFEA29  bl 0x82d6c918
	ctx.lr = 0x82D6DEF4;
	sub_82D6C918(ctx, base);
	// 82D6DEF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D6DEF8: 48000011  bl 0x82d6df08
	ctx.lr = 0x82D6DEFC;
	sub_82D6DF08(ctx, base);
	// 82D6DEFC: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82D6DF00: 4BF3B554  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6DF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D6DF08 size=260
    let mut pc: u32 = 0x82D6DF08;
    'dispatch: loop {
        match pc {
            0x82D6DF08 => {
    //   block [0x82D6DF08..0x82D6E00C)
	// 82D6DF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6DF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D6DF10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D6DF14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6DF18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D6DF1C: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82D6DF20: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D6DF24: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D6DF28: 409A0020  bne cr6, 0x82d6df48
	if !ctx.cr[6].eq {
	pc = 0x82D6DF48; continue 'dispatch;
	}
	// 82D6DF2C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6DF30: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D6DF34: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D6DF38: 809F003C  lwz r4, 0x3c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82D6DF3C: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D6DF40: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D6DF44: 4BFE7385  bl 0x82d552c8
	ctx.lr = 0x82D6DF48;
	sub_82D552C8(ctx, base);
	// 82D6DF48: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82D6DF4C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D6DF50: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D6DF54: 409A0020  bne cr6, 0x82d6df74
	if !ctx.cr[6].eq {
	pc = 0x82D6DF74; continue 'dispatch;
	}
	// 82D6DF58: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6DF5C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D6DF60: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D6DF64: 809F0030  lwz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82D6DF68: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6DF6C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D6DF70: 4BFE7359  bl 0x82d552c8
	ctx.lr = 0x82D6DF74;
	sub_82D552C8(ctx, base);
	// 82D6DF74: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D6DF78: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D6DF7C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D6DF80: 409A0020  bne cr6, 0x82d6dfa0
	if !ctx.cr[6].eq {
	pc = 0x82D6DFA0; continue 'dispatch;
	}
	// 82D6DF84: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6DF88: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D6DF8C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D6DF90: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6DF94: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6DF98: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D6DF9C: 4BFE732D  bl 0x82d552c8
	ctx.lr = 0x82D6DFA0;
	sub_82D552C8(ctx, base);
	// 82D6DFA0: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D6DFA4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D6DFA8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D6DFAC: 409A0020  bne cr6, 0x82d6dfcc
	if !ctx.cr[6].eq {
	pc = 0x82D6DFCC; continue 'dispatch;
	}
	// 82D6DFB0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6DFB4: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D6DFB8: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D6DFBC: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D6DFC0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6DFC4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D6DFC8: 4BFE7301  bl 0x82d552c8
	ctx.lr = 0x82D6DFCC;
	sub_82D552C8(ctx, base);
	// 82D6DFCC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D6DFD0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D6DFD4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D6DFD8: 409A0020  bne cr6, 0x82d6dff8
	if !ctx.cr[6].eq {
	pc = 0x82D6DFF8; continue 'dispatch;
	}
	// 82D6DFDC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6DFE0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D6DFE4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D6DFE8: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6DFEC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D6DFF0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D6DFF4: 4BFE72D5  bl 0x82d552c8
	ctx.lr = 0x82D6DFF8;
	sub_82D552C8(ctx, base);
	// 82D6DFF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D6DFFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D6E000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D6E004: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D6E008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6E010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D6E010 size=12
    let mut pc: u32 = 0x82D6E010;
    'dispatch: loop {
        match pc {
            0x82D6E010 => {
    //   block [0x82D6E010..0x82D6E01C)
	// 82D6E010: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82D6E014: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D6E018: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6E01C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D6E01C size=36
    let mut pc: u32 = 0x82D6E01C;
    'dispatch: loop {
        match pc {
            0x82D6E01C => {
    //   block [0x82D6E01C..0x82D6E040)
	// 82D6E01C: 3963002C  addi r11, r3, 0x2c
	ctx.r[11].s64 = ctx.r[3].s64 + 44;
	// 82D6E020: 812BFFFC  lwz r9, -4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82D6E024: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D6E028: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D6E02C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D6E030: 912BFFF8  stw r9, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[9].u32 ) };
	// 82D6E034: 396B0030  addi r11, r11, 0x30
	ctx.r[11].s64 = ctx.r[11].s64 + 48;
	// 82D6E038: 409AFFE8  bne cr6, 0x82d6e020
	if !ctx.cr[6].eq {
	pc = 0x82D6E020; continue 'dispatch;
	}
	// 82D6E03C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6E040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D6E040 size=16
    let mut pc: u32 = 0x82D6E040;
    'dispatch: loop {
        match pc {
            0x82D6E040 => {
    //   block [0x82D6E040..0x82D6E050)
	// 82D6E040: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82D6E044: 409A000C  bne cr6, 0x82d6e050
	if !ctx.cr[6].eq {
		sub_82D6E050(ctx, base);
		return;
	}
	// 82D6E048: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D6E04C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6E050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D6E050 size=96
    let mut pc: u32 = 0x82D6E050;
    'dispatch: loop {
        match pc {
            0x82D6E050 => {
    //   block [0x82D6E050..0x82D6E0B0)
	// 82D6E050: 7C8B1670  srawi r11, r4, 2
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[4].s32 >> 2) as i64;
	// 82D6E054: 0CC50000  twi 6, r5, 0
	// 82D6E058: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82D6E05C: 7D2B2BD6  divw r9, r11, r5
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[5].s32;
	// 82D6E060: 556A083E  rotlwi r10, r11, 1
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(1)) as u64;
	// 82D6E064: 7D2929D6  mullw r9, r9, r5
	ctx.r[9].s64 = (ctx.r[9].s32 as i64) * (ctx.r[5].s32 as i64);
	// 82D6E068: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D6E06C: 7D295850  subf r9, r9, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82D6E070: 7CAA5078  andc r10, r5, r10
	ctx.r[10].u64 = ctx.r[5].u64 & !ctx.r[10].u64;
	// 82D6E074: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82D6E078: 0CAAFFFF  twi 5, r10, -1
	// 82D6E07C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D6E080: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82D6E084: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6E088: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D6E08C: 4098001C  bge cr6, 0x82d6e0a8
	if !ctx.cr[6].lt {
	pc = 0x82D6E0A8; continue 'dispatch;
	}
	// 82D6E090: 54A9103A  slwi r9, r5, 2
	ctx.r[9].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D6E094: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82D6E098: 7D655850  subf r11, r5, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 82D6E09C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6E0A0: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82D6E0A4: 4198FFF0  blt cr6, 0x82d6e094
	if ctx.cr[6].lt {
	pc = 0x82D6E094; continue 'dispatch;
	}
	// 82D6E0A8: 7C6B2A14  add r3, r11, r5
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82D6E0AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6E0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D6E0B0 size=20
    let mut pc: u32 = 0x82D6E0B0;
    'dispatch: loop {
        match pc {
            0x82D6E0B0 => {
    //   block [0x82D6E0B0..0x82D6E0C4)
	// 82D6E0B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D6E0B4: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 82D6E0B8: 894B0028  lbz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D6E0BC: 2B0A00FF  cmplwi cr6, r10, 0xff
	ctx.cr[6].compare_u32(ctx.r[10].u32, 255 as u32, &mut ctx.xer);
	// 82D6E0C0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6E0C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D6E0C4 size=24
    let mut pc: u32 = 0x82D6E0C4;
    'dispatch: loop {
        match pc {
            0x82D6E0C4 => {
    //   block [0x82D6E0C4..0x82D6E0DC)
	// 82D6E0C4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6E0C8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82D6E0CC: 409A0010  bne cr6, 0x82d6e0dc
	if !ctx.cr[6].eq {
		sub_82D6E0DC(ctx, base);
		return;
	}
	// 82D6E0D0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D6E0D4: 386B68C0  addi r3, r11, 0x68c0
	ctx.r[3].s64 = ctx.r[11].s64 + 26816;
	// 82D6E0D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6E0DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D6E0DC size=20
    let mut pc: u32 = 0x82D6E0DC;
    'dispatch: loop {
        match pc {
            0x82D6E0DC => {
    //   block [0x82D6E0DC..0x82D6E0F0)
	// 82D6E0DC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82D6E0E0: 409A0010  bne cr6, 0x82d6e0f0
	if !ctx.cr[6].eq {
		sub_82D6E0F0(ctx, base);
		return;
	}
	// 82D6E0E4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D6E0E8: 386B68B4  addi r3, r11, 0x68b4
	ctx.r[3].s64 = ctx.r[11].s64 + 26804;
	// 82D6E0EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6E0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D6E0F0 size=8
    let mut pc: u32 = 0x82D6E0F0;
    'dispatch: loop {
        match pc {
            0x82D6E0F0 => {
    //   block [0x82D6E0F0..0x82D6E0F8)
	// 82D6E0F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D6E0F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6E0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D6E0F8 size=148
    let mut pc: u32 = 0x82D6E0F8;
    'dispatch: loop {
        match pc {
            0x82D6E0F8 => {
    //   block [0x82D6E0F8..0x82D6E18C)
	// 82D6E0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6E0FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D6E100: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D6E104: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6E108: 4BFE7649  bl 0x82d55750
	ctx.lr = 0x82D6E10C;
	sub_82D55750(ctx, base);
	// 82D6E10C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D6E110: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D6E114: 388B3CD8  addi r4, r11, 0x3cd8
	ctx.r[4].s64 = ctx.r[11].s64 + 15576;
	// 82D6E118: 4BFEA8C1  bl 0x82d589d8
	ctx.lr = 0x82D6E11C;
	sub_82D589D8(ctx, base);
	// 82D6E11C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D6E120: 419A0050  beq cr6, 0x82d6e170
	if ctx.cr[6].eq {
	pc = 0x82D6E170; continue 'dispatch;
	}
	// 82D6E124: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D6E128: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D6E12C: 388B41B4  addi r4, r11, 0x41b4
	ctx.r[4].s64 = ctx.r[11].s64 + 16820;
	// 82D6E130: 4BFEA8A9  bl 0x82d589d8
	ctx.lr = 0x82D6E134;
	sub_82D589D8(ctx, base);
	// 82D6E134: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D6E138: 419A0038  beq cr6, 0x82d6e170
	if ctx.cr[6].eq {
	pc = 0x82D6E170; continue 'dispatch;
	}
	// 82D6E13C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D6E140: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D6E144: 388B3D58  addi r4, r11, 0x3d58
	ctx.r[4].s64 = ctx.r[11].s64 + 15704;
	// 82D6E148: 4BFEA891  bl 0x82d589d8
	ctx.lr = 0x82D6E14C;
	sub_82D589D8(ctx, base);
	// 82D6E14C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D6E150: 419A0020  beq cr6, 0x82d6e170
	if ctx.cr[6].eq {
	pc = 0x82D6E170; continue 'dispatch;
	}
	// 82D6E154: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D6E158: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D6E15C: 388B3D48  addi r4, r11, 0x3d48
	ctx.r[4].s64 = ctx.r[11].s64 + 15688;
	// 82D6E160: 4BFEA879  bl 0x82d589d8
	ctx.lr = 0x82D6E164;
	sub_82D589D8(ctx, base);
	// 82D6E164: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D6E168: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D6E16C: 409A0008  bne cr6, 0x82d6e174
	if !ctx.cr[6].eq {
	pc = 0x82D6E174; continue 'dispatch;
	}
	// 82D6E170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D6E174: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82D6E178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D6E17C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D6E180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D6E184: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D6E188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6E190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D6E190 size=8
    let mut pc: u32 = 0x82D6E190;
    'dispatch: loop {
        match pc {
            0x82D6E190 => {
    //   block [0x82D6E190..0x82D6E198)
	// 82D6E190: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D6E194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6E198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D6E198 size=8
    let mut pc: u32 = 0x82D6E198;
    'dispatch: loop {
        match pc {
            0x82D6E198 => {
    //   block [0x82D6E198..0x82D6E1A0)
	// 82D6E198: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D6E19C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6E1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D6E1A0 size=12
    let mut pc: u32 = 0x82D6E1A0;
    'dispatch: loop {
        match pc {
            0x82D6E1A0 => {
    //   block [0x82D6E1A0..0x82D6E1AC)
	// 82D6E1A0: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D6E1A4: 806B0014  lwz r3, 0x14(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D6E1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6E1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D6E1B0 size=24
    let mut pc: u32 = 0x82D6E1B0;
    'dispatch: loop {
        match pc {
            0x82D6E1B0 => {
    //   block [0x82D6E1B0..0x82D6E1C8)
	// 82D6E1B0: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D6E1B4: 81430020  lwz r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D6E1B8: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82D6E1BC: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D6E1C0: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D6E1C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6E1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D6E1C8 size=116
    let mut pc: u32 = 0x82D6E1C8;
    'dispatch: loop {
        match pc {
            0x82D6E1C8 => {
    //   block [0x82D6E1C8..0x82D6E23C)
	// 82D6E1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6E1CC: 4BF3B23D  bl 0x82ca9408
	ctx.lr = 0x82D6E1D0;
	sub_82CA93D0(ctx, base);
	// 82D6E1D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6E1D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D6E1D8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82D6E1DC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82D6E1E0: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D6E1E4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D6E1E8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D6E1EC: 40990038  ble cr6, 0x82d6e224
	if !ctx.cr[6].gt {
	pc = 0x82D6E224; continue 'dispatch;
	}
	// 82D6E1F0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82D6E1F4: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D6E1F8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82D6E1FC: 7C7F5A14  add r3, r31, r11
	ctx.r[3].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82D6E200: 4BFEA7D9  bl 0x82d589d8
	ctx.lr = 0x82D6E204;
	sub_82D589D8(ctx, base);
	// 82D6E204: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D6E208: 419A0028  beq cr6, 0x82d6e230
	if ctx.cr[6].eq {
	pc = 0x82D6E230; continue 'dispatch;
	}
	// 82D6E20C: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D6E210: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82D6E214: 3BFF0030  addi r31, r31, 0x30
	ctx.r[31].s64 = ctx.r[31].s64 + 48;
	// 82D6E218: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D6E21C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D6E220: 4198FFD4  blt cr6, 0x82d6e1f4
	if ctx.cr[6].lt {
	pc = 0x82D6E1F4; continue 'dispatch;
	}
	// 82D6E224: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82D6E228: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D6E22C: 4BF3B22C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82D6E230: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D6E234: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D6E238: 4BF3B220  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6E240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D6E240 size=140
    let mut pc: u32 = 0x82D6E240;
    'dispatch: loop {
        match pc {
            0x82D6E240 => {
    //   block [0x82D6E240..0x82D6E2CC)
	// 82D6E240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6E244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D6E248: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D6E24C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D6E250: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6E254: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D6E258: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D6E25C: 808B0018  lwz r4, 0x18(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D6E260: 80AB001C  lwz r5, 0x1c(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D6E264: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82D6E268: 41980018  blt cr6, 0x82d6e280
	if ctx.cr[6].lt {
	pc = 0x82D6E280; continue 'dispatch;
	}
	// 82D6E26C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82D6E270: 41980010  blt cr6, 0x82d6e280
	if ctx.cr[6].lt {
	pc = 0x82D6E280; continue 'dispatch;
	}
	// 82D6E274: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6E278: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82D6E27C: 4800002C  b 0x82d6e2a8
	pc = 0x82D6E2A8; continue 'dispatch;
	// 82D6E280: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6E284: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D6E288: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D6E28C: 388B68CC  addi r4, r11, 0x68cc
	ctx.r[4].s64 = ctx.r[11].s64 + 26828;
	// 82D6E290: 817E0034  lwz r11, 0x34(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82D6E294: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D6E298: 4E800421  bctrl
	ctx.lr = 0x82D6E29C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D6E29C: 817E0038  lwz r11, 0x38(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82D6E2A0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82D6E2A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82D6E2A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D6E2AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D6E2B0: 4E800421  bctrl
	ctx.lr = 0x82D6E2B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D6E2B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D6E2B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D6E2BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D6E2C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D6E2C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D6E2C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6E2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D6E2D0 size=44
    let mut pc: u32 = 0x82D6E2D0;
    'dispatch: loop {
        match pc {
            0x82D6E2D0 => {
    //   block [0x82D6E2D0..0x82D6E2FC)
	// 82D6E2D0: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D6E2D4: 808B0020  lwz r4, 0x20(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D6E2D8: 80AB0024  lwz r5, 0x24(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6E2DC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82D6E2E0: 4198001C  blt cr6, 0x82d6e2fc
	if ctx.cr[6].lt {
		sub_82D6E2FC(ctx, base);
		return;
	}
	// 82D6E2E4: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82D6E2E8: 41980014  blt cr6, 0x82d6e2fc
	if ctx.cr[6].lt {
		sub_82D6E2FC(ctx, base);
		return;
	}
	// 82D6E2EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6E2F0: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82D6E2F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D6E2F8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6E2FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D6E2FC size=8
    let mut pc: u32 = 0x82D6E2FC;
    'dispatch: loop {
        match pc {
            0x82D6E2FC => {
    //   block [0x82D6E2FC..0x82D6E304)
	// 82D6E2FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D6E300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6E308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D6E308 size=312
    let mut pc: u32 = 0x82D6E308;
    'dispatch: loop {
        match pc {
            0x82D6E308 => {
    //   block [0x82D6E308..0x82D6E440)
	// 82D6E308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D6E30C: 4BF3B0F5  bl 0x82ca9400
	ctx.lr = 0x82D6E310;
	sub_82CA93D0(ctx, base);
	// 82D6E310: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D6E314: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82D6E318: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D6E31C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82D6E320: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6E324: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D6E328: 4099010C  ble cr6, 0x82d6e434
	if !ctx.cr[6].gt {
	pc = 0x82D6E434; continue 'dispatch;
	}
	// 82D6E32C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82D6E330: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6E334: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6E338: 7FDB5A14  add r30, r27, r11
	ctx.r[30].u64 = ctx.r[27].u64 + ctx.r[11].u64;
	// 82D6E33C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6E340: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6E344: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D6E348: 83A50000  lwz r29, 0(r5)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6E34C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D6E350: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D6E354: 4E800421  bctrl
	ctx.lr = 0x82D6E358;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D6E358: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82D6E35C: 419A0080  beq cr6, 0x82d6e3dc
	if ctx.cr[6].eq {
	pc = 0x82D6E3DC; continue 'dispatch;
	}
	// 82D6E360: 80DF0014  lwz r6, 0x14(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D6E364: 80BF0010  lwz r5, 0x10(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6E368: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6E36C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6E370: 48004EA1  bl 0x82d73210
	ctx.lr = 0x82D6E374;
	sub_82D73210(ctx, base);
	// 82D6E374: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D6E378: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6E37C: 4BFE7775  bl 0x82d55af0
	ctx.lr = 0x82D6E380;
	sub_82D55AF0(ctx, base);
	// 82D6E380: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6E384: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D6E388: 419A0048  beq cr6, 0x82d6e3d0
	if ctx.cr[6].eq {
	pc = 0x82D6E3D0; continue 'dispatch;
	}
	// 82D6E38C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6E390: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82D6E394: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D6E398: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 82D6E39C: 4BFF22ED  bl 0x82d60688
	ctx.lr = 0x82D6E3A0;
	sub_82D60688(ctx, base);
	// 82D6E3A0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D6E3A4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82D6E3A8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82D6E3AC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6E3B0: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6E3B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D6E3B8: 4E800421  bctrl
	ctx.lr = 0x82D6E3BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D6E3BC: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82D6E3C0: 80DF0014  lwz r6, 0x14(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D6E3C4: 80BF0010  lwz r5, 0x10(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D6E3C8: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D6E3CC: 48004E45  bl 0x82d73210
	ctx.lr = 0x82D6E3D0;
	sub_82D73210(ctx, base);
	// 82D6E3D0: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82D6E3D4: 3B7B0008  addi r27, r27, 8
	ctx.r[27].s64 = ctx.r[27].s64 + 8;
	// 82D6E3D8: 48000050  b 0x82d6e428
	pc = 0x82D6E428; continue 'dispatch;
	// 82D6E3DC: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6E3E0: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 82D6E3E4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D6E3E8: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D6E3EC: 917C0004  stw r11, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D6E3F0: 40980038  bge cr6, 0x82d6e428
	if !ctx.cr[6].lt {
	pc = 0x82D6E428; continue 'dispatch;
	}
	// 82D6E3F4: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 82D6E3F8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6E3FC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D6E400: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82D6E404: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82D6E408: 390B0008  addi r8, r11, 8
	ctx.r[8].s64 = ctx.r[11].s64 + 8;
	// 82D6E40C: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6E410: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82D6E414: 81080004  lwz r8, 4(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6E418: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82D6E41C: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6E420: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D6E424: 4198FFD4  blt cr6, 0x82d6e3f8
	if ctx.cr[6].lt {
	pc = 0x82D6E3F8; continue 'dispatch;
	}
	// 82D6E428: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D6E42C: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D6E430: 4198FF00  blt cr6, 0x82d6e330
	if ctx.cr[6].lt {
	pc = 0x82D6E330; continue 'dispatch;
	}
	// 82D6E434: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D6E438: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D6E43C: 4BF3B014  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6E440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D6E440 size=16
    let mut pc: u32 = 0x82D6E440;
    'dispatch: loop {
        match pc {
            0x82D6E440 => {
    //   block [0x82D6E440..0x82D6E450)
	// 82D6E440: 8123001C  lwz r9, 0x1c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D6E444: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D6E448: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82D6E44C: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6E450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D6E450 size=36
    let mut pc: u32 = 0x82D6E450;
    'dispatch: loop {
        match pc {
            0x82D6E450 => {
    //   block [0x82D6E450..0x82D6E474)
	// 82D6E450: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D6E454: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6E458: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82D6E45C: 419A0018  beq cr6, 0x82d6e474
	if ctx.cr[6].eq {
		sub_82D6E474(ctx, base);
		return;
	}
	// 82D6E460: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D6E464: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82D6E468: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D6E46C: 4198FFE8  blt cr6, 0x82d6e454
	if ctx.cr[6].lt {
	pc = 0x82D6E454; continue 'dispatch;
	}
	// 82D6E470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6E474(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D6E474 size=48
    let mut pc: u32 = 0x82D6E474;
    'dispatch: loop {
        match pc {
            0x82D6E474 => {
    //   block [0x82D6E474..0x82D6E4A4)
	// 82D6E474: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D6E478: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D6E47C: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D6E480: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82D6E484: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D6E488: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D6E48C: 5546103A  slwi r6, r10, 2
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82D6E490: 9143001C  stw r10, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82D6E494: 7D46582E  lwzx r10, r6, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D6E498: 7D48592E  stwx r10, r8, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82D6E49C: 7C67482E  lwzx r3, r7, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82D6E4A0: 4BFE7030  b 0x82d554d0
	sub_82D554D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D6E4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D6E4A8 size=32
    let mut pc: u32 = 0x82D6E4A8;
    'dispatch: loop {
        match pc {
            0x82D6E4A8 => {
    //   block [0x82D6E4A8..0x82D6E4C8)
	// 82D6E4A8: 81430024  lwz r10, 0x24(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D6E4AC: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D6E4B0: 7D2B502E  lwzx r9, r11, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D6E4B4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D6E4B8: 419A0010  beq cr6, 0x82d6e4c8
	if ctx.cr[6].eq {
		sub_82D6E4C8(ctx, base);
		return;
	}
	// 82D6E4BC: 552B003E  slwi r11, r9, 0
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D6E4C0: 7C655A14  add r3, r5, r11
	ctx.r[3].u64 = ctx.r[5].u64 + ctx.r[11].u64;
	// 82D6E4C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


