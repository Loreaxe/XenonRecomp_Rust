pub fn sub_830B1A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B1A88 size=100
    let mut pc: u32 = 0x830B1A88;
    'dispatch: loop {
        match pc {
            0x830B1A88 => {
    //   block [0x830B1A88..0x830B1AEC)
	// 830B1A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B1A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B1A90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B1A94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B1A98: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830B1A9C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B1AA0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B1AA4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B1AA8: 396B7308  addi r11, r11, 0x7308
	ctx.r[11].s64 = ctx.r[11].s64 + 29448;
	// 830B1AAC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830B1AB0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B1AB4: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830B1AB8: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830B1ABC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B1AC0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B1AC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B1AC8: 4E800421  bctrl
	ctx.lr = 0x830B1ACC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B1ACC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B1AD0: 4BFF3B99  bl 0x830a5668
	ctx.lr = 0x830B1AD4;
	sub_830A5668(ctx, base);
	// 830B1AD4: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830B1AD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B1ADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B1AE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B1AE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B1AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1AEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B1AEC size=40
    let mut pc: u32 = 0x830B1AEC;
    'dispatch: loop {
        match pc {
            0x830B1AEC => {
    //   block [0x830B1AEC..0x830B1B14)
	// 830B1AEC: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830B1AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B1AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B1AF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B1AFC: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830B1B00: 4BFF3B69  bl 0x830a5668
	ctx.lr = 0x830B1B04;
	sub_830A5668(ctx, base);
	// 830B1B04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B1B08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B1B0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B1B10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B1B18 size=76
    let mut pc: u32 = 0x830B1B18;
    'dispatch: loop {
        match pc {
            0x830B1B18 => {
    //   block [0x830B1B18..0x830B1B64)
	// 830B1B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B1B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B1B20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B1B24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B1B28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B1B2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B1B30: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B1B34: 4BFFFF55  bl 0x830b1a88
	ctx.lr = 0x830B1B38;
	sub_830B1A88(ctx, base);
	// 830B1B38: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B1B3C: 4182000C  beq 0x830b1b48
	if ctx.cr[0].eq {
	pc = 0x830B1B48; continue 'dispatch;
	}
	// 830B1B40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B1B44: 4BF2679D  bl 0x82fd82e0
	ctx.lr = 0x830B1B48;
	sub_82FD82E0(ctx, base);
	// 830B1B48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B1B4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B1B50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B1B54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B1B58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B1B5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B1B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B1B68 size=76
    let mut pc: u32 = 0x830B1B68;
    'dispatch: loop {
        match pc {
            0x830B1B68 => {
    //   block [0x830B1B68..0x830B1BB4)
	// 830B1B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B1B6C: 480F65FD  bl 0x831a8168
	ctx.lr = 0x830B1B70;
	sub_831A8130(ctx, base);
	// 830B1B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B1B74: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B1B78: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830B1B7C: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 830B1B80: 38800019  li r4, 0x19
	ctx.r[4].s64 = 25;
	// 830B1B84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B1B88: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 830B1B8C: 4BFF393D  bl 0x830a54c8
	ctx.lr = 0x830B1B90;
	sub_830A54C8(ctx, base);
	// 830B1B90: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B1B94: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 830B1B98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B1B9C: 939F0010  stw r28, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 830B1BA0: 396B73C8  addi r11, r11, 0x73c8
	ctx.r[11].s64 = ctx.r[11].s64 + 29640;
	// 830B1BA4: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 830B1BA8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B1BAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B1BB0: 480F6608  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B1BB8 size=88
    let mut pc: u32 = 0x830B1BB8;
    'dispatch: loop {
        match pc {
            0x830B1BB8 => {
    //   block [0x830B1BB8..0x830B1C10)
	// 830B1BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B1BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B1BC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B1BC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B1BC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B1BCC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B1BD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B1BD4: 396B73C8  addi r11, r11, 0x73c8
	ctx.r[11].s64 = ctx.r[11].s64 + 29640;
	// 830B1BD8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B1BDC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B1BE0: 4BFF3A89  bl 0x830a5668
	ctx.lr = 0x830B1BE4;
	sub_830A5668(ctx, base);
	// 830B1BE4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B1BE8: 4182000C  beq 0x830b1bf4
	if ctx.cr[0].eq {
	pc = 0x830B1BF4; continue 'dispatch;
	}
	// 830B1BEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B1BF0: 4BF266F1  bl 0x82fd82e0
	ctx.lr = 0x830B1BF4;
	sub_82FD82E0(ctx, base);
	// 830B1BF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B1BF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B1BFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B1C00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B1C04: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B1C08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B1C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B1C10 size=84
    let mut pc: u32 = 0x830B1C10;
    'dispatch: loop {
        match pc {
            0x830B1C10 => {
    //   block [0x830B1C10..0x830B1C64)
	// 830B1C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B1C14: 480F6551  bl 0x831a8164
	ctx.lr = 0x830B1C18;
	sub_831A8130(ctx, base);
	// 830B1C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B1C1C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B1C20: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830B1C24: 7D054378  mr r5, r8
	ctx.r[5].u64 = ctx.r[8].u64;
	// 830B1C28: 3880001A  li r4, 0x1a
	ctx.r[4].s64 = 26;
	// 830B1C2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B1C30: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 830B1C34: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 830B1C38: 4BFF3891  bl 0x830a54c8
	ctx.lr = 0x830B1C3C;
	sub_830A54C8(ctx, base);
	// 830B1C3C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B1C40: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 830B1C44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B1C48: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 830B1C4C: 396B7410  addi r11, r11, 0x7410
	ctx.r[11].s64 = ctx.r[11].s64 + 29712;
	// 830B1C50: 939F0014  stw r28, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 830B1C54: 937F0018  stw r27, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[27].u32 ) };
	// 830B1C58: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B1C5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B1C60: 480F6554  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B1C68 size=24
    let mut pc: u32 = 0x830B1C68;
    'dispatch: loop {
        match pc {
            0x830B1C68 => {
    //   block [0x830B1C68..0x830B1C80)
	// 830B1C68: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 830B1C6C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830B1C70: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830B1C74: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 830B1C78: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 830B1C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B1C80 size=100
    let mut pc: u32 = 0x830B1C80;
    'dispatch: loop {
        match pc {
            0x830B1C80 => {
    //   block [0x830B1C80..0x830B1CE4)
	// 830B1C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B1C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B1C88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B1C8C: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 830B1C90: 41990028  bgt cr6, 0x830b1cb8
	if ctx.cr[6].gt {
	pc = 0x830B1CB8; continue 'dispatch;
	}
	// 830B1C94: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 830B1C98: 409A0018  bne cr6, 0x830b1cb0
	if !ctx.cr[6].eq {
	pc = 0x830B1CB0; continue 'dispatch;
	}
	// 830B1C9C: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 830B1CA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B1CA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B1CA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B1CAC: 4E800020  blr
	return;
	// 830B1CB0: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 830B1CB4: 4BFFFFEC  b 0x830b1ca0
	pc = 0x830B1CA0; continue 'dispatch;
	// 830B1CB8: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B1CBC: 80E30008  lwz r7, 8(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B1CC0: 38C00127  li r6, 0x127
	ctx.r[6].s64 = 295;
	// 830B1CC4: 388B7458  addi r4, r11, 0x7458
	ctx.r[4].s64 = ctx.r[11].s64 + 29784;
	// 830B1CC8: 38A00052  li r5, 0x52
	ctx.r[5].s64 = 82;
	// 830B1CCC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B1CD0: 4BF1F389  bl 0x82fd1058
	ctx.lr = 0x830B1CD4;
	sub_82FD1058(ctx, base);
	// 830B1CD4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830B1CD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B1CDC: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830B1CE0: 480FEF49  bl 0x831b0c28
	ctx.lr = 0x830B1CE4;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B1CE8 size=88
    let mut pc: u32 = 0x830B1CE8;
    'dispatch: loop {
        match pc {
            0x830B1CE8 => {
    //   block [0x830B1CE8..0x830B1D40)
	// 830B1CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B1CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B1CF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B1CF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B1CF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B1CFC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B1D00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B1D04: 396B7410  addi r11, r11, 0x7410
	ctx.r[11].s64 = ctx.r[11].s64 + 29712;
	// 830B1D08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B1D0C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B1D10: 4BFF3959  bl 0x830a5668
	ctx.lr = 0x830B1D14;
	sub_830A5668(ctx, base);
	// 830B1D14: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B1D18: 4182000C  beq 0x830b1d24
	if ctx.cr[0].eq {
	pc = 0x830B1D24; continue 'dispatch;
	}
	// 830B1D1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B1D20: 4BF265C1  bl 0x82fd82e0
	ctx.lr = 0x830B1D24;
	sub_82FD82E0(ctx, base);
	// 830B1D24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B1D28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B1D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B1D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B1D34: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B1D38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B1D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B1D40 size=40
    let mut pc: u32 = 0x830B1D40;
    'dispatch: loop {
        match pc {
            0x830B1D40 => {
    //   block [0x830B1D40..0x830B1D68)
	// 830B1D40: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 830B1D44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B1D48: 394AD414  addi r10, r10, -0x2bec
	ctx.r[10].s64 = ctx.r[10].s64 + -11244;
	// 830B1D4C: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 830B1D50: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830B1D54: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830B1D58: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830B1D5C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830B1D60: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 830B1D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B1D68 size=280
    let mut pc: u32 = 0x830B1D68;
    'dispatch: loop {
        match pc {
            0x830B1D68 => {
    //   block [0x830B1D68..0x830B1E80)
	// 830B1D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B1D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B1D70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B1D74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B1D78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B1D7C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B1D80: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B1D84: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830B1D88: 408200E4  bne 0x830b1e6c
	if !ctx.cr[0].eq {
	pc = 0x830B1E6C; continue 'dispatch;
	}
	// 830B1D8C: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B1D90: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830B1D94: 419A00C0  beq cr6, 0x830b1e54
	if ctx.cr[6].eq {
	pc = 0x830B1E54; continue 'dispatch;
	}
	// 830B1D98: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830B1D9C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 830B1DA0: 419800A4  blt cr6, 0x830b1e44
	if ctx.cr[6].lt {
	pc = 0x830B1E44; continue 'dispatch;
	}
	// 830B1DA4: 419A0088  beq cr6, 0x830b1e2c
	if ctx.cr[6].eq {
	pc = 0x830B1E2C; continue 'dispatch;
	}
	// 830B1DA8: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 830B1DAC: 419800A8  blt cr6, 0x830b1e54
	if ctx.cr[6].lt {
	pc = 0x830B1E54; continue 'dispatch;
	}
	// 830B1DB0: 419A0060  beq cr6, 0x830b1e10
	if ctx.cr[6].eq {
	pc = 0x830B1E10; continue 'dispatch;
	}
	// 830B1DB4: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 830B1DB8: 41980014  blt cr6, 0x830b1dcc
	if ctx.cr[6].lt {
	pc = 0x830B1DCC; continue 'dispatch;
	}
	// 830B1DBC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B1DC0: 386B7490  addi r3, r11, 0x7490
	ctx.r[3].s64 = ctx.r[11].s64 + 29840;
	// 830B1DC4: 48006E3D  bl 0x830b8c00
	ctx.lr = 0x830B1DC8;
	sub_830B8C00(ctx, base);
	// 830B1DC8: 4800008C  b 0x830b1e54
	pc = 0x830B1E54; continue 'dispatch;
	// 830B1DCC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B1DD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B1DD4: 419A0018  beq cr6, 0x830b1dec
	if ctx.cr[6].eq {
	pc = 0x830B1DEC; continue 'dispatch;
	}
	// 830B1DD8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B1DDC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830B1DE0: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B1DE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B1DE8: 4E800421  bctrl
	ctx.lr = 0x830B1DEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B1DEC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B1DF0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B1DF4: 419A0060  beq cr6, 0x830b1e54
	if ctx.cr[6].eq {
	pc = 0x830B1E54; continue 'dispatch;
	}
	// 830B1DF8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B1DFC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830B1E00: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B1E04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B1E08: 4E800421  bctrl
	ctx.lr = 0x830B1E0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B1E0C: 48000014  b 0x830b1e20
	pc = 0x830B1E20; continue 'dispatch;
	// 830B1E10: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B1E14: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830B1E18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B1E1C: 4E800421  bctrl
	ctx.lr = 0x830B1E20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B1E20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B1E24: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830B1E28: 4800002C  b 0x830b1e54
	pc = 0x830B1E54; continue 'dispatch;
	// 830B1E2C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B1E30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B1E34: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B1E38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B1E3C: 4E800421  bctrl
	ctx.lr = 0x830B1E40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B1E40: 48000014  b 0x830b1e54
	pc = 0x830B1E54; continue 'dispatch;
	// 830B1E44: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830B1E48: 48002E39  bl 0x830b4c80
	ctx.lr = 0x830B1E4C;
	sub_830B4C80(ctx, base);
	// 830B1E4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B1E50: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830B1E54: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B1E58: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830B1E5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B1E60: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B1E64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B1E68: 4E800421  bctrl
	ctx.lr = 0x830B1E6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B1E6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B1E70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B1E74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B1E78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B1E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B1E80 size=16
    let mut pc: u32 = 0x830B1E80;
    'dispatch: loop {
        match pc {
            0x830B1E80 => {
    //   block [0x830B1E80..0x830B1E90)
	// 830B1E80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B1E84: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830B1E88: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830B1E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B1E90 size=20
    let mut pc: u32 = 0x830B1E90;
    'dispatch: loop {
        match pc {
            0x830B1E90 => {
    //   block [0x830B1E90..0x830B1EA4)
	// 830B1E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B1E94: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830B1E98: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830B1E9C: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 830B1EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B1EA8 size=20
    let mut pc: u32 = 0x830B1EA8;
    'dispatch: loop {
        match pc {
            0x830B1EA8 => {
    //   block [0x830B1EA8..0x830B1EBC)
	// 830B1EA8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B1EAC: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 830B1EB0: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 830B1EB4: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830B1EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B1EC0 size=24
    let mut pc: u32 = 0x830B1EC0;
    'dispatch: loop {
        match pc {
            0x830B1EC0 => {
    //   block [0x830B1EC0..0x830B1ED8)
	// 830B1EC0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B1EC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B1EC8: 396B74B0  addi r11, r11, 0x74b0
	ctx.r[11].s64 = ctx.r[11].s64 + 29872;
	// 830B1ECC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830B1ED0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B1ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B1ED8 size=76
    let mut pc: u32 = 0x830B1ED8;
    'dispatch: loop {
        match pc {
            0x830B1ED8 => {
    //   block [0x830B1ED8..0x830B1F24)
	// 830B1ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B1EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B1EE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B1EE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B1EE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B1EEC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B1EF0: 396B74B0  addi r11, r11, 0x74b0
	ctx.r[11].s64 = ctx.r[11].s64 + 29872;
	// 830B1EF4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B1EF8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B1EFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B1F00: 419A0010  beq cr6, 0x830b1f10
	if ctx.cr[6].eq {
	pc = 0x830B1F10; continue 'dispatch;
	}
	// 830B1F04: 4BFFFE65  bl 0x830b1d68
	ctx.lr = 0x830B1F08;
	sub_830B1D68(ctx, base);
	// 830B1F08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B1F0C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830B1F10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B1F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B1F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B1F1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B1F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B1F28 size=20
    let mut pc: u32 = 0x830B1F28;
    'dispatch: loop {
        match pc {
            0x830B1F28 => {
    //   block [0x830B1F28..0x830B1F3C)
	// 830B1F28: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B1F2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B1F30: 419A000C  beq cr6, 0x830b1f3c
	if ctx.cr[6].eq {
		sub_830B1F3C(ctx, base);
		return;
	}
	// 830B1F34: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B1F38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1F3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B1F3C size=8
    let mut pc: u32 = 0x830B1F3C;
    'dispatch: loop {
        match pc {
            0x830B1F3C => {
    //   block [0x830B1F3C..0x830B1F44)
	// 830B1F3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830B1F40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B1F48 size=36
    let mut pc: u32 = 0x830B1F48;
    'dispatch: loop {
        match pc {
            0x830B1F48 => {
    //   block [0x830B1F48..0x830B1F6C)
	// 830B1F48: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B1F4C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B1F50: 419A001C  beq cr6, 0x830b1f6c
	if ctx.cr[6].eq {
		sub_830B1F6C(ctx, base);
		return;
	}
	// 830B1F54: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B1F58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B1F5C: 419A0010  beq cr6, 0x830b1f6c
	if ctx.cr[6].eq {
		sub_830B1F6C(ctx, base);
		return;
	}
	// 830B1F60: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B1F64: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B1F68: 48000008  b 0x830b1f70
	sub_830B1F6C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1F6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B1F6C size=20
    let mut pc: u32 = 0x830B1F6C;
    'dispatch: loop {
        match pc {
            0x830B1F6C => {
    //   block [0x830B1F6C..0x830B1F80)
	// 830B1F6C: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B1F70: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 830B1F74: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830B1F78: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830B1F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B1F80 size=16
    let mut pc: u32 = 0x830B1F80;
    'dispatch: loop {
        match pc {
            0x830B1F80 => {
    //   block [0x830B1F80..0x830B1F90)
	// 830B1F80: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B1F84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830B1F88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B1F8C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B1F90 size=8
    let mut pc: u32 = 0x830B1F90;
    'dispatch: loop {
        match pc {
            0x830B1F90 => {
    //   block [0x830B1F90..0x830B1F98)
	// 830B1F90: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B1F94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B1F98 size=140
    let mut pc: u32 = 0x830B1F98;
    'dispatch: loop {
        match pc {
            0x830B1F98 => {
    //   block [0x830B1F98..0x830B2024)
	// 830B1F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B1F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B1FA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B1FA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B1FA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B1FAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B1FB0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B1FB4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B1FB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B1FBC: 419A001C  beq cr6, 0x830b1fd8
	if ctx.cr[6].eq {
	pc = 0x830B1FD8; continue 'dispatch;
	}
	// 830B1FC0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B1FC4: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 830B1FC8: 419A0044  beq cr6, 0x830b200c
	if ctx.cr[6].eq {
	pc = 0x830B200C; continue 'dispatch;
	}
	// 830B1FCC: 4BFFFD9D  bl 0x830b1d68
	ctx.lr = 0x830B1FD0;
	sub_830B1D68(ctx, base);
	// 830B1FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B1FD4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830B1FD8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830B1FDC: 419A0030  beq cr6, 0x830b200c
	if ctx.cr[6].eq {
	pc = 0x830B200C; continue 'dispatch;
	}
	// 830B1FE0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B1FE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B1FE8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B1FEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B1FF0: 4E800421  bctrl
	ctx.lr = 0x830B1FF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B1FF4: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 830B1FF8: 93C30004  stw r30, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 830B1FFC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B2000: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B2004: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830B2008: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830B200C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B2010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B2014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B2018: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B201C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B2020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B2028 size=132
    let mut pc: u32 = 0x830B2028;
    'dispatch: loop {
        match pc {
            0x830B2028 => {
    //   block [0x830B2028..0x830B20AC)
	// 830B2028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B202C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B2030: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B2034: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B2038: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B203C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B2040: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B2044: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B2048: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B204C: 419A002C  beq cr6, 0x830b2078
	if ctx.cr[6].eq {
	pc = 0x830B2078; continue 'dispatch;
	}
	// 830B2050: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B2054: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B2058: 419A0014  beq cr6, 0x830b206c
	if ctx.cr[6].eq {
	pc = 0x830B206C; continue 'dispatch;
	}
	// 830B205C: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B2060: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B2064: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B2068: 419A002C  beq cr6, 0x830b2094
	if ctx.cr[6].eq {
	pc = 0x830B2094; continue 'dispatch;
	}
	// 830B206C: 4BFFFCFD  bl 0x830b1d68
	ctx.lr = 0x830B2070;
	sub_830B1D68(ctx, base);
	// 830B2070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B2074: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830B2078: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B207C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B2080: 419A0014  beq cr6, 0x830b2094
	if ctx.cr[6].eq {
	pc = 0x830B2094; continue 'dispatch;
	}
	// 830B2084: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830B2088: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B208C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830B2090: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 830B2094: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B2098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B209C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B20A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B20A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B20A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B20B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B20B0 size=72
    let mut pc: u32 = 0x830B20B0;
    'dispatch: loop {
        match pc {
            0x830B20B0 => {
    //   block [0x830B20B0..0x830B20F8)
	// 830B20B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B20B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B20B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B20BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B20C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B20C4: 4BFFFED5  bl 0x830b1f98
	ctx.lr = 0x830B20C8;
	sub_830B1F98(ctx, base);
	// 830B20C8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B20CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B20D0: 419A0014  beq cr6, 0x830b20e4
	if ctx.cr[6].eq {
	pc = 0x830B20E4; continue 'dispatch;
	}
	// 830B20D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B20D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 830B20DC: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830B20E0: 912B0010  stw r9, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 830B20E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B20E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B20EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B20F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B20F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B20F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B20F8 size=8
    let mut pc: u32 = 0x830B20F8;
    'dispatch: loop {
        match pc {
            0x830B20F8 => {
    //   block [0x830B20F8..0x830B2100)
	// 830B20F8: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 830B20FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B2100 size=8
    let mut pc: u32 = 0x830B2100;
    'dispatch: loop {
        match pc {
            0x830B2100 => {
    //   block [0x830B2100..0x830B2108)
	// 830B2100: 806300CC  lwz r3, 0xcc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(204 as u32) ) } as u64;
	// 830B2104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B2108 size=8
    let mut pc: u32 = 0x830B2108;
    'dispatch: loop {
        match pc {
            0x830B2108 => {
    //   block [0x830B2108..0x830B2110)
	// 830B2108: D0230084  stfs f1, 0x84(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 830B210C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B2110 size=8
    let mut pc: u32 = 0x830B2110;
    'dispatch: loop {
        match pc {
            0x830B2110 => {
    //   block [0x830B2110..0x830B2118)
	// 830B2110: C0230084  lfs f1, 0x84(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B2114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B2118 size=24
    let mut pc: u32 = 0x830B2118;
    'dispatch: loop {
        match pc {
            0x830B2118 => {
    //   block [0x830B2118..0x830B2130)
	// 830B2118: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830B211C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B2120: C02B440C  lfs f1, 0x440c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17420 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B2124: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B2128: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B212C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B2130 size=12
    let mut pc: u32 = 0x830B2130;
    'dispatch: loop {
        match pc {
            0x830B2130 => {
    //   block [0x830B2130..0x830B213C)
	// 830B2130: D023007C  stfs f1, 0x7c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 830B2134: D0230080  stfs f1, 0x80(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 830B2138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B2140 size=8
    let mut pc: u32 = 0x830B2140;
    'dispatch: loop {
        match pc {
            0x830B2140 => {
    //   block [0x830B2140..0x830B2148)
	// 830B2140: C023007C  lfs f1, 0x7c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B2144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B2148 size=8
    let mut pc: u32 = 0x830B2148;
    'dispatch: loop {
        match pc {
            0x830B2148 => {
    //   block [0x830B2148..0x830B2150)
	// 830B2148: D0230088  stfs f1, 0x88(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 830B214C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B2150 size=8
    let mut pc: u32 = 0x830B2150;
    'dispatch: loop {
        match pc {
            0x830B2150 => {
    //   block [0x830B2150..0x830B2158)
	// 830B2150: C0230088  lfs f1, 0x88(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(136 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B2154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B2158 size=8
    let mut pc: u32 = 0x830B2158;
    'dispatch: loop {
        match pc {
            0x830B2158 => {
    //   block [0x830B2158..0x830B2160)
	// 830B2158: D023008C  stfs f1, 0x8c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 830B215C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B2160 size=8
    let mut pc: u32 = 0x830B2160;
    'dispatch: loop {
        match pc {
            0x830B2160 => {
    //   block [0x830B2160..0x830B2168)
	// 830B2160: C023008C  lfs f1, 0x8c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B2164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B2168 size=12
    let mut pc: u32 = 0x830B2168;
    'dispatch: loop {
        match pc {
            0x830B2168 => {
    //   block [0x830B2168..0x830B2174)
	// 830B2168: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830B216C: 9163009C  stw r11, 0x9c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 830B2170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B2178 size=12
    let mut pc: u32 = 0x830B2178;
    'dispatch: loop {
        match pc {
            0x830B2178 => {
    //   block [0x830B2178..0x830B2184)
	// 830B2178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B217C: 9163009C  stw r11, 0x9c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 830B2180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B2188 size=8
    let mut pc: u32 = 0x830B2188;
    'dispatch: loop {
        match pc {
            0x830B2188 => {
    //   block [0x830B2188..0x830B2190)
	// 830B2188: 908300B4  stw r4, 0xb4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(180 as u32), ctx.r[4].u32 ) };
	// 830B218C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B2190 size=24
    let mut pc: u32 = 0x830B2190;
    'dispatch: loop {
        match pc {
            0x830B2190 => {
    //   block [0x830B2190..0x830B21A8)
	// 830B2190: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830B2194: 806400D8  lwz r3, 0xd8(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(216 as u32) ) } as u64;
	// 830B2198: C00400BC  lfs f0, 0xbc(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(188 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B219C: D02400BC  stfs f1, 0xbc(r4)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 830B21A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B21A4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B21A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B21A8 size=8
    let mut pc: u32 = 0x830B21A8;
    'dispatch: loop {
        match pc {
            0x830B21A8 => {
    //   block [0x830B21A8..0x830B21B0)
	// 830B21A8: FC200090  fmr f1, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[0].f64;
	// 830B21AC: 4800407C  b 0x830b6228
	sub_830B6228(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B21B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B21B0 size=4
    let mut pc: u32 = 0x830B21B0;
    'dispatch: loop {
        match pc {
            0x830B21B0 => {
    //   block [0x830B21B0..0x830B21B4)
	// 830B21B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B21B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B21B8 size=8
    let mut pc: u32 = 0x830B21B8;
    'dispatch: loop {
        match pc {
            0x830B21B8 => {
    //   block [0x830B21B8..0x830B21C0)
	// 830B21B8: C02300BC  lfs f1, 0xbc(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(188 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B21BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B21C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B21C0 size=16
    let mut pc: u32 = 0x830B21C0;
    'dispatch: loop {
        match pc {
            0x830B21C0 => {
    //   block [0x830B21C0..0x830B21D0)
	// 830B21C0: 80A30074  lwz r5, 0x74(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(116 as u32) ) } as u64;
	// 830B21C4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830B21C8: 80630078  lwz r3, 0x78(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) } as u64;
	// 830B21CC: 480070FC  b 0x830b92c8
	sub_830B92C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B21D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B21D0 size=84
    let mut pc: u32 = 0x830B21D0;
    'dispatch: loop {
        match pc {
            0x830B21D0 => {
    //   block [0x830B21D0..0x830B2224)
	// 830B21D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B21D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B21D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B21DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B21E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B21E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B21E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B21EC: 4BFFFCD5  bl 0x830b1ec0
	ctx.lr = 0x830B21F0;
	sub_830B1EC0(ctx, base);
	// 830B21F0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 830B21F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B21F8: 396BC418  addi r11, r11, -0x3be8
	ctx.r[11].s64 = ctx.r[11].s64 + -15336;
	// 830B21FC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B2200: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B2204: 4BFFFEAD  bl 0x830b20b0
	ctx.lr = 0x830B2208;
	sub_830B20B0(ctx, base);
	// 830B2208: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B220C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B2210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B2214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B2218: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B221C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B2220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B2228 size=100
    let mut pc: u32 = 0x830B2228;
    'dispatch: loop {
        match pc {
            0x830B2228 => {
    //   block [0x830B2228..0x830B228C)
	// 830B2228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B222C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B2230: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B2234: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B2238: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B223C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B2240: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B2244: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B2248: 396B74B8  addi r11, r11, 0x74b8
	ctx.r[11].s64 = ctx.r[11].s64 + 29880;
	// 830B224C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 830B2250: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830B2254: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B2258: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B225C: 4BFFFC7D  bl 0x830b1ed8
	ctx.lr = 0x830B2260;
	sub_830B1ED8(ctx, base);
	// 830B2260: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B2264: 4182000C  beq 0x830b2270
	if ctx.cr[0].eq {
	pc = 0x830B2270; continue 'dispatch;
	}
	// 830B2268: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B226C: 4B20DFFD  bl 0x822c0268
	ctx.lr = 0x830B2270;
	sub_822C0268(ctx, base);
	// 830B2270: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B2274: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B2278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B227C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B2280: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B2284: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B2288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B2290 size=116
    let mut pc: u32 = 0x830B2290;
    'dispatch: loop {
        match pc {
            0x830B2290 => {
    //   block [0x830B2290..0x830B2304)
	// 830B2290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B2294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B2298: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B229C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B22A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B22A4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830B22A8: 419A0040  beq cr6, 0x830b22e8
	if ctx.cr[6].eq {
	pc = 0x830B22E8; continue 'dispatch;
	}
	// 830B22AC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 830B22B0: 0CDF0000  twi 6, r31, 0
	// 830B22B4: 7D6BFB96  divwu r11, r11, r31
	ctx.r[11].u32 = ctx.r[11].u32 / ctx.r[31].u32;
	// 830B22B8: 2B0B000C  cmplwi cr6, r11, 0xc
	ctx.cr[6].compare_u32(ctx.r[11].u32, 12 as u32, &mut ctx.xer);
	// 830B22BC: 4098002C  bge cr6, 0x830b22e8
	if !ctx.cr[6].lt {
	pc = 0x830B22E8; continue 'dispatch;
	}
	// 830B22C0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 830B22C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B22C8: 396B0828  addi r11, r11, 0x828
	ctx.r[11].s64 = ctx.r[11].s64 + 2088;
	// 830B22CC: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 830B22D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B22D4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830B22D8: 4B211FD9  bl 0x822c42b0
	ctx.lr = 0x830B22DC;
	sub_822C42B0(ctx, base);
	// 830B22DC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 830B22E0: 396B0818  addi r11, r11, 0x818
	ctx.r[11].s64 = ctx.r[11].s64 + 2072;
	// 830B22E4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830B22E8: 1C7F000C  mulli r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 * 12;
	// 830B22EC: 4B20E64D  bl 0x822c0938
	ctx.lr = 0x830B22F0;
	sub_822C0938(ctx, base);
	// 830B22F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B22F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B22F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B22FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B2300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B2308 size=16
    let mut pc: u32 = 0x830B2308;
    'dispatch: loop {
        match pc {
            0x830B2308 => {
    //   block [0x830B2308..0x830B2318)
	// 830B2308: 81830000  lwz r12, 0(r3)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B230C: 816C000C  lwz r11, 0xc(r12)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B2310: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B2314: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B2318 size=16
    let mut pc: u32 = 0x830B2318;
    'dispatch: loop {
        match pc {
            0x830B2318 => {
    //   block [0x830B2318..0x830B2328)
	// 830B2318: 81830000  lwz r12, 0(r3)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B231C: 816C0010  lwz r11, 0x10(r12)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(16 as u32) ) } as u64;
	// 830B2320: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B2324: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B2328 size=16
    let mut pc: u32 = 0x830B2328;
    'dispatch: loop {
        match pc {
            0x830B2328 => {
    //   block [0x830B2328..0x830B2338)
	// 830B2328: 81830000  lwz r12, 0(r3)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B232C: 816C0004  lwz r11, 4(r12)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B2330: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B2334: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B2338 size=16
    let mut pc: u32 = 0x830B2338;
    'dispatch: loop {
        match pc {
            0x830B2338 => {
    //   block [0x830B2338..0x830B2348)
	// 830B2338: 81830000  lwz r12, 0(r3)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B233C: 816C0008  lwz r11, 8(r12)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B2340: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B2344: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B2348 size=180
    let mut pc: u32 = 0x830B2348;
    'dispatch: loop {
        match pc {
            0x830B2348 => {
    //   block [0x830B2348..0x830B23FC)
	// 830B2348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B234C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B2350: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B2354: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B2358: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 830B235C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B2360: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 830B2364: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 830B2368: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B236C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830B2370: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 830B2374: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 830B2378: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B237C: C00A08A8  lfs f0, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B2380: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 830B2384: D3FF007C  stfs f31, 0x7c(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 830B2388: 93DF0070  stw r30, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[30].u32 ) };
	// 830B238C: D3FF0080  stfs f31, 0x80(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 830B2390: 93DF0074  stw r30, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 830B2394: D01F0084  stfs f0, 0x84(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 830B2398: 913F00B8  stw r9, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[9].u32 ) };
	// 830B239C: 93DF0078  stw r30, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[30].u32 ) };
	// 830B23A0: 4BFFFD11  bl 0x830b20b0
	ctx.lr = 0x830B23A4;
	sub_830B20B0(ctx, base);
	// 830B23A4: 817F00DC  lwz r11, 0xdc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 830B23A8: D3FF00BC  stfs f31, 0xbc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 830B23AC: 93DF0098  stw r30, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[30].u32 ) };
	// 830B23B0: 93DF009C  stw r30, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[30].u32 ) };
	// 830B23B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B23B8: 93DF00CC  stw r30, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[30].u32 ) };
	// 830B23BC: 93DF00D8  stw r30, 0xd8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[30].u32 ) };
	// 830B23C0: 93DF006C  stw r30, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 830B23C4: 409A001C  bne cr6, 0x830b23e0
	if !ctx.cr[6].eq {
	pc = 0x830B23E0; continue 'dispatch;
	}
	// 830B23C8: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B23CC: 38A000AE  li r5, 0xae
	ctx.r[5].s64 = 174;
	// 830B23D0: 388B74D0  addi r4, r11, 0x74d0
	ctx.r[4].s64 = ctx.r[11].s64 + 29904;
	// 830B23D4: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 830B23D8: 48002881  bl 0x830b4c58
	ctx.lr = 0x830B23DC;
	sub_830B4C58(ctx, base);
	// 830B23DC: 907F00DC  stw r3, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[3].u32 ) };
	// 830B23E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B23E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B23E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B23EC: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 830B23F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B23F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B23F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B2400 size=272
    let mut pc: u32 = 0x830B2400;
    'dispatch: loop {
        match pc {
            0x830B2400 => {
    //   block [0x830B2400..0x830B2510)
	// 830B2400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B2404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B2408: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B240C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B2410: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B2414: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B2418: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B241C: 817F00B8  lwz r11, 0xb8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 830B2420: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830B2424: 409A000C  bne cr6, 0x830b2430
	if !ctx.cr[6].eq {
	pc = 0x830B2430; continue 'dispatch;
	}
	// 830B2428: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830B242C: 480000CC  b 0x830b24f8
	pc = 0x830B24F8; continue 'dispatch;
	// 830B2430: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B2434: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B2438: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830B243C: 409800B8  bge cr6, 0x830b24f4
	if !ctx.cr[6].lt {
	pc = 0x830B24F4; continue 'dispatch;
	}
	// 830B2440: 807F0070  lwz r3, 0x70(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 830B2444: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B2448: 419A0020  beq cr6, 0x830b2468
	if ctx.cr[6].eq {
	pc = 0x830B2468; continue 'dispatch;
	}
	// 830B244C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B2450: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B2454: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B2458: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B245C: 4E800421  bctrl
	ctx.lr = 0x830B2460;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B2460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B2464: 917F0070  stw r11, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 830B2468: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B246C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B2470: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 830B2474: 815F00DC  lwz r10, 0xdc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 830B2478: 346A0028  addic. r3, r10, 0x28
	ctx.xer.ca = (ctx.r[10].u32 > (!(40 as u32)));
	ctx.r[3].s64 = ctx.r[10].s64 + 40;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 830B247C: 41820034  beq 0x830b24b0
	if ctx.cr[0].eq {
	pc = 0x830B24B0; continue 'dispatch;
	}
	// 830B2480: 41980014  blt cr6, 0x830b2494
	if ctx.cr[6].lt {
	pc = 0x830B2494; continue 'dispatch;
	}
	// 830B2484: 812B0048  lwz r9, 0x48(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 830B2488: 57C8103A  slwi r8, r30, 2
	ctx.r[8].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 830B248C: 7CA9402E  lwzx r5, r9, r8
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 830B2490: 48000008  b 0x830b2498
	pc = 0x830B2498; continue 'dispatch;
	// 830B2494: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830B2498: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 830B249C: 57CA1838  slwi r10, r30, 3
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830B24A0: 38DF0004  addi r6, r31, 4
	ctx.r[6].s64 = ctx.r[31].s64 + 4;
	// 830B24A4: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830B24A8: 48007041  bl 0x830b94e8
	ctx.lr = 0x830B24AC;
	sub_830B94E8(ctx, base);
	// 830B24AC: 48000008  b 0x830b24b4
	pc = 0x830B24B4; continue 'dispatch;
	// 830B24B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830B24B4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B24B8: 907F0070  stw r3, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 830B24BC: 93DF00B8  stw r30, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[30].u32 ) };
	// 830B24C0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B24C4: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 830B24C8: 4198FF60  blt cr6, 0x830b2428
	if ctx.cr[6].lt {
	pc = 0x830B2428; continue 'dispatch;
	}
	// 830B24CC: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 830B24D0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B24D4: 419AFF54  beq cr6, 0x830b2428
	if ctx.cr[6].eq {
	pc = 0x830B2428; continue 'dispatch;
	}
	// 830B24D8: 57CB1838  slwi r11, r30, 3
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830B24DC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830B24E0: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B24E4: D01F0088  stfs f0, 0x88(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 830B24E8: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B24EC: D01F008C  stfs f0, 0x8c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 830B24F0: 4BFFFF38  b 0x830b2428
	pc = 0x830B2428; continue 'dispatch;
	// 830B24F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830B24F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B24FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B2500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B2504: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B2508: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B250C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B2510 size=156
    let mut pc: u32 = 0x830B2510;
    'dispatch: loop {
        match pc {
            0x830B2510 => {
    //   block [0x830B2510..0x830B25AC)
	// 830B2510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B2514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B2518: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B251C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B2520: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B2524: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 830B2528: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B252C: 419A0020  beq cr6, 0x830b254c
	if ctx.cr[6].eq {
	pc = 0x830B254C; continue 'dispatch;
	}
	// 830B2530: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B2534: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B2538: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B253C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B2540: 4E800421  bctrl
	ctx.lr = 0x830B2544;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B2544: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B2548: 917F0078  stw r11, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 830B254C: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B2550: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 830B2554: 419A0044  beq cr6, 0x830b2598
	if ctx.cr[6].eq {
	pc = 0x830B2598; continue 'dispatch;
	}
	// 830B2558: 817F0074  lwz r11, 0x74(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 830B255C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B2560: 419A0018  beq cr6, 0x830b2578
	if ctx.cr[6].eq {
	pc = 0x830B2578; continue 'dispatch;
	}
	// 830B2564: 815F00DC  lwz r10, 0xdc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 830B2568: 346A0014  addic. r3, r10, 0x14
	ctx.xer.ca = (ctx.r[10].u32 > (!(20 as u32)));
	ctx.r[3].s64 = ctx.r[10].s64 + 20;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 830B256C: 41820024  beq 0x830b2590
	if ctx.cr[0].eq {
	pc = 0x830B2590; continue 'dispatch;
	}
	// 830B2570: 80AB000C  lwz r5, 0xc(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B2574: 48000014  b 0x830b2588
	pc = 0x830B2588; continue 'dispatch;
	// 830B2578: 817F00DC  lwz r11, 0xdc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 830B257C: 346B0014  addic. r3, r11, 0x14
	ctx.xer.ca = (ctx.r[11].u32 > (!(20 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 20;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 830B2580: 41820010  beq 0x830b2590
	if ctx.cr[0].eq {
	pc = 0x830B2590; continue 'dispatch;
	}
	// 830B2584: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830B2588: 48006E49  bl 0x830b93d0
	ctx.lr = 0x830B258C;
	sub_830B93D0(ctx, base);
	// 830B258C: 48000008  b 0x830b2594
	pc = 0x830B2594; continue 'dispatch;
	// 830B2590: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830B2594: 907F0078  stw r3, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[3].u32 ) };
	// 830B2598: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B259C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B25A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B25A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B25A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B25B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B25B0 size=116
    let mut pc: u32 = 0x830B25B0;
    'dispatch: loop {
        match pc {
            0x830B25B0 => {
    //   block [0x830B25B0..0x830B2624)
	// 830B25B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B25B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B25B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B25BC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B25C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B25C4: 8084000C  lwz r4, 0xc(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B25C8: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830B25CC: 480092C5  bl 0x830bb890
	ctx.lr = 0x830B25D0;
	sub_830BB890(ctx, base);
	// 830B25D0: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B25D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B25D8: 41820020  beq 0x830b25f8
	if ctx.cr[0].eq {
	pc = 0x830B25F8; continue 'dispatch;
	}
	// 830B25DC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B25E0: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B25E4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B25E8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830B25EC: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 830B25F0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830B25F4: 4800001C  b 0x830b2610
	pc = 0x830B2610; continue 'dispatch;
	// 830B25F8: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830B25FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B2600: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 830B2604: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B2608: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830B260C: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 830B2610: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B2614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B2618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B261C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B2620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B2628 size=100
    let mut pc: u32 = 0x830B2628;
    'dispatch: loop {
        match pc {
            0x830B2628 => {
    //   block [0x830B2628..0x830B268C)
	// 830B2628: 81630078  lwz r11, 0x78(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) } as u64;
	// 830B262C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830B2630: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B2634: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B2638: 80CB0000  lwz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B263C: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 830B2640: 40990044  ble cr6, 0x830b2684
	if !ctx.cr[6].gt {
	pc = 0x830B2684; continue 'dispatch;
	}
	// 830B2644: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B2648: 812A0028  lwz r9, 0x28(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 830B264C: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 830B2650: 39690008  addi r11, r9, 8
	ctx.r[11].s64 = ctx.r[9].s64 + 8;
	// 830B2654: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B2658: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B265C: 1D290144  mulli r9, r9, 0x144
	ctx.r[9].s64 = ctx.r[9].s64 * 324;
	// 830B2660: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 830B2664: 81290018  lwz r9, 0x18(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) } as u64;
	// 830B2668: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 830B266C: 409A0020  bne cr6, 0x830b268c
	if !ctx.cr[6].eq {
		sub_830B268C(ctx, base);
		return;
	}
	// 830B2670: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 830B2674: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 830B2678: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 830B267C: 7F073000  cmpw cr6, r7, r6
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[6].s32, &mut ctx.xer);
	// 830B2680: 4198FFD4  blt cr6, 0x830b2654
	if ctx.cr[6].lt {
	pc = 0x830B2654; continue 'dispatch;
	}
	// 830B2684: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830B2688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B268C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B268C size=8
    let mut pc: u32 = 0x830B268C;
    'dispatch: loop {
        match pc {
            0x830B268C => {
    //   block [0x830B268C..0x830B2694)
	// 830B268C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830B2690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B2698 size=24
    let mut pc: u32 = 0x830B2698;
    'dispatch: loop {
        match pc {
            0x830B2698 => {
    //   block [0x830B2698..0x830B26B0)
	// 830B2698: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B269C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B26A0: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 830B26A4: 4198000C  blt cr6, 0x830b26b0
	if ctx.cr[6].lt {
		sub_830B26B0(ctx, base);
		return;
	}
	// 830B26A8: C02B0040  lfs f1, 0x40(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B26AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B26B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B26B0 size=12
    let mut pc: u32 = 0x830B26B0;
    'dispatch: loop {
        match pc {
            0x830B26B0 => {
    //   block [0x830B26B0..0x830B26BC)
	// 830B26B0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B26B4: C02B750C  lfs f1, 0x750c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29964 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B26B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B26C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B26C0 size=112
    let mut pc: u32 = 0x830B26C0;
    'dispatch: loop {
        match pc {
            0x830B26C0 => {
    //   block [0x830B26C0..0x830B2730)
	// 830B26C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B26C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B26C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B26CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B26D0: 48002579  bl 0x830b4c48
	ctx.lr = 0x830B26D4;
	sub_830B4C48(ctx, base);
	// 830B26D4: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B26D8: 38C000A5  li r6, 0xa5
	ctx.r[6].s64 = 165;
	// 830B26DC: 38AB7510  addi r5, r11, 0x7510
	ctx.r[5].s64 = ctx.r[11].s64 + 29968;
	// 830B26E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B26E4: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 830B26E8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B26EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B26F0: 4E800421  bctrl
	ctx.lr = 0x830B26F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B26F4: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830B26F8: 41820020  beq 0x830b2718
	if ctx.cr[0].eq {
	pc = 0x830B2718; continue 'dispatch;
	}
	// 830B26FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B2700: 4BFFF641  bl 0x830b1d40
	ctx.lr = 0x830B2704;
	sub_830B1D40(ctx, base);
	// 830B2704: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B2708: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B270C: 396B74C8  addi r11, r11, 0x74c8
	ctx.r[11].s64 = ctx.r[11].s64 + 29896;
	// 830B2710: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B2714: 48000008  b 0x830b271c
	pc = 0x830B271C; continue 'dispatch;
	// 830B2718: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830B271C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B2720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B2724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B2728: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B272C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B2730 size=72
    let mut pc: u32 = 0x830B2730;
    'dispatch: loop {
        match pc {
            0x830B2730 => {
    //   block [0x830B2730..0x830B2778)
	// 830B2730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B2734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B2738: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B273C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B2740: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B2744: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B2748: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830B274C: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 830B2750: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 830B2754: 4BFFF8D5  bl 0x830b2028
	ctx.lr = 0x830B2758;
	sub_830B2028(ctx, base);
	// 830B2758: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B275C: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830B2760: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B2764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B2768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B276C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B2770: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B2774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B2778 size=20
    let mut pc: u32 = 0x830B2778;
    'dispatch: loop {
        match pc {
            0x830B2778 => {
    //   block [0x830B2778..0x830B278C)
	// 830B2778: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B277C: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B2780: 896A001D  lbz r11, 0x1d(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(29 as u32) ) } as u64;
	// 830B2784: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B2788: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B278C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B278C size=60
    let mut pc: u32 = 0x830B278C;
    'dispatch: loop {
        match pc {
            0x830B278C => {
    //   block [0x830B278C..0x830B27C8)
	// 830B278C: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B2790: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B2794: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 830B2798: 409A0010  bne cr6, 0x830b27a8
	if !ctx.cr[6].eq {
	pc = 0x830B27A8; continue 'dispatch;
	}
	// 830B279C: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 830B27A0: 81040004  lwz r8, 4(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B27A4: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 830B27A8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830B27AC: 41980008  blt cr6, 0x830b27b4
	if ctx.cr[6].lt {
	pc = 0x830B27B4; continue 'dispatch;
	}
	// 830B27B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B27B4: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830B27B8: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B27BC: 4182000C  beq 0x830b27c8
	if ctx.cr[0].eq {
		sub_830B27C8(ctx, base);
		return;
	}
	// 830B27C0: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B27C4: 4800000C  b 0x830b27d0
	sub_830B27C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B27C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B27C8 size=24
    let mut pc: u32 = 0x830B27C8;
    'dispatch: loop {
        match pc {
            0x830B27C8 => {
    //   block [0x830B27C8..0x830B27E0)
	// 830B27C8: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 830B27CC: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B27D0: 896A001D  lbz r11, 0x1d(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(29 as u32) ) } as u64;
	// 830B27D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B27D8: 419AFFB8  beq cr6, 0x830b2790
	if ctx.cr[6].eq {
		sub_830B278C(ctx, base);
		return;
	}
	// 830B27DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B27E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B27E0 size=112
    let mut pc: u32 = 0x830B27E0;
    'dispatch: loop {
        match pc {
            0x830B27E0 => {
    //   block [0x830B27E0..0x830B2850)
	// 830B27E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B27E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B27E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B27EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B27F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B27F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B27F8: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 830B27FC: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 830B2800: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 830B2804: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 830B2808: 38870008  addi r4, r7, 8
	ctx.r[4].s64 = ctx.r[7].s64 + 8;
	// 830B280C: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 830B2810: 90DF0008  stw r6, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 830B2814: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B2818: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830B281C: 81670004  lwz r11, 4(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B2820: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830B2824: 4B41E47D  bl 0x824d0ca0
	ctx.lr = 0x830B2828;
	sub_824D0CA0(ctx, base);
	// 830B2828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B282C: 9BDF001C  stb r30, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 830B2830: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B2834: 997F001D  stb r11, 0x1d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 830B2838: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B283C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B2840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B2844: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B2848: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B284C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B2850 size=84
    let mut pc: u32 = 0x830B2850;
    'dispatch: loop {
        match pc {
            0x830B2850 => {
    //   block [0x830B2850..0x830B28A4)
	// 830B2850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B2854: 480F5919  bl 0x831a816c
	ctx.lr = 0x830B2858;
	sub_831A8130(ctx, base);
	// 830B2858: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B285C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B2860: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830B2864: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830B2868: 4BFFF659  bl 0x830b1ec0
	ctx.lr = 0x830B286C;
	sub_830B1EC0(ctx, base);
	// 830B286C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 830B2870: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830B2874: 396BC418  addi r11, r11, -0x3be8
	ctx.r[11].s64 = ctx.r[11].s64 + -15336;
	// 830B2878: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B287C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B2880: 4BFFF719  bl 0x830b1f98
	ctx.lr = 0x830B2884;
	sub_830B1F98(ctx, base);
	// 830B2884: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B2888: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B288C: 419A000C  beq cr6, 0x830b2898
	if ctx.cr[6].eq {
	pc = 0x830B2898; continue 'dispatch;
	}
	// 830B2890: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B2894: 4BFFF5FD  bl 0x830b1e90
	ctx.lr = 0x830B2898;
	sub_830B1E90(ctx, base);
	// 830B2898: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B289C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B28A0: 480F591C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B28A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B28A8 size=76
    let mut pc: u32 = 0x830B28A8;
    'dispatch: loop {
        match pc {
            0x830B28A8 => {
    //   block [0x830B28A8..0x830B28F4)
	// 830B28A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B28AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B28B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B28B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B28B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B28BC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B28C0: 48006C91  bl 0x830b9550
	ctx.lr = 0x830B28C4;
	sub_830B9550(ctx, base);
	// 830B28C4: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 830B28C8: 41800014  blt 0x830b28dc
	if ctx.cr[0].lt {
	pc = 0x830B28DC; continue 'dispatch;
	}
	// 830B28CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B28D0: 4BFFFB31  bl 0x830b2400
	ctx.lr = 0x830B28D4;
	sub_830B2400(ctx, base);
	// 830B28D4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830B28D8: 48000008  b 0x830b28e0
	pc = 0x830B28E0; continue 'dispatch;
	// 830B28DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830B28E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B28E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B28E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B28EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B28F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B28F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B28F8 size=144
    let mut pc: u32 = 0x830B28F8;
    'dispatch: loop {
        match pc {
            0x830B28F8 => {
    //   block [0x830B28F8..0x830B2988)
	// 830B28F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B28FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B2900: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B2904: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B2908: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B290C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B2910: 3BDF0090  addi r30, r31, 0x90
	ctx.r[30].s64 = ctx.r[31].s64 + 144;
	// 830B2914: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B2918: 4BFFF711  bl 0x830b2028
	ctx.lr = 0x830B291C;
	sub_830B2028(ctx, base);
	// 830B291C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B2920: 4BFFF661  bl 0x830b1f80
	ctx.lr = 0x830B2924;
	sub_830B1F80(ctx, base);
	// 830B2924: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B2928: 41820020  beq 0x830b2948
	if ctx.cr[0].eq {
	pc = 0x830B2948; continue 'dispatch;
	}
	// 830B292C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B2930: 4BFFFCF9  bl 0x830b2628
	ctx.lr = 0x830B2934;
	sub_830B2628(ctx, base);
	// 830B2934: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830B2938: 41820010  beq 0x830b2948
	if ctx.cr[0].eq {
	pc = 0x830B2948; continue 'dispatch;
	}
	// 830B293C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B2940: 4BFFF5E9  bl 0x830b1f28
	ctx.lr = 0x830B2944;
	sub_830B1F28(ctx, base);
	// 830B2944: 4800482D  bl 0x830b7170
	ctx.lr = 0x830B2948;
	sub_830B7170(ctx, base);
	// 830B2948: 807F0098  lwz r3, 0x98(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 830B294C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B2950: 419A0020  beq cr6, 0x830b2970
	if ctx.cr[6].eq {
	pc = 0x830B2970; continue 'dispatch;
	}
	// 830B2954: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B2958: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830B295C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B2960: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B2964: 4E800421  bctrl
	ctx.lr = 0x830B2968;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B2968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B296C: 917F0098  stw r11, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 830B2970: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B2974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B2978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B297C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B2980: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B2984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B2988 size=120
    let mut pc: u32 = 0x830B2988;
    'dispatch: loop {
        match pc {
            0x830B2988 => {
    //   block [0x830B2988..0x830B2A00)
	// 830B2988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B298C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B2990: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B2994: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B2998: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B299C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B29A0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B29A4: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 830B29A8: 396B74B8  addi r11, r11, 0x74b8
	ctx.r[11].s64 = ctx.r[11].s64 + 29880;
	// 830B29AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B29B0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B29B4: 4BFFF50D  bl 0x830b1ec0
	ctx.lr = 0x830B29B8;
	sub_830B1EC0(ctx, base);
	// 830B29B8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 830B29BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B29C0: 396BEDD8  addi r11, r11, -0x1228
	ctx.r[11].s64 = ctx.r[11].s64 + -4648;
	// 830B29C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B29C8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830B29CC: 4BFFF5CD  bl 0x830b1f98
	ctx.lr = 0x830B29D0;
	sub_830B1F98(ctx, base);
	// 830B29D0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B29D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B29D8: 4BFFF6D9  bl 0x830b20b0
	ctx.lr = 0x830B29DC;
	sub_830B20B0(ctx, base);
	// 830B29DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B29E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B29E4: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830B29E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B29EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B29F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B29F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B29F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B29FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B2A00 size=96
    let mut pc: u32 = 0x830B2A00;
    'dispatch: loop {
        match pc {
            0x830B2A00 => {
    //   block [0x830B2A00..0x830B2A60)
	// 830B2A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B2A04: 480F5769  bl 0x831a816c
	ctx.lr = 0x830B2A08;
	sub_831A8130(ctx, base);
	// 830B2A08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B2A0C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830B2A10: 83A50000  lwz r29, 0(r5)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B2A14: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B2A18: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830B2A1C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B2A20: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B2A24: 419A002C  beq cr6, 0x830b2a50
	if ctx.cr[6].eq {
	pc = 0x830B2A50; continue 'dispatch;
	}
	// 830B2A28: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B2A2C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B2A30: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830B2A34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B2A38: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B2A3C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830B2A40: 4B20D829  bl 0x822c0268
	ctx.lr = 0x830B2A44;
	sub_822C0268(ctx, base);
	// 830B2A44: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B2A48: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 830B2A4C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830B2A50: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 830B2A54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B2A58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B2A5C: 480F5760  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B2A60 size=92
    let mut pc: u32 = 0x830B2A60;
    'dispatch: loop {
        match pc {
            0x830B2A60 => {
    //   block [0x830B2A60..0x830B2ABC)
	// 830B2A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B2A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B2A68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B2A6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B2A70: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830B2A74: 4B7FF1ED  bl 0x828b1c60
	ctx.lr = 0x830B2A78;
	sub_828B1C60(ctx, base);
	// 830B2A78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B2A7C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B2A80: 41820008  beq 0x830b2a88
	if ctx.cr[0].eq {
	pc = 0x830B2A88; continue 'dispatch;
	}
	// 830B2A84: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830B2A88: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B2A8C: 41820008  beq 0x830b2a94
	if ctx.cr[0].eq {
	pc = 0x830B2A94; continue 'dispatch;
	}
	// 830B2A90: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830B2A94: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B2A98: 41820008  beq 0x830b2aa0
	if ctx.cr[0].eq {
	pc = 0x830B2AA0; continue 'dispatch;
	}
	// 830B2A9C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830B2AA0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830B2AA4: 9943001D  stb r10, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[10].u8 ) };
	// 830B2AA8: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 830B2AAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B2AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B2AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B2AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B2AC0 size=84
    let mut pc: u32 = 0x830B2AC0;
    'dispatch: loop {
        match pc {
            0x830B2AC0 => {
    //   block [0x830B2AC0..0x830B2B14)
	// 830B2AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B2AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B2AC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B2ACC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B2AD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B2AD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B2AD8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B2ADC: 4BFFFEAD  bl 0x830b2988
	ctx.lr = 0x830B2AE0;
	sub_830B2988(ctx, base);
	// 830B2AE0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B2AE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B2AE8: 396B7538  addi r11, r11, 0x7538
	ctx.r[11].s64 = ctx.r[11].s64 + 30008;
	// 830B2AEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B2AF0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B2AF4: 480069A5  bl 0x830b9498
	ctx.lr = 0x830B2AF8;
	sub_830B9498(ctx, base);
	// 830B2AF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B2AFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B2B00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B2B04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B2B08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B2B0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B2B10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B2B18 size=116
    let mut pc: u32 = 0x830B2B18;
    'dispatch: loop {
        match pc {
            0x830B2B18 => {
    //   block [0x830B2B18..0x830B2B8C)
	// 830B2B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B2B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B2B20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B2B24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B2B28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B2B2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B2B30: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B2B34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B2B38: 396B74B8  addi r11, r11, 0x74b8
	ctx.r[11].s64 = ctx.r[11].s64 + 29880;
	// 830B2B3C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 830B2B40: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830B2B44: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B2B48: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B2B4C: 4BFFF38D  bl 0x830b1ed8
	ctx.lr = 0x830B2B50;
	sub_830B1ED8(ctx, base);
	// 830B2B50: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B2B54: 4182001C  beq 0x830b2b70
	if ctx.cr[0].eq {
	pc = 0x830B2B70; continue 'dispatch;
	}
	// 830B2B58: 480020F1  bl 0x830b4c48
	ctx.lr = 0x830B2B5C;
	sub_830B4C48(ctx, base);
	// 830B2B5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B2B60: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B2B64: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830B2B68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B2B6C: 4E800421  bctrl
	ctx.lr = 0x830B2B70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B2B70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B2B74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B2B78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B2B7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B2B80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B2B84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B2B88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B2B90 size=96
    let mut pc: u32 = 0x830B2B90;
    'dispatch: loop {
        match pc {
            0x830B2B90 => {
    //   block [0x830B2B90..0x830B2BF0)
	// 830B2B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B2B94: 480F55D9  bl 0x831a816c
	ctx.lr = 0x830B2B98;
	sub_831A8130(ctx, base);
	// 830B2B98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B2B9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B2BA0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830B2BA4: 4BFFFDE5  bl 0x830b2988
	ctx.lr = 0x830B2BA8;
	sub_830B2988(ctx, base);
	// 830B2BA8: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B2BAC: 3BDF001C  addi r30, r31, 0x1c
	ctx.r[30].s64 = ctx.r[31].s64 + 28;
	// 830B2BB0: 396B7540  addi r11, r11, 0x7540
	ctx.r[11].s64 = ctx.r[11].s64 + 30016;
	// 830B2BB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B2BB8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B2BBC: 4BFFF305  bl 0x830b1ec0
	ctx.lr = 0x830B2BC0;
	sub_830B1EC0(ctx, base);
	// 830B2BC0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B2BC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B2BC8: 396B74C0  addi r11, r11, 0x74c0
	ctx.r[11].s64 = ctx.r[11].s64 + 29888;
	// 830B2BCC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B2BD0: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 830B2BD4: 4BFFF3C5  bl 0x830b1f98
	ctx.lr = 0x830B2BD8;
	sub_830B1F98(ctx, base);
	// 830B2BD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B2BDC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830B2BE0: 48009D71  bl 0x830bc950
	ctx.lr = 0x830B2BE4;
	sub_830BC950(ctx, base);
	// 830B2BE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B2BE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B2BEC: 480F55D0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B2BF0 size=76
    let mut pc: u32 = 0x830B2BF0;
    'dispatch: loop {
        match pc {
            0x830B2BF0 => {
    //   block [0x830B2BF0..0x830B2C3C)
	// 830B2BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B2BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B2BF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B2BFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B2C00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B2C04: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 830B2C08: 4BFFF2D1  bl 0x830b1ed8
	ctx.lr = 0x830B2C0C;
	sub_830B1ED8(ctx, base);
	// 830B2C0C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B2C10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B2C14: 396B74B8  addi r11, r11, 0x74b8
	ctx.r[11].s64 = ctx.r[11].s64 + 29880;
	// 830B2C18: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830B2C1C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 830B2C20: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B2C24: 4BFFF2B5  bl 0x830b1ed8
	ctx.lr = 0x830B2C28;
	sub_830B1ED8(ctx, base);
	// 830B2C28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B2C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B2C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B2C34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B2C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B2C40 size=92
    let mut pc: u32 = 0x830B2C40;
    'dispatch: loop {
        match pc {
            0x830B2C40 => {
    //   block [0x830B2C40..0x830B2C9C)
	// 830B2C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B2C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B2C48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B2C4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B2C50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B2C54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B2C58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B2C5C: 4BFFFF95  bl 0x830b2bf0
	ctx.lr = 0x830B2C60;
	sub_830B2BF0(ctx, base);
	// 830B2C60: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B2C64: 4182001C  beq 0x830b2c80
	if ctx.cr[0].eq {
	pc = 0x830B2C80; continue 'dispatch;
	}
	// 830B2C68: 48001FE1  bl 0x830b4c48
	ctx.lr = 0x830B2C6C;
	sub_830B4C48(ctx, base);
	// 830B2C6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B2C70: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B2C74: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830B2C78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B2C7C: 4E800421  bctrl
	ctx.lr = 0x830B2C80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B2C80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B2C84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B2C88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B2C8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B2C90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B2C94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B2C98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B2CA0 size=92
    let mut pc: u32 = 0x830B2CA0;
    'dispatch: loop {
        match pc {
            0x830B2CA0 => {
    //   block [0x830B2CA0..0x830B2CFC)
	// 830B2CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B2CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B2CA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B2CAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B2CB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B2CB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B2CB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B2CBC: 4BFFFCCD  bl 0x830b2988
	ctx.lr = 0x830B2CC0;
	sub_830B2988(ctx, base);
	// 830B2CC0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B2CC4: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 830B2CC8: 396B7548  addi r11, r11, 0x7548
	ctx.r[11].s64 = ctx.r[11].s64 + 30024;
	// 830B2CCC: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 830B2CD0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B2CD4: 4BFFF355  bl 0x830b2028
	ctx.lr = 0x830B2CD8;
	sub_830B2028(ctx, base);
	// 830B2CD8: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B2CDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B2CE0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830B2CE4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B2CE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B2CEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B2CF0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B2CF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B2CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B2D00 size=304
    let mut pc: u32 = 0x830B2D00;
    'dispatch: loop {
        match pc {
            0x830B2D00 => {
    //   block [0x830B2D00..0x830B2E30)
	// 830B2D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B2D04: 480F5469  bl 0x831a816c
	ctx.lr = 0x830B2D08;
	sub_831A8130(ctx, base);
	// 830B2D08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B2D0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B2D10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B2D14: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 830B2D18: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 830B2D1C: 4BFFF30D  bl 0x830b2028
	ctx.lr = 0x830B2D20;
	sub_830B2028(ctx, base);
	// 830B2D20: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B2D24: 807F0070  lwz r3, 0x70(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 830B2D28: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830B2D2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B2D30: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830B2D34: 419A001C  beq cr6, 0x830b2d50
	if ctx.cr[6].eq {
	pc = 0x830B2D50; continue 'dispatch;
	}
	// 830B2D38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B2D3C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B2D40: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B2D44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B2D48: 4E800421  bctrl
	ctx.lr = 0x830B2D4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B2D4C: 93BF0070  stw r29, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[29].u32 ) };
	// 830B2D50: 809E0070  lwz r4, 0x70(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(112 as u32) ) } as u64;
	// 830B2D54: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830B2D58: 419A0020  beq cr6, 0x830b2d78
	if ctx.cr[6].eq {
	pc = 0x830B2D78; continue 'dispatch;
	}
	// 830B2D5C: 817F00DC  lwz r11, 0xdc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 830B2D60: 346B0028  addic. r3, r11, 0x28
	ctx.xer.ca = (ctx.r[11].u32 > (!(40 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 830B2D64: 4182000C  beq 0x830b2d70
	if ctx.cr[0].eq {
	pc = 0x830B2D70; continue 'dispatch;
	}
	// 830B2D68: 4BFFFD59  bl 0x830b2ac0
	ctx.lr = 0x830B2D6C;
	sub_830B2AC0(ctx, base);
	// 830B2D6C: 48000008  b 0x830b2d74
	pc = 0x830B2D74; continue 'dispatch;
	// 830B2D70: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B2D74: 907F0070  stw r3, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 830B2D78: 807F0074  lwz r3, 0x74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 830B2D7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B2D80: 419A001C  beq cr6, 0x830b2d9c
	if ctx.cr[6].eq {
	pc = 0x830B2D9C; continue 'dispatch;
	}
	// 830B2D84: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B2D88: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B2D8C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B2D90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B2D94: 4E800421  bctrl
	ctx.lr = 0x830B2D98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B2D98: 93BF0074  stw r29, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 830B2D9C: 809E0074  lwz r4, 0x74(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(116 as u32) ) } as u64;
	// 830B2DA0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830B2DA4: 419A0020  beq cr6, 0x830b2dc4
	if ctx.cr[6].eq {
	pc = 0x830B2DC4; continue 'dispatch;
	}
	// 830B2DA8: 807F00DC  lwz r3, 0xdc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 830B2DAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B2DB0: 419A000C  beq cr6, 0x830b2dbc
	if ctx.cr[6].eq {
	pc = 0x830B2DBC; continue 'dispatch;
	}
	// 830B2DB4: 4BFFFEED  bl 0x830b2ca0
	ctx.lr = 0x830B2DB8;
	sub_830B2CA0(ctx, base);
	// 830B2DB8: 48000008  b 0x830b2dc0
	pc = 0x830B2DC0; continue 'dispatch;
	// 830B2DBC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B2DC0: 907F0074  stw r3, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 830B2DC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B2DC8: 4BFFF749  bl 0x830b2510
	ctx.lr = 0x830B2DCC;
	sub_830B2510(ctx, base);
	// 830B2DCC: 389E0090  addi r4, r30, 0x90
	ctx.r[4].s64 = ctx.r[30].s64 + 144;
	// 830B2DD0: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 830B2DD4: 4BFFF255  bl 0x830b2028
	ctx.lr = 0x830B2DD8;
	sub_830B2028(ctx, base);
	// 830B2DD8: C01E007C  lfs f0, 0x7c(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B2DDC: D01F007C  stfs f0, 0x7c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 830B2DE0: C01E0080  lfs f0, 0x80(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B2DE4: D01F0080  stfs f0, 0x80(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 830B2DE8: C01E0084  lfs f0, 0x84(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B2DEC: D01F0084  stfs f0, 0x84(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 830B2DF0: C01E0088  lfs f0, 0x88(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B2DF4: D01F0088  stfs f0, 0x88(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 830B2DF8: C01E008C  lfs f0, 0x8c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B2DFC: D01F008C  stfs f0, 0x8c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 830B2E00: 817E00B8  lwz r11, 0xb8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(184 as u32) ) } as u64;
	// 830B2E04: 917F00B8  stw r11, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[11].u32 ) };
	// 830B2E08: 817E009C  lwz r11, 0x9c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(156 as u32) ) } as u64;
	// 830B2E0C: 917F009C  stw r11, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 830B2E10: 817E00B0  lwz r11, 0xb0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(176 as u32) ) } as u64;
	// 830B2E14: 917F00B0  stw r11, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 830B2E18: 817E00B4  lwz r11, 0xb4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(180 as u32) ) } as u64;
	// 830B2E1C: 917F00B4  stw r11, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 830B2E20: 817E00B8  lwz r11, 0xb8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(184 as u32) ) } as u64;
	// 830B2E24: 917F00B8  stw r11, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[11].u32 ) };
	// 830B2E28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B2E2C: 480F5390  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B2E30 size=176
    let mut pc: u32 = 0x830B2E30;
    'dispatch: loop {
        match pc {
            0x830B2E30 => {
    //   block [0x830B2E30..0x830B2EE0)
	// 830B2E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B2E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B2E38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B2E3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B2E40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B2E44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B2E48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B2E4C: 807F0098  lwz r3, 0x98(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 830B2E50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B2E54: 419A0020  beq cr6, 0x830b2e74
	if ctx.cr[6].eq {
	pc = 0x830B2E74; continue 'dispatch;
	}
	// 830B2E58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B2E5C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830B2E60: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B2E64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B2E68: 4E800421  bctrl
	ctx.lr = 0x830B2E6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B2E6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B2E70: 917F0098  stw r11, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 830B2E74: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B2E78: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 830B2E7C: 4BFFF235  bl 0x830b20b0
	ctx.lr = 0x830B2E80;
	sub_830B20B0(ctx, base);
	// 830B2E80: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830B2E84: 419A0044  beq cr6, 0x830b2ec8
	if ctx.cr[6].eq {
	pc = 0x830B2EC8; continue 'dispatch;
	}
	// 830B2E88: 48001DC1  bl 0x830b4c48
	ctx.lr = 0x830B2E8C;
	sub_830B4C48(ctx, base);
	// 830B2E8C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B2E90: 38C00469  li r6, 0x469
	ctx.r[6].s64 = 1129;
	// 830B2E94: 38AB74D0  addi r5, r11, 0x74d0
	ctx.r[5].s64 = ctx.r[11].s64 + 29904;
	// 830B2E98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B2E9C: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 830B2EA0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B2EA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B2EA8: 4E800421  bctrl
	ctx.lr = 0x830B2EAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B2EAC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B2EB0: 41820010  beq 0x830b2ec0
	if ctx.cr[0].eq {
	pc = 0x830B2EC0; continue 'dispatch;
	}
	// 830B2EB4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B2EB8: 4BFFFCD9  bl 0x830b2b90
	ctx.lr = 0x830B2EBC;
	sub_830B2B90(ctx, base);
	// 830B2EBC: 48000008  b 0x830b2ec4
	pc = 0x830B2EC4; continue 'dispatch;
	// 830B2EC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830B2EC4: 907F0098  stw r3, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[3].u32 ) };
	// 830B2EC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B2ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B2ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B2ED4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B2ED8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B2EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B2EE0 size=200
    let mut pc: u32 = 0x830B2EE0;
    'dispatch: loop {
        match pc {
            0x830B2EE0 => {
    //   block [0x830B2EE0..0x830B2FA8)
	// 830B2EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B2EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B2EE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B2EEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B2EF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B2EF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B2EF8: 807E0098  lwz r3, 0x98(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(152 as u32) ) } as u64;
	// 830B2EFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B2F00: 409A0090  bne cr6, 0x830b2f90
	if !ctx.cr[6].eq {
	pc = 0x830B2F90; continue 'dispatch;
	}
	// 830B2F04: 48001D45  bl 0x830b4c48
	ctx.lr = 0x830B2F08;
	sub_830B4C48(ctx, base);
	// 830B2F08: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B2F0C: 38C0047F  li r6, 0x47f
	ctx.r[6].s64 = 1151;
	// 830B2F10: 38AB74D0  addi r5, r11, 0x74d0
	ctx.r[5].s64 = ctx.r[11].s64 + 29904;
	// 830B2F14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B2F18: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 830B2F1C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B2F20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B2F24: 4E800421  bctrl
	ctx.lr = 0x830B2F28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B2F28: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B2F2C: 41820030  beq 0x830b2f5c
	if ctx.cr[0].eq {
	pc = 0x830B2F5C; continue 'dispatch;
	}
	// 830B2F30: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B2F34: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B2F38: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 830B2F3C: 4198000C  blt cr6, 0x830b2f48
	if ctx.cr[6].lt {
	pc = 0x830B2F48; continue 'dispatch;
	}
	// 830B2F40: C02B0040  lfs f1, 0x40(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B2F44: 4800000C  b 0x830b2f50
	pc = 0x830B2F50; continue 'dispatch;
	// 830B2F48: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B2F4C: C02B750C  lfs f1, 0x750c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29964 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B2F50: 48009CC9  bl 0x830bcc18
	ctx.lr = 0x830B2F54;
	sub_830BCC18(ctx, base);
	// 830B2F54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B2F58: 48000008  b 0x830b2f60
	pc = 0x830B2F60; continue 'dispatch;
	// 830B2F5C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830B2F60: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B2F64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B2F68: 4BFFFEC9  bl 0x830b2e30
	ctx.lr = 0x830B2F6C;
	sub_830B2E30(ctx, base);
	// 830B2F6C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830B2F70: 419A001C  beq cr6, 0x830b2f8c
	if ctx.cr[6].eq {
	pc = 0x830B2F8C; continue 'dispatch;
	}
	// 830B2F74: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B2F78: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830B2F7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B2F80: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B2F84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B2F88: 4E800421  bctrl
	ctx.lr = 0x830B2F8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B2F8C: 807E0098  lwz r3, 0x98(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(152 as u32) ) } as u64;
	// 830B2F90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B2F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B2F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B2F9C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B2FA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B2FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B2FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B2FA8 size=84
    let mut pc: u32 = 0x830B2FA8;
    'dispatch: loop {
        match pc {
            0x830B2FA8 => {
    //   block [0x830B2FA8..0x830B2FFC)
	// 830B2FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B2FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B2FB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B2FB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B2FB8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830B2FBC: 4BFFFF25  bl 0x830b2ee0
	ctx.lr = 0x830B2FC0;
	sub_830B2EE0(ctx, base);
	// 830B2FC0: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B2FC4: 93EB0028  stw r31, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[31].u32 ) };
	// 830B2FC8: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B2FCC: 814B007C  lwz r10, 0x7c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830B2FD0: 554A003C  rlwinm r10, r10, 0, 0, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830B2FD4: 914B007C  stw r10, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 830B2FD8: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B2FDC: 814B007C  lwz r10, 0x7c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830B2FE0: 554A0356  rlwinm r10, r10, 0, 0xd, 0xb
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830B2FE4: 914B007C  stw r10, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 830B2FE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B2FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B2FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B2FF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B2FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B3000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B3000 size=156
    let mut pc: u32 = 0x830B3000;
    'dispatch: loop {
        match pc {
            0x830B3000 => {
    //   block [0x830B3000..0x830B309C)
	// 830B3000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B3004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B3008: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B300C: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 830B3010: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 830B3014: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B3018: D021008C  stfs f1, 0x8c(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 830B301C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B3020: D0410094  stfs f2, 0x94(r1)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 830B3024: 38810094  addi r4, r1, 0x94
	ctx.r[4].s64 = ctx.r[1].s64 + 148;
	// 830B3028: 3861008C  addi r3, r1, 0x8c
	ctx.r[3].s64 = ctx.r[1].s64 + 140;
	// 830B302C: 480051CD  bl 0x830b81f8
	ctx.lr = 0x830B3030;
	sub_830B81F8(ctx, base);
	// 830B3030: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B3034: C3E10094  lfs f31, 0x94(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 830B3038: C3C1008C  lfs f30, 0x8c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 830B303C: 4BFFFEA5  bl 0x830b2ee0
	ctx.lr = 0x830B3040;
	sub_830B2EE0(ctx, base);
	// 830B3040: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B3044: D3CB002C  stfs f30, 0x2c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 830B3048: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B304C: D3EB0030  stfs f31, 0x30(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 830B3050: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B3054: 814B007C  lwz r10, 0x7c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830B3058: 554A07FA  rlwinm r10, r10, 0, 0x1f, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830B305C: 914B007C  stw r10, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 830B3060: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B3064: 814B007C  lwz r10, 0x7c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830B3068: 554A07B8  rlwinm r10, r10, 0, 0x1e, 0x1c
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830B306C: 914B007C  stw r10, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 830B3070: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B3074: 814B007C  lwz r10, 0x7c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830B3078: 554A0356  rlwinm r10, r10, 0, 0xd, 0xb
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830B307C: 914B007C  stw r10, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 830B3080: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B3084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B3088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B308C: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 830B3090: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B3094: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B3098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B30A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B30A0 size=100
    let mut pc: u32 = 0x830B30A0;
    'dispatch: loop {
        match pc {
            0x830B30A0 => {
    //   block [0x830B30A0..0x830B3104)
	// 830B30A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B30A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B30A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B30AC: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 830B30B0: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 830B30B4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B30B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B30BC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 830B30C0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B30C4: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 830B30C8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 830B30CC: 4800531D  bl 0x830b83e8
	ctx.lr = 0x830B30D0;
	sub_830B83E8(ctx, base);
	// 830B30D0: C0010050  lfs f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B30D4: C1A10054  lfs f13, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B30D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B30DC: EC40F02A  fadds f2, f0, f30
	ctx.f[2].f64 = ((ctx.f[0].f64 + ctx.f[30].f64) as f32) as f64;
	// 830B30E0: EC2DF82A  fadds f1, f13, f31
	ctx.f[1].f64 = ((ctx.f[13].f64 + ctx.f[31].f64) as f32) as f64;
	// 830B30E4: 4BFFFF1D  bl 0x830b3000
	ctx.lr = 0x830B30E8;
	sub_830B3000(ctx, base);
	// 830B30E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B30EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B30F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B30F4: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 830B30F8: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B30FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B3100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B3108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B3108 size=120
    let mut pc: u32 = 0x830B3108;
    'dispatch: loop {
        match pc {
            0x830B3108 => {
    //   block [0x830B3108..0x830B3180)
	// 830B3108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B310C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B3110: DBC1FFE8  stfd f30, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[30].u64 ) };
	// 830B3114: DBE1FFF0  stfd f31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 830B3118: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B311C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 830B3120: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 830B3124: 4BFFFDBD  bl 0x830b2ee0
	ctx.lr = 0x830B3128;
	sub_830B2EE0(ctx, base);
	// 830B3128: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B312C: D3EB002C  stfs f31, 0x2c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 830B3130: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B3134: D3CB0030  stfs f30, 0x30(r11)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 830B3138: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B313C: 814B007C  lwz r10, 0x7c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830B3140: 554A07FA  rlwinm r10, r10, 0, 0x1f, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830B3144: 914B007C  stw r10, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 830B3148: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B314C: 814B007C  lwz r10, 0x7c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830B3150: 554A07B8  rlwinm r10, r10, 0, 0x1e, 0x1c
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830B3154: 914B007C  stw r10, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 830B3158: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B315C: 814B007C  lwz r10, 0x7c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830B3160: 554A0356  rlwinm r10, r10, 0, 0xd, 0xb
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830B3164: 914B007C  stw r10, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 830B3168: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B316C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B3170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B3174: CBC1FFE8  lfd f30, -0x18(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B3178: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B317C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B3180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B3180 size=164
    let mut pc: u32 = 0x830B3180;
    'dispatch: loop {
        match pc {
            0x830B3180 => {
    //   block [0x830B3180..0x830B3224)
	// 830B3180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B3184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B3188: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B318C: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 830B3190: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 830B3194: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B3198: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B319C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 830B31A0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B31A4: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 830B31A8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 830B31AC: 4800512D  bl 0x830b82d8
	ctx.lr = 0x830B31B0;
	sub_830B82D8(ctx, base);
	// 830B31B0: C0010050  lfs f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B31B4: C1A10054  lfs f13, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B31B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B31BC: EFC0F02A  fadds f30, f0, f30
	ctx.f[30].f64 = ((ctx.f[0].f64 + ctx.f[30].f64) as f32) as f64;
	// 830B31C0: EFEDF82A  fadds f31, f13, f31
	ctx.f[31].f64 = ((ctx.f[13].f64 + ctx.f[31].f64) as f32) as f64;
	// 830B31C4: 4BFFFD1D  bl 0x830b2ee0
	ctx.lr = 0x830B31C8;
	sub_830B2EE0(ctx, base);
	// 830B31C8: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B31CC: D3EB002C  stfs f31, 0x2c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 830B31D0: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B31D4: D3CB0030  stfs f30, 0x30(r11)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 830B31D8: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B31DC: 814B007C  lwz r10, 0x7c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830B31E0: 554A07FA  rlwinm r10, r10, 0, 0x1f, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830B31E4: 914B007C  stw r10, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 830B31E8: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B31EC: 814B007C  lwz r10, 0x7c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830B31F0: 554A07B8  rlwinm r10, r10, 0, 0x1e, 0x1c
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830B31F4: 914B007C  stw r10, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 830B31F8: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B31FC: 814B007C  lwz r10, 0x7c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830B3200: 554A0356  rlwinm r10, r10, 0, 0xd, 0xb
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830B3204: 914B007C  stw r10, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 830B3208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B320C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B3210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B3214: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 830B3218: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B321C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B3220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B3228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B3228 size=84
    let mut pc: u32 = 0x830B3228;
    'dispatch: loop {
        match pc {
            0x830B3228 => {
    //   block [0x830B3228..0x830B327C)
	// 830B3228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B322C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B3230: DBE1FFF0  stfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 830B3234: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B3238: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 830B323C: 4BFFFCA5  bl 0x830b2ee0
	ctx.lr = 0x830B3240;
	sub_830B2EE0(ctx, base);
	// 830B3240: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B3244: D3EB0034  stfs f31, 0x34(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 830B3248: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B324C: 814B007C  lwz r10, 0x7c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830B3250: 554A0776  rlwinm r10, r10, 0, 0x1d, 0x1b
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830B3254: 914B007C  stw r10, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 830B3258: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B325C: 814B007C  lwz r10, 0x7c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830B3260: 554A0356  rlwinm r10, r10, 0, 0xd, 0xb
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830B3264: 914B007C  stw r10, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 830B3268: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B326C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B3270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B3274: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B3278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B3280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B3280 size=120
    let mut pc: u32 = 0x830B3280;
    'dispatch: loop {
        match pc {
            0x830B3280 => {
    //   block [0x830B3280..0x830B32F8)
	// 830B3280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B3284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B3288: DBC1FFE8  stfd f30, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[30].u64 ) };
	// 830B328C: DBE1FFF0  stfd f31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 830B3290: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B3294: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 830B3298: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 830B329C: 4BFFFC45  bl 0x830b2ee0
	ctx.lr = 0x830B32A0;
	sub_830B2EE0(ctx, base);
	// 830B32A0: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B32A4: D3EB0038  stfs f31, 0x38(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 830B32A8: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B32AC: D3CB003C  stfs f30, 0x3c(r11)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 830B32B0: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B32B4: 814B007C  lwz r10, 0x7c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830B32B8: 554A0734  rlwinm r10, r10, 0, 0x1c, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830B32BC: 914B007C  stw r10, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 830B32C0: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B32C4: 814B007C  lwz r10, 0x7c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830B32C8: 554A06F2  rlwinm r10, r10, 0, 0x1b, 0x19
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830B32CC: 914B007C  stw r10, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 830B32D0: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B32D4: 814B007C  lwz r10, 0x7c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830B32D8: 554A0356  rlwinm r10, r10, 0, 0xd, 0xb
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830B32DC: 914B007C  stw r10, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 830B32E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B32E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B32E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B32EC: CBC1FFE8  lfd f30, -0x18(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B32F0: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B32F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B32F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B32F8 size=88
    let mut pc: u32 = 0x830B32F8;
    'dispatch: loop {
        match pc {
            0x830B32F8 => {
    //   block [0x830B32F8..0x830B3350)
	// 830B32F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B32FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B3300: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B3304: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B3308: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830B330C: 4BFFFBD5  bl 0x830b2ee0
	ctx.lr = 0x830B3310;
	sub_830B2EE0(ctx, base);
	// 830B3310: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3314: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B3318: 916A0044  stw r11, 0x44(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 830B331C: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B3320: 814B007C  lwz r10, 0x7c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830B3324: 554A066E  rlwinm r10, r10, 0, 0x19, 0x17
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830B3328: 914B007C  stw r10, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 830B332C: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B3330: 814B007C  lwz r10, 0x7c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830B3334: 554A0356  rlwinm r10, r10, 0, 0xd, 0xb
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830B3338: 914B007C  stw r10, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 830B333C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B3340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B3344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B3348: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B334C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B3350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B3350 size=192
    let mut pc: u32 = 0x830B3350;
    'dispatch: loop {
        match pc {
            0x830B3350 => {
    //   block [0x830B3350..0x830B3410)
	// 830B3350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B3354: 480F4E09  bl 0x831a815c
	ctx.lr = 0x830B3358;
	sub_831A8130(ctx, base);
	// 830B3358: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B335C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B3360: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830B3364: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 830B3368: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 830B336C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 830B3370: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 830B3374: 4BFFEFD5  bl 0x830b2348
	ctx.lr = 0x830B3378;
	sub_830B2348(ctx, base);
	// 830B3378: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 830B337C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B3380: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B3384: 4BFFED2D  bl 0x830b20b0
	ctx.lr = 0x830B3388;
	sub_830B20B0(ctx, base);
	// 830B3388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B338C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830B3390: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830B3394: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B3398: 4BFFEC91  bl 0x830b2028
	ctx.lr = 0x830B339C;
	sub_830B2028(ctx, base);
	// 830B339C: 939F000C  stw r28, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 830B33A0: 933F00CC  stw r25, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[25].u32 ) };
	// 830B33A4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830B33A8: 419A0024  beq cr6, 0x830b33cc
	if ctx.cr[6].eq {
	pc = 0x830B33CC; continue 'dispatch;
	}
	// 830B33AC: 807F00DC  lwz r3, 0xdc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 830B33B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B33B4: 419A0010  beq cr6, 0x830b33c4
	if ctx.cr[6].eq {
	pc = 0x830B33C4; continue 'dispatch;
	}
	// 830B33B8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830B33BC: 4BFFF8E5  bl 0x830b2ca0
	ctx.lr = 0x830B33C0;
	sub_830B2CA0(ctx, base);
	// 830B33C0: 48000008  b 0x830b33c8
	pc = 0x830B33C8; continue 'dispatch;
	// 830B33C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830B33C8: 907F0074  stw r3, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 830B33CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B33D0: 4BFFF141  bl 0x830b2510
	ctx.lr = 0x830B33D4;
	sub_830B2510(ctx, base);
	// 830B33D4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 830B33D8: C00B08A8  lfs f0, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B33DC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B33E0: C1AB000C  lfs f13, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B33E4: D1BF0088  stfs f13, 0x88(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 830B33E8: C18B0010  lfs f12, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 830B33EC: D19F008C  stfs f12, 0x8c(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 830B33F0: D1BF007C  stfs f13, 0x7c(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 830B33F4: D1BF0080  stfs f13, 0x80(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 830B33F8: C1BC0004  lfs f13, 4(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B33FC: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 830B3400: D01F00BC  stfs f0, 0xbc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 830B3404: 937F00D8  stw r27, 0xd8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[27].u32 ) };
	// 830B3408: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830B340C: 480F4DA0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B3410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B3410 size=140
    let mut pc: u32 = 0x830B3410;
    'dispatch: loop {
        match pc {
            0x830B3410 => {
    //   block [0x830B3410..0x830B349C)
	// 830B3410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B3414: 480F4D59  bl 0x831a816c
	ctx.lr = 0x830B3418;
	sub_831A8130(ctx, base);
	// 830B3418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B341C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830B3420: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 830B3424: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B3428: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B342C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B3430: 4BFFF349  bl 0x830b2778
	ctx.lr = 0x830B3434;
	sub_830B2778(ctx, base);
	// 830B3434: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3438: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830B343C: 7F034840  cmplw cr6, r3, r9
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830B3440: 419A0040  beq cr6, 0x830b3480
	if ctx.cr[6].eq {
	pc = 0x830B3480; continue 'dispatch;
	}
	// 830B3444: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3448: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B344C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 830B3450: 409A0010  bne cr6, 0x830b3460
	if !ctx.cr[6].eq {
	pc = 0x830B3460; continue 'dispatch;
	}
	// 830B3454: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3458: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 830B345C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 830B3460: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830B3464: 41980008  blt cr6, 0x830b346c
	if ctx.cr[6].lt {
	pc = 0x830B346C; continue 'dispatch;
	}
	// 830B3468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B346C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830B3470: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B3474: 4082000C  bne 0x830b3480
	if !ctx.cr[0].eq {
	pc = 0x830B3480; continue 'dispatch;
	}
	// 830B3478: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830B347C: 4800000C  b 0x830b3488
	pc = 0x830B3488; continue 'dispatch;
	// 830B3480: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 830B3484: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 830B3488: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B348C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B3490: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B3494: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B3498: 480F4D24  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B34A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B34A0 size=84
    let mut pc: u32 = 0x830B34A0;
    'dispatch: loop {
        match pc {
            0x830B34A0 => {
    //   block [0x830B34A0..0x830B34F4)
	// 830B34A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B34A4: 480F4CC9  bl 0x831a816c
	ctx.lr = 0x830B34A8;
	sub_831A8130(ctx, base);
	// 830B34A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B34AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B34B0: 83A40000  lwz r29, 0(r4)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B34B4: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B34B8: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B34BC: 48000028  b 0x830b34e4
	pc = 0x830B34E4; continue 'dispatch;
	// 830B34C0: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B34C4: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 830B34C8: 409A0018  bne cr6, 0x830b34e0
	if !ctx.cr[6].eq {
	pc = 0x830B34E0; continue 'dispatch;
	}
	// 830B34CC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B34D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B34D4: 4BFFF52D  bl 0x830b2a00
	ctx.lr = 0x830B34D8;
	sub_830B2A00(ctx, base);
	// 830B34D8: 80A30000  lwz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B34DC: 48000008  b 0x830b34e4
	pc = 0x830B34E4; continue 'dispatch;
	// 830B34E0: 80A50000  lwz r5, 0(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B34E4: 7F05F840  cmplw cr6, r5, r31
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[31].u32, &mut ctx.xer);
	// 830B34E8: 409AFFD8  bne cr6, 0x830b34c0
	if !ctx.cr[6].eq {
	pc = 0x830B34C0; continue 'dispatch;
	}
	// 830B34EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B34F0: 480F4CCC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B34F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B34F8 size=88
    let mut pc: u32 = 0x830B34F8;
    'dispatch: loop {
        match pc {
            0x830B34F8 => {
    //   block [0x830B34F8..0x830B3550)
	// 830B34F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B34FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B3500: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B3504: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B3508: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B350C: 4BFFF555  bl 0x830b2a60
	ctx.lr = 0x830B3510;
	sub_830B2A60(ctx, base);
	// 830B3510: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830B3514: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 830B3518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B351C: 9963001D  stb r11, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 830B3520: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3524: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830B3528: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B352C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B3530: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3534: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830B3538: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 830B353C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B3540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B3544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B3548: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B354C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B3550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B3550 size=76
    let mut pc: u32 = 0x830B3550;
    'dispatch: loop {
        match pc {
            0x830B3550 => {
    //   block [0x830B3550..0x830B359C)
	// 830B3550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B3554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B3558: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B355C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B3560: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B3564: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B3568: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 830B356C: 396B7550  addi r11, r11, 0x7550
	ctx.r[11].s64 = ctx.r[11].s64 + 30032;
	// 830B3570: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B3574: 48003FB5  bl 0x830b7528
	ctx.lr = 0x830B3578;
	sub_830B7528(ctx, base);
	// 830B3578: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B357C: 4B20CCED  bl 0x822c0268
	ctx.lr = 0x830B3580;
	sub_822C0268(ctx, base);
	// 830B3580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B3584: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830B3588: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B358C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B3590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B3594: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B3598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B35A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B35A0 size=12
    let mut pc: u32 = 0x830B35A0;
    'dispatch: loop {
        match pc {
            0x830B35A0 => {
    //   block [0x830B35A0..0x830B35AC)
	// 830B35A0: C0440004  lfs f2, 4(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 830B35A4: C0240000  lfs f1, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B35A8: 4BFFFA58  b 0x830b3000
	sub_830B3000(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B35B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B35B0 size=12
    let mut pc: u32 = 0x830B35B0;
    'dispatch: loop {
        match pc {
            0x830B35B0 => {
    //   block [0x830B35B0..0x830B35BC)
	// 830B35B0: C0440004  lfs f2, 4(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 830B35B4: C0240000  lfs f1, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B35B8: 4BFFFAE8  b 0x830b30a0
	sub_830B30A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B35C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B35C0 size=12
    let mut pc: u32 = 0x830B35C0;
    'dispatch: loop {
        match pc {
            0x830B35C0 => {
    //   block [0x830B35C0..0x830B35CC)
	// 830B35C0: C0440004  lfs f2, 4(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 830B35C4: C0240000  lfs f1, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B35C8: 4BFFFB40  b 0x830b3108
	sub_830B3108(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B35D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B35D0 size=12
    let mut pc: u32 = 0x830B35D0;
    'dispatch: loop {
        match pc {
            0x830B35D0 => {
    //   block [0x830B35D0..0x830B35DC)
	// 830B35D0: C0440004  lfs f2, 4(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 830B35D4: C0240000  lfs f1, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B35D8: 4BFFFBA8  b 0x830b3180
	sub_830B3180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B35E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B35E0 size=208
    let mut pc: u32 = 0x830B35E0;
    'dispatch: loop {
        match pc {
            0x830B35E0 => {
    //   block [0x830B35E0..0x830B36B0)
	// 830B35E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B35E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B35E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B35EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B35F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B35F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B35F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B35FC: 807F0074  lwz r3, 0x74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 830B3600: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B3604: 419A0020  beq cr6, 0x830b3624
	if ctx.cr[6].eq {
	pc = 0x830B3624; continue 'dispatch;
	}
	// 830B3608: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B360C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B3610: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3614: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B3618: 4E800421  bctrl
	ctx.lr = 0x830B361C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B361C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B3620: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 830B3624: 815F00C4  lwz r10, 0xc4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 830B3628: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B362C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830B3630: 48000024  b 0x830b3654
	pc = 0x830B3654; continue 'dispatch;
	// 830B3634: 386B0014  addi r3, r11, 0x14
	ctx.r[3].s64 = ctx.r[11].s64 + 20;
	// 830B3638: 4BFFE8F1  bl 0x830b1f28
	ctx.lr = 0x830B363C;
	sub_830B1F28(ctx, base);
	// 830B363C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B3640: 48003F59  bl 0x830b7598
	ctx.lr = 0x830B3644;
	sub_830B7598(ctx, base);
	// 830B3644: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B3648: 4B7A7371  bl 0x8285a9b8
	ctx.lr = 0x830B364C;
	sub_8285A9B8(ctx, base);
	// 830B364C: 815F00C4  lwz r10, 0xc4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 830B3650: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B3654: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830B3658: 409AFFDC  bne cr6, 0x830b3634
	if !ctx.cr[6].eq {
	pc = 0x830B3634; continue 'dispatch;
	}
	// 830B365C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830B3660: 419A0038  beq cr6, 0x830b3698
	if ctx.cr[6].eq {
	pc = 0x830B3698; continue 'dispatch;
	}
	// 830B3664: 807F00DC  lwz r3, 0xdc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 830B3668: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B366C: 419A0010  beq cr6, 0x830b367c
	if ctx.cr[6].eq {
	pc = 0x830B367C; continue 'dispatch;
	}
	// 830B3670: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B3674: 4BFFF62D  bl 0x830b2ca0
	ctx.lr = 0x830B3678;
	sub_830B2CA0(ctx, base);
	// 830B3678: 48000008  b 0x830b3680
	pc = 0x830B3680; continue 'dispatch;
	// 830B367C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830B3680: 907F0074  stw r3, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 830B3684: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 830B3688: 80A3000C  lwz r5, 0xc(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B368C: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B3690: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B3694: 4800A2F5  bl 0x830bd988
	ctx.lr = 0x830B3698;
	sub_830BD988(ctx, base);
	// 830B3698: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B369C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B36A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B36A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B36A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B36AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B36B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B36B0 size=88
    let mut pc: u32 = 0x830B36B0;
    'dispatch: loop {
        match pc {
            0x830B36B0 => {
    //   block [0x830B36B0..0x830B3708)
	// 830B36B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B36B4: 480F4AB9  bl 0x831a816c
	ctx.lr = 0x830B36B8;
	sub_831A8130(ctx, base);
	// 830B36B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B36BC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B36C0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830B36C4: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 830B36C8: 897E001D  lbz r11, 0x1d(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(29 as u32) ) } as u64;
	// 830B36CC: 4800002C  b 0x830b36f8
	pc = 0x830B36F8; continue 'dispatch;
	// 830B36D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B36D4: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B36D8: 4BFFFFD9  bl 0x830b36b0
	ctx.lr = 0x830B36DC;
	sub_830B36B0(ctx, base);
	// 830B36DC: 387E0014  addi r3, r30, 0x14
	ctx.r[3].s64 = ctx.r[30].s64 + 20;
	// 830B36E0: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B36E4: 4BFFE7F5  bl 0x830b1ed8
	ctx.lr = 0x830B36E8;
	sub_830B1ED8(ctx, base);
	// 830B36E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B36EC: 4B20CB7D  bl 0x822c0268
	ctx.lr = 0x830B36F0;
	sub_822C0268(ctx, base);
	// 830B36F0: 897F001D  lbz r11, 0x1d(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(29 as u32) ) } as u64;
	// 830B36F4: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 830B36F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B36FC: 419AFFD4  beq cr6, 0x830b36d0
	if ctx.cr[6].eq {
	pc = 0x830B36D0; continue 'dispatch;
	}
	// 830B3700: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B3704: 480F4AB8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B3708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B3708 size=100
    let mut pc: u32 = 0x830B3708;
    'dispatch: loop {
        match pc {
            0x830B3708 => {
    //   block [0x830B3708..0x830B376C)
	// 830B3708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B370C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B3710: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B3714: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B3718: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B371C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B3720: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B3724: 93C1008C  stw r30, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[30].u32 ) };
	// 830B3728: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B372C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3730: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B3734: 4E800421  bctrl
	ctx.lr = 0x830B3738;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B3738: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830B373C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B3740: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 830B3744: 4BFFFD5D  bl 0x830b34a0
	ctx.lr = 0x830B3748;
	sub_830B34A0(ctx, base);
	// 830B3748: 3881008C  addi r4, r1, 0x8c
	ctx.r[4].s64 = ctx.r[1].s64 + 140;
	// 830B374C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 830B3750: 4BFFFD51  bl 0x830b34a0
	ctx.lr = 0x830B3754;
	sub_830B34A0(ctx, base);
	// 830B3754: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B3758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B375C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B3760: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B3764: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B3768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B3770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B3770 size=84
    let mut pc: u32 = 0x830B3770;
    'dispatch: loop {
        match pc {
            0x830B3770 => {
    //   block [0x830B3770..0x830B37C4)
	// 830B3770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B3774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B3778: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B377C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B3780: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B3784: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3788: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B378C: 4BFFFF25  bl 0x830b36b0
	ctx.lr = 0x830B3790;
	sub_830B36B0(ctx, base);
	// 830B3790: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3794: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B3798: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830B379C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830B37A0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B37A4: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830B37A8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B37AC: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830B37B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B37B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B37B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B37BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B37C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B37C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B37C8 size=568
    let mut pc: u32 = 0x830B37C8;
    'dispatch: loop {
        match pc {
            0x830B37C8 => {
    //   block [0x830B37C8..0x830B3A00)
	// 830B37C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B37CC: 480F4991  bl 0x831a815c
	ctx.lr = 0x830B37D0;
	sub_831A8130(ctx, base);
	// 830B37D0: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B37D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B37D8: 3D600FFF  lis r11, 0xfff
	ctx.r[11].s64 = 268369920;
	// 830B37DC: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 830B37E0: 616BFFFE  ori r11, r11, 0xfffe
	ctx.r[11].u64 = ctx.r[11].u64 | 65534;
	// 830B37E4: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 830B37E8: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B37EC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 830B37F0: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 830B37F4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B37F8: 41980048  blt cr6, 0x830b3840
	if ctx.cr[6].lt {
	pc = 0x830B3840; continue 'dispatch;
	}
	// 830B37FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 830B3800: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B3804: 388B9BCC  addi r4, r11, -0x6434
	ctx.r[4].s64 = ctx.r[11].s64 + -25652;
	// 830B3808: 4B2120C1  bl 0x822c58c8
	ctx.lr = 0x830B380C;
	sub_822C58C8(ctx, base);
	// 830B380C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B3810: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830B3814: 4B212005  bl 0x822c5818
	ctx.lr = 0x830B3818;
	sub_822C5818(ctx, base);
	// 830B3818: 4B210A99  bl 0x822c42b0
	ctx.lr = 0x830B381C;
	sub_822C42B0(ctx, base);
	// 830B381C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 830B3820: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830B3824: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 830B3828: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 830B382C: 4B211C45  bl 0x822c5470
	ctx.lr = 0x830B3830;
	sub_822C5470(ctx, base);
	// 830B3830: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830B3834: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830B3838: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B383C: 4B2114A5  bl 0x822c4ce0
	ctx.lr = 0x830B3840;
	sub_822C4CE0(ctx, base);
	// 830B3840: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B3844: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3848: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830B384C: 4B7FE415  bl 0x828b1c60
	ctx.lr = 0x830B3850;
	sub_828B1C60(ctx, base);
	// 830B3850: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830B3854: 41820020  beq 0x830b3874
	if ctx.cr[0].eq {
	pc = 0x830B3874; continue 'dispatch;
	}
	// 830B3858: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830B385C: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 830B3860: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 830B3864: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830B3868: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830B386C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830B3870: 4BFFEF71  bl 0x830b27e0
	ctx.lr = 0x830B3874;
	sub_830B27E0(ctx, base);
	// 830B3874: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3878: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B387C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830B3880: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B3884: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 830B3888: 409A0018  bne cr6, 0x830b38a0
	if !ctx.cr[6].eq {
	pc = 0x830B38A0; continue 'dispatch;
	}
	// 830B388C: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 830B3890: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3894: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830B3898: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B389C: 4800003C  b 0x830b38d8
	pc = 0x830B38D8; continue 'dispatch;
	// 830B38A0: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B38A4: 41820020  beq 0x830b38c4
	if ctx.cr[0].eq {
	pc = 0x830B38C4; continue 'dispatch;
	}
	// 830B38A8: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830B38AC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B38B0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B38B4: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830B38B8: 409A0024  bne cr6, 0x830b38dc
	if !ctx.cr[6].eq {
	pc = 0x830B38DC; continue 'dispatch;
	}
	// 830B38BC: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830B38C0: 4800001C  b 0x830b38dc
	pc = 0x830B38DC; continue 'dispatch;
	// 830B38C4: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 830B38C8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B38CC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B38D0: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830B38D4: 409A0008  bne cr6, 0x830b38dc
	if !ctx.cr[6].eq {
	pc = 0x830B38DC; continue 'dispatch;
	}
	// 830B38D8: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 830B38DC: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B38E0: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 830B38E4: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 830B38E8: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 830B38EC: 894A001C  lbz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 830B38F0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B38F4: 409A00F0  bne cr6, 0x830b39e4
	if !ctx.cr[6].eq {
	pc = 0x830B39E4; continue 'dispatch;
	}
	// 830B38F8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 830B38FC: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3900: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3904: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3908: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830B390C: 409A0054  bne cr6, 0x830b3960
	if !ctx.cr[6].eq {
	pc = 0x830B3960; continue 'dispatch;
	}
	// 830B3910: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3914: 892A001C  lbz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 830B3918: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 830B391C: 419A0054  beq cr6, 0x830b3970
	if ctx.cr[6].eq {
	pc = 0x830B3970; continue 'dispatch;
	}
	// 830B3920: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3924: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B3928: 409A0010  bne cr6, 0x830b3938
	if !ctx.cr[6].eq {
	pc = 0x830B3938; continue 'dispatch;
	}
	// 830B392C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B3930: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830B3934: 4B47BC55  bl 0x8252f588
	ctx.lr = 0x830B3938;
	sub_8252F588(ctx, base);
	// 830B3938: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B393C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B3940: 9BAB001C  stb r29, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 830B3944: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3948: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B394C: 9B6B001C  stb r27, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[27].u8 ) };
	// 830B3950: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3954: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3958: 4B47BC99  bl 0x8252f5f0
	ctx.lr = 0x830B395C;
	sub_8252F5F0(ctx, base);
	// 830B395C: 48000074  b 0x830b39d0
	pc = 0x830B39D0; continue 'dispatch;
	// 830B3960: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3964: 892A001C  lbz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 830B3968: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 830B396C: 409A0028  bne cr6, 0x830b3994
	if !ctx.cr[6].eq {
	pc = 0x830B3994; continue 'dispatch;
	}
	// 830B3970: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3974: 9BA9001C  stb r29, 0x1c(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 830B3978: 9BAA001C  stb r29, 0x1c(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 830B397C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3980: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3984: 9B6A001C  stb r27, 0x1c(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(28 as u32), ctx.r[27].u8 ) };
	// 830B3988: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B398C: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3990: 48000040  b 0x830b39d0
	pc = 0x830B39D0; continue 'dispatch;
	// 830B3994: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3998: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B399C: 409A0010  bne cr6, 0x830b39ac
	if !ctx.cr[6].eq {
	pc = 0x830B39AC; continue 'dispatch;
	}
	// 830B39A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B39A4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830B39A8: 4B47BC49  bl 0x8252f5f0
	ctx.lr = 0x830B39AC;
	sub_8252F5F0(ctx, base);
	// 830B39AC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B39B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B39B4: 9BAB001C  stb r29, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 830B39B8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B39BC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B39C0: 9B6B001C  stb r27, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[27].u8 ) };
	// 830B39C4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B39C8: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B39CC: 4B47BBBD  bl 0x8252f588
	ctx.lr = 0x830B39D0;
	sub_8252F588(ctx, base);
	// 830B39D0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B39D4: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 830B39D8: 894A001C  lbz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 830B39DC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B39E0: 419AFF1C  beq cr6, 0x830b38fc
	if ctx.cr[6].eq {
	pc = 0x830B38FC; continue 'dispatch;
	}
	// 830B39E4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B39E8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 830B39EC: 93990000  stw r28, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830B39F0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B39F4: 9BAB001C  stb r29, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 830B39F8: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 830B39FC: 480F47B0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B3A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B3A00 size=1016
    let mut pc: u32 = 0x830B3A00;
    'dispatch: loop {
        match pc {
            0x830B3A00 => {
    //   block [0x830B3A00..0x830B3DF8)
	// 830B3A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B3A04: 480F4755  bl 0x831a8158
	ctx.lr = 0x830B3A08;
	sub_831A8130(ctx, base);
	// 830B3A08: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B3A0C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 830B3A10: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 830B3A14: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 830B3A18: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 830B3A1C: 897F001D  lbz r11, 0x1d(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(29 as u32) ) } as u64;
	// 830B3A20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B3A24: 419A0048  beq cr6, 0x830b3a6c
	if ctx.cr[6].eq {
	pc = 0x830B3A6C; continue 'dispatch;
	}
	// 830B3A28: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 830B3A2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B3A30: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 830B3A34: 4B211E95  bl 0x822c58c8
	ctx.lr = 0x830B3A38;
	sub_822C58C8(ctx, base);
	// 830B3A38: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B3A3C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830B3A40: 4B216471  bl 0x822c9eb0
	ctx.lr = 0x830B3A44;
	sub_822C9EB0(ctx, base);
	// 830B3A44: 4B21086D  bl 0x822c42b0
	ctx.lr = 0x830B3A48;
	sub_822C42B0(ctx, base);
	// 830B3A48: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 830B3A4C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830B3A50: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 830B3A54: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 830B3A58: 4B211A19  bl 0x822c5470
	ctx.lr = 0x830B3A5C;
	sub_822C5470(ctx, base);
	// 830B3A5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830B3A60: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830B3A64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B3A68: 4B211279  bl 0x822c4ce0
	ctx.lr = 0x830B3A6C;
	sub_822C4CE0(ctx, base);
	// 830B3A6C: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 830B3A70: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 830B3A74: 4B7A6F45  bl 0x8285a9b8
	ctx.lr = 0x830B3A78;
	sub_8285A9B8(ctx, base);
	// 830B3A78: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3A7C: 894B001D  lbz r10, 0x1d(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 830B3A80: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B3A84: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 830B3A88: 419A000C  beq cr6, 0x830b3a94
	if ctx.cr[6].eq {
	pc = 0x830B3A94; continue 'dispatch;
	}
	// 830B3A8C: 839A0008  lwz r28, 8(r26)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3A90: 48000028  b 0x830b3ab8
	pc = 0x830B3AB8; continue 'dispatch;
	// 830B3A94: 815A0008  lwz r10, 8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3A98: 894A001D  lbz r10, 0x1d(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(29 as u32) ) } as u64;
	// 830B3A9C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B3AA0: 419A000C  beq cr6, 0x830b3aac
	if ctx.cr[6].eq {
	pc = 0x830B3AAC; continue 'dispatch;
	}
	// 830B3AA4: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 830B3AA8: 48000010  b 0x830b3ab8
	pc = 0x830B3AB8; continue 'dispatch;
	// 830B3AAC: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3AB0: 7F19D040  cmplw cr6, r25, r26
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[26].u32, &mut ctx.xer);
	// 830B3AB4: 409A00DC  bne cr6, 0x830b3b90
	if !ctx.cr[6].eq {
	pc = 0x830B3B90; continue 'dispatch;
	}
	// 830B3AB8: 897C001D  lbz r11, 0x1d(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(29 as u32) ) } as u64;
	// 830B3ABC: 83FA0004  lwz r31, 4(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3AC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B3AC4: 409A0008  bne cr6, 0x830b3acc
	if !ctx.cr[6].eq {
	pc = 0x830B3ACC; continue 'dispatch;
	}
	// 830B3AC8: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 830B3ACC: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3AD0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3AD4: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 830B3AD8: 409A000C  bne cr6, 0x830b3ae4
	if !ctx.cr[6].eq {
	pc = 0x830B3AE4; continue 'dispatch;
	}
	// 830B3ADC: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 830B3AE0: 4800001C  b 0x830b3afc
	pc = 0x830B3AFC; continue 'dispatch;
	// 830B3AE4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3AE8: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 830B3AEC: 409A000C  bne cr6, 0x830b3af8
	if !ctx.cr[6].eq {
	pc = 0x830B3AF8; continue 'dispatch;
	}
	// 830B3AF0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830B3AF4: 48000008  b 0x830b3afc
	pc = 0x830B3AFC; continue 'dispatch;
	// 830B3AF8: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 830B3AFC: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3B00: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3B04: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 830B3B08: 409A003C  bne cr6, 0x830b3b44
	if !ctx.cr[6].eq {
	pc = 0x830B3B44; continue 'dispatch;
	}
	// 830B3B0C: 897C001D  lbz r11, 0x1d(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(29 as u32) ) } as u64;
	// 830B3B10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B3B14: 419A000C  beq cr6, 0x830b3b20
	if ctx.cr[6].eq {
	pc = 0x830B3B20; continue 'dispatch;
	}
	// 830B3B18: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 830B3B1C: 48000024  b 0x830b3b40
	pc = 0x830B3B40; continue 'dispatch;
	// 830B3B20: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3B24: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 830B3B28: 4800000C  b 0x830b3b34
	pc = 0x830B3B34; continue 'dispatch;
	// 830B3B2C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 830B3B30: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3B34: 890B001D  lbz r8, 0x1d(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 830B3B38: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 830B3B3C: 419AFFF0  beq cr6, 0x830b3b2c
	if ctx.cr[6].eq {
	pc = 0x830B3B2C; continue 'dispatch;
	}
	// 830B3B40: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830B3B44: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3B48: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3B4C: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 830B3B50: 409A00D4  bne cr6, 0x830b3c24
	if !ctx.cr[6].eq {
	pc = 0x830B3C24; continue 'dispatch;
	}
	// 830B3B54: 897C001D  lbz r11, 0x1d(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(29 as u32) ) } as u64;
	// 830B3B58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B3B5C: 419A000C  beq cr6, 0x830b3b68
	if ctx.cr[6].eq {
	pc = 0x830B3B68; continue 'dispatch;
	}
	// 830B3B60: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 830B3B64: 48000024  b 0x830b3b88
	pc = 0x830B3B88; continue 'dispatch;
	// 830B3B68: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3B6C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 830B3B70: 4800000C  b 0x830b3b7c
	pc = 0x830B3B7C; continue 'dispatch;
	// 830B3B74: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 830B3B78: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3B7C: 890B001D  lbz r8, 0x1d(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 830B3B80: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 830B3B84: 419AFFF0  beq cr6, 0x830b3b74
	if ctx.cr[6].eq {
	pc = 0x830B3B74; continue 'dispatch;
	}
	// 830B3B88: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 830B3B8C: 48000098  b 0x830b3c24
	pc = 0x830B3C24; continue 'dispatch;
	// 830B3B90: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 830B3B94: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3B98: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B3B9C: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3BA0: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B3BA4: 409A000C  bne cr6, 0x830b3bb0
	if !ctx.cr[6].eq {
	pc = 0x830B3BB0; continue 'dispatch;
	}
	// 830B3BA8: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 830B3BAC: 4800002C  b 0x830b3bd8
	pc = 0x830B3BD8; continue 'dispatch;
	// 830B3BB0: 897C001D  lbz r11, 0x1d(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(29 as u32) ) } as u64;
	// 830B3BB4: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3BB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B3BBC: 409A0008  bne cr6, 0x830b3bc4
	if !ctx.cr[6].eq {
	pc = 0x830B3BC4; continue 'dispatch;
	}
	// 830B3BC0: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 830B3BC4: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830B3BC8: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3BCC: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830B3BD0: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3BD4: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 830B3BD8: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3BDC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3BE0: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 830B3BE4: 409A000C  bne cr6, 0x830b3bf0
	if !ctx.cr[6].eq {
	pc = 0x830B3BF0; continue 'dispatch;
	}
	// 830B3BE8: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 830B3BEC: 48000020  b 0x830b3c0c
	pc = 0x830B3C0C; continue 'dispatch;
	// 830B3BF0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3BF4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3BF8: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 830B3BFC: 409A000C  bne cr6, 0x830b3c08
	if !ctx.cr[6].eq {
	pc = 0x830B3C08; continue 'dispatch;
	}
	// 830B3C00: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 830B3C04: 48000008  b 0x830b3c0c
	pc = 0x830B3C0C; continue 'dispatch;
	// 830B3C08: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 830B3C0C: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3C10: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830B3C14: 897A001C  lbz r11, 0x1c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 830B3C18: 8959001C  lbz r10, 0x1c(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(28 as u32) ) } as u64;
	// 830B3C1C: 9979001C  stb r11, 0x1c(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 830B3C20: 995A001C  stb r10, 0x1c(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(28 as u32), ctx.r[10].u8 ) };
	// 830B3C24: 897A001C  lbz r11, 0x1c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 830B3C28: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 830B3C2C: 409A0198  bne cr6, 0x830b3dc4
	if !ctx.cr[6].eq {
	pc = 0x830B3DC4; continue 'dispatch;
	}
	// 830B3C30: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3C34: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 830B3C38: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3C3C: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B3C40: 419A0180  beq cr6, 0x830b3dc0
	if ctx.cr[6].eq {
	pc = 0x830B3DC0; continue 'dispatch;
	}
	// 830B3C44: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830B3C48: 897C001C  lbz r11, 0x1c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 830B3C4C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 830B3C50: 409A0170  bne cr6, 0x830b3dc0
	if !ctx.cr[6].eq {
	pc = 0x830B3DC0; continue 'dispatch;
	}
	// 830B3C54: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3C58: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B3C5C: 409A00A8  bne cr6, 0x830b3d04
	if !ctx.cr[6].eq {
	pc = 0x830B3D04; continue 'dispatch;
	}
	// 830B3C60: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3C64: 894B001C  lbz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 830B3C68: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B3C6C: 409A001C  bne cr6, 0x830b3c88
	if !ctx.cr[6].eq {
	pc = 0x830B3C88; continue 'dispatch;
	}
	// 830B3C70: 9BCB001C  stb r30, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 830B3C74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B3C78: 9BBF001C  stb r29, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 830B3C7C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830B3C80: 4B47B909  bl 0x8252f588
	ctx.lr = 0x830B3C84;
	sub_8252F588(ctx, base);
	// 830B3C84: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3C88: 894B001D  lbz r10, 0x1d(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 830B3C8C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B3C90: 409A00C8  bne cr6, 0x830b3d58
	if !ctx.cr[6].eq {
	pc = 0x830B3D58; continue 'dispatch;
	}
	// 830B3C94: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3C98: 894A001C  lbz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 830B3C9C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 830B3CA0: 409A0014  bne cr6, 0x830b3cb4
	if !ctx.cr[6].eq {
	pc = 0x830B3CB4; continue 'dispatch;
	}
	// 830B3CA4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3CA8: 894A001C  lbz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 830B3CAC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 830B3CB0: 419A00A4  beq cr6, 0x830b3d54
	if ctx.cr[6].eq {
	pc = 0x830B3D54; continue 'dispatch;
	}
	// 830B3CB4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3CB8: 894A001C  lbz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 830B3CBC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 830B3CC0: 409A0020  bne cr6, 0x830b3ce0
	if !ctx.cr[6].eq {
	pc = 0x830B3CE0; continue 'dispatch;
	}
	// 830B3CC4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3CC8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 830B3CCC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830B3CD0: 9BCA001C  stb r30, 0x1c(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 830B3CD4: 9BAB001C  stb r29, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 830B3CD8: 4B47B919  bl 0x8252f5f0
	ctx.lr = 0x830B3CDC;
	sub_8252F5F0(ctx, base);
	// 830B3CDC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3CE0: 895F001C  lbz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830B3CE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B3CE8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830B3CEC: 994B001C  stb r10, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u8 ) };
	// 830B3CF0: 9BDF001C  stb r30, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 830B3CF4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3CF8: 9BCB001C  stb r30, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 830B3CFC: 4B47B88D  bl 0x8252f588
	ctx.lr = 0x830B3D00;
	sub_8252F588(ctx, base);
	// 830B3D00: 480000C0  b 0x830b3dc0
	pc = 0x830B3DC0; continue 'dispatch;
	// 830B3D04: 894B001C  lbz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 830B3D08: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B3D0C: 409A001C  bne cr6, 0x830b3d28
	if !ctx.cr[6].eq {
	pc = 0x830B3D28; continue 'dispatch;
	}
	// 830B3D10: 9BCB001C  stb r30, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 830B3D14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B3D18: 9BBF001C  stb r29, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 830B3D1C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830B3D20: 4B47B8D1  bl 0x8252f5f0
	ctx.lr = 0x830B3D24;
	sub_8252F5F0(ctx, base);
	// 830B3D24: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3D28: 894B001D  lbz r10, 0x1d(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 830B3D2C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B3D30: 409A0028  bne cr6, 0x830b3d58
	if !ctx.cr[6].eq {
	pc = 0x830B3D58; continue 'dispatch;
	}
	// 830B3D34: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3D38: 894A001C  lbz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 830B3D3C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 830B3D40: 409A0034  bne cr6, 0x830b3d74
	if !ctx.cr[6].eq {
	pc = 0x830B3D74; continue 'dispatch;
	}
	// 830B3D44: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3D48: 894A001C  lbz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 830B3D4C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 830B3D50: 409A0024  bne cr6, 0x830b3d74
	if !ctx.cr[6].eq {
	pc = 0x830B3D74; continue 'dispatch;
	}
	// 830B3D54: 9BAB001C  stb r29, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 830B3D58: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3D5C: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 830B3D60: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3D64: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3D68: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B3D6C: 409AFEDC  bne cr6, 0x830b3c48
	if !ctx.cr[6].eq {
	pc = 0x830B3C48; continue 'dispatch;
	}
	// 830B3D70: 48000050  b 0x830b3dc0
	pc = 0x830B3DC0; continue 'dispatch;
	// 830B3D74: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3D78: 894A001C  lbz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 830B3D7C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 830B3D80: 409A0020  bne cr6, 0x830b3da0
	if !ctx.cr[6].eq {
	pc = 0x830B3DA0; continue 'dispatch;
	}
	// 830B3D84: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3D88: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 830B3D8C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830B3D90: 9BCA001C  stb r30, 0x1c(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 830B3D94: 9BAB001C  stb r29, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 830B3D98: 4B47B7F1  bl 0x8252f588
	ctx.lr = 0x830B3D9C;
	sub_8252F588(ctx, base);
	// 830B3D9C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3DA0: 895F001C  lbz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830B3DA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B3DA8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830B3DAC: 994B001C  stb r10, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u8 ) };
	// 830B3DB0: 9BDF001C  stb r30, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 830B3DB4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3DB8: 9BCB001C  stb r30, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 830B3DBC: 4B47B835  bl 0x8252f5f0
	ctx.lr = 0x830B3DC0;
	sub_8252F5F0(ctx, base);
	// 830B3DC0: 9BDC001C  stb r30, 0x1c(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 830B3DC4: 387A0014  addi r3, r26, 0x14
	ctx.r[3].s64 = ctx.r[26].s64 + 20;
	// 830B3DC8: 4BFFE111  bl 0x830b1ed8
	ctx.lr = 0x830B3DCC;
	sub_830B1ED8(ctx, base);
	// 830B3DCC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830B3DD0: 4B20C499  bl 0x822c0268
	ctx.lr = 0x830B3DD4;
	sub_822C0268(ctx, base);
	// 830B3DD4: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3DD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B3DDC: 419A000C  beq cr6, 0x830b3de8
	if ctx.cr[6].eq {
	pc = 0x830B3DE8; continue 'dispatch;
	}
	// 830B3DE0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 830B3DE4: 917B0008  stw r11, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830B3DE8: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 830B3DEC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 830B3DF0: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 830B3DF4: 480F43B4  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B3DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B3DF8 size=1124
    let mut pc: u32 = 0x830B3DF8;
    'dispatch: loop {
        match pc {
            0x830B3DF8 => {
    //   block [0x830B3DF8..0x830B425C)
	// 830B3DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B3DFC: 480F4365  bl 0x831a8160
	ctx.lr = 0x830B3E00;
	sub_831A8130(ctx, base);
	// 830B3E00: DBC1FFB8  stfd f30, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[30].u64 ) };
	// 830B3E04: DBE1FFC0  stfd f31, -0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 830B3E08: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B3E0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B3E10: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 830B3E14: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B3E18: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B3E1C: 419A0430  beq cr6, 0x830b424c
	if ctx.cr[6].eq {
	pc = 0x830B424C; continue 'dispatch;
	}
	// 830B3E20: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830B3E24: 3B5F0010  addi r26, r31, 0x10
	ctx.r[26].s64 = ctx.r[31].s64 + 16;
	// 830B3E28: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3E2C: 48000018  b 0x830b3e44
	pc = 0x830B3E44; continue 'dispatch;
	// 830B3E30: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B3E34: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3E38: 4BFFE4D1  bl 0x830b2308
	ctx.lr = 0x830B3E3C;
	sub_830B2308(ctx, base);
	// 830B3E3C: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3E40: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3E44: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B3E48: 409AFFE8  bne cr6, 0x830b3e30
	if !ctx.cr[6].eq {
	pc = 0x830B3E30; continue 'dispatch;
	}
	// 830B3E4C: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 830B3E50: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3E54: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 830B3E58: 48000050  b 0x830b3ea8
	pc = 0x830B3EA8; continue 'dispatch;
	// 830B3E5C: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 830B3E60: 4BFFE0C9  bl 0x830b1f28
	ctx.lr = 0x830B3E64;
	sub_830B1F28(ctx, base);
	// 830B3E64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3E68: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B3E6C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3E70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B3E74: 4E800421  bctrl
	ctx.lr = 0x830B3E78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B3E78: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B3E7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B3E80: 4082001C  bne 0x830b3e9c
	if !ctx.cr[0].eq {
	pc = 0x830B3E9C; continue 'dispatch;
	}
	// 830B3E84: 4B2ED805  bl 0x823a1688
	ctx.lr = 0x830B3E88;
	sub_823A1688(ctx, base);
	// 830B3E88: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830B3E8C: 389F003C  addi r4, r31, 0x3c
	ctx.r[4].s64 = ctx.r[31].s64 + 60;
	// 830B3E90: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 830B3E94: 4BA97715  bl 0x82b4b5a8
	ctx.lr = 0x830B3E98;
	sub_82B4B5A8(ctx, base);
	// 830B3E98: 48000008  b 0x830b3ea0
	pc = 0x830B3EA0; continue 'dispatch;
	// 830B3E9C: 4B2ED7ED  bl 0x823a1688
	ctx.lr = 0x830B3EA0;
	sub_823A1688(ctx, base);
	// 830B3EA0: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 830B3EA4: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B3EA8: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B3EAC: 409AFFB0  bne cr6, 0x830b3e5c
	if !ctx.cr[6].eq {
	pc = 0x830B3E5C; continue 'dispatch;
	}
	// 830B3EB0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 830B3EB4: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 830B3EB8: FF1EF800  fcmpu cr6, f30, f31
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[31].f64);
	// 830B3EBC: 40990184  ble cr6, 0x830b4040
	if !ctx.cr[6].gt {
	pc = 0x830B4040; continue 'dispatch;
	}
	// 830B3EC0: C01F008C  lfs f0, 0x8c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B3EC4: C1BF007C  lfs f13, 0x7c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B3EC8: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 830B3ECC: 419800B4  blt cr6, 0x830b3f80
	if ctx.cr[6].lt {
	pc = 0x830B3F80; continue 'dispatch;
	}
	// 830B3ED0: C01F0084  lfs f0, 0x84(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B3ED4: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 830B3ED8: 409900A8  ble cr6, 0x830b3f80
	if !ctx.cr[6].gt {
	pc = 0x830B3F80; continue 'dispatch;
	}
	// 830B3EDC: 817F00A0  lwz r11, 0xa0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 830B3EE0: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 830B3EE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B3EE8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B3EEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B3EF0: 4E800421  bctrl
	ctx.lr = 0x830B3EF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B3EF4: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3EF8: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3EFC: 48000018  b 0x830b3f14
	pc = 0x830B3F14; continue 'dispatch;
	// 830B3F00: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B3F04: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3F08: 4BFFE421  bl 0x830b2328
	ctx.lr = 0x830B3F0C;
	sub_830B2328(ctx, base);
	// 830B3F0C: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3F10: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3F14: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B3F18: 409AFFE8  bne cr6, 0x830b3f00
	if !ctx.cr[6].eq {
	pc = 0x830B3F00; continue 'dispatch;
	}
	// 830B3F1C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830B3F20: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3F24: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 830B3F28: 48000050  b 0x830b3f78
	pc = 0x830B3F78; continue 'dispatch;
	// 830B3F2C: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 830B3F30: 4BFFDFF9  bl 0x830b1f28
	ctx.lr = 0x830B3F34;
	sub_830B1F28(ctx, base);
	// 830B3F34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3F38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B3F3C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3F40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B3F44: 4E800421  bctrl
	ctx.lr = 0x830B3F48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B3F48: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B3F4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B3F50: 4082001C  bne 0x830b3f6c
	if !ctx.cr[0].eq {
	pc = 0x830B3F6C; continue 'dispatch;
	}
	// 830B3F54: 4B2ED735  bl 0x823a1688
	ctx.lr = 0x830B3F58;
	sub_823A1688(ctx, base);
	// 830B3F58: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830B3F5C: 389F0024  addi r4, r31, 0x24
	ctx.r[4].s64 = ctx.r[31].s64 + 36;
	// 830B3F60: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 830B3F64: 4BA97645  bl 0x82b4b5a8
	ctx.lr = 0x830B3F68;
	sub_82B4B5A8(ctx, base);
	// 830B3F68: 48000008  b 0x830b3f70
	pc = 0x830B3F70; continue 'dispatch;
	// 830B3F6C: 4B2ED71D  bl 0x823a1688
	ctx.lr = 0x830B3F70;
	sub_823A1688(ctx, base);
	// 830B3F70: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830B3F74: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B3F78: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B3F7C: 409AFFB0  bne cr6, 0x830b3f2c
	if !ctx.cr[6].eq {
	pc = 0x830B3F2C; continue 'dispatch;
	}
	// 830B3F80: C01F007C  lfs f0, 0x7c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B3F84: C1BF0088  lfs f13, 0x88(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B3F88: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 830B3F8C: 419900B4  bgt cr6, 0x830b4040
	if ctx.cr[6].gt {
	pc = 0x830B4040; continue 'dispatch;
	}
	// 830B3F90: C01F0084  lfs f0, 0x84(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B3F94: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 830B3F98: 409800A8  bge cr6, 0x830b4040
	if !ctx.cr[6].lt {
	pc = 0x830B4040; continue 'dispatch;
	}
	// 830B3F9C: 817F00A0  lwz r11, 0xa0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 830B3FA0: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 830B3FA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B3FA8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3FAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B3FB0: 4E800421  bctrl
	ctx.lr = 0x830B3FB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B3FB4: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3FB8: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3FBC: 48000018  b 0x830b3fd4
	pc = 0x830B3FD4; continue 'dispatch;
	// 830B3FC0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B3FC4: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3FC8: 4BFFE371  bl 0x830b2338
	ctx.lr = 0x830B3FCC;
	sub_830B2338(ctx, base);
	// 830B3FCC: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B3FD0: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3FD4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B3FD8: 409AFFE8  bne cr6, 0x830b3fc0
	if !ctx.cr[6].eq {
	pc = 0x830B3FC0; continue 'dispatch;
	}
	// 830B3FDC: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B3FE0: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3FE4: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 830B3FE8: 48000050  b 0x830b4038
	pc = 0x830B4038; continue 'dispatch;
	// 830B3FEC: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 830B3FF0: 4BFFDF39  bl 0x830b1f28
	ctx.lr = 0x830B3FF4;
	sub_830B1F28(ctx, base);
	// 830B3FF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B3FF8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B3FFC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4000: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B4004: 4E800421  bctrl
	ctx.lr = 0x830B4008;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B4008: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B400C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B4010: 4082001C  bne 0x830b402c
	if !ctx.cr[0].eq {
	pc = 0x830B402C; continue 'dispatch;
	}
	// 830B4014: 4B2ED675  bl 0x823a1688
	ctx.lr = 0x830B4018;
	sub_823A1688(ctx, base);
	// 830B4018: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830B401C: 389F0030  addi r4, r31, 0x30
	ctx.r[4].s64 = ctx.r[31].s64 + 48;
	// 830B4020: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 830B4024: 4BA97585  bl 0x82b4b5a8
	ctx.lr = 0x830B4028;
	sub_82B4B5A8(ctx, base);
	// 830B4028: 48000008  b 0x830b4030
	pc = 0x830B4030; continue 'dispatch;
	// 830B402C: 4B2ED65D  bl 0x823a1688
	ctx.lr = 0x830B4030;
	sub_823A1688(ctx, base);
	// 830B4030: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B4034: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B4038: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B403C: 409AFFB0  bne cr6, 0x830b3fec
	if !ctx.cr[6].eq {
	pc = 0x830B3FEC; continue 'dispatch;
	}
	// 830B4040: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 830B4044: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830B4048: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 830B404C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B4050: 419A001C  beq cr6, 0x830b406c
	if ctx.cr[6].eq {
	pc = 0x830B406C; continue 'dispatch;
	}
	// 830B4054: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B4058: 838B000C  lwz r28, 0xc(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B405C: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4060: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 830B4064: 41980008  blt cr6, 0x830b406c
	if ctx.cr[6].lt {
	pc = 0x830B406C; continue 'dispatch;
	}
	// 830B4068: 836B0014  lwz r27, 0x14(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 830B406C: 3BDF0090  addi r30, r31, 0x90
	ctx.r[30].s64 = ctx.r[31].s64 + 144;
	// 830B4070: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830B4074: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B4078: 4BFFDF09  bl 0x830b1f80
	ctx.lr = 0x830B407C;
	sub_830B1F80(ctx, base);
	// 830B407C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B4080: 41820014  beq 0x830b4094
	if ctx.cr[0].eq {
	pc = 0x830B4094; continue 'dispatch;
	}
	// 830B4084: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B4088: 4BFFDEA1  bl 0x830b1f28
	ctx.lr = 0x830B408C;
	sub_830B1F28(ctx, base);
	// 830B408C: 81630034  lwz r11, 0x34(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B4090: 48000018  b 0x830b40a8
	pc = 0x830B40A8; continue 'dispatch;
	// 830B4094: 807F0098  lwz r3, 0x98(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 830B4098: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B409C: 419A0010  beq cr6, 0x830b40ac
	if ctx.cr[6].eq {
	pc = 0x830B40AC; continue 'dispatch;
	}
	// 830B40A0: 48008909  bl 0x830bc9a8
	ctx.lr = 0x830B40A4;
	sub_830BC9A8(ctx, base);
	// 830B40A4: 817F0098  lwz r11, 0x98(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 830B40A8: 83AB000C  lwz r29, 0xc(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B40AC: 817F0074  lwz r11, 0x74(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 830B40B0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830B40B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B40B8: 419A0008  beq cr6, 0x830b40c0
	if ctx.cr[6].eq {
	pc = 0x830B40C0; continue 'dispatch;
	}
	// 830B40BC: 83CB000C  lwz r30, 0xc(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B40C0: 815F00C4  lwz r10, 0xc4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 830B40C4: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B40C8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830B40CC: 48000020  b 0x830b40ec
	pc = 0x830B40EC; continue 'dispatch;
	// 830B40D0: 386B0014  addi r3, r11, 0x14
	ctx.r[3].s64 = ctx.r[11].s64 + 20;
	// 830B40D4: 4BFFDE55  bl 0x830b1f28
	ctx.lr = 0x830B40D8;
	sub_830B1F28(ctx, base);
	// 830B40D8: 480037F9  bl 0x830b78d0
	ctx.lr = 0x830B40DC;
	sub_830B78D0(ctx, base);
	// 830B40DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B40E0: 4B7A68D9  bl 0x8285a9b8
	ctx.lr = 0x830B40E4;
	sub_8285A9B8(ctx, base);
	// 830B40E4: 815F00C4  lwz r10, 0xc4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 830B40E8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B40EC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830B40F0: 409AFFE0  bne cr6, 0x830b40d0
	if !ctx.cr[6].eq {
	pc = 0x830B40D0; continue 'dispatch;
	}
	// 830B40F4: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 830B40F8: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 830B40FC: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 830B4100: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B4104: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 830B4108: C03F0080  lfs f1, 0x80(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B410C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830B4110: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B4114: 48007125  bl 0x830bb238
	ctx.lr = 0x830B4118;
	sub_830BB238(ctx, base);
	// 830B4118: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B411C: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4120: 48000018  b 0x830b4138
	pc = 0x830B4138; continue 'dispatch;
	// 830B4124: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B4128: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B412C: 4BFFE1ED  bl 0x830b2318
	ctx.lr = 0x830B4130;
	sub_830B2318(ctx, base);
	// 830B4130: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B4134: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4138: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B413C: 409AFFE8  bne cr6, 0x830b4124
	if !ctx.cr[6].eq {
	pc = 0x830B4124; continue 'dispatch;
	}
	// 830B4140: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 830B4144: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4148: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 830B414C: 48000050  b 0x830b419c
	pc = 0x830B419C; continue 'dispatch;
	// 830B4150: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 830B4154: 4BFFDDD5  bl 0x830b1f28
	ctx.lr = 0x830B4158;
	sub_830B1F28(ctx, base);
	// 830B4158: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B415C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B4160: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4164: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B4168: 4E800421  bctrl
	ctx.lr = 0x830B416C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B416C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B4170: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B4174: 4082001C  bne 0x830b4190
	if !ctx.cr[0].eq {
	pc = 0x830B4190; continue 'dispatch;
	}
	// 830B4178: 4B2ED511  bl 0x823a1688
	ctx.lr = 0x830B417C;
	sub_823A1688(ctx, base);
	// 830B417C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830B4180: 389F0048  addi r4, r31, 0x48
	ctx.r[4].s64 = ctx.r[31].s64 + 72;
	// 830B4184: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 830B4188: 4BA97421  bl 0x82b4b5a8
	ctx.lr = 0x830B418C;
	sub_82B4B5A8(ctx, base);
	// 830B418C: 48000008  b 0x830b4194
	pc = 0x830B4194; continue 'dispatch;
	// 830B4190: 4B2ED4F9  bl 0x823a1688
	ctx.lr = 0x830B4194;
	sub_823A1688(ctx, base);
	// 830B4194: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 830B4198: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B419C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B41A0: 409AFFB0  bne cr6, 0x830b4150
	if !ctx.cr[6].eq {
	pc = 0x830B4150; continue 'dispatch;
	}
	// 830B41A4: 817F009C  lwz r11, 0x9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 830B41A8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B41AC: 409A0054  bne cr6, 0x830b4200
	if !ctx.cr[6].eq {
	pc = 0x830B4200; continue 'dispatch;
	}
	// 830B41B0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B41B4: C01F0080  lfs f0, 0x80(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B41B8: C19F0084  lfs f12, 0x84(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 830B41BC: D01F007C  stfs f0, 0x7c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 830B41C0: C1BF008C  lfs f13, 0x8c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B41C4: C16B0008  lfs f11, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 830B41C8: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 830B41CC: EC0C07BA  fmadds f0, f12, f30, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[30].f64 + ctx.f[0].f64) as f32) as f64);
	// 830B41D0: D01F0080  stfs f0, 0x80(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 830B41D4: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 830B41D8: 4099000C  ble cr6, 0x830b41e4
	if !ctx.cr[6].gt {
	pc = 0x830B41E4; continue 'dispatch;
	}
	// 830B41DC: 817F00B0  lwz r11, 0xb0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 830B41E0: 48000014  b 0x830b41f4
	pc = 0x830B41F4; continue 'dispatch;
	// 830B41E4: C1BF0088  lfs f13, 0x88(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B41E8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 830B41EC: 40980014  bge cr6, 0x830b4200
	if !ctx.cr[6].lt {
	pc = 0x830B4200; continue 'dispatch;
	}
	// 830B41F0: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 830B41F4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 830B41F8: 419A0008  beq cr6, 0x830b4200
	if ctx.cr[6].eq {
	pc = 0x830B4200; continue 'dispatch;
	}
	// 830B41FC: D1BF0080  stfs f13, 0x80(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 830B4200: 815F00C4  lwz r10, 0xc4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 830B4204: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4208: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830B420C: 48000038  b 0x830b4244
	pc = 0x830B4244; continue 'dispatch;
	// 830B4210: 3BCB0014  addi r30, r11, 0x14
	ctx.r[30].s64 = ctx.r[11].s64 + 20;
	// 830B4214: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B4218: 4BFFDD11  bl 0x830b1f28
	ctx.lr = 0x830B421C;
	sub_830B1F28(ctx, base);
	// 830B421C: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 830B4220: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B4224: 40990010  ble cr6, 0x830b4234
	if !ctx.cr[6].gt {
	pc = 0x830B4234; continue 'dispatch;
	}
	// 830B4228: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B422C: 4BFFDCFD  bl 0x830b1f28
	ctx.lr = 0x830B4230;
	sub_830B1F28(ctx, base);
	// 830B4230: 48003999  bl 0x830b7bc8
	ctx.lr = 0x830B4234;
	sub_830B7BC8(ctx, base);
	// 830B4234: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B4238: 4B7A6781  bl 0x8285a9b8
	ctx.lr = 0x830B423C;
	sub_8285A9B8(ctx, base);
	// 830B423C: 815F00C4  lwz r10, 0xc4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 830B4240: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B4244: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830B4248: 409AFFC8  bne cr6, 0x830b4210
	if !ctx.cr[6].eq {
	pc = 0x830B4210; continue 'dispatch;
	}
	// 830B424C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 830B4250: CBC1FFB8  lfd f30, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 830B4254: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 830B4258: 480F3F58  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B4260 size=380
    let mut pc: u32 = 0x830B4260;
    'dispatch: loop {
        match pc {
            0x830B4260 => {
    //   block [0x830B4260..0x830B43DC)
	// 830B4260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B4264: 480F3F05  bl 0x831a8168
	ctx.lr = 0x830B4268;
	sub_831A8130(ctx, base);
	// 830B4268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B426C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B4270: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830B4274: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B4278: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B427C: 419A0158  beq cr6, 0x830b43d4
	if ctx.cr[6].eq {
	pc = 0x830B43D4; continue 'dispatch;
	}
	// 830B4280: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830B4284: 3BBF0010  addi r29, r31, 0x10
	ctx.r[29].s64 = ctx.r[31].s64 + 16;
	// 830B4288: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B428C: 48000018  b 0x830b42a4
	pc = 0x830B42A4; continue 'dispatch;
	// 830B4290: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B4294: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B4298: 4BAE28C1  bl 0x82b96b58
	ctx.lr = 0x830B429C;
	sub_82B96B58(ctx, base);
	// 830B429C: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B42A0: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B42A4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B42A8: 409AFFE8  bne cr6, 0x830b4290
	if !ctx.cr[6].eq {
	pc = 0x830B4290; continue 'dispatch;
	}
	// 830B42AC: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 830B42B0: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B42B4: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 830B42B8: 48000050  b 0x830b4308
	pc = 0x830B4308; continue 'dispatch;
	// 830B42BC: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 830B42C0: 4BFFDC69  bl 0x830b1f28
	ctx.lr = 0x830B42C4;
	sub_830B1F28(ctx, base);
	// 830B42C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B42C8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B42CC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B42D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B42D4: 4E800421  bctrl
	ctx.lr = 0x830B42D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B42D8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B42DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B42E0: 4082001C  bne 0x830b42fc
	if !ctx.cr[0].eq {
	pc = 0x830B42FC; continue 'dispatch;
	}
	// 830B42E4: 4B2ED3A5  bl 0x823a1688
	ctx.lr = 0x830B42E8;
	sub_823A1688(ctx, base);
	// 830B42E8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830B42EC: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 830B42F0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 830B42F4: 4BA972B5  bl 0x82b4b5a8
	ctx.lr = 0x830B42F8;
	sub_82B4B5A8(ctx, base);
	// 830B42F8: 48000008  b 0x830b4300
	pc = 0x830B4300; continue 'dispatch;
	// 830B42FC: 4B2ED38D  bl 0x823a1688
	ctx.lr = 0x830B4300;
	sub_823A1688(ctx, base);
	// 830B4300: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 830B4304: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B4308: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B430C: 409AFFB0  bne cr6, 0x830b42bc
	if !ctx.cr[6].eq {
	pc = 0x830B42BC; continue 'dispatch;
	}
	// 830B4310: 480047F1  bl 0x830b8b00
	ctx.lr = 0x830B4314;
	sub_830B8B00(ctx, base);
	// 830B4314: 480047FD  bl 0x830b8b10
	ctx.lr = 0x830B4318;
	sub_830B8B10(ctx, base);
	// 830B4318: 817F0074  lwz r11, 0x74(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 830B431C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830B4320: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B4324: 419A0008  beq cr6, 0x830b432c
	if ctx.cr[6].eq {
	pc = 0x830B432C; continue 'dispatch;
	}
	// 830B4328: 80AB000C  lwz r5, 0xc(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B432C: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 830B4330: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830B4334: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B4338: 80CB000C  lwz r6, 0xc(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B433C: 48009D5D  bl 0x830be098
	ctx.lr = 0x830B4340;
	sub_830BE098(ctx, base);
	// 830B4340: 480047C1  bl 0x830b8b00
	ctx.lr = 0x830B4344;
	sub_830B8B00(ctx, base);
	// 830B4344: 4800481D  bl 0x830b8b60
	ctx.lr = 0x830B4348;
	sub_830B8B60(ctx, base);
	// 830B4348: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B434C: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4350: 48000018  b 0x830b4368
	pc = 0x830B4368; continue 'dispatch;
	// 830B4354: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B4358: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B435C: 4BAE280D  bl 0x82b96b68
	ctx.lr = 0x830B4360;
	sub_82B96B68(ctx, base);
	// 830B4360: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B4364: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4368: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B436C: 409AFFE8  bne cr6, 0x830b4354
	if !ctx.cr[6].eq {
	pc = 0x830B4354; continue 'dispatch;
	}
	// 830B4370: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 830B4374: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4378: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 830B437C: 48000050  b 0x830b43cc
	pc = 0x830B43CC; continue 'dispatch;
	// 830B4380: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 830B4384: 4BFFDBA5  bl 0x830b1f28
	ctx.lr = 0x830B4388;
	sub_830B1F28(ctx, base);
	// 830B4388: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B438C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B4390: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4394: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B4398: 4E800421  bctrl
	ctx.lr = 0x830B439C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B439C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B43A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B43A4: 4082001C  bne 0x830b43c0
	if !ctx.cr[0].eq {
	pc = 0x830B43C0; continue 'dispatch;
	}
	// 830B43A8: 4B2ED2E1  bl 0x823a1688
	ctx.lr = 0x830B43AC;
	sub_823A1688(ctx, base);
	// 830B43AC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830B43B0: 389F0060  addi r4, r31, 0x60
	ctx.r[4].s64 = ctx.r[31].s64 + 96;
	// 830B43B4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 830B43B8: 4BA971F1  bl 0x82b4b5a8
	ctx.lr = 0x830B43BC;
	sub_82B4B5A8(ctx, base);
	// 830B43BC: 48000008  b 0x830b43c4
	pc = 0x830B43C4; continue 'dispatch;
	// 830B43C0: 4B2ED2C9  bl 0x823a1688
	ctx.lr = 0x830B43C4;
	sub_823A1688(ctx, base);
	// 830B43C4: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 830B43C8: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B43CC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B43D0: 409AFFB0  bne cr6, 0x830b4380
	if !ctx.cr[6].eq {
	pc = 0x830B4380; continue 'dispatch;
	}
	// 830B43D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B43D8: 480F3DE0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B43E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B43E0 size=308
    let mut pc: u32 = 0x830B43E0;
    'dispatch: loop {
        match pc {
            0x830B43E0 => {
    //   block [0x830B43E0..0x830B4514)
	// 830B43E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B43E4: 480F3D7D  bl 0x831a8160
	ctx.lr = 0x830B43E8;
	sub_831A8130(ctx, base);
	// 830B43E8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B43EC: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 830B43F0: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 830B43F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B43F8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830B43FC: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 830B4400: 83DB0004  lwz r30, 4(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B4404: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B4408: 894B001D  lbz r10, 0x1d(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 830B440C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B4410: 409A0058  bne cr6, 0x830b4468
	if !ctx.cr[6].eq {
	pc = 0x830B4468; continue 'dispatch;
	}
	// 830B4414: 813C0000  lwz r9, 0(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4418: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B441C: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 830B4420: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 830B4424: 409A0010  bne cr6, 0x830b4434
	if !ctx.cr[6].eq {
	pc = 0x830B4434; continue 'dispatch;
	}
	// 830B4428: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B442C: 810B0010  lwz r8, 0x10(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830B4430: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 830B4434: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 830B4438: 41980008  blt cr6, 0x830b4440
	if ctx.cr[6].lt {
	pc = 0x830B4440; continue 'dispatch;
	}
	// 830B443C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B4440: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 830B4444: 5548063F  clrlwi. r8, r10, 0x18
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 830B4448: 7D5D5378  mr r29, r10
	ctx.r[29].u64 = ctx.r[10].u64;
	// 830B444C: 4182000C  beq 0x830b4458
	if ctx.cr[0].eq {
	pc = 0x830B4458; continue 'dispatch;
	}
	// 830B4450: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4454: 48000008  b 0x830b445c
	pc = 0x830B445C; continue 'dispatch;
	// 830B4458: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B445C: 894B001D  lbz r10, 0x1d(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 830B4460: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B4464: 419AFFB4  beq cr6, 0x830b4418
	if ctx.cr[6].eq {
	pc = 0x830B4418; continue 'dispatch;
	}
	// 830B4468: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 830B446C: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B4470: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 830B4474: 41820048  beq 0x830b44bc
	if ctx.cr[0].eq {
	pc = 0x830B44BC; continue 'dispatch;
	}
	// 830B4478: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B447C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B4480: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4484: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B4488: 409A002C  bne cr6, 0x830b44b4
	if !ctx.cr[6].eq {
	pc = 0x830B44B4; continue 'dispatch;
	}
	// 830B448C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830B4490: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830B4494: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 830B4498: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 830B449C: 4BFFF32D  bl 0x830b37c8
	ctx.lr = 0x830B44A0;
	sub_830B37C8(ctx, base);
	// 830B44A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830B44A4: 9B5F0004  stb r26, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u8 ) };
	// 830B44A8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B44AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B44B0: 48000058  b 0x830b4508
	pc = 0x830B4508; continue 'dispatch;
	// 830B44B4: 4B7A65D5  bl 0x8285aa88
	ctx.lr = 0x830B44B8;
	sub_8285AA88(ctx, base);
	// 830B44B8: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B44BC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B44C0: 8149000C  lwz r10, 0xc(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B44C4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830B44C8: 409A0010  bne cr6, 0x830b44d8
	if !ctx.cr[6].eq {
	pc = 0x830B44D8; continue 'dispatch;
	}
	// 830B44CC: 81690010  lwz r11, 0x10(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 830B44D0: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B44D4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 830B44D8: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 830B44DC: 41980008  blt cr6, 0x830b44e4
	if ctx.cr[6].lt {
	pc = 0x830B44E4; continue 'dispatch;
	}
	// 830B44E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B44E4: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830B44E8: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B44EC: 41820010  beq 0x830b44fc
	if ctx.cr[0].eq {
	pc = 0x830B44FC; continue 'dispatch;
	}
	// 830B44F0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830B44F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B44F8: 4BFFFF98  b 0x830b4490
	pc = 0x830B4490; continue 'dispatch;
	// 830B44FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B4500: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 830B4504: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 830B4508: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B450C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830B4510: 480F3CA0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B4518 size=80
    let mut pc: u32 = 0x830B4518;
    'dispatch: loop {
        match pc {
            0x830B4518 => {
    //   block [0x830B4518..0x830B4568)
	// 830B4518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B451C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B4520: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B4524: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B4528: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B452C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B4530: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B4534: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B4538: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B453C: 4BA97755  bl 0x82b4bc90
	ctx.lr = 0x830B4540;
	sub_82B4BC90(ctx, base);
	// 830B4540: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B4544: 4B20BD25  bl 0x822c0268
	ctx.lr = 0x830B4548;
	sub_822C0268(ctx, base);
	// 830B4548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B454C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830B4550: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830B4554: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B4558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B455C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B4560: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B4564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B4568 size=112
    let mut pc: u32 = 0x830B4568;
    'dispatch: loop {
        match pc {
            0x830B4568 => {
    //   block [0x830B4568..0x830B45D8)
	// 830B4568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B456C: 480F3BF5  bl 0x831a8160
	ctx.lr = 0x830B4570;
	sub_831A8130(ctx, base);
	// 830B4570: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B4574: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B4578: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 830B457C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B4580: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830B4584: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 830B4588: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B458C: 4BFFDD05  bl 0x830b2290
	ctx.lr = 0x830B4590;
	sub_830B2290(ctx, base);
	// 830B4590: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830B4594: 41820008  beq 0x830b459c
	if ctx.cr[0].eq {
	pc = 0x830B459C; continue 'dispatch;
	}
	// 830B4598: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 830B459C: 37BF0004  addic. r29, r31, 4
	ctx.xer.ca = (ctx.r[31].u32 > (!(4 as u32)));
	ctx.r[29].s64 = ctx.r[31].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830B45A0: 41820008  beq 0x830b45a8
	if ctx.cr[0].eq {
	pc = 0x830B45A8; continue 'dispatch;
	}
	// 830B45A4: 939D0000  stw r28, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830B45A8: 357F0008  addic. r11, r31, 8
	ctx.xer.ca = (ctx.r[31].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B45AC: 4182000C  beq 0x830b45b8
	if ctx.cr[0].eq {
	pc = 0x830B45B8; continue 'dispatch;
	}
	// 830B45B0: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B45B4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830B45B8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830B45BC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830B45C0: 4BB15839  bl 0x82bc9df8
	ctx.lr = 0x830B45C4;
	sub_82BC9DF8(ctx, base);
	// 830B45C4: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 830B45C8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B45CC: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 830B45D0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830B45D4: 480F3BDC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B45D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B45D8 size=132
    let mut pc: u32 = 0x830B45D8;
    'dispatch: loop {
        match pc {
            0x830B45D8 => {
    //   block [0x830B45D8..0x830B465C)
	// 830B45D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B45DC: 480F3B8D  bl 0x831a8168
	ctx.lr = 0x830B45E0;
	sub_831A8130(ctx, base);
	// 830B45E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B45E4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830B45E8: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 830B45EC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830B45F0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 830B45F4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B45F8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B45FC: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830B4600: 409A0044  bne cr6, 0x830b4644
	if !ctx.cr[6].eq {
	pc = 0x830B4644; continue 'dispatch;
	}
	// 830B4604: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B4608: 409A003C  bne cr6, 0x830b4644
	if !ctx.cr[6].eq {
	pc = 0x830B4644; continue 'dispatch;
	}
	// 830B460C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B4610: 4BFFF161  bl 0x830b3770
	ctx.lr = 0x830B4614;
	sub_830B3770(ctx, base);
	// 830B4614: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B4618: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B461C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B4620: 48000030  b 0x830b4650
	pc = 0x830B4650; continue 'dispatch;
	// 830B4624: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 830B4628: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830B462C: 4B7A638D  bl 0x8285a9b8
	ctx.lr = 0x830B4630;
	sub_8285A9B8(ctx, base);
	// 830B4630: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830B4634: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B4638: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B463C: 4BFFF3C5  bl 0x830b3a00
	ctx.lr = 0x830B4640;
	sub_830B3A00(ctx, base);
	// 830B4640: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 830B4644: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 830B4648: 409AFFDC  bne cr6, 0x830b4624
	if !ctx.cr[6].eq {
	pc = 0x830B4624; continue 'dispatch;
	}
	// 830B464C: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 830B4650: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B4654: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B4658: 480F3B60  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B4660 size=368
    let mut pc: u32 = 0x830B4660;
    'dispatch: loop {
        match pc {
            0x830B4660 => {
    //   block [0x830B4660..0x830B47D0)
	// 830B4660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B4664: 480F3AF9  bl 0x831a815c
	ctx.lr = 0x830B4668;
	sub_831A8130(ctx, base);
	// 830B4668: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B466C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830B4670: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 830B4674: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830B4678: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 830B467C: 419A013C  beq cr6, 0x830b47b8
	if ctx.cr[6].eq {
	pc = 0x830B47B8; continue 'dispatch;
	}
	// 830B4680: 83DC0004  lwz r30, 4(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B4684: 3B7F00C0  addi r27, r31, 0xc0
	ctx.r[27].s64 = ctx.r[31].s64 + 192;
	// 830B4688: 83BC0008  lwz r29, 8(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B468C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 830B4690: 7FDAF378  mr r26, r30
	ctx.r[26].u64 = ctx.r[30].u64;
	// 830B4694: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830B4698: 93410060  stw r26, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[26].u32 ) };
	// 830B469C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 830B46A0: 93A10064  stw r29, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 830B46A4: 4BFFED6D  bl 0x830b3410
	ctx.lr = 0x830B46A8;
	sub_830B3410(ctx, base);
	// 830B46A8: 815F00C4  lwz r10, 0xc4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 830B46AC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830B46B0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830B46B4: 419A0014  beq cr6, 0x830b46c8
	if ctx.cr[6].eq {
	pc = 0x830B46C8; continue 'dispatch;
	}
	// 830B46B8: 388B0014  addi r4, r11, 0x14
	ctx.r[4].s64 = ctx.r[11].s64 + 20;
	// 830B46BC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 830B46C0: 4B41C5E1  bl 0x824d0ca0
	ctx.lr = 0x830B46C4;
	sub_824D0CA0(ctx, base);
	// 830B46C4: 48000100  b 0x830b47c4
	pc = 0x830B47C4; continue 'dispatch;
	// 830B46C8: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B46CC: 57CB2036  slwi r11, r30, 4
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830B46D0: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 830B46D4: 57A8103A  slwi r8, r29, 2
	ctx.r[8].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 830B46D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830B46DC: 814A0028  lwz r10, 0x28(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 830B46E0: 98E90000  stb r7, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 830B46E4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 830B46E8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B46EC: 7FCB402E  lwzx r30, r11, r8
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 830B46F0: 48000559  bl 0x830b4c48
	ctx.lr = 0x830B46F4;
	sub_830B4C48(ctx, base);
	// 830B46F4: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B46F8: 38C003E9  li r6, 0x3e9
	ctx.r[6].s64 = 1001;
	// 830B46FC: 38AB74D0  addi r5, r11, 0x74d0
	ctx.r[5].s64 = ctx.r[11].s64 + 29904;
	// 830B4700: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4704: 38800064  li r4, 0x64
	ctx.r[4].s64 = 100;
	// 830B4708: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B470C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B4710: 4E800421  bctrl
	ctx.lr = 0x830B4714;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B4714: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B4718: 41820028  beq 0x830b4740
	if ctx.cr[0].eq {
	pc = 0x830B4740; continue 'dispatch;
	}
	// 830B471C: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 830B4720: 813F0074  lwz r9, 0x74(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 830B4724: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 830B4728: 80DF0078  lwz r6, 0x78(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 830B472C: 38BF0004  addi r5, r31, 4
	ctx.r[5].s64 = ctx.r[31].s64 + 4;
	// 830B4730: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B4734: 480038DD  bl 0x830b8010
	ctx.lr = 0x830B4738;
	sub_830B8010(ctx, base);
	// 830B4738: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830B473C: 48000008  b 0x830b4744
	pc = 0x830B4744; continue 'dispatch;
	// 830B4740: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B4744: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830B4748: 88A10050  lbz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B474C: 4BFFE105  bl 0x830b2850
	ctx.lr = 0x830B4750;
	sub_830B2850(ctx, base);
	// 830B4750: 93410070  stw r26, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[26].u32 ) };
	// 830B4754: 93A10074  stw r29, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 830B4758: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 830B475C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 830B4760: 4B41C541  bl 0x824d0ca0
	ctx.lr = 0x830B4764;
	sub_824D0CA0(ctx, base);
	// 830B4764: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 830B4768: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830B476C: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 830B4770: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 830B4774: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 830B4778: 91410084  stw r10, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[10].u32 ) };
	// 830B477C: 4B41C525  bl 0x824d0ca0
	ctx.lr = 0x830B4780;
	sub_824D0CA0(ctx, base);
	// 830B4780: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 830B4784: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830B4788: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830B478C: 4BFFFC55  bl 0x830b43e0
	ctx.lr = 0x830B4790;
	sub_830B43E0(ctx, base);
	// 830B4790: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 830B4794: 4BFFD745  bl 0x830b1ed8
	ctx.lr = 0x830B4798;
	sub_830B1ED8(ctx, base);
	// 830B4798: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 830B479C: 4BFFD73D  bl 0x830b1ed8
	ctx.lr = 0x830B47A0;
	sub_830B1ED8(ctx, base);
	// 830B47A0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 830B47A4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 830B47A8: 4B41C4F9  bl 0x824d0ca0
	ctx.lr = 0x830B47AC;
	sub_824D0CA0(ctx, base);
	// 830B47AC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830B47B0: 4BFFD729  bl 0x830b1ed8
	ctx.lr = 0x830B47B4;
	sub_830B1ED8(ctx, base);
	// 830B47B4: 48000010  b 0x830b47c4
	pc = 0x830B47C4; continue 'dispatch;
	// 830B47B8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B47BC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 830B47C0: 4BFFDA11  bl 0x830b21d0
	ctx.lr = 0x830B47C4;
	sub_830B21D0(ctx, base);
	// 830B47C4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 830B47C8: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 830B47CC: 480F39E0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B47D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B47D0 size=144
    let mut pc: u32 = 0x830B47D0;
    'dispatch: loop {
        match pc {
            0x830B47D0 => {
    //   block [0x830B47D0..0x830B4860)
	// 830B47D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B47D4: 480F398D  bl 0x831a8160
	ctx.lr = 0x830B47D8;
	sub_831A8130(ctx, base);
	// 830B47D8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B47DC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830B47E0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830B47E4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830B47E8: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B47EC: 83CB002C  lwz r30, 0x2c(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830B47F0: 83EB0030  lwz r31, 0x30(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830B47F4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830B47F8: 40990060  ble cr6, 0x830b4858
	if !ctx.cr[6].gt {
	pc = 0x830B4858; continue 'dispatch;
	}
	// 830B47FC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830B4800: 4BFFD729  bl 0x830b1f28
	ctx.lr = 0x830B4804;
	sub_830B1F28(ctx, base);
	// 830B4804: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830B4808: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830B480C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830B4810: 4BFFFE51  bl 0x830b4660
	ctx.lr = 0x830B4814;
	sub_830B4660(ctx, base);
	// 830B4814: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830B4818: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830B481C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B4820: 4BFFFE41  bl 0x830b4660
	ctx.lr = 0x830B4824;
	sub_830B4660(ctx, base);
	// 830B4824: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B4828: 3B410058  addi r26, r1, 0x58
	ctx.r[26].s64 = ctx.r[1].s64 + 88;
	// 830B482C: 4BFFD6FD  bl 0x830b1f28
	ctx.lr = 0x830B4830;
	sub_830B1F28(ctx, base);
	// 830B4830: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830B4834: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 830B4838: 480036B9  bl 0x830b7ef0
	ctx.lr = 0x830B483C;
	sub_830B7EF0(ctx, base);
	// 830B483C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B4840: 4BFFD699  bl 0x830b1ed8
	ctx.lr = 0x830B4844;
	sub_830B1ED8(ctx, base);
	// 830B4844: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830B4848: 4BFFD691  bl 0x830b1ed8
	ctx.lr = 0x830B484C;
	sub_830B1ED8(ctx, base);
	// 830B484C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830B4850: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 830B4854: 4082FFA8  bne 0x830b47fc
	if !ctx.cr[0].eq {
	pc = 0x830B47FC; continue 'dispatch;
	}
	// 830B4858: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 830B485C: 480F3954  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B4860 size=100
    let mut pc: u32 = 0x830B4860;
    'dispatch: loop {
        match pc {
            0x830B4860 => {
    //   block [0x830B4860..0x830B48C4)
	// 830B4860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B4864: 480F3905  bl 0x831a8168
	ctx.lr = 0x830B4868;
	sub_831A8130(ctx, base);
	// 830B4868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B486C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830B4870: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830B4874: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B4878: 83EB002C  lwz r31, 0x2c(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830B487C: 83CB0030  lwz r30, 0x30(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830B4880: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830B4884: 40990038  ble cr6, 0x830b48bc
	if !ctx.cr[6].gt {
	pc = 0x830B48BC; continue 'dispatch;
	}
	// 830B4888: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830B488C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830B4890: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B4894: 4BFFFDCD  bl 0x830b4660
	ctx.lr = 0x830B4898;
	sub_830B4660(ctx, base);
	// 830B4898: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B489C: 4BFFD68D  bl 0x830b1f28
	ctx.lr = 0x830B48A0;
	sub_830B1F28(ctx, base);
	// 830B48A0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830B48A4: 480036DD  bl 0x830b7f80
	ctx.lr = 0x830B48A8;
	sub_830B7F80(ctx, base);
	// 830B48A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B48AC: 4BFFD62D  bl 0x830b1ed8
	ctx.lr = 0x830B48B0;
	sub_830B1ED8(ctx, base);
	// 830B48B0: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830B48B4: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 830B48B8: 4082FFD0  bne 0x830b4888
	if !ctx.cr[0].eq {
	pc = 0x830B4888; continue 'dispatch;
	}
	// 830B48BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B48C0: 480F38F8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B48C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B48C8 size=80
    let mut pc: u32 = 0x830B48C8;
    'dispatch: loop {
        match pc {
            0x830B48C8 => {
    //   block [0x830B48C8..0x830B4918)
	// 830B48C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B48CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B48D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B48D4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B48D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B48DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B48E0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B48E4: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B48E8: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B48EC: 4BFFFCED  bl 0x830b45d8
	ctx.lr = 0x830B48F0;
	sub_830B45D8(ctx, base);
	// 830B48F0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B48F4: 4B20B975  bl 0x822c0268
	ctx.lr = 0x830B48F8;
	sub_822C0268(ctx, base);
	// 830B48F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B48FC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830B4900: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830B4904: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B4908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B490C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B4910: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B4914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B4918 size=104
    let mut pc: u32 = 0x830B4918;
    'dispatch: loop {
        match pc {
            0x830B4918 => {
    //   block [0x830B4918..0x830B4980)
	// 830B4918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B491C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B4920: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B4924: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B4928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B492C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B4930: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B4934: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B4938: 4BFFDC79  bl 0x830b25b0
	ctx.lr = 0x830B493C;
	sub_830B25B0(ctx, base);
	// 830B493C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B4940: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B4944: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B4948: 419A0014  beq cr6, 0x830b495c
	if ctx.cr[6].eq {
	pc = 0x830B495C; continue 'dispatch;
	}
	// 830B494C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 830B4950: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B4954: 4BFFFD0D  bl 0x830b4660
	ctx.lr = 0x830B4958;
	sub_830B4660(ctx, base);
	// 830B4958: 4800000C  b 0x830b4964
	pc = 0x830B4964; continue 'dispatch;
	// 830B495C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B4960: 4BFFD871  bl 0x830b21d0
	ctx.lr = 0x830B4964;
	sub_830B21D0(ctx, base);
	// 830B4964: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B4968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B496C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B4970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B4974: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B4978: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B497C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B4980 size=436
    let mut pc: u32 = 0x830B4980;
    'dispatch: loop {
        match pc {
            0x830B4980 => {
    //   block [0x830B4980..0x830B4B34)
	// 830B4980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B4984: 480F37E5  bl 0x831a8168
	ctx.lr = 0x830B4988;
	sub_831A8130(ctx, base);
	// 830B4988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B498C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B4990: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B4994: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830B4998: 396B757C  addi r11, r11, 0x757c
	ctx.r[11].s64 = ctx.r[11].s64 + 30076;
	// 830B499C: 394A7574  addi r10, r10, 0x7574
	ctx.r[10].s64 = ctx.r[10].s64 + 30068;
	// 830B49A0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B49A4: 3B9F0010  addi r28, r31, 0x10
	ctx.r[28].s64 = ctx.r[31].s64 + 16;
	// 830B49A8: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 830B49AC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830B49B0: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B49B4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B49B8: 419A0028  beq cr6, 0x830b49e0
	if ctx.cr[6].eq {
	pc = 0x830B49E0; continue 'dispatch;
	}
	// 830B49BC: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 830B49C0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B49C4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B49C8: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830B49CC: 4BFFEAD5  bl 0x830b34a0
	ctx.lr = 0x830B49D0;
	sub_830B34A0(ctx, base);
	// 830B49D0: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B49D4: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B49D8: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B49DC: 409AFFE4  bne cr6, 0x830b49c0
	if !ctx.cr[6].eq {
	pc = 0x830B49C0; continue 'dispatch;
	}
	// 830B49E0: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 830B49E4: 48002B45  bl 0x830b7528
	ctx.lr = 0x830B49E8;
	sub_830B7528(ctx, base);
	// 830B49E8: 815F00C4  lwz r10, 0xc4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 830B49EC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830B49F0: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B49F4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830B49F8: 48000024  b 0x830b4a1c
	pc = 0x830B4A1C; continue 'dispatch;
	// 830B49FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B4A00: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 830B4A04: 4B7A5FB5  bl 0x8285a9b8
	ctx.lr = 0x830B4A08;
	sub_8285A9B8(ctx, base);
	// 830B4A08: 387D0014  addi r3, r29, 0x14
	ctx.r[3].s64 = ctx.r[29].s64 + 20;
	// 830B4A0C: 4BFFD51D  bl 0x830b1f28
	ctx.lr = 0x830B4A10;
	sub_830B1F28(ctx, base);
	// 830B4A10: 93C30040  stw r30, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 830B4A14: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B4A18: 815F00C4  lwz r10, 0xc4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 830B4A1C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830B4A20: 409AFFDC  bne cr6, 0x830b49fc
	if !ctx.cr[6].eq {
	pc = 0x830B49FC; continue 'dispatch;
	}
	// 830B4A24: 807F0070  lwz r3, 0x70(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 830B4A28: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B4A2C: 419A001C  beq cr6, 0x830b4a48
	if ctx.cr[6].eq {
	pc = 0x830B4A48; continue 'dispatch;
	}
	// 830B4A30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4A34: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B4A38: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4A3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B4A40: 4E800421  bctrl
	ctx.lr = 0x830B4A44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B4A44: 93DF0070  stw r30, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[30].u32 ) };
	// 830B4A48: 807F0074  lwz r3, 0x74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 830B4A4C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B4A50: 419A001C  beq cr6, 0x830b4a6c
	if ctx.cr[6].eq {
	pc = 0x830B4A6C; continue 'dispatch;
	}
	// 830B4A54: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4A58: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B4A5C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4A60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B4A64: 4E800421  bctrl
	ctx.lr = 0x830B4A68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B4A68: 93DF0074  stw r30, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 830B4A6C: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 830B4A70: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B4A74: 419A001C  beq cr6, 0x830b4a90
	if ctx.cr[6].eq {
	pc = 0x830B4A90; continue 'dispatch;
	}
	// 830B4A78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4A7C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B4A80: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4A84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B4A88: 4E800421  bctrl
	ctx.lr = 0x830B4A8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B4A8C: 93DF0078  stw r30, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[30].u32 ) };
	// 830B4A90: 807F00DC  lwz r3, 0xdc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 830B4A94: 480001ED  bl 0x830b4c80
	ctx.lr = 0x830B4A98;
	sub_830B4C80(ctx, base);
	// 830B4A98: 807F0098  lwz r3, 0x98(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 830B4A9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B4AA0: 419A001C  beq cr6, 0x830b4abc
	if ctx.cr[6].eq {
	pc = 0x830B4ABC; continue 'dispatch;
	}
	// 830B4AA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4AA8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830B4AAC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4AB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B4AB4: 4E800421  bctrl
	ctx.lr = 0x830B4AB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B4AB8: 93DF0098  stw r30, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[30].u32 ) };
	// 830B4ABC: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 830B4AC0: 4BFFD419  bl 0x830b1ed8
	ctx.lr = 0x830B4AC4;
	sub_830B1ED8(ctx, base);
	// 830B4AC4: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 830B4AC8: 4BFFFE01  bl 0x830b48c8
	ctx.lr = 0x830B4ACC;
	sub_830B48C8(ctx, base);
	// 830B4ACC: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 830B4AD0: 480039C9  bl 0x830b8498
	ctx.lr = 0x830B4AD4;
	sub_830B8498(ctx, base);
	// 830B4AD4: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 830B4AD8: 4BFFD401  bl 0x830b1ed8
	ctx.lr = 0x830B4ADC;
	sub_830B1ED8(ctx, base);
	// 830B4ADC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830B4AE0: 4BFFFA39  bl 0x830b4518
	ctx.lr = 0x830B4AE4;
	sub_830B4518(ctx, base);
	// 830B4AE4: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 830B4AE8: 4BFFFA31  bl 0x830b4518
	ctx.lr = 0x830B4AEC;
	sub_830B4518(ctx, base);
	// 830B4AEC: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 830B4AF0: 4BFFFA29  bl 0x830b4518
	ctx.lr = 0x830B4AF4;
	sub_830B4518(ctx, base);
	// 830B4AF4: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 830B4AF8: 4BFFFA21  bl 0x830b4518
	ctx.lr = 0x830B4AFC;
	sub_830B4518(ctx, base);
	// 830B4AFC: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 830B4B00: 4BFFFA19  bl 0x830b4518
	ctx.lr = 0x830B4B04;
	sub_830B4518(ctx, base);
	// 830B4B04: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 830B4B08: 4BFFFA11  bl 0x830b4518
	ctx.lr = 0x830B4B0C;
	sub_830B4518(ctx, base);
	// 830B4B0C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830B4B10: 4BFFEA41  bl 0x830b3550
	ctx.lr = 0x830B4B14;
	sub_830B3550(ctx, base);
	// 830B4B14: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B4B18: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 830B4B1C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 830B4B20: 396B74B8  addi r11, r11, 0x74b8
	ctx.r[11].s64 = ctx.r[11].s64 + 29880;
	// 830B4B24: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B4B28: 4BFFD3B1  bl 0x830b1ed8
	ctx.lr = 0x830B4B2C;
	sub_830B1ED8(ctx, base);
	// 830B4B2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B4B30: 480F3688  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B4B38 size=8
    let mut pc: u32 = 0x830B4B38;
    'dispatch: loop {
        match pc {
            0x830B4B38 => {
    //   block [0x830B4B38..0x830B4B40)
	// 830B4B38: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 830B4B3C: 48000004  b 0x830b4b40
	sub_830B4B40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B4B40 size=92
    let mut pc: u32 = 0x830B4B40;
    'dispatch: loop {
        match pc {
            0x830B4B40 => {
    //   block [0x830B4B40..0x830B4B9C)
	// 830B4B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B4B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B4B48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B4B4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B4B50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B4B54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B4B58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B4B5C: 4BFFFE25  bl 0x830b4980
	ctx.lr = 0x830B4B60;
	sub_830B4980(ctx, base);
	// 830B4B60: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B4B64: 4182001C  beq 0x830b4b80
	if ctx.cr[0].eq {
	pc = 0x830B4B80; continue 'dispatch;
	}
	// 830B4B68: 480000E1  bl 0x830b4c48
	ctx.lr = 0x830B4B6C;
	sub_830B4C48(ctx, base);
	// 830B4B6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4B70: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B4B74: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830B4B78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B4B7C: 4E800421  bctrl
	ctx.lr = 0x830B4B80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B4B80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B4B84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B4B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B4B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B4B90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B4B94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B4B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B4BA0 size=120
    let mut pc: u32 = 0x830B4BA0;
    'dispatch: loop {
        match pc {
            0x830B4BA0 => {
    //   block [0x830B4BA0..0x830B4C18)
	// 830B4BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B4BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B4BA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B4BAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B4BB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B4BB4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B4BB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B4BBC: 93C1008C  stw r30, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[30].u32 ) };
	// 830B4BC0: 38A1008C  addi r5, r1, 0x8c
	ctx.r[5].s64 = ctx.r[1].s64 + 140;
	// 830B4BC4: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 830B4BC8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B4BCC: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4BD0: 4BFFF999  bl 0x830b4568
	ctx.lr = 0x830B4BD4;
	sub_830B4568(ctx, base);
	// 830B4BD4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4BD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B4BDC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B4BE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B4BE4: 4E800421  bctrl
	ctx.lr = 0x830B4BE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B4BE8: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830B4BEC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B4BF0: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 830B4BF4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 830B4BF8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4BFC: 4BFFF96D  bl 0x830b4568
	ctx.lr = 0x830B4C00;
	sub_830B4568(ctx, base);
	// 830B4C00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B4C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B4C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B4C0C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B4C10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B4C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B4C18 size=8
    let mut pc: u32 = 0x830B4C18;
    'dispatch: loop {
        match pc {
            0x830B4C18 => {
    //   block [0x830B4C18..0x830B4C20)
	// 830B4C18: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 830B4C1C: 4BFFFF84  b 0x830b4ba0
	sub_830B4BA0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B4C20 size=28
    let mut pc: u32 = 0x830B4C20;
    'dispatch: loop {
        match pc {
            0x830B4C20 => {
    //   block [0x830B4C20..0x830B4C3C)
	// 830B4C20: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830B4C24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B4C28: 419A0014  beq cr6, 0x830b4c3c
	if ctx.cr[6].eq {
		sub_830B4C3C(ctx, base);
		return;
	}
	// 830B4C2C: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830B4C30: 806ABC9C  lwz r3, -0x4364(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17252 as u32) ) } as u64;
	// 830B4C34: 916ABC9C  stw r11, -0x4364(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17252 as u32), ctx.r[11].u32 ) };
	// 830B4C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4C3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B4C3C size=12
    let mut pc: u32 = 0x830B4C3C;
    'dispatch: loop {
        match pc {
            0x830B4C3C => {
    //   block [0x830B4C3C..0x830B4C48)
	// 830B4C3C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830B4C40: 806BBC9C  lwz r3, -0x4364(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17252 as u32) ) } as u64;
	// 830B4C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B4C48 size=12
    let mut pc: u32 = 0x830B4C48;
    'dispatch: loop {
        match pc {
            0x830B4C48 => {
    //   block [0x830B4C48..0x830B4C54)
	// 830B4C48: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830B4C4C: 806BBC9C  lwz r3, -0x4364(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17252 as u32) ) } as u64;
	// 830B4C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B4C58 size=36
    let mut pc: u32 = 0x830B4C58;
    'dispatch: loop {
        match pc {
            0x830B4C58 => {
    //   block [0x830B4C58..0x830B4C7C)
	// 830B4C58: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830B4C5C: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 830B4C60: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 830B4C64: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830B4C68: 806BBC9C  lwz r3, -0x4364(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17252 as u32) ) } as u64;
	// 830B4C6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4C70: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 830B4C74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B4C78: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B4C80 size=28
    let mut pc: u32 = 0x830B4C80;
    'dispatch: loop {
        match pc {
            0x830B4C80 => {
    //   block [0x830B4C80..0x830B4C9C)
	// 830B4C80: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830B4C84: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830B4C88: 806BBC9C  lwz r3, -0x4364(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17252 as u32) ) } as u64;
	// 830B4C8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4C90: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830B4C94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B4C98: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B4CA0 size=12
    let mut pc: u32 = 0x830B4CA0;
    'dispatch: loop {
        match pc {
            0x830B4CA0 => {
    //   block [0x830B4CA0..0x830B4CAC)
	// 830B4CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B4CA4: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 830B4CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B4CB0 size=84
    let mut pc: u32 = 0x830B4CB0;
    'dispatch: loop {
        match pc {
            0x830B4CB0 => {
    //   block [0x830B4CB0..0x830B4D04)
	// 830B4CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B4CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B4CB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B4CBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B4CC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B4CC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B4CC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B4CCC: 4BFFD1F5  bl 0x830b1ec0
	ctx.lr = 0x830B4CD0;
	sub_830B1EC0(ctx, base);
	// 830B4CD0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B4CD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B4CD8: 396B758C  addi r11, r11, 0x758c
	ctx.r[11].s64 = ctx.r[11].s64 + 30092;
	// 830B4CDC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B4CE0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B4CE4: 4BFFD3CD  bl 0x830b20b0
	ctx.lr = 0x830B4CE8;
	sub_830B20B0(ctx, base);
	// 830B4CE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B4CEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B4CF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B4CF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B4CF8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B4CFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B4D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B4D08 size=84
    let mut pc: u32 = 0x830B4D08;
    'dispatch: loop {
        match pc {
            0x830B4D08 => {
    //   block [0x830B4D08..0x830B4D5C)
	// 830B4D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B4D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B4D10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B4D14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B4D18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B4D1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B4D20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B4D24: 4BFFD19D  bl 0x830b1ec0
	ctx.lr = 0x830B4D28;
	sub_830B1EC0(ctx, base);
	// 830B4D28: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B4D2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B4D30: 396B758C  addi r11, r11, 0x758c
	ctx.r[11].s64 = ctx.r[11].s64 + 30092;
	// 830B4D34: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B4D38: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B4D3C: 4BFFD2ED  bl 0x830b2028
	ctx.lr = 0x830B4D40;
	sub_830B2028(ctx, base);
	// 830B4D40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B4D44: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B4D48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B4D4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B4D50: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B4D54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B4D58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B4D60 size=52
    let mut pc: u32 = 0x830B4D60;
    'dispatch: loop {
        match pc {
            0x830B4D60 => {
    //   block [0x830B4D60..0x830B4D94)
	// 830B4D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B4D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B4D68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B4D6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B4D70: 3884002C  addi r4, r4, 0x2c
	ctx.r[4].s64 = ctx.r[4].s64 + 44;
	// 830B4D74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B4D78: 4B412BF9  bl 0x824c7970
	ctx.lr = 0x830B4D7C;
	sub_824C7970(ctx, base);
	// 830B4D7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B4D80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B4D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B4D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B4D8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B4D90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B4D98 size=112
    let mut pc: u32 = 0x830B4D98;
    'dispatch: loop {
        match pc {
            0x830B4D98 => {
    //   block [0x830B4D98..0x830B4E08)
	// 830B4D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B4D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B4DA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B4DA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B4DA8: 4BFFFEA1  bl 0x830b4c48
	ctx.lr = 0x830B4DAC;
	sub_830B4C48(ctx, base);
	// 830B4DAC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B4DB0: 38C000A5  li r6, 0xa5
	ctx.r[6].s64 = 165;
	// 830B4DB4: 38AB7510  addi r5, r11, 0x7510
	ctx.r[5].s64 = ctx.r[11].s64 + 29968;
	// 830B4DB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4DBC: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 830B4DC0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B4DC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B4DC8: 4E800421  bctrl
	ctx.lr = 0x830B4DCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B4DCC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830B4DD0: 41820020  beq 0x830b4df0
	if ctx.cr[0].eq {
	pc = 0x830B4DF0; continue 'dispatch;
	}
	// 830B4DD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B4DD8: 4BFFCF69  bl 0x830b1d40
	ctx.lr = 0x830B4DDC;
	sub_830B1D40(ctx, base);
	// 830B4DDC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B4DE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B4DE4: 396B7594  addi r11, r11, 0x7594
	ctx.r[11].s64 = ctx.r[11].s64 + 30100;
	// 830B4DE8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B4DEC: 48000008  b 0x830b4df4
	pc = 0x830B4DF4; continue 'dispatch;
	// 830B4DF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830B4DF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B4DF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B4DFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B4E00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B4E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B4E08 size=20
    let mut pc: u32 = 0x830B4E08;
    'dispatch: loop {
        match pc {
            0x830B4E08 => {
    //   block [0x830B4E08..0x830B4E1C)
	// 830B4E08: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B4E0C: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B4E10: 89690019  lbz r11, 0x19(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(25 as u32) ) } as u64;
	// 830B4E14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B4E18: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4E1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B4E1C size=64
    let mut pc: u32 = 0x830B4E1C;
    'dispatch: loop {
        match pc {
            0x830B4E1C => {
    //   block [0x830B4E1C..0x830B4E5C)
	// 830B4E1C: 80E40000  lwz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4E20: 8169000C  lwz r11, 0xc(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B4E24: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 830B4E28: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4E2C: 88CA0000  lbz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4E30: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830B4E34: 7D064050  subf r8, r6, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[6].s64;
	// 830B4E38: 41820014  beq 0x830b4e4c
	if ctx.cr[0].eq {
	pc = 0x830B4E4C; continue 'dispatch;
	}
	// 830B4E3C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830B4E40: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830B4E44: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 830B4E48: 419AFFE0  beq cr6, 0x830b4e28
	if ctx.cr[6].eq {
	pc = 0x830B4E28; continue 'dispatch;
	}
	// 830B4E4C: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830B4E50: 4080000C  bge 0x830b4e5c
	if !ctx.cr[0].lt {
		sub_830B4E5C(ctx, base);
		return;
	}
	// 830B4E54: 81290008  lwz r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B4E58: 4800000C  b 0x830b4e64
	sub_830B4E5C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4E5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B4E5C size=24
    let mut pc: u32 = 0x830B4E5C;
    'dispatch: loop {
        match pc {
            0x830B4E5C => {
    //   block [0x830B4E5C..0x830B4E74)
	// 830B4E5C: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 830B4E60: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4E64: 89690019  lbz r11, 0x19(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(25 as u32) ) } as u64;
	// 830B4E68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B4E6C: 419AFFB4  beq cr6, 0x830b4e20
	if ctx.cr[6].eq {
		sub_830B4E1C(ctx, base);
		return;
	}
	// 830B4E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B4E78 size=20
    let mut pc: u32 = 0x830B4E78;
    'dispatch: loop {
        match pc {
            0x830B4E78 => {
    //   block [0x830B4E78..0x830B4E8C)
	// 830B4E78: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B4E7C: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B4E80: 89690019  lbz r11, 0x19(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(25 as u32) ) } as u64;
	// 830B4E84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B4E88: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4E8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B4E8C size=68
    let mut pc: u32 = 0x830B4E8C;
    'dispatch: loop {
        match pc {
            0x830B4E8C => {
    //   block [0x830B4E8C..0x830B4ED0)
	// 830B4E8C: 80E40000  lwz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4E90: 8149000C  lwz r10, 0xc(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B4E94: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 830B4E98: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4E9C: 88CA0000  lbz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4EA0: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830B4EA4: 7D064050  subf r8, r6, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[6].s64;
	// 830B4EA8: 41820014  beq 0x830b4ebc
	if ctx.cr[0].eq {
	pc = 0x830B4EBC; continue 'dispatch;
	}
	// 830B4EAC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830B4EB0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830B4EB4: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 830B4EB8: 419AFFE0  beq cr6, 0x830b4e98
	if ctx.cr[6].eq {
	pc = 0x830B4E98; continue 'dispatch;
	}
	// 830B4EBC: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830B4EC0: 40800010  bge 0x830b4ed0
	if !ctx.cr[0].lt {
		sub_830B4ED0(ctx, base);
		return;
	}
	// 830B4EC4: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 830B4EC8: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4ECC: 48000008  b 0x830b4ed4
	sub_830B4ED0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B4ED0 size=20
    let mut pc: u32 = 0x830B4ED0;
    'dispatch: loop {
        match pc {
            0x830B4ED0 => {
    //   block [0x830B4ED0..0x830B4EE4)
	// 830B4ED0: 81290008  lwz r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B4ED4: 89690019  lbz r11, 0x19(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(25 as u32) ) } as u64;
	// 830B4ED8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B4EDC: 419AFFB4  beq cr6, 0x830b4e90
	if ctx.cr[6].eq {
		sub_830B4E8C(ctx, base);
		return;
	}
	// 830B4EE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B4EE8 size=20
    let mut pc: u32 = 0x830B4EE8;
    'dispatch: loop {
        match pc {
            0x830B4EE8 => {
    //   block [0x830B4EE8..0x830B4EFC)
	// 830B4EE8: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B4EEC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B4EF0: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 830B4EF4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B4EF8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4EFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B4EFC size=40
    let mut pc: u32 = 0x830B4EFC;
    'dispatch: loop {
        match pc {
            0x830B4EFC => {
    //   block [0x830B4EFC..0x830B4F24)
	// 830B4EFC: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B4F00: C1AB000C  lfs f13, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B4F04: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830B4F08: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 830B4F0C: 41980008  blt cr6, 0x830b4f14
	if ctx.cr[6].lt {
	pc = 0x830B4F14; continue 'dispatch;
	}
	// 830B4F10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B4F14: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830B4F18: 4182000C  beq 0x830b4f24
	if ctx.cr[0].eq {
		sub_830B4F24(ctx, base);
		return;
	}
	// 830B4F1C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B4F20: 4800000C  b 0x830b4f2c
	sub_830B4F24(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4F24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B4F24 size=24
    let mut pc: u32 = 0x830B4F24;
    'dispatch: loop {
        match pc {
            0x830B4F24 => {
    //   block [0x830B4F24..0x830B4F3C)
	// 830B4F24: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830B4F28: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4F2C: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 830B4F30: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B4F34: 419AFFCC  beq cr6, 0x830b4f00
	if ctx.cr[6].eq {
		sub_830B4EFC(ctx, base);
		return;
	}
	// 830B4F38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B4F40 size=20
    let mut pc: u32 = 0x830B4F40;
    'dispatch: loop {
        match pc {
            0x830B4F40 => {
    //   block [0x830B4F40..0x830B4F54)
	// 830B4F40: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B4F44: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B4F48: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 830B4F4C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B4F50: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4F54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B4F54 size=44
    let mut pc: u32 = 0x830B4F54;
    'dispatch: loop {
        match pc {
            0x830B4F54 => {
    //   block [0x830B4F54..0x830B4F80)
	// 830B4F54: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B4F58: C1AB000C  lfs f13, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B4F5C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830B4F60: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 830B4F64: 41980008  blt cr6, 0x830b4f6c
	if ctx.cr[6].lt {
	pc = 0x830B4F6C; continue 'dispatch;
	}
	// 830B4F68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B4F6C: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830B4F70: 41820010  beq 0x830b4f80
	if ctx.cr[0].eq {
		sub_830B4F80(ctx, base);
		return;
	}
	// 830B4F74: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830B4F78: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4F7C: 48000008  b 0x830b4f84
	sub_830B4F80(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B4F80 size=20
    let mut pc: u32 = 0x830B4F80;
    'dispatch: loop {
        match pc {
            0x830B4F80 => {
    //   block [0x830B4F80..0x830B4F94)
	// 830B4F80: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B4F84: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 830B4F88: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B4F8C: 419AFFCC  beq cr6, 0x830b4f58
	if ctx.cr[6].eq {
		sub_830B4F54(ctx, base);
		return;
	}
	// 830B4F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B4F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B4F98 size=104
    let mut pc: u32 = 0x830B4F98;
    'dispatch: loop {
        match pc {
            0x830B4F98 => {
    //   block [0x830B4F98..0x830B5000)
	// 830B4F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B4F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B4FA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B4FA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B4FA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B4FAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B4FB0: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 830B4FB4: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 830B4FB8: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830B4FBC: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 830B4FC0: 38870004  addi r4, r7, 4
	ctx.r[4].s64 = ctx.r[7].s64 + 4;
	// 830B4FC4: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 830B4FC8: 90DF0008  stw r6, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 830B4FCC: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B4FD0: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830B4FD4: 4B27AD95  bl 0x8232fd68
	ctx.lr = 0x830B4FD8;
	sub_8232FD68(ctx, base);
	// 830B4FD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B4FDC: 9BDF0018  stb r30, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 830B4FE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B4FE4: 997F0019  stb r11, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 830B4FE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B4FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B4FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B4FF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B4FF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B4FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B5000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B5000 size=104
    let mut pc: u32 = 0x830B5000;
    'dispatch: loop {
        match pc {
            0x830B5000 => {
    //   block [0x830B5000..0x830B5068)
	// 830B5000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B5004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B5008: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B500C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B5010: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B5014: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B5018: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 830B501C: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 830B5020: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830B5024: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 830B5028: 38870004  addi r4, r7, 4
	ctx.r[4].s64 = ctx.r[7].s64 + 4;
	// 830B502C: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 830B5030: 90DF0008  stw r6, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 830B5034: C0070000  lfs f0, 0(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B5038: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 830B503C: 4B27AD2D  bl 0x8232fd68
	ctx.lr = 0x830B5040;
	sub_8232FD68(ctx, base);
	// 830B5040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B5044: 9BDF0018  stb r30, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 830B5048: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B504C: 997F0019  stb r11, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 830B5050: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B5054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B5058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B505C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B5060: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B5064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B5068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B5068 size=84
    let mut pc: u32 = 0x830B5068;
    'dispatch: loop {
        match pc {
            0x830B5068 => {
    //   block [0x830B5068..0x830B50BC)
	// 830B5068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B506C: 480F3101  bl 0x831a816c
	ctx.lr = 0x830B5070;
	sub_831A8130(ctx, base);
	// 830B5070: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B5074: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B5078: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830B507C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830B5080: 4BFFCE41  bl 0x830b1ec0
	ctx.lr = 0x830B5084;
	sub_830B1EC0(ctx, base);
	// 830B5084: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B5088: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830B508C: 396B758C  addi r11, r11, 0x758c
	ctx.r[11].s64 = ctx.r[11].s64 + 30092;
	// 830B5090: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B5094: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B5098: 4BFFCF01  bl 0x830b1f98
	ctx.lr = 0x830B509C;
	sub_830B1F98(ctx, base);
	// 830B509C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B50A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B50A4: 419A000C  beq cr6, 0x830b50b0
	if ctx.cr[6].eq {
	pc = 0x830B50B0; continue 'dispatch;
	}
	// 830B50A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B50AC: 4BFFCDE5  bl 0x830b1e90
	ctx.lr = 0x830B50B0;
	sub_830B1E90(ctx, base);
	// 830B50B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B50B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B50B8: 480F3104  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B50C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B50C0 size=88
    let mut pc: u32 = 0x830B50C0;
    'dispatch: loop {
        match pc {
            0x830B50C0 => {
    //   block [0x830B50C0..0x830B5118)
	// 830B50C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B50C4: 480F30A5  bl 0x831a8168
	ctx.lr = 0x830B50C8;
	sub_831A8130(ctx, base);
	// 830B50C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B50CC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B50D0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B50D4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830B50D8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830B50DC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 830B50E0: 4B366C61  bl 0x8241bd40
	ctx.lr = 0x830B50E4;
	sub_8241BD40(ctx, base);
	// 830B50E4: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830B50E8: 41820008  beq 0x830b50f0
	if ctx.cr[0].eq {
	pc = 0x830B50F0; continue 'dispatch;
	}
	// 830B50EC: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 830B50F0: 357F0004  addic. r11, r31, 4
	ctx.xer.ca = (ctx.r[31].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B50F4: 41820008  beq 0x830b50fc
	if ctx.cr[0].eq {
	pc = 0x830B50FC; continue 'dispatch;
	}
	// 830B50F8: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 830B50FC: 347F0008  addic. r3, r31, 8
	ctx.xer.ca = (ctx.r[31].u32 > (!(8 as u32)));
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 830B5100: 4182000C  beq 0x830b510c
	if ctx.cr[0].eq {
	pc = 0x830B510C; continue 'dispatch;
	}
	// 830B5104: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830B5108: 4B27AC61  bl 0x8232fd68
	ctx.lr = 0x830B510C;
	sub_8232FD68(ctx, base);
	// 830B510C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B5110: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B5114: 480F30A4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B5118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B5118 size=92
    let mut pc: u32 = 0x830B5118;
    'dispatch: loop {
        match pc {
            0x830B5118 => {
    //   block [0x830B5118..0x830B5174)
	// 830B5118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B511C: 480F3051  bl 0x831a816c
	ctx.lr = 0x830B5120;
	sub_831A8130(ctx, base);
	// 830B5120: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B5124: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830B5128: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B512C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830B5130: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830B5134: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 830B5138: 4B366C09  bl 0x8241bd40
	ctx.lr = 0x830B513C;
	sub_8241BD40(ctx, base);
	// 830B513C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B5140: 41820008  beq 0x830b5148
	if ctx.cr[0].eq {
	pc = 0x830B5148; continue 'dispatch;
	}
	// 830B5144: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 830B5148: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B514C: 41820008  beq 0x830b5154
	if ctx.cr[0].eq {
	pc = 0x830B5154; continue 'dispatch;
	}
	// 830B5150: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 830B5154: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B5158: 41820014  beq 0x830b516c
	if ctx.cr[0].eq {
	pc = 0x830B516C; continue 'dispatch;
	}
	// 830B515C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5160: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830B5164: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5168: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830B516C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B5170: 480F304C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B5178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B5178 size=92
    let mut pc: u32 = 0x830B5178;
    'dispatch: loop {
        match pc {
            0x830B5178 => {
    //   block [0x830B5178..0x830B51D4)
	// 830B5178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B517C: 480F2FED  bl 0x831a8168
	ctx.lr = 0x830B5180;
	sub_831A8130(ctx, base);
	// 830B5180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B5184: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B5188: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830B518C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830B5190: 4BFFD7F9  bl 0x830b2988
	ctx.lr = 0x830B5194;
	sub_830B2988(ctx, base);
	// 830B5194: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B5198: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 830B519C: 396B7548  addi r11, r11, 0x7548
	ctx.r[11].s64 = ctx.r[11].s64 + 30024;
	// 830B51A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B51A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B51A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B51AC: 4BFFCF05  bl 0x830b20b0
	ctx.lr = 0x830B51B0;
	sub_830B20B0(ctx, base);
	// 830B51B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B51B4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830B51B8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830B51BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B51C0: 4BFFCE69  bl 0x830b2028
	ctx.lr = 0x830B51C4;
	sub_830B2028(ctx, base);
	// 830B51C4: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 830B51C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B51CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B51D0: 480F2FE8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B51D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B51D8 size=200
    let mut pc: u32 = 0x830B51D8;
    'dispatch: loop {
        match pc {
            0x830B51D8 => {
    //   block [0x830B51D8..0x830B52A0)
	// 830B51D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B51DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B51E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B51E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B51E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B51EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B51F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B51F4: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 830B51F8: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 830B51FC: 4BFFCE2D  bl 0x830b2028
	ctx.lr = 0x830B5200;
	sub_830B2028(ctx, base);
	// 830B5200: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B5204: 389E002C  addi r4, r30, 0x2c
	ctx.r[4].s64 = ctx.r[30].s64 + 44;
	// 830B5208: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 830B520C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830B5210: 4BFFCE19  bl 0x830b2028
	ctx.lr = 0x830B5214;
	sub_830B2028(ctx, base);
	// 830B5214: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B5218: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B521C: 419A0020  beq cr6, 0x830b523c
	if ctx.cr[6].eq {
	pc = 0x830B523C; continue 'dispatch;
	}
	// 830B5220: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5224: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830B5228: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B522C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B5230: 4E800421  bctrl
	ctx.lr = 0x830B5234;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B5234: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B5238: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 830B523C: 817E0034  lwz r11, 0x34(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B5240: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B5244: 419A0044  beq cr6, 0x830b5288
	if ctx.cr[6].eq {
	pc = 0x830B5288; continue 'dispatch;
	}
	// 830B5248: 4BFFFA01  bl 0x830b4c48
	ctx.lr = 0x830B524C;
	sub_830B4C48(ctx, base);
	// 830B524C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B5250: 38C00053  li r6, 0x53
	ctx.r[6].s64 = 83;
	// 830B5254: 38AB759C  addi r5, r11, 0x759c
	ctx.r[5].s64 = ctx.r[11].s64 + 30108;
	// 830B5258: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B525C: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 830B5260: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B5264: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B5268: 4E800421  bctrl
	ctx.lr = 0x830B526C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B526C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B5270: 41820010  beq 0x830b5280
	if ctx.cr[0].eq {
	pc = 0x830B5280; continue 'dispatch;
	}
	// 830B5274: 809E0034  lwz r4, 0x34(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B5278: 4BFFDA29  bl 0x830b2ca0
	ctx.lr = 0x830B527C;
	sub_830B2CA0(ctx, base);
	// 830B527C: 48000008  b 0x830b5284
	pc = 0x830B5284; continue 'dispatch;
	// 830B5280: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830B5284: 907F0034  stw r3, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[3].u32 ) };
	// 830B5288: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B528C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B5290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B5294: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B5298: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B529C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B52A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B52A0 size=16
    let mut pc: u32 = 0x830B52A0;
    'dispatch: loop {
        match pc {
            0x830B52A0 => {
    //   block [0x830B52A0..0x830B52B0)
	// 830B52A0: 81640024  lwz r11, 0x24(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 830B52A4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B52A8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B52AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B52B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B52B0 size=12
    let mut pc: u32 = 0x830B52B0;
    'dispatch: loop {
        match pc {
            0x830B52B0 => {
    //   block [0x830B52B0..0x830B52BC)
	// 830B52B0: 81640024  lwz r11, 0x24(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 830B52B4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B52B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B52C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B52C0 size=164
    let mut pc: u32 = 0x830B52C0;
    'dispatch: loop {
        match pc {
            0x830B52C0 => {
    //   block [0x830B52C0..0x830B5364)
	// 830B52C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B52C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B52C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B52CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B52D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B52D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B52D8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B52DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B52E0: 419A006C  beq cr6, 0x830b534c
	if ctx.cr[6].eq {
	pc = 0x830B534C; continue 'dispatch;
	}
	// 830B52E4: 3BDF002C  addi r30, r31, 0x2c
	ctx.r[30].s64 = ctx.r[31].s64 + 44;
	// 830B52E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B52EC: 4BFFCC95  bl 0x830b1f80
	ctx.lr = 0x830B52F0;
	sub_830B1F80(ctx, base);
	// 830B52F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B52F4: 41820058  beq 0x830b534c
	if ctx.cr[0].eq {
	pc = 0x830B534C; continue 'dispatch;
	}
	// 830B52F8: 480092E1  bl 0x830be5d8
	ctx.lr = 0x830B52FC;
	sub_830BE5D8(ctx, base);
	// 830B52FC: 48009205  bl 0x830be500
	ctx.lr = 0x830B5300;
	sub_830BE500(ctx, base);
	// 830B5300: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830B5304: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5308: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830B530C: 48000030  b 0x830b533c
	pc = 0x830B533C; continue 'dispatch;
	// 830B5310: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 830B5314: 4BFFCC15  bl 0x830b1f28
	ctx.lr = 0x830B5318;
	sub_830B1F28(ctx, base);
	// 830B5318: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B531C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B5320: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B5324: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B5328: 4E800421  bctrl
	ctx.lr = 0x830B532C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B532C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B5330: 4B2EC359  bl 0x823a1688
	ctx.lr = 0x830B5334;
	sub_823A1688(ctx, base);
	// 830B5334: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830B5338: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B533C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830B5340: 409AFFD0  bne cr6, 0x830b5310
	if !ctx.cr[6].eq {
	pc = 0x830B5310; continue 'dispatch;
	}
	// 830B5344: 48009295  bl 0x830be5d8
	ctx.lr = 0x830B5348;
	sub_830BE5D8(ctx, base);
	// 830B5348: 480092F1  bl 0x830be638
	ctx.lr = 0x830B534C;
	sub_830BE638(ctx, base);
	// 830B534C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B5350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B5354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B5358: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B535C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B5360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B5368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B5368 size=96
    let mut pc: u32 = 0x830B5368;
    'dispatch: loop {
        match pc {
            0x830B5368 => {
    //   block [0x830B5368..0x830B53C8)
	// 830B5368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B536C: 480F2E01  bl 0x831a816c
	ctx.lr = 0x830B5370;
	sub_831A8130(ctx, base);
	// 830B5370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B5374: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 830B5378: 3BE40014  addi r31, r4, 0x14
	ctx.r[31].s64 = ctx.r[4].s64 + 20;
	// 830B537C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B5380: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B5384: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B5388: 4BFFFA81  bl 0x830b4e08
	ctx.lr = 0x830B538C;
	sub_830B4E08(ctx, base);
	// 830B538C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830B5390: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B5394: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B5398: 4BFFFAE1  bl 0x830b4e78
	ctx.lr = 0x830B539C;
	sub_830B4E78(ctx, base);
	// 830B539C: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 830B53A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B53A4: 419A0010  beq cr6, 0x830b53b4
	if ctx.cr[6].eq {
	pc = 0x830B53B4; continue 'dispatch;
	}
	// 830B53A8: 389D0010  addi r4, r29, 0x10
	ctx.r[4].s64 = ctx.r[29].s64 + 16;
	// 830B53AC: 4B27A9BD  bl 0x8232fd68
	ctx.lr = 0x830B53B0;
	sub_8232FD68(ctx, base);
	// 830B53B0: 4800000C  b 0x830b53bc
	pc = 0x830B53BC; continue 'dispatch;
	// 830B53B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B53B8: 4B89B859  bl 0x82950c10
	ctx.lr = 0x830B53BC;
	sub_82950C10(ctx, base);
	// 830B53BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B53C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B53C4: 480F2DF8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B53C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B53C8 size=164
    let mut pc: u32 = 0x830B53C8;
    'dispatch: loop {
        match pc {
            0x830B53C8 => {
    //   block [0x830B53C8..0x830B546C)
	// 830B53C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B53CC: 480F2D9D  bl 0x831a8168
	ctx.lr = 0x830B53D0;
	sub_831A8130(ctx, base);
	// 830B53D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B53D4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830B53D8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830B53DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B53E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830B53E4: 4BFFCDD5  bl 0x830b21b8
	ctx.lr = 0x830B53E8;
	sub_830B21B8(ctx, base);
	// 830B53E8: D0210050  stfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 830B53EC: 3BFF0020  addi r31, r31, 0x20
	ctx.r[31].s64 = ctx.r[31].s64 + 32;
	// 830B53F0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B53F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B53F8: 4BFFFAF1  bl 0x830b4ee8
	ctx.lr = 0x830B53FC;
	sub_830B4EE8(ctx, base);
	// 830B53FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B5400: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B5404: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B5408: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 830B540C: 4BFFFB35  bl 0x830b4f40
	ctx.lr = 0x830B5410;
	sub_830B4F40(ctx, base);
	// 830B5410: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 830B5414: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 830B5418: 419A002C  beq cr6, 0x830b5444
	if ctx.cr[6].eq {
	pc = 0x830B5444; continue 'dispatch;
	}
	// 830B541C: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 830B5420: 4BFFCB09  bl 0x830b1f28
	ctx.lr = 0x830B5424;
	sub_830B1F28(ctx, base);
	// 830B5424: 7F03E840  cmplw cr6, r3, r29
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[29].u32, &mut ctx.xer);
	// 830B5428: 419A0034  beq cr6, 0x830b545c
	if ctx.cr[6].eq {
	pc = 0x830B545C; continue 'dispatch;
	}
	// 830B542C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830B5430: 4B2EC259  bl 0x823a1688
	ctx.lr = 0x830B5434;
	sub_823A1688(ctx, base);
	// 830B5434: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 830B5438: 83C10058  lwz r30, 0x58(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 830B543C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B5440: 409AFFDC  bne cr6, 0x830b541c
	if !ctx.cr[6].eq {
	pc = 0x830B541C; continue 'dispatch;
	}
	// 830B5444: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B5448: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830B544C: 4B89B7C5  bl 0x82950c10
	ctx.lr = 0x830B5450;
	sub_82950C10(ctx, base);
	// 830B5450: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830B5454: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830B5458: 480F2D60  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 830B545C: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 830B5460: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830B5464: 4B27A905  bl 0x8232fd68
	ctx.lr = 0x830B5468;
	sub_8232FD68(ctx, base);
	// 830B5468: 4BFFFFE8  b 0x830b5450
	pc = 0x830B5450; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B5470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B5470 size=220
    let mut pc: u32 = 0x830B5470;
    'dispatch: loop {
        match pc {
            0x830B5470 => {
    //   block [0x830B5470..0x830B554C)
	// 830B5470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B5474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B5478: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B547C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B5480: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B5484: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B5488: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B548C: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B5490: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B5494: 419A0020  beq cr6, 0x830b54b4
	if ctx.cr[6].eq {
	pc = 0x830B54B4; continue 'dispatch;
	}
	// 830B5498: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B549C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830B54A0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B54A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B54A8: 4E800421  bctrl
	ctx.lr = 0x830B54AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B54AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B54B0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 830B54B4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830B54B8: 419A0044  beq cr6, 0x830b54fc
	if ctx.cr[6].eq {
	pc = 0x830B54FC; continue 'dispatch;
	}
	// 830B54BC: 4BFFF78D  bl 0x830b4c48
	ctx.lr = 0x830B54C0;
	sub_830B4C48(ctx, base);
	// 830B54C0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B54C4: 38C0052C  li r6, 0x52c
	ctx.r[6].s64 = 1324;
	// 830B54C8: 38AB759C  addi r5, r11, 0x759c
	ctx.r[5].s64 = ctx.r[11].s64 + 30108;
	// 830B54CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B54D0: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 830B54D4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B54D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B54DC: 4E800421  bctrl
	ctx.lr = 0x830B54E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B54E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B54E4: 41820010  beq 0x830b54f4
	if ctx.cr[0].eq {
	pc = 0x830B54F4; continue 'dispatch;
	}
	// 830B54E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B54EC: 4BFFD7B5  bl 0x830b2ca0
	ctx.lr = 0x830B54F0;
	sub_830B2CA0(ctx, base);
	// 830B54F0: 48000008  b 0x830b54f8
	pc = 0x830B54F8; continue 'dispatch;
	// 830B54F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830B54F8: 907F0034  stw r3, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[3].u32 ) };
	// 830B54FC: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830B5500: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5504: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830B5508: 48000024  b 0x830b552c
	pc = 0x830B552C; continue 'dispatch;
	// 830B550C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 830B5510: 4BFFCA19  bl 0x830b1f28
	ctx.lr = 0x830B5514;
	sub_830B1F28(ctx, base);
	// 830B5514: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B5518: 4BFFE0C9  bl 0x830b35e0
	ctx.lr = 0x830B551C;
	sub_830B35E0(ctx, base);
	// 830B551C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B5520: 4B2EC169  bl 0x823a1688
	ctx.lr = 0x830B5524;
	sub_823A1688(ctx, base);
	// 830B5524: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830B5528: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B552C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830B5530: 409AFFDC  bne cr6, 0x830b550c
	if !ctx.cr[6].eq {
	pc = 0x830B550C; continue 'dispatch;
	}
	// 830B5534: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B5538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B553C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B5540: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B5544: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B5548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B5550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B5550 size=212
    let mut pc: u32 = 0x830B5550;
    'dispatch: loop {
        match pc {
            0x830B5550 => {
    //   block [0x830B5550..0x830B5624)
	// 830B5550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B5554: 480F2C11  bl 0x831a8164
	ctx.lr = 0x830B5558;
	sub_831A8130(ctx, base);
	// 830B5558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B555C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830B5560: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B5564: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B5568: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830B556C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 830B5570: 48003C81  bl 0x830b91f0
	ctx.lr = 0x830B5574;
	sub_830B91F0(ctx, base);
	// 830B5574: 3BFE0004  addi r31, r30, 4
	ctx.r[31].s64 = ctx.r[30].s64 + 4;
	// 830B5578: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830B557C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B5580: 4BFFCA19  bl 0x830b1f98
	ctx.lr = 0x830B5584;
	sub_830B1F98(ctx, base);
	// 830B5584: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B5588: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B558C: 419A000C  beq cr6, 0x830b5598
	if ctx.cr[6].eq {
	pc = 0x830B5598; continue 'dispatch;
	}
	// 830B5590: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830B5594: 4BFFC8ED  bl 0x830b1e80
	ctx.lr = 0x830B5598;
	sub_830B1E80(ctx, base);
	// 830B5598: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 830B559C: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B55A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B55A4: 419A0078  beq cr6, 0x830b561c
	if ctx.cr[6].eq {
	pc = 0x830B561C; continue 'dispatch;
	}
	// 830B55A8: 4BFFF6A1  bl 0x830b4c48
	ctx.lr = 0x830B55AC;
	sub_830B4C48(ctx, base);
	// 830B55AC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B55B0: 38C000C7  li r6, 0xc7
	ctx.r[6].s64 = 199;
	// 830B55B4: 38AB75C4  addi r5, r11, 0x75c4
	ctx.r[5].s64 = ctx.r[11].s64 + 30148;
	// 830B55B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B55BC: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 830B55C0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B55C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B55C8: 4E800421  bctrl
	ctx.lr = 0x830B55CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B55CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B55D0: 4182001C  beq 0x830b55ec
	if ctx.cr[0].eq {
	pc = 0x830B55EC; continue 'dispatch;
	}
	// 830B55D4: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B55D8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830B55DC: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B55E0: 4BFFFB99  bl 0x830b5178
	ctx.lr = 0x830B55E4;
	sub_830B5178(ctx, base);
	// 830B55E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B55E8: 48000008  b 0x830b55f0
	pc = 0x830B55F0; continue 'dispatch;
	// 830B55EC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830B55F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B55F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B55F8: 4BFFFE79  bl 0x830b5470
	ctx.lr = 0x830B55FC;
	sub_830B5470(ctx, base);
	// 830B55FC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830B5600: 419A001C  beq cr6, 0x830b561c
	if ctx.cr[6].eq {
	pc = 0x830B561C; continue 'dispatch;
	}
	// 830B5604: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5608: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830B560C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B5610: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5614: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B5618: 4E800421  bctrl
	ctx.lr = 0x830B561C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B561C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B5620: 480F2B94  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B5628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B5628 size=364
    let mut pc: u32 = 0x830B5628;
    'dispatch: loop {
        match pc {
            0x830B5628 => {
    //   block [0x830B5628..0x830B5794)
	// 830B5628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B562C: 480F2B41  bl 0x831a816c
	ctx.lr = 0x830B5630;
	sub_831A8130(ctx, base);
	// 830B5630: 9421FE60  stwu r1, -0x1a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-416 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B5634: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B5638: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 830B563C: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 830B5640: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 830B5644: 38E10068  addi r7, r1, 0x68
	ctx.r[7].s64 = ctx.r[1].s64 + 104;
	// 830B5648: 38C1006C  addi r6, r1, 0x6c
	ctx.r[6].s64 = ctx.r[1].s64 + 108;
	// 830B564C: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 830B5650: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 830B5654: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830B5658: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 830B565C: 480037E5  bl 0x830b8e40
	ctx.lr = 0x830B5660;
	sub_830B8E40(ctx, base);
	// 830B5660: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 830B5664: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 830B5668: 48003B91  bl 0x830b91f8
	ctx.lr = 0x830B566C;
	sub_830B91F8(ctx, base);
	// 830B566C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830B5670: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B5674: 80A10058  lwz r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 830B5678: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B567C: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830B5680: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 830B5684: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B5688: 4BFFFEC9  bl 0x830b5550
	ctx.lr = 0x830B568C;
	sub_830B5550(ctx, base);
	// 830B568C: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 830B5690: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B5694: 419A00F4  beq cr6, 0x830b5788
	if ctx.cr[6].eq {
	pc = 0x830B5788; continue 'dispatch;
	}
	// 830B5698: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 830B569C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 830B56A0: 480035F1  bl 0x830b8c90
	ctx.lr = 0x830B56A4;
	sub_830B8C90(ctx, base);
	// 830B56A4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830B56A8: 419A000C  beq cr6, 0x830b56b4
	if ctx.cr[6].eq {
	pc = 0x830B56B4; continue 'dispatch;
	}
	// 830B56AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B56B0: 48003719  bl 0x830b8dc8
	ctx.lr = 0x830B56B4;
	sub_830B8DC8(ctx, base);
	// 830B56B4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830B56B8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830B56BC: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B56C0: 48002ED1  bl 0x830b8590
	ctx.lr = 0x830B56C4;
	sub_830B8590(ctx, base);
	// 830B56C4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830B56C8: 4BFFC8B9  bl 0x830b1f80
	ctx.lr = 0x830B56CC;
	sub_830B1F80(ctx, base);
	// 830B56CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B56D0: 4182003C  beq 0x830b570c
	if ctx.cr[0].eq {
	pc = 0x830B570C; continue 'dispatch;
	}
	// 830B56D4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830B56D8: 4BFFC851  bl 0x830b1f28
	ctx.lr = 0x830B56DC;
	sub_830B1F28(ctx, base);
	// 830B56DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B56E0: 80A1006C  lwz r5, 0x6c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 830B56E4: 8081005C  lwz r4, 0x5c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 830B56E8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B56EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B56F0: 4E800421  bctrl
	ctx.lr = 0x830B56F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B56F4: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830B56F8: 41820078  beq 0x830b5770
	if ctx.cr[0].eq {
	pc = 0x830B5770; continue 'dispatch;
	}
	// 830B56FC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 830B5700: 387E002C  addi r3, r30, 0x2c
	ctx.r[3].s64 = ctx.r[30].s64 + 44;
	// 830B5704: 4BFFC925  bl 0x830b2028
	ctx.lr = 0x830B5708;
	sub_830B2028(ctx, base);
	// 830B5708: 48000068  b 0x830b5770
	pc = 0x830B5770; continue 'dispatch;
	// 830B570C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830B5710: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B5714: 480033ED  bl 0x830b8b00
	ctx.lr = 0x830B5718;
	sub_830B8B00(ctx, base);
	// 830B5718: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B571C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B5720: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B5724: 4E800421  bctrl
	ctx.lr = 0x830B5728;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B5728: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830B572C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830B5730: 386A75EC  addi r3, r10, 0x75ec
	ctx.r[3].s64 = ctx.r[10].s64 + 30188;
	// 830B5734: 41820018  beq 0x830b574c
	if ctx.cr[0].eq {
	pc = 0x830B574C; continue 'dispatch;
	}
	// 830B5738: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 830B573C: 893F0002  lbz r9, 2(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 830B5740: 891F0001  lbz r8, 1(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 830B5744: 895F0000  lbz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5748: 48000014  b 0x830b575c
	pc = 0x830B575C; continue 'dispatch;
	// 830B574C: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5750: 893F0001  lbz r9, 1(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 830B5754: 891F0002  lbz r8, 2(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 830B5758: 895F0003  lbz r10, 3(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 830B575C: 7D670774  extsb r7, r11
	ctx.r[7].s64 = ctx.r[11].s8 as i64;
	// 830B5760: 7D260774  extsb r6, r9
	ctx.r[6].s64 = ctx.r[9].s8 as i64;
	// 830B5764: 7D050774  extsb r5, r8
	ctx.r[5].s64 = ctx.r[8].s8 as i64;
	// 830B5768: 7D440774  extsb r4, r10
	ctx.r[4].s64 = ctx.r[10].s8 as i64;
	// 830B576C: 4800341D  bl 0x830b8b88
	ctx.lr = 0x830B5770;
	sub_830B8B88(ctx, base);
	// 830B5770: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 830B5774: 4BFFF50D  bl 0x830b4c80
	ctx.lr = 0x830B5778;
	sub_830B4C80(ctx, base);
	// 830B5778: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 830B577C: 4800364D  bl 0x830b8dc8
	ctx.lr = 0x830B5780;
	sub_830B8DC8(ctx, base);
	// 830B5780: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830B5784: 4BFFC755  bl 0x830b1ed8
	ctx.lr = 0x830B5788;
	sub_830B1ED8(ctx, base);
	// 830B5788: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B578C: 382101A0  addi r1, r1, 0x1a0
	ctx.r[1].s64 = ctx.r[1].s64 + 416;
	// 830B5790: 480F2A2C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B5798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B5798 size=568
    let mut pc: u32 = 0x830B5798;
    'dispatch: loop {
        match pc {
            0x830B5798 => {
    //   block [0x830B5798..0x830B59D0)
	// 830B5798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B579C: 480F29C1  bl 0x831a815c
	ctx.lr = 0x830B57A0;
	sub_831A8130(ctx, base);
	// 830B57A0: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B57A4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B57A8: 3D601555  lis r11, 0x1555
	ctx.r[11].s64 = 357892096;
	// 830B57AC: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 830B57B0: 616B5554  ori r11, r11, 0x5554
	ctx.r[11].u64 = ctx.r[11].u64 | 21844;
	// 830B57B4: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 830B57B8: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B57BC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 830B57C0: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 830B57C4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B57C8: 41980048  blt cr6, 0x830b5810
	if ctx.cr[6].lt {
	pc = 0x830B5810; continue 'dispatch;
	}
	// 830B57CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 830B57D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B57D4: 388B9BCC  addi r4, r11, -0x6434
	ctx.r[4].s64 = ctx.r[11].s64 + -25652;
	// 830B57D8: 4B2100F1  bl 0x822c58c8
	ctx.lr = 0x830B57DC;
	sub_822C58C8(ctx, base);
	// 830B57DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B57E0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830B57E4: 4B210035  bl 0x822c5818
	ctx.lr = 0x830B57E8;
	sub_822C5818(ctx, base);
	// 830B57E8: 4B20EAC9  bl 0x822c42b0
	ctx.lr = 0x830B57EC;
	sub_822C42B0(ctx, base);
	// 830B57EC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 830B57F0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830B57F4: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 830B57F8: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 830B57FC: 4B20FC75  bl 0x822c5470
	ctx.lr = 0x830B5800;
	sub_822C5470(ctx, base);
	// 830B5800: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830B5804: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830B5808: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B580C: 4B20F4D5  bl 0x822c4ce0
	ctx.lr = 0x830B5810;
	sub_822C4CE0(ctx, base);
	// 830B5810: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B5814: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5818: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830B581C: 4B21E4BD  bl 0x822d3cd8
	ctx.lr = 0x830B5820;
	sub_822D3CD8(ctx, base);
	// 830B5820: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830B5824: 41820020  beq 0x830b5844
	if ctx.cr[0].eq {
	pc = 0x830B5844; continue 'dispatch;
	}
	// 830B5828: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830B582C: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 830B5830: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 830B5834: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830B5838: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830B583C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830B5840: 4BFFF759  bl 0x830b4f98
	ctx.lr = 0x830B5844;
	sub_830B4F98(ctx, base);
	// 830B5844: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B5848: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B584C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830B5850: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B5854: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 830B5858: 409A0018  bne cr6, 0x830b5870
	if !ctx.cr[6].eq {
	pc = 0x830B5870; continue 'dispatch;
	}
	// 830B585C: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 830B5860: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5864: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830B5868: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B586C: 4800003C  b 0x830b58a8
	pc = 0x830B58A8; continue 'dispatch;
	// 830B5870: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B5874: 41820020  beq 0x830b5894
	if ctx.cr[0].eq {
	pc = 0x830B5894; continue 'dispatch;
	}
	// 830B5878: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830B587C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5880: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5884: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830B5888: 409A0024  bne cr6, 0x830b58ac
	if !ctx.cr[6].eq {
	pc = 0x830B58AC; continue 'dispatch;
	}
	// 830B588C: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830B5890: 4800001C  b 0x830b58ac
	pc = 0x830B58AC; continue 'dispatch;
	// 830B5894: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 830B5898: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B589C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B58A0: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830B58A4: 409A0008  bne cr6, 0x830b58ac
	if !ctx.cr[6].eq {
	pc = 0x830B58AC; continue 'dispatch;
	}
	// 830B58A8: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 830B58AC: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B58B0: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 830B58B4: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 830B58B8: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 830B58BC: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 830B58C0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B58C4: 409A00F0  bne cr6, 0x830b59b4
	if !ctx.cr[6].eq {
	pc = 0x830B59B4; continue 'dispatch;
	}
	// 830B58C8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 830B58CC: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B58D0: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B58D4: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B58D8: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830B58DC: 409A0054  bne cr6, 0x830b5930
	if !ctx.cr[6].eq {
	pc = 0x830B5930; continue 'dispatch;
	}
	// 830B58E0: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B58E4: 892A0018  lbz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 830B58E8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 830B58EC: 419A0054  beq cr6, 0x830b5940
	if ctx.cr[6].eq {
	pc = 0x830B5940; continue 'dispatch;
	}
	// 830B58F0: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B58F4: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B58F8: 409A0010  bne cr6, 0x830b5908
	if !ctx.cr[6].eq {
	pc = 0x830B5908; continue 'dispatch;
	}
	// 830B58FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B5900: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830B5904: 4B38E665  bl 0x82443f68
	ctx.lr = 0x830B5908;
	sub_82443F68(ctx, base);
	// 830B5908: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B590C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B5910: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 830B5914: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5918: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B591C: 9B6B0018  stb r27, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 830B5920: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5924: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5928: 4B38E5D9  bl 0x82443f00
	ctx.lr = 0x830B592C;
	sub_82443F00(ctx, base);
	// 830B592C: 48000074  b 0x830b59a0
	pc = 0x830B59A0; continue 'dispatch;
	// 830B5930: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5934: 892A0018  lbz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 830B5938: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 830B593C: 409A0028  bne cr6, 0x830b5964
	if !ctx.cr[6].eq {
	pc = 0x830B5964; continue 'dispatch;
	}
	// 830B5940: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5944: 9BA90018  stb r29, 0x18(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 830B5948: 9BAA0018  stb r29, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 830B594C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5950: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5954: 9B6A0018  stb r27, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 830B5958: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B595C: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5960: 48000040  b 0x830b59a0
	pc = 0x830B59A0; continue 'dispatch;
	// 830B5964: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5968: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B596C: 409A0010  bne cr6, 0x830b597c
	if !ctx.cr[6].eq {
	pc = 0x830B597C; continue 'dispatch;
	}
	// 830B5970: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B5974: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830B5978: 4B38E589  bl 0x82443f00
	ctx.lr = 0x830B597C;
	sub_82443F00(ctx, base);
	// 830B597C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5980: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B5984: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 830B5988: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B598C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5990: 9B6B0018  stb r27, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 830B5994: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5998: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B599C: 4B38E5CD  bl 0x82443f68
	ctx.lr = 0x830B59A0;
	sub_82443F68(ctx, base);
	// 830B59A0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B59A4: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 830B59A8: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 830B59AC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B59B0: 419AFF1C  beq cr6, 0x830b58cc
	if ctx.cr[6].eq {
	pc = 0x830B58CC; continue 'dispatch;
	}
	// 830B59B4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B59B8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 830B59BC: 93990000  stw r28, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830B59C0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B59C4: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 830B59C8: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 830B59CC: 480F27E0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B59D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B59D0 size=568
    let mut pc: u32 = 0x830B59D0;
    'dispatch: loop {
        match pc {
            0x830B59D0 => {
    //   block [0x830B59D0..0x830B5C08)
	// 830B59D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B59D4: 480F2789  bl 0x831a815c
	ctx.lr = 0x830B59D8;
	sub_831A8130(ctx, base);
	// 830B59D8: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B59DC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B59E0: 3D601555  lis r11, 0x1555
	ctx.r[11].s64 = 357892096;
	// 830B59E4: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 830B59E8: 616B5554  ori r11, r11, 0x5554
	ctx.r[11].u64 = ctx.r[11].u64 | 21844;
	// 830B59EC: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 830B59F0: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B59F4: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 830B59F8: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 830B59FC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B5A00: 41980048  blt cr6, 0x830b5a48
	if ctx.cr[6].lt {
	pc = 0x830B5A48; continue 'dispatch;
	}
	// 830B5A04: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 830B5A08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B5A0C: 388B9BCC  addi r4, r11, -0x6434
	ctx.r[4].s64 = ctx.r[11].s64 + -25652;
	// 830B5A10: 4B20FEB9  bl 0x822c58c8
	ctx.lr = 0x830B5A14;
	sub_822C58C8(ctx, base);
	// 830B5A14: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B5A18: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830B5A1C: 4B20FDFD  bl 0x822c5818
	ctx.lr = 0x830B5A20;
	sub_822C5818(ctx, base);
	// 830B5A20: 4B20E891  bl 0x822c42b0
	ctx.lr = 0x830B5A24;
	sub_822C42B0(ctx, base);
	// 830B5A24: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 830B5A28: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830B5A2C: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 830B5A30: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 830B5A34: 4B20FA3D  bl 0x822c5470
	ctx.lr = 0x830B5A38;
	sub_822C5470(ctx, base);
	// 830B5A38: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830B5A3C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830B5A40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B5A44: 4B20F29D  bl 0x822c4ce0
	ctx.lr = 0x830B5A48;
	sub_822C4CE0(ctx, base);
	// 830B5A48: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B5A4C: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5A50: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830B5A54: 4B21E285  bl 0x822d3cd8
	ctx.lr = 0x830B5A58;
	sub_822D3CD8(ctx, base);
	// 830B5A58: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830B5A5C: 41820020  beq 0x830b5a7c
	if ctx.cr[0].eq {
	pc = 0x830B5A7C; continue 'dispatch;
	}
	// 830B5A60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830B5A64: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 830B5A68: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 830B5A6C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830B5A70: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830B5A74: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830B5A78: 4BFFF589  bl 0x830b5000
	ctx.lr = 0x830B5A7C;
	sub_830B5000(ctx, base);
	// 830B5A7C: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B5A80: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5A84: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830B5A88: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B5A8C: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 830B5A90: 409A0018  bne cr6, 0x830b5aa8
	if !ctx.cr[6].eq {
	pc = 0x830B5AA8; continue 'dispatch;
	}
	// 830B5A94: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 830B5A98: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5A9C: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830B5AA0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5AA4: 4800003C  b 0x830b5ae0
	pc = 0x830B5AE0; continue 'dispatch;
	// 830B5AA8: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B5AAC: 41820020  beq 0x830b5acc
	if ctx.cr[0].eq {
	pc = 0x830B5ACC; continue 'dispatch;
	}
	// 830B5AB0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830B5AB4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5AB8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5ABC: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830B5AC0: 409A0024  bne cr6, 0x830b5ae4
	if !ctx.cr[6].eq {
	pc = 0x830B5AE4; continue 'dispatch;
	}
	// 830B5AC4: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830B5AC8: 4800001C  b 0x830b5ae4
	pc = 0x830B5AE4; continue 'dispatch;
	// 830B5ACC: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 830B5AD0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5AD4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B5AD8: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830B5ADC: 409A0008  bne cr6, 0x830b5ae4
	if !ctx.cr[6].eq {
	pc = 0x830B5AE4; continue 'dispatch;
	}
	// 830B5AE0: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 830B5AE4: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5AE8: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 830B5AEC: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 830B5AF0: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 830B5AF4: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 830B5AF8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B5AFC: 409A00F0  bne cr6, 0x830b5bec
	if !ctx.cr[6].eq {
	pc = 0x830B5BEC; continue 'dispatch;
	}
	// 830B5B00: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 830B5B04: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5B08: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5B0C: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5B10: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830B5B14: 409A0054  bne cr6, 0x830b5b68
	if !ctx.cr[6].eq {
	pc = 0x830B5B68; continue 'dispatch;
	}
	// 830B5B18: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B5B1C: 892A0018  lbz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 830B5B20: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 830B5B24: 419A0054  beq cr6, 0x830b5b78
	if ctx.cr[6].eq {
	pc = 0x830B5B78; continue 'dispatch;
	}
	// 830B5B28: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B5B2C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B5B30: 409A0010  bne cr6, 0x830b5b40
	if !ctx.cr[6].eq {
	pc = 0x830B5B40; continue 'dispatch;
	}
	// 830B5B34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B5B38: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830B5B3C: 4B38E42D  bl 0x82443f68
	ctx.lr = 0x830B5B40;
	sub_82443F68(ctx, base);
	// 830B5B40: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5B44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B5B48: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 830B5B4C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5B50: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5B54: 9B6B0018  stb r27, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 830B5B58: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5B5C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5B60: 4B38E3A1  bl 0x82443f00
	ctx.lr = 0x830B5B64;
	sub_82443F00(ctx, base);
	// 830B5B64: 48000074  b 0x830b5bd8
	pc = 0x830B5BD8; continue 'dispatch;
	// 830B5B68: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5B6C: 892A0018  lbz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 830B5B70: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 830B5B74: 409A0028  bne cr6, 0x830b5b9c
	if !ctx.cr[6].eq {
	pc = 0x830B5B9C; continue 'dispatch;
	}
	// 830B5B78: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5B7C: 9BA90018  stb r29, 0x18(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 830B5B80: 9BAA0018  stb r29, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 830B5B84: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5B88: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5B8C: 9B6A0018  stb r27, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 830B5B90: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5B94: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5B98: 48000040  b 0x830b5bd8
	pc = 0x830B5BD8; continue 'dispatch;
	// 830B5B9C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5BA0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B5BA4: 409A0010  bne cr6, 0x830b5bb4
	if !ctx.cr[6].eq {
	pc = 0x830B5BB4; continue 'dispatch;
	}
	// 830B5BA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B5BAC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830B5BB0: 4B38E351  bl 0x82443f00
	ctx.lr = 0x830B5BB4;
	sub_82443F00(ctx, base);
	// 830B5BB4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5BB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B5BBC: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 830B5BC0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5BC4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5BC8: 9B6B0018  stb r27, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 830B5BCC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5BD0: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5BD4: 4B38E395  bl 0x82443f68
	ctx.lr = 0x830B5BD8;
	sub_82443F68(ctx, base);
	// 830B5BD8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5BDC: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 830B5BE0: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 830B5BE4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B5BE8: 419AFF1C  beq cr6, 0x830b5b04
	if ctx.cr[6].eq {
	pc = 0x830B5B04; continue 'dispatch;
	}
	// 830B5BEC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5BF0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 830B5BF4: 93990000  stw r28, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830B5BF8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5BFC: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 830B5C00: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 830B5C04: 480F25A8  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B5C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B5C08 size=156
    let mut pc: u32 = 0x830B5C08;
    'dispatch: loop {
        match pc {
            0x830B5C08 => {
    //   block [0x830B5C08..0x830B5CA4)
	// 830B5C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B5C0C: 480F2561  bl 0x831a816c
	ctx.lr = 0x830B5C10;
	sub_831A8130(ctx, base);
	// 830B5C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B5C14: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830B5C18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B5C1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B5C20: 4BFFC309  bl 0x830b1f28
	ctx.lr = 0x830B5C24;
	sub_830B1F28(ctx, base);
	// 830B5C24: 4BFFC4DD  bl 0x830b2100
	ctx.lr = 0x830B5C28;
	sub_830B2100(ctx, base);
	// 830B5C28: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830B5C2C: 3BFF0014  addi r31, r31, 0x14
	ctx.r[31].s64 = ctx.r[31].s64 + 20;
	// 830B5C30: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B5C34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B5C38: 4BFFF1D1  bl 0x830b4e08
	ctx.lr = 0x830B5C3C;
	sub_830B4E08(ctx, base);
	// 830B5C3C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B5C40: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B5C44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B5C48: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 830B5C4C: 4BFFF22D  bl 0x830b4e78
	ctx.lr = 0x830B5C50;
	sub_830B4E78(ctx, base);
	// 830B5C50: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 830B5C54: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 830B5C58: 419A0044  beq cr6, 0x830b5c9c
	if ctx.cr[6].eq {
	pc = 0x830B5C9C; continue 'dispatch;
	}
	// 830B5C5C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830B5C60: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 830B5C64: 4BFFC2E5  bl 0x830b1f48
	ctx.lr = 0x830B5C68;
	sub_830B1F48(ctx, base);
	// 830B5C68: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830B5C6C: 40820020  bne 0x830b5c8c
	if !ctx.cr[0].eq {
	pc = 0x830B5C8C; continue 'dispatch;
	}
	// 830B5C70: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830B5C74: 4B2EBA15  bl 0x823a1688
	ctx.lr = 0x830B5C78;
	sub_823A1688(ctx, base);
	// 830B5C78: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 830B5C7C: 83C10058  lwz r30, 0x58(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 830B5C80: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B5C84: 409AFFD8  bne cr6, 0x830b5c5c
	if !ctx.cr[6].eq {
	pc = 0x830B5C5C; continue 'dispatch;
	}
	// 830B5C88: 48000014  b 0x830b5c9c
	pc = 0x830B5C9C; continue 'dispatch;
	// 830B5C8C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830B5C90: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B5C94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B5C98: 4BA95911  bl 0x82b4b5a8
	ctx.lr = 0x830B5C9C;
	sub_82B4B5A8(ctx, base);
	// 830B5C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B5CA0: 480F251C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B5CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B5CA8 size=156
    let mut pc: u32 = 0x830B5CA8;
    'dispatch: loop {
        match pc {
            0x830B5CA8 => {
    //   block [0x830B5CA8..0x830B5D44)
	// 830B5CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B5CAC: 480F24C1  bl 0x831a816c
	ctx.lr = 0x830B5CB0;
	sub_831A8130(ctx, base);
	// 830B5CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B5CB4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830B5CB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B5CBC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B5CC0: 4BFFC269  bl 0x830b1f28
	ctx.lr = 0x830B5CC4;
	sub_830B1F28(ctx, base);
	// 830B5CC4: 4BFFC4F5  bl 0x830b21b8
	ctx.lr = 0x830B5CC8;
	sub_830B21B8(ctx, base);
	// 830B5CC8: D0210050  stfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 830B5CCC: 3BFF0020  addi r31, r31, 0x20
	ctx.r[31].s64 = ctx.r[31].s64 + 32;
	// 830B5CD0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B5CD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B5CD8: 4BFFF211  bl 0x830b4ee8
	ctx.lr = 0x830B5CDC;
	sub_830B4EE8(ctx, base);
	// 830B5CDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B5CE0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B5CE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B5CE8: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 830B5CEC: 4BFFF255  bl 0x830b4f40
	ctx.lr = 0x830B5CF0;
	sub_830B4F40(ctx, base);
	// 830B5CF0: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 830B5CF4: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 830B5CF8: 419A0044  beq cr6, 0x830b5d3c
	if ctx.cr[6].eq {
	pc = 0x830B5D3C; continue 'dispatch;
	}
	// 830B5CFC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830B5D00: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 830B5D04: 4BFFC245  bl 0x830b1f48
	ctx.lr = 0x830B5D08;
	sub_830B1F48(ctx, base);
	// 830B5D08: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830B5D0C: 40820020  bne 0x830b5d2c
	if !ctx.cr[0].eq {
	pc = 0x830B5D2C; continue 'dispatch;
	}
	// 830B5D10: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830B5D14: 4B2EB975  bl 0x823a1688
	ctx.lr = 0x830B5D18;
	sub_823A1688(ctx, base);
	// 830B5D18: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 830B5D1C: 83C10058  lwz r30, 0x58(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 830B5D20: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B5D24: 409AFFD8  bne cr6, 0x830b5cfc
	if !ctx.cr[6].eq {
	pc = 0x830B5CFC; continue 'dispatch;
	}
	// 830B5D28: 48000014  b 0x830b5d3c
	pc = 0x830B5D3C; continue 'dispatch;
	// 830B5D2C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830B5D30: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B5D34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B5D38: 4BA95871  bl 0x82b4b5a8
	ctx.lr = 0x830B5D3C;
	sub_82B4B5A8(ctx, base);
	// 830B5D3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B5D40: 480F247C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B5D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B5D48 size=104
    let mut pc: u32 = 0x830B5D48;
    'dispatch: loop {
        match pc {
            0x830B5D48 => {
    //   block [0x830B5D48..0x830B5DB0)
	// 830B5D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B5D4C: 480F2421  bl 0x831a816c
	ctx.lr = 0x830B5D50;
	sub_831A8130(ctx, base);
	// 830B5D50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B5D54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B5D58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B5D5C: 4BFFF0AD  bl 0x830b4e08
	ctx.lr = 0x830B5D60;
	sub_830B4E08(ctx, base);
	// 830B5D60: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830B5D64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B5D68: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B5D6C: 4BFFF10D  bl 0x830b4e78
	ctx.lr = 0x830B5D70;
	sub_830B4E78(ctx, base);
	// 830B5D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B5D74: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B5D78: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B5D7C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 830B5D80: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B5D84: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B5D88: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 830B5D8C: 4B868D45  bl 0x8291ead0
	ctx.lr = 0x830B5D90;
	sub_8291EAD0(ctx, base);
	// 830B5D90: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 830B5D94: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830B5D98: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B5D9C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830B5DA0: 4BA95EF1  bl 0x82b4bc90
	ctx.lr = 0x830B5DA4;
	sub_82B4BC90(ctx, base);
	// 830B5DA4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830B5DA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B5DAC: 480F2410  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B5DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B5DB0 size=200
    let mut pc: u32 = 0x830B5DB0;
    'dispatch: loop {
        match pc {
            0x830B5DB0 => {
    //   block [0x830B5DB0..0x830B5E78)
	// 830B5DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B5DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B5DB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B5DBC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B5DC0: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5DC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B5DC8: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 830B5DCC: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 830B5DD0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830B5DD4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5DD8: 892B0019  lbz r9, 0x19(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 830B5DDC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 830B5DE0: 409A0060  bne cr6, 0x830b5e40
	if !ctx.cr[6].eq {
	pc = 0x830B5E40; continue 'dispatch;
	}
	// 830B5DE4: 80A70000  lwz r5, 0(r7)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5DE8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B5DEC: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 830B5DF0: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 830B5DF4: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5DF8: 88690000  lbz r3, 0(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5DFC: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830B5E00: 7D034050  subf r8, r3, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[3].s64;
	// 830B5E04: 41820014  beq 0x830b5e18
	if ctx.cr[0].eq {
	pc = 0x830B5E18; continue 'dispatch;
	}
	// 830B5E08: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830B5E0C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 830B5E10: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 830B5E14: 419AFFE0  beq cr6, 0x830b5df4
	if ctx.cr[6].eq {
	pc = 0x830B5DF4; continue 'dispatch;
	}
	// 830B5E18: 7D0A0034  cntlzw r10, r8
	ctx.r[10].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 830B5E1C: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 830B5E20: 554ADFFF  rlwinm. r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830B5E24: 4182000C  beq 0x830b5e30
	if ctx.cr[0].eq {
	pc = 0x830B5E30; continue 'dispatch;
	}
	// 830B5E28: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5E2C: 48000008  b 0x830b5e34
	pc = 0x830B5E34; continue 'dispatch;
	// 830B5E30: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B5E34: 892B0019  lbz r9, 0x19(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 830B5E38: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 830B5E3C: 419AFFAC  beq cr6, 0x830b5de8
	if ctx.cr[6].eq {
	pc = 0x830B5DE8; continue 'dispatch;
	}
	// 830B5E40: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 830B5E44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B5E48: 4BFFF951  bl 0x830b5798
	ctx.lr = 0x830B5E4C;
	sub_830B5798(ctx, base);
	// 830B5E4C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830B5E50: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830B5E54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B5E58: 995F0004  stb r10, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 830B5E5C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5E60: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B5E64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B5E68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B5E6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B5E70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B5E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B5E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B5E78 size=168
    let mut pc: u32 = 0x830B5E78;
    'dispatch: loop {
        match pc {
            0x830B5E78 => {
    //   block [0x830B5E78..0x830B5F20)
	// 830B5E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B5E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B5E80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B5E84: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B5E88: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5E8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B5E90: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 830B5E94: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 830B5E98: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830B5E9C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B5EA0: 892B0019  lbz r9, 0x19(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 830B5EA4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 830B5EA8: 409A0040  bne cr6, 0x830b5ee8
	if !ctx.cr[6].eq {
	pc = 0x830B5EE8; continue 'dispatch;
	}
	// 830B5EAC: C0070000  lfs f0, 0(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B5EB0: C1AB000C  lfs f13, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B5EB4: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 830B5EB8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 830B5EBC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830B5EC0: 41980008  blt cr6, 0x830b5ec8
	if ctx.cr[6].lt {
	pc = 0x830B5EC8; continue 'dispatch;
	}
	// 830B5EC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B5EC8: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830B5ECC: 4182000C  beq 0x830b5ed8
	if ctx.cr[0].eq {
	pc = 0x830B5ED8; continue 'dispatch;
	}
	// 830B5ED0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5ED4: 48000008  b 0x830b5edc
	pc = 0x830B5EDC; continue 'dispatch;
	// 830B5ED8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B5EDC: 892B0019  lbz r9, 0x19(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 830B5EE0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 830B5EE4: 419AFFCC  beq cr6, 0x830b5eb0
	if ctx.cr[6].eq {
	pc = 0x830B5EB0; continue 'dispatch;
	}
	// 830B5EE8: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 830B5EEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B5EF0: 4BFFFAE1  bl 0x830b59d0
	ctx.lr = 0x830B5EF4;
	sub_830B59D0(ctx, base);
	// 830B5EF4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830B5EF8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830B5EFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B5F00: 995F0004  stb r10, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 830B5F04: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5F08: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B5F0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B5F10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B5F14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B5F18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B5F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B5F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B5F20 size=112
    let mut pc: u32 = 0x830B5F20;
    'dispatch: loop {
        match pc {
            0x830B5F20 => {
    //   block [0x830B5F20..0x830B5F90)
	// 830B5F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B5F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B5F28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B5F2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B5F30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B5F34: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830B5F38: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B5F3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B5F40: 4BFFBFE9  bl 0x830b1f28
	ctx.lr = 0x830B5F44;
	sub_830B1F28(ctx, base);
	// 830B5F44: 816300D8  lwz r11, 0xd8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(216 as u32) ) } as u64;
	// 830B5F48: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B5F4C: 419A002C  beq cr6, 0x830b5f78
	if ctx.cr[6].eq {
	pc = 0x830B5F78; continue 'dispatch;
	}
	// 830B5F50: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B5F54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B5F58: 4BFFFCB1  bl 0x830b5c08
	ctx.lr = 0x830B5F5C;
	sub_830B5C08(ctx, base);
	// 830B5F5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B5F60: 4BFFBFC9  bl 0x830b1f28
	ctx.lr = 0x830B5F64;
	sub_830B1F28(ctx, base);
	// 830B5F64: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B5F68: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B5F6C: 916300D8  stw r11, 0xd8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(216 as u32), ctx.r[11].u32 ) };
	// 830B5F70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B5F74: 4BFFFD35  bl 0x830b5ca8
	ctx.lr = 0x830B5F78;
	sub_830B5CA8(ctx, base);
	// 830B5F78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B5F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B5F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B5F84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B5F88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B5F8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B5F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B5F90 size=96
    let mut pc: u32 = 0x830B5F90;
    'dispatch: loop {
        match pc {
            0x830B5F90 => {
    //   block [0x830B5F90..0x830B5FF0)
	// 830B5F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B5F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B5F98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B5F9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B5FA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B5FA4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B5FA8: 817E003C  lwz r11, 0x3c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830B5FAC: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5FB0: 48000018  b 0x830b5fc8
	pc = 0x830B5FC8; continue 'dispatch;
	// 830B5FB4: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 830B5FB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B5FBC: 4BFFFF65  bl 0x830b5f20
	ctx.lr = 0x830B5FC0;
	sub_830B5F20(ctx, base);
	// 830B5FC0: 817E003C  lwz r11, 0x3c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830B5FC4: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B5FC8: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B5FCC: 409AFFE8  bne cr6, 0x830b5fb4
	if !ctx.cr[6].eq {
	pc = 0x830B5FB4; continue 'dispatch;
	}
	// 830B5FD0: 387E0038  addi r3, r30, 0x38
	ctx.r[3].s64 = ctx.r[30].s64 + 56;
	// 830B5FD4: 4B5287E5  bl 0x825de7b8
	ctx.lr = 0x830B5FD8;
	sub_825DE7B8(ctx, base);
	// 830B5FD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B5FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B5FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B5FE4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B5FE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B5FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B5FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B5FF0 size=120
    let mut pc: u32 = 0x830B5FF0;
    'dispatch: loop {
        match pc {
            0x830B5FF0 => {
    //   block [0x830B5FF0..0x830B6068)
	// 830B5FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B5FF4: 480F2179  bl 0x831a816c
	ctx.lr = 0x830B5FF8;
	sub_831A8130(ctx, base);
	// 830B5FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B5FFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B6000: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 830B6004: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B6008: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B600C: 4BFFF3BD  bl 0x830b53c8
	ctx.lr = 0x830B6010;
	sub_830B53C8(ctx, base);
	// 830B6010: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B6014: 4BFFBF6D  bl 0x830b1f80
	ctx.lr = 0x830B6018;
	sub_830B1F80(ctx, base);
	// 830B6018: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B601C: 4182003C  beq 0x830b6058
	if ctx.cr[0].eq {
	pc = 0x830B6058; continue 'dispatch;
	}
	// 830B6020: 83DF003C  lwz r30, 0x3c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 830B6024: 3BFF0038  addi r31, r31, 0x38
	ctx.r[31].s64 = ctx.r[31].s64 + 56;
	// 830B6028: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 830B602C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B6030: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B6034: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B6038: 4BFFF089  bl 0x830b50c0
	ctx.lr = 0x830B603C;
	sub_830B50C0(ctx, base);
	// 830B603C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830B6040: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830B6044: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B6048: 4BA522D9  bl 0x82b08320
	ctx.lr = 0x830B604C;
	sub_82B08320(ctx, base);
	// 830B604C: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 830B6050: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B6054: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 830B6058: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B605C: 4BFFBE7D  bl 0x830b1ed8
	ctx.lr = 0x830B6060;
	sub_830B1ED8(ctx, base);
	// 830B6060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B6064: 480F2158  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B6068 size=220
    let mut pc: u32 = 0x830B6068;
    'dispatch: loop {
        match pc {
            0x830B6068 => {
    //   block [0x830B6068..0x830B6144)
	// 830B6068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B606C: 480F2101  bl 0x831a816c
	ctx.lr = 0x830B6070;
	sub_831A8130(ctx, base);
	// 830B6070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B6074: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B6078: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 830B607C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830B6080: 396BEF60  addi r11, r11, -0x10a0
	ctx.r[11].s64 = ctx.r[11].s64 + -4256;
	// 830B6084: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830B6088: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B608C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B6090: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830B6094: 48000024  b 0x830b60b8
	pc = 0x830B60B8; continue 'dispatch;
	// 830B6098: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B609C: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 830B60A0: 4B2EB5E9  bl 0x823a1688
	ctx.lr = 0x830B60A4;
	sub_823A1688(ctx, base);
	// 830B60A4: 387D0010  addi r3, r29, 0x10
	ctx.r[3].s64 = ctx.r[29].s64 + 16;
	// 830B60A8: 4BFFBE81  bl 0x830b1f28
	ctx.lr = 0x830B60AC;
	sub_830B1F28(ctx, base);
	// 830B60AC: 93C300D8  stw r30, 0xd8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(216 as u32), ctx.r[30].u32 ) };
	// 830B60B0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B60B4: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830B60B8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830B60BC: 409AFFDC  bne cr6, 0x830b6098
	if !ctx.cr[6].eq {
	pc = 0x830B6098; continue 'dispatch;
	}
	// 830B60C0: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B60C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B60C8: 419A001C  beq cr6, 0x830b60e4
	if ctx.cr[6].eq {
	pc = 0x830B60E4; continue 'dispatch;
	}
	// 830B60CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B60D0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830B60D4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B60D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B60DC: 4E800421  bctrl
	ctx.lr = 0x830B60E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B60E0: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 830B60E4: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 830B60E8: 48001441  bl 0x830b7528
	ctx.lr = 0x830B60EC;
	sub_830B7528(ctx, base);
	// 830B60EC: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 830B60F0: 4B20A179  bl 0x822c0268
	ctx.lr = 0x830B60F4;
	sub_822C0268(ctx, base);
	// 830B60F4: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 830B60F8: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 830B60FC: 4B5286BD  bl 0x825de7b8
	ctx.lr = 0x830B6100;
	sub_825DE7B8(ctx, base);
	// 830B6100: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 830B6104: 4B20A165  bl 0x822c0268
	ctx.lr = 0x830B6108;
	sub_822C0268(ctx, base);
	// 830B6108: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 830B610C: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 830B6110: 4BFFBDC9  bl 0x830b1ed8
	ctx.lr = 0x830B6114;
	sub_830B1ED8(ctx, base);
	// 830B6114: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 830B6118: 4BFFE401  bl 0x830b4518
	ctx.lr = 0x830B611C;
	sub_830B4518(ctx, base);
	// 830B611C: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 830B6120: 4BFFE3F9  bl 0x830b4518
	ctx.lr = 0x830B6124;
	sub_830B4518(ctx, base);
	// 830B6124: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 830B6128: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 830B612C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 830B6130: 396BEF48  addi r11, r11, -0x10b8
	ctx.r[11].s64 = ctx.r[11].s64 + -4280;
	// 830B6134: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B6138: 4BFFBDA1  bl 0x830b1ed8
	ctx.lr = 0x830B613C;
	sub_830B1ED8(ctx, base);
	// 830B613C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B6140: 480F207C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B6148 size=4
    let mut pc: u32 = 0x830B6148;
    'dispatch: loop {
        match pc {
            0x830B6148 => {
    //   block [0x830B6148..0x830B614C)
	// 830B6148: 4BFFFEA8  b 0x830b5ff0
	sub_830B5FF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B6150 size=212
    let mut pc: u32 = 0x830B6150;
    'dispatch: loop {
        match pc {
            0x830B6150 => {
    //   block [0x830B6150..0x830B6224)
	// 830B6150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B6154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B6158: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B615C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B6160: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B6164: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B6168: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B616C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B6170: 4BFFBDB9  bl 0x830b1f28
	ctx.lr = 0x830B6174;
	sub_830B1F28(ctx, base);
	// 830B6174: 4BFFBF8D  bl 0x830b2100
	ctx.lr = 0x830B6178;
	sub_830B2100(ctx, base);
	// 830B6178: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 830B617C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B6180: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 830B6184: 4B279BE5  bl 0x8232fd68
	ctx.lr = 0x830B6188;
	sub_8232FD68(ctx, base);
	// 830B6188: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 830B618C: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 830B6190: 3861007C  addi r3, r1, 0x7c
	ctx.r[3].s64 = ctx.r[1].s64 + 124;
	// 830B6194: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 830B6198: 4B279BD1  bl 0x8232fd68
	ctx.lr = 0x830B619C;
	sub_8232FD68(ctx, base);
	// 830B619C: 38A10078  addi r5, r1, 0x78
	ctx.r[5].s64 = ctx.r[1].s64 + 120;
	// 830B61A0: 389F0014  addi r4, r31, 0x14
	ctx.r[4].s64 = ctx.r[31].s64 + 20;
	// 830B61A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B61A8: 4BFFFC09  bl 0x830b5db0
	ctx.lr = 0x830B61AC;
	sub_830B5DB0(ctx, base);
	// 830B61AC: 3861007C  addi r3, r1, 0x7c
	ctx.r[3].s64 = ctx.r[1].s64 + 124;
	// 830B61B0: 4BFFBD29  bl 0x830b1ed8
	ctx.lr = 0x830B61B4;
	sub_830B1ED8(ctx, base);
	// 830B61B4: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 830B61B8: 4BFFBD21  bl 0x830b1ed8
	ctx.lr = 0x830B61BC;
	sub_830B1ED8(ctx, base);
	// 830B61BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B61C0: 4BFFBD69  bl 0x830b1f28
	ctx.lr = 0x830B61C4;
	sub_830B1F28(ctx, base);
	// 830B61C4: 4BFFBFF5  bl 0x830b21b8
	ctx.lr = 0x830B61C8;
	sub_830B21B8(ctx, base);
	// 830B61C8: D0210068  stfs f1, 0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 830B61CC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B61D0: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 830B61D4: 4B279B95  bl 0x8232fd68
	ctx.lr = 0x830B61D8;
	sub_8232FD68(ctx, base);
	// 830B61D8: C0010068  lfs f0, 0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B61DC: D0010088  stfs f0, 0x88(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 830B61E0: 3881006C  addi r4, r1, 0x6c
	ctx.r[4].s64 = ctx.r[1].s64 + 108;
	// 830B61E4: 3861008C  addi r3, r1, 0x8c
	ctx.r[3].s64 = ctx.r[1].s64 + 140;
	// 830B61E8: 4B279B81  bl 0x8232fd68
	ctx.lr = 0x830B61EC;
	sub_8232FD68(ctx, base);
	// 830B61EC: 38A10088  addi r5, r1, 0x88
	ctx.r[5].s64 = ctx.r[1].s64 + 136;
	// 830B61F0: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 830B61F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B61F8: 4BFFFC81  bl 0x830b5e78
	ctx.lr = 0x830B61FC;
	sub_830B5E78(ctx, base);
	// 830B61FC: 3861008C  addi r3, r1, 0x8c
	ctx.r[3].s64 = ctx.r[1].s64 + 140;
	// 830B6200: 4BFFBCD9  bl 0x830b1ed8
	ctx.lr = 0x830B6204;
	sub_830B1ED8(ctx, base);
	// 830B6204: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 830B6208: 4BFFBCD1  bl 0x830b1ed8
	ctx.lr = 0x830B620C;
	sub_830B1ED8(ctx, base);
	// 830B620C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 830B6210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B6214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B6218: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B621C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B6220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B6228 size=84
    let mut pc: u32 = 0x830B6228;
    'dispatch: loop {
        match pc {
            0x830B6228 => {
    //   block [0x830B6228..0x830B627C)
	// 830B6228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B622C: 480F1F41  bl 0x831a816c
	ctx.lr = 0x830B6230;
	sub_831A8130(ctx, base);
	// 830B6230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B6234: 90810050  stw r4, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 830B6238: D0210054  stfs f1, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 830B623C: 83C30048  lwz r30, 0x48(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 830B6240: 3BE30044  addi r31, r3, 0x44
	ctx.r[31].s64 = ctx.r[3].s64 + 68;
	// 830B6244: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 830B6248: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B624C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B6250: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B6254: 4BFFEEC5  bl 0x830b5118
	ctx.lr = 0x830B6258;
	sub_830B5118(ctx, base);
	// 830B6258: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830B625C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830B6260: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B6264: 4BA520BD  bl 0x82b08320
	ctx.lr = 0x830B6268;
	sub_82B08320(ctx, base);
	// 830B6268: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 830B626C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B6270: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 830B6274: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B6278: 480F1F44  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B6280 size=300
    let mut pc: u32 = 0x830B6280;
    'dispatch: loop {
        match pc {
            0x830B6280 => {
    //   block [0x830B6280..0x830B63AC)
	// 830B6280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B6284: 480F1EE5  bl 0x831a8168
	ctx.lr = 0x830B6288;
	sub_831A8130(ctx, base);
	// 830B6288: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B628C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830B6290: 3BE30020  addi r31, r3, 0x20
	ctx.r[31].s64 = ctx.r[3].s64 + 32;
	// 830B6294: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B6298: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B629C: 3BAB000C  addi r29, r11, 0xc
	ctx.r[29].s64 = ctx.r[11].s64 + 12;
	// 830B62A0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830B62A4: 4BFFEC45  bl 0x830b4ee8
	ctx.lr = 0x830B62A8;
	sub_830B4EE8(ctx, base);
	// 830B62A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B62AC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830B62B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B62B4: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 830B62B8: 4BFFEC89  bl 0x830b4f40
	ctx.lr = 0x830B62BC;
	sub_830B4F40(ctx, base);
	// 830B62BC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830B62C0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830B62C4: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 830B62C8: 4BFFBBF9  bl 0x830b1ec0
	ctx.lr = 0x830B62CC;
	sub_830B1EC0(ctx, base);
	// 830B62CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 830B62D0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B62D4: 396BD3E4  addi r11, r11, -0x2c1c
	ctx.r[11].s64 = ctx.r[11].s64 + -11292;
	// 830B62D8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830B62DC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 830B62E0: 4BFFBCB9  bl 0x830b1f98
	ctx.lr = 0x830B62E4;
	sub_830B1F98(ctx, base);
	// 830B62E4: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 830B62E8: 419A0054  beq cr6, 0x830b633c
	if ctx.cr[6].eq {
	pc = 0x830B633C; continue 'dispatch;
	}
	// 830B62EC: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 830B62F0: 4BFFBC39  bl 0x830b1f28
	ctx.lr = 0x830B62F4;
	sub_830B1F28(ctx, base);
	// 830B62F4: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B62F8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B62FC: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B6300: 419A0020  beq cr6, 0x830b6320
	if ctx.cr[6].eq {
	pc = 0x830B6320; continue 'dispatch;
	}
	// 830B6304: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B6308: 4B2EB381  bl 0x823a1688
	ctx.lr = 0x830B630C;
	sub_823A1688(ctx, base);
	// 830B630C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830B6310: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B6314: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B6318: 409AFFD4  bne cr6, 0x830b62ec
	if !ctx.cr[6].eq {
	pc = 0x830B62EC; continue 'dispatch;
	}
	// 830B631C: 48000020  b 0x830b633c
	pc = 0x830B633C; continue 'dispatch;
	// 830B6320: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 830B6324: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830B6328: 4BFFBD01  bl 0x830b2028
	ctx.lr = 0x830B632C;
	sub_830B2028(ctx, base);
	// 830B632C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830B6330: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B6334: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B6338: 4BA95271  bl 0x82b4b5a8
	ctx.lr = 0x830B633C;
	sub_82B4B5A8(ctx, base);
	// 830B633C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830B6340: 4BFFBC41  bl 0x830b1f80
	ctx.lr = 0x830B6344;
	sub_830B1F80(ctx, base);
	// 830B6344: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B6348: 41820054  beq 0x830b639c
	if ctx.cr[0].eq {
	pc = 0x830B639C; continue 'dispatch;
	}
	// 830B634C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830B6350: 4BFFBBD9  bl 0x830b1f28
	ctx.lr = 0x830B6354;
	sub_830B1F28(ctx, base);
	// 830B6354: 4BFFBE65  bl 0x830b21b8
	ctx.lr = 0x830B6358;
	sub_830B21B8(ctx, base);
	// 830B6358: D0210060  stfs f1, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 830B635C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 830B6360: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 830B6364: 4B279A05  bl 0x8232fd68
	ctx.lr = 0x830B6368;
	sub_8232FD68(ctx, base);
	// 830B6368: C0010060  lfs f0, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B636C: D0010070  stfs f0, 0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 830B6370: 38810064  addi r4, r1, 0x64
	ctx.r[4].s64 = ctx.r[1].s64 + 100;
	// 830B6374: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 830B6378: 4B2799F1  bl 0x8232fd68
	ctx.lr = 0x830B637C;
	sub_8232FD68(ctx, base);
	// 830B637C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 830B6380: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B6384: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B6388: 4BFFFAF1  bl 0x830b5e78
	ctx.lr = 0x830B638C;
	sub_830B5E78(ctx, base);
	// 830B638C: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 830B6390: 4BFFBB49  bl 0x830b1ed8
	ctx.lr = 0x830B6394;
	sub_830B1ED8(ctx, base);
	// 830B6394: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 830B6398: 4BFFBB41  bl 0x830b1ed8
	ctx.lr = 0x830B639C;
	sub_830B1ED8(ctx, base);
	// 830B639C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830B63A0: 4BFFBB39  bl 0x830b1ed8
	ctx.lr = 0x830B63A4;
	sub_830B1ED8(ctx, base);
	// 830B63A4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 830B63A8: 480F1E10  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B63B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B63B0 size=100
    let mut pc: u32 = 0x830B63B0;
    'dispatch: loop {
        match pc {
            0x830B63B0 => {
    //   block [0x830B63B0..0x830B6414)
	// 830B63B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B63B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B63B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B63BC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B63C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B63C4: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830B63C8: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B63CC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830B63D0: 48000028  b 0x830b63f8
	pc = 0x830B63F8; continue 'dispatch;
	// 830B63D4: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 830B63D8: 4BFFBB51  bl 0x830b1f28
	ctx.lr = 0x830B63DC;
	sub_830B1F28(ctx, base);
	// 830B63DC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830B63E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B63E4: 4BFFFC0D  bl 0x830b5ff0
	ctx.lr = 0x830B63E8;
	sub_830B5FF0(ctx, base);
	// 830B63E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B63EC: 4B2EB29D  bl 0x823a1688
	ctx.lr = 0x830B63F0;
	sub_823A1688(ctx, base);
	// 830B63F0: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830B63F4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B63F8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830B63FC: 409AFFD8  bne cr6, 0x830b63d4
	if !ctx.cr[6].eq {
	pc = 0x830B63D4; continue 'dispatch;
	}
	// 830B6400: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B6404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B6408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B640C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B6410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B6418 size=60
    let mut pc: u32 = 0x830B6418;
    'dispatch: loop {
        match pc {
            0x830B6418 => {
    //   block [0x830B6418..0x830B6454)
	// 830B6418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B641C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B6420: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B6424: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B6428: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B642C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830B6430: 4BFFBAF9  bl 0x830b1f28
	ctx.lr = 0x830B6434;
	sub_830B1F28(ctx, base);
	// 830B6434: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830B6438: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B643C: 4BFFFBB5  bl 0x830b5ff0
	ctx.lr = 0x830B6440;
	sub_830B5FF0(ctx, base);
	// 830B6440: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B6444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B6448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B644C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B6450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B6458 size=140
    let mut pc: u32 = 0x830B6458;
    'dispatch: loop {
        match pc {
            0x830B6458 => {
    //   block [0x830B6458..0x830B64E4)
	// 830B6458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B645C: 480F1D09  bl 0x831a8164
	ctx.lr = 0x830B6460;
	sub_831A8130(ctx, base);
	// 830B6460: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B6464: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 830B6468: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830B646C: 93610050  stw r27, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 830B6470: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B6474: 3BFD0014  addi r31, r29, 0x14
	ctx.r[31].s64 = ctx.r[29].s64 + 20;
	// 830B6478: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B647C: 4BFFE98D  bl 0x830b4e08
	ctx.lr = 0x830B6480;
	sub_830B4E08(ctx, base);
	// 830B6480: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B6484: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B6488: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B648C: 4BFFE9ED  bl 0x830b4e78
	ctx.lr = 0x830B6490;
	sub_830B4E78(ctx, base);
	// 830B6490: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830B6494: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830B6498: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 830B649C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830B64A0: 419A002C  beq cr6, 0x830b64cc
	if ctx.cr[6].eq {
	pc = 0x830B64CC; continue 'dispatch;
	}
	// 830B64A4: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 830B64A8: 4BFFBA81  bl 0x830b1f28
	ctx.lr = 0x830B64AC;
	sub_830B1F28(ctx, base);
	// 830B64AC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830B64B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B64B4: 4BFFFB3D  bl 0x830b5ff0
	ctx.lr = 0x830B64B8;
	sub_830B5FF0(ctx, base);
	// 830B64B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B64BC: 4B2EB1CD  bl 0x823a1688
	ctx.lr = 0x830B64C0;
	sub_823A1688(ctx, base);
	// 830B64C0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B64C4: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 830B64C8: 409AFFDC  bne cr6, 0x830b64a4
	if !ctx.cr[6].eq {
	pc = 0x830B64A4; continue 'dispatch;
	}
	// 830B64CC: 93610050  stw r27, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 830B64D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B64D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B64D8: 4BFFF871  bl 0x830b5d48
	ctx.lr = 0x830B64DC;
	sub_830B5D48(ctx, base);
	// 830B64DC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830B64E0: 480F1CD4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B64E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B64E8 size=100
    let mut pc: u32 = 0x830B64E8;
    'dispatch: loop {
        match pc {
            0x830B64E8 => {
    //   block [0x830B64E8..0x830B654C)
	// 830B64E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B64EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B64F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B64F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B64F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B64FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B6500: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 830B6504: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B6508: 48000018  b 0x830b6520
	pc = 0x830B6520; continue 'dispatch;
	// 830B650C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B6510: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B6514: 4BFFFD6D  bl 0x830b6280
	ctx.lr = 0x830B6518;
	sub_830B6280(ctx, base);
	// 830B6518: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 830B651C: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B6520: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 830B6524: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B6528: 409AFFE4  bne cr6, 0x830b650c
	if !ctx.cr[6].eq {
	pc = 0x830B650C; continue 'dispatch;
	}
	// 830B652C: 387E0044  addi r3, r30, 0x44
	ctx.r[3].s64 = ctx.r[30].s64 + 68;
	// 830B6530: 48000FF9  bl 0x830b7528
	ctx.lr = 0x830B6534;
	sub_830B7528(ctx, base);
	// 830B6534: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B6538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B653C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B6540: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B6544: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B6548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B6550 size=280
    let mut pc: u32 = 0x830B6550;
    'dispatch: loop {
        match pc {
            0x830B6550 => {
    //   block [0x830B6550..0x830B6668)
	// 830B6550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B6554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B6558: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B655C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B6560: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B6564: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B6568: 4BFFC421  bl 0x830b2988
	ctx.lr = 0x830B656C;
	sub_830B2988(ctx, base);
	// 830B656C: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830B6570: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 830B6574: 394A7550  addi r10, r10, 0x7550
	ctx.r[10].s64 = ctx.r[10].s64 + 30032;
	// 830B6578: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830B657C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 830B6580: 480013B1  bl 0x830b7930
	ctx.lr = 0x830B6584;
	sub_830B7930(ctx, base);
	// 830B6584: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B6588: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830B658C: 396B757C  addi r11, r11, 0x757c
	ctx.r[11].s64 = ctx.r[11].s64 + 30076;
	// 830B6590: 394A7574  addi r10, r10, 0x7574
	ctx.r[10].s64 = ctx.r[10].s64 + 30068;
	// 830B6594: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B6598: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 830B659C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 830B65A0: 4B368269  bl 0x8241e808
	ctx.lr = 0x830B65A4;
	sub_8241E808(ctx, base);
	// 830B65A4: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 830B65A8: 4B368261  bl 0x8241e808
	ctx.lr = 0x830B65AC;
	sub_8241E808(ctx, base);
	// 830B65AC: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 830B65B0: 4B368259  bl 0x8241e808
	ctx.lr = 0x830B65B4;
	sub_8241E808(ctx, base);
	// 830B65B4: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 830B65B8: 4B368251  bl 0x8241e808
	ctx.lr = 0x830B65BC;
	sub_8241E808(ctx, base);
	// 830B65BC: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 830B65C0: 4B368249  bl 0x8241e808
	ctx.lr = 0x830B65C4;
	sub_8241E808(ctx, base);
	// 830B65C4: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830B65C8: 4B368241  bl 0x8241e808
	ctx.lr = 0x830B65CC;
	sub_8241E808(ctx, base);
	// 830B65CC: 3BDF0090  addi r30, r31, 0x90
	ctx.r[30].s64 = ctx.r[31].s64 + 144;
	// 830B65D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B65D4: 4BFFB8ED  bl 0x830b1ec0
	ctx.lr = 0x830B65D8;
	sub_830B1EC0(ctx, base);
	// 830B65D8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 830B65DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B65E0: 396BC418  addi r11, r11, -0x3be8
	ctx.r[11].s64 = ctx.r[11].s64 + -15336;
	// 830B65E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B65E8: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 830B65EC: 4BFFB9AD  bl 0x830b1f98
	ctx.lr = 0x830B65F0;
	sub_830B1F98(ctx, base);
	// 830B65F0: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 830B65F4: 4B48F55D  bl 0x82545b50
	ctx.lr = 0x830B65F8;
	sub_82545B50(ctx, base);
	// 830B65F8: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830B65FC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830B6600: 394A7558  addi r10, r10, 0x7558
	ctx.r[10].s64 = ctx.r[10].s64 + 30040;
	// 830B6604: 917F00B0  stw r11, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 830B6608: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 830B660C: 915F00A0  stw r10, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[10].u32 ) };
	// 830B6610: 917F00B4  stw r11, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 830B6614: 4BFFCEE5  bl 0x830b34f8
	ctx.lr = 0x830B6618;
	sub_830B34F8(ctx, base);
	// 830B6618: 3BDF00D0  addi r30, r31, 0xd0
	ctx.r[30].s64 = ctx.r[31].s64 + 208;
	// 830B661C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B6620: 4BFFB8A1  bl 0x830b1ec0
	ctx.lr = 0x830B6624;
	sub_830B1EC0(ctx, base);
	// 830B6624: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B6628: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B662C: 396B74C0  addi r11, r11, 0x74c0
	ctx.r[11].s64 = ctx.r[11].s64 + 29888;
	// 830B6630: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B6634: 917F00D0  stw r11, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[11].u32 ) };
	// 830B6638: 4BFFB961  bl 0x830b1f98
	ctx.lr = 0x830B663C;
	sub_830B1F98(ctx, base);
	// 830B663C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B6640: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B6644: 917F00DC  stw r11, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[11].u32 ) };
	// 830B6648: 4BFFBD01  bl 0x830b2348
	ctx.lr = 0x830B664C;
	sub_830B2348(ctx, base);
	// 830B664C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B6650: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B6654: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B6658: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B665C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B6660: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B6664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B6668 size=116
    let mut pc: u32 = 0x830B6668;
    'dispatch: loop {
        match pc {
            0x830B6668 => {
    //   block [0x830B6668..0x830B66DC)
	// 830B6668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B666C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B6670: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B6674: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B6678: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B667C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B6680: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B6684: 419A0044  beq cr6, 0x830b66c8
	if ctx.cr[6].eq {
	pc = 0x830B66C8; continue 'dispatch;
	}
	// 830B6688: 4BFFF909  bl 0x830b5f90
	ctx.lr = 0x830B668C;
	sub_830B5F90(ctx, base);
	// 830B668C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B6690: 4BFFFE59  bl 0x830b64e8
	ctx.lr = 0x830B6694;
	sub_830B64E8(ctx, base);
	// 830B6694: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830B6698: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B669C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830B66A0: 48000020  b 0x830b66c0
	pc = 0x830B66C0; continue 'dispatch;
	// 830B66A4: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 830B66A8: 4BFFB881  bl 0x830b1f28
	ctx.lr = 0x830B66AC;
	sub_830B1F28(ctx, base);
	// 830B66AC: 4BFFBA6D  bl 0x830b2118
	ctx.lr = 0x830B66B0;
	sub_830B2118(ctx, base);
	// 830B66B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B66B4: 4B2EAFD5  bl 0x823a1688
	ctx.lr = 0x830B66B8;
	sub_823A1688(ctx, base);
	// 830B66B8: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830B66BC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B66C0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830B66C4: 409AFFE0  bne cr6, 0x830b66a4
	if !ctx.cr[6].eq {
	pc = 0x830B66A4; continue 'dispatch;
	}
	// 830B66C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B66CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B66D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B66D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B66D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B66E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B66E0 size=144
    let mut pc: u32 = 0x830B66E0;
    'dispatch: loop {
        match pc {
            0x830B66E0 => {
    //   block [0x830B66E0..0x830B6770)
	// 830B66E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B66E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B66E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B66EC: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 830B66F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B66F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B66F8: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 830B66FC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B6700: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B6704: 419A0054  beq cr6, 0x830b6758
	if ctx.cr[6].eq {
	pc = 0x830B6758; continue 'dispatch;
	}
	// 830B6708: 4BFFF889  bl 0x830b5f90
	ctx.lr = 0x830B670C;
	sub_830B5F90(ctx, base);
	// 830B670C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B6710: 4BFFFDD9  bl 0x830b64e8
	ctx.lr = 0x830B6714;
	sub_830B64E8(ctx, base);
	// 830B6714: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830B6718: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B671C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830B6720: 48000030  b 0x830b6750
	pc = 0x830B6750; continue 'dispatch;
	// 830B6724: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 830B6728: 4BFFB801  bl 0x830b1f28
	ctx.lr = 0x830B672C;
	sub_830B1F28(ctx, base);
	// 830B672C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B6730: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 830B6734: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B6738: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B673C: 4E800421  bctrl
	ctx.lr = 0x830B6740;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B6740: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B6744: 4B2EAF45  bl 0x823a1688
	ctx.lr = 0x830B6748;
	sub_823A1688(ctx, base);
	// 830B6748: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830B674C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B6750: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830B6754: 409AFFD0  bne cr6, 0x830b6724
	if !ctx.cr[6].eq {
	pc = 0x830B6724; continue 'dispatch;
	}
	// 830B6758: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B675C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B6760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B6764: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B6768: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B676C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B6770 size=352
    let mut pc: u32 = 0x830B6770;
    'dispatch: loop {
        match pc {
            0x830B6770 => {
    //   block [0x830B6770..0x830B68D0)
	// 830B6770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B6774: 480F19ED  bl 0x831a8160
	ctx.lr = 0x830B6778;
	sub_831A8130(ctx, base);
	// 830B6778: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B677C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B6780: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 830B6784: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 830B6788: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 830B678C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B6790: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B6794: 409A0014  bne cr6, 0x830b67a8
	if !ctx.cr[6].eq {
	pc = 0x830B67A8; continue 'dispatch;
	}
	// 830B6798: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B679C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830B67A0: 4B89A471  bl 0x82950c10
	ctx.lr = 0x830B67A4;
	sub_82950C10(ctx, base);
	// 830B67A4: 48000120  b 0x830b68c4
	pc = 0x830B68C4; continue 'dispatch;
	// 830B67A8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 830B67AC: 3880002F  li r4, 0x2f
	ctx.r[4].s64 = 47;
	// 830B67B0: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 830B67B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B67B8: 480F6669  bl 0x831ace20
	ctx.lr = 0x830B67BC;
	sub_831ACE20(ctx, base);
	// 830B67BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B67C0: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B67C4: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 830B67C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B67CC: 4182000C  beq 0x830b67d8
	if ctx.cr[0].eq {
	pc = 0x830B67D8; continue 'dispatch;
	}
	// 830B67D0: 48005421  bl 0x830bbbf0
	ctx.lr = 0x830B67D4;
	sub_830BBBF0(ctx, base);
	// 830B67D4: 48000008  b 0x830b67dc
	pc = 0x830B67DC; continue 'dispatch;
	// 830B67D8: 48005401  bl 0x830bbbd8
	ctx.lr = 0x830B67DC;
	sub_830BBBD8(ctx, base);
	// 830B67DC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830B67E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B67E4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 830B67E8: 419AFFB4  beq cr6, 0x830b679c
	if ctx.cr[6].eq {
	pc = 0x830B679C; continue 'dispatch;
	}
	// 830B67EC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830B67F0: 4B89A421  bl 0x82950c10
	ctx.lr = 0x830B67F4;
	sub_82950C10(ctx, base);
	// 830B67F4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830B67F8: 419A0028  beq cr6, 0x830b6820
	if ctx.cr[6].eq {
	pc = 0x830B6820; continue 'dispatch;
	}
	// 830B67FC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B6800: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B6804: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B6808: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B680C: 4E800421  bctrl
	ctx.lr = 0x830B6810;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B6810: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830B6814: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830B6818: 4BFFB899  bl 0x830b20b0
	ctx.lr = 0x830B681C;
	sub_830B20B0(ctx, base);
	// 830B681C: 48000068  b 0x830b6884
	pc = 0x830B6884; continue 'dispatch;
	// 830B6820: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830B6824: 9B6B0000  stb r27, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u8 ) };
	// 830B6828: 4BFFE421  bl 0x830b4c48
	ctx.lr = 0x830B682C;
	sub_830B4C48(ctx, base);
	// 830B682C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B6830: 38C0057F  li r6, 0x57f
	ctx.r[6].s64 = 1407;
	// 830B6834: 38AB759C  addi r5, r11, 0x759c
	ctx.r[5].s64 = ctx.r[11].s64 + 30108;
	// 830B6838: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B683C: 388000E0  li r4, 0xe0
	ctx.r[4].s64 = 224;
	// 830B6840: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B6844: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B6848: 4E800421  bctrl
	ctx.lr = 0x830B684C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B684C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B6850: 41820010  beq 0x830b6860
	if ctx.cr[0].eq {
	pc = 0x830B6860; continue 'dispatch;
	}
	// 830B6854: 4BFFFCFD  bl 0x830b6550
	ctx.lr = 0x830B6858;
	sub_830B6550(ctx, base);
	// 830B6858: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830B685C: 48000008  b 0x830b6864
	pc = 0x830B6864; continue 'dispatch;
	// 830B6860: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830B6864: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830B6868: 8BE10050  lbz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B686C: 4BFFB72D  bl 0x830b1f98
	ctx.lr = 0x830B6870;
	sub_830B1F98(ctx, base);
	// 830B6870: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 830B6874: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B6878: 419A000C  beq cr6, 0x830b6884
	if ctx.cr[6].eq {
	pc = 0x830B6884; continue 'dispatch;
	}
	// 830B687C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B6880: 4BFFB611  bl 0x830b1e90
	ctx.lr = 0x830B6884;
	sub_830B1E90(ctx, base);
	// 830B6884: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830B6888: 4BFFB6A1  bl 0x830b1f28
	ctx.lr = 0x830B688C;
	sub_830B1F28(ctx, base);
	// 830B688C: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 830B6890: 38BE0004  addi r5, r30, 4
	ctx.r[5].s64 = ctx.r[30].s64 + 4;
	// 830B6894: 811E0034  lwz r8, 0x34(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B6898: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830B689C: 80C10054  lwz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830B68A0: 4BFFCAB1  bl 0x830b3350
	ctx.lr = 0x830B68A4;
	sub_830B3350(ctx, base);
	// 830B68A4: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 830B68A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B68AC: 4BFFF8A5  bl 0x830b6150
	ctx.lr = 0x830B68B0;
	sub_830B6150(ctx, base);
	// 830B68B0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 830B68B4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830B68B8: 4B2794B1  bl 0x8232fd68
	ctx.lr = 0x830B68BC;
	sub_8232FD68(ctx, base);
	// 830B68BC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830B68C0: 4BFFB619  bl 0x830b1ed8
	ctx.lr = 0x830B68C4;
	sub_830B1ED8(ctx, base);
	// 830B68C4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830B68C8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 830B68CC: 480F18E4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B68D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B68D0 size=264
    let mut pc: u32 = 0x830B68D0;
    'dispatch: loop {
        match pc {
            0x830B68D0 => {
    //   block [0x830B68D0..0x830B69D8)
	// 830B68D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B68D4: 480F188D  bl 0x831a8160
	ctx.lr = 0x830B68D8;
	sub_831A8130(ctx, base);
	// 830B68D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B68DC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830B68E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B68E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B68E8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830B68EC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 830B68F0: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 830B68F4: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 830B68F8: 4B89A319  bl 0x82950c10
	ctx.lr = 0x830B68FC;
	sub_82950C10(ctx, base);
	// 830B68FC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830B6900: 419A0028  beq cr6, 0x830b6928
	if ctx.cr[6].eq {
	pc = 0x830B6928; continue 'dispatch;
	}
	// 830B6904: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B6908: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B690C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B6910: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B6914: 4E800421  bctrl
	ctx.lr = 0x830B6918;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B6918: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830B691C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B6920: 4BFFB791  bl 0x830b20b0
	ctx.lr = 0x830B6924;
	sub_830B20B0(ctx, base);
	// 830B6924: 4800006C  b 0x830b6990
	pc = 0x830B6990; continue 'dispatch;
	// 830B6928: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830B692C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B6930: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 830B6934: 4BFFE315  bl 0x830b4c48
	ctx.lr = 0x830B6938;
	sub_830B4C48(ctx, base);
	// 830B6938: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B693C: 38C00599  li r6, 0x599
	ctx.r[6].s64 = 1433;
	// 830B6940: 38AB759C  addi r5, r11, 0x759c
	ctx.r[5].s64 = ctx.r[11].s64 + 30108;
	// 830B6944: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B6948: 388000E0  li r4, 0xe0
	ctx.r[4].s64 = 224;
	// 830B694C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B6950: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B6954: 4E800421  bctrl
	ctx.lr = 0x830B6958;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B6958: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B695C: 41820010  beq 0x830b696c
	if ctx.cr[0].eq {
	pc = 0x830B696C; continue 'dispatch;
	}
	// 830B6960: 4BFFFBF1  bl 0x830b6550
	ctx.lr = 0x830B6964;
	sub_830B6550(ctx, base);
	// 830B6964: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830B6968: 48000008  b 0x830b6970
	pc = 0x830B6970; continue 'dispatch;
	// 830B696C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B6970: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B6974: 8BC10050  lbz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B6978: 4BFFB621  bl 0x830b1f98
	ctx.lr = 0x830B697C;
	sub_830B1F98(ctx, base);
	// 830B697C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B6980: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B6984: 419A000C  beq cr6, 0x830b6990
	if ctx.cr[6].eq {
	pc = 0x830B6990; continue 'dispatch;
	}
	// 830B6988: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B698C: 4BFFB505  bl 0x830b1e90
	ctx.lr = 0x830B6990;
	sub_830B1E90(ctx, base);
	// 830B6990: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B6994: 4BFFB595  bl 0x830b1f28
	ctx.lr = 0x830B6998;
	sub_830B1F28(ctx, base);
	// 830B6998: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 830B699C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 830B69A0: 811D0034  lwz r8, 0x34(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B69A4: 38BD0004  addi r5, r29, 4
	ctx.r[5].s64 = ctx.r[29].s64 + 4;
	// 830B69A8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830B69AC: 4BFFC9A5  bl 0x830b3350
	ctx.lr = 0x830B69B0;
	sub_830B3350(ctx, base);
	// 830B69B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B69B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B69B8: 4BFFF799  bl 0x830b6150
	ctx.lr = 0x830B69BC;
	sub_830B6150(ctx, base);
	// 830B69BC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830B69C0: 4BFFB569  bl 0x830b1f28
	ctx.lr = 0x830B69C4;
	sub_830B1F28(ctx, base);
	// 830B69C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B69C8: 48007DC9  bl 0x830be790
	ctx.lr = 0x830B69CC;
	sub_830BE790(ctx, base);
	// 830B69CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B69D0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830B69D4: 480F17DC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B69D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B69D8 size=92
    let mut pc: u32 = 0x830B69D8;
    'dispatch: loop {
        match pc {
            0x830B69D8 => {
    //   block [0x830B69D8..0x830B6A34)
	// 830B69D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B69DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B69E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B69E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B69E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B69EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B69F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B69F4: 4BFFDB25  bl 0x830b4518
	ctx.lr = 0x830B69F8;
	sub_830B4518(ctx, base);
	// 830B69F8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B69FC: 4182001C  beq 0x830b6a18
	if ctx.cr[0].eq {
	pc = 0x830B6A18; continue 'dispatch;
	}
	// 830B6A00: 4BFFE249  bl 0x830b4c48
	ctx.lr = 0x830B6A04;
	sub_830B4C48(ctx, base);
	// 830B6A04: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B6A08: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B6A0C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830B6A10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B6A14: 4E800421  bctrl
	ctx.lr = 0x830B6A18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B6A18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B6A1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B6A20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B6A24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B6A28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B6A2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B6A30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B6A38 size=144
    let mut pc: u32 = 0x830B6A38;
    'dispatch: loop {
        match pc {
            0x830B6A38 => {
    //   block [0x830B6A38..0x830B6AC8)
	// 830B6A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B6A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B6A40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B6A44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B6A48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B6A4C: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B6A50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B6A54: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 830B6A58: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B6A5C: 409A0010  bne cr6, 0x830b6a6c
	if !ctx.cr[6].eq {
	pc = 0x830B6A6C; continue 'dispatch;
	}
	// 830B6A60: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B6A64: 4B89A1AD  bl 0x82950c10
	ctx.lr = 0x830B6A68;
	sub_82950C10(ctx, base);
	// 830B6A68: 48000044  b 0x830b6aac
	pc = 0x830B6AAC; continue 'dispatch;
	// 830B6A6C: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 830B6A70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B6A74: 4BFFFCFD  bl 0x830b6770
	ctx.lr = 0x830B6A78;
	sub_830B6770(ctx, base);
	// 830B6A78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B6A7C: 4BFFB505  bl 0x830b1f80
	ctx.lr = 0x830B6A80;
	sub_830B1F80(ctx, base);
	// 830B6A80: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B6A84: 41820014  beq 0x830b6a98
	if ctx.cr[0].eq {
	pc = 0x830B6A98; continue 'dispatch;
	}
	// 830B6A88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B6A8C: 4BFFB49D  bl 0x830b1f28
	ctx.lr = 0x830B6A90;
	sub_830B1F28(ctx, base);
	// 830B6A90: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B6A94: 4BFFB96D  bl 0x830b2400
	ctx.lr = 0x830B6A98;
	sub_830B2400(ctx, base);
	// 830B6A98: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B6A9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B6AA0: 4B2792C9  bl 0x8232fd68
	ctx.lr = 0x830B6AA4;
	sub_8232FD68(ctx, base);
	// 830B6AA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B6AA8: 4BFFB431  bl 0x830b1ed8
	ctx.lr = 0x830B6AAC;
	sub_830B1ED8(ctx, base);
	// 830B6AAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B6AB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B6AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B6AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B6ABC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B6AC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B6AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B6AC8 size=144
    let mut pc: u32 = 0x830B6AC8;
    'dispatch: loop {
        match pc {
            0x830B6AC8 => {
    //   block [0x830B6AC8..0x830B6B58)
	// 830B6AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B6ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B6AD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B6AD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B6AD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B6ADC: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B6AE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B6AE4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 830B6AE8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B6AEC: 409A0010  bne cr6, 0x830b6afc
	if !ctx.cr[6].eq {
	pc = 0x830B6AFC; continue 'dispatch;
	}
	// 830B6AF0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B6AF4: 4B89A11D  bl 0x82950c10
	ctx.lr = 0x830B6AF8;
	sub_82950C10(ctx, base);
	// 830B6AF8: 48000044  b 0x830b6b3c
	pc = 0x830B6B3C; continue 'dispatch;
	// 830B6AFC: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 830B6B00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B6B04: 4BFFFC6D  bl 0x830b6770
	ctx.lr = 0x830B6B08;
	sub_830B6770(ctx, base);
	// 830B6B08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B6B0C: 4BFFB475  bl 0x830b1f80
	ctx.lr = 0x830B6B10;
	sub_830B1F80(ctx, base);
	// 830B6B10: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B6B14: 41820014  beq 0x830b6b28
	if ctx.cr[0].eq {
	pc = 0x830B6B28; continue 'dispatch;
	}
	// 830B6B18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B6B1C: 4BFFB40D  bl 0x830b1f28
	ctx.lr = 0x830B6B20;
	sub_830B1F28(ctx, base);
	// 830B6B20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B6B24: 4BFFBD85  bl 0x830b28a8
	ctx.lr = 0x830B6B28;
	sub_830B28A8(ctx, base);
	// 830B6B28: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B6B2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B6B30: 4B279239  bl 0x8232fd68
	ctx.lr = 0x830B6B34;
	sub_8232FD68(ctx, base);
	// 830B6B34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B6B38: 4BFFB3A1  bl 0x830b1ed8
	ctx.lr = 0x830B6B3C;
	sub_830B1ED8(ctx, base);
	// 830B6B3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B6B40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B6B44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B6B48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B6B4C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B6B50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B6B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B6B58 size=80
    let mut pc: u32 = 0x830B6B58;
    'dispatch: loop {
        match pc {
            0x830B6B58 => {
    //   block [0x830B6B58..0x830B6BA8)
	// 830B6B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B6B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B6B60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B6B64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B6B68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B6B6C: 4BFFFC05  bl 0x830b6770
	ctx.lr = 0x830B6B70;
	sub_830B6770(ctx, base);
	// 830B6B70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B6B74: 4BFFB40D  bl 0x830b1f80
	ctx.lr = 0x830B6B78;
	sub_830B1F80(ctx, base);
	// 830B6B78: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B6B7C: 41820014  beq 0x830b6b90
	if ctx.cr[0].eq {
	pc = 0x830B6B90; continue 'dispatch;
	}
	// 830B6B80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B6B84: 4BFFB3A5  bl 0x830b1f28
	ctx.lr = 0x830B6B88;
	sub_830B1F28(ctx, base);
	// 830B6B88: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B6B8C: 4BFFB875  bl 0x830b2400
	ctx.lr = 0x830B6B90;
	sub_830B2400(ctx, base);
	// 830B6B90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B6B94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B6B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B6B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B6BA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B6BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B6BA8 size=12
    let mut pc: u32 = 0x830B6BA8;
    'dispatch: loop {
        match pc {
            0x830B6BA8 => {
    //   block [0x830B6BA8..0x830B6BB4)
	// 830B6BA8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830B6BAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B6BB0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6BB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B6BB4 size=8
    let mut pc: u32 = 0x830B6BB4;
    'dispatch: loop {
        match pc {
            0x830B6BB4 => {
    //   block [0x830B6BB4..0x830B6BBC)
	// 830B6BB4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830B6BB8: 4BFFFE20  b 0x830b69d8
	sub_830B69D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6BBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B6BBC size=4
    let mut pc: u32 = 0x830B6BBC;
    'dispatch: loop {
        match pc {
            0x830B6BBC => {
    //   block [0x830B6BBC..0x830B6BC0)
	// 830B6BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B6BC0 size=212
    let mut pc: u32 = 0x830B6BC0;
    'dispatch: loop {
        match pc {
            0x830B6BC0 => {
    //   block [0x830B6BC0..0x830B6C94)
	// 830B6BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B6BC4: 480F159D  bl 0x831a8160
	ctx.lr = 0x830B6BC8;
	sub_831A8130(ctx, base);
	// 830B6BC8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B6BCC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830B6BD0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830B6BD4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830B6BD8: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 830B6BDC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830B6BE0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B6BE4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B6BE8: 40990064  ble cr6, 0x830b6c4c
	if !ctx.cr[6].gt {
	pc = 0x830B6C4C; continue 'dispatch;
	}
	// 830B6BEC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830B6BF0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B6BF4: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 830B6BF8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B6BFC: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 830B6C00: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 830B6C04: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830B6C08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B6C0C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B6C10: 80CB0000  lwz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B6C14: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830B6C18: 7CAB502E  lwzx r5, r11, r10
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830B6C1C: 4BFFFCB5  bl 0x830b68d0
	ctx.lr = 0x830B6C20;
	sub_830B68D0(ctx, base);
	// 830B6C20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B6C24: 4BFFB305  bl 0x830b1f28
	ctx.lr = 0x830B6C28;
	sub_830B1F28(ctx, base);
	// 830B6C28: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B6C2C: 4BFFB7D5  bl 0x830b2400
	ctx.lr = 0x830B6C30;
	sub_830B2400(ctx, base);
	// 830B6C30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B6C34: 4BFFB2A5  bl 0x830b1ed8
	ctx.lr = 0x830B6C38;
	sub_830B1ED8(ctx, base);
	// 830B6C38: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B6C3C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 830B6C40: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 830B6C44: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830B6C48: 4198FFA8  blt cr6, 0x830b6bf0
	if ctx.cr[6].lt {
	pc = 0x830B6BF0; continue 'dispatch;
	}
	// 830B6C4C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B6C50: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830B6C54: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B6C58: 40990034  ble cr6, 0x830b6c8c
	if !ctx.cr[6].gt {
	pc = 0x830B6C8C; continue 'dispatch;
	}
	// 830B6C5C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830B6C60: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830B6C64: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 830B6C68: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 830B6C6C: 7C8BF214  add r4, r11, r30
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 830B6C70: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830B6C74: 4BFFFF4D  bl 0x830b6bc0
	ctx.lr = 0x830B6C78;
	sub_830B6BC0(ctx, base);
	// 830B6C78: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B6C7C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830B6C80: 3BDE0018  addi r30, r30, 0x18
	ctx.r[30].s64 = ctx.r[30].s64 + 24;
	// 830B6C84: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830B6C88: 4198FFD8  blt cr6, 0x830b6c60
	if ctx.cr[6].lt {
	pc = 0x830B6C60; continue 'dispatch;
	}
	// 830B6C8C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830B6C90: 480F1520  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B6C98 size=192
    let mut pc: u32 = 0x830B6C98;
    'dispatch: loop {
        match pc {
            0x830B6C98 => {
    //   block [0x830B6C98..0x830B6D58)
	// 830B6C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B6C9C: 480F14CD  bl 0x831a8168
	ctx.lr = 0x830B6CA0;
	sub_831A8130(ctx, base);
	// 830B6CA0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B6CA4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B6CA8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830B6CAC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830B6CB0: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B6CB4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B6CB8: 409A0010  bne cr6, 0x830b6cc8
	if !ctx.cr[6].eq {
	pc = 0x830B6CC8; continue 'dispatch;
	}
	// 830B6CBC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B6CC0: 4BFFDFF1  bl 0x830b4cb0
	ctx.lr = 0x830B6CC4;
	sub_830B4CB0(ctx, base);
	// 830B6CC4: 48000088  b 0x830b6d4c
	pc = 0x830B6D4C; continue 'dispatch;
	// 830B6CC8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830B6CCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B6CD0: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 830B6CD4: 4BFFDF75  bl 0x830b4c48
	ctx.lr = 0x830B6CD8;
	sub_830B4C48(ctx, base);
	// 830B6CD8: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B6CDC: 38C0029C  li r6, 0x29c
	ctx.r[6].s64 = 668;
	// 830B6CE0: 38AB759C  addi r5, r11, 0x759c
	ctx.r[5].s64 = ctx.r[11].s64 + 30108;
	// 830B6CE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B6CE8: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 830B6CEC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B6CF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B6CF4: 4E800421  bctrl
	ctx.lr = 0x830B6CF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B6CF8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830B6CFC: 41820014  beq 0x830b6d10
	if ctx.cr[0].eq {
	pc = 0x830B6D10; continue 'dispatch;
	}
	// 830B6D00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B6D04: 4B367B05  bl 0x8241e808
	ctx.lr = 0x830B6D08;
	sub_8241E808(ctx, base);
	// 830B6D08: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B6D0C: 48000008  b 0x830b6d14
	pc = 0x830B6D14; continue 'dispatch;
	// 830B6D10: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B6D14: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830B6D18: 88A10050  lbz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B6D1C: 4BFFE34D  bl 0x830b5068
	ctx.lr = 0x830B6D20;
	sub_830B5068(ctx, base);
	// 830B6D20: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B6D24: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 830B6D28: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 830B6D2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B6D30: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B6D34: 4BFFFE8D  bl 0x830b6bc0
	ctx.lr = 0x830B6D38;
	sub_830B6BC0(ctx, base);
	// 830B6D38: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 830B6D3C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B6D40: 4BFFDFC9  bl 0x830b4d08
	ctx.lr = 0x830B6D44;
	sub_830B4D08(ctx, base);
	// 830B6D44: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830B6D48: 4BFFB191  bl 0x830b1ed8
	ctx.lr = 0x830B6D4C;
	sub_830B1ED8(ctx, base);
	// 830B6D4C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B6D50: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830B6D54: 480F1464  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B6D58 size=52
    let mut pc: u32 = 0x830B6D58;
    'dispatch: loop {
        match pc {
            0x830B6D58 => {
    //   block [0x830B6D58..0x830B6D8C)
	// 830B6D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B6D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B6D60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B6D64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B6D68: 80840034  lwz r4, 0x34(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B6D6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B6D70: 48004F99  bl 0x830bbd08
	ctx.lr = 0x830B6D74;
	sub_830BBD08(ctx, base);
	// 830B6D74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B6D78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B6D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B6D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B6D84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B6D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B6D90 size=52
    let mut pc: u32 = 0x830B6D90;
    'dispatch: loop {
        match pc {
            0x830B6D90 => {
    //   block [0x830B6D90..0x830B6DC4)
	// 830B6D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B6D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B6D98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B6D9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B6DA0: 80840034  lwz r4, 0x34(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B6DA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B6DA8: 48004FC9  bl 0x830bbd70
	ctx.lr = 0x830B6DAC;
	sub_830BBD70(ctx, base);
	// 830B6DAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B6DB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B6DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B6DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B6DBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B6DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B6DC8 size=8
    let mut pc: u32 = 0x830B6DC8;
    'dispatch: loop {
        match pc {
            0x830B6DC8 => {
    //   block [0x830B6DC8..0x830B6DD0)
	// 830B6DC8: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B6DCC: 4800504C  b 0x830bbe18
	sub_830BBE18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B6DD0 size=8
    let mut pc: u32 = 0x830B6DD0;
    'dispatch: loop {
        match pc {
            0x830B6DD0 => {
    //   block [0x830B6DD0..0x830B6DD8)
	// 830B6DD0: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B6DD4: 4800510C  b 0x830bbee0
	sub_830BBEE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B6DD8 size=8
    let mut pc: u32 = 0x830B6DD8;
    'dispatch: loop {
        match pc {
            0x830B6DD8 => {
    //   block [0x830B6DD8..0x830B6DE0)
	// 830B6DD8: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B6DDC: 48005264  b 0x830bc040
	sub_830BC040(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B6DE0 size=8
    let mut pc: u32 = 0x830B6DE0;
    'dispatch: loop {
        match pc {
            0x830B6DE0 => {
    //   block [0x830B6DE0..0x830B6DE8)
	// 830B6DE0: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B6DE4: 480053A4  b 0x830bc188
	sub_830BC188(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B6DE8 size=8
    let mut pc: u32 = 0x830B6DE8;
    'dispatch: loop {
        match pc {
            0x830B6DE8 => {
    //   block [0x830B6DE8..0x830B6DF0)
	// 830B6DE8: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B6DEC: 48005464  b 0x830bc250
	sub_830BC250(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B6DF0 size=8
    let mut pc: u32 = 0x830B6DF0;
    'dispatch: loop {
        match pc {
            0x830B6DF0 => {
    //   block [0x830B6DF0..0x830B6DF8)
	// 830B6DF0: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B6DF4: 480054BC  b 0x830bc2b0
	sub_830BC2B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B6DF8 size=60
    let mut pc: u32 = 0x830B6DF8;
    'dispatch: loop {
        match pc {
            0x830B6DF8 => {
    //   block [0x830B6DF8..0x830B6E34)
	// 830B6DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B6DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B6E00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B6E04: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830B6E08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B6E0C: 808B0034  lwz r4, 0x34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B6E10: 48004F61  bl 0x830bbd70
	ctx.lr = 0x830B6E14;
	sub_830BBD70(ctx, base);
	// 830B6E14: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 830B6E18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B6E1C: 4800135D  bl 0x830b8178
	ctx.lr = 0x830B6E20;
	sub_830B8178(ctx, base);
	// 830B6E20: C0210050  lfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B6E24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B6E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B6E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B6E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B6E38 size=48
    let mut pc: u32 = 0x830B6E38;
    'dispatch: loop {
        match pc {
            0x830B6E38 => {
    //   block [0x830B6E38..0x830B6E68)
	// 830B6E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B6E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B6E40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B6E44: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830B6E48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B6E4C: 808B0034  lwz r4, 0x34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B6E50: 48004F21  bl 0x830bbd70
	ctx.lr = 0x830B6E54;
	sub_830BBD70(ctx, base);
	// 830B6E54: C0210050  lfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B6E58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B6E5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B6E60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B6E64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B6E68 size=60
    let mut pc: u32 = 0x830B6E68;
    'dispatch: loop {
        match pc {
            0x830B6E68 => {
    //   block [0x830B6E68..0x830B6EA4)
	// 830B6E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B6E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B6E70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B6E74: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830B6E78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B6E7C: 808B0034  lwz r4, 0x34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B6E80: 48004EF1  bl 0x830bbd70
	ctx.lr = 0x830B6E84;
	sub_830BBD70(ctx, base);
	// 830B6E84: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 830B6E88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B6E8C: 480012ED  bl 0x830b8178
	ctx.lr = 0x830B6E90;
	sub_830B8178(ctx, base);
	// 830B6E90: C0210054  lfs f1, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B6E94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B6E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B6E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B6EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B6EA8 size=48
    let mut pc: u32 = 0x830B6EA8;
    'dispatch: loop {
        match pc {
            0x830B6EA8 => {
    //   block [0x830B6EA8..0x830B6ED8)
	// 830B6EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B6EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B6EB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B6EB4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830B6EB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B6EBC: 808B0034  lwz r4, 0x34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B6EC0: 48004EB1  bl 0x830bbd70
	ctx.lr = 0x830B6EC4;
	sub_830BBD70(ctx, base);
	// 830B6EC4: C0210054  lfs f1, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B6EC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B6ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B6ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B6ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B6ED8 size=48
    let mut pc: u32 = 0x830B6ED8;
    'dispatch: loop {
        match pc {
            0x830B6ED8 => {
    //   block [0x830B6ED8..0x830B6F08)
	// 830B6ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B6EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B6EE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B6EE4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830B6EE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B6EEC: 808B0034  lwz r4, 0x34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B6EF0: 480051F1  bl 0x830bc0e0
	ctx.lr = 0x830B6EF4;
	sub_830BC0E0(ctx, base);
	// 830B6EF4: C0210050  lfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B6EF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B6EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B6F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B6F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B6F08 size=8
    let mut pc: u32 = 0x830B6F08;
    'dispatch: loop {
        match pc {
            0x830B6F08 => {
    //   block [0x830B6F08..0x830B6F10)
	// 830B6F08: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B6F0C: 48005904  b 0x830bc810
	sub_830BC810(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B6F10 size=48
    let mut pc: u32 = 0x830B6F10;
    'dispatch: loop {
        match pc {
            0x830B6F10 => {
    //   block [0x830B6F10..0x830B6F40)
	// 830B6F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B6F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B6F18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B6F1C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830B6F20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B6F24: 808B0034  lwz r4, 0x34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B6F28: 480051B9  bl 0x830bc0e0
	ctx.lr = 0x830B6F2C;
	sub_830BC0E0(ctx, base);
	// 830B6F2C: C0210054  lfs f1, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B6F30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B6F34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B6F38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B6F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B6F40 size=12
    let mut pc: u32 = 0x830B6F40;
    'dispatch: loop {
        match pc {
            0x830B6F40 => {
    //   block [0x830B6F40..0x830B6F4C)
	// 830B6F40: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B6F44: 908B0008  stw r4, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 830B6F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B6F50 size=64
    let mut pc: u32 = 0x830B6F50;
    'dispatch: loop {
        match pc {
            0x830B6F50 => {
    //   block [0x830B6F50..0x830B6F90)
	// 830B6F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B6F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B6F58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B6F5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B6F60: 80840034  lwz r4, 0x34(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B6F64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B6F68: 48004E09  bl 0x830bbd70
	ctx.lr = 0x830B6F6C;
	sub_830BBD70(ctx, base);
	// 830B6F6C: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 830B6F70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B6F74: 48001205  bl 0x830b8178
	ctx.lr = 0x830B6F78;
	sub_830B8178(ctx, base);
	// 830B6F78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B6F7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B6F80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B6F84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B6F88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B6F8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B6F90 size=76
    let mut pc: u32 = 0x830B6F90;
    'dispatch: loop {
        match pc {
            0x830B6F90 => {
    //   block [0x830B6F90..0x830B6FDC)
	// 830B6F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B6F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B6F98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B6F9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B6FA0: D021007C  stfs f1, 0x7c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 830B6FA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B6FA8: D0410084  stfs f2, 0x84(r1)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 830B6FAC: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 830B6FB0: 3861007C  addi r3, r1, 0x7c
	ctx.r[3].s64 = ctx.r[1].s64 + 124;
	// 830B6FB4: 48001245  bl 0x830b81f8
	ctx.lr = 0x830B6FB8;
	sub_830B81F8(ctx, base);
	// 830B6FB8: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B6FBC: C0410084  lfs f2, 0x84(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 830B6FC0: C021007C  lfs f1, 0x7c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B6FC4: 48004E55  bl 0x830bbe18
	ctx.lr = 0x830B6FC8;
	sub_830BBE18(ctx, base);
	// 830B6FC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B6FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B6FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B6FD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B6FD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B6FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B6FE0 size=92
    let mut pc: u32 = 0x830B6FE0;
    'dispatch: loop {
        match pc {
            0x830B6FE0 => {
    //   block [0x830B6FE0..0x830B703C)
	// 830B6FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B6FE4: 480F1185  bl 0x831a8168
	ctx.lr = 0x830B6FE8;
	sub_831A8130(ctx, base);
	// 830B6FE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B6FEC: 83C4000C  lwz r30, 0xc(r4)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B6FF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B6FF4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830B6FF8: 83BE002C  lwz r29, 0x2c(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 830B6FFC: 480045A5  bl 0x830bb5a0
	ctx.lr = 0x830B7000;
	sub_830BB5A0(ctx, base);
	// 830B7000: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 830B7004: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B7008: 409A0018  bne cr6, 0x830b7020
	if !ctx.cr[6].eq {
	pc = 0x830B7020; continue 'dispatch;
	}
	// 830B700C: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830B7010: 578A103A  slwi r10, r28, 2
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830B7014: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830B7018: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B701C: 48000018  b 0x830b7034
	pc = 0x830B7034; continue 'dispatch;
	// 830B7020: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830B7024: 80DE0030  lwz r6, 0x30(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830B7028: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830B702C: 48004B2D  bl 0x830bbb58
	ctx.lr = 0x830B7030;
	sub_830BBB58(ctx, base);
	// 830B7030: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B7034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B7038: 480F1180  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B7040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B7040 size=68
    let mut pc: u32 = 0x830B7040;
    'dispatch: loop {
        match pc {
            0x830B7040 => {
    //   block [0x830B7040..0x830B7084)
	// 830B7040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B7044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B7048: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B704C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B7050: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 830B7054: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 830B7058: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B705C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830B7060: C02A08A4  lfs f1, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B7064: 48002C25  bl 0x830b9c88
	ctx.lr = 0x830B7068;
	sub_830B9C88(ctx, base);
	// 830B7068: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B706C: 4800534D  bl 0x830bc3b8
	ctx.lr = 0x830B7070;
	sub_830BC3B8(ctx, base);
	// 830B7070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B7074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B7078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B707C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B7080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B7088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B7088 size=16
    let mut pc: u32 = 0x830B7088;
    'dispatch: loop {
        match pc {
            0x830B7088 => {
    //   block [0x830B7088..0x830B7098)
	// 830B7088: 81630034  lwz r11, 0x34(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B708C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B7090: 806B0024  lwz r3, 0x24(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 830B7094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B7098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B7098 size=56
    let mut pc: u32 = 0x830B7098;
    'dispatch: loop {
        match pc {
            0x830B7098 => {
    //   block [0x830B7098..0x830B70D0)
	// 830B7098: 81640034  lwz r11, 0x34(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B709C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B70A0: 814B0094  lwz r10, 0x94(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(148 as u32) ) } as u64;
	// 830B70A4: 816B0090  lwz r11, 0x90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 830B70A8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B70AC: 1D450014  mulli r10, r5, 0x14
	ctx.r[10].s64 = ctx.r[5].s64 * 20;
	// 830B70B0: 409A0008  bne cr6, 0x830b70b8
	if !ctx.cr[6].eq {
	pc = 0x830B70B8; continue 'dispatch;
	}
	// 830B70B4: 1D45000C  mulli r10, r5, 0xc
	ctx.r[10].s64 = ctx.r[5].s64 * 12;
	// 830B70B8: 7D2B502E  lwzx r9, r11, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830B70BC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830B70C0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B70C4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 830B70C8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830B70CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B70D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B70D0 size=80
    let mut pc: u32 = 0x830B70D0;
    'dispatch: loop {
        match pc {
            0x830B70D0 => {
    //   block [0x830B70D0..0x830B7120)
	// 830B70D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B70D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B70D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B70DC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B70E0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 830B70E4: D0210050  stfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 830B70E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B70EC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 830B70F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B70F4: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B70F8: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 830B70FC: 480010FD  bl 0x830b81f8
	ctx.lr = 0x830B7100;
	sub_830B81F8(ctx, base);
	// 830B7100: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B7104: C0210050  lfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B7108: 480055C9  bl 0x830bc6d0
	ctx.lr = 0x830B710C;
	sub_830BC6D0(ctx, base);
	// 830B710C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B7110: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B7114: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B7118: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B711C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B7120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B7120 size=80
    let mut pc: u32 = 0x830B7120;
    'dispatch: loop {
        match pc {
            0x830B7120 => {
    //   block [0x830B7120..0x830B7170)
	// 830B7120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B7124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B7128: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B712C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B7130: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 830B7134: D0210050  stfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 830B7138: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B713C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B7140: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 830B7144: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B7148: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 830B714C: 480010AD  bl 0x830b81f8
	ctx.lr = 0x830B7150;
	sub_830B81F8(ctx, base);
	// 830B7150: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B7154: C0210050  lfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B7158: 48005619  bl 0x830bc770
	ctx.lr = 0x830B715C;
	sub_830BC770(ctx, base);
	// 830B715C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B7160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B7164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B7168: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B716C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B7170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B7170 size=112
    let mut pc: u32 = 0x830B7170;
    'dispatch: loop {
        match pc {
            0x830B7170 => {
    //   block [0x830B7170..0x830B71E0)
	// 830B7170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B7174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B7178: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B717C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B7180: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B7184: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B7188: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B718C: 814B0034  lwz r10, 0x34(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B7190: 7D5E5378  mr r30, r10
	ctx.r[30].u64 = ctx.r[10].u64;
	// 830B7194: 554A05A6  rlwinm r10, r10, 0, 0x16, 0x13
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830B7198: 914B0034  stw r10, 0x34(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 830B719C: 807F0040  lwz r3, 0x40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 830B71A0: 4BFFB021  bl 0x830b21c0
	ctx.lr = 0x830B71A4;
	sub_830B21C0(ctx, base);
	// 830B71A4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B71A8: 93CB0034  stw r30, 0x34(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 830B71AC: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B71B0: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B71B4: 8163007C  lwz r11, 0x7c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) } as u64;
	// 830B71B8: 556B0356  rlwinm r11, r11, 0, 0xd, 0xb
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 830B71BC: 556B02D2  rlwinm r11, r11, 0, 0xb, 9
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 830B71C0: 9163007C  stw r11, 0x7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 830B71C4: 48006295  bl 0x830bd458
	ctx.lr = 0x830B71C8;
	sub_830BD458(ctx, base);
	// 830B71C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B71CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B71D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B71D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B71D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B71DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B71E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B71E0 size=20
    let mut pc: u32 = 0x830B71E0;
    'dispatch: loop {
        match pc {
            0x830B71E0 => {
    //   block [0x830B71E0..0x830B71F4)
	// 830B71E0: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B71E4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B71E8: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 830B71EC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B71F0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B71F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B71F4 size=40
    let mut pc: u32 = 0x830B71F4;
    'dispatch: loop {
        match pc {
            0x830B71F4 => {
    //   block [0x830B71F4..0x830B721C)
	// 830B71F4: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B71F8: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B71FC: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 830B7200: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830B7204: 41980008  blt cr6, 0x830b720c
	if ctx.cr[6].lt {
	pc = 0x830B720C; continue 'dispatch;
	}
	// 830B7208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B720C: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830B7210: 4182000C  beq 0x830b721c
	if ctx.cr[0].eq {
		sub_830B721C(ctx, base);
		return;
	}
	// 830B7214: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B7218: 4800000C  b 0x830b7224
	sub_830B721C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B721C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B721C size=24
    let mut pc: u32 = 0x830B721C;
    'dispatch: loop {
        match pc {
            0x830B721C => {
    //   block [0x830B721C..0x830B7234)
	// 830B721C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830B7220: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7224: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 830B7228: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B722C: 419AFFCC  beq cr6, 0x830b71f8
	if ctx.cr[6].eq {
		sub_830B71F4(ctx, base);
		return;
	}
	// 830B7230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B7238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B7238 size=20
    let mut pc: u32 = 0x830B7238;
    'dispatch: loop {
        match pc {
            0x830B7238 => {
    //   block [0x830B7238..0x830B724C)
	// 830B7238: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B723C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7240: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 830B7244: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B7248: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B724C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B724C size=44
    let mut pc: u32 = 0x830B724C;
    'dispatch: loop {
        match pc {
            0x830B724C => {
    //   block [0x830B724C..0x830B7278)
	// 830B724C: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7250: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B7254: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 830B7258: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830B725C: 41980008  blt cr6, 0x830b7264
	if ctx.cr[6].lt {
	pc = 0x830B7264; continue 'dispatch;
	}
	// 830B7260: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B7264: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830B7268: 41820010  beq 0x830b7278
	if ctx.cr[0].eq {
		sub_830B7278(ctx, base);
		return;
	}
	// 830B726C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830B7270: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7274: 48000008  b 0x830b727c
	sub_830B7278(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B7278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B7278 size=20
    let mut pc: u32 = 0x830B7278;
    'dispatch: loop {
        match pc {
            0x830B7278 => {
    //   block [0x830B7278..0x830B728C)
	// 830B7278: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B727C: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 830B7280: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B7284: 419AFFCC  beq cr6, 0x830b7250
	if ctx.cr[6].eq {
		sub_830B724C(ctx, base);
		return;
	}
	// 830B7288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B7290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B7290 size=104
    let mut pc: u32 = 0x830B7290;
    'dispatch: loop {
        match pc {
            0x830B7290 => {
    //   block [0x830B7290..0x830B72F8)
	// 830B7290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B7294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B7298: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B729C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B72A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B72A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B72A8: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 830B72AC: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 830B72B0: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830B72B4: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 830B72B8: 38870004  addi r4, r7, 4
	ctx.r[4].s64 = ctx.r[7].s64 + 4;
	// 830B72BC: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 830B72C0: 90DF0008  stw r6, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 830B72C4: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B72C8: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830B72CC: 4B4199D5  bl 0x824d0ca0
	ctx.lr = 0x830B72D0;
	sub_824D0CA0(ctx, base);
	// 830B72D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B72D4: 9BDF0018  stb r30, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 830B72D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B72DC: 997F0019  stb r11, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 830B72E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B72E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B72E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B72EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B72F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B72F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B72F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B72F8 size=104
    let mut pc: u32 = 0x830B72F8;
    'dispatch: loop {
        match pc {
            0x830B72F8 => {
    //   block [0x830B72F8..0x830B7360)
	// 830B72F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B72FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B7300: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B7304: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B7308: C1AB000C  lfs f13, 0xc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B730C: C00B001C  lfs f0, 0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B7310: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 830B7314: C18B0024  lfs f12, 0x24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 830B7318: C1AB0014  lfs f13, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B731C: EDAC6828  fsubs f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 - ctx.f[13].f64) as f32) as f64);
	// 830B7320: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 830B7324: 4099000C  ble cr6, 0x830b7330
	if !ctx.cr[6].gt {
	pc = 0x830B7330; continue 'dispatch;
	}
	// 830B7328: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 830B732C: 48000008  b 0x830b7334
	pc = 0x830B7334; continue 'dispatch;
	// 830B7330: D1A10050  stfs f13, 0x50(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 830B7334: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 830B7338: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 830B733C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B7340: C00B08A8  lfs f0, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B7344: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 830B7348: 48000E71  bl 0x830b81b8
	ctx.lr = 0x830B734C;
	sub_830B81B8(ctx, base);
	// 830B734C: C0210050  lfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B7350: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B7354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B7358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B735C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B7360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B7360 size=104
    let mut pc: u32 = 0x830B7360;
    'dispatch: loop {
        match pc {
            0x830B7360 => {
    //   block [0x830B7360..0x830B73C8)
	// 830B7360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B7364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B7368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B736C: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B7370: C1AB0010  lfs f13, 0x10(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B7374: C00B0018  lfs f0, 0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B7378: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 830B737C: C18B0028  lfs f12, 0x28(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 830B7380: C1AB0020  lfs f13, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B7384: EDAC6828  fsubs f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 - ctx.f[13].f64) as f32) as f64);
	// 830B7388: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 830B738C: 4099000C  ble cr6, 0x830b7398
	if !ctx.cr[6].gt {
	pc = 0x830B7398; continue 'dispatch;
	}
	// 830B7390: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 830B7394: 48000008  b 0x830b739c
	pc = 0x830B739C; continue 'dispatch;
	// 830B7398: D1A10050  stfs f13, 0x50(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 830B739C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 830B73A0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B73A4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 830B73A8: C00B08A8  lfs f0, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B73AC: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 830B73B0: 48000E09  bl 0x830b81b8
	ctx.lr = 0x830B73B4;
	sub_830B81B8(ctx, base);
	// 830B73B4: C0210050  lfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B73B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B73BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B73C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B73C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B73C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B73C8 size=12
    let mut pc: u32 = 0x830B73C8;
    'dispatch: loop {
        match pc {
            0x830B73C8 => {
    //   block [0x830B73C8..0x830B73D4)
	// 830B73C8: C0440004  lfs f2, 4(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 830B73CC: C0240000  lfs f1, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B73D0: 4BFFFBC0  b 0x830b6f90
	sub_830B6F90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B73D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B73D8 size=332
    let mut pc: u32 = 0x830B73D8;
    'dispatch: loop {
        match pc {
            0x830B73D8 => {
    //   block [0x830B73D8..0x830B7524)
	// 830B73D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B73DC: 480F0D8D  bl 0x831a8168
	ctx.lr = 0x830B73E0;
	sub_831A8130(ctx, base);
	// 830B73E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B73E4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830B73E8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830B73EC: 817D0034  lwz r11, 0x34(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B73F0: 838B000C  lwz r28, 0xc(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B73F4: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 830B73F8: 815C0064  lwz r10, 0x64(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(100 as u32) ) } as u64;
	// 830B73FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B7400: 419A002C  beq cr6, 0x830b742c
	if ctx.cr[6].eq {
	pc = 0x830B742C; continue 'dispatch;
	}
	// 830B7404: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7408: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B740C: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830B7410: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 830B7414: 41820044  beq 0x830b7458
	if ctx.cr[0].eq {
	pc = 0x830B7458; continue 'dispatch;
	}
	// 830B7418: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830B741C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830B7420: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 830B7424: 419AFFE0  beq cr6, 0x830b7404
	if ctx.cr[6].eq {
	pc = 0x830B7404; continue 'dispatch;
	}
	// 830B7428: 48000030  b 0x830b7458
	pc = 0x830B7458; continue 'dispatch;
	// 830B742C: 815D000C  lwz r10, 0xc(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B7430: 814A0044  lwz r10, 0x44(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(68 as u32) ) } as u64;
	// 830B7434: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7438: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B743C: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830B7440: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 830B7444: 41820014  beq 0x830b7458
	if ctx.cr[0].eq {
	pc = 0x830B7458; continue 'dispatch;
	}
	// 830B7448: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830B744C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830B7450: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 830B7454: 419AFFE0  beq cr6, 0x830b7434
	if ctx.cr[6].eq {
	pc = 0x830B7434; continue 'dispatch;
	}
	// 830B7458: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830B745C: 418200C0  beq 0x830b751c
	if ctx.cr[0].eq {
	pc = 0x830B751C; continue 'dispatch;
	}
	// 830B7460: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 830B7464: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7468: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830B746C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B7470: 409AFFF4  bne cr6, 0x830b7464
	if !ctx.cr[6].eq {
	pc = 0x830B7464; continue 'dispatch;
	}
	// 830B7474: 7D7F5850  subf r11, r31, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 830B7478: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830B747C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 830B7480: 388A7628  addi r4, r10, 0x7628
	ctx.r[4].s64 = ctx.r[10].s64 + 30248;
	// 830B7484: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830B7488: 38A0080D  li r5, 0x80d
	ctx.r[5].s64 = 2061;
	// 830B748C: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 830B7490: 4BFFD7C9  bl 0x830b4c58
	ctx.lr = 0x830B7494;
	sub_830B4C58(ctx, base);
	// 830B7494: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830B7498: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 830B749C: 7D5F2050  subf r10, r31, r4
	ctx.r[10].s64 = ctx.r[4].s64 - ctx.r[31].s64;
	// 830B74A0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B74A4: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B74A8: 7D2A59AE  stbx r9, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 830B74AC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830B74B0: 4082FFF0  bne 0x830b74a0
	if !ctx.cr[0].eq {
	pc = 0x830B74A0; continue 'dispatch;
	}
	// 830B74B4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830B74B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B74BC: 3BFD004C  addi r31, r29, 0x4c
	ctx.r[31].s64 = ctx.r[29].s64 + 76;
	// 830B74C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B74C4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 830B74C8: 8BC10050  lbz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B74CC: 4BFFAACD  bl 0x830b1f98
	ctx.lr = 0x830B74D0;
	sub_830B1F98(ctx, base);
	// 830B74D0: 807D0050  lwz r3, 0x50(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B74D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B74D8: 419A000C  beq cr6, 0x830b74e4
	if ctx.cr[6].eq {
	pc = 0x830B74E4; continue 'dispatch;
	}
	// 830B74DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B74E0: 4BFFA9A1  bl 0x830b1e80
	ctx.lr = 0x830B74E4;
	sub_830B1E80(ctx, base);
	// 830B74E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B74E8: 4BFFAA41  bl 0x830b1f28
	ctx.lr = 0x830B74EC;
	sub_830B1F28(ctx, base);
	// 830B74EC: 907C0064  stw r3, 0x64(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 830B74F0: 817D0038  lwz r11, 0x38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 830B74F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B74F8: 419A001C  beq cr6, 0x830b7514
	if ctx.cr[6].eq {
	pc = 0x830B7514; continue 'dispatch;
	}
	// 830B74FC: 815D0040  lwz r10, 0x40(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(64 as u32) ) } as u64;
	// 830B7500: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830B7504: 80CB000C  lwz r6, 0xc(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B7508: 809D000C  lwz r4, 0xc(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B750C: 80AA000C  lwz r5, 0xc(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B7510: 48005931  bl 0x830bce40
	ctx.lr = 0x830B7514;
	sub_830BCE40(ctx, base);
	// 830B7514: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830B7518: 48005F41  bl 0x830bd458
	ctx.lr = 0x830B751C;
	sub_830BD458(ctx, base);
	// 830B751C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B7520: 480F0C98  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B7528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B7528 size=112
    let mut pc: u32 = 0x830B7528;
    'dispatch: loop {
        match pc {
            0x830B7528 => {
    //   block [0x830B7528..0x830B7598)
	// 830B7528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B752C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B7530: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B7534: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B7538: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B753C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B7540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B7544: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7548: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B754C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B7550: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7554: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830B7558: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B755C: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B7560: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 830B7564: 419A001C  beq cr6, 0x830b7580
	if ctx.cr[6].eq {
	pc = 0x830B7580; continue 'dispatch;
	}
	// 830B7568: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B756C: 4B208CFD  bl 0x822c0268
	ctx.lr = 0x830B7570;
	sub_822C0268(ctx, base);
	// 830B7570: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7574: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B7578: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B757C: 409AFFEC  bne cr6, 0x830b7568
	if !ctx.cr[6].eq {
	pc = 0x830B7568; continue 'dispatch;
	}
	// 830B7580: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B7584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B7588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B758C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B7590: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B7594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B7598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B7598 size=132
    let mut pc: u32 = 0x830B7598;
    'dispatch: loop {
        match pc {
            0x830B7598 => {
    //   block [0x830B7598..0x830B761C)
	// 830B7598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B759C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B75A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B75A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B75A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B75AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B75B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B75B4: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 830B75B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B75BC: 419A0020  beq cr6, 0x830b75dc
	if ctx.cr[6].eq {
	pc = 0x830B75DC; continue 'dispatch;
	}
	// 830B75C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B75C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B75C8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B75CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B75D0: 4E800421  bctrl
	ctx.lr = 0x830B75D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B75D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B75D8: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 830B75DC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830B75E0: 419A0024  beq cr6, 0x830b7604
	if ctx.cr[6].eq {
	pc = 0x830B7604; continue 'dispatch;
	}
	// 830B75E4: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 830B75E8: 346B0024  addic. r3, r11, 0x24
	ctx.xer.ca = (ctx.r[11].u32 > (!(36 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 36;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 830B75EC: 41820010  beq 0x830b75fc
	if ctx.cr[0].eq {
	pc = 0x830B75FC; continue 'dispatch;
	}
	// 830B75F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B75F4: 4BFFB6AD  bl 0x830b2ca0
	ctx.lr = 0x830B75F8;
	sub_830B2CA0(ctx, base);
	// 830B75F8: 48000008  b 0x830b7600
	pc = 0x830B7600; continue 'dispatch;
	// 830B75FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830B7600: 907F0038  stw r3, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[3].u32 ) };
	// 830B7604: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B7608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B760C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B7610: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B7614: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B7618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B7620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B7620 size=684
    let mut pc: u32 = 0x830B7620;
    'dispatch: loop {
        match pc {
            0x830B7620 => {
    //   block [0x830B7620..0x830B78CC)
	// 830B7620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B7624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B7628: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B762C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B7630: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7634: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B7638: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 830B763C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7640: 2B0A000E  cmplwi cr6, r10, 0xe
	ctx.cr[6].compare_u32(ctx.r[10].u32, 14 as u32, &mut ctx.xer);
	// 830B7644: 41990274  bgt cr6, 0x830b78b8
	if ctx.cr[6].gt {
	pc = 0x830B78B8; continue 'dispatch;
	}
	// 830B7648: 3D808218  lis r12, -0x7de8
	ctx.r[12].s64 = -2112356352;
	// 830B764C: 398C7650  addi r12, r12, 0x7650
	ctx.r[12].s64 = ctx.r[12].s64 + 30288;
	// 830B7650: 7C0C50AE  lbzx r0, r12, r10
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830B7654: 5400103A  slwi r0, r0, 2
	ctx.r[0].u32 = ctx.r[0].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 830B7658: 3D80830B  lis r12, -0x7cf5
	ctx.r[12].s64 = -2096431104;
	// 830B765C: 398C7670  addi r12, r12, 0x7670
	ctx.r[12].s64 = ctx.r[12].s64 + 30320;
	// 830B7660: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 830B7664: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 830B7668: 60000000  nop
	// 830B766C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	// 830B7670: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830B7674: 4BFFA8B5  bl 0x830b1f28
	ctx.lr = 0x830B7678;
	sub_830B1F28(ctx, base);
	// 830B7678: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B767C: 48004905  bl 0x830bbf80
	ctx.lr = 0x830B7680;
	sub_830BBF80(ctx, base);
	// 830B7680: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830B7684: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B7688: 48004859  bl 0x830bbee0
	ctx.lr = 0x830B768C;
	sub_830BBEE0(ctx, base);
	// 830B768C: 4800022C  b 0x830b78b8
	pc = 0x830B78B8; continue 'dispatch;
	// 830B7690: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830B7694: 4BFFA895  bl 0x830b1f28
	ctx.lr = 0x830B7698;
	sub_830B1F28(ctx, base);
	// 830B7698: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830B769C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830B76A0: 808B0034  lwz r4, 0x34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B76A4: 480046CD  bl 0x830bbd70
	ctx.lr = 0x830B76A8;
	sub_830BBD70(ctx, base);
	// 830B76A8: 38810074  addi r4, r1, 0x74
	ctx.r[4].s64 = ctx.r[1].s64 + 116;
	// 830B76AC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830B76B0: 48000AC9  bl 0x830b8178
	ctx.lr = 0x830B76B4;
	sub_830B8178(ctx, base);
	// 830B76B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B76B8: C0210070  lfs f1, 0x70(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B76BC: 4BFFFA15  bl 0x830b70d0
	ctx.lr = 0x830B76C0;
	sub_830B70D0(ctx, base);
	// 830B76C0: 480001F8  b 0x830b78b8
	pc = 0x830B78B8; continue 'dispatch;
	// 830B76C4: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830B76C8: 4BFFA861  bl 0x830b1f28
	ctx.lr = 0x830B76CC;
	sub_830B1F28(ctx, base);
	// 830B76CC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830B76D0: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 830B76D4: 808B0034  lwz r4, 0x34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B76D8: 48004699  bl 0x830bbd70
	ctx.lr = 0x830B76DC;
	sub_830BBD70(ctx, base);
	// 830B76DC: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 830B76E0: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 830B76E4: 48000A95  bl 0x830b8178
	ctx.lr = 0x830B76E8;
	sub_830B8178(ctx, base);
	// 830B76E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B76EC: C021007C  lfs f1, 0x7c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B76F0: 4BFFFA31  bl 0x830b7120
	ctx.lr = 0x830B76F4;
	sub_830B7120(ctx, base);
	// 830B76F4: 480001C4  b 0x830b78b8
	pc = 0x830B78B8; continue 'dispatch;
	// 830B76F8: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830B76FC: 4BFFA82D  bl 0x830b1f28
	ctx.lr = 0x830B7700;
	sub_830B1F28(ctx, base);
	// 830B7700: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B7704: 480048DD  bl 0x830bbfe0
	ctx.lr = 0x830B7708;
	sub_830BBFE0(ctx, base);
	// 830B7708: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B770C: 48004935  bl 0x830bc040
	ctx.lr = 0x830B7710;
	sub_830BC040(ctx, base);
	// 830B7710: 480001A8  b 0x830b78b8
	pc = 0x830B78B8; continue 'dispatch;
	// 830B7714: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830B7718: 4BFFA811  bl 0x830b1f28
	ctx.lr = 0x830B771C;
	sub_830B1F28(ctx, base);
	// 830B771C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830B7720: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 830B7724: 808B0034  lwz r4, 0x34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B7728: 480049B9  bl 0x830bc0e0
	ctx.lr = 0x830B772C;
	sub_830BC0E0(ctx, base);
	// 830B772C: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B7730: C0210080  lfs f1, 0x80(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B7734: 480050DD  bl 0x830bc810
	ctx.lr = 0x830B7738;
	sub_830BC810(ctx, base);
	// 830B7738: 48000180  b 0x830b78b8
	pc = 0x830B78B8; continue 'dispatch;
	// 830B773C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830B7740: 4BFFA7E9  bl 0x830b1f28
	ctx.lr = 0x830B7744;
	sub_830B1F28(ctx, base);
	// 830B7744: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830B7748: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 830B774C: 808B0034  lwz r4, 0x34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B7750: 48004991  bl 0x830bc0e0
	ctx.lr = 0x830B7754;
	sub_830BC0E0(ctx, base);
	// 830B7754: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B7758: C0210084  lfs f1, 0x84(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B775C: 48005155  bl 0x830bc8b0
	ctx.lr = 0x830B7760;
	sub_830BC8B0(ctx, base);
	// 830B7760: 48000158  b 0x830b78b8
	pc = 0x830B78B8; continue 'dispatch;
	// 830B7764: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830B7768: 4BFFA7C1  bl 0x830b1f28
	ctx.lr = 0x830B776C;
	sub_830B1F28(ctx, base);
	// 830B776C: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B7770: 48004BE9  bl 0x830bc358
	ctx.lr = 0x830B7774;
	sub_830BC358(ctx, base);
	// 830B7774: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 830B7778: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B777C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830B7780: 48002561  bl 0x830b9ce0
	ctx.lr = 0x830B7784;
	sub_830B9CE0(ctx, base);
	// 830B7784: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B7788: C0210054  lfs f1, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B778C: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 830B7790: 480024F9  bl 0x830b9c88
	ctx.lr = 0x830B7794;
	sub_830B9C88(ctx, base);
	// 830B7794: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B7798: 48004C21  bl 0x830bc3b8
	ctx.lr = 0x830B779C;
	sub_830BC3B8(ctx, base);
	// 830B779C: 4800011C  b 0x830b78b8
	pc = 0x830B78B8; continue 'dispatch;
	// 830B77A0: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830B77A4: 4BFFA785  bl 0x830b1f28
	ctx.lr = 0x830B77A8;
	sub_830B1F28(ctx, base);
	// 830B77A8: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B77AC: 48004AA5  bl 0x830bc250
	ctx.lr = 0x830B77B0;
	sub_830BC250(ctx, base);
	// 830B77B0: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 830B77B4: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 830B77B8: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B77BC: 48004AF5  bl 0x830bc2b0
	ctx.lr = 0x830B77C0;
	sub_830BC2B0(ctx, base);
	// 830B77C0: 480000F8  b 0x830b78b8
	pc = 0x830B78B8; continue 'dispatch;
	// 830B77C4: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830B77C8: 4BFFA761  bl 0x830b1f28
	ctx.lr = 0x830B77CC;
	sub_830B1F28(ctx, base);
	// 830B77CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B77D0: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B77D4: 48004CBD  bl 0x830bc490
	ctx.lr = 0x830B77D8;
	sub_830BC490(ctx, base);
	// 830B77D8: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 830B77DC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 830B77E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B77E4: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B77E8: 48004D19  bl 0x830bc500
	ctx.lr = 0x830B77EC;
	sub_830BC500(ctx, base);
	// 830B77EC: 480000CC  b 0x830b78b8
	pc = 0x830B78B8; continue 'dispatch;
	// 830B77F0: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830B77F4: 4BFFA735  bl 0x830b1f28
	ctx.lr = 0x830B77F8;
	sub_830B1F28(ctx, base);
	// 830B77F8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830B77FC: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B7800: 48004C91  bl 0x830bc490
	ctx.lr = 0x830B7804;
	sub_830BC490(ctx, base);
	// 830B7804: 90610064  stw r3, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 830B7808: 38A10064  addi r5, r1, 0x64
	ctx.r[5].s64 = ctx.r[1].s64 + 100;
	// 830B780C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830B7810: 4BFFFFD4  b 0x830b77e4
	pc = 0x830B77E4; continue 'dispatch;
	// 830B7814: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830B7818: 4BFFA711  bl 0x830b1f28
	ctx.lr = 0x830B781C;
	sub_830B1F28(ctx, base);
	// 830B781C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 830B7820: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B7824: 48004C6D  bl 0x830bc490
	ctx.lr = 0x830B7828;
	sub_830BC490(ctx, base);
	// 830B7828: 90610068  stw r3, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 830B782C: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 830B7830: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 830B7834: 4BFFFFB0  b 0x830b77e4
	pc = 0x830B77E4; continue 'dispatch;
	// 830B7838: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830B783C: 4BFFA6ED  bl 0x830b1f28
	ctx.lr = 0x830B7840;
	sub_830B1F28(ctx, base);
	// 830B7840: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 830B7844: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B7848: 48004C49  bl 0x830bc490
	ctx.lr = 0x830B784C;
	sub_830BC490(ctx, base);
	// 830B784C: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 830B7850: 38A1006C  addi r5, r1, 0x6c
	ctx.r[5].s64 = ctx.r[1].s64 + 108;
	// 830B7854: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 830B7858: 4BFFFF8C  b 0x830b77e4
	pc = 0x830B77E4; continue 'dispatch;
	// 830B785C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830B7860: 4BFFA6C9  bl 0x830b1f28
	ctx.lr = 0x830B7864;
	sub_830B1F28(ctx, base);
	// 830B7864: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B7868: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B786C: 48004D55  bl 0x830bc5c0
	ctx.lr = 0x830B7870;
	sub_830BC5C0(ctx, base);
	// 830B7870: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B7874: 48000038  b 0x830b78ac
	pc = 0x830B78AC; continue 'dispatch;
	// 830B7878: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830B787C: 4BFFA6AD  bl 0x830b1f28
	ctx.lr = 0x830B7880;
	sub_830B1F28(ctx, base);
	// 830B7880: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830B7884: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B7888: 48004D39  bl 0x830bc5c0
	ctx.lr = 0x830B788C;
	sub_830BC5C0(ctx, base);
	// 830B788C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830B7890: 4800001C  b 0x830b78ac
	pc = 0x830B78AC; continue 'dispatch;
	// 830B7894: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830B7898: 4BFFA691  bl 0x830b1f28
	ctx.lr = 0x830B789C;
	sub_830B1F28(ctx, base);
	// 830B789C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 830B78A0: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B78A4: 48004D1D  bl 0x830bc5c0
	ctx.lr = 0x830B78A8;
	sub_830BC5C0(ctx, base);
	// 830B78A8: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 830B78AC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830B78B0: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B78B4: 48004D7D  bl 0x830bc630
	ctx.lr = 0x830B78B8;
	sub_830BC630(ctx, base);
	// 830B78B8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 830B78BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B78C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B78C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B78C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B78D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B78D0 size=96
    let mut pc: u32 = 0x830B78D0;
    'dispatch: loop {
        match pc {
            0x830B78D0 => {
    //   block [0x830B78D0..0x830B7930)
	// 830B78D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B78D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B78D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B78DC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B78E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B78E4: 815F0058  lwz r10, 0x58(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 830B78E8: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B78EC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830B78F0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830B78F4: 419A0028  beq cr6, 0x830b791c
	if ctx.cr[6].eq {
	pc = 0x830B791C; continue 'dispatch;
	}
	// 830B78F8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B78FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B7900: 4BFFFD21  bl 0x830b7620
	ctx.lr = 0x830B7904;
	sub_830B7620(ctx, base);
	// 830B7904: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B7908: 4B2E9D81  bl 0x823a1688
	ctx.lr = 0x830B790C;
	sub_823A1688(ctx, base);
	// 830B790C: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 830B7910: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B7914: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B7918: 409AFFE0  bne cr6, 0x830b78f8
	if !ctx.cr[6].eq {
	pc = 0x830B78F8; continue 'dispatch;
	}
	// 830B791C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B7920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B7924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B7928: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B792C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B7930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B7930 size=92
    let mut pc: u32 = 0x830B7930;
    'dispatch: loop {
        match pc {
            0x830B7930 => {
    //   block [0x830B7930..0x830B798C)
	// 830B7930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B7934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B7938: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B793C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B7940: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B7944: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B7948: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830B794C: 4BFFA945  bl 0x830b2290
	ctx.lr = 0x830B7950;
	sub_830B2290(ctx, base);
	// 830B7950: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B7954: 41820008  beq 0x830b795c
	if ctx.cr[0].eq {
	pc = 0x830B795C; continue 'dispatch;
	}
	// 830B7958: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 830B795C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B7960: 41820008  beq 0x830b7968
	if ctx.cr[0].eq {
	pc = 0x830B7968; continue 'dispatch;
	}
	// 830B7964: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 830B7968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B796C: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 830B7970: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B7974: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830B7978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B797C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B7980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B7984: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B7988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B7990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B7990 size=568
    let mut pc: u32 = 0x830B7990;
    'dispatch: loop {
        match pc {
            0x830B7990 => {
    //   block [0x830B7990..0x830B7BC8)
	// 830B7990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B7994: 480F07C9  bl 0x831a815c
	ctx.lr = 0x830B7998;
	sub_831A8130(ctx, base);
	// 830B7998: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B799C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B79A0: 3D601555  lis r11, 0x1555
	ctx.r[11].s64 = 357892096;
	// 830B79A4: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 830B79A8: 616B5554  ori r11, r11, 0x5554
	ctx.r[11].u64 = ctx.r[11].u64 | 21844;
	// 830B79AC: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 830B79B0: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B79B4: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 830B79B8: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 830B79BC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B79C0: 41980048  blt cr6, 0x830b7a08
	if ctx.cr[6].lt {
	pc = 0x830B7A08; continue 'dispatch;
	}
	// 830B79C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 830B79C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B79CC: 388B9BCC  addi r4, r11, -0x6434
	ctx.r[4].s64 = ctx.r[11].s64 + -25652;
	// 830B79D0: 4B20DEF9  bl 0x822c58c8
	ctx.lr = 0x830B79D4;
	sub_822C58C8(ctx, base);
	// 830B79D4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B79D8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830B79DC: 4B20DE3D  bl 0x822c5818
	ctx.lr = 0x830B79E0;
	sub_822C5818(ctx, base);
	// 830B79E0: 4B20C8D1  bl 0x822c42b0
	ctx.lr = 0x830B79E4;
	sub_822C42B0(ctx, base);
	// 830B79E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 830B79E8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830B79EC: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 830B79F0: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 830B79F4: 4B20DA7D  bl 0x822c5470
	ctx.lr = 0x830B79F8;
	sub_822C5470(ctx, base);
	// 830B79F8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830B79FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830B7A00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B7A04: 4B20D2DD  bl 0x822c4ce0
	ctx.lr = 0x830B7A08;
	sub_822C4CE0(ctx, base);
	// 830B7A08: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B7A0C: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7A10: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830B7A14: 4B21C2C5  bl 0x822d3cd8
	ctx.lr = 0x830B7A18;
	sub_822D3CD8(ctx, base);
	// 830B7A18: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830B7A1C: 41820020  beq 0x830b7a3c
	if ctx.cr[0].eq {
	pc = 0x830B7A3C; continue 'dispatch;
	}
	// 830B7A20: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830B7A24: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 830B7A28: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 830B7A2C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830B7A30: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830B7A34: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830B7A38: 4BFFF859  bl 0x830b7290
	ctx.lr = 0x830B7A3C;
	sub_830B7290(ctx, base);
	// 830B7A3C: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B7A40: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7A44: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830B7A48: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B7A4C: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 830B7A50: 409A0018  bne cr6, 0x830b7a68
	if !ctx.cr[6].eq {
	pc = 0x830B7A68; continue 'dispatch;
	}
	// 830B7A54: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 830B7A58: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7A5C: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830B7A60: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7A64: 4800003C  b 0x830b7aa0
	pc = 0x830B7AA0; continue 'dispatch;
	// 830B7A68: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B7A6C: 41820020  beq 0x830b7a8c
	if ctx.cr[0].eq {
	pc = 0x830B7A8C; continue 'dispatch;
	}
	// 830B7A70: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830B7A74: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7A78: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7A7C: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830B7A80: 409A0024  bne cr6, 0x830b7aa4
	if !ctx.cr[6].eq {
	pc = 0x830B7AA4; continue 'dispatch;
	}
	// 830B7A84: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830B7A88: 4800001C  b 0x830b7aa4
	pc = 0x830B7AA4; continue 'dispatch;
	// 830B7A8C: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 830B7A90: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7A94: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B7A98: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830B7A9C: 409A0008  bne cr6, 0x830b7aa4
	if !ctx.cr[6].eq {
	pc = 0x830B7AA4; continue 'dispatch;
	}
	// 830B7AA0: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 830B7AA4: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7AA8: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 830B7AAC: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 830B7AB0: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 830B7AB4: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 830B7AB8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B7ABC: 409A00F0  bne cr6, 0x830b7bac
	if !ctx.cr[6].eq {
	pc = 0x830B7BAC; continue 'dispatch;
	}
	// 830B7AC0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 830B7AC4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7AC8: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7ACC: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7AD0: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830B7AD4: 409A0054  bne cr6, 0x830b7b28
	if !ctx.cr[6].eq {
	pc = 0x830B7B28; continue 'dispatch;
	}
	// 830B7AD8: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B7ADC: 892A0018  lbz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 830B7AE0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 830B7AE4: 419A0054  beq cr6, 0x830b7b38
	if ctx.cr[6].eq {
	pc = 0x830B7B38; continue 'dispatch;
	}
	// 830B7AE8: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B7AEC: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B7AF0: 409A0010  bne cr6, 0x830b7b00
	if !ctx.cr[6].eq {
	pc = 0x830B7B00; continue 'dispatch;
	}
	// 830B7AF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B7AF8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830B7AFC: 4B38C46D  bl 0x82443f68
	ctx.lr = 0x830B7B00;
	sub_82443F68(ctx, base);
	// 830B7B00: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7B04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B7B08: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 830B7B0C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7B10: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7B14: 9B6B0018  stb r27, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 830B7B18: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7B1C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7B20: 4B38C3E1  bl 0x82443f00
	ctx.lr = 0x830B7B24;
	sub_82443F00(ctx, base);
	// 830B7B24: 48000074  b 0x830b7b98
	pc = 0x830B7B98; continue 'dispatch;
	// 830B7B28: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7B2C: 892A0018  lbz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 830B7B30: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 830B7B34: 409A0028  bne cr6, 0x830b7b5c
	if !ctx.cr[6].eq {
	pc = 0x830B7B5C; continue 'dispatch;
	}
	// 830B7B38: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7B3C: 9BA90018  stb r29, 0x18(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 830B7B40: 9BAA0018  stb r29, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 830B7B44: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7B48: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7B4C: 9B6A0018  stb r27, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 830B7B50: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7B54: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7B58: 48000040  b 0x830b7b98
	pc = 0x830B7B98; continue 'dispatch;
	// 830B7B5C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7B60: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B7B64: 409A0010  bne cr6, 0x830b7b74
	if !ctx.cr[6].eq {
	pc = 0x830B7B74; continue 'dispatch;
	}
	// 830B7B68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B7B6C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830B7B70: 4B38C391  bl 0x82443f00
	ctx.lr = 0x830B7B74;
	sub_82443F00(ctx, base);
	// 830B7B74: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7B78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B7B7C: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 830B7B80: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7B84: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7B88: 9B6B0018  stb r27, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 830B7B8C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7B90: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7B94: 4B38C3D5  bl 0x82443f68
	ctx.lr = 0x830B7B98;
	sub_82443F68(ctx, base);
	// 830B7B98: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7B9C: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 830B7BA0: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 830B7BA4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B7BA8: 419AFF1C  beq cr6, 0x830b7ac4
	if ctx.cr[6].eq {
	pc = 0x830B7AC4; continue 'dispatch;
	}
	// 830B7BAC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7BB0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 830B7BB4: 93990000  stw r28, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830B7BB8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7BBC: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 830B7BC0: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 830B7BC4: 480F05E8  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B7BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B7BC8 size=168
    let mut pc: u32 = 0x830B7BC8;
    'dispatch: loop {
        match pc {
            0x830B7BC8 => {
    //   block [0x830B7BC8..0x830B7C70)
	// 830B7BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B7BCC: 480F05A1  bl 0x831a816c
	ctx.lr = 0x830B7BD0;
	sub_831A8130(ctx, base);
	// 830B7BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B7BD4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B7BD8: 3BBE0010  addi r29, r30, 0x10
	ctx.r[29].s64 = ctx.r[30].s64 + 16;
	// 830B7BDC: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 830B7BE0: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7BE4: 48000018  b 0x830b7bfc
	pc = 0x830B7BFC; continue 'dispatch;
	// 830B7BE8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B7BEC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B7BF0: 4BFFA739  bl 0x830b2328
	ctx.lr = 0x830B7BF4;
	sub_830B2328(ctx, base);
	// 830B7BF4: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B7BF8: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7BFC: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B7C00: 409AFFE8  bne cr6, 0x830b7be8
	if !ctx.cr[6].eq {
	pc = 0x830B7BE8; continue 'dispatch;
	}
	// 830B7C04: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830B7C08: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7C0C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 830B7C10: 48000050  b 0x830b7c60
	pc = 0x830B7C60; continue 'dispatch;
	// 830B7C14: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 830B7C18: 4BFFA311  bl 0x830b1f28
	ctx.lr = 0x830B7C1C;
	sub_830B1F28(ctx, base);
	// 830B7C1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7C20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B7C24: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7C28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B7C2C: 4E800421  bctrl
	ctx.lr = 0x830B7C30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B7C30: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B7C34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B7C38: 4082001C  bne 0x830b7c54
	if !ctx.cr[0].eq {
	pc = 0x830B7C54; continue 'dispatch;
	}
	// 830B7C3C: 4B2E9A4D  bl 0x823a1688
	ctx.lr = 0x830B7C40;
	sub_823A1688(ctx, base);
	// 830B7C40: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830B7C44: 389E0024  addi r4, r30, 0x24
	ctx.r[4].s64 = ctx.r[30].s64 + 36;
	// 830B7C48: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 830B7C4C: 4BA9395D  bl 0x82b4b5a8
	ctx.lr = 0x830B7C50;
	sub_82B4B5A8(ctx, base);
	// 830B7C50: 48000008  b 0x830b7c58
	pc = 0x830B7C58; continue 'dispatch;
	// 830B7C54: 4B2E9A35  bl 0x823a1688
	ctx.lr = 0x830B7C58;
	sub_823A1688(ctx, base);
	// 830B7C58: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830B7C5C: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B7C60: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B7C64: 409AFFB0  bne cr6, 0x830b7c14
	if !ctx.cr[6].eq {
	pc = 0x830B7C14; continue 'dispatch;
	}
	// 830B7C68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B7C6C: 480F0550  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B7C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B7C70 size=264
    let mut pc: u32 = 0x830B7C70;
    'dispatch: loop {
        match pc {
            0x830B7C70 => {
    //   block [0x830B7C70..0x830B7D78)
	// 830B7C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B7C74: 480F04ED  bl 0x831a8160
	ctx.lr = 0x830B7C78;
	sub_831A8130(ctx, base);
	// 830B7C78: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B7C7C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830B7C80: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 830B7C84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B7C88: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830B7C8C: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 830B7C90: 83DC0004  lwz r30, 4(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7C94: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7C98: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 830B7C9C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B7CA0: 409A0040  bne cr6, 0x830b7ce0
	if !ctx.cr[6].eq {
	pc = 0x830B7CE0; continue 'dispatch;
	}
	// 830B7CA4: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7CA8: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B7CAC: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 830B7CB0: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 830B7CB4: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 830B7CB8: 41980008  blt cr6, 0x830b7cc0
	if ctx.cr[6].lt {
	pc = 0x830B7CC0; continue 'dispatch;
	}
	// 830B7CBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B7CC0: 555D063F  clrlwi. r29, r10, 0x18
	ctx.r[29].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830B7CC4: 4182000C  beq 0x830b7cd0
	if ctx.cr[0].eq {
	pc = 0x830B7CD0; continue 'dispatch;
	}
	// 830B7CC8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7CCC: 48000008  b 0x830b7cd4
	pc = 0x830B7CD4; continue 'dispatch;
	// 830B7CD0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B7CD4: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 830B7CD8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830B7CDC: 419AFFCC  beq cr6, 0x830b7ca8
	if ctx.cr[6].eq {
	pc = 0x830B7CA8; continue 'dispatch;
	}
	// 830B7CE0: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830B7CE4: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B7CE8: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 830B7CEC: 41820048  beq 0x830b7d34
	if ctx.cr[0].eq {
	pc = 0x830B7D34; continue 'dispatch;
	}
	// 830B7CF0: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B7CF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B7CF8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7CFC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B7D00: 409A002C  bne cr6, 0x830b7d2c
	if !ctx.cr[6].eq {
	pc = 0x830B7D2C; continue 'dispatch;
	}
	// 830B7D04: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830B7D08: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830B7D0C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 830B7D10: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 830B7D14: 4BFFFC7D  bl 0x830b7990
	ctx.lr = 0x830B7D18;
	sub_830B7990(ctx, base);
	// 830B7D18: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830B7D1C: 9B5F0004  stb r26, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u8 ) };
	// 830B7D20: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7D24: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B7D28: 48000044  b 0x830b7d6c
	pc = 0x830B7D6C; continue 'dispatch;
	// 830B7D2C: 4B6F20ED  bl 0x827a9e18
	ctx.lr = 0x830B7D30;
	sub_827A9E18(ctx, base);
	// 830B7D30: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B7D34: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B7D38: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7D3C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 830B7D40: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 830B7D44: 41980008  blt cr6, 0x830b7d4c
	if ctx.cr[6].lt {
	pc = 0x830B7D4C; continue 'dispatch;
	}
	// 830B7D48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B7D4C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B7D50: 41820010  beq 0x830b7d60
	if ctx.cr[0].eq {
	pc = 0x830B7D60; continue 'dispatch;
	}
	// 830B7D54: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830B7D58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B7D5C: 4BFFFFAC  b 0x830b7d08
	pc = 0x830B7D08; continue 'dispatch;
	// 830B7D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B7D64: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830B7D68: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 830B7D6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B7D70: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830B7D74: 480F043C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B7D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B7D78 size=104
    let mut pc: u32 = 0x830B7D78;
    'dispatch: loop {
        match pc {
            0x830B7D78 => {
    //   block [0x830B7D78..0x830B7DE0)
	// 830B7D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B7D7C: 480F03F1  bl 0x831a816c
	ctx.lr = 0x830B7D80;
	sub_831A8130(ctx, base);
	// 830B7D80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B7D84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B7D88: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B7D8C: 4BFFF455  bl 0x830b71e0
	ctx.lr = 0x830B7D90;
	sub_830B71E0(ctx, base);
	// 830B7D90: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830B7D94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B7D98: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B7D9C: 4BFFF49D  bl 0x830b7238
	ctx.lr = 0x830B7DA0;
	sub_830B7238(ctx, base);
	// 830B7DA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B7DA4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B7DA8: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B7DAC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 830B7DB0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B7DB4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B7DB8: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 830B7DBC: 4B866D15  bl 0x8291ead0
	ctx.lr = 0x830B7DC0;
	sub_8291EAD0(ctx, base);
	// 830B7DC0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 830B7DC4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830B7DC8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B7DCC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830B7DD0: 4BA93EC1  bl 0x82b4bc90
	ctx.lr = 0x830B7DD4;
	sub_82B4BC90(ctx, base);
	// 830B7DD4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830B7DD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B7DDC: 480F03E0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B7DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B7DE0 size=260
    let mut pc: u32 = 0x830B7DE0;
    'dispatch: loop {
        match pc {
            0x830B7DE0 => {
    //   block [0x830B7DE0..0x830B7EE4)
	// 830B7DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B7DE4: 480F0389  bl 0x831a816c
	ctx.lr = 0x830B7DE8;
	sub_831A8130(ctx, base);
	// 830B7DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B7DEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B7DF0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B7DF4: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830B7DF8: 396B7668  addi r11, r11, 0x7668
	ctx.r[11].s64 = ctx.r[11].s64 + 30312;
	// 830B7DFC: 394A7660  addi r10, r10, 0x7660
	ctx.r[10].s64 = ctx.r[10].s64 + 30304;
	// 830B7E00: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B7E04: 3BBF0010  addi r29, r31, 0x10
	ctx.r[29].s64 = ctx.r[31].s64 + 16;
	// 830B7E08: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 830B7E0C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830B7E10: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7E14: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B7E18: 419A0028  beq cr6, 0x830b7e40
	if ctx.cr[6].eq {
	pc = 0x830B7E40; continue 'dispatch;
	}
	// 830B7E1C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 830B7E20: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B7E24: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830B7E28: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830B7E2C: 4BFFB675  bl 0x830b34a0
	ctx.lr = 0x830B7E30;
	sub_830B34A0(ctx, base);
	// 830B7E30: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B7E34: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7E38: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B7E3C: 409AFFE4  bne cr6, 0x830b7e20
	if !ctx.cr[6].eq {
	pc = 0x830B7E20; continue 'dispatch;
	}
	// 830B7E40: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 830B7E44: 4BFFF6E5  bl 0x830b7528
	ctx.lr = 0x830B7E48;
	sub_830B7528(ctx, base);
	// 830B7E48: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B7E4C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830B7E50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B7E54: 419A001C  beq cr6, 0x830b7e70
	if ctx.cr[6].eq {
	pc = 0x830B7E70; continue 'dispatch;
	}
	// 830B7E58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7E5C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B7E60: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7E64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B7E68: 4E800421  bctrl
	ctx.lr = 0x830B7E6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B7E6C: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 830B7E70: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 830B7E74: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B7E78: 419A001C  beq cr6, 0x830b7e94
	if ctx.cr[6].eq {
	pc = 0x830B7E94; continue 'dispatch;
	}
	// 830B7E7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7E80: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B7E84: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7E88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B7E8C: 4E800421  bctrl
	ctx.lr = 0x830B7E90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B7E90: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 830B7E94: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 830B7E98: 4BFFCDE9  bl 0x830b4c80
	ctx.lr = 0x830B7E9C;
	sub_830B4C80(ctx, base);
	// 830B7E9C: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 830B7EA0: 4BFFC679  bl 0x830b4518
	ctx.lr = 0x830B7EA4;
	sub_830B4518(ctx, base);
	// 830B7EA4: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 830B7EA8: 4BFFA031  bl 0x830b1ed8
	ctx.lr = 0x830B7EAC;
	sub_830B1ED8(ctx, base);
	// 830B7EAC: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 830B7EB0: 4BFFA029  bl 0x830b1ed8
	ctx.lr = 0x830B7EB4;
	sub_830B1ED8(ctx, base);
	// 830B7EB4: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 830B7EB8: 4BFFC661  bl 0x830b4518
	ctx.lr = 0x830B7EBC;
	sub_830B4518(ctx, base);
	// 830B7EBC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B7EC0: 4BFFB691  bl 0x830b3550
	ctx.lr = 0x830B7EC4;
	sub_830B3550(ctx, base);
	// 830B7EC4: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B7EC8: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 830B7ECC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 830B7ED0: 396B74B8  addi r11, r11, 0x74b8
	ctx.r[11].s64 = ctx.r[11].s64 + 29880;
	// 830B7ED4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B7ED8: 4BFFA001  bl 0x830b1ed8
	ctx.lr = 0x830B7EDC;
	sub_830B1ED8(ctx, base);
	// 830B7EDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B7EE0: 480F02DC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B7EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B7EE8 size=8
    let mut pc: u32 = 0x830B7EE8;
    'dispatch: loop {
        match pc {
            0x830B7EE8 => {
    //   block [0x830B7EE8..0x830B7EF0)
	// 830B7EE8: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 830B7EEC: 480000C4  b 0x830b7fb0
	sub_830B7FB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B7EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B7EF0 size=140
    let mut pc: u32 = 0x830B7EF0;
    'dispatch: loop {
        match pc {
            0x830B7EF0 => {
    //   block [0x830B7EF0..0x830B7F7C)
	// 830B7EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B7EF4: 480F0279  bl 0x831a816c
	ctx.lr = 0x830B7EF8;
	sub_831A8130(ctx, base);
	// 830B7EF8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B7EFC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B7F00: 3BE30054  addi r31, r3, 0x54
	ctx.r[31].s64 = ctx.r[3].s64 + 84;
	// 830B7F04: 93C100BC  stw r30, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[30].u32 ) };
	// 830B7F08: 388100BC  addi r4, r1, 0xbc
	ctx.r[4].s64 = ctx.r[1].s64 + 188;
	// 830B7F0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B7F10: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830B7F14: 4BFFFE65  bl 0x830b7d78
	ctx.lr = 0x830B7F18;
	sub_830B7D78(ctx, base);
	// 830B7F18: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830B7F1C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830B7F20: 4B418D81  bl 0x824d0ca0
	ctx.lr = 0x830B7F24;
	sub_824D0CA0(ctx, base);
	// 830B7F24: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 830B7F28: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B7F2C: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 830B7F30: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B7F34: 4B418D6D  bl 0x824d0ca0
	ctx.lr = 0x830B7F38;
	sub_824D0CA0(ctx, base);
	// 830B7F38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B7F3C: 4BFF9F9D  bl 0x830b1ed8
	ctx.lr = 0x830B7F40;
	sub_830B1ED8(ctx, base);
	// 830B7F40: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 830B7F44: 38810064  addi r4, r1, 0x64
	ctx.r[4].s64 = ctx.r[1].s64 + 100;
	// 830B7F48: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 830B7F4C: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 830B7F50: 4B418D51  bl 0x824d0ca0
	ctx.lr = 0x830B7F54;
	sub_824D0CA0(ctx, base);
	// 830B7F54: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 830B7F58: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B7F5C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 830B7F60: 4BFFFD11  bl 0x830b7c70
	ctx.lr = 0x830B7F64;
	sub_830B7C70(ctx, base);
	// 830B7F64: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 830B7F68: 4BFF9F71  bl 0x830b1ed8
	ctx.lr = 0x830B7F6C;
	sub_830B1ED8(ctx, base);
	// 830B7F6C: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 830B7F70: 4BFF9F69  bl 0x830b1ed8
	ctx.lr = 0x830B7F74;
	sub_830B1ED8(ctx, base);
	// 830B7F74: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 830B7F78: 480F0244  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B7F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B7F80 size=44
    let mut pc: u32 = 0x830B7F80;
    'dispatch: loop {
        match pc {
            0x830B7F80 => {
    //   block [0x830B7F80..0x830B7FAC)
	// 830B7F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B7F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B7F88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B7F8C: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 830B7F90: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 830B7F94: 38630054  addi r3, r3, 0x54
	ctx.r[3].s64 = ctx.r[3].s64 + 84;
	// 830B7F98: 4BFFFDE1  bl 0x830b7d78
	ctx.lr = 0x830B7F9C;
	sub_830B7D78(ctx, base);
	// 830B7F9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B7FA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B7FA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B7FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B7FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B7FB0 size=92
    let mut pc: u32 = 0x830B7FB0;
    'dispatch: loop {
        match pc {
            0x830B7FB0 => {
    //   block [0x830B7FB0..0x830B800C)
	// 830B7FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B7FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B7FB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B7FBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B7FC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B7FC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B7FC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B7FCC: 4BFFFE15  bl 0x830b7de0
	ctx.lr = 0x830B7FD0;
	sub_830B7DE0(ctx, base);
	// 830B7FD0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B7FD4: 4182001C  beq 0x830b7ff0
	if ctx.cr[0].eq {
	pc = 0x830B7FF0; continue 'dispatch;
	}
	// 830B7FD8: 4BFFCC71  bl 0x830b4c48
	ctx.lr = 0x830B7FDC;
	sub_830B4C48(ctx, base);
	// 830B7FDC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B7FE0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B7FE4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830B7FE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B7FEC: 4E800421  bctrl
	ctx.lr = 0x830B7FF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B7FF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B7FF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B7FF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B7FFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B8000: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B8004: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B8008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B8010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830B8010 size=356
    let mut pc: u32 = 0x830B8010;
    'dispatch: loop {
        match pc {
            0x830B8010 => {
    //   block [0x830B8010..0x830B8174)
	// 830B8010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B8014: 480F013D  bl 0x831a8150
	ctx.lr = 0x830B8018;
	sub_831A8130(ctx, base);
	// 830B8018: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B801C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B8020: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 830B8024: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 830B8028: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 830B802C: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 830B8030: 7D174378  mr r23, r8
	ctx.r[23].u64 = ctx.r[8].u64;
	// 830B8034: 7D3C4B78  mr r28, r9
	ctx.r[28].u64 = ctx.r[9].u64;
	// 830B8038: 4BFFA951  bl 0x830b2988
	ctx.lr = 0x830B803C;
	sub_830B2988(ctx, base);
	// 830B803C: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830B8040: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 830B8044: 394A7550  addi r10, r10, 0x7550
	ctx.r[10].s64 = ctx.r[10].s64 + 30032;
	// 830B8048: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830B804C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 830B8050: 4BFFF8E1  bl 0x830b7930
	ctx.lr = 0x830B8054;
	sub_830B7930(ctx, base);
	// 830B8054: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B8058: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830B805C: 396B7668  addi r11, r11, 0x7668
	ctx.r[11].s64 = ctx.r[11].s64 + 30312;
	// 830B8060: 394A7660  addi r10, r10, 0x7660
	ctx.r[10].s64 = ctx.r[10].s64 + 30304;
	// 830B8064: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B8068: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 830B806C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 830B8070: 4B366799  bl 0x8241e808
	ctx.lr = 0x830B8074;
	sub_8241E808(ctx, base);
	// 830B8074: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830B8078: 3BDF0044  addi r30, r31, 0x44
	ctx.r[30].s64 = ctx.r[31].s64 + 68;
	// 830B807C: 93BF0030  stw r29, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 830B8080: 93BF0038  stw r29, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 830B8084: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B8088: 4BFF9E39  bl 0x830b1ec0
	ctx.lr = 0x830B808C;
	sub_830B1EC0(ctx, base);
	// 830B808C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B8090: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B8094: 3ACB74C0  addi r22, r11, 0x74c0
	ctx.r[22].s64 = ctx.r[11].s64 + 29888;
	// 830B8098: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B809C: 92DF0044  stw r22, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[22].u32 ) };
	// 830B80A0: 4BFF9EF9  bl 0x830b1f98
	ctx.lr = 0x830B80A4;
	sub_830B1F98(ctx, base);
	// 830B80A4: 3BDF004C  addi r30, r31, 0x4c
	ctx.r[30].s64 = ctx.r[31].s64 + 76;
	// 830B80A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B80AC: 4BFF9E15  bl 0x830b1ec0
	ctx.lr = 0x830B80B0;
	sub_830B1EC0(ctx, base);
	// 830B80B0: 92DF004C  stw r22, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[22].u32 ) };
	// 830B80B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B80B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B80BC: 4BFF9EDD  bl 0x830b1f98
	ctx.lr = 0x830B80C0;
	sub_830B1F98(ctx, base);
	// 830B80C0: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 830B80C4: 4B366745  bl 0x8241e808
	ctx.lr = 0x830B80C8;
	sub_8241E808(ctx, base);
	// 830B80C8: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 830B80CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B80D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B80D4: 4BFF9FDD  bl 0x830b20b0
	ctx.lr = 0x830B80D8;
	sub_830B20B0(ctx, base);
	// 830B80D8: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 830B80DC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830B80E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B80E4: 4BFF9F45  bl 0x830b2028
	ctx.lr = 0x830B80E8;
	sub_830B2028(ctx, base);
	// 830B80E8: 937F000C  stw r27, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 830B80EC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B80F0: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 830B80F4: 388B7628  addi r4, r11, 0x7628
	ctx.r[4].s64 = ctx.r[11].s64 + 30248;
	// 830B80F8: 38600038  li r3, 0x38
	ctx.r[3].s64 = 56;
	// 830B80FC: 4BFFCB5D  bl 0x830b4c58
	ctx.lr = 0x830B8100;
	sub_830B4C58(ctx, base);
	// 830B8100: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830B8104: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 830B8108: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 830B810C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 830B8110: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 830B8114: 4800133D  bl 0x830b9450
	ctx.lr = 0x830B8118;
	sub_830B9450(ctx, base);
	// 830B8118: 907F0034  stw r3, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[3].u32 ) };
	// 830B811C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 830B8120: 4BFFA579  bl 0x830b2698
	ctx.lr = 0x830B8124;
	sub_830B2698(ctx, base);
	// 830B8124: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B8128: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 830B812C: D02B0018  stfs f1, 0x18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 830B8130: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B8134: 93EB0014  stw r31, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 830B8138: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B813C: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 830B8140: 92FF0040  stw r23, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[23].u32 ) };
	// 830B8144: 419A0024  beq cr6, 0x830b8168
	if ctx.cr[6].eq {
	pc = 0x830B8168; continue 'dispatch;
	}
	// 830B8148: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 830B814C: 346B0024  addic. r3, r11, 0x24
	ctx.xer.ca = (ctx.r[11].u32 > (!(36 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 36;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 830B8150: 41820010  beq 0x830b8160
	if ctx.cr[0].eq {
	pc = 0x830B8160; continue 'dispatch;
	}
	// 830B8154: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830B8158: 4BFFAB49  bl 0x830b2ca0
	ctx.lr = 0x830B815C;
	sub_830B2CA0(ctx, base);
	// 830B815C: 48000008  b 0x830b8164
	pc = 0x830B8164; continue 'dispatch;
	// 830B8160: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B8164: 907F0038  stw r3, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[3].u32 ) };
	// 830B8168: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B816C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 830B8170: 480F0030  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B8178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B8178 size=60
    let mut pc: u32 = 0x830B8178;
    'dispatch: loop {
        match pc {
            0x830B8178 => {
    //   block [0x830B8178..0x830B81B4)
	// 830B8178: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830B817C: C1830000  lfs f12, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 830B8180: 394BBCA0  addi r10, r11, -0x4360
	ctx.r[10].s64 = ctx.r[11].s64 + -17248;
	// 830B8184: C1ABBCA0  lfs f13, -0x4360(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17248 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B8188: C00A0010  lfs f0, 0x10(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B818C: EC0C002A  fadds f0, f12, f0
	ctx.f[0].f64 = ((ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64;
	// 830B8190: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 830B8194: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830B8198: C00A0014  lfs f0, 0x14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B819C: C1A40000  lfs f13, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B81A0: EDAD002A  fadds f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 830B81A4: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B81A8: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 830B81AC: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830B81B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B81B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B81B8 size=60
    let mut pc: u32 = 0x830B81B8;
    'dispatch: loop {
        match pc {
            0x830B81B8 => {
    //   block [0x830B81B8..0x830B81F4)
	// 830B81B8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830B81BC: C1830000  lfs f12, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 830B81C0: 394BBCBC  addi r10, r11, -0x4344
	ctx.r[10].s64 = ctx.r[11].s64 + -17220;
	// 830B81C4: C1ABBCBC  lfs f13, -0x4344(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17220 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B81C8: C00A0010  lfs f0, 0x10(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B81CC: EC0C002A  fadds f0, f12, f0
	ctx.f[0].f64 = ((ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64;
	// 830B81D0: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 830B81D4: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830B81D8: C00A0014  lfs f0, 0x14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B81DC: C1A40000  lfs f13, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B81E0: EDAD002A  fadds f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 830B81E4: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B81E8: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 830B81EC: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830B81F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B81F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B81F8 size=76
    let mut pc: u32 = 0x830B81F8;
    'dispatch: loop {
        match pc {
            0x830B81F8 => {
    //   block [0x830B81F8..0x830B8244)
	// 830B81F8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830B81FC: C1A30000  lfs f13, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B8200: 396BBCA0  addi r11, r11, -0x4360
	ctx.r[11].s64 = ctx.r[11].s64 + -17248;
	// 830B8204: C00B0008  lfs f0, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B8208: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 830B820C: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830B8210: C1A40000  lfs f13, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B8214: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B8218: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 830B821C: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830B8220: C1A30000  lfs f13, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B8224: C00B0010  lfs f0, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B8228: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 830B822C: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830B8230: C00B0014  lfs f0, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B8234: C1A40000  lfs f13, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B8238: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 830B823C: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830B8240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B8248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B8248 size=76
    let mut pc: u32 = 0x830B8248;
    'dispatch: loop {
        match pc {
            0x830B8248 => {
    //   block [0x830B8248..0x830B8294)
	// 830B8248: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830B824C: C1A30000  lfs f13, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B8250: 396BBCBC  addi r11, r11, -0x4344
	ctx.r[11].s64 = ctx.r[11].s64 + -17220;
	// 830B8254: C00B0008  lfs f0, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B8258: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 830B825C: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830B8260: C1A40000  lfs f13, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B8264: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B8268: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 830B826C: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830B8270: C1A30000  lfs f13, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B8274: C00B0010  lfs f0, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B8278: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 830B827C: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830B8280: C00B0014  lfs f0, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B8284: C1A40000  lfs f13, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B8288: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 830B828C: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830B8290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B8298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B8298 size=28
    let mut pc: u32 = 0x830B8298;
    'dispatch: loop {
        match pc {
            0x830B8298 => {
    //   block [0x830B8298..0x830B82B4)
	// 830B8298: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830B829C: 394BBCA0  addi r10, r11, -0x4360
	ctx.r[10].s64 = ctx.r[11].s64 + -17248;
	// 830B82A0: C00BBCA0  lfs f0, -0x4360(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17248 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B82A4: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830B82A8: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B82AC: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830B82B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B82B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B82B8 size=28
    let mut pc: u32 = 0x830B82B8;
    'dispatch: loop {
        match pc {
            0x830B82B8 => {
    //   block [0x830B82B8..0x830B82D4)
	// 830B82B8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830B82BC: 394BBCBC  addi r10, r11, -0x4344
	ctx.r[10].s64 = ctx.r[11].s64 + -17220;
	// 830B82C0: C00BBCBC  lfs f0, -0x4344(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17220 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B82C4: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830B82C8: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B82CC: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830B82D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B82D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B82D8 size=28
    let mut pc: u32 = 0x830B82D8;
    'dispatch: loop {
        match pc {
            0x830B82D8 => {
    //   block [0x830B82D8..0x830B82F4)
	// 830B82D8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830B82DC: 396BBCA0  addi r11, r11, -0x4360
	ctx.r[11].s64 = ctx.r[11].s64 + -17248;
	// 830B82E0: C00B0010  lfs f0, 0x10(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B82E4: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830B82E8: C00B0014  lfs f0, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B82EC: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830B82F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B82F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B82F8 size=44
    let mut pc: u32 = 0x830B82F8;
    'dispatch: loop {
        match pc {
            0x830B82F8 => {
    //   block [0x830B82F8..0x830B8324)
	// 830B82F8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830B82FC: C1A30000  lfs f13, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B8300: 394BBCA0  addi r10, r11, -0x4360
	ctx.r[10].s64 = ctx.r[11].s64 + -17248;
	// 830B8304: C00BBCA0  lfs f0, -0x4360(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17248 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B8308: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 830B830C: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830B8310: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830B8314: C1A40000  lfs f13, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830B8318: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 830B831C: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830B8320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B8328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B8328 size=16
    let mut pc: u32 = 0x830B8328;
    'dispatch: loop {
        match pc {
            0x830B8328 => {
    //   block [0x830B8328..0x830B8338)
	// 830B8328: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830B832C: 396BBCBC  addi r11, r11, -0x4344
	ctx.r[11].s64 = ctx.r[11].s64 + -17220;
	// 830B8330: C02B0018  lfs f1, 0x18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830B8334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B8338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830B8338 size=20
    let mut pc: u32 = 0x830B8338;
    'dispatch: loop {
        match pc {
            0x830B8338 => {
    //   block [0x830B8338..0x830B834C)
	// 830B8338: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830B833C: 394B4420  addi r10, r11, 0x4420
	ctx.r[10].s64 = ctx.r[11].s64 + 17440;
	// 830B8340: D02B4420  stfs f1, 0x4420(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(17440 as u32), tmp.u32 ) };
	// 830B8344: D04A0004  stfs f2, 4(r10)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 830B8348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


