pub fn sub_825A9990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A9990 size=108
    let mut pc: u32 = 0x825A9990;
    'dispatch: loop {
        match pc {
            0x825A9990 => {
    //   block [0x825A9990..0x825A99FC)
	// 825A9990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A9994: 48BFE7D5  bl 0x831a8168
	ctx.lr = 0x825A9998;
	sub_831A8130(ctx, base);
	// 825A9998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A999C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A99A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A99A4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A99A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A99AC: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 825A99B0: 4884A059  bl 0x82df3a08
	ctx.lr = 0x825A99B4;
	sub_82DF3A08(ctx, base);
	// 825A99B4: 3BBE0038  addi r29, r30, 0x38
	ctx.r[29].s64 = ctx.r[30].s64 + 56;
	// 825A99B8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A99BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A99C0: 48849949  bl 0x82df3308
	ctx.lr = 0x825A99C4;
	sub_82DF3308(ctx, base);
	// 825A99C4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825A99C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A99CC: 48849A5D  bl 0x82df3428
	ctx.lr = 0x825A99D0;
	sub_82DF3428(ctx, base);
	// 825A99D0: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A99D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A99D8: 41820010  beq 0x825a99e8
	if ctx.cr[0].eq {
	pc = 0x825A99E8; continue 'dispatch;
	}
	// 825A99DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A99E0: 4BFFCAA9  bl 0x825a6488
	ctx.lr = 0x825A99E4;
	sub_825A6488(ctx, base);
	// 825A99E4: 4800000C  b 0x825a99f0
	pc = 0x825A99F0; continue 'dispatch;
	// 825A99E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825A99EC: 4884A215  bl 0x82df3c00
	ctx.lr = 0x825A99F0;
	sub_82DF3C00(ctx, base);
	// 825A99F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A99F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A99F8: 48BFE7C0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A9A00 size=124
    let mut pc: u32 = 0x825A9A00;
    'dispatch: loop {
        match pc {
            0x825A9A00 => {
    //   block [0x825A9A00..0x825A9A7C)
	// 825A9A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A9A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A9A08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A9A0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A9A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A9A14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A9A18: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825A9A1C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A9A20: 48030371  bl 0x825d9d90
	ctx.lr = 0x825A9A24;
	sub_825D9D90(ctx, base);
	// 825A9A24: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A9A28: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825A9A2C: 388BB300  addi r4, r11, -0x4d00
	ctx.r[4].s64 = ctx.r[11].s64 + -19712;
	// 825A9A30: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825A9A34: 4803041D  bl 0x825d9e50
	ctx.lr = 0x825A9A38;
	sub_825D9E50(ctx, base);
	// 825A9A38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A9A3C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825A9A40: 480303C9  bl 0x825d9e08
	ctx.lr = 0x825A9A44;
	sub_825D9E08(ctx, base);
	// 825A9A44: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A9A48: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A9A4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A9A50: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 825A9A54: 419A0008  beq cr6, 0x825a9a5c
	if ctx.cr[6].eq {
	pc = 0x825A9A5C; continue 'dispatch;
	}
	// 825A9A58: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A9A5C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825A9A60: 48030369  bl 0x825d9dc8
	ctx.lr = 0x825A9A64;
	sub_825D9DC8(ctx, base);
	// 825A9A64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A9A68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A9A6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A9A70: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A9A74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A9A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A9A80 size=116
    let mut pc: u32 = 0x825A9A80;
    'dispatch: loop {
        match pc {
            0x825A9A80 => {
    //   block [0x825A9A80..0x825A9AF4)
	// 825A9A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A9A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A9A88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A9A8C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A9A90: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A9A94: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825A9A98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A9A9C: 419A000C  beq cr6, 0x825a9aa8
	if ctx.cr[6].eq {
	pc = 0x825A9AA8; continue 'dispatch;
	}
	// 825A9AA0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9AA4: 48000008  b 0x825a9aac
	pc = 0x825A9AAC; continue 'dispatch;
	// 825A9AA8: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A9AAC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A9AB0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825A9AB4: 480302DD  bl 0x825d9d90
	ctx.lr = 0x825A9AB8;
	sub_825D9D90(ctx, base);
	// 825A9AB8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A9ABC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825A9AC0: 388BB300  addi r4, r11, -0x4d00
	ctx.r[4].s64 = ctx.r[11].s64 + -19712;
	// 825A9AC4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825A9AC8: 48030389  bl 0x825d9e50
	ctx.lr = 0x825A9ACC;
	sub_825D9E50(ctx, base);
	// 825A9ACC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A9AD0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825A9AD4: 4803032D  bl 0x825d9e00
	ctx.lr = 0x825A9AD8;
	sub_825D9E00(ctx, base);
	// 825A9AD8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825A9ADC: 480302ED  bl 0x825d9dc8
	ctx.lr = 0x825A9AE0;
	sub_825D9DC8(ctx, base);
	// 825A9AE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A9AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A9AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A9AEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A9AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A9AF8 size=136
    let mut pc: u32 = 0x825A9AF8;
    'dispatch: loop {
        match pc {
            0x825A9AF8 => {
    //   block [0x825A9AF8..0x825A9B80)
	// 825A9AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A9AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A9B00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A9B04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A9B08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A9B0C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A9B10: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825A9B14: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 825A9B18: 409A0020  bne cr6, 0x825a9b38
	if !ctx.cr[6].eq {
	pc = 0x825A9B38; continue 'dispatch;
	}
	// 825A9B1C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825A9B20: 419A0048  beq cr6, 0x825a9b68
	if ctx.cr[6].eq {
	pc = 0x825A9B68; continue 'dispatch;
	}
	// 825A9B24: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9B28: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A9B2C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A9B30: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A9B34: 48000034  b 0x825a9b68
	pc = 0x825A9B68; continue 'dispatch;
	// 825A9B38: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 825A9B3C: 419A002C  beq cr6, 0x825a9b68
	if ctx.cr[6].eq {
	pc = 0x825A9B68; continue 'dispatch;
	}
	// 825A9B40: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825A9B44: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9B48: 388B8360  addi r4, r11, -0x7ca0
	ctx.r[4].s64 = ctx.r[11].s64 + -31904;
	// 825A9B4C: 48BFE5AD  bl 0x831a80f8
	ctx.lr = 0x825A9B50;
	sub_831A80F8(ctx, base);
	// 825A9B50: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A9B54: 4182000C  beq 0x825a9b60
	if ctx.cr[0].eq {
	pc = 0x825A9B60; continue 'dispatch;
	}
	// 825A9B58: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 825A9B5C: 4800000C  b 0x825a9b68
	pc = 0x825A9B68; continue 'dispatch;
	// 825A9B60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A9B64: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A9B68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A9B6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A9B70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A9B74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A9B78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A9B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A9B80 size=116
    let mut pc: u32 = 0x825A9B80;
    'dispatch: loop {
        match pc {
            0x825A9B80 => {
    //   block [0x825A9B80..0x825A9BF4)
	// 825A9B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A9B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A9B88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A9B8C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A9B90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A9B94: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A9B98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A9B9C: 419A000C  beq cr6, 0x825a9ba8
	if ctx.cr[6].eq {
	pc = 0x825A9BA8; continue 'dispatch;
	}
	// 825A9BA0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9BA4: 48000008  b 0x825a9bac
	pc = 0x825A9BAC; continue 'dispatch;
	// 825A9BA8: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A9BAC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825A9BB0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825A9BB4: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9BB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A9BBC: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9BC0: 816A0060  lwz r11, 0x60(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(96 as u32) ) } as u64;
	// 825A9BC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A9BC8: 4E800421  bctrl
	ctx.lr = 0x825A9BCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A9BCC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A9BD0: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 825A9BD4: 48849FFD  bl 0x82df3bd0
	ctx.lr = 0x825A9BD8;
	sub_82DF3BD0(ctx, base);
	// 825A9BD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A9BDC: 4884984D  bl 0x82df3428
	ctx.lr = 0x825A9BE0;
	sub_82DF3428(ctx, base);
	// 825A9BE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A9BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A9BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A9BEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A9BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A9BF8 size=168
    let mut pc: u32 = 0x825A9BF8;
    'dispatch: loop {
        match pc {
            0x825A9BF8 => {
    //   block [0x825A9BF8..0x825A9CA0)
	// 825A9BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A9BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A9C00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A9C04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A9C08: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A9C0C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825A9C10: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A9C14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A9C18: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9C1C: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 825A9C20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A9C24: 4E800421  bctrl
	ctx.lr = 0x825A9C28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A9C28: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9C2C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A9C30: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A9C34: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 825A9C38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A9C3C: 4E800421  bctrl
	ctx.lr = 0x825A9C40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A9C40: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 825A9C44: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A9CA0 size=456
    let mut pc: u32 = 0x825A9CA0;
    'dispatch: loop {
        match pc {
            0x825A9CA0 => {
    //   block [0x825A9CA0..0x825A9E68)
	// 825A9CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A9CA4: 48BFE4C9  bl 0x831a816c
	ctx.lr = 0x825A9CA8;
	sub_831A8130(ctx, base);
	// 825A9CA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A9CAC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825A9CB0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A9CB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A9CB8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 825A9CBC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9CC0: 816B0074  lwz r11, 0x74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 825A9CC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A9CC8: 4E800421  bctrl
	ctx.lr = 0x825A9CCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A9CCC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9CD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A9CD4: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A9CD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A9CDC: 4E800421  bctrl
	ctx.lr = 0x825A9CE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A9CE0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A9CE4: 41820110  beq 0x825a9df4
	if ctx.cr[0].eq {
	pc = 0x825A9DF4; continue 'dispatch;
	}
	// 825A9CE8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9CEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A9CF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A9CF4: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 825A9CF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A9CFC: 4E800421  bctrl
	ctx.lr = 0x825A9D00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A9D00: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 825A9D04: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A9D08: 816B853C  lwz r11, -0x7ac4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31428 as u32) ) } as u64;
	// 825A9D0C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825A9D10: 419A00E4  beq cr6, 0x825a9df4
	if ctx.cr[6].eq {
	pc = 0x825A9DF4; continue 'dispatch;
	}
	// 825A9D14: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9D18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A9D1C: 816B007C  lwz r11, 0x7c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 825A9D20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A9D24: 4E800421  bctrl
	ctx.lr = 0x825A9D28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A9D28: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A9D2C: 408200C8  bne 0x825a9df4
	if !ctx.cr[0].eq {
	pc = 0x825A9DF4; continue 'dispatch;
	}
	// 825A9D30: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A9D34: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A9D38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A9D3C: 915E000C  stw r10, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 825A9D40: 419A0008  beq cr6, 0x825a9d48
	if ctx.cr[6].eq {
	pc = 0x825A9D48; continue 'dispatch;
	}
	// 825A9D44: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A9D48: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9D4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A9D50: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825A9D54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A9D58: 4E800421  bctrl
	ctx.lr = 0x825A9D5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A9D5C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9D60: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A9D64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A9D68: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A9D6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A9D70: 4E800421  bctrl
	ctx.lr = 0x825A9D74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A9D74: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9D78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A9D7C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825A9D80: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 825A9D84: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9D88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A9D8C: 4E800421  bctrl
	ctx.lr = 0x825A9D90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A9D90: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A9D94: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A9D98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A9D9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A9DA0: 4E800421  bctrl
	ctx.lr = 0x825A9DA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A9DA4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9DA8: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 825A9DAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A9DB0: 816B0074  lwz r11, 0x74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 825A9DB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A9DB8: 4E800421  bctrl
	ctx.lr = 0x825A9DBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A9DBC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9DC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A9DC4: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A9DC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A9DCC: 4E800421  bctrl
	ctx.lr = 0x825A9DD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A9DD0: 817E0050  lwz r11, 0x50(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A9DD4: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A9DD8: 4182001C  beq 0x825a9df4
	if ctx.cr[0].eq {
	pc = 0x825A9DF4; continue 'dispatch;
	}
	// 825A9DDC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9DE0: 389E0040  addi r4, r30, 0x40
	ctx.r[4].s64 = ctx.r[30].s64 + 64;
	// 825A9DE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A9DE8: 816B0068  lwz r11, 0x68(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 825A9DEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A9DF0: 4E800421  bctrl
	ctx.lr = 0x825A9DF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A9DF4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9DF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A9DFC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 825A9E00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A9E04: 4E800421  bctrl
	ctx.lr = 0x825A9E08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A9E08: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A9E0C: 41820054  beq 0x825a9e60
	if ctx.cr[0].eq {
	pc = 0x825A9E60; continue 'dispatch;
	}
	// 825A9E10: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9E14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A9E18: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A9E1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A9E20: 4E800421  bctrl
	ctx.lr = 0x825A9E24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A9E24: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9E28: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 825A9E2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A9E30: 816B0074  lwz r11, 0x74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 825A9E34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A9E38: 4E800421  bctrl
	ctx.lr = 0x825A9E3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A9E3C: 817E0050  lwz r11, 0x50(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A9E40: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A9E44: 4182001C  beq 0x825a9e60
	if ctx.cr[0].eq {
	pc = 0x825A9E60; continue 'dispatch;
	}
	// 825A9E48: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9E4C: 389E0040  addi r4, r30, 0x40
	ctx.r[4].s64 = ctx.r[30].s64 + 64;
	// 825A9E50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A9E54: 816B0068  lwz r11, 0x68(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 825A9E58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A9E5C: 4E800421  bctrl
	ctx.lr = 0x825A9E60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A9E60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A9E64: 48BFE358  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A9E68 size=100
    let mut pc: u32 = 0x825A9E68;
    'dispatch: loop {
        match pc {
            0x825A9E68 => {
    //   block [0x825A9E68..0x825A9ECC)
	// 825A9E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A9E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A9E70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A9E74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A9E78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A9E7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A9E80: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A9E84: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 825A9E88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A9E8C: 419A0008  beq cr6, 0x825a9e94
	if ctx.cr[6].eq {
	pc = 0x825A9E94; continue 'dispatch;
	}
	// 825A9E90: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A9E94: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A9E98: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 825A9E9C: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 825A9EA0: 488499D9  bl 0x82df3878
	ctx.lr = 0x825A9EA4;
	sub_82DF3878(ctx, base);
	// 825A9EA4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9EA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A9EAC: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825A9EB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A9EB4: 4E800421  bctrl
	ctx.lr = 0x825A9EB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A9EB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A9EBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A9EC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A9EC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A9EC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A9ED0 size=20
    let mut pc: u32 = 0x825A9ED0;
    'dispatch: loop {
        match pc {
            0x825A9ED0 => {
    //   block [0x825A9ED0..0x825A9EE4)
	// 825A9ED0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A9ED4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A9ED8: 419A000C  beq cr6, 0x825a9ee4
	if ctx.cr[6].eq {
		sub_825A9EE4(ctx, base);
		return;
	}
	// 825A9EDC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9EE0: 48000008  b 0x825a9ee8
	sub_825A9EE4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9EE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A9EE4 size=12
    let mut pc: u32 = 0x825A9EE4;
    'dispatch: loop {
        match pc {
            0x825A9EE4 => {
    //   block [0x825A9EE4..0x825A9EF0)
	// 825A9EE4: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A9EE8: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 825A9EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A9EF0 size=144
    let mut pc: u32 = 0x825A9EF0;
    'dispatch: loop {
        match pc {
            0x825A9EF0 => {
    //   block [0x825A9EF0..0x825A9F80)
	// 825A9EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A9EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A9EF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A9EFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A9F00: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825A9F04: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 825A9F08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A9F0C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825A9F10: 38CB8400  addi r6, r11, -0x7c00
	ctx.r[6].s64 = ctx.r[11].s64 + -31744;
	// 825A9F14: 38AA7B58  addi r5, r10, 0x7b58
	ctx.r[5].s64 = ctx.r[10].s64 + 31576;
	// 825A9F18: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825A9F1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825A9F20: 48C00029  bl 0x831a9f48
	ctx.lr = 0x825A9F24;
	sub_831A9F48(ctx, base);
	// 825A9F24: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A9F28: 41820044  beq 0x825a9f6c
	if ctx.cr[0].eq {
	pc = 0x825A9F6C; continue 'dispatch;
	}
	// 825A9F2C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A9F30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A9F34: 419A000C  beq cr6, 0x825a9f40
	if ctx.cr[6].eq {
	pc = 0x825A9F40; continue 'dispatch;
	}
	// 825A9F38: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9F3C: 48000008  b 0x825a9f44
	pc = 0x825A9F44; continue 'dispatch;
	// 825A9F40: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A9F44: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A9F48: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 825A9F4C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A9F50: 419A0008  beq cr6, 0x825a9f58
	if ctx.cr[6].eq {
	pc = 0x825A9F58; continue 'dispatch;
	}
	// 825A9F54: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A9F58: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9F5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A9F60: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825A9F64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A9F68: 4E800421  bctrl
	ctx.lr = 0x825A9F6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A9F6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A9F70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A9F74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A9F78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A9F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A9F80 size=20
    let mut pc: u32 = 0x825A9F80;
    'dispatch: loop {
        match pc {
            0x825A9F80 => {
    //   block [0x825A9F80..0x825A9F94)
	// 825A9F80: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A9F84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A9F88: 419A000C  beq cr6, 0x825a9f94
	if ctx.cr[6].eq {
		sub_825A9F94(ctx, base);
		return;
	}
	// 825A9F8C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9F90: 48000008  b 0x825a9f98
	sub_825A9F94(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9F94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A9F94 size=12
    let mut pc: u32 = 0x825A9F94;
    'dispatch: loop {
        match pc {
            0x825A9F94 => {
    //   block [0x825A9F94..0x825A9FA0)
	// 825A9F94: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A9F98: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 825A9F9C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A9FA0 size=12
    let mut pc: u32 = 0x825A9FA0;
    'dispatch: loop {
        match pc {
            0x825A9FA0 => {
    //   block [0x825A9FA0..0x825A9FAC)
	// 825A9FA0: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 825A9FA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A9FA8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9FAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A9FAC size=8
    let mut pc: u32 = 0x825A9FAC;
    'dispatch: loop {
        match pc {
            0x825A9FAC => {
    //   block [0x825A9FAC..0x825A9FB4)
	// 825A9FAC: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 825A9FB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A9FB8 size=24
    let mut pc: u32 = 0x825A9FB8;
    'dispatch: loop {
        match pc {
            0x825A9FB8 => {
    //   block [0x825A9FB8..0x825A9FD0)
	// 825A9FB8: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A9FBC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A9FC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A9FC4: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 825A9FC8: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 825A9FCC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A9FD0 size=12
    let mut pc: u32 = 0x825A9FD0;
    'dispatch: loop {
        match pc {
            0x825A9FD0 => {
    //   block [0x825A9FD0..0x825A9FDC)
	// 825A9FD0: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825A9FD4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A9FD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A9FE0 size=72
    let mut pc: u32 = 0x825A9FE0;
    'dispatch: loop {
        match pc {
            0x825A9FE0 => {
    //   block [0x825A9FE0..0x825AA028)
	// 825A9FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A9FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A9FE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A9FEC: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 825A9FF0: 419A001C  beq cr6, 0x825aa00c
	if ctx.cr[6].eq {
	pc = 0x825AA00C; continue 'dispatch;
	}
	// 825A9FF4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825A9FF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A9FFC: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 825AA000: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AA004: 4BFFFAF5  bl 0x825a9af8
	ctx.lr = 0x825AA008;
	sub_825A9AF8(ctx, base);
	// 825AA008: 48000010  b 0x825aa018
	pc = 0x825AA018; continue 'dispatch;
	// 825AA00C: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825AA010: 396B8360  addi r11, r11, -0x7ca0
	ctx.r[11].s64 = ctx.r[11].s64 + -31904;
	// 825AA014: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AA018: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AA01C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AA020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AA024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AA028 size=144
    let mut pc: u32 = 0x825AA028;
    'dispatch: loop {
        match pc {
            0x825AA028 => {
    //   block [0x825AA028..0x825AA0B8)
	// 825AA028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA02C: 48BFE13D  bl 0x831a8168
	ctx.lr = 0x825AA030;
	sub_831A8130(ctx, base);
	// 825AA030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AA034: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AA038: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AA03C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 825AA040: 396BAF84  addi r11, r11, -0x507c
	ctx.r[11].s64 = ctx.r[11].s64 + -20604;
	// 825AA044: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825AA048: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AA04C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825AA050: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 825AA054: 4884909D  bl 0x82df30f0
	ctx.lr = 0x825AA058;
	sub_82DF30F0(ctx, base);
	// 825AA058: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AA05C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 825AA060: 396BB3B4  addi r11, r11, -0x4c4c
	ctx.r[11].s64 = ctx.r[11].s64 + -19532;
	// 825AA064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825AA068: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AA06C: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 825AA070: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825AA074: 816A853C  lwz r11, -0x7ac4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-31428 as u32) ) } as u64;
	// 825AA078: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 825AA07C: 816A853C  lwz r11, -0x7ac4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-31428 as u32) ) } as u64;
	// 825AA080: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 825AA084: 913F0018  stw r9, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 825AA088: 48849B79  bl 0x82df3c00
	ctx.lr = 0x825AA08C;
	sub_82DF3C00(ctx, base);
	// 825AA08C: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 825AA090: 48849061  bl 0x82df30f0
	ctx.lr = 0x825AA094;
	sub_82DF30F0(ctx, base);
	// 825AA094: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 825AA098: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 825AA09C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AA0A0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA0A4: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 825AA0A8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA0AC: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 825AA0B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825AA0B4: 48BFE104  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AA0B8 size=152
    let mut pc: u32 = 0x825AA0B8;
    'dispatch: loop {
        match pc {
            0x825AA0B8 => {
    //   block [0x825AA0B8..0x825AA150)
	// 825AA0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA0BC: 48BFE0A9  bl 0x831a8164
	ctx.lr = 0x825AA0C0;
	sub_831A8130(ctx, base);
	// 825AA0C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AA0C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AA0C8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AA0CC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 825AA0D0: 396BAF84  addi r11, r11, -0x507c
	ctx.r[11].s64 = ctx.r[11].s64 + -20604;
	// 825AA0D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825AA0D8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AA0DC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825AA0E0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 825AA0E4: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 825AA0E8: 48849009  bl 0x82df30f0
	ctx.lr = 0x825AA0EC;
	sub_82DF30F0(ctx, base);
	// 825AA0EC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AA0F0: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 825AA0F4: 396BB3B4  addi r11, r11, -0x4c4c
	ctx.r[11].s64 = ctx.r[11].s64 + -19532;
	// 825AA0F8: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 825AA0FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AA100: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825AA104: 816A853C  lwz r11, -0x7ac4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-31428 as u32) ) } as u64;
	// 825AA108: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 825AA10C: 816A853C  lwz r11, -0x7ac4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-31428 as u32) ) } as u64;
	// 825AA110: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 825AA114: 4BFE1DFD  bl 0x8258bf10
	ctx.lr = 0x825AA118;
	sub_8258BF10(ctx, base);
	// 825AA118: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 825AA11C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825AA120: 48849AE1  bl 0x82df3c00
	ctx.lr = 0x825AA124;
	sub_82DF3C00(ctx, base);
	// 825AA124: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 825AA128: 48848FC9  bl 0x82df30f0
	ctx.lr = 0x825AA12C;
	sub_82DF30F0(ctx, base);
	// 825AA12C: 937F0050  stw r27, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 825AA130: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 825AA134: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AA138: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA13C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 825AA140: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA144: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 825AA148: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825AA14C: 48BFE068  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AA150 size=104
    let mut pc: u32 = 0x825AA150;
    'dispatch: loop {
        match pc {
            0x825AA150 => {
    //   block [0x825AA150..0x825AA1B8)
	// 825AA150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AA158: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AA15C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AA160: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AA164: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AA168: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825AA16C: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 825AA170: 488492B9  bl 0x82df3428
	ctx.lr = 0x825AA174;
	sub_82DF3428(ctx, base);
	// 825AA174: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 825AA178: 488492B1  bl 0x82df3428
	ctx.lr = 0x825AA17C;
	sub_82DF3428(ctx, base);
	// 825AA17C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 825AA180: 4BD1EB39  bl 0x822c8cb8
	ctx.lr = 0x825AA184;
	sub_822C8CB8(ctx, base);
	// 825AA184: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AA188: 4BFFC289  bl 0x825a6410
	ctx.lr = 0x825AA18C;
	sub_825A6410(ctx, base);
	// 825AA18C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AA190: 4182000C  beq 0x825aa19c
	if ctx.cr[0].eq {
	pc = 0x825AA19C; continue 'dispatch;
	}
	// 825AA194: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AA198: 4BD160D1  bl 0x822c0268
	ctx.lr = 0x825AA19C;
	sub_822C0268(ctx, base);
	// 825AA19C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AA1A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AA1A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AA1A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AA1AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AA1B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AA1B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AA1B8 size=300
    let mut pc: u32 = 0x825AA1B8;
    'dispatch: loop {
        match pc {
            0x825AA1B8 => {
    //   block [0x825AA1B8..0x825AA2E4)
	// 825AA1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA1BC: 48BFDFA5  bl 0x831a8160
	ctx.lr = 0x825AA1C0;
	sub_831A8130(ctx, base);
	// 825AA1C0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AA1C4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825AA1C8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AA1CC: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 825AA1D0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825AA1D4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825AA1D8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825AA1DC: 388BB408  addi r4, r11, -0x4bf8
	ctx.r[4].s64 = ctx.r[11].s64 + -19448;
	// 825AA1E0: 38A00082  li r5, 0x82
	ctx.r[5].s64 = 130;
	// 825AA1E4: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 825AA1E8: 4BD161F1  bl 0x822c03d8
	ctx.lr = 0x825AA1EC;
	sub_822C03D8(ctx, base);
	// 825AA1EC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 825AA1F0: 41820048  beq 0x825aa238
	if ctx.cr[0].eq {
	pc = 0x825AA238; continue 'dispatch;
	}
	// 825AA1F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA1F8: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 825AA1FC: 837F0008  lwz r27, 8(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AA200: 3B5F0018  addi r26, r31, 0x18
	ctx.r[26].s64 = ctx.r[31].s64 + 24;
	// 825AA204: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AA208: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AA20C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 825AA210: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AA214: 4E800421  bctrl
	ctx.lr = 0x825AA218;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AA218: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AA21C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AA220: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 825AA224: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 825AA228: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825AA22C: 4BFFFE8D  bl 0x825aa0b8
	ctx.lr = 0x825AA230;
	sub_825AA0B8(ctx, base);
	// 825AA230: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825AA234: 48000008  b 0x825aa23c
	pc = 0x825AA23C; continue 'dispatch;
	// 825AA238: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825AA23C: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 825AA240: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825AA244: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825AA248: 4BFFD4D9  bl 0x825a7720
	ctx.lr = 0x825AA24C;
	sub_825A7720(ctx, base);
	// 825AA24C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825AA250: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825AA254: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825AA258: 4BD15DA9  bl 0x822c0000
	ctx.lr = 0x825AA25C;
	sub_822C0000(ctx, base);
	// 825AA25C: 57AB07FF  clrlwi. r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AA260: 4182000C  beq 0x825aa26c
	if ctx.cr[0].eq {
	pc = 0x825AA26C; continue 'dispatch;
	}
	// 825AA264: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AA268: 488491C1  bl 0x82df3428
	ctx.lr = 0x825AA26C;
	sub_82DF3428(ctx, base);
	// 825AA26C: 83C10058  lwz r30, 0x58(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825AA270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AA274: 387E0018  addi r3, r30, 0x18
	ctx.r[3].s64 = ctx.r[30].s64 + 24;
	// 825AA278: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825AA27C: 4BD1EA3D  bl 0x822c8cb8
	ctx.lr = 0x825AA280;
	sub_822C8CB8(ctx, base);
	// 825AA280: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AA284: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AA288: 4BFFC201  bl 0x825a6488
	ctx.lr = 0x825AA28C;
	sub_825A6488(ctx, base);
	// 825AA28C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AA290: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AA294: 4822B7D5  bl 0x827d5a68
	ctx.lr = 0x825AA298;
	sub_827D5A68(ctx, base);
	// 825AA298: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AA29C: 4884918D  bl 0x82df3428
	ctx.lr = 0x825AA2A0;
	sub_82DF3428(ctx, base);
	// 825AA2A0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825AA2A4: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 825AA2A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AA2AC: 907C0004  stw r3, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 825AA2B0: 419A0028  beq cr6, 0x825aa2d8
	if ctx.cr[6].eq {
	pc = 0x825AA2D8; continue 'dispatch;
	}
	// 825AA2B4: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 825AA2B8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825AA2BC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AA2C0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825AA2C4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825AA2C8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825AA2CC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AA2D0: 4082FFE8  bne 0x825aa2b8
	if !ctx.cr[0].eq {
	pc = 0x825AA2B8; continue 'dispatch;
	}
	// 825AA2D4: 4BD165BD  bl 0x822c0890
	ctx.lr = 0x825AA2D8;
	sub_822C0890(ctx, base);
	// 825AA2D8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825AA2DC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825AA2E0: 48BFDED0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AA2E8 size=16
    let mut pc: u32 = 0x825AA2E8;
    'dispatch: loop {
        match pc {
            0x825AA2E8 => {
    //   block [0x825AA2E8..0x825AA2F8)
	// 825AA2E8: 38630018  addi r3, r3, 0x18
	ctx.r[3].s64 = ctx.r[3].s64 + 24;
	// 825AA2EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA2F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AA2F4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AA2F8 size=16
    let mut pc: u32 = 0x825AA2F8;
    'dispatch: loop {
        match pc {
            0x825AA2F8 => {
    //   block [0x825AA2F8..0x825AA308)
	// 825AA2F8: 3D60822C  lis r11, -0x7dd4
	ctx.r[11].s64 = -2111045632;
	// 825AA2FC: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 825AA300: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AA304: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AA308 size=8
    let mut pc: u32 = 0x825AA308;
    'dispatch: loop {
        match pc {
            0x825AA308 => {
    //   block [0x825AA308..0x825AA310)
	// 825AA308: 4BEA9378  b 0x82453680
	sub_82453680(ctx, base);
	return;
	// 825AA30C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AA310 size=128
    let mut pc: u32 = 0x825AA310;
    'dispatch: loop {
        match pc {
            0x825AA310 => {
    //   block [0x825AA310..0x825AA390)
	// 825AA310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA314: 48BFDE59  bl 0x831a816c
	ctx.lr = 0x825AA318;
	sub_831A8130(ctx, base);
	// 825AA318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AA31C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 825AA320: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825AA324: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825AA328: 3BEB7B88  addi r31, r11, 0x7b88
	ctx.r[31].s64 = ctx.r[11].s64 + 31624;
	// 825AA32C: 816A7B90  lwz r11, 0x7b90(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(31632 as u32) ) } as u64;
	// 825AA330: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 825AA334: 40820024  bne 0x825aa358
	if !ctx.cr[0].eq {
	pc = 0x825AA358; continue 'dispatch;
	}
	// 825AA338: 3D208256  lis r9, -0x7daa
	ctx.r[9].s64 = -2108293120;
	// 825AA33C: 3D00825B  lis r8, -0x7da5
	ctx.r[8].s64 = -2107965440;
	// 825AA340: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 825AA344: 3929DA70  addi r9, r9, -0x2590
	ctx.r[9].s64 = ctx.r[9].s64 + -9616;
	// 825AA348: 39089FE0  addi r8, r8, -0x6020
	ctx.r[8].s64 = ctx.r[8].s64 + -24608;
	// 825AA34C: 916A7B90  stw r11, 0x7b90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31632 as u32), ctx.r[11].u32 ) };
	// 825AA350: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825AA354: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 825AA358: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825AA35C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825AA360: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AA364: 38BE0008  addi r5, r30, 8
	ctx.r[5].s64 = ctx.r[30].s64 + 8;
	// 825AA368: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 825AA36C: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AA370: 480285B1  bl 0x825d2920
	ctx.lr = 0x825AA374;
	sub_825D2920(ctx, base);
	// 825AA374: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AA378: 4182000C  beq 0x825aa384
	if ctx.cr[0].eq {
	pc = 0x825AA384; continue 'dispatch;
	}
	// 825AA37C: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825AA380: 48000008  b 0x825aa388
	pc = 0x825AA388; continue 'dispatch;
	// 825AA384: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825AA388: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825AA38C: 48BFDE30  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AA390 size=132
    let mut pc: u32 = 0x825AA390;
    'dispatch: loop {
        match pc {
            0x825AA390 => {
    //   block [0x825AA390..0x825AA414)
	// 825AA390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AA398: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AA39C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AA3A0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AA3A4: 3D60825B  lis r11, -0x7da5
	ctx.r[11].s64 = -2107965440;
	// 825AA3A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AA3AC: 396B9A00  addi r11, r11, -0x6600
	ctx.r[11].s64 = ctx.r[11].s64 + -26112;
	// 825AA3B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825AA3B4: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 825AA3B8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825AA3BC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825AA3C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825AA3C4: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825AA3C8: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 825AA3CC: 4BFFFF45  bl 0x825aa310
	ctx.lr = 0x825AA3D0;
	sub_825AA310(ctx, base);
	// 825AA3D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AA3D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AA3D8: 3BE10060  addi r31, r1, 0x60
	ctx.r[31].s64 = ctx.r[1].s64 + 96;
	// 825AA3DC: 4BFFC0AD  bl 0x825a6488
	ctx.lr = 0x825AA3E0;
	sub_825A6488(ctx, base);
	// 825AA3E0: 48848DD1  bl 0x82df31b0
	ctx.lr = 0x825AA3E4;
	sub_82DF31B0(ctx, base);
	// 825AA3E4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AA3E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AA3EC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825AA3F0: 4802FAF1  bl 0x825d9ee0
	ctx.lr = 0x825AA3F4;
	sub_825D9EE0(ctx, base);
	// 825AA3F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AA3F8: 48849031  bl 0x82df3428
	ctx.lr = 0x825AA3FC;
	sub_82DF3428(ctx, base);
	// 825AA3FC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825AA400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AA404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AA408: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AA40C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AA410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AA418 size=132
    let mut pc: u32 = 0x825AA418;
    'dispatch: loop {
        match pc {
            0x825AA418 => {
    //   block [0x825AA418..0x825AA49C)
	// 825AA418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AA420: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AA424: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AA428: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AA42C: 3D60825B  lis r11, -0x7da5
	ctx.r[11].s64 = -2107965440;
	// 825AA430: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AA434: 396B9A80  addi r11, r11, -0x6580
	ctx.r[11].s64 = ctx.r[11].s64 + -25984;
	// 825AA438: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825AA43C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 825AA440: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825AA444: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825AA448: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825AA44C: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825AA450: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 825AA454: 4BFFFEBD  bl 0x825aa310
	ctx.lr = 0x825AA458;
	sub_825AA310(ctx, base);
	// 825AA458: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AA45C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AA460: 3BE10060  addi r31, r1, 0x60
	ctx.r[31].s64 = ctx.r[1].s64 + 96;
	// 825AA464: 4BFFC025  bl 0x825a6488
	ctx.lr = 0x825AA468;
	sub_825A6488(ctx, base);
	// 825AA468: 48848D49  bl 0x82df31b0
	ctx.lr = 0x825AA46C;
	sub_82DF31B0(ctx, base);
	// 825AA46C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AA470: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AA474: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825AA478: 4802FA69  bl 0x825d9ee0
	ctx.lr = 0x825AA47C;
	sub_825D9EE0(ctx, base);
	// 825AA47C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AA480: 48848FA9  bl 0x82df3428
	ctx.lr = 0x825AA484;
	sub_82DF3428(ctx, base);
	// 825AA484: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825AA488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AA48C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AA490: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AA494: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AA498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AA4A0 size=88
    let mut pc: u32 = 0x825AA4A0;
    'dispatch: loop {
        match pc {
            0x825AA4A0 => {
    //   block [0x825AA4A0..0x825AA4F8)
	// 825AA4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AA4A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AA4AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AA4B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AA4B4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825AA4B8: 4BFFBA61  bl 0x825a5f18
	ctx.lr = 0x825AA4BC;
	sub_825A5F18(ctx, base);
	// 825AA4BC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AA4C0: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 825AA4C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AA4C8: 419A0008  beq cr6, 0x825aa4d0
	if ctx.cr[6].eq {
	pc = 0x825AA4D0; continue 'dispatch;
	}
	// 825AA4CC: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 825AA4D0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA4D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AA4D8: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AA4DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AA4E0: 4E800421  bctrl
	ctx.lr = 0x825AA4E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AA4E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AA4E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AA4EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AA4F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AA4F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AA4F8 size=60
    let mut pc: u32 = 0x825AA4F8;
    'dispatch: loop {
        match pc {
            0x825AA4F8 => {
    //   block [0x825AA4F8..0x825AA534)
	// 825AA4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AA500: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AA504: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 825AA508: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AA50C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 825AA510: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA514: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA518: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AA51C: 4E800421  bctrl
	ctx.lr = 0x825AA520;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AA520: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825AA524: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AA528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AA52C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AA530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AA538 size=56
    let mut pc: u32 = 0x825AA538;
    'dispatch: loop {
        match pc {
            0x825AA538 => {
    //   block [0x825AA538..0x825AA570)
	// 825AA538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA53C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AA540: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AA544: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AA548: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825AA54C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AA550: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 825AA554: 488494B5  bl 0x82df3a08
	ctx.lr = 0x825AA558;
	sub_82DF3A08(ctx, base);
	// 825AA558: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AA55C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AA560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AA564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AA568: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AA56C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AA570 size=136
    let mut pc: u32 = 0x825AA570;
    'dispatch: loop {
        match pc {
            0x825AA570 => {
    //   block [0x825AA570..0x825AA5F8)
	// 825AA570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AA578: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AA57C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AA580: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AA584: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825AA588: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825AA58C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 825AA590: 409A0020  bne cr6, 0x825aa5b0
	if !ctx.cr[6].eq {
	pc = 0x825AA5B0; continue 'dispatch;
	}
	// 825AA594: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825AA598: 419A0048  beq cr6, 0x825aa5e0
	if ctx.cr[6].eq {
	pc = 0x825AA5E0; continue 'dispatch;
	}
	// 825AA59C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA5A0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AA5A4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AA5A8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825AA5AC: 48000034  b 0x825aa5e0
	pc = 0x825AA5E0; continue 'dispatch;
	// 825AA5B0: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 825AA5B4: 419A002C  beq cr6, 0x825aa5e0
	if ctx.cr[6].eq {
	pc = 0x825AA5E0; continue 'dispatch;
	}
	// 825AA5B8: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825AA5BC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA5C0: 388B8428  addi r4, r11, -0x7bd8
	ctx.r[4].s64 = ctx.r[11].s64 + -31704;
	// 825AA5C4: 48BFDB35  bl 0x831a80f8
	ctx.lr = 0x825AA5C8;
	sub_831A80F8(ctx, base);
	// 825AA5C8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AA5CC: 4182000C  beq 0x825aa5d8
	if ctx.cr[0].eq {
	pc = 0x825AA5D8; continue 'dispatch;
	}
	// 825AA5D0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 825AA5D4: 4800000C  b 0x825aa5e0
	pc = 0x825AA5E0; continue 'dispatch;
	// 825AA5D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AA5DC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AA5E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AA5E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AA5E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AA5EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AA5F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AA5F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AA5F8 size=20
    let mut pc: u32 = 0x825AA5F8;
    'dispatch: loop {
        match pc {
            0x825AA5F8 => {
    //   block [0x825AA5F8..0x825AA60C)
	// 825AA5F8: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825AA5FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AA600: 409A000C  bne cr6, 0x825aa60c
	if !ctx.cr[6].eq {
		sub_825AA60C(ctx, base);
		return;
	}
	// 825AA604: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825AA608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA60C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AA60C size=16
    let mut pc: u32 = 0x825AA60C;
    'dispatch: loop {
        match pc {
            0x825AA60C => {
    //   block [0x825AA60C..0x825AA61C)
	// 825AA60C: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AA610: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 825AA614: 7D631670  srawi r3, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 825AA618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AA620 size=20
    let mut pc: u32 = 0x825AA620;
    'dispatch: loop {
        match pc {
            0x825AA620 => {
    //   block [0x825AA620..0x825AA634)
	// 825AA620: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 825AA624: 54AA103A  slwi r10, r5, 2
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825AA628: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825AA62C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AA630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AA638 size=196
    let mut pc: u32 = 0x825AA638;
    'dispatch: loop {
        match pc {
            0x825AA638 => {
    //   block [0x825AA638..0x825AA6FC)
	// 825AA638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AA640: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AA644: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AA648: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AA64C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825AA650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AA654: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825AA658: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825AA65C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AA660: 4BD162D9  bl 0x822c0938
	ctx.lr = 0x825AA664;
	sub_822C0938(ctx, base);
	// 825AA664: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825AA668: 41820028  beq 0x825aa690
	if ctx.cr[0].eq {
	pc = 0x825AA690; continue 'dispatch;
	}
	// 825AA66C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AA670: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825AA674: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825AA678: 392BB450  addi r9, r11, -0x4bb0
	ctx.r[9].s64 = ctx.r[11].s64 + -19376;
	// 825AA67C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825AA680: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825AA684: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825AA688: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825AA68C: 48000008  b 0x825aa694
	pc = 0x825AA694; continue 'dispatch;
	// 825AA690: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AA694: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AA698: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AA69C: 409A0044  bne cr6, 0x825aa6e0
	if !ctx.cr[6].eq {
	pc = 0x825AA6E0; continue 'dispatch;
	}
	// 825AA6A0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825AA6A4: 419A001C  beq cr6, 0x825aa6c0
	if ctx.cr[6].eq {
	pc = 0x825AA6C0; continue 'dispatch;
	}
	// 825AA6A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA6AC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825AA6B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AA6B4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA6B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AA6BC: 4E800421  bctrl
	ctx.lr = 0x825AA6C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AA6C0: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825AA6C4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825AA6C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AA6CC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825AA6D0: 816B8420  lwz r11, -0x7be0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31712 as u32) ) } as u64;
	// 825AA6D4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825AA6D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825AA6DC: 4BD15925  bl 0x822c0000
	ctx.lr = 0x825AA6E0;
	sub_822C0000(ctx, base);
	// 825AA6E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AA6E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AA6E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AA6EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AA6F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AA6F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AA6F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AA700 size=72
    let mut pc: u32 = 0x825AA700;
    'dispatch: loop {
        match pc {
            0x825AA700 => {
    //   block [0x825AA700..0x825AA748)
	// 825AA700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AA708: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AA70C: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 825AA710: 419A001C  beq cr6, 0x825aa72c
	if ctx.cr[6].eq {
	pc = 0x825AA72C; continue 'dispatch;
	}
	// 825AA714: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825AA718: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825AA71C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 825AA720: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AA724: 4BFFFE4D  bl 0x825aa570
	ctx.lr = 0x825AA728;
	sub_825AA570(ctx, base);
	// 825AA728: 48000010  b 0x825aa738
	pc = 0x825AA738; continue 'dispatch;
	// 825AA72C: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825AA730: 396B8428  addi r11, r11, -0x7bd8
	ctx.r[11].s64 = ctx.r[11].s64 + -31704;
	// 825AA734: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AA738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AA73C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AA740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AA744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AA748 size=8
    let mut pc: u32 = 0x825AA748;
    'dispatch: loop {
        match pc {
            0x825AA748 => {
    //   block [0x825AA748..0x825AA750)
	// 825AA748: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825AA74C: 48000018  b 0x825aa764
	sub_825AA750(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AA750 size=36
    let mut pc: u32 = 0x825AA750;
    'dispatch: loop {
        match pc {
            0x825AA750 => {
    //   block [0x825AA750..0x825AA774)
	// 825AA750: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA754: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 825AA758: 409A0008  bne cr6, 0x825aa760
	if !ctx.cr[6].eq {
	pc = 0x825AA760; continue 'dispatch;
	}
	// 825AA75C: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 825AA760: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825AA764: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AA768: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825AA76C: 409AFFE4  bne cr6, 0x825aa750
	if !ctx.cr[6].eq {
	pc = 0x825AA750; continue 'dispatch;
	}
	// 825AA770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AA778 size=120
    let mut pc: u32 = 0x825AA778;
    'dispatch: loop {
        match pc {
            0x825AA778 => {
    //   block [0x825AA778..0x825AA7F0)
	// 825AA778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA77C: 48BFD9ED  bl 0x831a8168
	ctx.lr = 0x825AA780;
	sub_831A8130(ctx, base);
	// 825AA780: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AA784: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825AA788: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 825AA78C: 83FE000C  lwz r31, 0xc(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 825AA790: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AA794: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AA798: 419A0050  beq cr6, 0x825aa7e8
	if ctx.cr[6].eq {
	pc = 0x825AA7E8; continue 'dispatch;
	}
	// 825AA79C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AA7A0: 3BABB300  addi r29, r11, -0x4d00
	ctx.r[29].s64 = ctx.r[11].s64 + -19712;
	// 825AA7A4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825AA7A8: 4802F5E9  bl 0x825d9d90
	ctx.lr = 0x825AA7AC;
	sub_825D9D90(ctx, base);
	// 825AA7AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA7B0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825AA7B4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825AA7B8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825AA7BC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825AA7C0: 4802F691  bl 0x825d9e50
	ctx.lr = 0x825AA7C4;
	sub_825D9E50(ctx, base);
	// 825AA7C4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825AA7C8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825AA7CC: 4802F635  bl 0x825d9e00
	ctx.lr = 0x825AA7D0;
	sub_825D9E00(ctx, base);
	// 825AA7D0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825AA7D4: 4802F5F5  bl 0x825d9dc8
	ctx.lr = 0x825AA7D8;
	sub_825D9DC8(ctx, base);
	// 825AA7D8: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AA7DC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 825AA7E0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AA7E4: 409AFFC0  bne cr6, 0x825aa7a4
	if !ctx.cr[6].eq {
	pc = 0x825AA7A4; continue 'dispatch;
	}
	// 825AA7E8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825AA7EC: 48BFD9CC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AA7F0 size=12
    let mut pc: u32 = 0x825AA7F0;
    'dispatch: loop {
        match pc {
            0x825AA7F0 => {
    //   block [0x825AA7F0..0x825AA7FC)
	// 825AA7F0: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825AA7F4: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AA7F8: 48000014  b 0x825aa80c
	sub_825AA7FC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA7FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AA7FC size=104
    let mut pc: u32 = 0x825AA7FC;
    'dispatch: loop {
        match pc {
            0x825AA7FC => {
    //   block [0x825AA7FC..0x825AA864)
	// 825AA7FC: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA800: 7F092040  cmplw cr6, r9, r4
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[4].u32, &mut ctx.xer);
	// 825AA804: 419A0010  beq cr6, 0x825aa814
	if ctx.cr[6].eq {
	pc = 0x825AA814; continue 'dispatch;
	}
	// 825AA808: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825AA80C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825AA810: 409AFFEC  bne cr6, 0x825aa7fc
	if !ctx.cr[6].eq {
	pc = 0x825AA7FC; continue 'dispatch;
	}
	// 825AA814: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825AA818: 419A003C  beq cr6, 0x825aa854
	if ctx.cr[6].eq {
	pc = 0x825AA854; continue 'dispatch;
	}
	// 825AA81C: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AA820: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 825AA824: 39030008  addi r8, r3, 8
	ctx.r[8].s64 = ctx.r[3].s64 + 8;
	// 825AA828: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825AA82C: 419A001C  beq cr6, 0x825aa848
	if ctx.cr[6].eq {
	pc = 0x825AA848; continue 'dispatch;
	}
	// 825AA830: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 825AA834: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA838: 7CEB512E  stwx r7, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[7].u32) };
	// 825AA83C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 825AA840: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825AA844: 409AFFF0  bne cr6, 0x825aa834
	if !ctx.cr[6].eq {
	pc = 0x825AA834; continue 'dispatch;
	}
	// 825AA848: 81680008  lwz r11, 8(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AA84C: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 825AA850: 91680008  stw r11, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825AA854: 38630030  addi r3, r3, 0x30
	ctx.r[3].s64 = ctx.r[3].s64 + 48;
	// 825AA858: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AA85C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AA860: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA864(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AA864 size=16
    let mut pc: u32 = 0x825AA864;
    'dispatch: loop {
        match pc {
            0x825AA864 => {
    //   block [0x825AA864..0x825AA874)
	// 825AA864: 3D60822C  lis r11, -0x7dd4
	ctx.r[11].s64 = -2111045632;
	// 825AA868: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 825AA86C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AA870: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA874(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AA874 size=8
    let mut pc: u32 = 0x825AA874;
    'dispatch: loop {
        match pc {
            0x825AA874 => {
    //   block [0x825AA874..0x825AA87C)
	// 825AA874: 4BEA8E0C  b 0x82453680
	sub_82453680(ctx, base);
	return;
	// 825AA878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AA880 size=128
    let mut pc: u32 = 0x825AA880;
    'dispatch: loop {
        match pc {
            0x825AA880 => {
    //   block [0x825AA880..0x825AA900)
	// 825AA880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA884: 48BFD8E9  bl 0x831a816c
	ctx.lr = 0x825AA888;
	sub_831A8130(ctx, base);
	// 825AA888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AA88C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 825AA890: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825AA894: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825AA898: 3BEB7B94  addi r31, r11, 0x7b94
	ctx.r[31].s64 = ctx.r[11].s64 + 31636;
	// 825AA89C: 816A7B9C  lwz r11, 0x7b9c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(31644 as u32) ) } as u64;
	// 825AA8A0: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 825AA8A4: 40820024  bne 0x825aa8c8
	if !ctx.cr[0].eq {
	pc = 0x825AA8C8; continue 'dispatch;
	}
	// 825AA8A8: 3D208256  lis r9, -0x7daa
	ctx.r[9].s64 = -2108293120;
	// 825AA8AC: 3D00825B  lis r8, -0x7da5
	ctx.r[8].s64 = -2107965440;
	// 825AA8B0: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 825AA8B4: 3929DA70  addi r9, r9, -0x2590
	ctx.r[9].s64 = ctx.r[9].s64 + -9616;
	// 825AA8B8: 3908A700  addi r8, r8, -0x5900
	ctx.r[8].s64 = ctx.r[8].s64 + -22784;
	// 825AA8BC: 916A7B9C  stw r11, 0x7b9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31644 as u32), ctx.r[11].u32 ) };
	// 825AA8C0: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825AA8C4: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 825AA8C8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825AA8CC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825AA8D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AA8D4: 38BE0008  addi r5, r30, 8
	ctx.r[5].s64 = ctx.r[30].s64 + 8;
	// 825AA8D8: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 825AA8DC: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AA8E0: 48028041  bl 0x825d2920
	ctx.lr = 0x825AA8E4;
	sub_825D2920(ctx, base);
	// 825AA8E4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AA8E8: 4182000C  beq 0x825aa8f4
	if ctx.cr[0].eq {
	pc = 0x825AA8F4; continue 'dispatch;
	}
	// 825AA8EC: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825AA8F0: 48000008  b 0x825aa8f8
	pc = 0x825AA8F8; continue 'dispatch;
	// 825AA8F4: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825AA8F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825AA8FC: 48BFD8C0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AA900 size=132
    let mut pc: u32 = 0x825AA900;
    'dispatch: loop {
        match pc {
            0x825AA900 => {
    //   block [0x825AA900..0x825AA984)
	// 825AA900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AA908: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AA90C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AA910: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AA914: 3D60825B  lis r11, -0x7da5
	ctx.r[11].s64 = -2107965440;
	// 825AA918: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AA91C: 396BA778  addi r11, r11, -0x5888
	ctx.r[11].s64 = ctx.r[11].s64 + -22664;
	// 825AA920: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825AA924: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 825AA928: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825AA92C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825AA930: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825AA934: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825AA938: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 825AA93C: 4BFFFF45  bl 0x825aa880
	ctx.lr = 0x825AA940;
	sub_825AA880(ctx, base);
	// 825AA940: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AA944: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AA948: 3BE10060  addi r31, r1, 0x60
	ctx.r[31].s64 = ctx.r[1].s64 + 96;
	// 825AA94C: 4BFFBB3D  bl 0x825a6488
	ctx.lr = 0x825AA950;
	sub_825A6488(ctx, base);
	// 825AA950: 48848861  bl 0x82df31b0
	ctx.lr = 0x825AA954;
	sub_82DF31B0(ctx, base);
	// 825AA954: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AA958: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AA95C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825AA960: 4802F581  bl 0x825d9ee0
	ctx.lr = 0x825AA964;
	sub_825D9EE0(ctx, base);
	// 825AA964: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AA968: 48848AC1  bl 0x82df3428
	ctx.lr = 0x825AA96C;
	sub_82DF3428(ctx, base);
	// 825AA96C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825AA970: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AA974: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AA978: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AA97C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AA980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AA988 size=108
    let mut pc: u32 = 0x825AA988;
    'dispatch: loop {
        match pc {
            0x825AA988 => {
    //   block [0x825AA988..0x825AA9F4)
	// 825AA988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA98C: 48BFD7E1  bl 0x831a816c
	ctx.lr = 0x825AA990;
	sub_831A8130(ctx, base);
	// 825AA990: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AA994: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AA998: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AA99C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 825AA9A0: 396BAF84  addi r11, r11, -0x507c
	ctx.r[11].s64 = ctx.r[11].s64 + -20604;
	// 825AA9A4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825AA9A8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AA9AC: 48848745  bl 0x82df30f0
	ctx.lr = 0x825AA9B0;
	sub_82DF30F0(ctx, base);
	// 825AA9B0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AA9B4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825AA9B8: 396BB464  addi r11, r11, -0x4b9c
	ctx.r[11].s64 = ctx.r[11].s64 + -19356;
	// 825AA9BC: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 825AA9C0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AA9C4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825AA9C8: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 825AA9CC: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 825AA9D0: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 825AA9D4: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 825AA9D8: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 825AA9DC: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 825AA9E0: 48849221  bl 0x82df3c00
	ctx.lr = 0x825AA9E4;
	sub_82DF3C00(ctx, base);
	// 825AA9E4: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 825AA9E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AA9EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AA9F0: 48BFD7CC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AA9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AA9F8 size=80
    let mut pc: u32 = 0x825AA9F8;
    'dispatch: loop {
        match pc {
            0x825AA9F8 => {
    //   block [0x825AA9F8..0x825AAA48)
	// 825AA9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AA9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AAA00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AAA04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AAA08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AAA0C: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 825AAA10: 4BD1E2A9  bl 0x822c8cb8
	ctx.lr = 0x825AAA14;
	sub_822C8CB8(ctx, base);
	// 825AAA14: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 825AAA18: 48848A11  bl 0x82df3428
	ctx.lr = 0x825AAA1C;
	sub_82DF3428(ctx, base);
	// 825AAA1C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 825AAA20: 4BEBF5E1  bl 0x8246a000
	ctx.lr = 0x825AAA24;
	sub_8246A000(ctx, base);
	// 825AAA24: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 825AAA28: 4BEBF5D9  bl 0x8246a000
	ctx.lr = 0x825AAA2C;
	sub_8246A000(ctx, base);
	// 825AAA2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AAA30: 4BFFB9E1  bl 0x825a6410
	ctx.lr = 0x825AAA34;
	sub_825A6410(ctx, base);
	// 825AAA34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AAA38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AAA3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AAA40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AAA44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AAA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AAA48 size=76
    let mut pc: u32 = 0x825AAA48;
    'dispatch: loop {
        match pc {
            0x825AAA48 => {
    //   block [0x825AAA48..0x825AAA94)
	// 825AAA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AAA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AAA50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AAA54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AAA58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AAA5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AAA60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825AAA64: 4BFFFF95  bl 0x825aa9f8
	ctx.lr = 0x825AAA68;
	sub_825AA9F8(ctx, base);
	// 825AAA68: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AAA6C: 4182000C  beq 0x825aaa78
	if ctx.cr[0].eq {
	pc = 0x825AAA78; continue 'dispatch;
	}
	// 825AAA70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AAA74: 4BD157F5  bl 0x822c0268
	ctx.lr = 0x825AAA78;
	sub_822C0268(ctx, base);
	// 825AAA78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AAA7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AAA80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AAA84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AAA88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AAA8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AAA90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AAA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AAA98 size=120
    let mut pc: u32 = 0x825AAA98;
    'dispatch: loop {
        match pc {
            0x825AAA98 => {
    //   block [0x825AAA98..0x825AAB10)
	// 825AAA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AAA9C: 48BFD6D1  bl 0x831a816c
	ctx.lr = 0x825AAAA0;
	sub_831A8130(ctx, base);
	// 825AAAA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AAAA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AAAA8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AAAAC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 825AAAB0: 396BAF84  addi r11, r11, -0x507c
	ctx.r[11].s64 = ctx.r[11].s64 + -20604;
	// 825AAAB4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825AAAB8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AAABC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825AAAC0: 48848631  bl 0x82df30f0
	ctx.lr = 0x825AAAC4;
	sub_82DF30F0(ctx, base);
	// 825AAAC4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825AAAC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AAACC: 394AB464  addi r10, r10, -0x4b9c
	ctx.r[10].s64 = ctx.r[10].s64 + -19356;
	// 825AAAD0: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 825AAAD4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825AAAD8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825AAADC: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 825AAAE0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 825AAAE4: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 825AAAE8: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 825AAAEC: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825AAAF0: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825AAAF4: 4884910D  bl 0x82df3c00
	ctx.lr = 0x825AAAF8;
	sub_82DF3C00(ctx, base);
	// 825AAAF8: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 825AAAFC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825AAB00: 4BFE1411  bl 0x8258bf10
	ctx.lr = 0x825AAB04;
	sub_8258BF10(ctx, base);
	// 825AAB04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AAB08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AAB0C: 48BFD6B0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AAB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AAB10 size=88
    let mut pc: u32 = 0x825AAB10;
    'dispatch: loop {
        match pc {
            0x825AAB10 => {
    //   block [0x825AAB10..0x825AAB68)
	// 825AAB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AAB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AAB18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AAB1C: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825AAB20: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AAB24: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 825AAB28: 48000014  b 0x825aab3c
	pc = 0x825AAB3C; continue 'dispatch;
	// 825AAB2C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AAB30: 7F092040  cmplw cr6, r9, r4
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[4].u32, &mut ctx.xer);
	// 825AAB34: 419A0010  beq cr6, 0x825aab44
	if ctx.cr[6].eq {
	pc = 0x825AAB44; continue 'dispatch;
	}
	// 825AAB38: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825AAB3C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825AAB40: 409AFFEC  bne cr6, 0x825aab2c
	if !ctx.cr[6].eq {
	pc = 0x825AAB2C; continue 'dispatch;
	}
	// 825AAB44: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825AAB48: 409A0010  bne cr6, 0x825aab58
	if !ctx.cr[6].eq {
	pc = 0x825AAB58; continue 'dispatch;
	}
	// 825AAB4C: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 825AAB50: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 825AAB54: 4BF0A685  bl 0x824b51d8
	ctx.lr = 0x825AAB58;
	sub_824B51D8(ctx, base);
	// 825AAB58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AAB5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AAB60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AAB64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AAB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AAB68 size=80
    let mut pc: u32 = 0x825AAB68;
    'dispatch: loop {
        match pc {
            0x825AAB68 => {
    //   block [0x825AAB68..0x825AABB8)
	// 825AAB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AAB6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AAB70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AAB74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AAB78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AAB7C: 4BFFFF95  bl 0x825aab10
	ctx.lr = 0x825AAB80;
	sub_825AAB10(ctx, base);
	// 825AAB80: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 825AAB84: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 825AAB88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AAB8C: 419A0018  beq cr6, 0x825aaba4
	if ctx.cr[6].eq {
	pc = 0x825AABA4; continue 'dispatch;
	}
	// 825AAB90: 3D60822C  lis r11, -0x7dd4
	ctx.r[11].s64 = -2111045632;
	// 825AAB94: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 825AAB98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AAB9C: 419A0008  beq cr6, 0x825aaba4
	if ctx.cr[6].eq {
	pc = 0x825AABA4; continue 'dispatch;
	}
	// 825AABA0: 4BEA8AE1  bl 0x82453680
	ctx.lr = 0x825AABA4;
	sub_82453680(ctx, base);
	// 825AABA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AABA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AABAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AABB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AABB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AABB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AABB8 size=120
    let mut pc: u32 = 0x825AABB8;
    'dispatch: loop {
        match pc {
            0x825AABB8 => {
    //   block [0x825AABB8..0x825AAC30)
	// 825AABB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AABBC: 48BFD5B1  bl 0x831a816c
	ctx.lr = 0x825AABC0;
	sub_831A8130(ctx, base);
	// 825AABC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AABC4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825AABC8: 3BDD0008  addi r30, r29, 8
	ctx.r[30].s64 = ctx.r[29].s64 + 8;
	// 825AABCC: 813D000C  lwz r9, 0xc(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 825AABD0: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AABD4: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AABD8: 419A002C  beq cr6, 0x825aac04
	if ctx.cr[6].eq {
	pc = 0x825AAC04; continue 'dispatch;
	}
	// 825AABDC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 825AABE0: 7F0B5840  cmplw cr6, r11, r11
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AABE4: 419A001C  beq cr6, 0x825aac00
	if ctx.cr[6].eq {
	pc = 0x825AAC00; continue 'dispatch;
	}
	// 825AABE8: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AABEC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 825AABF0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AABF4: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 825AABF8: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 825AABFC: 409AFFEC  bne cr6, 0x825aabe8
	if !ctx.cr[6].eq {
	pc = 0x825AABE8; continue 'dispatch;
	}
	// 825AAC00: 913E0008  stw r9, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 825AAC04: 83FD001C  lwz r31, 0x1c(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 825AAC08: 48000014  b 0x825aac1c
	pc = 0x825AAC1C; continue 'dispatch;
	// 825AAC0C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AAC10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AAC14: 4BF0A5C5  bl 0x824b51d8
	ctx.lr = 0x825AAC18;
	sub_824B51D8(ctx, base);
	// 825AAC18: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 825AAC1C: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 825AAC20: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AAC24: 409AFFE8  bne cr6, 0x825aac0c
	if !ctx.cr[6].eq {
	pc = 0x825AAC0C; continue 'dispatch;
	}
	// 825AAC28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AAC2C: 48BFD590  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AAC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AAC30 size=120
    let mut pc: u32 = 0x825AAC30;
    'dispatch: loop {
        match pc {
            0x825AAC30 => {
    //   block [0x825AAC30..0x825AACA8)
	// 825AAC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AAC34: 48BFD539  bl 0x831a816c
	ctx.lr = 0x825AAC38;
	sub_831A8130(ctx, base);
	// 825AAC38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AAC3C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825AAC40: 3BDD0018  addi r30, r29, 0x18
	ctx.r[30].s64 = ctx.r[29].s64 + 24;
	// 825AAC44: 813D001C  lwz r9, 0x1c(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 825AAC48: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 825AAC4C: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AAC50: 419A002C  beq cr6, 0x825aac7c
	if ctx.cr[6].eq {
	pc = 0x825AAC7C; continue 'dispatch;
	}
	// 825AAC54: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 825AAC58: 7F0B5840  cmplw cr6, r11, r11
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AAC5C: 419A001C  beq cr6, 0x825aac78
	if ctx.cr[6].eq {
	pc = 0x825AAC78; continue 'dispatch;
	}
	// 825AAC60: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AAC64: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 825AAC68: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AAC6C: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 825AAC70: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 825AAC74: 409AFFEC  bne cr6, 0x825aac60
	if !ctx.cr[6].eq {
	pc = 0x825AAC60; continue 'dispatch;
	}
	// 825AAC78: 913E0008  stw r9, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 825AAC7C: 83FD000C  lwz r31, 0xc(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 825AAC80: 48000014  b 0x825aac94
	pc = 0x825AAC94; continue 'dispatch;
	// 825AAC84: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AAC88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AAC8C: 4BF0A54D  bl 0x824b51d8
	ctx.lr = 0x825AAC90;
	sub_824B51D8(ctx, base);
	// 825AAC90: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 825AAC94: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AAC98: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AAC9C: 409AFFE8  bne cr6, 0x825aac84
	if !ctx.cr[6].eq {
	pc = 0x825AAC84; continue 'dispatch;
	}
	// 825AACA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AACA4: 48BFD518  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AACA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AACA8 size=160
    let mut pc: u32 = 0x825AACA8;
    'dispatch: loop {
        match pc {
            0x825AACA8 => {
    //   block [0x825AACA8..0x825AAD48)
	// 825AACA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AACAC: 48BFD4C1  bl 0x831a816c
	ctx.lr = 0x825AACB0;
	sub_831A8130(ctx, base);
	// 825AACB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AACB4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825AACB8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825AACBC: 3BAB0008  addi r29, r11, 8
	ctx.r[29].s64 = ctx.r[11].s64 + 8;
	// 825AACC0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825AACC4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AACC8: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AACCC: 419A002C  beq cr6, 0x825aacf8
	if ctx.cr[6].eq {
	pc = 0x825AACF8; continue 'dispatch;
	}
	// 825AACD0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 825AACD4: 7F0B5840  cmplw cr6, r11, r11
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AACD8: 419A001C  beq cr6, 0x825aacf4
	if ctx.cr[6].eq {
	pc = 0x825AACF4; continue 'dispatch;
	}
	// 825AACDC: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AACE0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 825AACE4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AACE8: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 825AACEC: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 825AACF0: 409AFFEC  bne cr6, 0x825aacdc
	if !ctx.cr[6].eq {
	pc = 0x825AACDC; continue 'dispatch;
	}
	// 825AACF4: 913D0008  stw r9, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 825AACF8: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825AACFC: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 825AAD00: 38CB8518  addi r6, r11, -0x7ae8
	ctx.r[6].s64 = ctx.r[11].s64 + -31464;
	// 825AAD04: 38AA7B58  addi r5, r10, 0x7b58
	ctx.r[5].s64 = ctx.r[10].s64 + 31576;
	// 825AAD08: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825AAD0C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825AAD10: 48BFF239  bl 0x831a9f48
	ctx.lr = 0x825AAD14;
	sub_831A9F48(ctx, base);
	// 825AAD14: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 825AAD18: 41820028  beq 0x825aad40
	if ctx.cr[0].eq {
	pc = 0x825AAD40; continue 'dispatch;
	}
	// 825AAD1C: 83FE000C  lwz r31, 0xc(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 825AAD20: 48000014  b 0x825aad34
	pc = 0x825AAD34; continue 'dispatch;
	// 825AAD24: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AAD28: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825AAD2C: 4BF0A4AD  bl 0x824b51d8
	ctx.lr = 0x825AAD30;
	sub_824B51D8(ctx, base);
	// 825AAD30: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 825AAD34: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AAD38: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AAD3C: 409AFFE8  bne cr6, 0x825aad24
	if !ctx.cr[6].eq {
	pc = 0x825AAD24; continue 'dispatch;
	}
	// 825AAD40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AAD44: 48BFD478  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AAD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AAD48 size=272
    let mut pc: u32 = 0x825AAD48;
    'dispatch: loop {
        match pc {
            0x825AAD48 => {
    //   block [0x825AAD48..0x825AAE58)
	// 825AAD48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AAD4C: 48BFD415  bl 0x831a8160
	ctx.lr = 0x825AAD50;
	sub_831A8130(ctx, base);
	// 825AAD50: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AAD54: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AAD58: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 825AAD5C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825AAD60: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825AAD64: 388BB4B8  addi r4, r11, -0x4b48
	ctx.r[4].s64 = ctx.r[11].s64 + -19272;
	// 825AAD68: 38A00099  li r5, 0x99
	ctx.r[5].s64 = 153;
	// 825AAD6C: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 825AAD70: 4BD15669  bl 0x822c03d8
	ctx.lr = 0x825AAD74;
	sub_822C03D8(ctx, base);
	// 825AAD74: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825AAD78: 41820014  beq 0x825aad8c
	if ctx.cr[0].eq {
	pc = 0x825AAD8C; continue 'dispatch;
	}
	// 825AAD7C: 389E0028  addi r4, r30, 0x28
	ctx.r[4].s64 = ctx.r[30].s64 + 40;
	// 825AAD80: 4BFFFC09  bl 0x825aa988
	ctx.lr = 0x825AAD84;
	sub_825AA988(ctx, base);
	// 825AAD84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AAD88: 48000008  b 0x825aad90
	pc = 0x825AAD90; continue 'dispatch;
	// 825AAD8C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825AAD90: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 825AAD94: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AAD98: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825AAD9C: 4BFFF89D  bl 0x825aa638
	ctx.lr = 0x825AADA0;
	sub_825AA638(ctx, base);
	// 825AADA0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825AADA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AADA8: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825AADAC: 4BD15255  bl 0x822c0000
	ctx.lr = 0x825AADB0;
	sub_822C0000(ctx, base);
	// 825AADB0: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AADB4: 83A10058  lwz r29, 0x58(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825AADB8: 83FE000C  lwz r31, 0xc(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 825AADBC: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AADC0: 419A0034  beq cr6, 0x825aadf4
	if ctx.cr[6].eq {
	pc = 0x825AADF4; continue 'dispatch;
	}
	// 825AADC4: 3B9D0008  addi r28, r29, 8
	ctx.r[28].s64 = ctx.r[29].s64 + 8;
	// 825AADC8: 3B7D0018  addi r27, r29, 0x18
	ctx.r[27].s64 = ctx.r[29].s64 + 24;
	// 825AADCC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AADD0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825AADD4: 4BF0A405  bl 0x824b51d8
	ctx.lr = 0x825AADD8;
	sub_824B51D8(ctx, base);
	// 825AADD8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AADDC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 825AADE0: 4BF0A3F9  bl 0x824b51d8
	ctx.lr = 0x825AADE4;
	sub_824B51D8(ctx, base);
	// 825AADE4: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AADE8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 825AADEC: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AADF0: 409AFFDC  bne cr6, 0x825aadcc
	if !ctx.cr[6].eq {
	pc = 0x825AADCC; continue 'dispatch;
	}
	// 825AADF4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825AADF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AADFC: 4BFFB68D  bl 0x825a6488
	ctx.lr = 0x825AAE00;
	sub_825A6488(ctx, base);
	// 825AAE00: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AAE04: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825AAE08: 4822AC61  bl 0x827d5a68
	ctx.lr = 0x825AAE0C;
	sub_827D5A68(ctx, base);
	// 825AAE0C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AAE10: 48848619  bl 0x82df3428
	ctx.lr = 0x825AAE14;
	sub_82DF3428(ctx, base);
	// 825AAE14: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825AAE18: 93BA0000  stw r29, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825AAE1C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AAE20: 907A0004  stw r3, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 825AAE24: 419A0028  beq cr6, 0x825aae4c
	if ctx.cr[6].eq {
	pc = 0x825AAE4C; continue 'dispatch;
	}
	// 825AAE28: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 825AAE2C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825AAE30: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AAE34: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825AAE38: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825AAE3C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825AAE40: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AAE44: 4082FFE8  bne 0x825aae2c
	if !ctx.cr[0].eq {
	pc = 0x825AAE2C; continue 'dispatch;
	}
	// 825AAE48: 4BD15A49  bl 0x822c0890
	ctx.lr = 0x825AAE4C;
	sub_822C0890(ctx, base);
	// 825AAE4C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825AAE50: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825AAE54: 48BFD35C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AAE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AAE58 size=12
    let mut pc: u32 = 0x825AAE58;
    'dispatch: loop {
        match pc {
            0x825AAE58 => {
    //   block [0x825AAE58..0x825AAE64)
	// 825AAE58: 38830008  addi r4, r3, 8
	ctx.r[4].s64 = ctx.r[3].s64 + 8;
	// 825AAE5C: 38630018  addi r3, r3, 0x18
	ctx.r[3].s64 = ctx.r[3].s64 + 24;
	// 825AAE60: 4BEBFCC8  b 0x8246ab28
	sub_8246AB28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AAE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AAE68 size=88
    let mut pc: u32 = 0x825AAE68;
    'dispatch: loop {
        match pc {
            0x825AAE68 => {
    //   block [0x825AAE68..0x825AAEC0)
	// 825AAE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AAE6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AAE70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AAE74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AAE78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AAE7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AAE80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AAE84: 4802E74D  bl 0x825d95d0
	ctx.lr = 0x825AAE88;
	sub_825D95D0(ctx, base);
	// 825AAE88: 4BFFB091  bl 0x825a5f18
	ctx.lr = 0x825AAE8C;
	sub_825A5F18(ctx, base);
	// 825AAE8C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825AAE90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AAE94: 48848595  bl 0x82df3428
	ctx.lr = 0x825AAE98;
	sub_82DF3428(ctx, base);
	// 825AAE98: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 825AAE9C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825AAEA0: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 825AAEA4: 4BF0A335  bl 0x824b51d8
	ctx.lr = 0x825AAEA8;
	sub_824B51D8(ctx, base);
	// 825AAEA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AAEAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AAEB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AAEB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AAEB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AAEBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AAEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AAEC0 size=136
    let mut pc: u32 = 0x825AAEC0;
    'dispatch: loop {
        match pc {
            0x825AAEC0 => {
    //   block [0x825AAEC0..0x825AAF48)
	// 825AAEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AAEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AAEC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AAECC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AAED0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AAED4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AAED8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AAEDC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825AAEE0: 4802EEB1  bl 0x825d9d90
	ctx.lr = 0x825AAEE4;
	sub_825D9D90(ctx, base);
	// 825AAEE4: 3D60825B  lis r11, -0x7da5
	ctx.r[11].s64 = -2107965440;
	// 825AAEE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825AAEEC: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 825AAEF0: 396BAE68  addi r11, r11, -0x5198
	ctx.r[11].s64 = ctx.r[11].s64 + -20888;
	// 825AAEF4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825AAEF8: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 825AAEFC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 825AAF00: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 825AAF04: 4BFFF97D  bl 0x825aa880
	ctx.lr = 0x825AAF08;
	sub_825AA880(ctx, base);
	// 825AAF08: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AAF0C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 825AAF10: 388BB300  addi r4, r11, -0x4d00
	ctx.r[4].s64 = ctx.r[11].s64 + -19712;
	// 825AAF14: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AAF18: 4802EFC9  bl 0x825d9ee0
	ctx.lr = 0x825AAF1C;
	sub_825D9EE0(ctx, base);
	// 825AAF1C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825AAF20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AAF24: 4802EEE5  bl 0x825d9e08
	ctx.lr = 0x825AAF28;
	sub_825D9E08(ctx, base);
	// 825AAF28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AAF2C: 4802EE9D  bl 0x825d9dc8
	ctx.lr = 0x825AAF30;
	sub_825D9DC8(ctx, base);
	// 825AAF30: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825AAF34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AAF38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AAF3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AAF40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AAF44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AAF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AAF48 size=132
    let mut pc: u32 = 0x825AAF48;
    'dispatch: loop {
        match pc {
            0x825AAF48 => {
    //   block [0x825AAF48..0x825AAFCC)
	// 825AAF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AAF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AAF50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AAF54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AAF58: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AAF5C: 3D60825B  lis r11, -0x7da5
	ctx.r[11].s64 = -2107965440;
	// 825AAF60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AAF64: 396BAEC0  addi r11, r11, -0x5140
	ctx.r[11].s64 = ctx.r[11].s64 + -20800;
	// 825AAF68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825AAF6C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 825AAF70: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825AAF74: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825AAF78: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825AAF7C: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825AAF80: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 825AAF84: 4BFFF8FD  bl 0x825aa880
	ctx.lr = 0x825AAF88;
	sub_825AA880(ctx, base);
	// 825AAF88: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AAF8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AAF90: 3BE10060  addi r31, r1, 0x60
	ctx.r[31].s64 = ctx.r[1].s64 + 96;
	// 825AAF94: 4BFFB4F5  bl 0x825a6488
	ctx.lr = 0x825AAF98;
	sub_825A6488(ctx, base);
	// 825AAF98: 48848219  bl 0x82df31b0
	ctx.lr = 0x825AAF9C;
	sub_82DF31B0(ctx, base);
	// 825AAF9C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AAFA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AAFA4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825AAFA8: 4802EF39  bl 0x825d9ee0
	ctx.lr = 0x825AAFAC;
	sub_825D9EE0(ctx, base);
	// 825AAFAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AAFB0: 48848479  bl 0x82df3428
	ctx.lr = 0x825AAFB4;
	sub_82DF3428(ctx, base);
	// 825AAFB4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825AAFB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AAFBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AAFC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AAFC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AAFC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AAFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AAFD0 size=16
    let mut pc: u32 = 0x825AAFD0;
    'dispatch: loop {
        match pc {
            0x825AAFD0 => {
    //   block [0x825AAFD0..0x825AAFE0)
	// 825AAFD0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AAFD4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825AAFD8: 994B006C  stb r10, 0x6c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(108 as u32), ctx.r[10].u8 ) };
	// 825AAFDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AAFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AAFE0 size=140
    let mut pc: u32 = 0x825AAFE0;
    'dispatch: loop {
        match pc {
            0x825AAFE0 => {
    //   block [0x825AAFE0..0x825AB06C)
	// 825AAFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AAFE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AAFE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AAFEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AAFF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AAFF4: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AAFF8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825AAFFC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AB000: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AB004: 814B0064  lwz r10, 0x64(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 825AB008: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 825AB00C: 409A0024  bne cr6, 0x825ab030
	if !ctx.cr[6].eq {
	pc = 0x825AB030; continue 'dispatch;
	}
	// 825AB010: 3BCB0068  addi r30, r11, 0x68
	ctx.r[30].s64 = ctx.r[11].s64 + 104;
	// 825AB014: 4BFFB475  bl 0x825a6488
	ctx.lr = 0x825AB018;
	sub_825A6488(ctx, base);
	// 825AB018: 48848199  bl 0x82df31b0
	ctx.lr = 0x825AB01C;
	sub_82DF31B0(ctx, base);
	// 825AB01C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AB020: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AB024: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825AB028: 4802EDE9  bl 0x825d9e10
	ctx.lr = 0x825AB02C;
	sub_825D9E10(ctx, base);
	// 825AB02C: 48000020  b 0x825ab04c
	pc = 0x825AB04C; continue 'dispatch;
	// 825AB030: 3BCB0054  addi r30, r11, 0x54
	ctx.r[30].s64 = ctx.r[11].s64 + 84;
	// 825AB034: 4BFFB455  bl 0x825a6488
	ctx.lr = 0x825AB038;
	sub_825A6488(ctx, base);
	// 825AB038: 48848179  bl 0x82df31b0
	ctx.lr = 0x825AB03C;
	sub_82DF31B0(ctx, base);
	// 825AB03C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AB040: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AB044: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825AB048: 4802EE09  bl 0x825d9e50
	ctx.lr = 0x825AB04C;
	sub_825D9E50(ctx, base);
	// 825AB04C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AB050: 488483D9  bl 0x82df3428
	ctx.lr = 0x825AB054;
	sub_82DF3428(ctx, base);
	// 825AB054: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AB058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AB05C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AB060: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AB064: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AB068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AB070 size=56
    let mut pc: u32 = 0x825AB070;
    'dispatch: loop {
        match pc {
            0x825AB070 => {
    //   block [0x825AB070..0x825AB0A8)
	// 825AB070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AB074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AB078: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AB07C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AB080: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB084: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AB088: 388B005C  addi r4, r11, 0x5c
	ctx.r[4].s64 = ctx.r[11].s64 + 92;
	// 825AB08C: 48848B75  bl 0x82df3c00
	ctx.lr = 0x825AB090;
	sub_82DF3C00(ctx, base);
	// 825AB090: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AB094: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AB098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AB09C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AB0A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AB0A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AB0A8 size=24
    let mut pc: u32 = 0x825AB0A8;
    'dispatch: loop {
        match pc {
            0x825AB0A8 => {
    //   block [0x825AB0A8..0x825AB0C0)
	// 825AB0A8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB0AC: 814B0050  lwz r10, 0x50(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AB0B0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825AB0B4: 419A000C  beq cr6, 0x825ab0c0
	if ctx.cr[6].eq {
		sub_825AB0C0(ctx, base);
		return;
	}
	// 825AB0B8: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB0BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AB0C0 size=8
    let mut pc: u32 = 0x825AB0C0;
    'dispatch: loop {
        match pc {
            0x825AB0C0 => {
    //   block [0x825AB0C0..0x825AB0C8)
	// 825AB0C0: 806B0054  lwz r3, 0x54(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 825AB0C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AB0C8 size=12
    let mut pc: u32 = 0x825AB0C8;
    'dispatch: loop {
        match pc {
            0x825AB0C8 => {
    //   block [0x825AB0C8..0x825AB0D4)
	// 825AB0C8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB0CC: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB0D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AB0D8 size=24
    let mut pc: u32 = 0x825AB0D8;
    'dispatch: loop {
        match pc {
            0x825AB0D8 => {
    //   block [0x825AB0D8..0x825AB0F0)
	// 825AB0D8: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB0DC: 816A0050  lwz r11, 0x50(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AB0E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AB0E4: 419A000C  beq cr6, 0x825ab0f0
	if ctx.cr[6].eq {
		sub_825AB0F0(ctx, base);
		return;
	}
	// 825AB0E8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB0EC: 48000008  b 0x825ab0f4
	sub_825AB0F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AB0F0 size=12
    let mut pc: u32 = 0x825AB0F0;
    'dispatch: loop {
        match pc {
            0x825AB0F0 => {
    //   block [0x825AB0F0..0x825AB0FC)
	// 825AB0F0: 816A0054  lwz r11, 0x54(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(84 as u32) ) } as u64;
	// 825AB0F4: 916A0058  stw r11, 0x58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 825AB0F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AB100 size=108
    let mut pc: u32 = 0x825AB100;
    'dispatch: loop {
        match pc {
            0x825AB100 => {
    //   block [0x825AB100..0x825AB16C)
	// 825AB100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AB104: 48BFD05D  bl 0x831a8160
	ctx.lr = 0x825AB108;
	sub_831A8130(ctx, base);
	// 825AB108: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AB10C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825AB110: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825AB114: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825AB118: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825AB11C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 825AB120: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 825AB124: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 825AB128: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 825AB12C: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 825AB130: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 825AB134: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 825AB138: 48846F91  bl 0x82df20c8
	ctx.lr = 0x825AB13C;
	sub_82DF20C8(ctx, base);
	// 825AB13C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825AB140: 41820020  beq 0x825ab160
	if ctx.cr[0].eq {
	pc = 0x825AB160; continue 'dispatch;
	}
	// 825AB144: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 825AB148: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 825AB14C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 825AB150: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825AB154: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825AB158: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AB15C: 4BEE70D5  bl 0x82492230
	ctx.lr = 0x825AB160;
	sub_82492230(ctx, base);
	// 825AB160: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AB164: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825AB168: 48BFD048  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AB170 size=60
    let mut pc: u32 = 0x825AB170;
    'dispatch: loop {
        match pc {
            0x825AB170 => {
    //   block [0x825AB170..0x825AB1AC)
	// 825AB170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AB174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AB178: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AB17C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AB180: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB184: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825AB188: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 825AB18C: 4BEC5F3D  bl 0x824710c8
	ctx.lr = 0x825AB190;
	sub_824710C8(ctx, base);
	// 825AB190: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AB194: 4BD1DB25  bl 0x822c8cb8
	ctx.lr = 0x825AB198;
	sub_822C8CB8(ctx, base);
	// 825AB198: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AB19C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AB1A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AB1A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AB1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AB1B0 size=120
    let mut pc: u32 = 0x825AB1B0;
    'dispatch: loop {
        match pc {
            0x825AB1B0 => {
    //   block [0x825AB1B0..0x825AB228)
	// 825AB1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AB1B4: 48BFCFB5  bl 0x831a8168
	ctx.lr = 0x825AB1B8;
	sub_831A8130(ctx, base);
	// 825AB1B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AB1BC: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB1C0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825AB1C4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 825AB1C8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825AB1CC: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB1D0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB1D4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825AB1D8: 4800001C  b 0x825ab1f4
	pc = 0x825AB1F4; continue 'dispatch;
	// 825AB1DC: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 825AB1E0: 419A0038  beq cr6, 0x825ab218
	if ctx.cr[6].eq {
	pc = 0x825AB218; continue 'dispatch;
	}
	// 825AB1E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AB1E8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 825AB1EC: 4BF3DEA5  bl 0x824e9090
	ctx.lr = 0x825AB1F0;
	sub_824E9090(ctx, base);
	// 825AB1F0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AB1F4: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 825AB1F8: 409AFFE4  bne cr6, 0x825ab1dc
	if !ctx.cr[6].eq {
	pc = 0x825AB1DC; continue 'dispatch;
	}
	// 825AB1FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825AB200: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825AB204: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 825AB208: 48848801  bl 0x82df3a08
	ctx.lr = 0x825AB20C;
	sub_82DF3A08(ctx, base);
	// 825AB20C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825AB210: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825AB214: 48BFCFA4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 825AB218: 388B0010  addi r4, r11, 0x10
	ctx.r[4].s64 = ctx.r[11].s64 + 16;
	// 825AB21C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825AB220: 488489E1  bl 0x82df3c00
	ctx.lr = 0x825AB224;
	sub_82DF3C00(ctx, base);
	// 825AB224: 4BFFFFE8  b 0x825ab20c
	pc = 0x825AB20C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AB228 size=92
    let mut pc: u32 = 0x825AB228;
    'dispatch: loop {
        match pc {
            0x825AB228 => {
    //   block [0x825AB228..0x825AB284)
	// 825AB228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AB22C: 48BFCF41  bl 0x831a816c
	ctx.lr = 0x825AB230;
	sub_831A8130(ctx, base);
	// 825AB230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AB234: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB238: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825AB23C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825AB240: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB244: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB248: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825AB24C: 4800001C  b 0x825ab268
	pc = 0x825AB268; continue 'dispatch;
	// 825AB250: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 825AB254: 419A0028  beq cr6, 0x825ab27c
	if ctx.cr[6].eq {
	pc = 0x825AB27C; continue 'dispatch;
	}
	// 825AB258: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AB25C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 825AB260: 4BF3DE31  bl 0x824e9090
	ctx.lr = 0x825AB264;
	sub_824E9090(ctx, base);
	// 825AB264: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AB268: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 825AB26C: 409AFFE4  bne cr6, 0x825ab250
	if !ctx.cr[6].eq {
	pc = 0x825AB250; continue 'dispatch;
	}
	// 825AB270: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825AB274: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825AB278: 48BFCF44  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 825AB27C: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825AB280: 4BFFFFF4  b 0x825ab274
	pc = 0x825AB274; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AB288 size=20
    let mut pc: u32 = 0x825AB288;
    'dispatch: loop {
        match pc {
            0x825AB288 => {
    //   block [0x825AB288..0x825AB29C)
	// 825AB288: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB28C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB290: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB294: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825AB298: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB29C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AB29C size=48
    let mut pc: u32 = 0x825AB29C;
    'dispatch: loop {
        match pc {
            0x825AB29C => {
    //   block [0x825AB29C..0x825AB2CC)
	// 825AB29C: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB2A0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825AB2A4: 81690050  lwz r11, 0x50(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AB2A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AB2AC: 419A0008  beq cr6, 0x825ab2b4
	if ctx.cr[6].eq {
	pc = 0x825AB2B4; continue 'dispatch;
	}
	// 825AB2B0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825AB2B4: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB2B8: 914B0054  stw r10, 0x54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 825AB2BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB2C0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825AB2C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AB2C8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB2CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AB2CC size=4
    let mut pc: u32 = 0x825AB2CC;
    'dispatch: loop {
        match pc {
            0x825AB2CC => {
    //   block [0x825AB2CC..0x825AB2D0)
	// 825AB2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AB2D0 size=100
    let mut pc: u32 = 0x825AB2D0;
    'dispatch: loop {
        match pc {
            0x825AB2D0 => {
    //   block [0x825AB2D0..0x825AB334)
	// 825AB2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AB2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AB2D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AB2DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AB2E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AB2E4: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 825AB2E8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 825AB2EC: 83C30008  lwz r30, 8(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB2F0: 38A1008C  addi r5, r1, 0x8c
	ctx.r[5].s64 = ctx.r[1].s64 + 140;
	// 825AB2F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AB2F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825AB2FC: 4BF70EAD  bl 0x8251c1a8
	ctx.lr = 0x825AB300;
	sub_8251C1A8(ctx, base);
	// 825AB300: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB304: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AB308: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825AB30C: 419A0010  beq cr6, 0x825ab31c
	if ctx.cr[6].eq {
	pc = 0x825AB31C; continue 'dispatch;
	}
	// 825AB310: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AB314: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 825AB318: 488488B9  bl 0x82df3bd0
	ctx.lr = 0x825AB31C;
	sub_82DF3BD0(ctx, base);
	// 825AB31C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AB320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AB324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AB328: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AB32C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AB330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825AB338 size=216
    let mut pc: u32 = 0x825AB338;
    'dispatch: loop {
        match pc {
            0x825AB338 => {
    //   block [0x825AB338..0x825AB410)
	// 825AB338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AB33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AB340: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AB344: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AB348: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AB34C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AB350: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825AB354: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB358: 896B006C  lbz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 825AB35C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825AB360: 41820014  beq 0x825ab374
	if ctx.cr[0].eq {
	pc = 0x825AB374; continue 'dispatch;
	}
	// 825AB364: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB368: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AB36C: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 825AB370: 48000078  b 0x825ab3e8
	pc = 0x825AB3E8; continue 'dispatch;
	// 825AB374: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825AB378: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB37C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AB380: C02B08A8  lfs f1, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825AB384: 816A0034  lwz r11, 0x34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 825AB388: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AB38C: 4E800421  bctrl
	ctx.lr = 0x825AB390;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AB390: 83FF0008  lwz r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB394: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AB398: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AB39C: 419A000C  beq cr6, 0x825ab3a8
	if ctx.cr[6].eq {
	pc = 0x825AB3A8; continue 'dispatch;
	}
	// 825AB3A0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB3A4: 48000008  b 0x825ab3ac
	pc = 0x825AB3AC; continue 'dispatch;
	// 825AB3A8: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 825AB3AC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825AB3B0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825AB3B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AB3B8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825AB3BC: 4BF70DED  bl 0x8251c1a8
	ctx.lr = 0x825AB3C0;
	sub_8251C1A8(ctx, base);
	// 825AB3C0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB3C4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825AB3C8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825AB3CC: 419A0028  beq cr6, 0x825ab3f4
	if ctx.cr[6].eq {
	pc = 0x825AB3F4; continue 'dispatch;
	}
	// 825AB3D0: 388B0010  addi r4, r11, 0x10
	ctx.r[4].s64 = ctx.r[11].s64 + 16;
	// 825AB3D4: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB3D8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825AB3DC: 48848825  bl 0x82df3c00
	ctx.lr = 0x825AB3E0;
	sub_82DF3C00(ctx, base);
	// 825AB3E0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AB3E4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AB3E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AB3EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AB3F0: 4E800421  bctrl
	ctx.lr = 0x825AB3F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AB3F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825AB3F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AB3FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AB400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AB404: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AB408: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AB40C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AB410 size=144
    let mut pc: u32 = 0x825AB410;
    'dispatch: loop {
        match pc {
            0x825AB410 => {
    //   block [0x825AB410..0x825AB4A0)
	// 825AB410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AB414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AB418: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AB41C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AB420: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AB424: 83E40008  lwz r31, 8(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB428: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825AB42C: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AB430: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AB434: 419A000C  beq cr6, 0x825ab440
	if ctx.cr[6].eq {
	pc = 0x825AB440; continue 'dispatch;
	}
	// 825AB438: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB43C: 48000008  b 0x825ab444
	pc = 0x825AB444; continue 'dispatch;
	// 825AB440: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 825AB444: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825AB448: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825AB44C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AB450: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825AB454: 4BF70D55  bl 0x8251c1a8
	ctx.lr = 0x825AB458;
	sub_8251C1A8(ctx, base);
	// 825AB458: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB45C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB460: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AB464: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825AB468: 419A0010  beq cr6, 0x825ab478
	if ctx.cr[6].eq {
	pc = 0x825AB478; continue 'dispatch;
	}
	// 825AB46C: 388B0010  addi r4, r11, 0x10
	ctx.r[4].s64 = ctx.r[11].s64 + 16;
	// 825AB470: 48848791  bl 0x82df3c00
	ctx.lr = 0x825AB474;
	sub_82DF3C00(ctx, base);
	// 825AB474: 48000010  b 0x825ab484
	pc = 0x825AB484; continue 'dispatch;
	// 825AB478: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825AB47C: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 825AB480: 48848589  bl 0x82df3a08
	ctx.lr = 0x825AB484;
	sub_82DF3A08(ctx, base);
	// 825AB484: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AB488: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AB48C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AB490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AB494: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AB498: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AB49C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AB4A0 size=124
    let mut pc: u32 = 0x825AB4A0;
    'dispatch: loop {
        match pc {
            0x825AB4A0 => {
    //   block [0x825AB4A0..0x825AB51C)
	// 825AB4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AB4A4: 48BFCCC9  bl 0x831a816c
	ctx.lr = 0x825AB4A8;
	sub_831A8130(ctx, base);
	// 825AB4A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AB4AC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825AB4B0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825AB4B4: 83FE0008  lwz r31, 8(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB4B8: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AB4BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AB4C0: 419A000C  beq cr6, 0x825ab4cc
	if ctx.cr[6].eq {
	pc = 0x825AB4CC; continue 'dispatch;
	}
	// 825AB4C4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB4C8: 48000008  b 0x825ab4d0
	pc = 0x825AB4D0; continue 'dispatch;
	// 825AB4CC: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 825AB4D0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825AB4D4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825AB4D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AB4DC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825AB4E0: 4BF70CC9  bl 0x8251c1a8
	ctx.lr = 0x825AB4E4;
	sub_8251C1A8(ctx, base);
	// 825AB4E4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB4E8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825AB4EC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825AB4F0: 419A0010  beq cr6, 0x825ab500
	if ctx.cr[6].eq {
	pc = 0x825AB500; continue 'dispatch;
	}
	// 825AB4F4: 388B0010  addi r4, r11, 0x10
	ctx.r[4].s64 = ctx.r[11].s64 + 16;
	// 825AB4F8: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 825AB4FC: 488486D5  bl 0x82df3bd0
	ctx.lr = 0x825AB500;
	sub_82DF3BD0(ctx, base);
	// 825AB500: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB504: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825AB508: 388B0068  addi r4, r11, 0x68
	ctx.r[4].s64 = ctx.r[11].s64 + 104;
	// 825AB50C: 488486F5  bl 0x82df3c00
	ctx.lr = 0x825AB510;
	sub_82DF3C00(ctx, base);
	// 825AB510: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825AB514: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825AB518: 48BFCCA4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AB520 size=160
    let mut pc: u32 = 0x825AB520;
    'dispatch: loop {
        match pc {
            0x825AB520 => {
    //   block [0x825AB520..0x825AB5C0)
	// 825AB520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AB524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AB528: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AB52C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AB530: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AB534: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825AB538: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB53C: 814B0064  lwz r10, 0x64(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 825AB540: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 825AB544: 409A0064  bne cr6, 0x825ab5a8
	if !ctx.cr[6].eq {
	pc = 0x825AB5A8; continue 'dispatch;
	}
	// 825AB548: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB54C: 83EA0000  lwz r31, 0(r10)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB550: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 825AB554: 4800004C  b 0x825ab5a0
	pc = 0x825AB5A0; continue 'dispatch;
	// 825AB558: 388B0068  addi r4, r11, 0x68
	ctx.r[4].s64 = ctx.r[11].s64 + 104;
	// 825AB55C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 825AB560: 48847DA9  bl 0x82df3308
	ctx.lr = 0x825AB564;
	sub_82DF3308(ctx, base);
	// 825AB564: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AB568: 41820024  beq 0x825ab58c
	if ctx.cr[0].eq {
	pc = 0x825AB58C; continue 'dispatch;
	}
	// 825AB56C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB570: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825AB574: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AB578: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AB57C: 419A0008  beq cr6, 0x825ab584
	if ctx.cr[6].eq {
	pc = 0x825AB584; continue 'dispatch;
	}
	// 825AB580: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825AB584: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB588: 914B0054  stw r10, 0x54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 825AB58C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AB590: 4BF3DB01  bl 0x824e9090
	ctx.lr = 0x825AB594;
	sub_824E9090(ctx, base);
	// 825AB594: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB598: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB59C: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AB5A0: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825AB5A4: 409AFFB4  bne cr6, 0x825ab558
	if !ctx.cr[6].eq {
	pc = 0x825AB558; continue 'dispatch;
	}
	// 825AB5A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AB5AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AB5B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AB5B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AB5B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AB5BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AB5C0 size=8
    let mut pc: u32 = 0x825AB5C0;
    'dispatch: loop {
        match pc {
            0x825AB5C0 => {
    //   block [0x825AB5C0..0x825AB5C8)
	// 825AB5C0: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB5C4: 483A84D4  b 0x82953a98
	sub_82953A98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AB5C8 size=1024
    let mut pc: u32 = 0x825AB5C8;
    'dispatch: loop {
        match pc {
            0x825AB5C8 => {
    //   block [0x825AB5C8..0x825AB9C8)
	// 825AB5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AB5CC: 48BFCB8D  bl 0x831a8158
	ctx.lr = 0x825AB5D0;
	sub_831A8130(ctx, base);
	// 825AB5D0: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AB5D4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 825AB5D8: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 825AB5DC: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 825AB5E0: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 825AB5E4: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 825AB5E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AB5EC: 419A0048  beq cr6, 0x825ab634
	if ctx.cr[6].eq {
	pc = 0x825AB634; continue 'dispatch;
	}
	// 825AB5F0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825AB5F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AB5F8: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 825AB5FC: 4BD1A2CD  bl 0x822c58c8
	ctx.lr = 0x825AB600;
	sub_822C58C8(ctx, base);
	// 825AB600: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825AB604: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825AB608: 4BD1E8A9  bl 0x822c9eb0
	ctx.lr = 0x825AB60C;
	sub_822C9EB0(ctx, base);
	// 825AB60C: 4BD18CA5  bl 0x822c42b0
	ctx.lr = 0x825AB610;
	sub_822C42B0(ctx, base);
	// 825AB610: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825AB614: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825AB618: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 825AB61C: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 825AB620: 4BD19E51  bl 0x822c5470
	ctx.lr = 0x825AB624;
	sub_822C5470(ctx, base);
	// 825AB624: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825AB628: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825AB62C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AB630: 4BD196B1  bl 0x822c4ce0
	ctx.lr = 0x825AB634;
	sub_822C4CE0(ctx, base);
	// 825AB634: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 825AB638: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 825AB63C: 4BF3DA55  bl 0x824e9090
	ctx.lr = 0x825AB640;
	sub_824E9090(ctx, base);
	// 825AB640: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB644: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 825AB648: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825AB64C: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 825AB650: 419A000C  beq cr6, 0x825ab65c
	if ctx.cr[6].eq {
	pc = 0x825AB65C; continue 'dispatch;
	}
	// 825AB654: 839B0008  lwz r28, 8(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB658: 48000028  b 0x825ab680
	pc = 0x825AB680; continue 'dispatch;
	// 825AB65C: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB660: 894A0015  lbz r10, 0x15(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(21 as u32) ) } as u64;
	// 825AB664: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825AB668: 419A000C  beq cr6, 0x825ab674
	if ctx.cr[6].eq {
	pc = 0x825AB674; continue 'dispatch;
	}
	// 825AB66C: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 825AB670: 48000010  b 0x825ab680
	pc = 0x825AB680; continue 'dispatch;
	// 825AB674: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB678: 7F19D840  cmplw cr6, r25, r27
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[27].u32, &mut ctx.xer);
	// 825AB67C: 409A00DC  bne cr6, 0x825ab758
	if !ctx.cr[6].eq {
	pc = 0x825AB758; continue 'dispatch;
	}
	// 825AB680: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 825AB684: 83FB0004  lwz r31, 4(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB688: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AB68C: 409A0008  bne cr6, 0x825ab694
	if !ctx.cr[6].eq {
	pc = 0x825AB694; continue 'dispatch;
	}
	// 825AB690: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 825AB694: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB698: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB69C: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 825AB6A0: 409A000C  bne cr6, 0x825ab6ac
	if !ctx.cr[6].eq {
	pc = 0x825AB6AC; continue 'dispatch;
	}
	// 825AB6A4: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 825AB6A8: 4800001C  b 0x825ab6c4
	pc = 0x825AB6C4; continue 'dispatch;
	// 825AB6AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB6B0: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 825AB6B4: 409A000C  bne cr6, 0x825ab6c0
	if !ctx.cr[6].eq {
	pc = 0x825AB6C0; continue 'dispatch;
	}
	// 825AB6B8: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 825AB6BC: 48000008  b 0x825ab6c4
	pc = 0x825AB6C4; continue 'dispatch;
	// 825AB6C0: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 825AB6C4: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB6C8: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB6CC: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 825AB6D0: 409A003C  bne cr6, 0x825ab70c
	if !ctx.cr[6].eq {
	pc = 0x825AB70C; continue 'dispatch;
	}
	// 825AB6D4: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 825AB6D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AB6DC: 419A000C  beq cr6, 0x825ab6e8
	if ctx.cr[6].eq {
	pc = 0x825AB6E8; continue 'dispatch;
	}
	// 825AB6E0: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 825AB6E4: 48000024  b 0x825ab708
	pc = 0x825AB708; continue 'dispatch;
	// 825AB6E8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB6EC: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 825AB6F0: 4800000C  b 0x825ab6fc
	pc = 0x825AB6FC; continue 'dispatch;
	// 825AB6F4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 825AB6F8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB6FC: 890B0015  lbz r8, 0x15(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 825AB700: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 825AB704: 419AFFF0  beq cr6, 0x825ab6f4
	if ctx.cr[6].eq {
	pc = 0x825AB6F4; continue 'dispatch;
	}
	// 825AB708: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825AB70C: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB710: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB714: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 825AB718: 409A00D4  bne cr6, 0x825ab7ec
	if !ctx.cr[6].eq {
	pc = 0x825AB7EC; continue 'dispatch;
	}
	// 825AB71C: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 825AB720: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AB724: 419A000C  beq cr6, 0x825ab730
	if ctx.cr[6].eq {
	pc = 0x825AB730; continue 'dispatch;
	}
	// 825AB728: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 825AB72C: 48000024  b 0x825ab750
	pc = 0x825AB750; continue 'dispatch;
	// 825AB730: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB734: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 825AB738: 4800000C  b 0x825ab744
	pc = 0x825AB744; continue 'dispatch;
	// 825AB73C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 825AB740: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB744: 890B0015  lbz r8, 0x15(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 825AB748: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 825AB74C: 419AFFF0  beq cr6, 0x825ab73c
	if ctx.cr[6].eq {
	pc = 0x825AB73C; continue 'dispatch;
	}
	// 825AB750: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825AB754: 48000098  b 0x825ab7ec
	pc = 0x825AB7EC; continue 'dispatch;
	// 825AB758: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 825AB75C: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB760: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AB764: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB768: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AB76C: 409A000C  bne cr6, 0x825ab778
	if !ctx.cr[6].eq {
	pc = 0x825AB778; continue 'dispatch;
	}
	// 825AB770: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 825AB774: 4800002C  b 0x825ab7a0
	pc = 0x825AB7A0; continue 'dispatch;
	// 825AB778: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 825AB77C: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB780: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AB784: 409A0008  bne cr6, 0x825ab78c
	if !ctx.cr[6].eq {
	pc = 0x825AB78C; continue 'dispatch;
	}
	// 825AB788: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 825AB78C: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 825AB790: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB794: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825AB798: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB79C: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 825AB7A0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB7A4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB7A8: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 825AB7AC: 409A000C  bne cr6, 0x825ab7b8
	if !ctx.cr[6].eq {
	pc = 0x825AB7B8; continue 'dispatch;
	}
	// 825AB7B0: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 825AB7B4: 48000020  b 0x825ab7d4
	pc = 0x825AB7D4; continue 'dispatch;
	// 825AB7B8: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB7BC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB7C0: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 825AB7C4: 409A000C  bne cr6, 0x825ab7d0
	if !ctx.cr[6].eq {
	pc = 0x825AB7D0; continue 'dispatch;
	}
	// 825AB7C8: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 825AB7CC: 48000008  b 0x825ab7d4
	pc = 0x825AB7D4; continue 'dispatch;
	// 825AB7D0: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 825AB7D4: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB7D8: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825AB7DC: 897B0014  lbz r11, 0x14(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 825AB7E0: 89590014  lbz r10, 0x14(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(20 as u32) ) } as u64;
	// 825AB7E4: 99790014  stb r11, 0x14(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 825AB7E8: 995B0014  stb r10, 0x14(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 825AB7EC: 897B0014  lbz r11, 0x14(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 825AB7F0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 825AB7F4: 409A0198  bne cr6, 0x825ab98c
	if !ctx.cr[6].eq {
	pc = 0x825AB98C; continue 'dispatch;
	}
	// 825AB7F8: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB7FC: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 825AB800: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB804: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AB808: 419A0180  beq cr6, 0x825ab988
	if ctx.cr[6].eq {
	pc = 0x825AB988; continue 'dispatch;
	}
	// 825AB80C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825AB810: 897C0014  lbz r11, 0x14(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 825AB814: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 825AB818: 409A0170  bne cr6, 0x825ab988
	if !ctx.cr[6].eq {
	pc = 0x825AB988; continue 'dispatch;
	}
	// 825AB81C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB820: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AB824: 409A00A8  bne cr6, 0x825ab8cc
	if !ctx.cr[6].eq {
	pc = 0x825AB8CC; continue 'dispatch;
	}
	// 825AB828: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB82C: 894B0014  lbz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825AB830: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825AB834: 409A001C  bne cr6, 0x825ab850
	if !ctx.cr[6].eq {
	pc = 0x825AB850; continue 'dispatch;
	}
	// 825AB838: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 825AB83C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AB840: 9BBF0014  stb r29, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 825AB844: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825AB848: 4860B389  bl 0x82bb6bd0
	ctx.lr = 0x825AB84C;
	sub_82BB6BD0(ctx, base);
	// 825AB84C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB850: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 825AB854: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825AB858: 409A00C8  bne cr6, 0x825ab920
	if !ctx.cr[6].eq {
	pc = 0x825AB920; continue 'dispatch;
	}
	// 825AB85C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB860: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 825AB864: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 825AB868: 409A0014  bne cr6, 0x825ab87c
	if !ctx.cr[6].eq {
	pc = 0x825AB87C; continue 'dispatch;
	}
	// 825AB86C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB870: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 825AB874: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 825AB878: 419A00A4  beq cr6, 0x825ab91c
	if ctx.cr[6].eq {
	pc = 0x825AB91C; continue 'dispatch;
	}
	// 825AB87C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB880: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 825AB884: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 825AB888: 409A0020  bne cr6, 0x825ab8a8
	if !ctx.cr[6].eq {
	pc = 0x825AB8A8; continue 'dispatch;
	}
	// 825AB88C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB890: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825AB894: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825AB898: 9BCA0014  stb r30, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 825AB89C: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 825AB8A0: 4BD42C71  bl 0x822ee510
	ctx.lr = 0x825AB8A4;
	sub_822EE510(ctx, base);
	// 825AB8A4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB8A8: 895F0014  lbz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 825AB8AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AB8B0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825AB8B4: 994B0014  stb r10, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 825AB8B8: 9BDF0014  stb r30, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 825AB8BC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB8C0: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 825AB8C4: 4860B30D  bl 0x82bb6bd0
	ctx.lr = 0x825AB8C8;
	sub_82BB6BD0(ctx, base);
	// 825AB8C8: 480000C0  b 0x825ab988
	pc = 0x825AB988; continue 'dispatch;
	// 825AB8CC: 894B0014  lbz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825AB8D0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825AB8D4: 409A001C  bne cr6, 0x825ab8f0
	if !ctx.cr[6].eq {
	pc = 0x825AB8F0; continue 'dispatch;
	}
	// 825AB8D8: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 825AB8DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AB8E0: 9BBF0014  stb r29, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 825AB8E4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825AB8E8: 4BD42C29  bl 0x822ee510
	ctx.lr = 0x825AB8EC;
	sub_822EE510(ctx, base);
	// 825AB8EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB8F0: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 825AB8F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825AB8F8: 409A0028  bne cr6, 0x825ab920
	if !ctx.cr[6].eq {
	pc = 0x825AB920; continue 'dispatch;
	}
	// 825AB8FC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB900: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 825AB904: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 825AB908: 409A0034  bne cr6, 0x825ab93c
	if !ctx.cr[6].eq {
	pc = 0x825AB93C; continue 'dispatch;
	}
	// 825AB90C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB910: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 825AB914: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 825AB918: 409A0024  bne cr6, 0x825ab93c
	if !ctx.cr[6].eq {
	pc = 0x825AB93C; continue 'dispatch;
	}
	// 825AB91C: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 825AB920: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB924: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 825AB928: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB92C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AB930: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AB934: 409AFEDC  bne cr6, 0x825ab810
	if !ctx.cr[6].eq {
	pc = 0x825AB810; continue 'dispatch;
	}
	// 825AB938: 48000050  b 0x825ab988
	pc = 0x825AB988; continue 'dispatch;
	// 825AB93C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB940: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 825AB944: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 825AB948: 409A0020  bne cr6, 0x825ab968
	if !ctx.cr[6].eq {
	pc = 0x825AB968; continue 'dispatch;
	}
	// 825AB94C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB950: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825AB954: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825AB958: 9BCA0014  stb r30, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 825AB95C: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 825AB960: 4860B271  bl 0x82bb6bd0
	ctx.lr = 0x825AB964;
	sub_82BB6BD0(ctx, base);
	// 825AB964: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB968: 895F0014  lbz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 825AB96C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AB970: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825AB974: 994B0014  stb r10, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 825AB978: 9BDF0014  stb r30, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 825AB97C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AB980: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 825AB984: 4BD42B8D  bl 0x822ee510
	ctx.lr = 0x825AB988;
	sub_822EE510(ctx, base);
	// 825AB988: 9BDC0014  stb r30, 0x14(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 825AB98C: 387B0010  addi r3, r27, 0x10
	ctx.r[3].s64 = ctx.r[27].s64 + 16;
	// 825AB990: 48847A99  bl 0x82df3428
	ctx.lr = 0x825AB994;
	sub_82DF3428(ctx, base);
	// 825AB994: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825AB998: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 825AB99C: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 825AB9A0: 488467E9  bl 0x82df2188
	ctx.lr = 0x825AB9A4;
	sub_82DF2188(ctx, base);
	// 825AB9A4: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB9A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AB9AC: 419A000C  beq cr6, 0x825ab9b8
	if ctx.cr[6].eq {
	pc = 0x825AB9B8; continue 'dispatch;
	}
	// 825AB9B0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825AB9B4: 917A0008  stw r11, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825AB9B8: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 825AB9BC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 825AB9C0: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 825AB9C4: 48BFC7E4  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AB9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AB9C8 size=548
    let mut pc: u32 = 0x825AB9C8;
    'dispatch: loop {
        match pc {
            0x825AB9C8 => {
    //   block [0x825AB9C8..0x825ABBEC)
	// 825AB9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AB9CC: 48BFC795  bl 0x831a8160
	ctx.lr = 0x825AB9D0;
	sub_831A8130(ctx, base);
	// 825AB9D0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AB9D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825AB9D8: 3D601FFF  lis r11, 0x1fff
	ctx.r[11].s64 = 536805376;
	// 825AB9DC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 825AB9E0: 616BFFFE  ori r11, r11, 0xfffe
	ctx.r[11].u64 = ctx.r[11].u64 | 65534;
	// 825AB9E4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 825AB9E8: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AB9EC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 825AB9F0: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 825AB9F4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AB9F8: 41980048  blt cr6, 0x825aba40
	if ctx.cr[6].lt {
	pc = 0x825ABA40; continue 'dispatch;
	}
	// 825AB9FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825ABA00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825ABA04: 388B9BCC  addi r4, r11, -0x6434
	ctx.r[4].s64 = ctx.r[11].s64 + -25652;
	// 825ABA08: 4BD19EC1  bl 0x822c58c8
	ctx.lr = 0x825ABA0C;
	sub_822C58C8(ctx, base);
	// 825ABA0C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825ABA10: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825ABA14: 4BD19E05  bl 0x822c5818
	ctx.lr = 0x825ABA18;
	sub_822C5818(ctx, base);
	// 825ABA18: 4BD18899  bl 0x822c42b0
	ctx.lr = 0x825ABA1C;
	sub_822C42B0(ctx, base);
	// 825ABA1C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825ABA20: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825ABA24: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 825ABA28: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 825ABA2C: 4BD19A45  bl 0x822c5470
	ctx.lr = 0x825ABA30;
	sub_822C5470(ctx, base);
	// 825ABA30: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825ABA34: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825ABA38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825ABA3C: 4BD192A5  bl 0x822c4ce0
	ctx.lr = 0x825ABA40;
	sub_822C4CE0(ctx, base);
	// 825ABA40: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABA44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ABA48: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 825ABA4C: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 825ABA50: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825ABA54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825ABA58: 4BFFF6A9  bl 0x825ab100
	ctx.lr = 0x825ABA5C;
	sub_825AB100(ctx, base);
	// 825ABA5C: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ABA60: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABA64: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825ABA68: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825ABA6C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825ABA70: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825ABA74: 409A0018  bne cr6, 0x825aba8c
	if !ctx.cr[6].eq {
	pc = 0x825ABA8C; continue 'dispatch;
	}
	// 825ABA78: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 825ABA7C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABA80: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 825ABA84: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABA88: 4800003C  b 0x825abac4
	pc = 0x825ABAC4; continue 'dispatch;
	// 825ABA8C: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825ABA90: 41820020  beq 0x825abab0
	if ctx.cr[0].eq {
	pc = 0x825ABAB0; continue 'dispatch;
	}
	// 825ABA94: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 825ABA98: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABA9C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ABAA0: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825ABAA4: 409A0024  bne cr6, 0x825abac8
	if !ctx.cr[6].eq {
	pc = 0x825ABAC8; continue 'dispatch;
	}
	// 825ABAA8: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 825ABAAC: 4800001C  b 0x825abac8
	pc = 0x825ABAC8; continue 'dispatch;
	// 825ABAB0: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 825ABAB4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABAB8: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ABABC: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825ABAC0: 409A0008  bne cr6, 0x825abac8
	if !ctx.cr[6].eq {
	pc = 0x825ABAC8; continue 'dispatch;
	}
	// 825ABAC4: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 825ABAC8: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABACC: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 825ABAD0: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 825ABAD4: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 825ABAD8: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 825ABADC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825ABAE0: 409A00F0  bne cr6, 0x825abbd0
	if !ctx.cr[6].eq {
	pc = 0x825ABBD0; continue 'dispatch;
	}
	// 825ABAE4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 825ABAE8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ABAEC: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABAF0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ABAF4: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825ABAF8: 409A0054  bne cr6, 0x825abb4c
	if !ctx.cr[6].eq {
	pc = 0x825ABB4C; continue 'dispatch;
	}
	// 825ABAFC: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ABB00: 892A0014  lbz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 825ABB04: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 825ABB08: 419A0054  beq cr6, 0x825abb5c
	if ctx.cr[6].eq {
	pc = 0x825ABB5C; continue 'dispatch;
	}
	// 825ABB0C: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ABB10: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825ABB14: 409A0010  bne cr6, 0x825abb24
	if !ctx.cr[6].eq {
	pc = 0x825ABB24; continue 'dispatch;
	}
	// 825ABB18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825ABB1C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825ABB20: 4860B0B1  bl 0x82bb6bd0
	ctx.lr = 0x825ABB24;
	sub_82BB6BD0(ctx, base);
	// 825ABB24: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABB28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825ABB2C: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 825ABB30: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABB34: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABB38: 9B6B0014  stb r27, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[27].u8 ) };
	// 825ABB3C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABB40: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABB44: 4BD429CD  bl 0x822ee510
	ctx.lr = 0x825ABB48;
	sub_822EE510(ctx, base);
	// 825ABB48: 48000074  b 0x825abbbc
	pc = 0x825ABBBC; continue 'dispatch;
	// 825ABB4C: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ABB50: 892A0014  lbz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 825ABB54: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 825ABB58: 409A0028  bne cr6, 0x825abb80
	if !ctx.cr[6].eq {
	pc = 0x825ABB80; continue 'dispatch;
	}
	// 825ABB5C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ABB60: 9BA90014  stb r29, 0x14(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 825ABB64: 9BAA0014  stb r29, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 825ABB68: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ABB6C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABB70: 9B6A0014  stb r27, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[27].u8 ) };
	// 825ABB74: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ABB78: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABB7C: 48000040  b 0x825abbbc
	pc = 0x825ABBBC; continue 'dispatch;
	// 825ABB80: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ABB84: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825ABB88: 409A0010  bne cr6, 0x825abb98
	if !ctx.cr[6].eq {
	pc = 0x825ABB98; continue 'dispatch;
	}
	// 825ABB8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825ABB90: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825ABB94: 4BD4297D  bl 0x822ee510
	ctx.lr = 0x825ABB98;
	sub_822EE510(ctx, base);
	// 825ABB98: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABB9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825ABBA0: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 825ABBA4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABBA8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABBAC: 9B6B0014  stb r27, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[27].u8 ) };
	// 825ABBB0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABBB4: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABBB8: 4860B019  bl 0x82bb6bd0
	ctx.lr = 0x825ABBBC;
	sub_82BB6BD0(ctx, base);
	// 825ABBBC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABBC0: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 825ABBC4: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 825ABBC8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825ABBCC: 419AFF1C  beq cr6, 0x825abae8
	if ctx.cr[6].eq {
	pc = 0x825ABAE8; continue 'dispatch;
	}
	// 825ABBD0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABBD4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825ABBD8: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 825ABBDC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABBE0: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 825ABBE4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 825ABBE8: 48BFC5C8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ABBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ABBF0 size=132
    let mut pc: u32 = 0x825ABBF0;
    'dispatch: loop {
        match pc {
            0x825ABBF0 => {
    //   block [0x825ABBF0..0x825ABC74)
	// 825ABBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ABBF4: 48BFC575  bl 0x831a8168
	ctx.lr = 0x825ABBF8;
	sub_831A8130(ctx, base);
	// 825ABBF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ABBFC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825ABC00: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 825ABC04: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825ABC08: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 825ABC0C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABC10: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ABC14: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825ABC18: 409A0044  bne cr6, 0x825abc5c
	if !ctx.cr[6].eq {
	pc = 0x825ABC5C; continue 'dispatch;
	}
	// 825ABC1C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825ABC20: 409A003C  bne cr6, 0x825abc5c
	if !ctx.cr[6].eq {
	pc = 0x825ABC5C; continue 'dispatch;
	}
	// 825ABC24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825ABC28: 483A7E71  bl 0x82953a98
	ctx.lr = 0x825ABC2C;
	sub_82953A98(ctx, base);
	// 825ABC2C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABC30: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ABC34: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825ABC38: 48000030  b 0x825abc68
	pc = 0x825ABC68; continue 'dispatch;
	// 825ABC3C: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 825ABC40: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 825ABC44: 4BF3D44D  bl 0x824e9090
	ctx.lr = 0x825ABC48;
	sub_824E9090(ctx, base);
	// 825ABC48: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 825ABC4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825ABC50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825ABC54: 4BFFF975  bl 0x825ab5c8
	ctx.lr = 0x825ABC58;
	sub_825AB5C8(ctx, base);
	// 825ABC58: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 825ABC5C: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 825ABC60: 409AFFDC  bne cr6, 0x825abc3c
	if !ctx.cr[6].eq {
	pc = 0x825ABC3C; continue 'dispatch;
	}
	// 825ABC64: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 825ABC68: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825ABC6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ABC70: 48BFC548  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ABC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825ABC78 size=236
    let mut pc: u32 = 0x825ABC78;
    'dispatch: loop {
        match pc {
            0x825ABC78 => {
    //   block [0x825ABC78..0x825ABD64)
	// 825ABC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ABC7C: 48BFC4E5  bl 0x831a8160
	ctx.lr = 0x825ABC80;
	sub_831A8130(ctx, base);
	// 825ABC80: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ABC84: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 825ABC88: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 825ABC8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825ABC90: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 825ABC94: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 825ABC98: 83DC0004  lwz r30, 4(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABC9C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABCA0: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 825ABCA4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825ABCA8: 409A0038  bne cr6, 0x825abce0
	if !ctx.cr[6].eq {
	pc = 0x825ABCE0; continue 'dispatch;
	}
	// 825ABCAC: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ABCB0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825ABCB4: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 825ABCB8: 7D295010  subfc r9, r9, r10
	ctx.xer.ca = ctx.r[10].u32 >= ctx.r[9].u32;
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 825ABCBC: 7D294910  subfe r9, r9, r9
	let x = (!ctx.r[9].u32);
	let y = ctx.r[9].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[9].u32 = res;
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 825ABCC0: 553D07FF  clrlwi. r29, r9, 0x1f
	ctx.r[29].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 825ABCC4: 4182000C  beq 0x825abcd0
	if ctx.cr[0].eq {
	pc = 0x825ABCD0; continue 'dispatch;
	}
	// 825ABCC8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ABCCC: 48000008  b 0x825abcd4
	pc = 0x825ABCD4; continue 'dispatch;
	// 825ABCD0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ABCD4: 892B0015  lbz r9, 0x15(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 825ABCD8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 825ABCDC: 419AFFD4  beq cr6, 0x825abcb0
	if ctx.cr[6].eq {
	pc = 0x825ABCB0; continue 'dispatch;
	}
	// 825ABCE0: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 825ABCE4: 57AA063F  clrlwi. r10, r29, 0x18
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825ABCE8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825ABCEC: 41820044  beq 0x825abd30
	if ctx.cr[0].eq {
	pc = 0x825ABD30; continue 'dispatch;
	}
	// 825ABCF0: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABCF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825ABCF8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ABCFC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825ABD00: 409A0028  bne cr6, 0x825abd28
	if !ctx.cr[6].eq {
	pc = 0x825ABD28; continue 'dispatch;
	}
	// 825ABD04: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 825ABD08: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825ABD0C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 825ABD10: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 825ABD14: 4BFFFCB5  bl 0x825ab9c8
	ctx.lr = 0x825ABD18;
	sub_825AB9C8(ctx, base);
	// 825ABD18: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825ABD1C: 9B5F0004  stb r26, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u8 ) };
	// 825ABD20: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ABD24: 48000030  b 0x825abd54
	pc = 0x825ABD54; continue 'dispatch;
	// 825ABD28: 4860AF11  bl 0x82bb6c38
	ctx.lr = 0x825ABD2C;
	sub_82BB6C38(ctx, base);
	// 825ABD2C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825ABD30: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825ABD34: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ABD38: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825ABD3C: 40980010  bge cr6, 0x825abd4c
	if !ctx.cr[6].lt {
	pc = 0x825ABD4C; continue 'dispatch;
	}
	// 825ABD40: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825ABD44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825ABD48: 4BFFFFC0  b 0x825abd08
	pc = 0x825ABD08; continue 'dispatch;
	// 825ABD4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ABD50: 995F0004  stb r10, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 825ABD54: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825ABD58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825ABD5C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825ABD60: 48BFC450  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ABD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ABD68 size=88
    let mut pc: u32 = 0x825ABD68;
    'dispatch: loop {
        match pc {
            0x825ABD68 => {
    //   block [0x825ABD68..0x825ABDC0)
	// 825ABD68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ABD6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ABD70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825ABD74: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ABD78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825ABD7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825ABD80: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825ABD84: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABD88: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ABD8C: 4BFFFE65  bl 0x825abbf0
	ctx.lr = 0x825ABD90;
	sub_825ABBF0(ctx, base);
	// 825ABD90: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825ABD94: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ABD98: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 825ABD9C: 488463ED  bl 0x82df2188
	ctx.lr = 0x825ABDA0;
	sub_82DF2188(ctx, base);
	// 825ABDA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ABDA4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825ABDA8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825ABDAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825ABDB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ABDB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ABDB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825ABDBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ABDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ABDC0 size=128
    let mut pc: u32 = 0x825ABDC0;
    'dispatch: loop {
        match pc {
            0x825ABDC0 => {
    //   block [0x825ABDC0..0x825ABE40)
	// 825ABDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ABDC4: 48BFC3A9  bl 0x831a816c
	ctx.lr = 0x825ABDC8;
	sub_831A8130(ctx, base);
	// 825ABDC8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ABDCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825ABDD0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825ABDD4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 825ABDD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825ABDDC: 48847E25  bl 0x82df3c00
	ctx.lr = 0x825ABDE0;
	sub_82DF3C00(ctx, base);
	// 825ABDE0: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 825ABDE4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825ABDE8: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825ABDEC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825ABDF0: 48847E11  bl 0x82df3c00
	ctx.lr = 0x825ABDF4;
	sub_82DF3C00(ctx, base);
	// 825ABDF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825ABDF8: 48847631  bl 0x82df3428
	ctx.lr = 0x825ABDFC;
	sub_82DF3428(ctx, base);
	// 825ABDFC: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825ABE00: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 825ABE04: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 825ABE08: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 825ABE0C: 48847DF5  bl 0x82df3c00
	ctx.lr = 0x825ABE10;
	sub_82DF3C00(ctx, base);
	// 825ABE10: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 825ABE14: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 825ABE18: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ABE1C: 4BFFFE5D  bl 0x825abc78
	ctx.lr = 0x825ABE20;
	sub_825ABC78(ctx, base);
	// 825ABE20: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 825ABE24: 48847605  bl 0x82df3428
	ctx.lr = 0x825ABE28;
	sub_82DF3428(ctx, base);
	// 825ABE28: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825ABE2C: 488475FD  bl 0x82df3428
	ctx.lr = 0x825ABE30;
	sub_82DF3428(ctx, base);
	// 825ABE30: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ABE34: 93CB0060  stw r30, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 825ABE38: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825ABE3C: 48BFC380  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ABE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ABE40 size=120
    let mut pc: u32 = 0x825ABE40;
    'dispatch: loop {
        match pc {
            0x825ABE40 => {
    //   block [0x825ABE40..0x825ABEB8)
	// 825ABE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ABE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ABE48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825ABE4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ABE50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825ABE54: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ABE58: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 825ABE5C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825ABE60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ABE64: 419A0018  beq cr6, 0x825abe7c
	if ctx.cr[6].eq {
	pc = 0x825ABE7C; continue 'dispatch;
	}
	// 825ABE68: 3D60822C  lis r11, -0x7dd4
	ctx.r[11].s64 = -2111045632;
	// 825ABE6C: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 825ABE70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ABE74: 419A0008  beq cr6, 0x825abe7c
	if ctx.cr[6].eq {
	pc = 0x825ABE7C; continue 'dispatch;
	}
	// 825ABE78: 48005A51  bl 0x825b18c8
	ctx.lr = 0x825ABE7C;
	sub_825B18C8(ctx, base);
	// 825ABE7C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ABE80: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 825ABE84: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 825ABE88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ABE8C: 419A0018  beq cr6, 0x825abea4
	if ctx.cr[6].eq {
	pc = 0x825ABEA4; continue 'dispatch;
	}
	// 825ABE90: 3D60822C  lis r11, -0x7dd4
	ctx.r[11].s64 = -2111045632;
	// 825ABE94: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 825ABE98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ABE9C: 419A0008  beq cr6, 0x825abea4
	if ctx.cr[6].eq {
	pc = 0x825ABEA4; continue 'dispatch;
	}
	// 825ABEA0: 4BEA77E1  bl 0x82453680
	ctx.lr = 0x825ABEA4;
	sub_82453680(ctx, base);
	// 825ABEA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825ABEA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ABEAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ABEB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825ABEB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ABEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825ABEB8 size=56
    let mut pc: u32 = 0x825ABEB8;
    'dispatch: loop {
        match pc {
            0x825ABEB8 => {
    //   block [0x825ABEB8..0x825ABEF0)
	// 825ABEB8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ABEBC: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825ABEC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ABEC4: 419A0014  beq cr6, 0x825abed8
	if ctx.cr[6].eq {
	pc = 0x825ABED8; continue 'dispatch;
	}
	// 825ABEC8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ABECC: 814B0050  lwz r10, 0x50(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825ABED0: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 825ABED4: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825ABED8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ABEDC: 814B0054  lwz r10, 0x54(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 825ABEE0: 914B0058  stw r10, 0x58(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 825ABEE4: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ABEE8: 808B0054  lwz r4, 0x54(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 825ABEEC: 4BFFFF54  b 0x825abe40
	sub_825ABE40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ABEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825ABEF0 size=32
    let mut pc: u32 = 0x825ABEF0;
    'dispatch: loop {
        match pc {
            0x825ABEF0 => {
    //   block [0x825ABEF0..0x825ABF10)
	// 825ABEF0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ABEF4: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825ABEF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ABEFC: 419A0008  beq cr6, 0x825abf04
	if ctx.cr[6].eq {
	pc = 0x825ABF04; continue 'dispatch;
	}
	// 825ABF00: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 825ABF04: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ABF08: 908B0054  stw r4, 0x54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 825ABF0C: 4BFFFF34  b 0x825abe40
	sub_825ABE40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ABF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825ABF10 size=52
    let mut pc: u32 = 0x825ABF10;
    'dispatch: loop {
        match pc {
            0x825ABF10 => {
    //   block [0x825ABF10..0x825ABF44)
	// 825ABF10: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ABF14: 814B0050  lwz r10, 0x50(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825ABF18: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 825ABF1C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825ABF20: 419A0010  beq cr6, 0x825abf30
	if ctx.cr[6].eq {
	pc = 0x825ABF30; continue 'dispatch;
	}
	// 825ABF24: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ABF28: 814A0050  lwz r10, 0x50(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(80 as u32) ) } as u64;
	// 825ABF2C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825ABF30: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ABF34: 916A0054  stw r11, 0x54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825ABF38: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ABF3C: 808B0058  lwz r4, 0x58(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 825ABF40: 4BFFFF00  b 0x825abe40
	sub_825ABE40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ABF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ABF48 size=212
    let mut pc: u32 = 0x825ABF48;
    'dispatch: loop {
        match pc {
            0x825ABF48 => {
    //   block [0x825ABF48..0x825AC01C)
	// 825ABF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ABF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ABF50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825ABF54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825ABF58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ABF5C: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825ABF60: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 825ABF64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825ABF68: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825ABF6C: 38CB8544  addi r6, r11, -0x7abc
	ctx.r[6].s64 = ctx.r[11].s64 + -31420;
	// 825ABF70: 38AA7B58  addi r5, r10, 0x7b58
	ctx.r[5].s64 = ctx.r[10].s64 + 31576;
	// 825ABF74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ABF78: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825ABF7C: 48BFDFCD  bl 0x831a9f48
	ctx.lr = 0x825ABF80;
	sub_831A9F48(ctx, base);
	// 825ABF80: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 825ABF84: 41820080  beq 0x825ac004
	if ctx.cr[0].eq {
	pc = 0x825AC004; continue 'dispatch;
	}
	// 825ABF88: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ABF8C: 814B0050  lwz r10, 0x50(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825ABF90: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825ABF94: 419A000C  beq cr6, 0x825abfa0
	if ctx.cr[6].eq {
	pc = 0x825ABFA0; continue 'dispatch;
	}
	// 825ABF98: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ABF9C: 48000008  b 0x825abfa4
	pc = 0x825ABFA4; continue 'dispatch;
	// 825ABFA0: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 825ABFA4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ABFA8: 814A0050  lwz r10, 0x50(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(80 as u32) ) } as u64;
	// 825ABFAC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825ABFB0: 419A0008  beq cr6, 0x825abfb8
	if ctx.cr[6].eq {
	pc = 0x825ABFB8; continue 'dispatch;
	}
	// 825ABFB4: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825ABFB8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ABFBC: 916A0054  stw r11, 0x54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825ABFC0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ABFC4: 814B0050  lwz r10, 0x50(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825ABFC8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825ABFCC: 419A000C  beq cr6, 0x825abfd8
	if ctx.cr[6].eq {
	pc = 0x825ABFD8; continue 'dispatch;
	}
	// 825ABFD0: 808A0000  lwz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ABFD4: 48000008  b 0x825abfdc
	pc = 0x825ABFDC; continue 'dispatch;
	// 825ABFD8: 808B0054  lwz r4, 0x54(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 825ABFDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825ABFE0: 4BFFFE61  bl 0x825abe40
	ctx.lr = 0x825ABFE4;
	sub_825ABE40(ctx, base);
	// 825ABFE4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ABFE8: 814B0064  lwz r10, 0x64(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 825ABFEC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 825ABFF0: 409A0014  bne cr6, 0x825ac004
	if !ctx.cr[6].eq {
	pc = 0x825AC004; continue 'dispatch;
	}
	// 825ABFF4: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ABFF8: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 825ABFFC: 388A0068  addi r4, r10, 0x68
	ctx.r[4].s64 = ctx.r[10].s64 + 104;
	// 825AC000: 48847BD1  bl 0x82df3bd0
	ctx.lr = 0x825AC004;
	sub_82DF3BD0(ctx, base);
	// 825AC004: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AC008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AC00C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AC010: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AC014: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AC018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AC020 size=204
    let mut pc: u32 = 0x825AC020;
    'dispatch: loop {
        match pc {
            0x825AC020 => {
    //   block [0x825AC020..0x825AC0EC)
	// 825AC020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AC024: 48BFC149  bl 0x831a816c
	ctx.lr = 0x825AC028;
	sub_831A8130(ctx, base);
	// 825AC028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AC02C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825AC030: 83FD0008  lwz r31, 8(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC034: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC038: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 825AC03C: 41980084  blt cr6, 0x825ac0c0
	if ctx.cr[6].lt {
	pc = 0x825AC0C0; continue 'dispatch;
	}
	// 825AC040: 83DF0050  lwz r30, 0x50(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AC044: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 825AC048: 419A000C  beq cr6, 0x825ac054
	if ctx.cr[6].eq {
	pc = 0x825AC054; continue 'dispatch;
	}
	// 825AC04C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AC050: 48000008  b 0x825ac058
	pc = 0x825AC058; continue 'dispatch;
	// 825AC054: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 825AC058: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825AC05C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 825AC060: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AC064: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AC068: 4BF70141  bl 0x8251c1a8
	ctx.lr = 0x825AC06C;
	sub_8251C1A8(ctx, base);
	// 825AC06C: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AC070: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AC074: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 825AC078: 409A001C  bne cr6, 0x825ac094
	if !ctx.cr[6].eq {
	pc = 0x825AC094; continue 'dispatch;
	}
	// 825AC07C: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC080: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AC084: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AC088: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AC08C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AC090: 4800001C  b 0x825ac0ac
	pc = 0x825AC0AC; continue 'dispatch;
	// 825AC094: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AC098: 4BF3CFF9  bl 0x824e9090
	ctx.lr = 0x825AC09C;
	sub_824E9090(ctx, base);
	// 825AC09C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AC0A0: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 825AC0A4: 419AFFD8  beq cr6, 0x825ac07c
	if ctx.cr[6].eq {
	pc = 0x825AC07C; continue 'dispatch;
	}
	// 825AC0A8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 825AC0AC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825AC0B0: 419A0008  beq cr6, 0x825ac0b8
	if ctx.cr[6].eq {
	pc = 0x825AC0B8; continue 'dispatch;
	}
	// 825AC0B4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AC0B8: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC0BC: 916A0054  stw r11, 0x54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825AC0C0: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC0C4: 814B0050  lwz r10, 0x50(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AC0C8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825AC0CC: 419A000C  beq cr6, 0x825ac0d8
	if ctx.cr[6].eq {
	pc = 0x825AC0D8; continue 'dispatch;
	}
	// 825AC0D0: 808A0000  lwz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AC0D4: 48000008  b 0x825ac0dc
	pc = 0x825AC0DC; continue 'dispatch;
	// 825AC0D8: 808B0054  lwz r4, 0x54(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 825AC0DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825AC0E0: 4BFFFD61  bl 0x825abe40
	ctx.lr = 0x825AC0E4;
	sub_825ABE40(ctx, base);
	// 825AC0E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825AC0E8: 48BFC0D4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AC0F0 size=80
    let mut pc: u32 = 0x825AC0F0;
    'dispatch: loop {
        match pc {
            0x825AC0F0 => {
    //   block [0x825AC0F0..0x825AC140)
	// 825AC0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AC0F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AC0F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AC0FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AC100: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AC104: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 825AC108: 48847321  bl 0x82df3428
	ctx.lr = 0x825AC10C;
	sub_82DF3428(ctx, base);
	// 825AC10C: 387F005C  addi r3, r31, 0x5c
	ctx.r[3].s64 = ctx.r[31].s64 + 92;
	// 825AC110: 48847319  bl 0x82df3428
	ctx.lr = 0x825AC114;
	sub_82DF3428(ctx, base);
	// 825AC114: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 825AC118: 4BD1CBA1  bl 0x822c8cb8
	ctx.lr = 0x825AC11C;
	sub_822C8CB8(ctx, base);
	// 825AC11C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 825AC120: 4BD1CB99  bl 0x822c8cb8
	ctx.lr = 0x825AC124;
	sub_822C8CB8(ctx, base);
	// 825AC124: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AC128: 4BFFFC41  bl 0x825abd68
	ctx.lr = 0x825AC12C;
	sub_825ABD68(ctx, base);
	// 825AC12C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AC130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AC134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AC138: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AC13C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AC140 size=76
    let mut pc: u32 = 0x825AC140;
    'dispatch: loop {
        match pc {
            0x825AC140 => {
    //   block [0x825AC140..0x825AC18C)
	// 825AC140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AC144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AC148: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AC14C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AC150: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AC154: 4BD5D2D5  bl 0x82309428
	ctx.lr = 0x825AC158;
	sub_82309428(ctx, base);
	// 825AC158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AC15C: 387F005C  addi r3, r31, 0x5c
	ctx.r[3].s64 = ctx.r[31].s64 + 92;
	// 825AC160: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 825AC164: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825AC168: 48846F89  bl 0x82df30f0
	ctx.lr = 0x825AC16C;
	sub_82DF30F0(ctx, base);
	// 825AC16C: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 825AC170: 48846F81  bl 0x82df30f0
	ctx.lr = 0x825AC174;
	sub_82DF30F0(ctx, base);
	// 825AC174: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AC178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AC17C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AC180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AC184: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AC188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AC190 size=212
    let mut pc: u32 = 0x825AC190;
    'dispatch: loop {
        match pc {
            0x825AC190 => {
    //   block [0x825AC190..0x825AC264)
	// 825AC190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AC194: 48BFBFD1  bl 0x831a8164
	ctx.lr = 0x825AC198;
	sub_831A8130(ctx, base);
	// 825AC198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AC19C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AC1A0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AC1A4: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 825AC1A8: 396BAF84  addi r11, r11, -0x507c
	ctx.r[11].s64 = ctx.r[11].s64 + -20604;
	// 825AC1AC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 825AC1B0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AC1B4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825AC1B8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 825AC1BC: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 825AC1C0: 48846F31  bl 0x82df30f0
	ctx.lr = 0x825AC1C4;
	sub_82DF30F0(ctx, base);
	// 825AC1C4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AC1C8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825AC1CC: 396BB54C  addi r11, r11, -0x4ab4
	ctx.r[11].s64 = ctx.r[11].s64 + -19124;
	// 825AC1D0: 388AB500  addi r4, r10, -0x4b00
	ctx.r[4].s64 = ctx.r[10].s64 + -19200;
	// 825AC1D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AC1D8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825AC1DC: 38A0003F  li r5, 0x3f
	ctx.r[5].s64 = 63;
	// 825AC1E0: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 825AC1E4: 48846205  bl 0x82df23e8
	ctx.lr = 0x825AC1E8;
	sub_82DF23E8(ctx, base);
	// 825AC1E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825AC1EC: 4182000C  beq 0x825ac1f8
	if ctx.cr[0].eq {
	pc = 0x825AC1F8; continue 'dispatch;
	}
	// 825AC1F0: 4BFFFF51  bl 0x825ac140
	ctx.lr = 0x825AC1F4;
	sub_825AC140(ctx, base);
	// 825AC1F4: 48000008  b 0x825ac1fc
	pc = 0x825AC1FC; continue 'dispatch;
	// 825AC1F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825AC1FC: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 825AC200: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825AC204: 3863005C  addi r3, r3, 0x5c
	ctx.r[3].s64 = ctx.r[3].s64 + 92;
	// 825AC208: 488479C9  bl 0x82df3bd0
	ctx.lr = 0x825AC20C;
	sub_82DF3BD0(ctx, base);
	// 825AC20C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC210: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825AC214: 93CB0050  stw r30, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 825AC218: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AC21C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC220: 914B0054  stw r10, 0x54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 825AC224: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC228: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AC22C: 914B0058  stw r10, 0x58(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 825AC230: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC234: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 825AC238: 4BEC4E91  bl 0x824710c8
	ctx.lr = 0x825AC23C;
	sub_824710C8(ctx, base);
	// 825AC23C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AC244: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825AC248: 936A0064  stw r27, 0x64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 825AC24C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC250: 996A006C  stb r11, 0x6c(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(108 as u32), ctx.r[11].u8 ) };
	// 825AC254: 4BD1CA65  bl 0x822c8cb8
	ctx.lr = 0x825AC258;
	sub_822C8CB8(ctx, base);
	// 825AC258: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AC25C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825AC260: 48BFBF54  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AC268 size=24
    let mut pc: u32 = 0x825AC268;
    'dispatch: loop {
        match pc {
            0x825AC268 => {
    //   block [0x825AC268..0x825AC280)
	// 825AC268: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC26C: 814B0050  lwz r10, 0x50(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AC270: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825AC274: 419A000C  beq cr6, 0x825ac280
	if ctx.cr[6].eq {
		sub_825AC280(ctx, base);
		return;
	}
	// 825AC278: 808A0000  lwz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AC27C: 48000008  b 0x825ac284
	sub_825AC280(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AC280 size=8
    let mut pc: u32 = 0x825AC280;
    'dispatch: loop {
        match pc {
            0x825AC280 => {
    //   block [0x825AC280..0x825AC288)
	// 825AC280: 808B0054  lwz r4, 0x54(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 825AC284: 4BFFFBBC  b 0x825abe40
	sub_825ABE40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AC288 size=392
    let mut pc: u32 = 0x825AC288;
    'dispatch: loop {
        match pc {
            0x825AC288 => {
    //   block [0x825AC288..0x825AC410)
	// 825AC288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AC28C: 48BFBED5  bl 0x831a8160
	ctx.lr = 0x825AC290;
	sub_831A8130(ctx, base);
	// 825AC290: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AC294: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825AC298: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AC29C: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 825AC2A0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825AC2A4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825AC2A8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825AC2AC: 388BB500  addi r4, r11, -0x4b00
	ctx.r[4].s64 = ctx.r[11].s64 + -19200;
	// 825AC2B0: 38A000FF  li r5, 0xff
	ctx.r[5].s64 = 255;
	// 825AC2B4: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 825AC2B8: 4BD14121  bl 0x822c03d8
	ctx.lr = 0x825AC2BC;
	sub_822C03D8(ctx, base);
	// 825AC2BC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 825AC2C0: 4182005C  beq 0x825ac31c
	if ctx.cr[0].eq {
	pc = 0x825AC31C; continue 'dispatch;
	}
	// 825AC2C4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC2C8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825AC2CC: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 825AC2D0: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 825AC2D4: 4BFDFC3D  bl 0x8258bf10
	ctx.lr = 0x825AC2D8;
	sub_8258BF10(ctx, base);
	// 825AC2D8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC2DC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AC2E0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 825AC2E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AC2E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AC2EC: 834B0050  lwz r26, 0x50(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AC2F0: 816A002C  lwz r11, 0x2c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 825AC2F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AC2F8: 4E800421  bctrl
	ctx.lr = 0x825AC2FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AC2FC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AC300: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AC304: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 825AC308: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 825AC30C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825AC310: 4BFFFE81  bl 0x825ac190
	ctx.lr = 0x825AC314;
	sub_825AC190(ctx, base);
	// 825AC314: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825AC318: 48000008  b 0x825ac320
	pc = 0x825AC320; continue 'dispatch;
	// 825AC31C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825AC320: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 825AC324: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825AC328: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825AC32C: 4BF4708D  bl 0x824f33b8
	ctx.lr = 0x825AC330;
	sub_824F33B8(ctx, base);
	// 825AC330: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825AC334: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825AC338: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825AC33C: 4BD13CC5  bl 0x822c0000
	ctx.lr = 0x825AC340;
	sub_822C0000(ctx, base);
	// 825AC340: 57AB07FF  clrlwi. r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AC344: 4182000C  beq 0x825ac350
	if ctx.cr[0].eq {
	pc = 0x825AC350; continue 'dispatch;
	}
	// 825AC348: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AC34C: 488470DD  bl 0x82df3428
	ctx.lr = 0x825AC350;
	sub_82DF3428(ctx, base);
	// 825AC350: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AC354: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AC358: 4BFFA131  bl 0x825a6488
	ctx.lr = 0x825AC35C;
	sub_825A6488(ctx, base);
	// 825AC35C: 83C10058  lwz r30, 0x58(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825AC360: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AC364: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AC368: 48229701  bl 0x827d5a68
	ctx.lr = 0x825AC36C;
	sub_827D5A68(ctx, base);
	// 825AC36C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AC370: 488470B9  bl 0x82df3428
	ctx.lr = 0x825AC374;
	sub_82DF3428(ctx, base);
	// 825AC374: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AC37C: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825AC380: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC384: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 825AC388: 4BD1C931  bl 0x822c8cb8
	ctx.lr = 0x825AC38C;
	sub_822C8CB8(ctx, base);
	// 825AC38C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC390: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 825AC394: 4BD1C925  bl 0x822c8cb8
	ctx.lr = 0x825AC398;
	sub_822C8CB8(ctx, base);
	// 825AC398: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC39C: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC3A0: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 825AC3A4: 916A0064  stw r11, 0x64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825AC3A8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC3AC: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 825AC3B0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 825AC3B4: 409A0018  bne cr6, 0x825ac3cc
	if !ctx.cr[6].eq {
	pc = 0x825AC3CC; continue 'dispatch;
	}
	// 825AC3B8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC3BC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC3C0: 388A0068  addi r4, r10, 0x68
	ctx.r[4].s64 = ctx.r[10].s64 + 104;
	// 825AC3C4: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 825AC3C8: 48847809  bl 0x82df3bd0
	ctx.lr = 0x825AC3CC;
	sub_82DF3BD0(ctx, base);
	// 825AC3CC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825AC3D0: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 825AC3D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AC3D8: 907C0004  stw r3, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 825AC3DC: 419A0028  beq cr6, 0x825ac404
	if ctx.cr[6].eq {
	pc = 0x825AC404; continue 'dispatch;
	}
	// 825AC3E0: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 825AC3E4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825AC3E8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AC3EC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825AC3F0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825AC3F4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825AC3F8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AC3FC: 4082FFE8  bne 0x825ac3e4
	if !ctx.cr[0].eq {
	pc = 0x825AC3E4; continue 'dispatch;
	}
	// 825AC400: 4BD14491  bl 0x822c0890
	ctx.lr = 0x825AC404;
	sub_822C0890(ctx, base);
	// 825AC404: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825AC408: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 825AC40C: 48BFBDA4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AC410 size=184
    let mut pc: u32 = 0x825AC410;
    'dispatch: loop {
        match pc {
            0x825AC410 => {
    //   block [0x825AC410..0x825AC4C8)
	// 825AC410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AC414: 48BFBD55  bl 0x831a8168
	ctx.lr = 0x825AC418;
	sub_831A8130(ctx, base);
	// 825AC418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AC41C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AC420: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AC424: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 825AC428: 396BAF84  addi r11, r11, -0x507c
	ctx.r[11].s64 = ctx.r[11].s64 + -20604;
	// 825AC42C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825AC430: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AC434: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825AC438: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 825AC43C: 48846CB5  bl 0x82df30f0
	ctx.lr = 0x825AC440;
	sub_82DF30F0(ctx, base);
	// 825AC440: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AC444: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825AC448: 396BB54C  addi r11, r11, -0x4ab4
	ctx.r[11].s64 = ctx.r[11].s64 + -19124;
	// 825AC44C: 388AB500  addi r4, r10, -0x4b00
	ctx.r[4].s64 = ctx.r[10].s64 + -19200;
	// 825AC450: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AC454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825AC458: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 825AC45C: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 825AC460: 48845F89  bl 0x82df23e8
	ctx.lr = 0x825AC464;
	sub_82DF23E8(ctx, base);
	// 825AC464: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825AC468: 4182000C  beq 0x825ac474
	if ctx.cr[0].eq {
	pc = 0x825AC474; continue 'dispatch;
	}
	// 825AC46C: 4BFFFCD5  bl 0x825ac140
	ctx.lr = 0x825AC470;
	sub_825AC140(ctx, base);
	// 825AC470: 48000008  b 0x825ac478
	pc = 0x825AC478; continue 'dispatch;
	// 825AC474: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825AC478: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 825AC47C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825AC480: 3863005C  addi r3, r3, 0x5c
	ctx.r[3].s64 = ctx.r[3].s64 + 92;
	// 825AC484: 4884774D  bl 0x82df3bd0
	ctx.lr = 0x825AC488;
	sub_82DF3BD0(ctx, base);
	// 825AC488: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC48C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AC490: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AC494: 93CA0050  stw r30, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 825AC498: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AC49C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC4A0: 912A0054  stw r9, 0x54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 825AC4A4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC4A8: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AC4AC: 912A0058  stw r9, 0x58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 825AC4B0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC4B4: 938A0064  stw r28, 0x64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 825AC4B8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC4BC: 996A006C  stb r11, 0x6c(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(108 as u32), ctx.r[11].u8 ) };
	// 825AC4C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825AC4C4: 48BFBCF4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AC4C8 size=96
    let mut pc: u32 = 0x825AC4C8;
    'dispatch: loop {
        match pc {
            0x825AC4C8 => {
    //   block [0x825AC4C8..0x825AC528)
	// 825AC4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AC4CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AC4D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AC4D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AC4D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AC4DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AC4E0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AC4E4: 396BB54C  addi r11, r11, -0x4ab4
	ctx.r[11].s64 = ctx.r[11].s64 + -19124;
	// 825AC4E8: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC4EC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AC4F0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 825AC4F4: 419A0014  beq cr6, 0x825ac508
	if ctx.cr[6].eq {
	pc = 0x825AC508; continue 'dispatch;
	}
	// 825AC4F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AC4FC: 4BFFFBF5  bl 0x825ac0f0
	ctx.lr = 0x825AC500;
	sub_825AC0F0(ctx, base);
	// 825AC500: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AC504: 48845ED5  bl 0x82df23d8
	ctx.lr = 0x825AC508;
	sub_82DF23D8(ctx, base);
	// 825AC508: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AC50C: 4BFF9F05  bl 0x825a6410
	ctx.lr = 0x825AC510;
	sub_825A6410(ctx, base);
	// 825AC510: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AC514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AC518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AC51C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AC520: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AC524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AC528 size=76
    let mut pc: u32 = 0x825AC528;
    'dispatch: loop {
        match pc {
            0x825AC528 => {
    //   block [0x825AC528..0x825AC574)
	// 825AC528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AC52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AC530: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AC534: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AC538: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AC53C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AC540: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825AC544: 4BFFFF85  bl 0x825ac4c8
	ctx.lr = 0x825AC548;
	sub_825AC4C8(ctx, base);
	// 825AC548: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AC54C: 4182000C  beq 0x825ac558
	if ctx.cr[0].eq {
	pc = 0x825AC558; continue 'dispatch;
	}
	// 825AC550: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AC554: 4BD13D15  bl 0x822c0268
	ctx.lr = 0x825AC558;
	sub_822C0268(ctx, base);
	// 825AC558: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AC55C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AC560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AC564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AC568: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AC56C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AC570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AC578 size=92
    let mut pc: u32 = 0x825AC578;
    'dispatch: loop {
        match pc {
            0x825AC578 => {
    //   block [0x825AC578..0x825AC5D4)
	// 825AC578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AC57C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AC580: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AC584: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AC588: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AC58C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825AC590: 3BC30008  addi r30, r3, 8
	ctx.r[30].s64 = ctx.r[3].s64 + 8;
	// 825AC594: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AC598: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AC59C: 4BFF9EED  bl 0x825a6488
	ctx.lr = 0x825AC5A0;
	sub_825A6488(ctx, base);
	// 825AC5A0: 48846C11  bl 0x82df31b0
	ctx.lr = 0x825AC5A4;
	sub_82DF31B0(ctx, base);
	// 825AC5A4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AC5A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AC5AC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825AC5B0: 4802D879  bl 0x825d9e28
	ctx.lr = 0x825AC5B4;
	sub_825D9E28(ctx, base);
	// 825AC5B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AC5B8: 48846E71  bl 0x82df3428
	ctx.lr = 0x825AC5BC;
	sub_82DF3428(ctx, base);
	// 825AC5BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AC5C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AC5C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AC5C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AC5CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AC5D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825AC5D8 size=28
    let mut pc: u32 = 0x825AC5D8;
    'dispatch: loop {
        match pc {
            0x825AC5D8 => {
    //   block [0x825AC5D8..0x825AC5F4)
	// 825AC5D8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AC5DC: C0030008  lfs f0, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825AC5E0: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825AC5E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AC5E8: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AC5EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AC5F0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AC5F8 size=12
    let mut pc: u32 = 0x825AC5F8;
    'dispatch: loop {
        match pc {
            0x825AC5F8 => {
    //   block [0x825AC5F8..0x825AC604)
	// 825AC5F8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AC5FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AC600: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC604(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825AC604 size=24
    let mut pc: u32 = 0x825AC604;
    'dispatch: loop {
        match pc {
            0x825AC604 => {
    //   block [0x825AC604..0x825AC61C)
	// 825AC604: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AC608: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825AC60C: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825AC610: 816A003C  lwz r11, 0x3c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AC614: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AC618: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC61C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AC61C size=4
    let mut pc: u32 = 0x825AC61C;
    'dispatch: loop {
        match pc {
            0x825AC61C => {
    //   block [0x825AC61C..0x825AC620)
	// 825AC61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825AC620 size=160
    let mut pc: u32 = 0x825AC620;
    'dispatch: loop {
        match pc {
            0x825AC620 => {
    //   block [0x825AC620..0x825AC6C0)
	// 825AC620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AC624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AC628: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AC62C: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AC630: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AC634: 419A000C  beq cr6, 0x825ac640
	if ctx.cr[6].eq {
	pc = 0x825AC640; continue 'dispatch;
	}
	// 825AC638: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825AC63C: 48000008  b 0x825ac644
	pc = 0x825AC644; continue 'dispatch;
	// 825AC640: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825AC644: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825AC648: C1A30020  lfs f13, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825AC64C: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 825AC650: ED6D0072  fmuls f11, f13, f1
	ctx.f[11].f64 = (((ctx.f[13].f64 * ctx.f[1].f64) as f32) as f64);
	// 825AC654: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AC658: C1AA9F7C  lfs f13, -0x6084(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-24708 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825AC65C: C189B59C  lfs f12, -0x4a64(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-19044 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825AC660: EDA10372  fmuls f13, f1, f13
	ctx.f[13].f64 = (((ctx.f[1].f64 * ctx.f[13].f64) as f32) as f64);
	// 825AC664: ED810332  fmuls f12, f1, f12
	ctx.f[12].f64 = (((ctx.f[1].f64 * ctx.f[12].f64) as f32) as f64);
	// 825AC668: FDA0636E  fsel f13, f0, f13, f12
	ctx.f[13].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[12].f64 };
	// 825AC66C: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 825AC670: EC005824  fdivs f0, f0, f11
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[11].f64) as f32) as f64;
	// 825AC674: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 825AC678: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 825AC67C: E9410056  lwa r10, 0x54(r1)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as i32) as i64;
	// 825AC680: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 825AC684: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825AC688: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 825AC68C: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 825AC690: EC0002F2  fmuls f0, f0, f11
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[11].f64) as f32) as f64);
	// 825AC694: 419A0008  beq cr6, 0x825ac69c
	if ctx.cr[6].eq {
	pc = 0x825AC69C; continue 'dispatch;
	}
	// 825AC698: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825AC69C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AC6A0: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825AC6A4: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AC6A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AC6AC: 4E800421  bctrl
	ctx.lr = 0x825AC6B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AC6B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AC6B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AC6B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AC6BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AC6C0 size=92
    let mut pc: u32 = 0x825AC6C0;
    'dispatch: loop {
        match pc {
            0x825AC6C0 => {
    //   block [0x825AC6C0..0x825AC71C)
	// 825AC6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AC6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AC6C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AC6CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AC6D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AC6D4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825AC6D8: 3BC30008  addi r30, r3, 8
	ctx.r[30].s64 = ctx.r[3].s64 + 8;
	// 825AC6DC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AC6E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AC6E4: 4BFF9DA5  bl 0x825a6488
	ctx.lr = 0x825AC6E8;
	sub_825A6488(ctx, base);
	// 825AC6E8: 48846AC9  bl 0x82df31b0
	ctx.lr = 0x825AC6EC;
	sub_82DF31B0(ctx, base);
	// 825AC6EC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AC6F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AC6F4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825AC6F8: 4802D759  bl 0x825d9e50
	ctx.lr = 0x825AC6FC;
	sub_825D9E50(ctx, base);
	// 825AC6FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AC700: 48846D29  bl 0x82df3428
	ctx.lr = 0x825AC704;
	sub_82DF3428(ctx, base);
	// 825AC704: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AC708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AC70C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AC710: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AC714: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AC718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AC720 size=28
    let mut pc: u32 = 0x825AC720;
    'dispatch: loop {
        match pc {
            0x825AC720 => {
    //   block [0x825AC720..0x825AC73C)
	// 825AC720: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AC724: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC728: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825AC72C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AC730: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AC734: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AC738: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825AC740 size=208
    let mut pc: u32 = 0x825AC740;
    'dispatch: loop {
        match pc {
            0x825AC740 => {
    //   block [0x825AC740..0x825AC810)
	// 825AC740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AC744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AC748: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AC74C: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AC750: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825AC754: 419A000C  beq cr6, 0x825ac760
	if ctx.cr[6].eq {
	pc = 0x825AC760; continue 'dispatch;
	}
	// 825AC758: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AC75C: 48000008  b 0x825ac764
	pc = 0x825AC764; continue 'dispatch;
	// 825AC760: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC764: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 825AC768: 81230020  lwz r9, 0x20(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 825AC76C: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 825AC770: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 825AC774: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825AC778: FD80069C  fcfid f12, f0
	ctx.f[12].f64 = (ctx.f[0].s64 as f64);
	// 825AC77C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AC780: C1ABB59C  lfs f13, -0x4a64(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19044 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825AC784: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 825AC788: C0089F7C  lfs f0, -0x6084(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-24708 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825AC78C: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 825AC790: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825AC794: EC010032  fmuls f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 825AC798: EDA10372  fmuls f13, f1, f13
	ctx.f[13].f64 = (((ctx.f[1].f64 * ctx.f[13].f64) as f32) as f64);
	// 825AC79C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825AC7A0: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 825AC7A4: FC0C682E  fsel f0, f12, f0, f13
	ctx.f[0].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 825AC7A8: FD605818  frsp f11, f11
	ctx.f[11].f64 = (ctx.f[11].f64 as f32) as f64;
	// 825AC7AC: EC00602A  fadds f0, f0, f12
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	// 825AC7B0: EDAB0072  fmuls f13, f11, f1
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[1].f64) as f32) as f64);
	// 825AC7B4: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 825AC7B8: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 825AC7BC: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 825AC7C0: E9610056  lwa r11, 0x54(r1)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as i32) as i64;
	// 825AC7C4: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 825AC7C8: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825AC7CC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 825AC7D0: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 825AC7D4: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 825AC7D8: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825AC7DC: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 825AC7E0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825AC7E4: 419A0008  beq cr6, 0x825ac7ec
	if ctx.cr[6].eq {
	pc = 0x825AC7EC; continue 'dispatch;
	}
	// 825AC7E8: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AC7EC: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825AC7F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AC7F4: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AC7F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AC7FC: 4E800421  bctrl
	ctx.lr = 0x825AC800;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AC800: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AC804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AC808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AC80C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AC810 size=92
    let mut pc: u32 = 0x825AC810;
    'dispatch: loop {
        match pc {
            0x825AC810 => {
    //   block [0x825AC810..0x825AC86C)
	// 825AC810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AC814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AC818: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AC81C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AC820: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AC824: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825AC828: 3BC30008  addi r30, r3, 8
	ctx.r[30].s64 = ctx.r[3].s64 + 8;
	// 825AC82C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AC830: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AC834: 4BFF9C55  bl 0x825a6488
	ctx.lr = 0x825AC838;
	sub_825A6488(ctx, base);
	// 825AC838: 48846979  bl 0x82df31b0
	ctx.lr = 0x825AC83C;
	sub_82DF31B0(ctx, base);
	// 825AC83C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AC840: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AC844: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825AC848: 4802D601  bl 0x825d9e48
	ctx.lr = 0x825AC84C;
	sub_825D9E48(ctx, base);
	// 825AC84C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AC850: 48846BD9  bl 0x82df3428
	ctx.lr = 0x825AC854;
	sub_82DF3428(ctx, base);
	// 825AC854: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AC858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AC85C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AC860: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AC864: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AC868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825AC870 size=208
    let mut pc: u32 = 0x825AC870;
    'dispatch: loop {
        match pc {
            0x825AC870 => {
    //   block [0x825AC870..0x825AC940)
	// 825AC870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AC874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AC878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AC87C: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AC880: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825AC884: 419A000C  beq cr6, 0x825ac890
	if ctx.cr[6].eq {
	pc = 0x825AC890; continue 'dispatch;
	}
	// 825AC888: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AC88C: 48000008  b 0x825ac894
	pc = 0x825AC894; continue 'dispatch;
	// 825AC890: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AC894: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 825AC898: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 825AC89C: E9230022  lwa r9, 0x20(r3)
	ctx.r[9].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as i32) as i64;
	// 825AC8A0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AC8A4: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 825AC8A8: C1ABB59C  lfs f13, -0x4a64(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19044 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825AC8AC: EDA10372  fmuls f13, f1, f13
	ctx.f[13].f64 = (((ctx.f[1].f64 * ctx.f[13].f64) as f32) as f64);
	// 825AC8B0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825AC8B4: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825AC8B8: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 825AC8BC: FD80069C  fcfid f12, f0
	ctx.f[12].f64 = (ctx.f[0].s64 as f64);
	// 825AC8C0: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825AC8C4: C0089F7C  lfs f0, -0x6084(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-24708 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825AC8C8: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 825AC8CC: EC010032  fmuls f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 825AC8D0: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 825AC8D4: FD605818  frsp f11, f11
	ctx.f[11].f64 = (ctx.f[11].f64 as f32) as f64;
	// 825AC8D8: FC0C682E  fsel f0, f12, f0, f13
	ctx.f[0].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 825AC8DC: EDAB0072  fmuls f13, f11, f1
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[1].f64) as f32) as f64);
	// 825AC8E0: EC00602A  fadds f0, f0, f12
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	// 825AC8E4: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 825AC8E8: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 825AC8EC: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 825AC8F0: E9610056  lwa r11, 0x54(r1)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as i32) as i64;
	// 825AC8F4: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 825AC8F8: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825AC8FC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 825AC900: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 825AC904: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 825AC908: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 825AC90C: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 825AC910: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825AC914: 419A0008  beq cr6, 0x825ac91c
	if ctx.cr[6].eq {
	pc = 0x825AC91C; continue 'dispatch;
	}
	// 825AC918: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AC91C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825AC920: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AC924: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AC928: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AC92C: 4E800421  bctrl
	ctx.lr = 0x825AC930;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AC930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AC934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AC938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AC93C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AC940 size=92
    let mut pc: u32 = 0x825AC940;
    'dispatch: loop {
        match pc {
            0x825AC940 => {
    //   block [0x825AC940..0x825AC99C)
	// 825AC940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AC944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AC948: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AC94C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AC950: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AC954: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825AC958: 3BC30008  addi r30, r3, 8
	ctx.r[30].s64 = ctx.r[3].s64 + 8;
	// 825AC95C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AC960: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AC964: 4BFF9B25  bl 0x825a6488
	ctx.lr = 0x825AC968;
	sub_825A6488(ctx, base);
	// 825AC968: 48846849  bl 0x82df31b0
	ctx.lr = 0x825AC96C;
	sub_82DF31B0(ctx, base);
	// 825AC96C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AC970: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AC974: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825AC978: 4802D4E9  bl 0x825d9e60
	ctx.lr = 0x825AC97C;
	sub_825D9E60(ctx, base);
	// 825AC97C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AC980: 48846AA9  bl 0x82df3428
	ctx.lr = 0x825AC984;
	sub_82DF3428(ctx, base);
	// 825AC984: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AC988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AC98C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AC990: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AC994: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AC998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AC9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AC9A0 size=108
    let mut pc: u32 = 0x825AC9A0;
    'dispatch: loop {
        match pc {
            0x825AC9A0 => {
    //   block [0x825AC9A0..0x825ACA0C)
	// 825AC9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AC9A4: 48BFB7C5  bl 0x831a8168
	ctx.lr = 0x825AC9A8;
	sub_831A8130(ctx, base);
	// 825AC9A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AC9AC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825AC9B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AC9B4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825AC9B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AC9BC: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 825AC9C0: 48847049  bl 0x82df3a08
	ctx.lr = 0x825AC9C4;
	sub_82DF3A08(ctx, base);
	// 825AC9C4: 3BBE0048  addi r29, r30, 0x48
	ctx.r[29].s64 = ctx.r[30].s64 + 72;
	// 825AC9C8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825AC9CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825AC9D0: 48846939  bl 0x82df3308
	ctx.lr = 0x825AC9D4;
	sub_82DF3308(ctx, base);
	// 825AC9D4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825AC9D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AC9DC: 48846A4D  bl 0x82df3428
	ctx.lr = 0x825AC9E0;
	sub_82DF3428(ctx, base);
	// 825AC9E0: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AC9E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AC9E8: 41820010  beq 0x825ac9f8
	if ctx.cr[0].eq {
	pc = 0x825AC9F8; continue 'dispatch;
	}
	// 825AC9EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825AC9F0: 4BFF9A99  bl 0x825a6488
	ctx.lr = 0x825AC9F4;
	sub_825A6488(ctx, base);
	// 825AC9F4: 4800000C  b 0x825aca00
	pc = 0x825ACA00; continue 'dispatch;
	// 825AC9F8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825AC9FC: 48847205  bl 0x82df3c00
	ctx.lr = 0x825ACA00;
	sub_82DF3C00(ctx, base);
	// 825ACA00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825ACA04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ACA08: 48BFB7B0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ACA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825ACA10 size=12
    let mut pc: u32 = 0x825ACA10;
    'dispatch: loop {
        match pc {
            0x825ACA10 => {
    //   block [0x825ACA10..0x825ACA1C)
	// 825ACA10: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825ACA14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ACA18: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ACA1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825ACA1C size=24
    let mut pc: u32 = 0x825ACA1C;
    'dispatch: loop {
        match pc {
            0x825ACA1C => {
    //   block [0x825ACA1C..0x825ACA34)
	// 825ACA1C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ACA20: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ACA24: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825ACA28: 816A003C  lwz r11, 0x3c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(60 as u32) ) } as u64;
	// 825ACA2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825ACA30: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ACA34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825ACA34 size=4
    let mut pc: u32 = 0x825ACA34;
    'dispatch: loop {
        match pc {
            0x825ACA34 => {
    //   block [0x825ACA34..0x825ACA38)
	// 825ACA34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ACA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825ACA38 size=40
    let mut pc: u32 = 0x825ACA38;
    'dispatch: loop {
        match pc {
            0x825ACA38 => {
    //   block [0x825ACA38..0x825ACA60)
	// 825ACA38: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825ACA3C: C0030014  lfs f0, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ACA40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ACA44: 419A0008  beq cr6, 0x825aca4c
	if ctx.cr[6].eq {
	pc = 0x825ACA4C; continue 'dispatch;
	}
	// 825ACA48: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825ACA4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ACA50: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825ACA54: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825ACA58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825ACA5C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ACA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825ACA60 size=20
    let mut pc: u32 = 0x825ACA60;
    'dispatch: loop {
        match pc {
            0x825ACA60 => {
    //   block [0x825ACA60..0x825ACA74)
	// 825ACA60: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825ACA64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ACA68: 419A000C  beq cr6, 0x825aca74
	if ctx.cr[6].eq {
		sub_825ACA74(ctx, base);
		return;
	}
	// 825ACA6C: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ACA70: 48000008  b 0x825aca78
	sub_825ACA74(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ACA74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825ACA74 size=12
    let mut pc: u32 = 0x825ACA74;
    'dispatch: loop {
        match pc {
            0x825ACA74 => {
    //   block [0x825ACA74..0x825ACA80)
	// 825ACA74: C0030008  lfs f0, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ACA78: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825ACA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ACA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825ACA80 size=144
    let mut pc: u32 = 0x825ACA80;
    'dispatch: loop {
        match pc {
            0x825ACA80 => {
    //   block [0x825ACA80..0x825ACB10)
	// 825ACA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ACA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ACA88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825ACA8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ACA90: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825ACA94: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 825ACA98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825ACA9C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825ACAA0: 38CB7B78  addi r6, r11, 0x7b78
	ctx.r[6].s64 = ctx.r[11].s64 + 31608;
	// 825ACAA4: 38AA7B58  addi r5, r10, 0x7b58
	ctx.r[5].s64 = ctx.r[10].s64 + 31576;
	// 825ACAA8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ACAAC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825ACAB0: 48BFD499  bl 0x831a9f48
	ctx.lr = 0x825ACAB4;
	sub_831A9F48(ctx, base);
	// 825ACAB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825ACAB8: 41820044  beq 0x825acafc
	if ctx.cr[0].eq {
	pc = 0x825ACAFC; continue 'dispatch;
	}
	// 825ACABC: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825ACAC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ACAC4: 419A000C  beq cr6, 0x825acad0
	if ctx.cr[6].eq {
	pc = 0x825ACAD0; continue 'dispatch;
	}
	// 825ACAC8: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ACACC: 48000008  b 0x825acad4
	pc = 0x825ACAD4; continue 'dispatch;
	// 825ACAD0: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ACAD4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825ACAD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ACADC: 419A0008  beq cr6, 0x825acae4
	if ctx.cr[6].eq {
	pc = 0x825ACAE4; continue 'dispatch;
	}
	// 825ACAE0: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825ACAE4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ACAE8: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825ACAEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825ACAF0: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825ACAF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825ACAF8: 4E800421  bctrl
	ctx.lr = 0x825ACAFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825ACAFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825ACB00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ACB04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ACB08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825ACB0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ACB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825ACB10 size=20
    let mut pc: u32 = 0x825ACB10;
    'dispatch: loop {
        match pc {
            0x825ACB10 => {
    //   block [0x825ACB10..0x825ACB24)
	// 825ACB10: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825ACB14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ACB18: 419A000C  beq cr6, 0x825acb24
	if ctx.cr[6].eq {
		sub_825ACB24(ctx, base);
		return;
	}
	// 825ACB1C: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ACB20: 48000008  b 0x825acb28
	sub_825ACB24(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ACB24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825ACB24 size=60
    let mut pc: u32 = 0x825ACB24;
    'dispatch: loop {
        match pc {
            0x825ACB24 => {
    //   block [0x825ACB24..0x825ACB60)
	// 825ACB24: C0030008  lfs f0, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ACB28: C1A30020  lfs f13, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825ACB2C: EC0D007A  fmadds f0, f13, f1, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64);
	// 825ACB30: C1A3001C  lfs f13, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825ACB34: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 825ACB38: 40990008  ble cr6, 0x825acb40
	if !ctx.cr[6].gt {
	pc = 0x825ACB40; continue 'dispatch;
	}
	// 825ACB3C: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 825ACB40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ACB44: 419A0008  beq cr6, 0x825acb4c
	if ctx.cr[6].eq {
	pc = 0x825ACB4C; continue 'dispatch;
	}
	// 825ACB48: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825ACB4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ACB50: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825ACB54: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825ACB58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825ACB5C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ACB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825ACB60 size=20
    let mut pc: u32 = 0x825ACB60;
    'dispatch: loop {
        match pc {
            0x825ACB60 => {
    //   block [0x825ACB60..0x825ACB74)
	// 825ACB60: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825ACB64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ACB68: 419A000C  beq cr6, 0x825acb74
	if ctx.cr[6].eq {
		sub_825ACB74(ctx, base);
		return;
	}
	// 825ACB6C: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ACB70: 48000008  b 0x825acb78
	sub_825ACB74(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ACB74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825ACB74 size=76
    let mut pc: u32 = 0x825ACB74;
    'dispatch: loop {
        match pc {
            0x825ACB74 => {
    //   block [0x825ACB74..0x825ACBC0)
	// 825ACB74: C0030008  lfs f0, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ACB78: C1A30020  lfs f13, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825ACB7C: EC0D007C  fnmsubs f0, f13, f1, f0
	ctx.f[0].f64 = -(((ctx.f[13].f64 * ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 825ACB80: C1A30018  lfs f13, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825ACB84: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 825ACB88: 40980008  bge cr6, 0x825acb90
	if !ctx.cr[6].lt {
	pc = 0x825ACB90; continue 'dispatch;
	}
	// 825ACB8C: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 825ACB90: C183001C  lfs f12, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825ACB94: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 825ACB98: 40990008  ble cr6, 0x825acba0
	if !ctx.cr[6].gt {
	pc = 0x825ACBA0; continue 'dispatch;
	}
	// 825ACB9C: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 825ACBA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ACBA4: 419A0008  beq cr6, 0x825acbac
	if ctx.cr[6].eq {
	pc = 0x825ACBAC; continue 'dispatch;
	}
	// 825ACBA8: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825ACBAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ACBB0: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825ACBB4: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825ACBB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825ACBBC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ACBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825ACBC0 size=20
    let mut pc: u32 = 0x825ACBC0;
    'dispatch: loop {
        match pc {
            0x825ACBC0 => {
    //   block [0x825ACBC0..0x825ACBD4)
	// 825ACBC0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825ACBC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ACBC8: 419A000C  beq cr6, 0x825acbd4
	if ctx.cr[6].eq {
		sub_825ACBD4(ctx, base);
		return;
	}
	// 825ACBCC: C1AB0000  lfs f13, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825ACBD0: 48000008  b 0x825acbd8
	sub_825ACBD4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ACBD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825ACBD4 size=68
    let mut pc: u32 = 0x825ACBD4;
    'dispatch: loop {
        match pc {
            0x825ACBD4 => {
    //   block [0x825ACBD4..0x825ACC18)
	// 825ACBD4: C1A30008  lfs f13, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825ACBD8: C003001C  lfs f0, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ACBDC: C1830018  lfs f12, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825ACBE0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 825ACBE4: 41980008  blt cr6, 0x825acbec
	if ctx.cr[6].lt {
	pc = 0x825ACBEC; continue 'dispatch;
	}
	// 825ACBE8: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 825ACBEC: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 825ACBF0: 40990008  ble cr6, 0x825acbf8
	if !ctx.cr[6].gt {
	pc = 0x825ACBF8; continue 'dispatch;
	}
	// 825ACBF4: FC006090  fmr f0, f12
	ctx.f[0].f64 = ctx.f[12].f64;
	// 825ACBF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ACBFC: 419A0008  beq cr6, 0x825acc04
	if ctx.cr[6].eq {
	pc = 0x825ACC04; continue 'dispatch;
	}
	// 825ACC00: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825ACC04: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ACC08: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825ACC0C: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825ACC10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825ACC14: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ACC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825ACC18 size=164
    let mut pc: u32 = 0x825ACC18;
    'dispatch: loop {
        match pc {
            0x825ACC18 => {
    //   block [0x825ACC18..0x825ACCBC)
	// 825ACC18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ACC1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ACC20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825ACC24: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 825ACC28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ACC2C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 825ACC30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825ACC34: C00B89AC  lfs f0, -0x7654(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ACC38: EFE10032  fmuls f31, f1, f0
	ctx.f[31].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 825ACC3C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ACC40: 816A0034  lwz r11, 0x34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 825ACC44: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825ACC48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825ACC4C: 4E800421  bctrl
	ctx.lr = 0x825ACC50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825ACC50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825ACC54: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825ACC58: 4BFFF9C9  bl 0x825ac620
	ctx.lr = 0x825ACC5C;
	sub_825AC620(ctx, base);
	// 825ACC5C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825ACC60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ACC64: 419A000C  beq cr6, 0x825acc70
	if ctx.cr[6].eq {
	pc = 0x825ACC70; continue 'dispatch;
	}
	// 825ACC68: C1AB0000  lfs f13, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825ACC6C: 48000008  b 0x825acc74
	pc = 0x825ACC74; continue 'dispatch;
	// 825ACC70: C1BF0008  lfs f13, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825ACC74: C01F001C  lfs f0, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ACC78: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 825ACC7C: 40990028  ble cr6, 0x825acca4
	if !ctx.cr[6].gt {
	pc = 0x825ACCA4; continue 'dispatch;
	}
	// 825ACC80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ACC84: 419A0008  beq cr6, 0x825acc8c
	if ctx.cr[6].eq {
	pc = 0x825ACC8C; continue 'dispatch;
	}
	// 825ACC88: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825ACC8C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ACC90: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825ACC94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825ACC98: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825ACC9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825ACCA0: 4E800421  bctrl
	ctx.lr = 0x825ACCA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825ACCA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825ACCA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ACCAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ACCB0: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825ACCB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825ACCB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ACCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825ACCC0 size=164
    let mut pc: u32 = 0x825ACCC0;
    'dispatch: loop {
        match pc {
            0x825ACCC0 => {
    //   block [0x825ACCC0..0x825ACD64)
	// 825ACCC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ACCC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ACCC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825ACCCC: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 825ACCD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ACCD4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 825ACCD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825ACCDC: C00B89AC  lfs f0, -0x7654(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ACCE0: EFE10032  fmuls f31, f1, f0
	ctx.f[31].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 825ACCE4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ACCE8: 816A0038  lwz r11, 0x38(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(56 as u32) ) } as u64;
	// 825ACCEC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825ACCF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825ACCF4: 4E800421  bctrl
	ctx.lr = 0x825ACCF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825ACCF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825ACCFC: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825ACD00: 4BFFF921  bl 0x825ac620
	ctx.lr = 0x825ACD04;
	sub_825AC620(ctx, base);
	// 825ACD04: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825ACD08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ACD0C: 419A000C  beq cr6, 0x825acd18
	if ctx.cr[6].eq {
	pc = 0x825ACD18; continue 'dispatch;
	}
	// 825ACD10: C1AB0000  lfs f13, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825ACD14: 48000008  b 0x825acd1c
	pc = 0x825ACD1C; continue 'dispatch;
	// 825ACD18: C1BF0008  lfs f13, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825ACD1C: C01F0018  lfs f0, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ACD20: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 825ACD24: 40980028  bge cr6, 0x825acd4c
	if !ctx.cr[6].lt {
	pc = 0x825ACD4C; continue 'dispatch;
	}
	// 825ACD28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ACD2C: 419A0008  beq cr6, 0x825acd34
	if ctx.cr[6].eq {
	pc = 0x825ACD34; continue 'dispatch;
	}
	// 825ACD30: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825ACD34: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ACD38: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825ACD3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825ACD40: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825ACD44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825ACD48: 4E800421  bctrl
	ctx.lr = 0x825ACD4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825ACD4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825ACD50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ACD54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ACD58: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825ACD5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825ACD60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ACD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825ACD68 size=40
    let mut pc: u32 = 0x825ACD68;
    'dispatch: loop {
        match pc {
            0x825ACD68 => {
    //   block [0x825ACD68..0x825ACD90)
	// 825ACD68: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825ACD6C: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 825ACD70: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825ACD74: 419A0008  beq cr6, 0x825acd7c
	if ctx.cr[6].eq {
	pc = 0x825ACD7C; continue 'dispatch;
	}
	// 825ACD78: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825ACD7C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825ACD80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ACD84: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825ACD88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825ACD8C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ACD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ACD90 size=144
    let mut pc: u32 = 0x825ACD90;
    'dispatch: loop {
        match pc {
            0x825ACD90 => {
    //   block [0x825ACD90..0x825ACE20)
	// 825ACD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ACD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ACD98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825ACD9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ACDA0: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825ACDA4: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 825ACDA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825ACDAC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825ACDB0: 38CB7BE4  addi r6, r11, 0x7be4
	ctx.r[6].s64 = ctx.r[11].s64 + 31716;
	// 825ACDB4: 38AA7B58  addi r5, r10, 0x7b58
	ctx.r[5].s64 = ctx.r[10].s64 + 31576;
	// 825ACDB8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ACDBC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825ACDC0: 48BFD189  bl 0x831a9f48
	ctx.lr = 0x825ACDC4;
	sub_831A9F48(ctx, base);
	// 825ACDC4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825ACDC8: 41820044  beq 0x825ace0c
	if ctx.cr[0].eq {
	pc = 0x825ACE0C; continue 'dispatch;
	}
	// 825ACDCC: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825ACDD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ACDD4: 419A000C  beq cr6, 0x825acde0
	if ctx.cr[6].eq {
	pc = 0x825ACDE0; continue 'dispatch;
	}
	// 825ACDD8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ACDDC: 48000008  b 0x825acde4
	pc = 0x825ACDE4; continue 'dispatch;
	// 825ACDE0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ACDE4: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825ACDE8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825ACDEC: 419A0008  beq cr6, 0x825acdf4
	if ctx.cr[6].eq {
	pc = 0x825ACDF4; continue 'dispatch;
	}
	// 825ACDF0: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825ACDF4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825ACDF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825ACDFC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ACE00: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825ACE04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825ACE08: 4E800421  bctrl
	ctx.lr = 0x825ACE0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825ACE0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825ACE10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ACE14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ACE18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825ACE1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ACE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ACE20 size=124
    let mut pc: u32 = 0x825ACE20;
    'dispatch: loop {
        match pc {
            0x825ACE20 => {
    //   block [0x825ACE20..0x825ACE9C)
	// 825ACE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ACE24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ACE28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ACE2C: 81030010  lwz r8, 0x10(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825ACE30: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 825ACE34: 419A000C  beq cr6, 0x825ace40
	if ctx.cr[6].eq {
	pc = 0x825ACE40; continue 'dispatch;
	}
	// 825ACE38: 81280000  lwz r9, 0(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ACE3C: 48000008  b 0x825ace44
	pc = 0x825ACE44; continue 'dispatch;
	// 825ACE40: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ACE44: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 825ACE48: FC000E5E  fctidz f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].s64 = if ctx.f[1].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[1].f64.trunc() as i64 };
	// 825ACE4C: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 825ACE50: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825ACE54: 7D6B51D6  mullw r11, r11, r10
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 825ACE58: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 825ACE5C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 825ACE60: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825ACE64: 40990008  ble cr6, 0x825ace6c
	if !ctx.cr[6].gt {
	pc = 0x825ACE6C; continue 'dispatch;
	}
	// 825ACE68: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 825ACE6C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 825ACE70: 419A0008  beq cr6, 0x825ace78
	if ctx.cr[6].eq {
	pc = 0x825ACE78; continue 'dispatch;
	}
	// 825ACE74: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825ACE78: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825ACE7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ACE80: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825ACE84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825ACE88: 4E800421  bctrl
	ctx.lr = 0x825ACE8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825ACE8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825ACE90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ACE94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ACE98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ACEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ACEA0 size=140
    let mut pc: u32 = 0x825ACEA0;
    'dispatch: loop {
        match pc {
            0x825ACEA0 => {
    //   block [0x825ACEA0..0x825ACF2C)
	// 825ACEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ACEA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ACEA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ACEAC: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825ACEB0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 825ACEB4: 419A000C  beq cr6, 0x825acec0
	if ctx.cr[6].eq {
	pc = 0x825ACEC0; continue 'dispatch;
	}
	// 825ACEB8: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ACEBC: 48000008  b 0x825acec4
	pc = 0x825ACEC4; continue 'dispatch;
	// 825ACEC0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ACEC4: 81430020  lwz r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 825ACEC8: FC000E5E  fctidz f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].s64 = if ctx.f[1].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[1].f64.trunc() as i64 };
	// 825ACECC: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 825ACED0: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825ACED4: 7D0A41D6  mullw r8, r10, r8
	ctx.r[8].s64 = (ctx.r[10].s32 as i64) * (ctx.r[8].s32 as i64);
	// 825ACED8: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 825ACEDC: 7D685850  subf r11, r8, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 825ACEE0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825ACEE4: 40980008  bge cr6, 0x825aceec
	if !ctx.cr[6].lt {
	pc = 0x825ACEEC; continue 'dispatch;
	}
	// 825ACEE8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 825ACEEC: 8103001C  lwz r8, 0x1c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 825ACEF0: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 825ACEF4: 40990008  ble cr6, 0x825acefc
	if !ctx.cr[6].gt {
	pc = 0x825ACEFC; continue 'dispatch;
	}
	// 825ACEF8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 825ACEFC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 825ACF00: 419A0008  beq cr6, 0x825acf08
	if ctx.cr[6].eq {
	pc = 0x825ACF08; continue 'dispatch;
	}
	// 825ACF04: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825ACF08: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825ACF0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ACF10: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825ACF14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825ACF18: 4E800421  bctrl
	ctx.lr = 0x825ACF1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825ACF1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825ACF20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ACF24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ACF28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ACF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825ACF30 size=20
    let mut pc: u32 = 0x825ACF30;
    'dispatch: loop {
        match pc {
            0x825ACF30 => {
    //   block [0x825ACF30..0x825ACF44)
	// 825ACF30: 81030010  lwz r8, 0x10(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825ACF34: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 825ACF38: 419A000C  beq cr6, 0x825acf44
	if ctx.cr[6].eq {
		sub_825ACF44(ctx, base);
		return;
	}
	// 825ACF3C: 81280000  lwz r9, 0(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ACF40: 48000008  b 0x825acf48
	sub_825ACF44(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ACF44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825ACF44 size=68
    let mut pc: u32 = 0x825ACF44;
    'dispatch: loop {
        match pc {
            0x825ACF44 => {
    //   block [0x825ACF44..0x825ACF88)
	// 825ACF44: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ACF48: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 825ACF4C: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 825ACF50: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825ACF54: 41980008  blt cr6, 0x825acf5c
	if ctx.cr[6].lt {
	pc = 0x825ACF5C; continue 'dispatch;
	}
	// 825ACF58: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 825ACF5C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825ACF60: 41990008  bgt cr6, 0x825acf68
	if ctx.cr[6].gt {
	pc = 0x825ACF68; continue 'dispatch;
	}
	// 825ACF64: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 825ACF68: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 825ACF6C: 419A0008  beq cr6, 0x825acf74
	if ctx.cr[6].eq {
	pc = 0x825ACF74; continue 'dispatch;
	}
	// 825ACF70: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825ACF74: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825ACF78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ACF7C: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825ACF80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825ACF84: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ACF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825ACF88 size=164
    let mut pc: u32 = 0x825ACF88;
    'dispatch: loop {
        match pc {
            0x825ACF88 => {
    //   block [0x825ACF88..0x825AD02C)
	// 825ACF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ACF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ACF90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825ACF94: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 825ACF98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ACF9C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 825ACFA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825ACFA4: C00B89AC  lfs f0, -0x7654(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ACFA8: EFE10032  fmuls f31, f1, f0
	ctx.f[31].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 825ACFAC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ACFB0: 816A0034  lwz r11, 0x34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 825ACFB4: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825ACFB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825ACFBC: 4E800421  bctrl
	ctx.lr = 0x825ACFC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825ACFC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825ACFC4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825ACFC8: 4BFFF779  bl 0x825ac740
	ctx.lr = 0x825ACFCC;
	sub_825AC740(ctx, base);
	// 825ACFCC: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825ACFD0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 825ACFD4: 419A000C  beq cr6, 0x825acfe0
	if ctx.cr[6].eq {
	pc = 0x825ACFE0; continue 'dispatch;
	}
	// 825ACFD8: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ACFDC: 48000008  b 0x825acfe4
	pc = 0x825ACFE4; continue 'dispatch;
	// 825ACFE0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ACFE4: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 825ACFE8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825ACFEC: 40990028  ble cr6, 0x825ad014
	if !ctx.cr[6].gt {
	pc = 0x825AD014; continue 'dispatch;
	}
	// 825ACFF0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 825ACFF4: 419A0008  beq cr6, 0x825acffc
	if ctx.cr[6].eq {
	pc = 0x825ACFFC; continue 'dispatch;
	}
	// 825ACFF8: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825ACFFC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825AD000: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD004: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD008: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AD00C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD010: 4E800421  bctrl
	ctx.lr = 0x825AD014;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD014: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AD018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AD01C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AD020: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AD024: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AD028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AD030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825AD030 size=164
    let mut pc: u32 = 0x825AD030;
    'dispatch: loop {
        match pc {
            0x825AD030 => {
    //   block [0x825AD030..0x825AD0D4)
	// 825AD030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AD034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AD038: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AD03C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 825AD040: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AD044: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 825AD048: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AD04C: C00B89AC  lfs f0, -0x7654(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825AD050: EFE10032  fmuls f31, f1, f0
	ctx.f[31].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 825AD054: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD058: 816A0038  lwz r11, 0x38(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(56 as u32) ) } as u64;
	// 825AD05C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825AD060: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD064: 4E800421  bctrl
	ctx.lr = 0x825AD068;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD068: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD06C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825AD070: 4BFFF6D1  bl 0x825ac740
	ctx.lr = 0x825AD074;
	sub_825AC740(ctx, base);
	// 825AD074: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AD078: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 825AD07C: 419A000C  beq cr6, 0x825ad088
	if ctx.cr[6].eq {
	pc = 0x825AD088; continue 'dispatch;
	}
	// 825AD080: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD084: 48000008  b 0x825ad08c
	pc = 0x825AD08C; continue 'dispatch;
	// 825AD088: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AD08C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 825AD090: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AD094: 40980028  bge cr6, 0x825ad0bc
	if !ctx.cr[6].lt {
	pc = 0x825AD0BC; continue 'dispatch;
	}
	// 825AD098: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 825AD09C: 419A0008  beq cr6, 0x825ad0a4
	if ctx.cr[6].eq {
	pc = 0x825AD0A4; continue 'dispatch;
	}
	// 825AD0A0: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AD0A4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825AD0A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD0AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD0B0: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AD0B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD0B8: 4E800421  bctrl
	ctx.lr = 0x825AD0BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD0BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AD0C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AD0C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AD0C8: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AD0CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AD0D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AD0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AD0D8 size=144
    let mut pc: u32 = 0x825AD0D8;
    'dispatch: loop {
        match pc {
            0x825AD0D8 => {
    //   block [0x825AD0D8..0x825AD168)
	// 825AD0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AD0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AD0E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AD0E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AD0E8: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825AD0EC: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 825AD0F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AD0F4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825AD0F8: 38CB7BC0  addi r6, r11, 0x7bc0
	ctx.r[6].s64 = ctx.r[11].s64 + 31680;
	// 825AD0FC: 38AA7B58  addi r5, r10, 0x7b58
	ctx.r[5].s64 = ctx.r[10].s64 + 31576;
	// 825AD100: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825AD104: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825AD108: 48BFCE41  bl 0x831a9f48
	ctx.lr = 0x825AD10C;
	sub_831A9F48(ctx, base);
	// 825AD10C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825AD110: 41820044  beq 0x825ad154
	if ctx.cr[0].eq {
	pc = 0x825AD154; continue 'dispatch;
	}
	// 825AD114: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AD118: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AD11C: 419A000C  beq cr6, 0x825ad128
	if ctx.cr[6].eq {
	pc = 0x825AD128; continue 'dispatch;
	}
	// 825AD120: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD124: 48000008  b 0x825ad12c
	pc = 0x825AD12C; continue 'dispatch;
	// 825AD128: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AD12C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AD130: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825AD134: 419A0008  beq cr6, 0x825ad13c
	if ctx.cr[6].eq {
	pc = 0x825AD13C; continue 'dispatch;
	}
	// 825AD138: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AD13C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825AD140: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD144: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD148: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AD14C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD150: 4E800421  bctrl
	ctx.lr = 0x825AD154;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD154: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AD158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AD15C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AD160: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AD164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AD168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AD168 size=20
    let mut pc: u32 = 0x825AD168;
    'dispatch: loop {
        match pc {
            0x825AD168 => {
    //   block [0x825AD168..0x825AD17C)
	// 825AD168: 81030010  lwz r8, 0x10(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AD16C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 825AD170: 419A000C  beq cr6, 0x825ad17c
	if ctx.cr[6].eq {
		sub_825AD17C(ctx, base);
		return;
	}
	// 825AD174: 81280000  lwz r9, 0(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD178: 48000008  b 0x825ad180
	sub_825AD17C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AD17C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AD17C size=68
    let mut pc: u32 = 0x825AD17C;
    'dispatch: loop {
        match pc {
            0x825AD17C => {
    //   block [0x825AD17C..0x825AD1C0)
	// 825AD17C: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AD180: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 825AD184: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 825AD188: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 825AD18C: 41980008  blt cr6, 0x825ad194
	if ctx.cr[6].lt {
	pc = 0x825AD194; continue 'dispatch;
	}
	// 825AD190: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 825AD194: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 825AD198: 41990008  bgt cr6, 0x825ad1a0
	if ctx.cr[6].gt {
	pc = 0x825AD1A0; continue 'dispatch;
	}
	// 825AD19C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 825AD1A0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 825AD1A4: 419A0008  beq cr6, 0x825ad1ac
	if ctx.cr[6].eq {
	pc = 0x825AD1AC; continue 'dispatch;
	}
	// 825AD1A8: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AD1AC: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825AD1B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD1B4: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AD1B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD1BC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AD1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825AD1C0 size=164
    let mut pc: u32 = 0x825AD1C0;
    'dispatch: loop {
        match pc {
            0x825AD1C0 => {
    //   block [0x825AD1C0..0x825AD264)
	// 825AD1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AD1C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AD1C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AD1CC: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 825AD1D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AD1D4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 825AD1D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AD1DC: C00B89AC  lfs f0, -0x7654(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825AD1E0: EFE10032  fmuls f31, f1, f0
	ctx.f[31].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 825AD1E4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD1E8: 816A0038  lwz r11, 0x38(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(56 as u32) ) } as u64;
	// 825AD1EC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825AD1F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD1F4: 4E800421  bctrl
	ctx.lr = 0x825AD1F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD1F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD1FC: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825AD200: 4BFFF671  bl 0x825ac870
	ctx.lr = 0x825AD204;
	sub_825AC870(ctx, base);
	// 825AD204: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AD208: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 825AD20C: 419A000C  beq cr6, 0x825ad218
	if ctx.cr[6].eq {
	pc = 0x825AD218; continue 'dispatch;
	}
	// 825AD210: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD214: 48000008  b 0x825ad21c
	pc = 0x825AD21C; continue 'dispatch;
	// 825AD218: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AD21C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 825AD220: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825AD224: 40980028  bge cr6, 0x825ad24c
	if !ctx.cr[6].lt {
	pc = 0x825AD24C; continue 'dispatch;
	}
	// 825AD228: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 825AD22C: 419A0008  beq cr6, 0x825ad234
	if ctx.cr[6].eq {
	pc = 0x825AD234; continue 'dispatch;
	}
	// 825AD230: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AD234: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825AD238: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD23C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD240: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AD244: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD248: 4E800421  bctrl
	ctx.lr = 0x825AD24C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD24C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AD250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AD254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AD258: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AD25C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AD260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AD268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AD268 size=20
    let mut pc: u32 = 0x825AD268;
    'dispatch: loop {
        match pc {
            0x825AD268 => {
    //   block [0x825AD268..0x825AD27C)
	// 825AD268: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AD26C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AD270: 419A000C  beq cr6, 0x825ad27c
	if ctx.cr[6].eq {
		sub_825AD27C(ctx, base);
		return;
	}
	// 825AD274: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD278: 48000008  b 0x825ad280
	sub_825AD27C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AD27C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AD27C size=12
    let mut pc: u32 = 0x825AD27C;
    'dispatch: loop {
        match pc {
            0x825AD27C => {
    //   block [0x825AD27C..0x825AD288)
	// 825AD27C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AD280: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 825AD284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AD288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AD288 size=144
    let mut pc: u32 = 0x825AD288;
    'dispatch: loop {
        match pc {
            0x825AD288 => {
    //   block [0x825AD288..0x825AD318)
	// 825AD288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AD28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AD290: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AD294: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AD298: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825AD29C: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 825AD2A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AD2A4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825AD2A8: 38CB7B9C  addi r6, r11, 0x7b9c
	ctx.r[6].s64 = ctx.r[11].s64 + 31644;
	// 825AD2AC: 38AA7B58  addi r5, r10, 0x7b58
	ctx.r[5].s64 = ctx.r[10].s64 + 31576;
	// 825AD2B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825AD2B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825AD2B8: 48BFCC91  bl 0x831a9f48
	ctx.lr = 0x825AD2BC;
	sub_831A9F48(ctx, base);
	// 825AD2BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825AD2C0: 41820044  beq 0x825ad304
	if ctx.cr[0].eq {
	pc = 0x825AD304; continue 'dispatch;
	}
	// 825AD2C4: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AD2C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AD2CC: 419A000C  beq cr6, 0x825ad2d8
	if ctx.cr[6].eq {
	pc = 0x825AD2D8; continue 'dispatch;
	}
	// 825AD2D0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD2D4: 48000008  b 0x825ad2dc
	pc = 0x825AD2DC; continue 'dispatch;
	// 825AD2D8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AD2DC: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AD2E0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825AD2E4: 419A0008  beq cr6, 0x825ad2ec
	if ctx.cr[6].eq {
	pc = 0x825AD2EC; continue 'dispatch;
	}
	// 825AD2E8: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AD2EC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825AD2F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD2F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD2F8: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AD2FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD300: 4E800421  bctrl
	ctx.lr = 0x825AD304;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD304: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AD308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AD30C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AD310: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AD314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AD318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AD318 size=124
    let mut pc: u32 = 0x825AD318;
    'dispatch: loop {
        match pc {
            0x825AD318 => {
    //   block [0x825AD318..0x825AD394)
	// 825AD318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AD31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AD320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AD324: 81030010  lwz r8, 0x10(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AD328: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 825AD32C: 419A000C  beq cr6, 0x825ad338
	if ctx.cr[6].eq {
	pc = 0x825AD338; continue 'dispatch;
	}
	// 825AD330: 81280000  lwz r9, 0(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD334: 48000008  b 0x825ad33c
	pc = 0x825AD33C; continue 'dispatch;
	// 825AD338: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AD33C: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 825AD340: FC00081E  fctiwz f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].s64 = if ctx.f[1].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[1].f64.trunc() as i32 as i64 };
	// 825AD344: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 825AD348: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825AD34C: 7D6B51D6  mullw r11, r11, r10
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 825AD350: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 825AD354: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 825AD358: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 825AD35C: 40990008  ble cr6, 0x825ad364
	if !ctx.cr[6].gt {
	pc = 0x825AD364; continue 'dispatch;
	}
	// 825AD360: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 825AD364: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 825AD368: 419A0008  beq cr6, 0x825ad370
	if ctx.cr[6].eq {
	pc = 0x825AD370; continue 'dispatch;
	}
	// 825AD36C: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AD370: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825AD374: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD378: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AD37C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD380: 4E800421  bctrl
	ctx.lr = 0x825AD384;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD384: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AD388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AD38C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AD390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AD398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AD398 size=140
    let mut pc: u32 = 0x825AD398;
    'dispatch: loop {
        match pc {
            0x825AD398 => {
    //   block [0x825AD398..0x825AD424)
	// 825AD398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AD39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AD3A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AD3A4: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AD3A8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 825AD3AC: 419A000C  beq cr6, 0x825ad3b8
	if ctx.cr[6].eq {
	pc = 0x825AD3B8; continue 'dispatch;
	}
	// 825AD3B0: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD3B4: 48000008  b 0x825ad3bc
	pc = 0x825AD3BC; continue 'dispatch;
	// 825AD3B8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AD3BC: 81430020  lwz r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 825AD3C0: FC00081E  fctiwz f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].s64 = if ctx.f[1].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[1].f64.trunc() as i32 as i64 };
	// 825AD3C4: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 825AD3C8: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825AD3CC: 7D0A41D6  mullw r8, r10, r8
	ctx.r[8].s64 = (ctx.r[10].s32 as i64) * (ctx.r[8].s32 as i64);
	// 825AD3D0: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 825AD3D4: 7D685850  subf r11, r8, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 825AD3D8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 825AD3DC: 40980008  bge cr6, 0x825ad3e4
	if !ctx.cr[6].lt {
	pc = 0x825AD3E4; continue 'dispatch;
	}
	// 825AD3E0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 825AD3E4: 8103001C  lwz r8, 0x1c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 825AD3E8: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 825AD3EC: 40990008  ble cr6, 0x825ad3f4
	if !ctx.cr[6].gt {
	pc = 0x825AD3F4; continue 'dispatch;
	}
	// 825AD3F0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 825AD3F4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 825AD3F8: 419A0008  beq cr6, 0x825ad400
	if ctx.cr[6].eq {
	pc = 0x825AD400; continue 'dispatch;
	}
	// 825AD3FC: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AD400: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825AD404: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD408: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AD40C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD410: 4E800421  bctrl
	ctx.lr = 0x825AD414;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD414: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AD418: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AD41C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AD420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AD428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AD428 size=20
    let mut pc: u32 = 0x825AD428;
    'dispatch: loop {
        match pc {
            0x825AD428 => {
    //   block [0x825AD428..0x825AD43C)
	// 825AD428: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AD42C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825AD430: 419A000C  beq cr6, 0x825ad43c
	if ctx.cr[6].eq {
		sub_825AD43C(ctx, base);
		return;
	}
	// 825AD434: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD438: 48000008  b 0x825ad440
	sub_825AD43C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AD43C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AD43C size=36
    let mut pc: u32 = 0x825AD43C;
    'dispatch: loop {
        match pc {
            0x825AD43C => {
    //   block [0x825AD43C..0x825AD460)
	// 825AD43C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AD440: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825AD444: 419A0008  beq cr6, 0x825ad44c
	if ctx.cr[6].eq {
	pc = 0x825AD44C; continue 'dispatch;
	}
	// 825AD448: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AD44C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825AD450: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD454: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AD458: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD45C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AD460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825AD460 size=164
    let mut pc: u32 = 0x825AD460;
    'dispatch: loop {
        match pc {
            0x825AD460 => {
    //   block [0x825AD460..0x825AD504)
	// 825AD460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AD464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AD468: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AD46C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 825AD470: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AD474: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 825AD478: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AD47C: C00B89AC  lfs f0, -0x7654(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825AD480: EFE10032  fmuls f31, f1, f0
	ctx.f[31].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 825AD484: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD488: 816A0034  lwz r11, 0x34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 825AD48C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825AD490: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD494: 4E800421  bctrl
	ctx.lr = 0x825AD498;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD498: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD49C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825AD4A0: 4BFFF3D1  bl 0x825ac870
	ctx.lr = 0x825AD4A4;
	sub_825AC870(ctx, base);
	// 825AD4A4: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AD4A8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 825AD4AC: 419A000C  beq cr6, 0x825ad4b8
	if ctx.cr[6].eq {
	pc = 0x825AD4B8; continue 'dispatch;
	}
	// 825AD4B0: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD4B4: 48000008  b 0x825ad4bc
	pc = 0x825AD4BC; continue 'dispatch;
	// 825AD4B8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AD4BC: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 825AD4C0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825AD4C4: 40990028  ble cr6, 0x825ad4ec
	if !ctx.cr[6].gt {
	pc = 0x825AD4EC; continue 'dispatch;
	}
	// 825AD4C8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 825AD4CC: 419A0008  beq cr6, 0x825ad4d4
	if ctx.cr[6].eq {
	pc = 0x825AD4D4; continue 'dispatch;
	}
	// 825AD4D0: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AD4D4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825AD4D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD4DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD4E0: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AD4E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD4E8: 4E800421  bctrl
	ctx.lr = 0x825AD4EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD4EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AD4F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AD4F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AD4F8: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AD4FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AD500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AD508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825AD508 size=444
    let mut pc: u32 = 0x825AD508;
    'dispatch: loop {
        match pc {
            0x825AD508 => {
    //   block [0x825AD508..0x825AD6C4)
	// 825AD508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AD50C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AD510: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AD514: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AD518: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 825AD51C: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 825AD520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AD524: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825AD528: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825AD52C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD530: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD534: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 825AD538: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD53C: 4E800421  bctrl
	ctx.lr = 0x825AD540;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD540: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD544: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD548: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 825AD54C: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 825AD550: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD554: 4E800421  bctrl
	ctx.lr = 0x825AD558;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD558: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD55C: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 825AD560: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD564: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 825AD568: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD56C: 4E800421  bctrl
	ctx.lr = 0x825AD570;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD570: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AD574: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD578: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD57C: 418200A4  beq 0x825ad620
	if ctx.cr[0].eq {
	pc = 0x825AD620; continue 'dispatch;
	}
	// 825AD580: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 825AD584: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD588: 4E800421  bctrl
	ctx.lr = 0x825AD58C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD58C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AD590: 41820018  beq 0x825ad5a8
	if ctx.cr[0].eq {
	pc = 0x825AD5A8; continue 'dispatch;
	}
	// 825AD594: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AD598: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 825AD59C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825AD5A0: 4BFFF679  bl 0x825acc18
	ctx.lr = 0x825AD5A4;
	sub_825ACC18(ctx, base);
	// 825AD5A4: 480000D0  b 0x825ad674
	pc = 0x825AD674; continue 'dispatch;
	// 825AD5A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD5AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD5B0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 825AD5B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD5B8: 4E800421  bctrl
	ctx.lr = 0x825AD5BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD5BC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AD5C0: 41820018  beq 0x825ad5d8
	if ctx.cr[0].eq {
	pc = 0x825AD5D8; continue 'dispatch;
	}
	// 825AD5C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AD5C8: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 825AD5CC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825AD5D0: 4BFFF6F1  bl 0x825accc0
	ctx.lr = 0x825AD5D4;
	sub_825ACCC0(ctx, base);
	// 825AD5D4: 480000A0  b 0x825ad674
	pc = 0x825AD674; continue 'dispatch;
	// 825AD5D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD5DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD5E0: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 825AD5E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD5E8: 4E800421  bctrl
	ctx.lr = 0x825AD5EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD5EC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AD5F0: 4082FFA4  bne 0x825ad594
	if !ctx.cr[0].eq {
	pc = 0x825AD594; continue 'dispatch;
	}
	// 825AD5F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD5F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD5FC: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AD600: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD604: 4E800421  bctrl
	ctx.lr = 0x825AD608;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD608: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AD60C: 4082FFB8  bne 0x825ad5c4
	if !ctx.cr[0].eq {
	pc = 0x825AD5C4; continue 'dispatch;
	}
	// 825AD610: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825AD614: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825AD618: D01E0050  stfs f0, 0x50(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825AD61C: 48000088  b 0x825ad6a4
	pc = 0x825AD6A4; continue 'dispatch;
	// 825AD620: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 825AD624: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD628: 4E800421  bctrl
	ctx.lr = 0x825AD62C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD62C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AD630: 41820010  beq 0x825ad640
	if ctx.cr[0].eq {
	pc = 0x825AD640; continue 'dispatch;
	}
	// 825AD634: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD638: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 825AD63C: 48000028  b 0x825ad664
	pc = 0x825AD664; continue 'dispatch;
	// 825AD640: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD644: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD648: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AD64C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD650: 4E800421  bctrl
	ctx.lr = 0x825AD654;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD654: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AD658: 4182004C  beq 0x825ad6a4
	if ctx.cr[0].eq {
	pc = 0x825AD6A4; continue 'dispatch;
	}
	// 825AD65C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD660: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 825AD664: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AD668: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825AD66C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD670: 4E800421  bctrl
	ctx.lr = 0x825AD674;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD674: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD678: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825AD67C: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD680: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AD684: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 825AD688: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD68C: 4E800421  bctrl
	ctx.lr = 0x825AD690;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD690: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AD694: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AD698: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD69C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD6A0: 4E800421  bctrl
	ctx.lr = 0x825AD6A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD6A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825AD6A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AD6AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AD6B0: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 825AD6B4: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 825AD6B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AD6BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AD6C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AD6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825AD6C8 size=444
    let mut pc: u32 = 0x825AD6C8;
    'dispatch: loop {
        match pc {
            0x825AD6C8 => {
    //   block [0x825AD6C8..0x825AD884)
	// 825AD6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AD6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AD6D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AD6D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AD6D8: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 825AD6DC: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 825AD6E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AD6E4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825AD6E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825AD6EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD6F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD6F4: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 825AD6F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD6FC: 4E800421  bctrl
	ctx.lr = 0x825AD700;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD700: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD704: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD708: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 825AD70C: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 825AD710: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD714: 4E800421  bctrl
	ctx.lr = 0x825AD718;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD718: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD71C: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 825AD720: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD724: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 825AD728: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD72C: 4E800421  bctrl
	ctx.lr = 0x825AD730;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD730: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AD734: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD738: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD73C: 418200A4  beq 0x825ad7e0
	if ctx.cr[0].eq {
	pc = 0x825AD7E0; continue 'dispatch;
	}
	// 825AD740: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 825AD744: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD748: 4E800421  bctrl
	ctx.lr = 0x825AD74C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD74C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AD750: 41820018  beq 0x825ad768
	if ctx.cr[0].eq {
	pc = 0x825AD768; continue 'dispatch;
	}
	// 825AD754: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AD758: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 825AD75C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825AD760: 4BFFF829  bl 0x825acf88
	ctx.lr = 0x825AD764;
	sub_825ACF88(ctx, base);
	// 825AD764: 480000D0  b 0x825ad834
	pc = 0x825AD834; continue 'dispatch;
	// 825AD768: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD76C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD770: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 825AD774: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD778: 4E800421  bctrl
	ctx.lr = 0x825AD77C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD77C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AD780: 41820018  beq 0x825ad798
	if ctx.cr[0].eq {
	pc = 0x825AD798; continue 'dispatch;
	}
	// 825AD784: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AD788: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 825AD78C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825AD790: 4BFFF8A1  bl 0x825ad030
	ctx.lr = 0x825AD794;
	sub_825AD030(ctx, base);
	// 825AD794: 480000A0  b 0x825ad834
	pc = 0x825AD834; continue 'dispatch;
	// 825AD798: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD79C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD7A0: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 825AD7A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD7A8: 4E800421  bctrl
	ctx.lr = 0x825AD7AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD7AC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AD7B0: 4082FFA4  bne 0x825ad754
	if !ctx.cr[0].eq {
	pc = 0x825AD754; continue 'dispatch;
	}
	// 825AD7B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD7B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD7BC: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AD7C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD7C4: 4E800421  bctrl
	ctx.lr = 0x825AD7C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD7C8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AD7CC: 4082FFB8  bne 0x825ad784
	if !ctx.cr[0].eq {
	pc = 0x825AD784; continue 'dispatch;
	}
	// 825AD7D0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825AD7D4: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825AD7D8: D01E0050  stfs f0, 0x50(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825AD7DC: 48000088  b 0x825ad864
	pc = 0x825AD864; continue 'dispatch;
	// 825AD7E0: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 825AD7E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD7E8: 4E800421  bctrl
	ctx.lr = 0x825AD7EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD7EC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AD7F0: 41820010  beq 0x825ad800
	if ctx.cr[0].eq {
	pc = 0x825AD800; continue 'dispatch;
	}
	// 825AD7F4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD7F8: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 825AD7FC: 48000028  b 0x825ad824
	pc = 0x825AD824; continue 'dispatch;
	// 825AD800: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD804: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD808: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AD80C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD810: 4E800421  bctrl
	ctx.lr = 0x825AD814;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD814: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AD818: 4182004C  beq 0x825ad864
	if ctx.cr[0].eq {
	pc = 0x825AD864; continue 'dispatch;
	}
	// 825AD81C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD820: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 825AD824: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AD828: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825AD82C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD830: 4E800421  bctrl
	ctx.lr = 0x825AD834;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD834: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD838: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825AD83C: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD840: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AD844: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 825AD848: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD84C: 4E800421  bctrl
	ctx.lr = 0x825AD850;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD850: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AD854: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AD858: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD85C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD860: 4E800421  bctrl
	ctx.lr = 0x825AD864;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD864: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825AD868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AD86C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AD870: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 825AD874: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 825AD878: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AD87C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AD880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AD888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825AD888 size=444
    let mut pc: u32 = 0x825AD888;
    'dispatch: loop {
        match pc {
            0x825AD888 => {
    //   block [0x825AD888..0x825ADA44)
	// 825AD888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AD88C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AD890: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AD894: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AD898: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 825AD89C: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 825AD8A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AD8A4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825AD8A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825AD8AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD8B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD8B4: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 825AD8B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD8BC: 4E800421  bctrl
	ctx.lr = 0x825AD8C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD8C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD8C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD8C8: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 825AD8CC: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 825AD8D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD8D4: 4E800421  bctrl
	ctx.lr = 0x825AD8D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD8D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD8DC: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 825AD8E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD8E4: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 825AD8E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD8EC: 4E800421  bctrl
	ctx.lr = 0x825AD8F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD8F0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AD8F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD8F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD8FC: 418200A4  beq 0x825ad9a0
	if ctx.cr[0].eq {
	pc = 0x825AD9A0; continue 'dispatch;
	}
	// 825AD900: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 825AD904: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD908: 4E800421  bctrl
	ctx.lr = 0x825AD90C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD90C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AD910: 41820018  beq 0x825ad928
	if ctx.cr[0].eq {
	pc = 0x825AD928; continue 'dispatch;
	}
	// 825AD914: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AD918: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 825AD91C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825AD920: 4BFFFB41  bl 0x825ad460
	ctx.lr = 0x825AD924;
	sub_825AD460(ctx, base);
	// 825AD924: 480000D0  b 0x825ad9f4
	pc = 0x825AD9F4; continue 'dispatch;
	// 825AD928: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD92C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD930: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 825AD934: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD938: 4E800421  bctrl
	ctx.lr = 0x825AD93C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD93C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AD940: 41820018  beq 0x825ad958
	if ctx.cr[0].eq {
	pc = 0x825AD958; continue 'dispatch;
	}
	// 825AD944: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AD948: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 825AD94C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825AD950: 4BFFF871  bl 0x825ad1c0
	ctx.lr = 0x825AD954;
	sub_825AD1C0(ctx, base);
	// 825AD954: 480000A0  b 0x825ad9f4
	pc = 0x825AD9F4; continue 'dispatch;
	// 825AD958: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD95C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD960: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 825AD964: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD968: 4E800421  bctrl
	ctx.lr = 0x825AD96C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD96C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AD970: 4082FFA4  bne 0x825ad914
	if !ctx.cr[0].eq {
	pc = 0x825AD914; continue 'dispatch;
	}
	// 825AD974: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD978: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD97C: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AD980: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD984: 4E800421  bctrl
	ctx.lr = 0x825AD988;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD988: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AD98C: 4082FFB8  bne 0x825ad944
	if !ctx.cr[0].eq {
	pc = 0x825AD944; continue 'dispatch;
	}
	// 825AD990: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825AD994: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825AD998: D01E0050  stfs f0, 0x50(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825AD99C: 48000088  b 0x825ada24
	pc = 0x825ADA24; continue 'dispatch;
	// 825AD9A0: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 825AD9A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD9A8: 4E800421  bctrl
	ctx.lr = 0x825AD9AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD9AC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AD9B0: 41820010  beq 0x825ad9c0
	if ctx.cr[0].eq {
	pc = 0x825AD9C0; continue 'dispatch;
	}
	// 825AD9B4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD9B8: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 825AD9BC: 48000028  b 0x825ad9e4
	pc = 0x825AD9E4; continue 'dispatch;
	// 825AD9C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD9C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AD9C8: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AD9CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD9D0: 4E800421  bctrl
	ctx.lr = 0x825AD9D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD9D4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AD9D8: 4182004C  beq 0x825ada24
	if ctx.cr[0].eq {
	pc = 0x825ADA24; continue 'dispatch;
	}
	// 825AD9DC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD9E0: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 825AD9E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AD9E8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825AD9EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AD9F0: 4E800421  bctrl
	ctx.lr = 0x825AD9F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AD9F4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AD9F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825AD9FC: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ADA00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825ADA04: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 825ADA08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825ADA0C: 4E800421  bctrl
	ctx.lr = 0x825ADA10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825ADA10: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825ADA14: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 825ADA18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825ADA1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825ADA20: 4E800421  bctrl
	ctx.lr = 0x825ADA24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825ADA24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ADA28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ADA2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ADA30: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 825ADA34: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 825ADA38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825ADA3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825ADA40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ADA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825ADA48 size=112
    let mut pc: u32 = 0x825ADA48;
    'dispatch: loop {
        match pc {
            0x825ADA48 => {
    //   block [0x825ADA48..0x825ADAB8)
	// 825ADA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ADA4C: 48BFA721  bl 0x831a816c
	ctx.lr = 0x825ADA50;
	sub_831A8130(ctx, base);
	// 825ADA50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ADA54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825ADA58: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 825ADA5C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825ADA60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825ADA64: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 825ADA68: D05F000C  stfs f2, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825ADA6C: 90BF0000  stw r5, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 825ADA70: D07F0010  stfs f3, 0x10(r31)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825ADA74: D03F0008  stfs f1, 8(r31)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825ADA78: 4BFDE499  bl 0x8258bf10
	ctx.lr = 0x825ADA7C;
	sub_8258BF10(ctx, base);
	// 825ADA7C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825ADA80: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 825ADA84: 4884617D  bl 0x82df3c00
	ctx.lr = 0x825ADA88;
	sub_82DF3C00(ctx, base);
	// 825ADA88: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ADA8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ADA90: 419A000C  beq cr6, 0x825ada9c
	if ctx.cr[6].eq {
	pc = 0x825ADA9C; continue 'dispatch;
	}
	// 825ADA94: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ADA98: 48000008  b 0x825adaa0
	pc = 0x825ADAA0; continue 'dispatch;
	// 825ADA9C: C01F0008  lfs f0, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ADAA0: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825ADAA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825ADAA8: 4BD1B211  bl 0x822c8cb8
	ctx.lr = 0x825ADAAC;
	sub_822C8CB8(ctx, base);
	// 825ADAAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825ADAB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825ADAB4: 48BFA708  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ADAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825ADAB8 size=104
    let mut pc: u32 = 0x825ADAB8;
    'dispatch: loop {
        match pc {
            0x825ADAB8 => {
    //   block [0x825ADAB8..0x825ADB20)
	// 825ADAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ADABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ADAC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825ADAC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ADAC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825ADACC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ADAD0: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 825ADAD4: D05F000C  stfs f2, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825ADAD8: 90BF0000  stw r5, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 825ADADC: D07F0010  stfs f3, 0x10(r31)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825ADAE0: D03F0008  stfs f1, 8(r31)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825ADAE4: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 825ADAE8: 48846119  bl 0x82df3c00
	ctx.lr = 0x825ADAEC;
	sub_82DF3C00(ctx, base);
	// 825ADAEC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ADAF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ADAF4: 419A000C  beq cr6, 0x825adb00
	if ctx.cr[6].eq {
	pc = 0x825ADB00; continue 'dispatch;
	}
	// 825ADAF8: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ADAFC: 48000008  b 0x825adb04
	pc = 0x825ADB04; continue 'dispatch;
	// 825ADB00: C01F0008  lfs f0, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ADB04: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825ADB08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825ADB0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825ADB10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ADB14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ADB18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825ADB1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ADB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ADB20 size=112
    let mut pc: u32 = 0x825ADB20;
    'dispatch: loop {
        match pc {
            0x825ADB20 => {
    //   block [0x825ADB20..0x825ADB90)
	// 825ADB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ADB24: 48BFA649  bl 0x831a816c
	ctx.lr = 0x825ADB28;
	sub_831A8130(ctx, base);
	// 825ADB28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ADB2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825ADB30: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 825ADB34: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825ADB38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825ADB3C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 825ADB40: 90BF0000  stw r5, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 825ADB44: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 825ADB48: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 825ADB4C: 90DF0008  stw r6, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 825ADB50: 4BFDE3C1  bl 0x8258bf10
	ctx.lr = 0x825ADB54;
	sub_8258BF10(ctx, base);
	// 825ADB54: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825ADB58: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 825ADB5C: 488460A5  bl 0x82df3c00
	ctx.lr = 0x825ADB60;
	sub_82DF3C00(ctx, base);
	// 825ADB60: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ADB64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ADB68: 419A000C  beq cr6, 0x825adb74
	if ctx.cr[6].eq {
	pc = 0x825ADB74; continue 'dispatch;
	}
	// 825ADB6C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ADB70: 48000008  b 0x825adb78
	pc = 0x825ADB78; continue 'dispatch;
	// 825ADB74: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ADB78: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825ADB7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825ADB80: 4BD1B139  bl 0x822c8cb8
	ctx.lr = 0x825ADB84;
	sub_822C8CB8(ctx, base);
	// 825ADB84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825ADB88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825ADB8C: 48BFA630  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ADB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ADB90 size=104
    let mut pc: u32 = 0x825ADB90;
    'dispatch: loop {
        match pc {
            0x825ADB90 => {
    //   block [0x825ADB90..0x825ADBF8)
	// 825ADB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ADB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ADB98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825ADB9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ADBA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825ADBA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ADBA8: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 825ADBAC: 90BF0000  stw r5, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 825ADBB0: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 825ADBB4: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 825ADBB8: 90DF0008  stw r6, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 825ADBBC: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 825ADBC0: 48846041  bl 0x82df3c00
	ctx.lr = 0x825ADBC4;
	sub_82DF3C00(ctx, base);
	// 825ADBC4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ADBC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825ADBCC: 419A000C  beq cr6, 0x825adbd8
	if ctx.cr[6].eq {
	pc = 0x825ADBD8; continue 'dispatch;
	}
	// 825ADBD0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ADBD4: 48000008  b 0x825adbdc
	pc = 0x825ADBDC; continue 'dispatch;
	// 825ADBD8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ADBDC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825ADBE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825ADBE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825ADBE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ADBEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ADBF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825ADBF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ADBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825ADBF8 size=120
    let mut pc: u32 = 0x825ADBF8;
    'dispatch: loop {
        match pc {
            0x825ADBF8 => {
    //   block [0x825ADBF8..0x825ADC70)
	// 825ADBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ADBFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ADC00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825ADC04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825ADC08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ADC0C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825ADC10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825ADC14: 389E0018  addi r4, r30, 0x18
	ctx.r[4].s64 = ctx.r[30].s64 + 24;
	// 825ADC18: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 825ADC1C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ADC20: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825ADC24: C01E0004  lfs f0, 4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ADC28: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825ADC2C: C01E0008  lfs f0, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ADC30: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825ADC34: C01E000C  lfs f0, 0xc(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ADC38: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825ADC3C: C01E0010  lfs f0, 0x10(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ADC40: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825ADC44: 4BFDE2CD  bl 0x8258bf10
	ctx.lr = 0x825ADC48;
	sub_8258BF10(ctx, base);
	// 825ADC48: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 825ADC4C: 389E0038  addi r4, r30, 0x38
	ctx.r[4].s64 = ctx.r[30].s64 + 56;
	// 825ADC50: 48845FB1  bl 0x82df3c00
	ctx.lr = 0x825ADC54;
	sub_82DF3C00(ctx, base);
	// 825ADC54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825ADC58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825ADC5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ADC60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ADC64: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825ADC68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825ADC6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ADC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ADC70 size=120
    let mut pc: u32 = 0x825ADC70;
    'dispatch: loop {
        match pc {
            0x825ADC70 => {
    //   block [0x825ADC70..0x825ADCE8)
	// 825ADC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ADC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ADC78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825ADC7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825ADC80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ADC84: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825ADC88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825ADC8C: 389E0018  addi r4, r30, 0x18
	ctx.r[4].s64 = ctx.r[30].s64 + 24;
	// 825ADC90: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 825ADC94: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ADC98: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825ADC9C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825ADCA0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825ADCA4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825ADCA8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825ADCAC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 825ADCB0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 825ADCB4: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 825ADCB8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 825ADCBC: 4BFDE255  bl 0x8258bf10
	ctx.lr = 0x825ADCC0;
	sub_8258BF10(ctx, base);
	// 825ADCC0: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 825ADCC4: 389E0038  addi r4, r30, 0x38
	ctx.r[4].s64 = ctx.r[30].s64 + 56;
	// 825ADCC8: 48845F39  bl 0x82df3c00
	ctx.lr = 0x825ADCCC;
	sub_82DF3C00(ctx, base);
	// 825ADCCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825ADCD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825ADCD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ADCD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ADCDC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825ADCE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825ADCE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ADCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ADCE8 size=76
    let mut pc: u32 = 0x825ADCE8;
    'dispatch: loop {
        match pc {
            0x825ADCE8 => {
    //   block [0x825ADCE8..0x825ADD34)
	// 825ADCE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ADCEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ADCF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825ADCF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825ADCF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ADCFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825ADD00: 3BFE0010  addi r31, r30, 0x10
	ctx.r[31].s64 = ctx.r[30].s64 + 16;
	// 825ADD04: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 825ADD08: 48845721  bl 0x82df3428
	ctx.lr = 0x825ADD0C;
	sub_82DF3428(ctx, base);
	// 825ADD0C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 825ADD10: 4BD1AFA9  bl 0x822c8cb8
	ctx.lr = 0x825ADD14;
	sub_822C8CB8(ctx, base);
	// 825ADD14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825ADD18: 4BFF86F9  bl 0x825a6410
	ctx.lr = 0x825ADD1C;
	sub_825A6410(ctx, base);
	// 825ADD1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825ADD20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ADD24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ADD28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825ADD2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825ADD30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ADD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825ADD38 size=124
    let mut pc: u32 = 0x825ADD38;
    'dispatch: loop {
        match pc {
            0x825ADD38 => {
    //   block [0x825ADD38..0x825ADDB4)
	// 825ADD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ADD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ADD40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825ADD44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825ADD48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ADD4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825ADD50: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825ADD54: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 825ADD58: 396BAF84  addi r11, r11, -0x507c
	ctx.r[11].s64 = ctx.r[11].s64 + -20604;
	// 825ADD5C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825ADD60: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825ADD64: 4884538D  bl 0x82df30f0
	ctx.lr = 0x825ADD68;
	sub_82DF30F0(ctx, base);
	// 825ADD68: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825ADD6C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 825ADD70: 396BB5A4  addi r11, r11, -0x4a5c
	ctx.r[11].s64 = ctx.r[11].s64 + -19036;
	// 825ADD74: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825ADD78: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825ADD7C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ADD80: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ADD84: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825ADD88: 4BFFFE71  bl 0x825adbf8
	ctx.lr = 0x825ADD8C;
	sub_825ADBF8(ctx, base);
	// 825ADD8C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825ADD90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825ADD94: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ADD98: D01F0050  stfs f0, 0x50(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825ADD9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825ADDA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ADDA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ADDA8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825ADDAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825ADDB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ADDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ADDB8 size=224
    let mut pc: u32 = 0x825ADDB8;
    'dispatch: loop {
        match pc {
            0x825ADDB8 => {
    //   block [0x825ADDB8..0x825ADE98)
	// 825ADDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ADDBC: 48BFA3B1  bl 0x831a816c
	ctx.lr = 0x825ADDC0;
	sub_831A8130(ctx, base);
	// 825ADDC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ADDC4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825ADDC8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825ADDCC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825ADDD0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ADDD4: 388BB5F8  addi r4, r11, -0x4a08
	ctx.r[4].s64 = ctx.r[11].s64 + -18952;
	// 825ADDD8: 38A00097  li r5, 0x97
	ctx.r[5].s64 = 151;
	// 825ADDDC: 38600058  li r3, 0x58
	ctx.r[3].s64 = 88;
	// 825ADDE0: 4BD125F9  bl 0x822c03d8
	ctx.lr = 0x825ADDE4;
	sub_822C03D8(ctx, base);
	// 825ADDE4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825ADDE8: 41820014  beq 0x825addfc
	if ctx.cr[0].eq {
	pc = 0x825ADDFC; continue 'dispatch;
	}
	// 825ADDEC: 389D0010  addi r4, r29, 0x10
	ctx.r[4].s64 = ctx.r[29].s64 + 16;
	// 825ADDF0: 4BFFFF49  bl 0x825add38
	ctx.lr = 0x825ADDF4;
	sub_825ADD38(ctx, base);
	// 825ADDF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825ADDF8: 48000008  b 0x825ade00
	pc = 0x825ADE00; continue 'dispatch;
	// 825ADDFC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825ADE00: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 825ADE04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825ADE08: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825ADE0C: 4BFF3125  bl 0x825a0f30
	ctx.lr = 0x825ADE10;
	sub_825A0F30(ctx, base);
	// 825ADE10: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825ADE14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825ADE18: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825ADE1C: 4BD121E5  bl 0x822c0000
	ctx.lr = 0x825ADE20;
	sub_822C0000(ctx, base);
	// 825ADE20: 83E10058  lwz r31, 0x58(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825ADE24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ADE28: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 825ADE2C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 825ADE30: 4BD1AE89  bl 0x822c8cb8
	ctx.lr = 0x825ADE34;
	sub_822C8CB8(ctx, base);
	// 825ADE34: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825ADE38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825ADE3C: 4BFF864D  bl 0x825a6488
	ctx.lr = 0x825ADE40;
	sub_825A6488(ctx, base);
	// 825ADE40: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825ADE44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825ADE48: 48227C21  bl 0x827d5a68
	ctx.lr = 0x825ADE4C;
	sub_827D5A68(ctx, base);
	// 825ADE4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825ADE50: 488455D9  bl 0x82df3428
	ctx.lr = 0x825ADE54;
	sub_82DF3428(ctx, base);
	// 825ADE54: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825ADE58: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825ADE5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825ADE60: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 825ADE64: 419A0028  beq cr6, 0x825ade8c
	if ctx.cr[6].eq {
	pc = 0x825ADE8C; continue 'dispatch;
	}
	// 825ADE68: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 825ADE6C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825ADE70: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825ADE74: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825ADE78: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825ADE7C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825ADE80: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825ADE84: 4082FFE8  bne 0x825ade6c
	if !ctx.cr[0].eq {
	pc = 0x825ADE6C; continue 'dispatch;
	}
	// 825ADE88: 4BD12A09  bl 0x822c0890
	ctx.lr = 0x825ADE8C;
	sub_822C0890(ctx, base);
	// 825ADE8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825ADE90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ADE94: 48BFA328  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ADE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825ADE98 size=124
    let mut pc: u32 = 0x825ADE98;
    'dispatch: loop {
        match pc {
            0x825ADE98 => {
    //   block [0x825ADE98..0x825ADF14)
	// 825ADE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ADE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ADEA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825ADEA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825ADEA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ADEAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825ADEB0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825ADEB4: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 825ADEB8: 396BAF84  addi r11, r11, -0x507c
	ctx.r[11].s64 = ctx.r[11].s64 + -20604;
	// 825ADEBC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825ADEC0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825ADEC4: 4884522D  bl 0x82df30f0
	ctx.lr = 0x825ADEC8;
	sub_82DF30F0(ctx, base);
	// 825ADEC8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825ADECC: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 825ADED0: 396BB63C  addi r11, r11, -0x49c4
	ctx.r[11].s64 = ctx.r[11].s64 + -18884;
	// 825ADED4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825ADED8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825ADEDC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ADEE0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825ADEE4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825ADEE8: 4BFFFD89  bl 0x825adc70
	ctx.lr = 0x825ADEEC;
	sub_825ADC70(ctx, base);
	// 825ADEEC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825ADEF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825ADEF4: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825ADEF8: D01F0050  stfs f0, 0x50(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825ADEFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825ADF00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ADF04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ADF08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825ADF0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825ADF10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ADF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ADF18 size=224
    let mut pc: u32 = 0x825ADF18;
    'dispatch: loop {
        match pc {
            0x825ADF18 => {
    //   block [0x825ADF18..0x825ADFF8)
	// 825ADF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ADF1C: 48BFA251  bl 0x831a816c
	ctx.lr = 0x825ADF20;
	sub_831A8130(ctx, base);
	// 825ADF20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ADF24: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825ADF28: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825ADF2C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825ADF30: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ADF34: 388BB5F8  addi r4, r11, -0x4a08
	ctx.r[4].s64 = ctx.r[11].s64 + -18952;
	// 825ADF38: 38A00097  li r5, 0x97
	ctx.r[5].s64 = 151;
	// 825ADF3C: 38600058  li r3, 0x58
	ctx.r[3].s64 = 88;
	// 825ADF40: 4BD12499  bl 0x822c03d8
	ctx.lr = 0x825ADF44;
	sub_822C03D8(ctx, base);
	// 825ADF44: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825ADF48: 41820014  beq 0x825adf5c
	if ctx.cr[0].eq {
	pc = 0x825ADF5C; continue 'dispatch;
	}
	// 825ADF4C: 389D0010  addi r4, r29, 0x10
	ctx.r[4].s64 = ctx.r[29].s64 + 16;
	// 825ADF50: 4BFFFF49  bl 0x825ade98
	ctx.lr = 0x825ADF54;
	sub_825ADE98(ctx, base);
	// 825ADF54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825ADF58: 48000008  b 0x825adf60
	pc = 0x825ADF60; continue 'dispatch;
	// 825ADF5C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825ADF60: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 825ADF64: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825ADF68: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825ADF6C: 4BFF321D  bl 0x825a1188
	ctx.lr = 0x825ADF70;
	sub_825A1188(ctx, base);
	// 825ADF70: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825ADF74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825ADF78: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825ADF7C: 4BD12085  bl 0x822c0000
	ctx.lr = 0x825ADF80;
	sub_822C0000(ctx, base);
	// 825ADF80: 83E10058  lwz r31, 0x58(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825ADF84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ADF88: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 825ADF8C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 825ADF90: 4BD1AD29  bl 0x822c8cb8
	ctx.lr = 0x825ADF94;
	sub_822C8CB8(ctx, base);
	// 825ADF94: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825ADF98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825ADF9C: 4BFF84ED  bl 0x825a6488
	ctx.lr = 0x825ADFA0;
	sub_825A6488(ctx, base);
	// 825ADFA0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825ADFA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825ADFA8: 48227AC1  bl 0x827d5a68
	ctx.lr = 0x825ADFAC;
	sub_827D5A68(ctx, base);
	// 825ADFAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825ADFB0: 48845479  bl 0x82df3428
	ctx.lr = 0x825ADFB4;
	sub_82DF3428(ctx, base);
	// 825ADFB4: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825ADFB8: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825ADFBC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825ADFC0: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 825ADFC4: 419A0028  beq cr6, 0x825adfec
	if ctx.cr[6].eq {
	pc = 0x825ADFEC; continue 'dispatch;
	}
	// 825ADFC8: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 825ADFCC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825ADFD0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825ADFD4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825ADFD8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825ADFDC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825ADFE0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825ADFE4: 4082FFE8  bne 0x825adfcc
	if !ctx.cr[0].eq {
	pc = 0x825ADFCC; continue 'dispatch;
	}
	// 825ADFE8: 4BD128A9  bl 0x822c0890
	ctx.lr = 0x825ADFEC;
	sub_822C0890(ctx, base);
	// 825ADFEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825ADFF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ADFF4: 48BFA1C8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ADFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825ADFF8 size=124
    let mut pc: u32 = 0x825ADFF8;
    'dispatch: loop {
        match pc {
            0x825ADFF8 => {
    //   block [0x825ADFF8..0x825AE074)
	// 825ADFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ADFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AE000: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AE004: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AE008: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AE00C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AE010: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AE014: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 825AE018: 396BAF84  addi r11, r11, -0x507c
	ctx.r[11].s64 = ctx.r[11].s64 + -20604;
	// 825AE01C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825AE020: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AE024: 488450CD  bl 0x82df30f0
	ctx.lr = 0x825AE028;
	sub_82DF30F0(ctx, base);
	// 825AE028: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AE02C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 825AE030: 396BB694  addi r11, r11, -0x496c
	ctx.r[11].s64 = ctx.r[11].s64 + -18796;
	// 825AE034: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825AE038: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AE03C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AE040: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AE044: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825AE048: 4BFFFC29  bl 0x825adc70
	ctx.lr = 0x825AE04C;
	sub_825ADC70(ctx, base);
	// 825AE04C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825AE050: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AE054: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825AE058: D01F0050  stfs f0, 0x50(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825AE05C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AE060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AE064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AE068: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AE06C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AE070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AE078 size=224
    let mut pc: u32 = 0x825AE078;
    'dispatch: loop {
        match pc {
            0x825AE078 => {
    //   block [0x825AE078..0x825AE158)
	// 825AE078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AE07C: 48BFA0F1  bl 0x831a816c
	ctx.lr = 0x825AE080;
	sub_831A8130(ctx, base);
	// 825AE080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AE084: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AE088: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825AE08C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825AE090: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825AE094: 388BB5F8  addi r4, r11, -0x4a08
	ctx.r[4].s64 = ctx.r[11].s64 + -18952;
	// 825AE098: 38A00097  li r5, 0x97
	ctx.r[5].s64 = 151;
	// 825AE09C: 38600058  li r3, 0x58
	ctx.r[3].s64 = 88;
	// 825AE0A0: 4BD12339  bl 0x822c03d8
	ctx.lr = 0x825AE0A4;
	sub_822C03D8(ctx, base);
	// 825AE0A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825AE0A8: 41820014  beq 0x825ae0bc
	if ctx.cr[0].eq {
	pc = 0x825AE0BC; continue 'dispatch;
	}
	// 825AE0AC: 389D0010  addi r4, r29, 0x10
	ctx.r[4].s64 = ctx.r[29].s64 + 16;
	// 825AE0B0: 4BFFFF49  bl 0x825adff8
	ctx.lr = 0x825AE0B4;
	sub_825ADFF8(ctx, base);
	// 825AE0B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AE0B8: 48000008  b 0x825ae0c0
	pc = 0x825AE0C0; continue 'dispatch;
	// 825AE0BC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825AE0C0: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 825AE0C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AE0C8: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825AE0CC: 4BFF2FF5  bl 0x825a10c0
	ctx.lr = 0x825AE0D0;
	sub_825A10C0(ctx, base);
	// 825AE0D0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825AE0D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AE0D8: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825AE0DC: 4BD11F25  bl 0x822c0000
	ctx.lr = 0x825AE0E0;
	sub_822C0000(ctx, base);
	// 825AE0E0: 83E10058  lwz r31, 0x58(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825AE0E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AE0E8: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 825AE0EC: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 825AE0F0: 4BD1ABC9  bl 0x822c8cb8
	ctx.lr = 0x825AE0F4;
	sub_822C8CB8(ctx, base);
	// 825AE0F4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825AE0F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AE0FC: 4BFF838D  bl 0x825a6488
	ctx.lr = 0x825AE100;
	sub_825A6488(ctx, base);
	// 825AE100: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AE104: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AE108: 48227961  bl 0x827d5a68
	ctx.lr = 0x825AE10C;
	sub_827D5A68(ctx, base);
	// 825AE10C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AE110: 48845319  bl 0x82df3428
	ctx.lr = 0x825AE114;
	sub_82DF3428(ctx, base);
	// 825AE114: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825AE118: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825AE11C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AE120: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 825AE124: 419A0028  beq cr6, 0x825ae14c
	if ctx.cr[6].eq {
	pc = 0x825AE14C; continue 'dispatch;
	}
	// 825AE128: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 825AE12C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825AE130: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AE134: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825AE138: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825AE13C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825AE140: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AE144: 4082FFE8  bne 0x825ae12c
	if !ctx.cr[0].eq {
	pc = 0x825AE12C; continue 'dispatch;
	}
	// 825AE148: 4BD12749  bl 0x822c0890
	ctx.lr = 0x825AE14C;
	sub_822C0890(ctx, base);
	// 825AE14C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AE150: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825AE154: 48BFA068  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825AE158 size=124
    let mut pc: u32 = 0x825AE158;
    'dispatch: loop {
        match pc {
            0x825AE158 => {
    //   block [0x825AE158..0x825AE1D4)
	// 825AE158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AE15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AE160: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AE164: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AE168: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AE16C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AE170: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AE174: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 825AE178: 396BAF84  addi r11, r11, -0x507c
	ctx.r[11].s64 = ctx.r[11].s64 + -20604;
	// 825AE17C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825AE180: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AE184: 48844F6D  bl 0x82df30f0
	ctx.lr = 0x825AE188;
	sub_82DF30F0(ctx, base);
	// 825AE188: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AE18C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 825AE190: 396BB6EC  addi r11, r11, -0x4914
	ctx.r[11].s64 = ctx.r[11].s64 + -18708;
	// 825AE194: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825AE198: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AE19C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AE1A0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AE1A4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825AE1A8: 4BFFFAC9  bl 0x825adc70
	ctx.lr = 0x825AE1AC;
	sub_825ADC70(ctx, base);
	// 825AE1AC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825AE1B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AE1B4: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825AE1B8: D01F0050  stfs f0, 0x50(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825AE1BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AE1C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AE1C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AE1C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AE1CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AE1D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AE1D8 size=224
    let mut pc: u32 = 0x825AE1D8;
    'dispatch: loop {
        match pc {
            0x825AE1D8 => {
    //   block [0x825AE1D8..0x825AE2B8)
	// 825AE1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AE1DC: 48BF9F91  bl 0x831a816c
	ctx.lr = 0x825AE1E0;
	sub_831A8130(ctx, base);
	// 825AE1E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AE1E4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AE1E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825AE1EC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825AE1F0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825AE1F4: 388BB5F8  addi r4, r11, -0x4a08
	ctx.r[4].s64 = ctx.r[11].s64 + -18952;
	// 825AE1F8: 38A00097  li r5, 0x97
	ctx.r[5].s64 = 151;
	// 825AE1FC: 38600058  li r3, 0x58
	ctx.r[3].s64 = 88;
	// 825AE200: 4BD121D9  bl 0x822c03d8
	ctx.lr = 0x825AE204;
	sub_822C03D8(ctx, base);
	// 825AE204: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825AE208: 41820014  beq 0x825ae21c
	if ctx.cr[0].eq {
	pc = 0x825AE21C; continue 'dispatch;
	}
	// 825AE20C: 389D0010  addi r4, r29, 0x10
	ctx.r[4].s64 = ctx.r[29].s64 + 16;
	// 825AE210: 4BFFFF49  bl 0x825ae158
	ctx.lr = 0x825AE214;
	sub_825AE158(ctx, base);
	// 825AE214: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AE218: 48000008  b 0x825ae220
	pc = 0x825AE220; continue 'dispatch;
	// 825AE21C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825AE220: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 825AE224: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AE228: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825AE22C: 4BFF2DCD  bl 0x825a0ff8
	ctx.lr = 0x825AE230;
	sub_825A0FF8(ctx, base);
	// 825AE230: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825AE234: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AE238: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825AE23C: 4BD11DC5  bl 0x822c0000
	ctx.lr = 0x825AE240;
	sub_822C0000(ctx, base);
	// 825AE240: 83E10058  lwz r31, 0x58(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825AE244: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AE248: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 825AE24C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 825AE250: 4BD1AA69  bl 0x822c8cb8
	ctx.lr = 0x825AE254;
	sub_822C8CB8(ctx, base);
	// 825AE254: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825AE258: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AE25C: 4BFF822D  bl 0x825a6488
	ctx.lr = 0x825AE260;
	sub_825A6488(ctx, base);
	// 825AE260: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AE264: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AE268: 48227801  bl 0x827d5a68
	ctx.lr = 0x825AE26C;
	sub_827D5A68(ctx, base);
	// 825AE26C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AE270: 488451B9  bl 0x82df3428
	ctx.lr = 0x825AE274;
	sub_82DF3428(ctx, base);
	// 825AE274: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825AE278: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825AE27C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AE280: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 825AE284: 419A0028  beq cr6, 0x825ae2ac
	if ctx.cr[6].eq {
	pc = 0x825AE2AC; continue 'dispatch;
	}
	// 825AE288: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 825AE28C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825AE290: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AE294: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825AE298: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825AE29C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825AE2A0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AE2A4: 4082FFE8  bne 0x825ae28c
	if !ctx.cr[0].eq {
	pc = 0x825AE28C; continue 'dispatch;
	}
	// 825AE2A8: 4BD125E9  bl 0x822c0890
	ctx.lr = 0x825AE2AC;
	sub_822C0890(ctx, base);
	// 825AE2AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AE2B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825AE2B4: 48BF9F08  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AE2B8 size=76
    let mut pc: u32 = 0x825AE2B8;
    'dispatch: loop {
        match pc {
            0x825AE2B8 => {
    //   block [0x825AE2B8..0x825AE304)
	// 825AE2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AE2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AE2C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AE2C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AE2C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AE2CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AE2D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825AE2D4: 4BFFFA15  bl 0x825adce8
	ctx.lr = 0x825AE2D8;
	sub_825ADCE8(ctx, base);
	// 825AE2D8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AE2DC: 4182000C  beq 0x825ae2e8
	if ctx.cr[0].eq {
	pc = 0x825AE2E8; continue 'dispatch;
	}
	// 825AE2E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AE2E4: 4BD11F85  bl 0x822c0268
	ctx.lr = 0x825AE2E8;
	sub_822C0268(ctx, base);
	// 825AE2E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AE2EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AE2F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AE2F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AE2F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AE2FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AE300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AE308 size=20
    let mut pc: u32 = 0x825AE308;
    'dispatch: loop {
        match pc {
            0x825AE308 => {
    //   block [0x825AE308..0x825AE31C)
	// 825AE308: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 825AE30C: 386A0028  addi r3, r10, 0x28
	ctx.r[3].s64 = ctx.r[10].s64 + 40;
	// 825AE310: 816A0028  lwz r11, 0x28(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 825AE314: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AE318: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE31C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AE31C size=16
    let mut pc: u32 = 0x825AE31C;
    'dispatch: loop {
        match pc {
            0x825AE31C => {
    //   block [0x825AE31C..0x825AE32C)
	// 825AE31C: 3D60822C  lis r11, -0x7dd4
	ctx.r[11].s64 = -2111045632;
	// 825AE320: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 825AE324: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AE328: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE32C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825AE32C size=20
    let mut pc: u32 = 0x825AE32C;
    'dispatch: loop {
        match pc {
            0x825AE32C => {
    //   block [0x825AE32C..0x825AE340)
	// 825AE32C: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AE330: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AE334: 419A000C  beq cr6, 0x825ae340
	if ctx.cr[6].eq {
		sub_825AE340(ctx, base);
		return;
	}
	// 825AE338: C02B0000  lfs f1, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825AE33C: 48000008  b 0x825ae344
	sub_825AE340(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825AE340 size=8
    let mut pc: u32 = 0x825AE340;
    'dispatch: loop {
        match pc {
            0x825AE340 => {
    //   block [0x825AE340..0x825AE348)
	// 825AE340: C02A0008  lfs f1, 8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825AE344: 4BD5C50C  b 0x8230a850
	sub_8230A850(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AE348 size=4
    let mut pc: u32 = 0x825AE348;
    'dispatch: loop {
        match pc {
            0x825AE348 => {
    //   block [0x825AE348..0x825AE34C)
	// 825AE348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AE350 size=20
    let mut pc: u32 = 0x825AE350;
    'dispatch: loop {
        match pc {
            0x825AE350 => {
    //   block [0x825AE350..0x825AE364)
	// 825AE350: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 825AE354: 386A0028  addi r3, r10, 0x28
	ctx.r[3].s64 = ctx.r[10].s64 + 40;
	// 825AE358: 816A0028  lwz r11, 0x28(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 825AE35C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AE360: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE364(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AE364 size=16
    let mut pc: u32 = 0x825AE364;
    'dispatch: loop {
        match pc {
            0x825AE364 => {
    //   block [0x825AE364..0x825AE374)
	// 825AE364: 3D60822C  lis r11, -0x7dd4
	ctx.r[11].s64 = -2111045632;
	// 825AE368: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 825AE36C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AE370: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE374(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AE374 size=20
    let mut pc: u32 = 0x825AE374;
    'dispatch: loop {
        match pc {
            0x825AE374 => {
    //   block [0x825AE374..0x825AE388)
	// 825AE374: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AE378: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AE37C: 419A000C  beq cr6, 0x825ae388
	if ctx.cr[6].eq {
		sub_825AE388(ctx, base);
		return;
	}
	// 825AE380: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AE384: 48000008  b 0x825ae38c
	sub_825AE388(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AE388 size=8
    let mut pc: u32 = 0x825AE388;
    'dispatch: loop {
        match pc {
            0x825AE388 => {
    //   block [0x825AE388..0x825AE390)
	// 825AE388: 808A0008  lwz r4, 8(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AE38C: 4800353C  b 0x825b18c8
	sub_825B18C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AE390 size=4
    let mut pc: u32 = 0x825AE390;
    'dispatch: loop {
        match pc {
            0x825AE390 => {
    //   block [0x825AE390..0x825AE394)
	// 825AE390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825AE398 size=88
    let mut pc: u32 = 0x825AE398;
    'dispatch: loop {
        match pc {
            0x825AE398 => {
    //   block [0x825AE398..0x825AE3F0)
	// 825AE398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AE39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AE3A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AE3A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AE3A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AE3AC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825AE3B0: 4BFF7841  bl 0x825a5bf0
	ctx.lr = 0x825AE3B4;
	sub_825A5BF0(ctx, base);
	// 825AE3B4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AE3B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AE3BC: 419A0008  beq cr6, 0x825ae3c4
	if ctx.cr[6].eq {
	pc = 0x825AE3C4; continue 'dispatch;
	}
	// 825AE3C0: D02B0000  stfs f1, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825AE3C4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AE3C8: D03F0008  stfs f1, 8(r31)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825AE3CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AE3D0: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AE3D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AE3D8: 4E800421  bctrl
	ctx.lr = 0x825AE3DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AE3DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AE3E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AE3E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AE3E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AE3EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825AE3F0 size=84
    let mut pc: u32 = 0x825AE3F0;
    'dispatch: loop {
        match pc {
            0x825AE3F0 => {
    //   block [0x825AE3F0..0x825AE444)
	// 825AE3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AE3F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AE3F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AE3FC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AE400: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AE404: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AE408: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AE40C: 419A000C  beq cr6, 0x825ae418
	if ctx.cr[6].eq {
	pc = 0x825AE418; continue 'dispatch;
	}
	// 825AE410: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825AE414: 48000008  b 0x825ae41c
	pc = 0x825AE41C; continue 'dispatch;
	// 825AE418: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825AE41C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825AE420: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825AE424: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AE428: 4BFF76F9  bl 0x825a5b20
	ctx.lr = 0x825AE42C;
	sub_825A5B20(ctx, base);
	// 825AE42C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AE430: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AE434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AE438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AE43C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AE440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AE448 size=88
    let mut pc: u32 = 0x825AE448;
    'dispatch: loop {
        match pc {
            0x825AE448 => {
    //   block [0x825AE448..0x825AE4A0)
	// 825AE448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AE44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AE450: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AE454: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AE458: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AE45C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825AE460: 4BFF7AB9  bl 0x825a5f18
	ctx.lr = 0x825AE464;
	sub_825A5F18(ctx, base);
	// 825AE464: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AE468: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AE46C: 419A0008  beq cr6, 0x825ae474
	if ctx.cr[6].eq {
	pc = 0x825AE474; continue 'dispatch;
	}
	// 825AE470: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 825AE474: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AE478: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 825AE47C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AE480: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AE484: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AE488: 4E800421  bctrl
	ctx.lr = 0x825AE48C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AE48C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AE490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AE494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AE498: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AE49C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AE4A0 size=84
    let mut pc: u32 = 0x825AE4A0;
    'dispatch: loop {
        match pc {
            0x825AE4A0 => {
    //   block [0x825AE4A0..0x825AE4F4)
	// 825AE4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AE4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AE4A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AE4AC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AE4B0: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AE4B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AE4B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AE4BC: 419A000C  beq cr6, 0x825ae4c8
	if ctx.cr[6].eq {
	pc = 0x825AE4C8; continue 'dispatch;
	}
	// 825AE4C0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AE4C4: 48000008  b 0x825ae4cc
	pc = 0x825AE4CC; continue 'dispatch;
	// 825AE4C8: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AE4CC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825AE4D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825AE4D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AE4D8: 4BFF7971  bl 0x825a5e48
	ctx.lr = 0x825AE4DC;
	sub_825A5E48(ctx, base);
	// 825AE4DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AE4E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AE4E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AE4E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AE4EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AE4F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AE4F8 size=88
    let mut pc: u32 = 0x825AE4F8;
    'dispatch: loop {
        match pc {
            0x825AE4F8 => {
    //   block [0x825AE4F8..0x825AE550)
	// 825AE4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AE4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AE500: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AE504: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AE508: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AE50C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825AE510: 4BFF7879  bl 0x825a5d88
	ctx.lr = 0x825AE514;
	sub_825A5D88(ctx, base);
	// 825AE514: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AE518: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AE51C: 419A0008  beq cr6, 0x825ae524
	if ctx.cr[6].eq {
	pc = 0x825AE524; continue 'dispatch;
	}
	// 825AE520: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 825AE524: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AE528: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 825AE52C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AE530: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AE534: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AE538: 4E800421  bctrl
	ctx.lr = 0x825AE53C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AE53C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AE540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AE544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AE548: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AE54C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AE550 size=84
    let mut pc: u32 = 0x825AE550;
    'dispatch: loop {
        match pc {
            0x825AE550 => {
    //   block [0x825AE550..0x825AE5A4)
	// 825AE550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AE554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AE558: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AE55C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AE560: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AE564: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AE568: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AE56C: 419A000C  beq cr6, 0x825ae578
	if ctx.cr[6].eq {
	pc = 0x825AE578; continue 'dispatch;
	}
	// 825AE570: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AE574: 48000008  b 0x825ae57c
	pc = 0x825AE57C; continue 'dispatch;
	// 825AE578: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AE57C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825AE580: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825AE584: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AE588: 4BFF7731  bl 0x825a5cb8
	ctx.lr = 0x825AE58C;
	sub_825A5CB8(ctx, base);
	// 825AE58C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AE590: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AE594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AE598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AE59C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AE5A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AE5A8 size=88
    let mut pc: u32 = 0x825AE5A8;
    'dispatch: loop {
        match pc {
            0x825AE5A8 => {
    //   block [0x825AE5A8..0x825AE600)
	// 825AE5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AE5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AE5B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AE5B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AE5B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AE5BC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825AE5C0: 4BFF7A19  bl 0x825a5fd8
	ctx.lr = 0x825AE5C4;
	sub_825A5FD8(ctx, base);
	// 825AE5C4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AE5C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AE5CC: 419A0008  beq cr6, 0x825ae5d4
	if ctx.cr[6].eq {
	pc = 0x825AE5D4; continue 'dispatch;
	}
	// 825AE5D0: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 825AE5D4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AE5D8: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 825AE5DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AE5E0: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825AE5E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AE5E8: 4E800421  bctrl
	ctx.lr = 0x825AE5EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AE5EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AE5F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AE5F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AE5F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AE5FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AE600 size=84
    let mut pc: u32 = 0x825AE600;
    'dispatch: loop {
        match pc {
            0x825AE600 => {
    //   block [0x825AE600..0x825AE654)
	// 825AE600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AE604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AE608: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AE60C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AE610: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AE614: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AE618: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AE61C: 419A000C  beq cr6, 0x825ae628
	if ctx.cr[6].eq {
	pc = 0x825AE628; continue 'dispatch;
	}
	// 825AE620: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AE624: 48000008  b 0x825ae62c
	pc = 0x825AE62C; continue 'dispatch;
	// 825AE628: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AE62C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825AE630: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825AE634: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AE638: 4BE05561  bl 0x823b3b98
	ctx.lr = 0x825AE63C;
	sub_823B3B98(ctx, base);
	// 825AE63C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AE640: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AE644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AE648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AE64C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AE650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AE658 size=104
    let mut pc: u32 = 0x825AE658;
    'dispatch: loop {
        match pc {
            0x825AE658 => {
    //   block [0x825AE658..0x825AE6C0)
	// 825AE658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AE65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AE660: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AE664: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AE668: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AE66C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AE670: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825AE674: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825AE678: 4802B719  bl 0x825d9d90
	ctx.lr = 0x825AE67C;
	sub_825D9D90(ctx, base);
	// 825AE67C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825AE680: 419A0010  beq cr6, 0x825ae690
	if ctx.cr[6].eq {
	pc = 0x825AE690; continue 'dispatch;
	}
	// 825AE684: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825AE688: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AE68C: 4BFF3F3D  bl 0x825a25c8
	ctx.lr = 0x825AE690;
	sub_825A25C8(ctx, base);
	// 825AE690: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825AE694: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AE698: 4802B769  bl 0x825d9e00
	ctx.lr = 0x825AE69C;
	sub_825D9E00(ctx, base);
	// 825AE69C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AE6A0: 4802B729  bl 0x825d9dc8
	ctx.lr = 0x825AE6A4;
	sub_825D9DC8(ctx, base);
	// 825AE6A4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825AE6A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AE6AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AE6B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AE6B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AE6B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AE6BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AE6C0 size=8
    let mut pc: u32 = 0x825AE6C0;
    'dispatch: loop {
        match pc {
            0x825AE6C0 => {
    //   block [0x825AE6C0..0x825AE6C8)
	// 825AE6C0: 88630150  lbz r3, 0x150(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(336 as u32) ) } as u64;
	// 825AE6C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AE6C8 size=136
    let mut pc: u32 = 0x825AE6C8;
    'dispatch: loop {
        match pc {
            0x825AE6C8 => {
    //   block [0x825AE6C8..0x825AE750)
	// 825AE6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AE6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AE6D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AE6D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AE6D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AE6DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825AE6E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825AE6E4: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 825AE6E8: 409A0020  bne cr6, 0x825ae708
	if !ctx.cr[6].eq {
	pc = 0x825AE708; continue 'dispatch;
	}
	// 825AE6EC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825AE6F0: 419A0048  beq cr6, 0x825ae738
	if ctx.cr[6].eq {
	pc = 0x825AE738; continue 'dispatch;
	}
	// 825AE6F4: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 825AE6F8: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 825AE6FC: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 825AE700: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 825AE704: 48000034  b 0x825ae738
	pc = 0x825AE738; continue 'dispatch;
	// 825AE708: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 825AE70C: 419A002C  beq cr6, 0x825ae738
	if ctx.cr[6].eq {
	pc = 0x825AE738; continue 'dispatch;
	}
	// 825AE710: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825AE714: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AE718: 388B8858  addi r4, r11, -0x77a8
	ctx.r[4].s64 = ctx.r[11].s64 + -30632;
	// 825AE71C: 48BF99DD  bl 0x831a80f8
	ctx.lr = 0x825AE720;
	sub_831A80F8(ctx, base);
	// 825AE720: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AE724: 4182000C  beq 0x825ae730
	if ctx.cr[0].eq {
	pc = 0x825AE730; continue 'dispatch;
	}
	// 825AE728: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 825AE72C: 4800000C  b 0x825ae738
	pc = 0x825AE738; continue 'dispatch;
	// 825AE730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AE734: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AE738: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AE73C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AE740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AE744: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AE748: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AE74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AE750 size=136
    let mut pc: u32 = 0x825AE750;
    'dispatch: loop {
        match pc {
            0x825AE750 => {
    //   block [0x825AE750..0x825AE7D8)
	// 825AE750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AE754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AE758: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AE75C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AE760: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AE764: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825AE768: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825AE76C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 825AE770: 409A0020  bne cr6, 0x825ae790
	if !ctx.cr[6].eq {
	pc = 0x825AE790; continue 'dispatch;
	}
	// 825AE774: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825AE778: 419A0048  beq cr6, 0x825ae7c0
	if ctx.cr[6].eq {
	pc = 0x825AE7C0; continue 'dispatch;
	}
	// 825AE77C: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 825AE780: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 825AE784: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 825AE788: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 825AE78C: 48000034  b 0x825ae7c0
	pc = 0x825AE7C0; continue 'dispatch;
	// 825AE790: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 825AE794: 419A002C  beq cr6, 0x825ae7c0
	if ctx.cr[6].eq {
	pc = 0x825AE7C0; continue 'dispatch;
	}
	// 825AE798: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825AE79C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AE7A0: 388B8918  addi r4, r11, -0x76e8
	ctx.r[4].s64 = ctx.r[11].s64 + -30440;
	// 825AE7A4: 48BF9955  bl 0x831a80f8
	ctx.lr = 0x825AE7A8;
	sub_831A80F8(ctx, base);
	// 825AE7A8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AE7AC: 4182000C  beq 0x825ae7b8
	if ctx.cr[0].eq {
	pc = 0x825AE7B8; continue 'dispatch;
	}
	// 825AE7B0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 825AE7B4: 4800000C  b 0x825ae7c0
	pc = 0x825AE7C0; continue 'dispatch;
	// 825AE7B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AE7BC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AE7C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AE7C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AE7C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AE7CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AE7D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AE7D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AE7D8 size=68
    let mut pc: u32 = 0x825AE7D8;
    'dispatch: loop {
        match pc {
            0x825AE7D8 => {
    //   block [0x825AE7D8..0x825AE81C)
	// 825AE7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AE7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AE7E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AE7E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AE7E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825AE7EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AE7F0: 488AAF51  bl 0x82e59740
	ctx.lr = 0x825AE7F4;
	sub_82E59740(ctx, base);
	// 825AE7F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AE7F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AE7FC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825AE800: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AE804: 4E800421  bctrl
	ctx.lr = 0x825AE808;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AE808: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AE80C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AE810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AE814: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AE818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AE820 size=40
    let mut pc: u32 = 0x825AE820;
    'dispatch: loop {
        match pc {
            0x825AE820 => {
    //   block [0x825AE820..0x825AE848)
	// 825AE820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AE824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AE828: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AE82C: 48BA5DD5  bl 0x83154600
	ctx.lr = 0x825AE830;
	sub_83154600(ctx, base);
	// 825AE830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AE834: 91630168  stw r11, 0x168(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(360 as u32), ctx.r[11].u32 ) };
	// 825AE838: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AE83C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AE840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AE844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AE848 size=40
    let mut pc: u32 = 0x825AE848;
    'dispatch: loop {
        match pc {
            0x825AE848 => {
    //   block [0x825AE848..0x825AE870)
	// 825AE848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AE84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AE850: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AE854: 48BA5DAD  bl 0x83154600
	ctx.lr = 0x825AE858;
	sub_83154600(ctx, base);
	// 825AE858: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825AE85C: 91630168  stw r11, 0x168(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(360 as u32), ctx.r[11].u32 ) };
	// 825AE860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AE864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AE868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AE86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825AE870 size=100
    let mut pc: u32 = 0x825AE870;
    'dispatch: loop {
        match pc {
            0x825AE870 => {
    //   block [0x825AE870..0x825AE8D4)
	// 825AE870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AE874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AE878: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AE87C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AE880: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AE884: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AE888: 48BA5D79  bl 0x83154600
	ctx.lr = 0x825AE88C;
	sub_83154600(ctx, base);
	// 825AE88C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825AE890: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AE894: 488AAEE5  bl 0x82e59778
	ctx.lr = 0x825AE898;
	sub_82E59778(ctx, base);
	// 825AE898: 387E0170  addi r3, r30, 0x170
	ctx.r[3].s64 = ctx.r[30].s64 + 368;
	// 825AE89C: 4800309D  bl 0x825b1938
	ctx.lr = 0x825AE8A0;
	sub_825B1938(ctx, base);
	// 825AE8A0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AE8A4: 40820018  bne 0x825ae8bc
	if !ctx.cr[0].eq {
	pc = 0x825AE8BC; continue 'dispatch;
	}
	// 825AE8A8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825AE8AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825AE8B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AE8B4: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825AE8B8: 488AE181  bl 0x82e5ca38
	ctx.lr = 0x825AE8BC;
	sub_82E5CA38(ctx, base);
	// 825AE8BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AE8C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AE8C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AE8C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AE8CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AE8D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AE8D8 size=196
    let mut pc: u32 = 0x825AE8D8;
    'dispatch: loop {
        match pc {
            0x825AE8D8 => {
    //   block [0x825AE8D8..0x825AE99C)
	// 825AE8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AE8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AE8E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AE8E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AE8E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AE8EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825AE8F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AE8F4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825AE8F8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825AE8FC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AE900: 4BD12039  bl 0x822c0938
	ctx.lr = 0x825AE904;
	sub_822C0938(ctx, base);
	// 825AE904: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825AE908: 41820028  beq 0x825ae930
	if ctx.cr[0].eq {
	pc = 0x825AE930; continue 'dispatch;
	}
	// 825AE90C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AE910: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825AE914: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825AE918: 392BB750  addi r9, r11, -0x48b0
	ctx.r[9].s64 = ctx.r[11].s64 + -18608;
	// 825AE91C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825AE920: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825AE924: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825AE928: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825AE92C: 48000008  b 0x825ae934
	pc = 0x825AE934; continue 'dispatch;
	// 825AE930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AE934: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AE938: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AE93C: 409A0044  bne cr6, 0x825ae980
	if !ctx.cr[6].eq {
	pc = 0x825AE980; continue 'dispatch;
	}
	// 825AE940: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825AE944: 419A001C  beq cr6, 0x825ae960
	if ctx.cr[6].eq {
	pc = 0x825AE960; continue 'dispatch;
	}
	// 825AE948: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AE94C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825AE950: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AE954: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AE958: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AE95C: 4E800421  bctrl
	ctx.lr = 0x825AE960;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AE960: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825AE964: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825AE968: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AE96C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825AE970: 816B856C  lwz r11, -0x7a94(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31380 as u32) ) } as u64;
	// 825AE974: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825AE978: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825AE97C: 4BD11685  bl 0x822c0000
	ctx.lr = 0x825AE980;
	sub_822C0000(ctx, base);
	// 825AE980: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AE984: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AE988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AE98C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AE990: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AE994: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AE998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AE9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AE9A0 size=196
    let mut pc: u32 = 0x825AE9A0;
    'dispatch: loop {
        match pc {
            0x825AE9A0 => {
    //   block [0x825AE9A0..0x825AEA64)
	// 825AE9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AE9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AE9A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AE9AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AE9B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AE9B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825AE9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AE9BC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825AE9C0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825AE9C4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AE9C8: 4BD11F71  bl 0x822c0938
	ctx.lr = 0x825AE9CC;
	sub_822C0938(ctx, base);
	// 825AE9CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825AE9D0: 41820028  beq 0x825ae9f8
	if ctx.cr[0].eq {
	pc = 0x825AE9F8; continue 'dispatch;
	}
	// 825AE9D4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AE9D8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825AE9DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825AE9E0: 392BB78C  addi r9, r11, -0x4874
	ctx.r[9].s64 = ctx.r[11].s64 + -18548;
	// 825AE9E4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825AE9E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825AE9EC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825AE9F0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825AE9F4: 48000008  b 0x825ae9fc
	pc = 0x825AE9FC; continue 'dispatch;
	// 825AE9F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AE9FC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AEA00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AEA04: 409A0044  bne cr6, 0x825aea48
	if !ctx.cr[6].eq {
	pc = 0x825AEA48; continue 'dispatch;
	}
	// 825AEA08: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825AEA0C: 419A001C  beq cr6, 0x825aea28
	if ctx.cr[6].eq {
	pc = 0x825AEA28; continue 'dispatch;
	}
	// 825AEA10: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AEA14: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825AEA18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AEA1C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AEA20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AEA24: 4E800421  bctrl
	ctx.lr = 0x825AEA28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AEA28: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825AEA2C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825AEA30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AEA34: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825AEA38: 816B856C  lwz r11, -0x7a94(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31380 as u32) ) } as u64;
	// 825AEA3C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825AEA40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825AEA44: 4BD115BD  bl 0x822c0000
	ctx.lr = 0x825AEA48;
	sub_822C0000(ctx, base);
	// 825AEA48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AEA4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AEA50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AEA54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AEA58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AEA5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AEA60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AEA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AEA68 size=196
    let mut pc: u32 = 0x825AEA68;
    'dispatch: loop {
        match pc {
            0x825AEA68 => {
    //   block [0x825AEA68..0x825AEB2C)
	// 825AEA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AEA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AEA70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AEA74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AEA78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AEA7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825AEA80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AEA84: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825AEA88: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825AEA8C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AEA90: 4BD11EA9  bl 0x822c0938
	ctx.lr = 0x825AEA94;
	sub_822C0938(ctx, base);
	// 825AEA94: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825AEA98: 41820028  beq 0x825aeac0
	if ctx.cr[0].eq {
	pc = 0x825AEAC0; continue 'dispatch;
	}
	// 825AEA9C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AEAA0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825AEAA4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825AEAA8: 392BB7A0  addi r9, r11, -0x4860
	ctx.r[9].s64 = ctx.r[11].s64 + -18528;
	// 825AEAAC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825AEAB0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825AEAB4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825AEAB8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825AEABC: 48000008  b 0x825aeac4
	pc = 0x825AEAC4; continue 'dispatch;
	// 825AEAC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AEAC4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AEAC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AEACC: 409A0044  bne cr6, 0x825aeb10
	if !ctx.cr[6].eq {
	pc = 0x825AEB10; continue 'dispatch;
	}
	// 825AEAD0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825AEAD4: 419A001C  beq cr6, 0x825aeaf0
	if ctx.cr[6].eq {
	pc = 0x825AEAF0; continue 'dispatch;
	}
	// 825AEAD8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AEADC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825AEAE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AEAE4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AEAE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AEAEC: 4E800421  bctrl
	ctx.lr = 0x825AEAF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AEAF0: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825AEAF4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825AEAF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AEAFC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825AEB00: 816B856C  lwz r11, -0x7a94(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31380 as u32) ) } as u64;
	// 825AEB04: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825AEB08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825AEB0C: 4BD114F5  bl 0x822c0000
	ctx.lr = 0x825AEB10;
	sub_822C0000(ctx, base);
	// 825AEB10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AEB14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AEB18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AEB1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AEB20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AEB24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AEB28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AEB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AEB30 size=196
    let mut pc: u32 = 0x825AEB30;
    'dispatch: loop {
        match pc {
            0x825AEB30 => {
    //   block [0x825AEB30..0x825AEBF4)
	// 825AEB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AEB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AEB38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AEB3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AEB40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AEB44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825AEB48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AEB4C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825AEB50: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825AEB54: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AEB58: 4BD11DE1  bl 0x822c0938
	ctx.lr = 0x825AEB5C;
	sub_822C0938(ctx, base);
	// 825AEB5C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825AEB60: 41820028  beq 0x825aeb88
	if ctx.cr[0].eq {
	pc = 0x825AEB88; continue 'dispatch;
	}
	// 825AEB64: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AEB68: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825AEB6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825AEB70: 392BB7B4  addi r9, r11, -0x484c
	ctx.r[9].s64 = ctx.r[11].s64 + -18508;
	// 825AEB74: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825AEB78: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825AEB7C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825AEB80: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825AEB84: 48000008  b 0x825aeb8c
	pc = 0x825AEB8C; continue 'dispatch;
	// 825AEB88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AEB8C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AEB90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AEB94: 409A0044  bne cr6, 0x825aebd8
	if !ctx.cr[6].eq {
	pc = 0x825AEBD8; continue 'dispatch;
	}
	// 825AEB98: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825AEB9C: 419A001C  beq cr6, 0x825aebb8
	if ctx.cr[6].eq {
	pc = 0x825AEBB8; continue 'dispatch;
	}
	// 825AEBA0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AEBA4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825AEBA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AEBAC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AEBB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AEBB4: 4E800421  bctrl
	ctx.lr = 0x825AEBB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AEBB8: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825AEBBC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825AEBC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AEBC4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825AEBC8: 816B856C  lwz r11, -0x7a94(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31380 as u32) ) } as u64;
	// 825AEBCC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825AEBD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825AEBD4: 4BD1142D  bl 0x822c0000
	ctx.lr = 0x825AEBD8;
	sub_822C0000(ctx, base);
	// 825AEBD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AEBDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AEBE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AEBE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AEBE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AEBEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AEBF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AEBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AEBF8 size=140
    let mut pc: u32 = 0x825AEBF8;
    'dispatch: loop {
        match pc {
            0x825AEBF8 => {
    //   block [0x825AEBF8..0x825AEC84)
	// 825AEBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AEBFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AEC00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AEC04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AEC08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AEC0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AEC10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825AEC14: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825AEC18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AEC1C: 419A003C  beq cr6, 0x825aec58
	if ctx.cr[6].eq {
	pc = 0x825AEC58; continue 'dispatch;
	}
	// 825AEC20: 39230008  addi r9, r3, 8
	ctx.r[9].s64 = ctx.r[3].s64 + 8;
	// 825AEC24: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 825AEC28: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AEC2C: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 825AEC30: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825AEC34: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825AEC38: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AEC3C: 4082FFE8  bne 0x825aec24
	if !ctx.cr[0].eq {
	pc = 0x825AEC24; continue 'dispatch;
	}
	// 825AEC40: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AEC44: 409A0014  bne cr6, 0x825aec58
	if !ctx.cr[6].eq {
	pc = 0x825AEC58; continue 'dispatch;
	}
	// 825AEC48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AEC4C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825AEC50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AEC54: 4E800421  bctrl
	ctx.lr = 0x825AEC58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AEC58: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AEC5C: 4182000C  beq 0x825aec68
	if ctx.cr[0].eq {
	pc = 0x825AEC68; continue 'dispatch;
	}
	// 825AEC60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AEC64: 4BD11605  bl 0x822c0268
	ctx.lr = 0x825AEC68;
	sub_822C0268(ctx, base);
	// 825AEC68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AEC6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AEC70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AEC74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AEC78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AEC7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AEC80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AEC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AEC88 size=72
    let mut pc: u32 = 0x825AEC88;
    'dispatch: loop {
        match pc {
            0x825AEC88 => {
    //   block [0x825AEC88..0x825AECD0)
	// 825AEC88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AEC8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AEC90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AEC94: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 825AEC98: 419A001C  beq cr6, 0x825aecb4
	if ctx.cr[6].eq {
	pc = 0x825AECB4; continue 'dispatch;
	}
	// 825AEC9C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825AECA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825AECA4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 825AECA8: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AECAC: 4BFFFA1D  bl 0x825ae6c8
	ctx.lr = 0x825AECB0;
	sub_825AE6C8(ctx, base);
	// 825AECB0: 48000010  b 0x825aecc0
	pc = 0x825AECC0; continue 'dispatch;
	// 825AECB4: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825AECB8: 396B8858  addi r11, r11, -0x77a8
	ctx.r[11].s64 = ctx.r[11].s64 + -30632;
	// 825AECBC: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AECC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AECC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AECC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AECCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AECD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AECD0 size=72
    let mut pc: u32 = 0x825AECD0;
    'dispatch: loop {
        match pc {
            0x825AECD0 => {
    //   block [0x825AECD0..0x825AED18)
	// 825AECD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AECD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AECD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AECDC: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 825AECE0: 419A001C  beq cr6, 0x825aecfc
	if ctx.cr[6].eq {
	pc = 0x825AECFC; continue 'dispatch;
	}
	// 825AECE4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825AECE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825AECEC: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 825AECF0: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AECF4: 4BFFFA5D  bl 0x825ae750
	ctx.lr = 0x825AECF8;
	sub_825AE750(ctx, base);
	// 825AECF8: 48000010  b 0x825aed08
	pc = 0x825AED08; continue 'dispatch;
	// 825AECFC: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825AED00: 396B8918  addi r11, r11, -0x76e8
	ctx.r[11].s64 = ctx.r[11].s64 + -30440;
	// 825AED04: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AED08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AED0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AED10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AED14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AED18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AED18 size=136
    let mut pc: u32 = 0x825AED18;
    'dispatch: loop {
        match pc {
            0x825AED18 => {
    //   block [0x825AED18..0x825AEDA0)
	// 825AED18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AED1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AED20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AED24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AED28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AED2C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825AED30: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 825AED34: 396AB818  addi r11, r10, -0x47e8
	ctx.r[11].s64 = ctx.r[10].s64 + -18408;
	// 825AED38: 98BF0004  stb r5, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 825AED3C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AED40: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AED44: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825AED48: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AED4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AED50: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 825AED54: 419A0024  beq cr6, 0x825aed78
	if ctx.cr[6].eq {
	pc = 0x825AED78; continue 'dispatch;
	}
	// 825AED58: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 825AED5C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825AED60: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AED64: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825AED68: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825AED6C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825AED70: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AED74: 4082FFE8  bne 0x825aed5c
	if !ctx.cr[0].eq {
	pc = 0x825AED5C; continue 'dispatch;
	}
	// 825AED78: 80640004  lwz r3, 4(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AED7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AED80: 419A0008  beq cr6, 0x825aed88
	if ctx.cr[6].eq {
	pc = 0x825AED88; continue 'dispatch;
	}
	// 825AED84: 4BD11B0D  bl 0x822c0890
	ctx.lr = 0x825AED88;
	sub_822C0890(ctx, base);
	// 825AED88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AED8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AED90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AED94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AED98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AED9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AEDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AEDA0 size=136
    let mut pc: u32 = 0x825AEDA0;
    'dispatch: loop {
        match pc {
            0x825AEDA0 => {
    //   block [0x825AEDA0..0x825AEE28)
	// 825AEDA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AEDA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AEDA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AEDAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AEDB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AEDB4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825AEDB8: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 825AEDBC: 396AB824  addi r11, r10, -0x47dc
	ctx.r[11].s64 = ctx.r[10].s64 + -18396;
	// 825AEDC0: 98BF0004  stb r5, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 825AEDC4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AEDC8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AEDCC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825AEDD0: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AEDD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AEDD8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 825AEDDC: 419A0024  beq cr6, 0x825aee00
	if ctx.cr[6].eq {
	pc = 0x825AEE00; continue 'dispatch;
	}
	// 825AEDE0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825AEDE4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825AEDE8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AEDEC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825AEDF0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825AEDF4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825AEDF8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AEDFC: 4082FFE8  bne 0x825aede4
	if !ctx.cr[0].eq {
	pc = 0x825AEDE4; continue 'dispatch;
	}
	// 825AEE00: 80640004  lwz r3, 4(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AEE04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AEE08: 419A0008  beq cr6, 0x825aee10
	if ctx.cr[6].eq {
	pc = 0x825AEE10; continue 'dispatch;
	}
	// 825AEE0C: 4BD11A85  bl 0x822c0890
	ctx.lr = 0x825AEE10;
	sub_822C0890(ctx, base);
	// 825AEE10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AEE14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AEE18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AEE1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AEE20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AEE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AEE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825AEE28 size=416
    let mut pc: u32 = 0x825AEE28;
    'dispatch: loop {
        match pc {
            0x825AEE28 => {
    //   block [0x825AEE28..0x825AEFC8)
	// 825AEE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AEE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AEE30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AEE34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AEE38: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 825AEE3C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AEE40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AEE44: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825AEE48: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825AEE4C: 389F0028  addi r4, r31, 0x28
	ctx.r[4].s64 = ctx.r[31].s64 + 40;
	// 825AEE50: 409A0008  bne cr6, 0x825aee58
	if !ctx.cr[6].eq {
	pc = 0x825AEE58; continue 'dispatch;
	}
	// 825AEE54: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825AEE58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AEE5C: 4BF59945  bl 0x825087a0
	ctx.lr = 0x825AEE60;
	sub_825087A0(ctx, base);
	// 825AEE60: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825AEE64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AEE68: 808BE258  lwz r4, -0x1da8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7592 as u32) ) } as u64;
	// 825AEE6C: 48844B9D  bl 0x82df3a08
	ctx.lr = 0x825AEE70;
	sub_82DF3A08(ctx, base);
	// 825AEE70: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825AEE74: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825AEE78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AEE7C: 4BF59905  bl 0x82508780
	ctx.lr = 0x825AEE80;
	sub_82508780(ctx, base);
	// 825AEE80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AEE84: 488445A5  bl 0x82df3428
	ctx.lr = 0x825AEE88;
	sub_82DF3428(ctx, base);
	// 825AEE88: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825AEE8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AEE90: 808BE254  lwz r4, -0x1dac(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7596 as u32) ) } as u64;
	// 825AEE94: 48844B75  bl 0x82df3a08
	ctx.lr = 0x825AEE98;
	sub_82DF3A08(ctx, base);
	// 825AEE98: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825AEE9C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825AEEA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AEEA4: 4BF598DD  bl 0x82508780
	ctx.lr = 0x825AEEA8;
	sub_82508780(ctx, base);
	// 825AEEA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AEEAC: 4884457D  bl 0x82df3428
	ctx.lr = 0x825AEEB0;
	sub_82DF3428(ctx, base);
	// 825AEEB0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825AEEB4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825AEEB8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825AEEBC: 915F0154  stw r10, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[10].u32 ) };
	// 825AEEC0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825AEEC4: 993F0150  stb r9, 0x150(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[9].u8 ) };
	// 825AEEC8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825AEECC: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825AEED0: 388AB830  addi r4, r10, -0x47d0
	ctx.r[4].s64 = ctx.r[10].s64 + -18384;
	// 825AEED4: D3FF014C  stfs f31, 0x14c(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(332 as u32), tmp.u32 ) };
	// 825AEED8: 38A00051  li r5, 0x51
	ctx.r[5].s64 = 81;
	// 825AEEDC: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 825AEEE0: 48843509  bl 0x82df23e8
	ctx.lr = 0x825AEEE4;
	sub_82DF23E8(ctx, base);
	// 825AEEE4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 825AEEE8: 4182001C  beq 0x825aef04
	if ctx.cr[0].eq {
	pc = 0x825AEF04; continue 'dispatch;
	}
	// 825AEEEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AEEF0: 488ADA39  bl 0x82e5c928
	ctx.lr = 0x825AEEF4;
	sub_82E5C928(ctx, base);
	// 825AEEF4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AEEF8: 396BB7F0  addi r11, r11, -0x4810
	ctx.r[11].s64 = ctx.r[11].s64 + -18448;
	// 825AEEFC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AEF00: 48000008  b 0x825aef08
	pc = 0x825AEF08; continue 'dispatch;
	// 825AEF04: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825AEF08: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 825AEF0C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825AEF10: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825AEF14: 4BFFF9C5  bl 0x825ae8d8
	ctx.lr = 0x825AEF18;
	sub_825AE8D8(ctx, base);
	// 825AEF18: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825AEF1C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825AEF20: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825AEF24: 4BD110DD  bl 0x822c0000
	ctx.lr = 0x825AEF28;
	sub_822C0000(ctx, base);
	// 825AEF28: 83C10054  lwz r30, 0x54(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825AEF2C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AEF30: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 825AEF34: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 825AEF38: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 825AEF3C: 419A0024  beq cr6, 0x825aef60
	if ctx.cr[6].eq {
	pc = 0x825AEF60; continue 'dispatch;
	}
	// 825AEF40: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 825AEF44: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825AEF48: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AEF4C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825AEF50: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825AEF54: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825AEF58: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AEF5C: 4082FFE8  bne 0x825aef44
	if !ctx.cr[0].eq {
	pc = 0x825AEF44; continue 'dispatch;
	}
	// 825AEF60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825AEF64: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825AEF68: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825AEF6C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 825AEF70: 389F00CC  addi r4, r31, 0xcc
	ctx.r[4].s64 = ctx.r[31].s64 + 204;
	// 825AEF74: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825AEF78: 488AF731  bl 0x82e5e6a8
	ctx.lr = 0x825AEF7C;
	sub_82E5E6A8(ctx, base);
	// 825AEF7C: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 825AEF80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AEF84: 419A0008  beq cr6, 0x825aef8c
	if ctx.cr[6].eq {
	pc = 0x825AEF8C; continue 'dispatch;
	}
	// 825AEF88: 4BD11909  bl 0x822c0890
	ctx.lr = 0x825AEF8C;
	sub_822C0890(ctx, base);
	// 825AEF8C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825AEF90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AEF94: 419A0008  beq cr6, 0x825aef9c
	if ctx.cr[6].eq {
	pc = 0x825AEF9C; continue 'dispatch;
	}
	// 825AEF98: 4BD118F9  bl 0x822c0890
	ctx.lr = 0x825AEF9C;
	sub_822C0890(ctx, base);
	// 825AEF9C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 825AEFA0: 419A000C  beq cr6, 0x825aefac
	if ctx.cr[6].eq {
	pc = 0x825AEFAC; continue 'dispatch;
	}
	// 825AEFA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AEFA8: 4BD118E9  bl 0x822c0890
	ctx.lr = 0x825AEFAC;
	sub_822C0890(ctx, base);
	// 825AEFAC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825AEFB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AEFB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AEFB8: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 825AEFBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AEFC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AEFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AEFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AEFC8 size=132
    let mut pc: u32 = 0x825AEFC8;
    'dispatch: loop {
        match pc {
            0x825AEFC8 => {
    //   block [0x825AEFC8..0x825AF04C)
	// 825AEFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AEFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AEFD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AEFD4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AEFD8: 81630144  lwz r11, 0x144(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(324 as u32) ) } as u64;
	// 825AEFDC: 81430130  lwz r10, 0x130(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(304 as u32) ) } as u64;
	// 825AEFE0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825AEFE4: 419A0050  beq cr6, 0x825af034
	if ctx.cr[6].eq {
	pc = 0x825AF034; continue 'dispatch;
	}
	// 825AEFE8: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AEFEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AEFF0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AEFF4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AEFF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AEFFC: 4E800421  bctrl
	ctx.lr = 0x825AF000;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AF000: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AF004: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825AF008: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AF00C: 419A001C  beq cr6, 0x825af028
	if ctx.cr[6].eq {
	pc = 0x825AF028; continue 'dispatch;
	}
	// 825AF010: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 825AF014: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AF018: 419A0008  beq cr6, 0x825af020
	if ctx.cr[6].eq {
	pc = 0x825AF020; continue 'dispatch;
	}
	// 825AF01C: 4BD11875  bl 0x822c0890
	ctx.lr = 0x825AF020;
	sub_822C0890(ctx, base);
	// 825AF020: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF024: 48000014  b 0x825af038
	pc = 0x825AF038; continue 'dispatch;
	// 825AF028: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AF02C: 419A0008  beq cr6, 0x825af034
	if ctx.cr[6].eq {
	pc = 0x825AF034; continue 'dispatch;
	}
	// 825AF030: 4BD11861  bl 0x822c0890
	ctx.lr = 0x825AF034;
	sub_822C0890(ctx, base);
	// 825AF034: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825AF038: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AF03C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AF040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AF044: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AF048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AF050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AF050 size=172
    let mut pc: u32 = 0x825AF050;
    'dispatch: loop {
        match pc {
            0x825AF050 => {
    //   block [0x825AF050..0x825AF0FC)
	// 825AF050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AF054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AF058: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AF05C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AF060: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 825AF064: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AF068: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AF06C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 825AF070: 4BFFFF59  bl 0x825aefc8
	ctx.lr = 0x825AF074;
	sub_825AEFC8(ctx, base);
	// 825AF074: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825AF078: 41820068  beq 0x825af0e0
	if ctx.cr[0].eq {
	pc = 0x825AF0E0; continue 'dispatch;
	}
	// 825AF07C: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 825AF080: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 825AF084: 41980020  blt cr6, 0x825af0a4
	if ctx.cr[6].lt {
	pc = 0x825AF0A4; continue 'dispatch;
	}
	// 825AF088: 815F01CC  lwz r10, 0x1cc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(460 as u32) ) } as u64;
	// 825AF08C: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 825AF090: 409A0050  bne cr6, 0x825af0e0
	if !ctx.cr[6].eq {
	pc = 0x825AF0E0; continue 'dispatch;
	}
	// 825AF094: 388BFFFE  addi r4, r11, -2
	ctx.r[4].s64 = ctx.r[11].s64 + -2;
	// 825AF098: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825AF09C: 4BFF345D  bl 0x825a24f8
	ctx.lr = 0x825AF0A0;
	sub_825A24F8(ctx, base);
	// 825AF0A0: 48000040  b 0x825af0e0
	pc = 0x825AF0E0; continue 'dispatch;
	// 825AF0A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AF0A8: 409A0038  bne cr6, 0x825af0e0
	if !ctx.cr[6].eq {
	pc = 0x825AF0E0; continue 'dispatch;
	}
	// 825AF0AC: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 825AF0B0: 3BDF0144  addi r30, r31, 0x144
	ctx.r[30].s64 = ctx.r[31].s64 + 324;
	// 825AF0B4: 815F0144  lwz r10, 0x144(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(324 as u32) ) } as u64;
	// 825AF0B8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AF0BC: 419A0024  beq cr6, 0x825af0e0
	if ctx.cr[6].eq {
	pc = 0x825AF0E0; continue 'dispatch;
	}
	// 825AF0C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AF0C4: 4BDF25C5  bl 0x823a1688
	ctx.lr = 0x825AF0C8;
	sub_823A1688(ctx, base);
	// 825AF0C8: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 825AF0CC: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AF0D0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AF0D4: 409A000C  bne cr6, 0x825af0e0
	if !ctx.cr[6].eq {
	pc = 0x825AF0E0; continue 'dispatch;
	}
	// 825AF0D8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AF0DC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AF0E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AF0E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AF0E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AF0EC: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 825AF0F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AF0F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AF0F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AF100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AF100 size=152
    let mut pc: u32 = 0x825AF100;
    'dispatch: loop {
        match pc {
            0x825AF100 => {
    //   block [0x825AF100..0x825AF198)
	// 825AF100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AF104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AF108: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AF10C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 825AF110: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AF114: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AF118: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 825AF11C: 4BFFFEAD  bl 0x825aefc8
	ctx.lr = 0x825AF120;
	sub_825AEFC8(ctx, base);
	// 825AF120: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825AF124: 4182005C  beq 0x825af180
	if ctx.cr[0].eq {
	pc = 0x825AF180; continue 'dispatch;
	}
	// 825AF128: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 825AF12C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 825AF130: 41980020  blt cr6, 0x825af150
	if ctx.cr[6].lt {
	pc = 0x825AF150; continue 'dispatch;
	}
	// 825AF134: 815F01CC  lwz r10, 0x1cc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(460 as u32) ) } as u64;
	// 825AF138: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 825AF13C: 409A0044  bne cr6, 0x825af180
	if !ctx.cr[6].eq {
	pc = 0x825AF180; continue 'dispatch;
	}
	// 825AF140: 388BFFFE  addi r4, r11, -2
	ctx.r[4].s64 = ctx.r[11].s64 + -2;
	// 825AF144: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825AF148: 4BFF33D1  bl 0x825a2518
	ctx.lr = 0x825AF14C;
	sub_825A2518(ctx, base);
	// 825AF14C: 48000034  b 0x825af180
	pc = 0x825AF180; continue 'dispatch;
	// 825AF150: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AF154: 409A002C  bne cr6, 0x825af180
	if !ctx.cr[6].eq {
	pc = 0x825AF180; continue 'dispatch;
	}
	// 825AF158: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 825AF15C: 387F0144  addi r3, r31, 0x144
	ctx.r[3].s64 = ctx.r[31].s64 + 324;
	// 825AF160: 815F0144  lwz r10, 0x144(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(324 as u32) ) } as u64;
	// 825AF164: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AF168: 419A0018  beq cr6, 0x825af180
	if ctx.cr[6].eq {
	pc = 0x825AF180; continue 'dispatch;
	}
	// 825AF16C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AF170: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825AF174: 409A0008  bne cr6, 0x825af17c
	if !ctx.cr[6].eq {
	pc = 0x825AF17C; continue 'dispatch;
	}
	// 825AF178: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AF17C: 481FAC9D  bl 0x827a9e18
	ctx.lr = 0x825AF180;
	sub_827A9E18(ctx, base);
	// 825AF180: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AF184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AF188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AF18C: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AF190: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AF194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AF198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AF198 size=88
    let mut pc: u32 = 0x825AF198;
    'dispatch: loop {
        match pc {
            0x825AF198 => {
    //   block [0x825AF198..0x825AF1F0)
	// 825AF198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AF19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AF1A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AF1A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AF1A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AF1AC: 4BFFFE1D  bl 0x825aefc8
	ctx.lr = 0x825AF1B0;
	sub_825AEFC8(ctx, base);
	// 825AF1B0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825AF1B4: 41820028  beq 0x825af1dc
	if ctx.cr[0].eq {
	pc = 0x825AF1DC; continue 'dispatch;
	}
	// 825AF1B8: 4BFF1931  bl 0x825a0ae8
	ctx.lr = 0x825AF1BC;
	sub_825A0AE8(ctx, base);
	// 825AF1BC: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 825AF1C0: 39430002  addi r10, r3, 2
	ctx.r[10].s64 = ctx.r[3].s64 + 2;
	// 825AF1C4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825AF1C8: 917F0148  stw r11, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[11].u32 ) };
	// 825AF1CC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 825AF1D0: 4198000C  blt cr6, 0x825af1dc
	if ctx.cr[6].lt {
	pc = 0x825AF1DC; continue 'dispatch;
	}
	// 825AF1D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AF1D8: 917F0148  stw r11, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[11].u32 ) };
	// 825AF1DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AF1E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AF1E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AF1E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AF1EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AF1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AF1F0 size=84
    let mut pc: u32 = 0x825AF1F0;
    'dispatch: loop {
        match pc {
            0x825AF1F0 => {
    //   block [0x825AF1F0..0x825AF244)
	// 825AF1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AF1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AF1F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AF1FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AF200: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AF204: 4BFFFDC5  bl 0x825aefc8
	ctx.lr = 0x825AF208;
	sub_825AEFC8(ctx, base);
	// 825AF208: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825AF20C: 41820024  beq 0x825af230
	if ctx.cr[0].eq {
	pc = 0x825AF230; continue 'dispatch;
	}
	// 825AF210: 4BFF18D9  bl 0x825a0ae8
	ctx.lr = 0x825AF214;
	sub_825A0AE8(ctx, base);
	// 825AF214: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 825AF218: 39430002  addi r10, r3, 2
	ctx.r[10].s64 = ctx.r[3].s64 + 2;
	// 825AF21C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AF220: 917F0148  stw r11, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[11].u32 ) };
	// 825AF224: 4080000C  bge 0x825af230
	if !ctx.cr[0].lt {
	pc = 0x825AF230; continue 'dispatch;
	}
	// 825AF228: 396AFFFF  addi r11, r10, -1
	ctx.r[11].s64 = ctx.r[10].s64 + -1;
	// 825AF22C: 917F0148  stw r11, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[11].u32 ) };
	// 825AF230: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AF234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AF238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AF23C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AF240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AF248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AF248 size=120
    let mut pc: u32 = 0x825AF248;
    'dispatch: loop {
        match pc {
            0x825AF248 => {
    //   block [0x825AF248..0x825AF2C0)
	// 825AF248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AF24C: 48BF8F21  bl 0x831a816c
	ctx.lr = 0x825AF250;
	sub_831A8130(ctx, base);
	// 825AF250: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AF254: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825AF258: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825AF25C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825AF260: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825AF264: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 825AF268: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 825AF26C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 825AF270: 48843179  bl 0x82df23e8
	ctx.lr = 0x825AF274;
	sub_82DF23E8(ctx, base);
	// 825AF274: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825AF278: 41820014  beq 0x825af28c
	if ctx.cr[0].eq {
	pc = 0x825AF28C; continue 'dispatch;
	}
	// 825AF27C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AF280: 485ABE71  bl 0x82b5b0f0
	ctx.lr = 0x825AF284;
	sub_82B5B0F0(ctx, base);
	// 825AF284: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AF288: 48000008  b 0x825af290
	pc = 0x825AF290; continue 'dispatch;
	// 825AF28C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825AF290: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825AF294: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 825AF298: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AF29C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AF2A0: 4BFFF7C9  bl 0x825aea68
	ctx.lr = 0x825AF2A4;
	sub_825AEA68(ctx, base);
	// 825AF2A4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825AF2A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AF2AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AF2B0: 4BD10D51  bl 0x822c0000
	ctx.lr = 0x825AF2B4;
	sub_822C0000(ctx, base);
	// 825AF2B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825AF2B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AF2BC: 48BF8F00  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AF2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AF2C0 size=120
    let mut pc: u32 = 0x825AF2C0;
    'dispatch: loop {
        match pc {
            0x825AF2C0 => {
    //   block [0x825AF2C0..0x825AF338)
	// 825AF2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AF2C4: 48BF8EA9  bl 0x831a816c
	ctx.lr = 0x825AF2C8;
	sub_831A8130(ctx, base);
	// 825AF2C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AF2CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825AF2D0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825AF2D4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825AF2D8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825AF2DC: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 825AF2E0: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 825AF2E4: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 825AF2E8: 48843101  bl 0x82df23e8
	ctx.lr = 0x825AF2EC;
	sub_82DF23E8(ctx, base);
	// 825AF2EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825AF2F0: 41820014  beq 0x825af304
	if ctx.cr[0].eq {
	pc = 0x825AF304; continue 'dispatch;
	}
	// 825AF2F4: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AF2F8: 485ABDA9  bl 0x82b5b0a0
	ctx.lr = 0x825AF2FC;
	sub_82B5B0A0(ctx, base);
	// 825AF2FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AF300: 48000008  b 0x825af308
	pc = 0x825AF308; continue 'dispatch;
	// 825AF304: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825AF308: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825AF30C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 825AF310: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AF314: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AF318: 4BFFF819  bl 0x825aeb30
	ctx.lr = 0x825AF31C;
	sub_825AEB30(ctx, base);
	// 825AF31C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825AF320: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AF324: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AF328: 4BD10CD9  bl 0x822c0000
	ctx.lr = 0x825AF32C;
	sub_822C0000(ctx, base);
	// 825AF32C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825AF330: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AF334: 48BF8E88  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AF338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AF338 size=52
    let mut pc: u32 = 0x825AF338;
    'dispatch: loop {
        match pc {
            0x825AF338 => {
    //   block [0x825AF338..0x825AF36C)
	// 825AF338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AF33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AF340: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AF344: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AF348: 38840008  addi r4, r4, 8
	ctx.r[4].s64 = ctx.r[4].s64 + 8;
	// 825AF34C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AF350: 48419409  bl 0x829c8758
	ctx.lr = 0x825AF354;
	sub_829C8758(ctx, base);
	// 825AF354: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF358: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825AF35C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AF360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AF364: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AF368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AF370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AF370 size=88
    let mut pc: u32 = 0x825AF370;
    'dispatch: loop {
        match pc {
            0x825AF370 => {
    //   block [0x825AF370..0x825AF3C8)
	// 825AF370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AF374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AF378: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AF37C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AF380: 38830008  addi r4, r3, 8
	ctx.r[4].s64 = ctx.r[3].s64 + 8;
	// 825AF384: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AF388: 484193D1  bl 0x829c8758
	ctx.lr = 0x825AF38C;
	sub_829C8758(ctx, base);
	// 825AF38C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AF390: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825AF394: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 825AF398: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AF39C: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 825AF3A0: 695F0001  xori r31, r10, 1
	ctx.r[31].u64 = ctx.r[10].u64 ^ 1;
	// 825AF3A4: 419A000C  beq cr6, 0x825af3b0
	if ctx.cr[6].eq {
	pc = 0x825AF3B0; continue 'dispatch;
	}
	// 825AF3A8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 825AF3AC: 4BD114E5  bl 0x822c0890
	ctx.lr = 0x825AF3B0;
	sub_822C0890(ctx, base);
	// 825AF3B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF3B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AF3B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AF3BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AF3C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AF3C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AF3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AF3C8 size=224
    let mut pc: u32 = 0x825AF3C8;
    'dispatch: loop {
        match pc {
            0x825AF3C8 => {
    //   block [0x825AF3C8..0x825AF4A8)
	// 825AF3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AF3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AF3D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AF3D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AF3D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AF3DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AF3E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AF3E4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825AF3E8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825AF3EC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825AF3F0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 825AF3F4: 997F0150  stb r11, 0x150(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[11].u8 ) };
	// 825AF3F8: 4BFFFEC9  bl 0x825af2c0
	ctx.lr = 0x825AF3FC;
	sub_825AF2C0(ctx, base);
	// 825AF3FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AF400: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 825AF404: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AF408: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AF40C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825AF410: 419A0024  beq cr6, 0x825af434
	if ctx.cr[6].eq {
	pc = 0x825AF434; continue 'dispatch;
	}
	// 825AF414: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825AF418: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825AF41C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AF420: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825AF424: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825AF428: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825AF42C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AF430: 4082FFE8  bne 0x825af418
	if !ctx.cr[0].eq {
	pc = 0x825AF418; continue 'dispatch;
	}
	// 825AF434: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AF438: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825AF43C: 4BF5FEF5  bl 0x8250f330
	ctx.lr = 0x825AF440;
	sub_8250F330(ctx, base);
	// 825AF440: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AF444: 3BC10058  addi r30, r1, 0x58
	ctx.r[30].s64 = ctx.r[1].s64 + 88;
	// 825AF448: 4BF3A419  bl 0x824e9860
	ctx.lr = 0x825AF44C;
	sub_824E9860(ctx, base);
	// 825AF44C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AF450: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 825AF454: 388BB830  addi r4, r11, -0x47d0
	ctx.r[4].s64 = ctx.r[11].s64 + -18384;
	// 825AF458: 38A000EC  li r5, 0xec
	ctx.r[5].s64 = 236;
	// 825AF45C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 825AF460: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 825AF464: 488A7B85  bl 0x82e56fe8
	ctx.lr = 0x825AF468;
	sub_82E56FE8(ctx, base);
	// 825AF468: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825AF46C: 48842825  bl 0x82df1c90
	ctx.lr = 0x825AF470;
	sub_82DF1C90(ctx, base);
	// 825AF470: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825AF474: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AF478: 419A0008  beq cr6, 0x825af480
	if ctx.cr[6].eq {
	pc = 0x825AF480; continue 'dispatch;
	}
	// 825AF47C: 4BD11415  bl 0x822c0890
	ctx.lr = 0x825AF480;
	sub_822C0890(ctx, base);
	// 825AF480: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 825AF484: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AF488: 419A0008  beq cr6, 0x825af490
	if ctx.cr[6].eq {
	pc = 0x825AF490; continue 'dispatch;
	}
	// 825AF48C: 4BD11405  bl 0x822c0890
	ctx.lr = 0x825AF490;
	sub_822C0890(ctx, base);
	// 825AF490: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825AF494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AF498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AF49C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AF4A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AF4A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AF4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825AF4A8 size=716
    let mut pc: u32 = 0x825AF4A8;
    'dispatch: loop {
        match pc {
            0x825AF4A8 => {
    //   block [0x825AF4A8..0x825AF774)
	// 825AF4A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AF4AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AF4B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AF4B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AF4B8: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 825AF4BC: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 825AF4C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AF4C4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825AF4C8: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 825AF4CC: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 825AF4D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AF4D4: C3EB08A8  lfs f31, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825AF4D8: 4BED1A49  bl 0x82480f20
	ctx.lr = 0x825AF4DC;
	sub_82480F20(ctx, base);
	// 825AF4DC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AF4E0: 4182000C  beq 0x825af4ec
	if ctx.cr[0].eq {
	pc = 0x825AF4EC; continue 'dispatch;
	}
	// 825AF4E4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 825AF4E8: C3EB89AC  lfs f31, -0x7654(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825AF4EC: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 825AF4F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF4F4: 4BED1A2D  bl 0x82480f20
	ctx.lr = 0x825AF4F8;
	sub_82480F20(ctx, base);
	// 825AF4F8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AF4FC: 4182000C  beq 0x825af508
	if ctx.cr[0].eq {
	pc = 0x825AF508; continue 'dispatch;
	}
	// 825AF500: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825AF504: C3EB964C  lfs f31, -0x69b4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825AF508: 817F01CC  lwz r11, 0x1cc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(460 as u32) ) } as u64;
	// 825AF50C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825AF510: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 825AF514: 4198011C  blt cr6, 0x825af630
	if ctx.cr[6].lt {
	pc = 0x825AF630; continue 'dispatch;
	}
	// 825AF518: 409A01E8  bne cr6, 0x825af700
	if !ctx.cr[6].eq {
	pc = 0x825AF700; continue 'dispatch;
	}
	// 825AF51C: 38800200  li r4, 0x200
	ctx.r[4].s64 = 512;
	// 825AF520: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF524: 4BFEC365  bl 0x8259b888
	ctx.lr = 0x825AF528;
	sub_8259B888(ctx, base);
	// 825AF528: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AF52C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF530: 41820010  beq 0x825af540
	if ctx.cr[0].eq {
	pc = 0x825AF540; continue 'dispatch;
	}
	// 825AF534: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825AF538: 4BFFFB19  bl 0x825af050
	ctx.lr = 0x825AF53C;
	sub_825AF050(ctx, base);
	// 825AF53C: 48000020  b 0x825af55c
	pc = 0x825AF55C; continue 'dispatch;
	// 825AF540: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 825AF544: 4BFEC345  bl 0x8259b888
	ctx.lr = 0x825AF548;
	sub_8259B888(ctx, base);
	// 825AF548: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AF54C: 41820010  beq 0x825af55c
	if ctx.cr[0].eq {
	pc = 0x825AF55C; continue 'dispatch;
	}
	// 825AF550: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF554: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825AF558: 4BFFFBA9  bl 0x825af100
	ctx.lr = 0x825AF55C;
	sub_825AF100(ctx, base);
	// 825AF55C: 38800200  li r4, 0x200
	ctx.r[4].s64 = 512;
	// 825AF560: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF564: 4BED19BD  bl 0x82480f20
	ctx.lr = 0x825AF568;
	sub_82480F20(ctx, base);
	// 825AF568: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AF56C: 41820030  beq 0x825af59c
	if ctx.cr[0].eq {
	pc = 0x825AF59C; continue 'dispatch;
	}
	// 825AF570: C01F014C  lfs f0, 0x14c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825AF574: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825AF578: EDA0F02A  fadds f13, f0, f30
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[30].f64) as f32) as f64;
	// 825AF57C: D1BF014C  stfs f13, 0x14c(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(332 as u32), tmp.u32 ) };
	// 825AF580: C00B9450  lfs f0, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825AF584: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 825AF588: 40990060  ble cr6, 0x825af5e8
	if !ctx.cr[6].gt {
	pc = 0x825AF5E8; continue 'dispatch;
	}
	// 825AF58C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF590: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825AF594: 4BFFFABD  bl 0x825af050
	ctx.lr = 0x825AF598;
	sub_825AF050(ctx, base);
	// 825AF598: 48000050  b 0x825af5e8
	pc = 0x825AF5E8; continue 'dispatch;
	// 825AF59C: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 825AF5A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF5A4: 4BED197D  bl 0x82480f20
	ctx.lr = 0x825AF5A8;
	sub_82480F20(ctx, base);
	// 825AF5A8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AF5AC: 41820030  beq 0x825af5dc
	if ctx.cr[0].eq {
	pc = 0x825AF5DC; continue 'dispatch;
	}
	// 825AF5B0: C01F014C  lfs f0, 0x14c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825AF5B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825AF5B8: EDA0F02A  fadds f13, f0, f30
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[30].f64) as f32) as f64;
	// 825AF5BC: D1BF014C  stfs f13, 0x14c(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(332 as u32), tmp.u32 ) };
	// 825AF5C0: C00B9450  lfs f0, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825AF5C4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 825AF5C8: 40990020  ble cr6, 0x825af5e8
	if !ctx.cr[6].gt {
	pc = 0x825AF5E8; continue 'dispatch;
	}
	// 825AF5CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF5D0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825AF5D4: 4BFFFB2D  bl 0x825af100
	ctx.lr = 0x825AF5D8;
	sub_825AF100(ctx, base);
	// 825AF5D8: 48000010  b 0x825af5e8
	pc = 0x825AF5E8; continue 'dispatch;
	// 825AF5DC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825AF5E0: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825AF5E4: D01F014C  stfs f0, 0x14c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(332 as u32), tmp.u32 ) };
	// 825AF5E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825AF5EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF5F0: 4BFEC299  bl 0x8259b888
	ctx.lr = 0x825AF5F4;
	sub_8259B888(ctx, base);
	// 825AF5F4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AF5F8: 41820008  beq 0x825af600
	if ctx.cr[0].eq {
	pc = 0x825AF600; continue 'dispatch;
	}
	// 825AF5FC: 93DF01CC  stw r30, 0x1cc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(460 as u32), ctx.r[30].u32 ) };
	// 825AF600: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 825AF604: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF608: 4BFEC281  bl 0x8259b888
	ctx.lr = 0x825AF60C;
	sub_8259B888(ctx, base);
	// 825AF60C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AF610: 418200F0  beq 0x825af700
	if ctx.cr[0].eq {
	pc = 0x825AF700; continue 'dispatch;
	}
	// 825AF614: 93DF01CC  stw r30, 0x1cc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(460 as u32), ctx.r[30].u32 ) };
	// 825AF618: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF61C: 4BFFF9AD  bl 0x825aefc8
	ctx.lr = 0x825AF620;
	sub_825AEFC8(ctx, base);
	// 825AF620: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 825AF624: 388BFFFE  addi r4, r11, -2
	ctx.r[4].s64 = ctx.r[11].s64 + -2;
	// 825AF628: 4BFF3101  bl 0x825a2728
	ctx.lr = 0x825AF62C;
	sub_825A2728(ctx, base);
	// 825AF62C: 480000D4  b 0x825af700
	pc = 0x825AF700; continue 'dispatch;
	// 825AF630: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 825AF634: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF638: 4BFEC251  bl 0x8259b888
	ctx.lr = 0x825AF63C;
	sub_8259B888(ctx, base);
	// 825AF63C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AF640: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF644: 4182000C  beq 0x825af650
	if ctx.cr[0].eq {
	pc = 0x825AF650; continue 'dispatch;
	}
	// 825AF648: 4BFFFB51  bl 0x825af198
	ctx.lr = 0x825AF64C;
	sub_825AF198(ctx, base);
	// 825AF64C: 480000B4  b 0x825af700
	pc = 0x825AF700; continue 'dispatch;
	// 825AF650: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 825AF654: 4BFEC235  bl 0x8259b888
	ctx.lr = 0x825AF658;
	sub_8259B888(ctx, base);
	// 825AF658: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AF65C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF660: 4182000C  beq 0x825af66c
	if ctx.cr[0].eq {
	pc = 0x825AF66C; continue 'dispatch;
	}
	// 825AF664: 4BFFFB8D  bl 0x825af1f0
	ctx.lr = 0x825AF668;
	sub_825AF1F0(ctx, base);
	// 825AF668: 48000098  b 0x825af700
	pc = 0x825AF700; continue 'dispatch;
	// 825AF66C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825AF670: 4BFEC219  bl 0x8259b888
	ctx.lr = 0x825AF674;
	sub_8259B888(ctx, base);
	// 825AF674: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AF678: 41820048  beq 0x825af6c0
	if ctx.cr[0].eq {
	pc = 0x825AF6C0; continue 'dispatch;
	}
	// 825AF67C: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 825AF680: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 825AF684: 409A0014  bne cr6, 0x825af698
	if !ctx.cr[6].eq {
	pc = 0x825AF698; continue 'dispatch;
	}
	// 825AF688: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF68C: 4BFFF93D  bl 0x825aefc8
	ctx.lr = 0x825AF690;
	sub_825AEFC8(ctx, base);
	// 825AF690: 4BFF2FA9  bl 0x825a2638
	ctx.lr = 0x825AF694;
	sub_825A2638(ctx, base);
	// 825AF694: 4800006C  b 0x825af700
	pc = 0x825AF700; continue 'dispatch;
	// 825AF698: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 825AF69C: 41980064  blt cr6, 0x825af700
	if ctx.cr[6].lt {
	pc = 0x825AF700; continue 'dispatch;
	}
	// 825AF6A0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825AF6A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF6A8: 917F01CC  stw r11, 0x1cc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(460 as u32), ctx.r[11].u32 ) };
	// 825AF6AC: 4BFFF91D  bl 0x825aefc8
	ctx.lr = 0x825AF6B0;
	sub_825AEFC8(ctx, base);
	// 825AF6B0: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 825AF6B4: 388BFFFE  addi r4, r11, -2
	ctx.r[4].s64 = ctx.r[11].s64 + -2;
	// 825AF6B8: 4BFF3051  bl 0x825a2708
	ctx.lr = 0x825AF6BC;
	sub_825A2708(ctx, base);
	// 825AF6BC: 48000044  b 0x825af700
	pc = 0x825AF700; continue 'dispatch;
	// 825AF6C0: 38800200  li r4, 0x200
	ctx.r[4].s64 = 512;
	// 825AF6C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF6C8: 4BFEC1C1  bl 0x8259b888
	ctx.lr = 0x825AF6CC;
	sub_8259B888(ctx, base);
	// 825AF6CC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AF6D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF6D4: 41820010  beq 0x825af6e4
	if ctx.cr[0].eq {
	pc = 0x825AF6E4; continue 'dispatch;
	}
	// 825AF6D8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825AF6DC: 4BFFF975  bl 0x825af050
	ctx.lr = 0x825AF6E0;
	sub_825AF050(ctx, base);
	// 825AF6E0: 48000020  b 0x825af700
	pc = 0x825AF700; continue 'dispatch;
	// 825AF6E4: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 825AF6E8: 4BFEC1A1  bl 0x8259b888
	ctx.lr = 0x825AF6EC;
	sub_8259B888(ctx, base);
	// 825AF6EC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AF6F0: 41820010  beq 0x825af700
	if ctx.cr[0].eq {
	pc = 0x825AF700; continue 'dispatch;
	}
	// 825AF6F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF6F8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825AF6FC: 4BFFFA05  bl 0x825af100
	ctx.lr = 0x825AF700;
	sub_825AF100(ctx, base);
	// 825AF700: 38800800  li r4, 0x800
	ctx.r[4].s64 = 2048;
	// 825AF704: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF708: 4BFEC181  bl 0x8259b888
	ctx.lr = 0x825AF70C;
	sub_8259B888(ctx, base);
	// 825AF70C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AF710: 4182001C  beq 0x825af72c
	if ctx.cr[0].eq {
	pc = 0x825AF72C; continue 'dispatch;
	}
	// 825AF714: 817F0154  lwz r11, 0x154(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 825AF718: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825AF71C: 917F0154  stw r11, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[11].u32 ) };
	// 825AF720: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 825AF724: 41980008  blt cr6, 0x825af72c
	if ctx.cr[6].lt {
	pc = 0x825AF72C; continue 'dispatch;
	}
	// 825AF728: 93DF0154  stw r30, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[30].u32 ) };
	// 825AF72C: 3C800001  lis r4, 1
	ctx.r[4].s64 = 65536;
	// 825AF730: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF734: 4BFEC155  bl 0x8259b888
	ctx.lr = 0x825AF738;
	sub_8259B888(ctx, base);
	// 825AF738: 3C800002  lis r4, 2
	ctx.r[4].s64 = 131072;
	// 825AF73C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF740: 4BFEC149  bl 0x8259b888
	ctx.lr = 0x825AF744;
	sub_8259B888(ctx, base);
	// 825AF744: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AF748: 4182000C  beq 0x825af754
	if ctx.cr[0].eq {
	pc = 0x825AF754; continue 'dispatch;
	}
	// 825AF74C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF750: 4BFFFC79  bl 0x825af3c8
	ctx.lr = 0x825AF754;
	sub_825AF3C8(ctx, base);
	// 825AF754: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825AF758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AF75C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AF760: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 825AF764: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 825AF768: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AF76C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AF770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AF778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AF778 size=216
    let mut pc: u32 = 0x825AF778;
    'dispatch: loop {
        match pc {
            0x825AF778 => {
    //   block [0x825AF778..0x825AF850)
	// 825AF778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AF77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AF780: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AF784: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AF788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AF78C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AF790: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825AF794: 389F012C  addi r4, r31, 0x12c
	ctx.r[4].s64 = ctx.r[31].s64 + 300;
	// 825AF798: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AF79C: 4824702D  bl 0x827f67c8
	ctx.lr = 0x825AF7A0;
	sub_827F67C8(ctx, base);
	// 825AF7A0: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 825AF7A4: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AF7A8: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825AF7AC: 419A0088  beq cr6, 0x825af834
	if ctx.cr[6].eq {
	pc = 0x825AF834; continue 'dispatch;
	}
	// 825AF7B0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AF7B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AF7B8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AF7BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AF7C0: 4E800421  bctrl
	ctx.lr = 0x825AF7C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AF7C4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AF7C8: 41820064  beq 0x825af82c
	if ctx.cr[0].eq {
	pc = 0x825AF82C; continue 'dispatch;
	}
	// 825AF7CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AF7D0: 4802A5C1  bl 0x825d9d90
	ctx.lr = 0x825AF7D4;
	sub_825D9D90(ctx, base);
	// 825AF7D4: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AF7D8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825AF7DC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AF7E0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AF7E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AF7E8: 4E800421  bctrl
	ctx.lr = 0x825AF7EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AF7EC: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825AF7F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AF7F4: 419A000C  beq cr6, 0x825af800
	if ctx.cr[6].eq {
	pc = 0x825AF800; continue 'dispatch;
	}
	// 825AF7F8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825AF7FC: 4BFF2DCD  bl 0x825a25c8
	ctx.lr = 0x825AF800;
	sub_825A25C8(ctx, base);
	// 825AF800: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825AF804: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AF808: 4802A601  bl 0x825d9e08
	ctx.lr = 0x825AF80C;
	sub_825D9E08(ctx, base);
	// 825AF80C: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825AF810: 4BFF2E91  bl 0x825a26a0
	ctx.lr = 0x825AF814;
	sub_825A26A0(ctx, base);
	// 825AF814: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825AF818: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AF81C: 419A0008  beq cr6, 0x825af824
	if ctx.cr[6].eq {
	pc = 0x825AF824; continue 'dispatch;
	}
	// 825AF820: 4BD11071  bl 0x822c0890
	ctx.lr = 0x825AF824;
	sub_822C0890(ctx, base);
	// 825AF824: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AF828: 4802A5A1  bl 0x825d9dc8
	ctx.lr = 0x825AF82C;
	sub_825D9DC8(ctx, base);
	// 825AF82C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825AF830: 48000008  b 0x825af838
	pc = 0x825AF838; continue 'dispatch;
	// 825AF834: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825AF838: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825AF83C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AF840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AF844: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AF848: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AF84C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AF850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825AF850 size=392
    let mut pc: u32 = 0x825AF850;
    'dispatch: loop {
        match pc {
            0x825AF850 => {
    //   block [0x825AF850..0x825AF9D8)
	// 825AF850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AF854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AF858: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AF85C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AF860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AF864: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825AF868: 48BA4D99  bl 0x83154600
	ctx.lr = 0x825AF86C;
	sub_83154600(ctx, base);
	// 825AF86C: 38800800  li r4, 0x800
	ctx.r[4].s64 = 2048;
	// 825AF870: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AF874: 4BED16AD  bl 0x82480f20
	ctx.lr = 0x825AF878;
	sub_82480F20(ctx, base);
	// 825AF878: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AF87C: 41820018  beq 0x825af894
	if ctx.cr[0].eq {
	pc = 0x825AF894; continue 'dispatch;
	}
	// 825AF880: 38800400  li r4, 0x400
	ctx.r[4].s64 = 1024;
	// 825AF884: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF888: 4BFEC001  bl 0x8259b888
	ctx.lr = 0x825AF88C;
	sub_8259B888(ctx, base);
	// 825AF88C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AF890: 4082002C  bne 0x825af8bc
	if !ctx.cr[0].eq {
	pc = 0x825AF8BC; continue 'dispatch;
	}
	// 825AF894: 38800400  li r4, 0x400
	ctx.r[4].s64 = 1024;
	// 825AF898: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF89C: 4BED1685  bl 0x82480f20
	ctx.lr = 0x825AF8A0;
	sub_82480F20(ctx, base);
	// 825AF8A0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AF8A4: 41820100  beq 0x825af9a4
	if ctx.cr[0].eq {
	pc = 0x825AF9A4; continue 'dispatch;
	}
	// 825AF8A8: 38800800  li r4, 0x800
	ctx.r[4].s64 = 2048;
	// 825AF8AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF8B0: 4BFEBFD9  bl 0x8259b888
	ctx.lr = 0x825AF8B4;
	sub_8259B888(ctx, base);
	// 825AF8B4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AF8B8: 418200EC  beq 0x825af9a4
	if ctx.cr[0].eq {
	pc = 0x825AF9A4; continue 'dispatch;
	}
	// 825AF8BC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AF8C0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825AF8C4: 388BB830  addi r4, r11, -0x47d0
	ctx.r[4].s64 = ctx.r[11].s64 + -18384;
	// 825AF8C8: 38A00415  li r5, 0x415
	ctx.r[5].s64 = 1045;
	// 825AF8CC: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 825AF8D0: 48842B19  bl 0x82df23e8
	ctx.lr = 0x825AF8D4;
	sub_82DF23E8(ctx, base);
	// 825AF8D4: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825AF8D8: 4182001C  beq 0x825af8f4
	if ctx.cr[0].eq {
	pc = 0x825AF8F4; continue 'dispatch;
	}
	// 825AF8DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF8E0: 488AD049  bl 0x82e5c928
	ctx.lr = 0x825AF8E4;
	sub_82E5C928(ctx, base);
	// 825AF8E4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AF8E8: 396BB7C8  addi r11, r11, -0x4838
	ctx.r[11].s64 = ctx.r[11].s64 + -18488;
	// 825AF8EC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AF8F0: 48000008  b 0x825af8f8
	pc = 0x825AF8F8; continue 'dispatch;
	// 825AF8F4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825AF8F8: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 825AF8FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AF900: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825AF904: 4BFFF09D  bl 0x825ae9a0
	ctx.lr = 0x825AF908;
	sub_825AE9A0(ctx, base);
	// 825AF908: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825AF90C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AF910: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825AF914: 4BD106ED  bl 0x822c0000
	ctx.lr = 0x825AF918;
	sub_822C0000(ctx, base);
	// 825AF918: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825AF91C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AF920: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825AF924: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 825AF928: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 825AF92C: 419A0024  beq cr6, 0x825af950
	if ctx.cr[6].eq {
	pc = 0x825AF950; continue 'dispatch;
	}
	// 825AF930: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 825AF934: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825AF938: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AF93C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825AF940: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825AF944: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825AF948: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AF94C: 4082FFE8  bne 0x825af934
	if !ctx.cr[0].eq {
	pc = 0x825AF934; continue 'dispatch;
	}
	// 825AF950: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825AF954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825AF958: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825AF95C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 825AF960: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825AF964: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825AF968: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825AF96C: 488AEFA5  bl 0x82e5e910
	ctx.lr = 0x825AF970;
	sub_82E5E910(ctx, base);
	// 825AF970: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 825AF974: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AF978: 419A0008  beq cr6, 0x825af980
	if ctx.cr[6].eq {
	pc = 0x825AF980; continue 'dispatch;
	}
	// 825AF97C: 4BD10F15  bl 0x822c0890
	ctx.lr = 0x825AF980;
	sub_822C0890(ctx, base);
	// 825AF980: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825AF984: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AF988: 419A0008  beq cr6, 0x825af990
	if ctx.cr[6].eq {
	pc = 0x825AF990; continue 'dispatch;
	}
	// 825AF98C: 4BD10F05  bl 0x822c0890
	ctx.lr = 0x825AF990;
	sub_822C0890(ctx, base);
	// 825AF990: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825AF994: 419A002C  beq cr6, 0x825af9c0
	if ctx.cr[6].eq {
	pc = 0x825AF9C0; continue 'dispatch;
	}
	// 825AF998: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF99C: 4BD10EF5  bl 0x822c0890
	ctx.lr = 0x825AF9A0;
	sub_822C0890(ctx, base);
	// 825AF9A0: 48000020  b 0x825af9c0
	pc = 0x825AF9C0; continue 'dispatch;
	// 825AF9A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AF9A8: 48BA4C59  bl 0x83154600
	ctx.lr = 0x825AF9AC;
	sub_83154600(ctx, base);
	// 825AF9AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AF9B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AF9B4: 488A9DC5  bl 0x82e59778
	ctx.lr = 0x825AF9B8;
	sub_82E59778(ctx, base);
	// 825AF9B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AF9BC: 4BFFFAED  bl 0x825af4a8
	ctx.lr = 0x825AF9C0;
	sub_825AF4A8(ctx, base);
	// 825AF9C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825AF9C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AF9C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AF9CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AF9D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AF9D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AF9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AF9D8 size=184
    let mut pc: u32 = 0x825AF9D8;
    'dispatch: loop {
        match pc {
            0x825AF9D8 => {
    //   block [0x825AF9D8..0x825AFA90)
	// 825AF9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AF9DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825AF9E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825AF9E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825AF9E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AF9EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825AF9F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AF9F4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825AF9F8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825AF9FC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AFA00: 4BD10F39  bl 0x822c0938
	ctx.lr = 0x825AFA04;
	sub_822C0938(ctx, base);
	// 825AFA04: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825AFA08: 41820028  beq 0x825afa30
	if ctx.cr[0].eq {
	pc = 0x825AFA30; continue 'dispatch;
	}
	// 825AFA0C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AFA10: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825AFA14: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825AFA18: 392BB764  addi r9, r11, -0x489c
	ctx.r[9].s64 = ctx.r[11].s64 + -18588;
	// 825AFA1C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825AFA20: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825AFA24: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825AFA28: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825AFA2C: 48000008  b 0x825afa34
	pc = 0x825AFA34; continue 'dispatch;
	// 825AFA30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825AFA34: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AFA38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AFA3C: 409A0038  bne cr6, 0x825afa74
	if !ctx.cr[6].eq {
	pc = 0x825AFA74; continue 'dispatch;
	}
	// 825AFA40: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825AFA44: 419A0010  beq cr6, 0x825afa54
	if ctx.cr[6].eq {
	pc = 0x825AFA54; continue 'dispatch;
	}
	// 825AFA48: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825AFA4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AFA50: 4BFFF1A9  bl 0x825aebf8
	ctx.lr = 0x825AFA54;
	sub_825AEBF8(ctx, base);
	// 825AFA54: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825AFA58: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825AFA5C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AFA60: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825AFA64: 816B856C  lwz r11, -0x7a94(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31380 as u32) ) } as u64;
	// 825AFA68: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825AFA6C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825AFA70: 4BD10591  bl 0x822c0000
	ctx.lr = 0x825AFA74;
	sub_822C0000(ctx, base);
	// 825AFA74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825AFA78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825AFA7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825AFA80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825AFA84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825AFA88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825AFA8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AFA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AFA90 size=12
    let mut pc: u32 = 0x825AFA90;
    'dispatch: loop {
        match pc {
            0x825AFA90 => {
    //   block [0x825AFA90..0x825AFA9C)
	// 825AFA90: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825AFA94: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AFA98: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AFA9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AFA9C size=8
    let mut pc: u32 = 0x825AFA9C;
    'dispatch: loop {
        match pc {
            0x825AFA9C => {
    //   block [0x825AFA9C..0x825AFAA4)
	// 825AFA9C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825AFAA0: 4BFFF158  b 0x825aebf8
	sub_825AEBF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AFAA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825AFAA4 size=4
    let mut pc: u32 = 0x825AFAA4;
    'dispatch: loop {
        match pc {
            0x825AFAA4 => {
    //   block [0x825AFAA4..0x825AFAA8)
	// 825AFAA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AFAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AFAA8 size=1256
    let mut pc: u32 = 0x825AFAA8;
    'dispatch: loop {
        match pc {
            0x825AFAA8 => {
    //   block [0x825AFAA8..0x825AFF90)
	// 825AFAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AFAAC: 48BF86AD  bl 0x831a8158
	ctx.lr = 0x825AFAB0;
	sub_831A8130(ctx, base);
	// 825AFAB0: 9421FE50  stwu r1, -0x1b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-432 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AFAB4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825AFAB8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825AFABC: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 825AFAC0: 806B6FF4  lwz r3, 0x6ff4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28660 as u32) ) } as u64;
	// 825AFAC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AFAC8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AFACC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AFAD0: 4E800421  bctrl
	ctx.lr = 0x825AFAD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AFAD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AFAD8: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 825AFADC: 93E10070  stw r31, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 825AFAE0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AFAE4: 4BF4FB0D  bl 0x824ff5f0
	ctx.lr = 0x825AFAE8;
	sub_824FF5F0(ctx, base);
	// 825AFAE8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825AFAEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AFAF0: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 825AFAF4: 4BD1050D  bl 0x822c0000
	ctx.lr = 0x825AFAF8;
	sub_822C0000(ctx, base);
	// 825AFAF8: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 825AFAFC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 825AFB00: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 825AFB04: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AFB08: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AFB0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AFB10: 4E800421  bctrl
	ctx.lr = 0x825AFB14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AFB14: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825AFB18: 480293E1  bl 0x825d8ef8
	ctx.lr = 0x825AFB1C;
	sub_825D8EF8(ctx, base);
	// 825AFB1C: 3BDC0160  addi r30, r28, 0x160
	ctx.r[30].s64 = ctx.r[28].s64 + 352;
	// 825AFB20: 833C0164  lwz r25, 0x164(r28)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(356 as u32) ) } as u64;
	// 825AFB24: 837C0160  lwz r27, 0x160(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(352 as u32) ) } as u64;
	// 825AFB28: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 825AFB2C: 419A0024  beq cr6, 0x825afb50
	if ctx.cr[6].eq {
	pc = 0x825AFB50; continue 'dispatch;
	}
	// 825AFB30: 39790004  addi r11, r25, 4
	ctx.r[11].s64 = ctx.r[25].s64 + 4;
	// 825AFB34: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825AFB38: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AFB3C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825AFB40: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825AFB44: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825AFB48: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AFB4C: 4082FFE8  bne 0x825afb34
	if !ctx.cr[0].eq {
	pc = 0x825AFB34; continue 'dispatch;
	}
	// 825AFB50: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825AFB54: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 825AFB58: 38ABB87C  addi r5, r11, -0x4784
	ctx.r[5].s64 = ctx.r[11].s64 + -18308;
	// 825AFB5C: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 825AFB60: 48029499  bl 0x825d8ff8
	ctx.lr = 0x825AFB64;
	sub_825D8FF8(ctx, base);
	// 825AFB64: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825AFB68: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 825AFB6C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 825AFB70: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AFB74: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AFB78: 4BD148E9  bl 0x822c4460
	ctx.lr = 0x825AFB7C;
	sub_822C4460(ctx, base);
	// 825AFB7C: 806100A4  lwz r3, 0xa4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 825AFB80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AFB84: 419A0008  beq cr6, 0x825afb8c
	if ctx.cr[6].eq {
	pc = 0x825AFB8C; continue 'dispatch;
	}
	// 825AFB88: 4BD10D09  bl 0x822c0890
	ctx.lr = 0x825AFB8C;
	sub_822C0890(ctx, base);
	// 825AFB8C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 825AFB90: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AFB94: 48028C75  bl 0x825d8808
	ctx.lr = 0x825AFB98;
	sub_825D8808(ctx, base);
	// 825AFB98: 83BC0130  lwz r29, 0x130(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(304 as u32) ) } as u64;
	// 825AFB9C: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AFBA0: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 825AFBA4: 4800008C  b 0x825afc30
	pc = 0x825AFC30; continue 'dispatch;
	// 825AFBA8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AFBAC: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AFBB0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825AFBB4: 41820070  beq 0x825afc24
	if ctx.cr[0].eq {
	pc = 0x825AFC24; continue 'dispatch;
	}
	// 825AFBB8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 825AFBBC: 83010078  lwz r24, 0x78(r1)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 825AFBC0: 488435F1  bl 0x82df31b0
	ctx.lr = 0x825AFBC4;
	sub_82DF31B0(ctx, base);
	// 825AFBC4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 825AFBC8: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 825AFBCC: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 825AFBD0: 48029BD9  bl 0x825d97a8
	ctx.lr = 0x825AFBD4;
	sub_825D97A8(ctx, base);
	// 825AFBD4: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825AFBD8: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 825AFBDC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AFBE0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AFBE4: 83E10088  lwz r31, 0x88(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 825AFBE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825AFBEC: 4E800421  bctrl
	ctx.lr = 0x825AFBF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825AFBF0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825AFBF4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825AFBF8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825AFBFC: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AFC00: 4BFFEA59  bl 0x825ae658
	ctx.lr = 0x825AFC04;
	sub_825AE658(ctx, base);
	// 825AFC04: 806100CC  lwz r3, 0xcc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(204 as u32) ) } as u64;
	// 825AFC08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AFC0C: 419A0008  beq cr6, 0x825afc14
	if ctx.cr[6].eq {
	pc = 0x825AFC14; continue 'dispatch;
	}
	// 825AFC10: 4BD10C81  bl 0x822c0890
	ctx.lr = 0x825AFC14;
	sub_822C0890(ctx, base);
	// 825AFC14: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 825AFC18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AFC1C: 419A0008  beq cr6, 0x825afc24
	if ctx.cr[6].eq {
	pc = 0x825AFC24; continue 'dispatch;
	}
	// 825AFC20: 4BD10C71  bl 0x822c0890
	ctx.lr = 0x825AFC24;
	sub_822C0890(ctx, base);
	// 825AFC24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AFC28: 4BDF1A61  bl 0x823a1688
	ctx.lr = 0x825AFC2C;
	sub_823A1688(ctx, base);
	// 825AFC2C: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825AFC30: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 825AFC34: 409AFF74  bne cr6, 0x825afba8
	if !ctx.cr[6].eq {
	pc = 0x825AFBA8; continue 'dispatch;
	}
	// 825AFC38: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 825AFC3C: 419A0148  beq cr6, 0x825afd84
	if ctx.cr[6].eq {
	pc = 0x825AFD84; continue 'dispatch;
	}
	// 825AFC40: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 825AFC44: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 825AFC48: 48028BC1  bl 0x825d8808
	ctx.lr = 0x825AFC4C;
	sub_825D8808(ctx, base);
	// 825AFC4C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825AFC50: 80810098  lwz r4, 0x98(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 825AFC54: 48029D4D  bl 0x825d99a0
	ctx.lr = 0x825AFC58;
	sub_825D99A0(ctx, base);
	// 825AFC58: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825AFC5C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 825AFC60: 419A0104  beq cr6, 0x825afd64
	if ctx.cr[6].eq {
	pc = 0x825AFD64; continue 'dispatch;
	}
	// 825AFC64: 3BFC012C  addi r31, r28, 0x12c
	ctx.r[31].s64 = ctx.r[28].s64 + 300;
	// 825AFC68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AFC6C: 480298FD  bl 0x825d9568
	ctx.lr = 0x825AFC70;
	sub_825D9568(ctx, base);
	// 825AFC70: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 825AFC74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825AFC78: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 825AFC7C: 48246B4D  bl 0x827f67c8
	ctx.lr = 0x825AFC80;
	sub_827F67C8(ctx, base);
	// 825AFC80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AFC84: 488437A5  bl 0x82df3428
	ctx.lr = 0x825AFC88;
	sub_82DF3428(ctx, base);
	// 825AFC88: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 825AFC8C: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825AFC90: 83A10078  lwz r29, 0x78(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 825AFC94: 480298D5  bl 0x825d9568
	ctx.lr = 0x825AFC98;
	sub_825D9568(ctx, base);
	// 825AFC98: 48843519  bl 0x82df31b0
	ctx.lr = 0x825AFC9C;
	sub_82DF31B0(ctx, base);
	// 825AFC9C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 825AFCA0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825AFCA4: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 825AFCA8: 48029F29  bl 0x825d9bd0
	ctx.lr = 0x825AFCAC;
	sub_825D9BD0(ctx, base);
	// 825AFCAC: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 825AFCB0: 48843779  bl 0x82df3428
	ctx.lr = 0x825AFCB4;
	sub_82DF3428(ctx, base);
	// 825AFCB4: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 825AFCB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AFCBC: 409A0058  bne cr6, 0x825afd14
	if !ctx.cr[6].eq {
	pc = 0x825AFD14; continue 'dispatch;
	}
	// 825AFCC0: 817C0130  lwz r11, 0x130(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(304 as u32) ) } as u64;
	// 825AFCC4: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 825AFCC8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825AFCCC: 409A0048  bne cr6, 0x825afd14
	if !ctx.cr[6].eq {
	pc = 0x825AFD14; continue 'dispatch;
	}
	// 825AFCD0: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825AFCD4: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825AFCD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AFCDC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825AFCE0: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 825AFCE4: 419A0024  beq cr6, 0x825afd08
	if ctx.cr[6].eq {
	pc = 0x825AFD08; continue 'dispatch;
	}
	// 825AFCE8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825AFCEC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825AFCF0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AFCF4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825AFCF8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825AFCFC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825AFD00: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AFD04: 4082FFE8  bne 0x825afcec
	if !ctx.cr[0].eq {
	pc = 0x825AFCEC; continue 'dispatch;
	}
	// 825AFD08: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 825AFD0C: 80610078  lwz r3, 0x78(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 825AFD10: 48029699  bl 0x825d93a8
	ctx.lr = 0x825AFD14;
	sub_825D93A8(ctx, base);
	// 825AFD14: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 825AFD18: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825AFD1C: 48029D9D  bl 0x825d9ab8
	ctx.lr = 0x825AFD20;
	sub_825D9AB8(ctx, base);
	// 825AFD20: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825AFD24: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825AFD28: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 825AFD2C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AFD30: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 825AFD34: 4BD1472D  bl 0x822c4460
	ctx.lr = 0x825AFD38;
	sub_822C4460(ctx, base);
	// 825AFD38: 806100BC  lwz r3, 0xbc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(188 as u32) ) } as u64;
	// 825AFD3C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AFD40: 419A0008  beq cr6, 0x825afd48
	if ctx.cr[6].eq {
	pc = 0x825AFD48; continue 'dispatch;
	}
	// 825AFD44: 4BD10B4D  bl 0x822c0890
	ctx.lr = 0x825AFD48;
	sub_822C0890(ctx, base);
	// 825AFD48: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 825AFD4C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AFD50: 419A0008  beq cr6, 0x825afd58
	if ctx.cr[6].eq {
	pc = 0x825AFD58; continue 'dispatch;
	}
	// 825AFD54: 4BD10B3D  bl 0x822c0890
	ctx.lr = 0x825AFD58;
	sub_822C0890(ctx, base);
	// 825AFD58: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825AFD5C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 825AFD60: 409AFF08  bne cr6, 0x825afc68
	if !ctx.cr[6].eq {
	pc = 0x825AFC68; continue 'dispatch;
	}
	// 825AFD64: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825AFD68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AFD6C: 419A0008  beq cr6, 0x825afd74
	if ctx.cr[6].eq {
	pc = 0x825AFD74; continue 'dispatch;
	}
	// 825AFD70: 4BD10B21  bl 0x822c0890
	ctx.lr = 0x825AFD74;
	sub_822C0890(ctx, base);
	// 825AFD74: 8061009C  lwz r3, 0x9c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 825AFD78: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AFD7C: 419A0008  beq cr6, 0x825afd84
	if ctx.cr[6].eq {
	pc = 0x825AFD84; continue 'dispatch;
	}
	// 825AFD80: 4BD10B11  bl 0x822c0890
	ctx.lr = 0x825AFD84;
	sub_822C0890(ctx, base);
	// 825AFD84: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 825AFD88: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825AFD8C: 48844CDD  bl 0x82df4a68
	ctx.lr = 0x825AFD90;
	sub_82DF4A68(ctx, base);
	// 825AFD90: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825AFD94: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AFD98: 48843419  bl 0x82df31b0
	ctx.lr = 0x825AFD9C;
	sub_82DF31B0(ctx, base);
	// 825AFD9C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AFDA0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825AFDA4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825AFDA8: 48028AF9  bl 0x825d88a0
	ctx.lr = 0x825AFDAC;
	sub_825D88A0(ctx, base);
	// 825AFDAC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825AFDB0: 4082004C  bne 0x825afdfc
	if !ctx.cr[0].eq {
	pc = 0x825AFDFC; continue 'dispatch;
	}
	// 825AFDB4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825AFDB8: 48843671  bl 0x82df3428
	ctx.lr = 0x825AFDBC;
	sub_82DF3428(ctx, base);
	// 825AFDBC: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 825AFDC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AFDC4: 419A0008  beq cr6, 0x825afdcc
	if ctx.cr[6].eq {
	pc = 0x825AFDCC; continue 'dispatch;
	}
	// 825AFDC8: 4BD10AC9  bl 0x822c0890
	ctx.lr = 0x825AFDCC;
	sub_822C0890(ctx, base);
	// 825AFDCC: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 825AFDD0: 419A000C  beq cr6, 0x825afddc
	if ctx.cr[6].eq {
	pc = 0x825AFDDC; continue 'dispatch;
	}
	// 825AFDD4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 825AFDD8: 4BD10AB9  bl 0x822c0890
	ctx.lr = 0x825AFDDC;
	sub_822C0890(ctx, base);
	// 825AFDDC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825AFDE0: 480291D1  bl 0x825d8fb0
	ctx.lr = 0x825AFDE4;
	sub_825D8FB0(ctx, base);
	// 825AFDE4: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 825AFDE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AFDEC: 419A0008  beq cr6, 0x825afdf4
	if ctx.cr[6].eq {
	pc = 0x825AFDF4; continue 'dispatch;
	}
	// 825AFDF0: 4BD10AA1  bl 0x822c0890
	ctx.lr = 0x825AFDF4;
	sub_822C0890(ctx, base);
	// 825AFDF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825AFDF8: 48000190  b 0x825aff88
	pc = 0x825AFF88; continue 'dispatch;
	// 825AFDFC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825AFE00: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 825AFE04: 4BF5F52D  bl 0x8250f330
	ctx.lr = 0x825AFE08;
	sub_8250F330(ctx, base);
	// 825AFE08: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AFE0C: 4BF39965  bl 0x824e9770
	ctx.lr = 0x825AFE10;
	sub_824E9770(ctx, base);
	// 825AFE10: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825AFE14: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 825AFE18: 48841E79  bl 0x82df1c90
	ctx.lr = 0x825AFE1C;
	sub_82DF1C90(ctx, base);
	// 825AFE1C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 825AFE20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AFE24: 48843DDD  bl 0x82df3c00
	ctx.lr = 0x825AFE28;
	sub_82DF3C00(ctx, base);
	// 825AFE28: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825AFE2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AFE30: 388B043C  addi r4, r11, 0x43c
	ctx.r[4].s64 = ctx.r[11].s64 + 1084;
	// 825AFE34: 48844545  bl 0x82df4378
	ctx.lr = 0x825AFE38;
	sub_82DF4378(ctx, base);
	// 825AFE38: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 825AFE3C: 816BB230  lwz r11, -0x4dd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 825AFE40: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825AFE44: 419A002C  beq cr6, 0x825afe70
	if ctx.cr[6].eq {
	pc = 0x825AFE70; continue 'dispatch;
	}
	// 825AFE48: 38A30001  addi r5, r3, 1
	ctx.r[5].s64 = ctx.r[3].s64 + 1;
	// 825AFE4C: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 825AFE50: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825AFE54: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 825AFE58: 48844711  bl 0x82df4568
	ctx.lr = 0x825AFE5C;
	sub_82DF4568(ctx, base);
	// 825AFE5C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825AFE60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AFE64: 48843D6D  bl 0x82df3bd0
	ctx.lr = 0x825AFE68;
	sub_82DF3BD0(ctx, base);
	// 825AFE68: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 825AFE6C: 488435BD  bl 0x82df3428
	ctx.lr = 0x825AFE70;
	sub_82DF3428(ctx, base);
	// 825AFE70: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825AFE74: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 825AFE78: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 825AFE7C: 93E100D8  stw r31, 0xd8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(216 as u32), ctx.r[31].u32 ) };
	// 825AFE80: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 825AFE84: 916100D0  stw r11, 0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), ctx.r[11].u32 ) };
	// 825AFE88: 914100D4  stw r10, 0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), ctx.r[10].u32 ) };
	// 825AFE8C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825AFE90: 93E100DC  stw r31, 0xdc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(220 as u32), ctx.r[31].u32 ) };
	// 825AFE94: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 825AFE98: 93E100E0  stw r31, 0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[31].u32 ) };
	// 825AFE9C: 93E10100  stw r31, 0x100(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(256 as u32), ctx.r[31].u32 ) };
	// 825AFEA0: 93E10120  stw r31, 0x120(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(288 as u32), ctx.r[31].u32 ) };
	// 825AFEA4: 93E10140  stw r31, 0x140(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(320 as u32), ctx.r[31].u32 ) };
	// 825AFEA8: 91210160  stw r9, 0x160(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(352 as u32), ctx.r[9].u32 ) };
	// 825AFEAC: 4BF5F61D  bl 0x8250f4c8
	ctx.lr = 0x825AFEB0;
	sub_8250F4C8(ctx, base);
	// 825AFEB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AFEB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AFEB8: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 825AFEBC: 409A0008  bne cr6, 0x825afec4
	if !ctx.cr[6].eq {
	pc = 0x825AFEC4; continue 'dispatch;
	}
	// 825AFEC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825AFEC4: 4BF58665  bl 0x82508528
	ctx.lr = 0x825AFEC8;
	sub_82508528(ctx, base);
	// 825AFEC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AFECC: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825AFED0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AFED4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AFED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825AFEDC: 419A0024  beq cr6, 0x825aff00
	if ctx.cr[6].eq {
	pc = 0x825AFF00; continue 'dispatch;
	}
	// 825AFEE0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825AFEE4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825AFEE8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AFEEC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825AFEF0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825AFEF4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825AFEF8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825AFEFC: 4082FFE8  bne 0x825afee4
	if !ctx.cr[0].eq {
	pc = 0x825AFEE4; continue 'dispatch;
	}
	// 825AFF00: 390100D0  addi r8, r1, 0xd0
	ctx.r[8].s64 = ctx.r[1].s64 + 208;
	// 825AFF04: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 825AFF08: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 825AFF0C: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 825AFF10: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825AFF14: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 825AFF18: 48855679  bl 0x82e05590
	ctx.lr = 0x825AFF1C;
	sub_82E05590(ctx, base);
	// 825AFF1C: 806100B4  lwz r3, 0xb4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 825AFF20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AFF24: 419A0008  beq cr6, 0x825aff2c
	if ctx.cr[6].eq {
	pc = 0x825AFF2C; continue 'dispatch;
	}
	// 825AFF28: 4BD10969  bl 0x822c0890
	ctx.lr = 0x825AFF2C;
	sub_822C0890(ctx, base);
	// 825AFF2C: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 825AFF30: 48841D61  bl 0x82df1c90
	ctx.lr = 0x825AFF34;
	sub_82DF1C90(ctx, base);
	// 825AFF34: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 825AFF38: 4BF21791  bl 0x824d16c8
	ctx.lr = 0x825AFF3C;
	sub_824D16C8(ctx, base);
	// 825AFF3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825AFF40: 488434E9  bl 0x82df3428
	ctx.lr = 0x825AFF44;
	sub_82DF3428(ctx, base);
	// 825AFF44: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825AFF48: 488434E1  bl 0x82df3428
	ctx.lr = 0x825AFF4C;
	sub_82DF3428(ctx, base);
	// 825AFF4C: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 825AFF50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AFF54: 419A0008  beq cr6, 0x825aff5c
	if ctx.cr[6].eq {
	pc = 0x825AFF5C; continue 'dispatch;
	}
	// 825AFF58: 4BD10939  bl 0x822c0890
	ctx.lr = 0x825AFF5C;
	sub_822C0890(ctx, base);
	// 825AFF5C: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 825AFF60: 419A000C  beq cr6, 0x825aff6c
	if ctx.cr[6].eq {
	pc = 0x825AFF6C; continue 'dispatch;
	}
	// 825AFF64: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 825AFF68: 4BD10929  bl 0x822c0890
	ctx.lr = 0x825AFF6C;
	sub_822C0890(ctx, base);
	// 825AFF6C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825AFF70: 48029041  bl 0x825d8fb0
	ctx.lr = 0x825AFF74;
	sub_825D8FB0(ctx, base);
	// 825AFF74: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 825AFF78: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825AFF7C: 419A0008  beq cr6, 0x825aff84
	if ctx.cr[6].eq {
	pc = 0x825AFF84; continue 'dispatch;
	}
	// 825AFF80: 4BD10911  bl 0x822c0890
	ctx.lr = 0x825AFF84;
	sub_822C0890(ctx, base);
	// 825AFF84: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825AFF88: 382101B0  addi r1, r1, 0x1b0
	ctx.r[1].s64 = ctx.r[1].s64 + 432;
	// 825AFF8C: 48BF821C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825AFF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825AFF90 size=172
    let mut pc: u32 = 0x825AFF90;
    'dispatch: loop {
        match pc {
            0x825AFF90 => {
    //   block [0x825AFF90..0x825B003C)
	// 825AFF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825AFF94: 48BF81D5  bl 0x831a8168
	ctx.lr = 0x825AFF98;
	sub_831A8130(ctx, base);
	// 825AFF98: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825AFF9C: 3D608242  lis r11, -0x7dbe
	ctx.r[11].s64 = -2109603840;
	// 825AFFA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825AFFA4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 825AFFA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825AFFAC: 396BEBE8  addi r11, r11, -0x1418
	ctx.r[11].s64 = ctx.r[11].s64 + -5144;
	// 825AFFB0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 825AFFB4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 825AFFB8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825AFFBC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825AFFC0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 825AFFC4: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 825AFFC8: 4884E931  bl 0x82dfe8f8
	ctx.lr = 0x825AFFCC;
	sub_82DFE8F8(ctx, base);
	// 825AFFCC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 825AFFD0: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825AFFD4: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 825AFFD8: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 825AFFDC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825AFFE0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825AFFE4: 48851985  bl 0x82e01968
	ctx.lr = 0x825AFFE8;
	sub_82E01968(ctx, base);
	// 825AFFE8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825AFFEC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825AFFF0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825AFFF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825AFFF8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825AFFFC: 419A0024  beq cr6, 0x825b0020
	if ctx.cr[6].eq {
	pc = 0x825B0020; continue 'dispatch;
	}
	// 825B0000: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825B0004: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B0008: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B000C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B0010: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B0014: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B0018: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B001C: 4082FFE8  bne 0x825b0004
	if !ctx.cr[0].eq {
	pc = 0x825B0004; continue 'dispatch;
	}
	// 825B0020: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825B0024: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B0028: 419A0008  beq cr6, 0x825b0030
	if ctx.cr[6].eq {
	pc = 0x825B0030; continue 'dispatch;
	}
	// 825B002C: 4BD10865  bl 0x822c0890
	ctx.lr = 0x825B0030;
	sub_822C0890(ctx, base);
	// 825B0030: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B0034: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825B0038: 48BF8180  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B0040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B0040 size=184
    let mut pc: u32 = 0x825B0040;
    'dispatch: loop {
        match pc {
            0x825B0040 => {
    //   block [0x825B0040..0x825B00F8)
	// 825B0040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B0044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B0048: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B004C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B0050: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B0054: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B0058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B005C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825B0060: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B0064: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B0068: 4BD108D1  bl 0x822c0938
	ctx.lr = 0x825B006C;
	sub_822C0938(ctx, base);
	// 825B006C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B0070: 41820028  beq 0x825b0098
	if ctx.cr[0].eq {
	pc = 0x825B0098; continue 'dispatch;
	}
	// 825B0074: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B0078: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825B007C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825B0080: 392BB778  addi r9, r11, -0x4888
	ctx.r[9].s64 = ctx.r[11].s64 + -18568;
	// 825B0084: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B0088: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B008C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825B0090: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825B0094: 48000008  b 0x825b009c
	pc = 0x825B009C; continue 'dispatch;
	// 825B0098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B009C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B00A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B00A4: 409A0038  bne cr6, 0x825b00dc
	if !ctx.cr[6].eq {
	pc = 0x825B00DC; continue 'dispatch;
	}
	// 825B00A8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B00AC: 419A0010  beq cr6, 0x825b00bc
	if ctx.cr[6].eq {
	pc = 0x825B00BC; continue 'dispatch;
	}
	// 825B00B0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B00B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B00B8: 48015021  bl 0x825c50d8
	ctx.lr = 0x825B00BC;
	sub_825C50D8(ctx, base);
	// 825B00BC: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825B00C0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B00C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B00C8: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825B00CC: 816B856C  lwz r11, -0x7a94(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31380 as u32) ) } as u64;
	// 825B00D0: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825B00D4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825B00D8: 4BD0FF29  bl 0x822c0000
	ctx.lr = 0x825B00DC;
	sub_822C0000(ctx, base);
	// 825B00DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B00E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B00E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B00E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B00EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B00F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B00F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B00F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B00F8 size=12
    let mut pc: u32 = 0x825B00F8;
    'dispatch: loop {
        match pc {
            0x825B00F8 => {
    //   block [0x825B00F8..0x825B0104)
	// 825B00F8: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825B00FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B0100: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B0104(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B0104 size=8
    let mut pc: u32 = 0x825B0104;
    'dispatch: loop {
        match pc {
            0x825B0104 => {
    //   block [0x825B0104..0x825B010C)
	// 825B0104: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B0108: 48014FD0  b 0x825c50d8
	sub_825C50D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B010C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B010C size=4
    let mut pc: u32 = 0x825B010C;
    'dispatch: loop {
        match pc {
            0x825B010C => {
    //   block [0x825B010C..0x825B0110)
	// 825B010C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B0110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B0110 size=88
    let mut pc: u32 = 0x825B0110;
    'dispatch: loop {
        match pc {
            0x825B0110 => {
    //   block [0x825B0110..0x825B0168)
	// 825B0110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B0114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B0118: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B011C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B0120: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B0124: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B0128: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B012C: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B0130: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B0134: 4BF5C8DD  bl 0x8250ca10
	ctx.lr = 0x825B0138;
	sub_8250CA10(ctx, base);
	// 825B0138: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825B013C: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B0140: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 825B0144: 48842045  bl 0x82df2188
	ctx.lr = 0x825B0148;
	sub_82DF2188(ctx, base);
	// 825B0148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B014C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825B0150: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825B0154: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B0158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B015C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B0160: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B0164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B0168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B0168 size=548
    let mut pc: u32 = 0x825B0168;
    'dispatch: loop {
        match pc {
            0x825B0168 => {
    //   block [0x825B0168..0x825B038C)
	// 825B0168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B016C: 48BF7FF5  bl 0x831a8160
	ctx.lr = 0x825B0170;
	sub_831A8130(ctx, base);
	// 825B0170: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B0174: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B0178: 3D601555  lis r11, 0x1555
	ctx.r[11].s64 = 357892096;
	// 825B017C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 825B0180: 616B5554  ori r11, r11, 0x5554
	ctx.r[11].u64 = ctx.r[11].u64 | 21844;
	// 825B0184: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 825B0188: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B018C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 825B0190: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 825B0194: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B0198: 41980048  blt cr6, 0x825b01e0
	if ctx.cr[6].lt {
	pc = 0x825B01E0; continue 'dispatch;
	}
	// 825B019C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825B01A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B01A4: 388B9BCC  addi r4, r11, -0x6434
	ctx.r[4].s64 = ctx.r[11].s64 + -25652;
	// 825B01A8: 4BD15721  bl 0x822c58c8
	ctx.lr = 0x825B01AC;
	sub_822C58C8(ctx, base);
	// 825B01AC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825B01B0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825B01B4: 4BD15665  bl 0x822c5818
	ctx.lr = 0x825B01B8;
	sub_822C5818(ctx, base);
	// 825B01B8: 4BD140F9  bl 0x822c42b0
	ctx.lr = 0x825B01BC;
	sub_822C42B0(ctx, base);
	// 825B01BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825B01C0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825B01C4: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 825B01C8: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 825B01CC: 4BD152A5  bl 0x822c5470
	ctx.lr = 0x825B01D0;
	sub_822C5470(ctx, base);
	// 825B01D0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B01D4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B01D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B01DC: 4BD14B05  bl 0x822c4ce0
	ctx.lr = 0x825B01E0;
	sub_822C4CE0(ctx, base);
	// 825B01E0: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B01E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825B01E8: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 825B01EC: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 825B01F0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825B01F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B01F8: 485D6611  bl 0x82b86808
	ctx.lr = 0x825B01FC;
	sub_82B86808(ctx, base);
	// 825B01FC: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B0200: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B0204: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825B0208: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B020C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B0210: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825B0214: 409A0018  bne cr6, 0x825b022c
	if !ctx.cr[6].eq {
	pc = 0x825B022C; continue 'dispatch;
	}
	// 825B0218: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 825B021C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B0220: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 825B0224: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B0228: 4800003C  b 0x825b0264
	pc = 0x825B0264; continue 'dispatch;
	// 825B022C: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B0230: 41820020  beq 0x825b0250
	if ctx.cr[0].eq {
	pc = 0x825B0250; continue 'dispatch;
	}
	// 825B0234: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 825B0238: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B023C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B0240: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825B0244: 409A0024  bne cr6, 0x825b0268
	if !ctx.cr[6].eq {
	pc = 0x825B0268; continue 'dispatch;
	}
	// 825B0248: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 825B024C: 4800001C  b 0x825b0268
	pc = 0x825B0268; continue 'dispatch;
	// 825B0250: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 825B0254: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B0258: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B025C: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825B0260: 409A0008  bne cr6, 0x825b0268
	if !ctx.cr[6].eq {
	pc = 0x825B0268; continue 'dispatch;
	}
	// 825B0264: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 825B0268: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B026C: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 825B0270: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 825B0274: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 825B0278: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 825B027C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825B0280: 409A00F0  bne cr6, 0x825b0370
	if !ctx.cr[6].eq {
	pc = 0x825B0370; continue 'dispatch;
	}
	// 825B0284: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 825B0288: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B028C: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B0290: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B0294: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825B0298: 409A0054  bne cr6, 0x825b02ec
	if !ctx.cr[6].eq {
	pc = 0x825B02EC; continue 'dispatch;
	}
	// 825B029C: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B02A0: 892A0018  lbz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 825B02A4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 825B02A8: 419A0054  beq cr6, 0x825b02fc
	if ctx.cr[6].eq {
	pc = 0x825B02FC; continue 'dispatch;
	}
	// 825B02AC: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B02B0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B02B4: 409A0010  bne cr6, 0x825b02c4
	if !ctx.cr[6].eq {
	pc = 0x825B02C4; continue 'dispatch;
	}
	// 825B02B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B02BC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B02C0: 4BE93CA9  bl 0x82443f68
	ctx.lr = 0x825B02C4;
	sub_82443F68(ctx, base);
	// 825B02C4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B02C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B02CC: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 825B02D0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B02D4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B02D8: 9B6B0018  stb r27, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 825B02DC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B02E0: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B02E4: 4BE93C1D  bl 0x82443f00
	ctx.lr = 0x825B02E8;
	sub_82443F00(ctx, base);
	// 825B02E8: 48000074  b 0x825b035c
	pc = 0x825B035C; continue 'dispatch;
	// 825B02EC: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B02F0: 892A0018  lbz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 825B02F4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 825B02F8: 409A0028  bne cr6, 0x825b0320
	if !ctx.cr[6].eq {
	pc = 0x825B0320; continue 'dispatch;
	}
	// 825B02FC: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B0300: 9BA90018  stb r29, 0x18(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 825B0304: 9BAA0018  stb r29, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 825B0308: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B030C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B0310: 9B6A0018  stb r27, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 825B0314: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B0318: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B031C: 48000040  b 0x825b035c
	pc = 0x825B035C; continue 'dispatch;
	// 825B0320: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B0324: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B0328: 409A0010  bne cr6, 0x825b0338
	if !ctx.cr[6].eq {
	pc = 0x825B0338; continue 'dispatch;
	}
	// 825B032C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B0330: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B0334: 4BE93BCD  bl 0x82443f00
	ctx.lr = 0x825B0338;
	sub_82443F00(ctx, base);
	// 825B0338: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B033C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B0340: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 825B0344: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B0348: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B034C: 9B6B0018  stb r27, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 825B0350: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B0354: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B0358: 4BE93C11  bl 0x82443f68
	ctx.lr = 0x825B035C;
	sub_82443F68(ctx, base);
	// 825B035C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B0360: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 825B0364: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 825B0368: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825B036C: 419AFF1C  beq cr6, 0x825b0288
	if ctx.cr[6].eq {
	pc = 0x825B0288; continue 'dispatch;
	}
	// 825B0370: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B0374: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825B0378: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 825B037C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B0380: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 825B0384: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 825B0388: 48BF7E28  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B0390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B0390 size=496
    let mut pc: u32 = 0x825B0390;
    'dispatch: loop {
        match pc {
            0x825B0390 => {
    //   block [0x825B0390..0x825B0580)
	// 825B0390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B0394: 48BF7DCD  bl 0x831a8160
	ctx.lr = 0x825B0398;
	sub_831A8130(ctx, base);
	// 825B0398: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B039C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825B03A0: 817C0130  lwz r11, 0x130(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(304 as u32) ) } as u64;
	// 825B03A4: 815C0144  lwz r10, 0x144(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(324 as u32) ) } as u64;
	// 825B03A8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B03AC: 419A01CC  beq cr6, 0x825b0578
	if ctx.cr[6].eq {
	pc = 0x825B0578; continue 'dispatch;
	}
	// 825B03B0: 4BFFEC19  bl 0x825aefc8
	ctx.lr = 0x825B03B4;
	sub_825AEFC8(ctx, base);
	// 825B03B4: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 825B03B8: 418201C0  beq 0x825b0578
	if ctx.cr[0].eq {
	pc = 0x825B0578; continue 'dispatch;
	}
	// 825B03BC: 817C0144  lwz r11, 0x144(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(324 as u32) ) } as u64;
	// 825B03C0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 825B03C4: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 825B03C8: 488493C1  bl 0x82df9788
	ctx.lr = 0x825B03CC;
	sub_82DF9788(ctx, base);
	// 825B03CC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B03D0: 3B4BB88C  addi r26, r11, -0x4774
	ctx.r[26].s64 = ctx.r[11].s64 + -18292;
	// 825B03D4: 817C0144  lwz r11, 0x144(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(324 as u32) ) } as u64;
	// 825B03D8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825B03DC: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B03E0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B03E4: 4182001C  beq 0x825b0400
	if ctx.cr[0].eq {
	pc = 0x825B0400; continue 'dispatch;
	}
	// 825B03E8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825B03EC: 48BF8CDD  bl 0x831a90c8
	ctx.lr = 0x825B03F0;
	sub_831A90C8(ctx, base);
	// 825B03F0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 825B03F4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 825B03F8: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 825B03FC: 4BD1988D  bl 0x822c9c88
	ctx.lr = 0x825B0400;
	sub_822C9C88(ctx, base);
	// 825B0400: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825B0404: 4BFF06E5  bl 0x825a0ae8
	ctx.lr = 0x825B0408;
	sub_825A0AE8(ctx, base);
	// 825B0408: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 825B040C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825B0410: 41820078  beq 0x825b0488
	if ctx.cr[0].eq {
	pc = 0x825B0488; continue 'dispatch;
	}
	// 825B0414: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825B0418: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825B041C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B0420: 4BFF2161  bl 0x825a2580
	ctx.lr = 0x825B0424;
	sub_825A2580(ctx, base);
	// 825B0424: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825B0428: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 825B042C: 4884935D  bl 0x82df9788
	ctx.lr = 0x825B0430;
	sub_82DF9788(ctx, base);
	// 825B0430: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B0434: 48842FF5  bl 0x82df3428
	ctx.lr = 0x825B0438;
	sub_82DF3428(ctx, base);
	// 825B0438: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825B043C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825B0440: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825B0444: 4BFF20F5  bl 0x825a2538
	ctx.lr = 0x825B0448;
	sub_825A2538(ctx, base);
	// 825B0448: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825B044C: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 825B0450: 48849339  bl 0x82df9788
	ctx.lr = 0x825B0454;
	sub_82DF9788(ctx, base);
	// 825B0454: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825B0458: 48842FD1  bl 0x82df3428
	ctx.lr = 0x825B045C;
	sub_82DF3428(ctx, base);
	// 825B045C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B0460: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B0464: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 825B0468: 4BD189C9  bl 0x822c8e30
	ctx.lr = 0x825B046C;
	sub_822C8E30(ctx, base);
	// 825B046C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B0470: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B0474: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 825B0478: 4BD189B9  bl 0x822c8e30
	ctx.lr = 0x825B047C;
	sub_822C8E30(ctx, base);
	// 825B047C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 825B0480: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 825B0484: 4198FF90  blt cr6, 0x825b0414
	if ctx.cr[6].lt {
	pc = 0x825B0414; continue 'dispatch;
	}
	// 825B0488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B048C: 837C0130  lwz r27, 0x130(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(304 as u32) ) } as u64;
	// 825B0490: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 825B0494: 809C0144  lwz r4, 0x144(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(324 as u32) ) } as u64;
	// 825B0498: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825B049C: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B04A0: 83BC0134  lwz r29, 0x134(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(308 as u32) ) } as u64;
	// 825B04A4: 83DB0000  lwz r30, 0(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B04A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B04AC: 4836E625  bl 0x8291ead0
	ctx.lr = 0x825B04B0;
	sub_8291EAD0(ctx, base);
	// 825B04B0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B04B4: 37EBFFFB  addic. r31, r11, -5
	ctx.xer.ca = (ctx.r[11].u32 > (!(-5 as u32)));
	ctx.r[31].s64 = ctx.r[11].s64 + -5;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825B04B8: 4080000C  bge 0x825b04c4
	if !ctx.cr[0].lt {
	pc = 0x825B04C4; continue 'dispatch;
	}
	// 825B04BC: 7FFFEA15  add. r31, r31, r29
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825B04C0: 4180FFFC  blt 0x825b04bc
	if ctx.cr[0].lt {
	pc = 0x825B04BC; continue 'dispatch;
	}
	// 825B04C4: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 825B04C8: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825B04CC: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 825B04D0: 40990028  ble cr6, 0x825b04f8
	if !ctx.cr[6].gt {
	pc = 0x825B04F8; continue 'dispatch;
	}
	// 825B04D4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B04D8: 4BDF11B1  bl 0x823a1688
	ctx.lr = 0x825B04DC;
	sub_823A1688(ctx, base);
	// 825B04DC: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B04E0: 7F1DD840  cmplw cr6, r29, r27
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[27].u32, &mut ctx.xer);
	// 825B04E4: 409A000C  bne cr6, 0x825b04f0
	if !ctx.cr[6].eq {
	pc = 0x825B04F0; continue 'dispatch;
	}
	// 825B04E8: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 825B04EC: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 825B04F0: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825B04F4: 4181FFE0  bgt 0x825b04d4
	if ctx.cr[0].gt {
	pc = 0x825B04D4; continue 'dispatch;
	}
	// 825B04F8: 3BE0000B  li r31, 0xb
	ctx.r[31].s64 = 11;
	// 825B04FC: 389D000C  addi r4, r29, 0xc
	ctx.r[4].s64 = ctx.r[29].s64 + 12;
	// 825B0500: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B0504: 48849285  bl 0x82df9788
	ctx.lr = 0x825B0508;
	sub_82DF9788(ctx, base);
	// 825B0508: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 825B050C: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B0510: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B0514: 4182001C  beq 0x825b0530
	if ctx.cr[0].eq {
	pc = 0x825B0530; continue 'dispatch;
	}
	// 825B0518: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825B051C: 48BF8BAD  bl 0x831a90c8
	ctx.lr = 0x825B0520;
	sub_831A90C8(ctx, base);
	// 825B0520: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 825B0524: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 825B0528: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B052C: 4BD1975D  bl 0x822c9c88
	ctx.lr = 0x825B0530;
	sub_822C9C88(ctx, base);
	// 825B0530: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B0534: 4BDF1155  bl 0x823a1688
	ctx.lr = 0x825B0538;
	sub_823A1688(ctx, base);
	// 825B0538: 817C0130  lwz r11, 0x130(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(304 as u32) ) } as u64;
	// 825B053C: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B0540: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B0544: 409A000C  bne cr6, 0x825b0550
	if !ctx.cr[6].eq {
	pc = 0x825B0550; continue 'dispatch;
	}
	// 825B0548: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B054C: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 825B0550: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B0554: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B0558: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B055C: 4BD188D5  bl 0x822c8e30
	ctx.lr = 0x825B0560;
	sub_822C8E30(ctx, base);
	// 825B0560: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825B0564: 4082FF98  bne 0x825b04fc
	if !ctx.cr[0].eq {
	pc = 0x825B04FC; continue 'dispatch;
	}
	// 825B0568: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B056C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B0570: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 825B0574: 4BD188BD  bl 0x822c8e30
	ctx.lr = 0x825B0578;
	sub_822C8E30(ctx, base);
	// 825B0578: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 825B057C: 48BF7C34  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B0580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B0580 size=148
    let mut pc: u32 = 0x825B0580;
    'dispatch: loop {
        match pc {
            0x825B0580 => {
    //   block [0x825B0580..0x825B0614)
	// 825B0580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B0584: 48BF7BE1  bl 0x831a8164
	ctx.lr = 0x825B0588;
	sub_831A8130(ctx, base);
	// 825B0588: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B058C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 825B0590: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 825B0594: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B0598: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 825B059C: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B05A0: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 825B05A4: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B05A8: 4800002C  b 0x825b05d4
	pc = 0x825B05D4; continue 'dispatch;
	// 825B05AC: 389F000C  addi r4, r31, 0xc
	ctx.r[4].s64 = ctx.r[31].s64 + 12;
	// 825B05B0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 825B05B4: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 825B05B8: 48842C81  bl 0x82df3238
	ctx.lr = 0x825B05BC;
	sub_82DF3238(ctx, base);
	// 825B05BC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 825B05C0: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B05C4: 4182000C  beq 0x825b05d0
	if ctx.cr[0].eq {
	pc = 0x825B05D0; continue 'dispatch;
	}
	// 825B05C8: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B05CC: 48000008  b 0x825b05d4
	pc = 0x825B05D4; continue 'dispatch;
	// 825B05D0: 83FF0008  lwz r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B05D4: 897F0019  lbz r11, 0x19(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 825B05D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B05DC: 419AFFD0  beq cr6, 0x825b05ac
	if ctx.cr[6].eq {
	pc = 0x825B05AC; continue 'dispatch;
	}
	// 825B05E0: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 825B05E4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 825B05E8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825B05EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B05F0: 4BFFFB79  bl 0x825b0168
	ctx.lr = 0x825B05F4;
	sub_825B0168(ctx, base);
	// 825B05F4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B05F8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825B05FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B0600: 995E0004  stb r10, 4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 825B0604: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B0608: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B060C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825B0610: 48BF7BA4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B0618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B0618 size=128
    let mut pc: u32 = 0x825B0618;
    'dispatch: loop {
        match pc {
            0x825B0618 => {
    //   block [0x825B0618..0x825B0698)
	// 825B0618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B061C: 48BF7B51  bl 0x831a816c
	ctx.lr = 0x825B0620;
	sub_831A8130(ctx, base);
	// 825B0620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B0624: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 825B0628: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825B062C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B0630: 3BEB7BA0  addi r31, r11, 0x7ba0
	ctx.r[31].s64 = ctx.r[11].s64 + 31648;
	// 825B0634: 816A7BA8  lwz r11, 0x7ba8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(31656 as u32) ) } as u64;
	// 825B0638: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 825B063C: 40820024  bne 0x825b0660
	if !ctx.cr[0].eq {
	pc = 0x825B0660; continue 'dispatch;
	}
	// 825B0640: 3D2082B9  lis r9, -0x7d47
	ctx.r[9].s64 = -2101805056;
	// 825B0644: 3D00825B  lis r8, -0x7da5
	ctx.r[8].s64 = -2107965440;
	// 825B0648: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 825B064C: 392994C8  addi r9, r9, -0x6b38
	ctx.r[9].s64 = ctx.r[9].s64 + -27448;
	// 825B0650: 3908EC88  addi r8, r8, -0x1378
	ctx.r[8].s64 = ctx.r[8].s64 + -4984;
	// 825B0654: 916A7BA8  stw r11, 0x7ba8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31656 as u32), ctx.r[11].u32 ) };
	// 825B0658: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825B065C: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 825B0660: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825B0664: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825B0668: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B066C: 38DE0008  addi r6, r30, 8
	ctx.r[6].s64 = ctx.r[30].s64 + 8;
	// 825B0670: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 825B0674: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B0678: 480A3F49  bl 0x826545c0
	ctx.lr = 0x825B067C;
	sub_826545C0(ctx, base);
	// 825B067C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B0680: 4182000C  beq 0x825b068c
	if ctx.cr[0].eq {
	pc = 0x825B068C; continue 'dispatch;
	}
	// 825B0684: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825B0688: 48000008  b 0x825b0690
	pc = 0x825B0690; continue 'dispatch;
	// 825B068C: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825B0690: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825B0694: 48BF7B28  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B0698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B0698 size=128
    let mut pc: u32 = 0x825B0698;
    'dispatch: loop {
        match pc {
            0x825B0698 => {
    //   block [0x825B0698..0x825B0718)
	// 825B0698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B069C: 48BF7AD1  bl 0x831a816c
	ctx.lr = 0x825B06A0;
	sub_831A8130(ctx, base);
	// 825B06A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B06A4: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 825B06A8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825B06AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B06B0: 3BEB7BAC  addi r31, r11, 0x7bac
	ctx.r[31].s64 = ctx.r[11].s64 + 31660;
	// 825B06B4: 816A7BB4  lwz r11, 0x7bb4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(31668 as u32) ) } as u64;
	// 825B06B8: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 825B06BC: 40820024  bne 0x825b06e0
	if !ctx.cr[0].eq {
	pc = 0x825B06E0; continue 'dispatch;
	}
	// 825B06C0: 3D2082BC  lis r9, -0x7d44
	ctx.r[9].s64 = -2101608448;
	// 825B06C4: 3D00825B  lis r8, -0x7da5
	ctx.r[8].s64 = -2107965440;
	// 825B06C8: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 825B06CC: 3929AD78  addi r9, r9, -0x5288
	ctx.r[9].s64 = ctx.r[9].s64 + -21128;
	// 825B06D0: 3908ECD0  addi r8, r8, -0x1330
	ctx.r[8].s64 = ctx.r[8].s64 + -4912;
	// 825B06D4: 916A7BB4  stw r11, 0x7bb4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31668 as u32), ctx.r[11].u32 ) };
	// 825B06D8: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825B06DC: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 825B06E0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825B06E4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825B06E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B06EC: 38DE0008  addi r6, r30, 8
	ctx.r[6].s64 = ctx.r[30].s64 + 8;
	// 825B06F0: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 825B06F4: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B06F8: 480A3EC9  bl 0x826545c0
	ctx.lr = 0x825B06FC;
	sub_826545C0(ctx, base);
	// 825B06FC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B0700: 4182000C  beq 0x825b070c
	if ctx.cr[0].eq {
	pc = 0x825B070C; continue 'dispatch;
	}
	// 825B0704: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825B0708: 48000008  b 0x825b0710
	pc = 0x825B0710; continue 'dispatch;
	// 825B070C: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825B0710: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825B0714: 48BF7AA8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B0718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B0718 size=704
    let mut pc: u32 = 0x825B0718;
    'dispatch: loop {
        match pc {
            0x825B0718 => {
    //   block [0x825B0718..0x825B09D8)
	// 825B0718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B071C: 48BF7A51  bl 0x831a816c
	ctx.lr = 0x825B0720;
	sub_831A8130(ctx, base);
	// 825B0720: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B0724: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 825B0728: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825B072C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B0730: 3BEB7BB8  addi r31, r11, 0x7bb8
	ctx.r[31].s64 = ctx.r[11].s64 + 31672;
	// 825B0734: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825B0738: 816A7BBC  lwz r11, 0x7bbc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(31676 as u32) ) } as u64;
	// 825B073C: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 825B0740: 4082001C  bne 0x825b075c
	if !ctx.cr[0].eq {
	pc = 0x825B075C; continue 'dispatch;
	}
	// 825B0744: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 825B0748: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 825B074C: 916A7BBC  stw r11, 0x7bbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31676 as u32), ctx.r[11].u32 ) };
	// 825B0750: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B0754: 38899430  addi r4, r9, -0x6bd0
	ctx.r[4].s64 = ctx.r[9].s64 + -27600;
	// 825B0758: 48848DD9  bl 0x82df9530
	ctx.lr = 0x825B075C;
	sub_82DF9530(ctx, base);
	// 825B075C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B0760: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B0764: 409A026C  bne cr6, 0x825b09d0
	if !ctx.cr[6].eq {
	pc = 0x825B09D0; continue 'dispatch;
	}
	// 825B0768: 897E008C  lbz r11, 0x8c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 825B076C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B0770: 41820260  beq 0x825b09d0
	if ctx.cr[0].eq {
	pc = 0x825B09D0; continue 'dispatch;
	}
	// 825B0774: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 825B0778: 4BD1DF01  bl 0x822ce678
	ctx.lr = 0x825B077C;
	sub_822CE678(ctx, base);
	// 825B077C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B0780: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 825B0784: 48B18AE5  bl 0x830c9268
	ctx.lr = 0x825B0788;
	sub_830C9268(ctx, base);
	// 825B0788: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825B078C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B0790: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 825B0794: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 825B0798: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B079C: C1AA08A8  lfs f13, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825B07A0: D0010070  stfs f0, 0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 825B07A4: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 825B07A8: D1A10078  stfs f13, 0x78(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 825B07AC: D1A1007C  stfs f13, 0x7c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 825B07B0: 48B18A51  bl 0x830c9200
	ctx.lr = 0x825B07B4;
	sub_830C9200(ctx, base);
	// 825B07B4: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 825B07B8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 825B07BC: 4198011C  blt cr6, 0x825b08d8
	if ctx.cr[6].lt {
	pc = 0x825B08D8; continue 'dispatch;
	}
	// 825B07C0: 419A00C0  beq cr6, 0x825b0880
	if ctx.cr[6].eq {
	pc = 0x825B0880; continue 'dispatch;
	}
	// 825B07C4: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 825B07C8: 41980060  blt cr6, 0x825b0828
	if ctx.cr[6].lt {
	pc = 0x825B0828; continue 'dispatch;
	}
	// 825B07CC: 409A016C  bne cr6, 0x825b0938
	if !ctx.cr[6].eq {
	pc = 0x825B0938; continue 'dispatch;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B09D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B09D8 size=488
    let mut pc: u32 = 0x825B09D8;
    'dispatch: loop {
        match pc {
            0x825B09D8 => {
    //   block [0x825B09D8..0x825B0BC0)
	// 825B09D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B09DC: 48BF7781  bl 0x831a815c
	ctx.lr = 0x825B09E0;
	sub_831A8130(ctx, base);
	// 825B09E0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B09E4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B09E8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825B09EC: 3BABB830  addi r29, r11, -0x47d0
	ctx.r[29].s64 = ctx.r[11].s64 + -18384;
	// 825B09F0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 825B09F4: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 825B09F8: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 825B09FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B0A00: 38A000F9  li r5, 0xf9
	ctx.r[5].s64 = 249;
	// 825B0A04: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825B0A08: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 825B0A0C: 4BD0F9CD  bl 0x822c03d8
	ctx.lr = 0x825B0A10;
	sub_822C03D8(ctx, base);
	// 825B0A10: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B0A14: 41820010  beq 0x825b0a24
	if ctx.cr[0].eq {
	pc = 0x825B0A24; continue 'dispatch;
	}
	// 825B0A18: 4BFF2241  bl 0x825a2c58
	ctx.lr = 0x825B0A1C;
	sub_825A2C58(ctx, base);
	// 825B0A1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B0A20: 48000008  b 0x825b0a28
	pc = 0x825B0A28; continue 'dispatch;
	// 825B0A24: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825B0A28: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825B0A2C: 3BDC0004  addi r30, r28, 4
	ctx.r[30].s64 = ctx.r[28].s64 + 4;
	// 825B0A30: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B0A34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B0A38: 4BFF1EA1  bl 0x825a28d8
	ctx.lr = 0x825B0A3C;
	sub_825A28D8(ctx, base);
	// 825B0A3C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825B0A40: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B0A44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B0A48: 4BD0F5B9  bl 0x822c0000
	ctx.lr = 0x825B0A4C;
	sub_822C0000(ctx, base);
	// 825B0A4C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825B0A50: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B0A54: 38A000FB  li r5, 0xfb
	ctx.r[5].s64 = 251;
	// 825B0A58: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825B0A5C: 4BD0F97D  bl 0x822c03d8
	ctx.lr = 0x825B0A60;
	sub_822C03D8(ctx, base);
	// 825B0A60: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B0A64: 41820050  beq 0x825b0ab4
	if ctx.cr[0].eq {
	pc = 0x825B0AB4; continue 'dispatch;
	}
	// 825B0A68: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B0A6C: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B0A70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B0A74: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825B0A78: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 825B0A7C: 419A0024  beq cr6, 0x825b0aa0
	if ctx.cr[6].eq {
	pc = 0x825B0AA0; continue 'dispatch;
	}
	// 825B0A80: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825B0A84: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B0A88: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B0A8C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B0A90: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B0A94: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B0A98: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B0A9C: 4082FFE8  bne 0x825b0a84
	if !ctx.cr[0].eq {
	pc = 0x825B0A84; continue 'dispatch;
	}
	// 825B0AA0: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 825B0AA4: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 825B0AA8: 4BFFE271  bl 0x825aed18
	ctx.lr = 0x825B0AAC;
	sub_825AED18(ctx, base);
	// 825B0AAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B0AB0: 48000008  b 0x825b0ab8
	pc = 0x825B0AB8; continue 'dispatch;
	// 825B0AB4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825B0AB8: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 825B0ABC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B0AC0: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825B0AC4: 4BFFEF15  bl 0x825af9d8
	ctx.lr = 0x825B0AC8;
	sub_825AF9D8(ctx, base);
	// 825B0AC8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825B0ACC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B0AD0: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825B0AD4: 4BD0F52D  bl 0x822c0000
	ctx.lr = 0x825B0AD8;
	sub_822C0000(ctx, base);
	// 825B0AD8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 825B0ADC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B0AE0: 48843121  bl 0x82df3c00
	ctx.lr = 0x825B0AE4;
	sub_82DF3C00(ctx, base);
	// 825B0AE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B0AE8: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 825B0AEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B0AF0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825B0AF4: 484DAD1D  bl 0x82a8b810
	ctx.lr = 0x825B0AF8;
	sub_82A8B810(ctx, base);
	// 825B0AF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B0AFC: 4884292D  bl 0x82df3428
	ctx.lr = 0x825B0B00;
	sub_82DF3428(ctx, base);
	// 825B0B00: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825B0B04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B0B08: 419A0008  beq cr6, 0x825b0b10
	if ctx.cr[6].eq {
	pc = 0x825B0B10; continue 'dispatch;
	}
	// 825B0B0C: 4BD0FD85  bl 0x822c0890
	ctx.lr = 0x825B0B10;
	sub_822C0890(ctx, base);
	// 825B0B10: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 825B0B14: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B0B18: 485D4C99  bl 0x82b857b0
	ctx.lr = 0x825B0B1C;
	sub_82B857B0(ctx, base);
	// 825B0B1C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 825B0B20: 389B012C  addi r4, r27, 0x12c
	ctx.r[4].s64 = ctx.r[27].s64 + 300;
	// 825B0B24: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825B0B28: 4BFFFA59  bl 0x825b0580
	ctx.lr = 0x825B0B2C;
	sub_825B0580(ctx, base);
	// 825B0B2C: 80610068  lwz r3, 0x68(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 825B0B30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B0B34: 419A0008  beq cr6, 0x825b0b3c
	if ctx.cr[6].eq {
	pc = 0x825B0B3C; continue 'dispatch;
	}
	// 825B0B38: 4BD0FD59  bl 0x822c0890
	ctx.lr = 0x825B0B3C;
	sub_822C0890(ctx, base);
	// 825B0B3C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B0B40: 488428E9  bl 0x82df3428
	ctx.lr = 0x825B0B44;
	sub_82DF3428(ctx, base);
	// 825B0B44: 80610078  lwz r3, 0x78(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 825B0B48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B0B4C: 419A0008  beq cr6, 0x825b0b54
	if ctx.cr[6].eq {
	pc = 0x825B0B54; continue 'dispatch;
	}
	// 825B0B50: 4BD0FD41  bl 0x822c0890
	ctx.lr = 0x825B0B54;
	sub_822C0890(ctx, base);
	// 825B0B54: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825B0B58: 488428D1  bl 0x82df3428
	ctx.lr = 0x825B0B5C;
	sub_82DF3428(ctx, base);
	// 825B0B5C: 817B0134  lwz r11, 0x134(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(308 as u32) ) } as u64;
	// 825B0B60: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 825B0B64: 409A0010  bne cr6, 0x825b0b74
	if !ctx.cr[6].eq {
	pc = 0x825B0B74; continue 'dispatch;
	}
	// 825B0B68: 817B0130  lwz r11, 0x130(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(304 as u32) ) } as u64;
	// 825B0B6C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B0B70: 917B0144  stw r11, 0x144(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(324 as u32), ctx.r[11].u32 ) };
	// 825B0B74: 572B063F  clrlwi. r11, r25, 0x18
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B0B78: 4182003C  beq 0x825b0bb4
	if ctx.cr[0].eq {
	pc = 0x825B0BB4; continue 'dispatch;
	}
	// 825B0B7C: 83DB013C  lwz r30, 0x13c(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(316 as u32) ) } as u64;
	// 825B0B80: 3BFB0138  addi r31, r27, 0x138
	ctx.r[31].s64 = ctx.r[27].s64 + 312;
	// 825B0B84: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 825B0B88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B0B8C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B0B90: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B0B94: 4BE74985  bl 0x82425518
	ctx.lr = 0x825B0B98;
	sub_82425518(ctx, base);
	// 825B0B98: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825B0B9C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B0BA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B0BA4: 48619255  bl 0x82bc9df8
	ctx.lr = 0x825B0BA8;
	sub_82BC9DF8(ctx, base);
	// 825B0BA8: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 825B0BAC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B0BB0: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825B0BB4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825B0BB8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 825B0BBC: 48BF75F0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B0BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B0BC0 size=592
    let mut pc: u32 = 0x825B0BC0;
    'dispatch: loop {
        match pc {
            0x825B0BC0 => {
    //   block [0x825B0BC0..0x825B0E10)
	// 825B0BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B0BC4: 48BF7595  bl 0x831a8158
	ctx.lr = 0x825B0BC8;
	sub_831A8130(ctx, base);
	// 825B0BC8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B0BCC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825B0BD0: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 825B0BD4: 3B9D012C  addi r28, r29, 0x12c
	ctx.r[28].s64 = ctx.r[29].s64 + 300;
	// 825B0BD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B0BDC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825B0BE0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 825B0BE4: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 825B0BE8: 48245BE1  bl 0x827f67c8
	ctx.lr = 0x825B0BEC;
	sub_827F67C8(ctx, base);
	// 825B0BEC: 817D0130  lwz r11, 0x130(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(304 as u32) ) } as u64;
	// 825B0BF0: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B0BF4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B0BF8: 419A0018  beq cr6, 0x825b0c10
	if ctx.cr[6].eq {
	pc = 0x825B0C10; continue 'dispatch;
	}
	// 825B0BFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B0C00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825B0C04: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B0C08: 91590004  stw r10, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B0C0C: 480001F8  b 0x825b0e04
	pc = 0x825B0E04; continue 'dispatch;
	// 825B0C10: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B0C14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B0C18: 3BCBB830  addi r30, r11, -0x47d0
	ctx.r[30].s64 = ctx.r[11].s64 + -18384;
	// 825B0C1C: 38A00117  li r5, 0x117
	ctx.r[5].s64 = 279;
	// 825B0C20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B0C24: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 825B0C28: 4BD0F7B1  bl 0x822c03d8
	ctx.lr = 0x825B0C2C;
	sub_822C03D8(ctx, base);
	// 825B0C2C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B0C30: 41820010  beq 0x825b0c40
	if ctx.cr[0].eq {
	pc = 0x825B0C40; continue 'dispatch;
	}
	// 825B0C34: 4BFF2025  bl 0x825a2c58
	ctx.lr = 0x825B0C38;
	sub_825A2C58(ctx, base);
	// 825B0C38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B0C3C: 48000008  b 0x825b0c44
	pc = 0x825B0C44; continue 'dispatch;
	// 825B0C40: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825B0C44: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 825B0C48: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B0C4C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825B0C50: 4BFF1C89  bl 0x825a28d8
	ctx.lr = 0x825B0C54;
	sub_825A28D8(ctx, base);
	// 825B0C54: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825B0C58: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B0C5C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825B0C60: 4BD0F3A1  bl 0x822c0000
	ctx.lr = 0x825B0C64;
	sub_822C0000(ctx, base);
	// 825B0C64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B0C68: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B0C6C: 38A00119  li r5, 0x119
	ctx.r[5].s64 = 281;
	// 825B0C70: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825B0C74: 4BD0F765  bl 0x822c03d8
	ctx.lr = 0x825B0C78;
	sub_822C03D8(ctx, base);
	// 825B0C78: 83C1005C  lwz r30, 0x5c(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825B0C7C: 83410058  lwz r26, 0x58(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825B0C80: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B0C84: 41820048  beq 0x825b0ccc
	if ctx.cr[0].eq {
	pc = 0x825B0CCC; continue 'dispatch;
	}
	// 825B0C88: 93410058  stw r26, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[26].u32 ) };
	// 825B0C8C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 825B0C90: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 825B0C94: 419A0024  beq cr6, 0x825b0cb8
	if ctx.cr[6].eq {
	pc = 0x825B0CB8; continue 'dispatch;
	}
	// 825B0C98: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 825B0C9C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B0CA0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B0CA4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B0CA8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B0CAC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B0CB0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B0CB4: 4082FFE8  bne 0x825b0c9c
	if !ctx.cr[0].eq {
	pc = 0x825B0C9C; continue 'dispatch;
	}
	// 825B0CB8: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 825B0CBC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 825B0CC0: 4BFFE0E1  bl 0x825aeda0
	ctx.lr = 0x825B0CC4;
	sub_825AEDA0(ctx, base);
	// 825B0CC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B0CC8: 48000008  b 0x825b0cd0
	pc = 0x825B0CD0; continue 'dispatch;
	// 825B0CCC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825B0CD0: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 825B0CD4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B0CD8: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825B0CDC: 4BFFF365  bl 0x825b0040
	ctx.lr = 0x825B0CE0;
	sub_825B0040(ctx, base);
	// 825B0CE0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825B0CE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B0CE8: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825B0CEC: 4BD0F315  bl 0x822c0000
	ctx.lr = 0x825B0CF0;
	sub_822C0000(ctx, base);
	// 825B0CF0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 825B0CF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B0CF8: 48842F09  bl 0x82df3c00
	ctx.lr = 0x825B0CFC;
	sub_82DF3C00(ctx, base);
	// 825B0CFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B0D00: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 825B0D04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B0D08: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825B0D0C: 484DAB05  bl 0x82a8b810
	ctx.lr = 0x825B0D10;
	sub_82A8B810(ctx, base);
	// 825B0D10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B0D14: 48842715  bl 0x82df3428
	ctx.lr = 0x825B0D18;
	sub_82DF3428(ctx, base);
	// 825B0D18: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825B0D1C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B0D20: 419A0008  beq cr6, 0x825b0d28
	if ctx.cr[6].eq {
	pc = 0x825B0D28; continue 'dispatch;
	}
	// 825B0D24: 4BD0FB6D  bl 0x822c0890
	ctx.lr = 0x825B0D28;
	sub_822C0890(ctx, base);
	// 825B0D28: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 825B0D2C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B0D30: 485D4A81  bl 0x82b857b0
	ctx.lr = 0x825B0D34;
	sub_82B857B0(ctx, base);
	// 825B0D34: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 825B0D38: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825B0D3C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825B0D40: 4BFFF841  bl 0x825b0580
	ctx.lr = 0x825B0D44;
	sub_825B0580(ctx, base);
	// 825B0D44: 80610068  lwz r3, 0x68(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 825B0D48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B0D4C: 419A0008  beq cr6, 0x825b0d54
	if ctx.cr[6].eq {
	pc = 0x825B0D54; continue 'dispatch;
	}
	// 825B0D50: 4BD0FB41  bl 0x822c0890
	ctx.lr = 0x825B0D54;
	sub_822C0890(ctx, base);
	// 825B0D54: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B0D58: 488426D1  bl 0x82df3428
	ctx.lr = 0x825B0D5C;
	sub_82DF3428(ctx, base);
	// 825B0D5C: 80610078  lwz r3, 0x78(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 825B0D60: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B0D64: 419A0008  beq cr6, 0x825b0d6c
	if ctx.cr[6].eq {
	pc = 0x825B0D6C; continue 'dispatch;
	}
	// 825B0D68: 4BD0FB29  bl 0x822c0890
	ctx.lr = 0x825B0D6C;
	sub_822C0890(ctx, base);
	// 825B0D6C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825B0D70: 488426B9  bl 0x82df3428
	ctx.lr = 0x825B0D74;
	sub_82DF3428(ctx, base);
	// 825B0D74: 817D0134  lwz r11, 0x134(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(308 as u32) ) } as u64;
	// 825B0D78: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 825B0D7C: 409A0010  bne cr6, 0x825b0d8c
	if !ctx.cr[6].eq {
	pc = 0x825B0D8C; continue 'dispatch;
	}
	// 825B0D80: 817D0130  lwz r11, 0x130(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(304 as u32) ) } as u64;
	// 825B0D84: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B0D88: 917D0144  stw r11, 0x144(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(324 as u32), ctx.r[11].u32 ) };
	// 825B0D8C: 570B063F  clrlwi. r11, r24, 0x18
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B0D90: 4182003C  beq 0x825b0dcc
	if ctx.cr[0].eq {
	pc = 0x825B0DCC; continue 'dispatch;
	}
	// 825B0D94: 839D013C  lwz r28, 0x13c(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(316 as u32) ) } as u64;
	// 825B0D98: 3BFD0138  addi r31, r29, 0x138
	ctx.r[31].s64 = ctx.r[29].s64 + 312;
	// 825B0D9C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 825B0DA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B0DA4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825B0DA8: 80BC0004  lwz r5, 4(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B0DAC: 4BE7476D  bl 0x82425518
	ctx.lr = 0x825B0DB0;
	sub_82425518(ctx, base);
	// 825B0DB0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825B0DB4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B0DB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B0DBC: 4861903D  bl 0x82bc9df8
	ctx.lr = 0x825B0DC0;
	sub_82BC9DF8(ctx, base);
	// 825B0DC0: 93BC0004  stw r29, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 825B0DC4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B0DC8: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825B0DCC: 93590000  stw r26, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 825B0DD0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 825B0DD4: 93D90004  stw r30, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 825B0DD8: 419A002C  beq cr6, 0x825b0e04
	if ctx.cr[6].eq {
	pc = 0x825B0E04; continue 'dispatch;
	}
	// 825B0DDC: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 825B0DE0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B0DE4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B0DE8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B0DEC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B0DF0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B0DF4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B0DF8: 4082FFE8  bne 0x825b0de0
	if !ctx.cr[0].eq {
	pc = 0x825B0DE0; continue 'dispatch;
	}
	// 825B0DFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B0E00: 4BD0FA91  bl 0x822c0890
	ctx.lr = 0x825B0E04;
	sub_822C0890(ctx, base);
	// 825B0E04: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 825B0E08: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 825B0E0C: 48BF739C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B0E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B0E10 size=204
    let mut pc: u32 = 0x825B0E10;
    'dispatch: loop {
        match pc {
            0x825B0E10 => {
    //   block [0x825B0E10..0x825B0EDC)
	// 825B0E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B0E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B0E18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B0E1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B0E20: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B0E24: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B0E28: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B0E2C: 809E0160  lwz r4, 0x160(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(352 as u32) ) } as u64;
	// 825B0E30: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 825B0E34: 419A0090  beq cr6, 0x825b0ec4
	if ctx.cr[6].eq {
	pc = 0x825B0EC4; continue 'dispatch;
	}
	// 825B0E38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B0E3C: 480279CD  bl 0x825d8808
	ctx.lr = 0x825B0E40;
	sub_825D8808(ctx, base);
	// 825B0E40: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B0E44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B0E48: 419A006C  beq cr6, 0x825b0eb4
	if ctx.cr[6].eq {
	pc = 0x825B0EB4; continue 'dispatch;
	}
	// 825B0E4C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825B0E50: 48028F41  bl 0x825d9d90
	ctx.lr = 0x825B0E54;
	sub_825D9D90(ctx, base);
	// 825B0E54: 3D40825B  lis r10, -0x7da5
	ctx.r[10].s64 = -2107965440;
	// 825B0E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B0E5C: 93C10068  stw r30, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 825B0E60: 394AF778  addi r10, r10, -0x888
	ctx.r[10].s64 = ctx.r[10].s64 + -2184;
	// 825B0E64: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 825B0E68: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825B0E6C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825B0E70: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 825B0E74: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 825B0E78: E8A10068  ld r5, 0x68(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 825B0E7C: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 825B0E80: 4BFFF819  bl 0x825b0698
	ctx.lr = 0x825B0E84;
	sub_825B0698(ctx, base);
	// 825B0E84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B0E88: 3BE10070  addi r31, r1, 0x70
	ctx.r[31].s64 = ctx.r[1].s64 + 112;
	// 825B0E8C: 48842325  bl 0x82df31b0
	ctx.lr = 0x825B0E90;
	sub_82DF31B0(ctx, base);
	// 825B0E90: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825B0E94: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825B0E98: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825B0E9C: 48029045  bl 0x825d9ee0
	ctx.lr = 0x825B0EA0;
	sub_825D9EE0(ctx, base);
	// 825B0EA0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825B0EA4: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B0EA8: 48028F61  bl 0x825d9e08
	ctx.lr = 0x825B0EAC;
	sub_825D9E08(ctx, base);
	// 825B0EAC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825B0EB0: 48028F19  bl 0x825d9dc8
	ctx.lr = 0x825B0EB4;
	sub_825D9DC8(ctx, base);
	// 825B0EB4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B0EB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B0EBC: 419A0008  beq cr6, 0x825b0ec4
	if ctx.cr[6].eq {
	pc = 0x825B0EC4; continue 'dispatch;
	}
	// 825B0EC0: 4BD0F9D1  bl 0x822c0890
	ctx.lr = 0x825B0EC4;
	sub_822C0890(ctx, base);
	// 825B0EC4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 825B0EC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B0ECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B0ED0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B0ED4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B0ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B0EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B0EE0 size=116
    let mut pc: u32 = 0x825B0EE0;
    'dispatch: loop {
        match pc {
            0x825B0EE0 => {
    //   block [0x825B0EE0..0x825B0F54)
	// 825B0EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B0EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B0EE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B0EEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B0EF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B0EF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B0EF8: 817E0160  lwz r11, 0x160(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(352 as u32) ) } as u64;
	// 825B0EFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B0F00: 419A003C  beq cr6, 0x825b0f3c
	if ctx.cr[6].eq {
	pc = 0x825B0F3C; continue 'dispatch;
	}
	// 825B0F04: 83FE0130  lwz r31, 0x130(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(304 as u32) ) } as u64;
	// 825B0F08: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B0F0C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825B0F10: 4800001C  b 0x825b0f2c
	pc = 0x825B0F2C; continue 'dispatch;
	// 825B0F14: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 825B0F18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B0F1C: 4BFFFEF5  bl 0x825b0e10
	ctx.lr = 0x825B0F20;
	sub_825B0E10(ctx, base);
	// 825B0F20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B0F24: 4BDF0765  bl 0x823a1688
	ctx.lr = 0x825B0F28;
	sub_823A1688(ctx, base);
	// 825B0F28: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B0F2C: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 825B0F30: 409AFFE4  bne cr6, 0x825b0f14
	if !ctx.cr[6].eq {
	pc = 0x825B0F14; continue 'dispatch;
	}
	// 825B0F34: 387E0138  addi r3, r30, 0x138
	ctx.r[3].s64 = ctx.r[30].s64 + 312;
	// 825B0F38: 4BE74819  bl 0x82425750
	ctx.lr = 0x825B0F3C;
	sub_82425750(ctx, base);
	// 825B0F3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B0F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B0F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B0F48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B0F4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B0F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B0F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B0F58 size=324
    let mut pc: u32 = 0x825B0F58;
    'dispatch: loop {
        match pc {
            0x825B0F58 => {
    //   block [0x825B0F58..0x825B109C)
	// 825B0F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B0F5C: 48BF720D  bl 0x831a8168
	ctx.lr = 0x825B0F60;
	sub_831A8130(ctx, base);
	// 825B0F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B0F64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B0F68: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 825B0F6C: C1BF0158  lfs f13, 0x158(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(344 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825B0F70: 817F0160  lwz r11, 0x160(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(352 as u32) ) } as u64;
	// 825B0F74: C01C0000  lfs f0, 0(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B0F78: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 825B0F7C: D01F0158  stfs f0, 0x158(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), tmp.u32 ) };
	// 825B0F80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B0F84: 419A0030  beq cr6, 0x825b0fb4
	if ctx.cr[6].eq {
	pc = 0x825B0FB4; continue 'dispatch;
	}
	// 825B0F88: 83BF013C  lwz r29, 0x13c(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(316 as u32) ) } as u64;
	// 825B0F8C: 83DD0000  lwz r30, 0(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B0F90: 48000014  b 0x825b0fa4
	pc = 0x825B0FA4; continue 'dispatch;
	// 825B0F94: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 825B0F98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B0F9C: 4BFFFE75  bl 0x825b0e10
	ctx.lr = 0x825B0FA0;
	sub_825B0E10(ctx, base);
	// 825B0FA0: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B0FA4: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 825B0FA8: 409AFFEC  bne cr6, 0x825b0f94
	if !ctx.cr[6].eq {
	pc = 0x825B0F94; continue 'dispatch;
	}
	// 825B0FAC: 387F0138  addi r3, r31, 0x138
	ctx.r[3].s64 = ctx.r[31].s64 + 312;
	// 825B0FB0: 4BE747A1  bl 0x82425750
	ctx.lr = 0x825B0FB4;
	sub_82425750(ctx, base);
	// 825B0FB4: 897F0150  lbz r11, 0x150(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 825B0FB8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B0FBC: 418200D8  beq 0x825b1094
	if ctx.cr[0].eq {
	pc = 0x825B1094; continue 'dispatch;
	}
	// 825B0FC0: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 825B0FC4: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B0FC8: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 825B0FCC: 4800007C  b 0x825b1048
	pc = 0x825B1048; continue 'dispatch;
	// 825B0FD0: 807D0010  lwz r3, 0x10(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 825B0FD4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B0FD8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B0FDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B0FE0: 4E800421  bctrl
	ctx.lr = 0x825B0FE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B0FE4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B0FE8: 40820050  bne 0x825b1038
	if !ctx.cr[0].eq {
	pc = 0x825B1038; continue 'dispatch;
	}
	// 825B0FEC: 817F0144  lwz r11, 0x144(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(324 as u32) ) } as u64;
	// 825B0FF0: 3BDF0144  addi r30, r31, 0x144
	ctx.r[30].s64 = ctx.r[31].s64 + 324;
	// 825B0FF4: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B0FF8: 409A0024  bne cr6, 0x825b101c
	if !ctx.cr[6].eq {
	pc = 0x825B101C; continue 'dispatch;
	}
	// 825B0FFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B1000: 4BDF0689  bl 0x823a1688
	ctx.lr = 0x825B1004;
	sub_823A1688(ctx, base);
	// 825B1004: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 825B1008: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B100C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B1010: 409A000C  bne cr6, 0x825b101c
	if !ctx.cr[6].eq {
	pc = 0x825B101C; continue 'dispatch;
	}
	// 825B1014: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B1018: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B101C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825B1020: 389F012C  addi r4, r31, 0x12c
	ctx.r[4].s64 = ctx.r[31].s64 + 300;
	// 825B1024: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B1028: 4BF5B5D1  bl 0x8250c5f8
	ctx.lr = 0x825B102C;
	sub_8250C5F8(ctx, base);
	// 825B102C: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B1030: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 825B1034: 48000010  b 0x825b1044
	pc = 0x825B1044; continue 'dispatch;
	// 825B1038: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B103C: 4BDF064D  bl 0x823a1688
	ctx.lr = 0x825B1040;
	sub_823A1688(ctx, base);
	// 825B1040: 83A10050  lwz r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B1044: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 825B1048: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B104C: 409AFF84  bne cr6, 0x825b0fd0
	if !ctx.cr[6].eq {
	pc = 0x825B0FD0; continue 'dispatch;
	}
	// 825B1050: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B1054: 4BFFDF75  bl 0x825aefc8
	ctx.lr = 0x825B1058;
	sub_825AEFC8(ctx, base);
	// 825B1058: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 825B105C: 4182002C  beq 0x825b1088
	if ctx.cr[0].eq {
	pc = 0x825B1088; continue 'dispatch;
	}
	// 825B1060: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B1064: 4BFEFA85  bl 0x825a0ae8
	ctx.lr = 0x825B1068;
	sub_825A0AE8(ctx, base);
	// 825B1068: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 825B106C: 39430002  addi r10, r3, 2
	ctx.r[10].s64 = ctx.r[3].s64 + 2;
	// 825B1070: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 825B1074: 41980014  blt cr6, 0x825b1088
	if ctx.cr[6].lt {
	pc = 0x825B1088; continue 'dispatch;
	}
	// 825B1078: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B107C: 4BFEFA6D  bl 0x825a0ae8
	ctx.lr = 0x825B1080;
	sub_825A0AE8(ctx, base);
	// 825B1080: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 825B1084: 917F0148  stw r11, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[11].u32 ) };
	// 825B1088: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825B108C: 387F00CC  addi r3, r31, 0xcc
	ctx.r[3].s64 = ctx.r[31].s64 + 204;
	// 825B1090: 488AC001  bl 0x82e5d090
	ctx.lr = 0x825B1094;
	sub_82E5D090(ctx, base);
	// 825B1094: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825B1098: 48BF7120  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B10A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B10A0 size=1240
    let mut pc: u32 = 0x825B10A0;
    'dispatch: loop {
        match pc {
            0x825B10A0 => {
    //   block [0x825B10A0..0x825B1578)
	// 825B10A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B10A4: 48BF70A9  bl 0x831a814c
	ctx.lr = 0x825B10A8;
	sub_831A8130(ctx, base);
	// 825B10A8: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B10AC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 825B10B0: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825B10B4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 825B10B8: 48027E41  bl 0x825d8ef8
	ctx.lr = 0x825B10BC;
	sub_825D8EF8(ctx, base);
	// 825B10BC: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 825B10C0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B10C4: 93010054  stw r24, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[24].u32 ) };
	// 825B10C8: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 825B10CC: 93010060  stw r24, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[24].u32 ) };
	// 825B10D0: 3AEBB830  addi r23, r11, -0x47d0
	ctx.r[23].s64 = ctx.r[11].s64 + -18384;
	// 825B10D4: 93010064  stw r24, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[24].u32 ) };
	// 825B10D8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 825B10DC: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 825B10E0: 4BF5E3E9  bl 0x8250f4c8
	ctx.lr = 0x825B10E4;
	sub_8250F4C8(ctx, base);
	// 825B10E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B10E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B10EC: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 825B10F0: 409A0008  bne cr6, 0x825b10f8
	if !ctx.cr[6].eq {
	pc = 0x825B10F8; continue 'dispatch;
	}
	// 825B10F4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 825B10F8: 4BF57431  bl 0x82508528
	ctx.lr = 0x825B10FC;
	sub_82508528(ctx, base);
	// 825B10FC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B1100: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825B1104: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 825B1108: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 825B110C: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 825B1110: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B1114: 4BFFEE7D  bl 0x825aff90
	ctx.lr = 0x825B1118;
	sub_825AFF90(ctx, base);
	// 825B1118: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B111C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B1120: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 825B1124: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B1128: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825B112C: 4BD13335  bl 0x822c4460
	ctx.lr = 0x825B1130;
	sub_822C4460(ctx, base);
	// 825B1130: 8061009C  lwz r3, 0x9c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 825B1134: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B1138: 419A0008  beq cr6, 0x825b1140
	if ctx.cr[6].eq {
	pc = 0x825B1140; continue 'dispatch;
	}
	// 825B113C: 4BD0F755  bl 0x822c0890
	ctx.lr = 0x825B1140;
	sub_822C0890(ctx, base);
	// 825B1140: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 825B1144: 48840B4D  bl 0x82df1c90
	ctx.lr = 0x825B1148;
	sub_82DF1C90(ctx, base);
	// 825B1148: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B114C: 83210054  lwz r25, 0x54(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B1150: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B1154: 419A00F0  beq cr6, 0x825b1244
	if ctx.cr[6].eq {
	pc = 0x825B1244; continue 'dispatch;
	}
	// 825B1158: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B115C: 4884D685  bl 0x82dfe7e0
	ctx.lr = 0x825B1160;
	sub_82DFE7E0(ctx, base);
	// 825B1160: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B1164: 40820058  bne 0x825b11bc
	if !ctx.cr[0].eq {
	pc = 0x825B11BC; continue 'dispatch;
	}
	// 825B1168: 93E10078  stw r31, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[31].u32 ) };
	// 825B116C: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 825B1170: 9321007C  stw r25, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[25].u32 ) };
	// 825B1174: 419A0024  beq cr6, 0x825b1198
	if ctx.cr[6].eq {
	pc = 0x825B1198; continue 'dispatch;
	}
	// 825B1178: 39790004  addi r11, r25, 4
	ctx.r[11].s64 = ctx.r[25].s64 + 4;
	// 825B117C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B1180: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B1184: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B1188: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B118C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B1190: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B1194: 4082FFE8  bne 0x825b117c
	if !ctx.cr[0].eq {
	pc = 0x825B117C; continue 'dispatch;
	}
	// 825B1198: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 825B119C: 4BF398ED  bl 0x824eaa88
	ctx.lr = 0x825B11A0;
	sub_824EAA88(ctx, base);
	// 825B11A0: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 825B11A4: 38C00369  li r6, 0x369
	ctx.r[6].s64 = 873;
	// 825B11A8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B11AC: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 825B11B0: 48608FD1  bl 0x82bba180
	ctx.lr = 0x825B11B4;
	sub_82BBA180(ctx, base);
	// 825B11B4: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 825B11B8: 48840AD9  bl 0x82df1c90
	ctx.lr = 0x825B11BC;
	sub_82DF1C90(ctx, base);
	// 825B11BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B11C0: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 825B11C4: 488E9745  bl 0x82e9a908
	ctx.lr = 0x825B11C8;
	sub_82E9A908(ctx, base);
	// 825B11C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B11CC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825B11D0: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B11D4: 48841FDD  bl 0x82df31b0
	ctx.lr = 0x825B11D8;
	sub_82DF31B0(ctx, base);
	// 825B11D8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 825B11DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B11E0: 48A56F49  bl 0x83008128
	ctx.lr = 0x825B11E4;
	sub_83008128(ctx, base);
	// 825B11E4: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 825B11E8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825B11EC: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 825B11F0: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 825B11F4: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 825B11F8: 48027F31  bl 0x825d9128
	ctx.lr = 0x825B11FC;
	sub_825D9128(ctx, base);
	// 825B11FC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B1200: 3BFA0160  addi r31, r26, 0x160
	ctx.r[31].s64 = ctx.r[26].s64 + 352;
	// 825B1204: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 825B1208: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 825B120C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B1210: 917A0160  stw r11, 0x160(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(352 as u32), ctx.r[11].u32 ) };
	// 825B1214: 4BD1324D  bl 0x822c4460
	ctx.lr = 0x825B1218;
	sub_822C4460(ctx, base);
	// 825B1218: 806100AC  lwz r3, 0xac(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 825B121C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B1220: 419A0008  beq cr6, 0x825b1228
	if ctx.cr[6].eq {
	pc = 0x825B1228; continue 'dispatch;
	}
	// 825B1224: 4BD0F66D  bl 0x822c0890
	ctx.lr = 0x825B1228;
	sub_822C0890(ctx, base);
	// 825B1228: 806100CC  lwz r3, 0xcc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(204 as u32) ) } as u64;
	// 825B122C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B1230: 419A0008  beq cr6, 0x825b1238
	if ctx.cr[6].eq {
	pc = 0x825B1238; continue 'dispatch;
	}
	// 825B1234: 4BD0F65D  bl 0x822c0890
	ctx.lr = 0x825B1238;
	sub_822C0890(ctx, base);
	// 825B1238: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B123C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B1240: 409A0010  bne cr6, 0x825b1250
	if !ctx.cr[6].eq {
	pc = 0x825B1250; continue 'dispatch;
	}
	// 825B1244: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 825B1248: 2F1E000A  cmpwi cr6, r30, 0xa
	ctx.cr[6].compare_i32(ctx.r[30].s32, 10, &mut ctx.xer);
	// 825B124C: 4198FE8C  blt cr6, 0x825b10d8
	if ctx.cr[6].lt {
	pc = 0x825B10D8; continue 'dispatch;
	}
	// 825B1250: 817A0160  lwz r11, 0x160(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(352 as u32) ) } as u64;
	// 825B1254: 3B7A0160  addi r27, r26, 0x160
	ctx.r[27].s64 = ctx.r[26].s64 + 352;
	// 825B1258: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B125C: 409A003C  bne cr6, 0x825b1298
	if !ctx.cr[6].eq {
	pc = 0x825B1298; continue 'dispatch;
	}
	// 825B1260: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 825B1264: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B1268: 419A0008  beq cr6, 0x825b1270
	if ctx.cr[6].eq {
	pc = 0x825B1270; continue 'dispatch;
	}
	// 825B126C: 4BD0F625  bl 0x822c0890
	ctx.lr = 0x825B1270;
	sub_822C0890(ctx, base);
	// 825B1270: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 825B1274: 419A000C  beq cr6, 0x825b1280
	if ctx.cr[6].eq {
	pc = 0x825B1280; continue 'dispatch;
	}
	// 825B1278: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 825B127C: 4BD0F615  bl 0x822c0890
	ctx.lr = 0x825B1280;
	sub_822C0890(ctx, base);
	// 825B1280: 7F1FC378  mr r31, r24
	ctx.r[31].u64 = ctx.r[24].u64;
	// 825B1284: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825B1288: 48027D29  bl 0x825d8fb0
	ctx.lr = 0x825B128C;
	sub_825D8FB0(ctx, base);
	// 825B128C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B1290: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 825B1294: 48BF6F08  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
	// 825B1298: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 825B129C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 825B12A0: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 825B12A4: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 825B12A8: 409801D8  bge cr6, 0x825b1480
	if !ctx.cr[6].lt {
	pc = 0x825B1480; continue 'dispatch;
	}
	// 825B12AC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B12B0: 3BABB8AC  addi r29, r11, -0x4754
	ctx.r[29].s64 = ctx.r[11].s64 + -18260;
	// 825B12B4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 825B12B8: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 825B12BC: 4BF5E20D  bl 0x8250f4c8
	ctx.lr = 0x825B12C0;
	sub_8250F4C8(ctx, base);
	// 825B12C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B12C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B12C8: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 825B12CC: 409A0008  bne cr6, 0x825b12d4
	if !ctx.cr[6].eq {
	pc = 0x825B12D4; continue 'dispatch;
	}
	// 825B12D0: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 825B12D4: 4BF57255  bl 0x82508528
	ctx.lr = 0x825B12D8;
	sub_82508528(ctx, base);
	// 825B12D8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B12DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825B12E0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 825B12E4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 825B12E8: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 825B12EC: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B12F0: 4BFFECA1  bl 0x825aff90
	ctx.lr = 0x825B12F4;
	sub_825AFF90(ctx, base);
	// 825B12F4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B12F8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B12FC: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 825B1300: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B1304: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825B1308: 4BD13159  bl 0x822c4460
	ctx.lr = 0x825B130C;
	sub_822C4460(ctx, base);
	// 825B130C: 806100BC  lwz r3, 0xbc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(188 as u32) ) } as u64;
	// 825B1310: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B1314: 419A0008  beq cr6, 0x825b131c
	if ctx.cr[6].eq {
	pc = 0x825B131C; continue 'dispatch;
	}
	// 825B1318: 4BD0F579  bl 0x822c0890
	ctx.lr = 0x825B131C;
	sub_822C0890(ctx, base);
	// 825B131C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 825B1320: 48840971  bl 0x82df1c90
	ctx.lr = 0x825B1324;
	sub_82DF1C90(ctx, base);
	// 825B1324: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B1328: 83210054  lwz r25, 0x54(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B132C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B1330: 419A0144  beq cr6, 0x825b1474
	if ctx.cr[6].eq {
	pc = 0x825B1474; continue 'dispatch;
	}
	// 825B1334: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B1338: 4884D4A9  bl 0x82dfe7e0
	ctx.lr = 0x825B133C;
	sub_82DFE7E0(ctx, base);
	// 825B133C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B1340: 40820058  bne 0x825b1398
	if !ctx.cr[0].eq {
	pc = 0x825B1398; continue 'dispatch;
	}
	// 825B1344: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 825B1348: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 825B134C: 9321006C  stw r25, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[25].u32 ) };
	// 825B1350: 419A0024  beq cr6, 0x825b1374
	if ctx.cr[6].eq {
	pc = 0x825B1374; continue 'dispatch;
	}
	// 825B1354: 39790004  addi r11, r25, 4
	ctx.r[11].s64 = ctx.r[25].s64 + 4;
	// 825B1358: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B135C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B1360: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B1364: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B1368: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B136C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B1370: 4082FFE8  bne 0x825b1358
	if !ctx.cr[0].eq {
	pc = 0x825B1358; continue 'dispatch;
	}
	// 825B1374: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 825B1378: 4BF39711  bl 0x824eaa88
	ctx.lr = 0x825B137C;
	sub_824EAA88(ctx, base);
	// 825B137C: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 825B1380: 38C00382  li r6, 0x382
	ctx.r[6].s64 = 898;
	// 825B1384: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B1388: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 825B138C: 48608DF5  bl 0x82bba180
	ctx.lr = 0x825B1390;
	sub_82BBA180(ctx, base);
	// 825B1390: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 825B1394: 488408FD  bl 0x82df1c90
	ctx.lr = 0x825B1398;
	sub_82DF1C90(ctx, base);
	// 825B1398: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B139C: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 825B13A0: 488E9569  bl 0x82e9a908
	ctx.lr = 0x825B13A4;
	sub_82E9A908(ctx, base);
	// 825B13A4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B13A8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825B13AC: 82CB0000  lwz r22, 0(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B13B0: 48841E01  bl 0x82df31b0
	ctx.lr = 0x825B13B4;
	sub_82DF31B0(ctx, base);
	// 825B13B4: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 825B13B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B13BC: 48A56D6D  bl 0x83008128
	ctx.lr = 0x825B13C0;
	sub_83008128(ctx, base);
	// 825B13C0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 825B13C4: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 825B13C8: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 825B13CC: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 825B13D0: 7EA7AB78  mr r7, r21
	ctx.r[7].u64 = ctx.r[21].u64;
	// 825B13D4: 48027D55  bl 0x825d9128
	ctx.lr = 0x825B13D8;
	sub_825D9128(ctx, base);
	// 825B13D8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B13DC: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 825B13E0: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 825B13E4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B13E8: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 825B13EC: 4BD13075  bl 0x822c4460
	ctx.lr = 0x825B13F0;
	sub_822C4460(ctx, base);
	// 825B13F0: 806100B4  lwz r3, 0xb4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 825B13F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B13F8: 419A0008  beq cr6, 0x825b1400
	if ctx.cr[6].eq {
	pc = 0x825B1400; continue 'dispatch;
	}
	// 825B13FC: 4BD0F495  bl 0x822c0890
	ctx.lr = 0x825B1400;
	sub_822C0890(ctx, base);
	// 825B1400: 806100C4  lwz r3, 0xc4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(196 as u32) ) } as u64;
	// 825B1404: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B1408: 419A0008  beq cr6, 0x825b1410
	if ctx.cr[6].eq {
	pc = 0x825B1410; continue 'dispatch;
	}
	// 825B140C: 4BD0F485  bl 0x822c0890
	ctx.lr = 0x825B1410;
	sub_822C0890(ctx, base);
	// 825B1410: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 825B1414: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 825B1418: 419A005C  beq cr6, 0x825b1474
	if ctx.cr[6].eq {
	pc = 0x825B1474; continue 'dispatch;
	}
	// 825B141C: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 825B1420: 480273E9  bl 0x825d8808
	ctx.lr = 0x825B1424;
	sub_825D8808(ctx, base);
	// 825B1424: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B1428: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 825B142C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 825B1430: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B1434: 4BE02765  bl 0x823b3b98
	ctx.lr = 0x825B1438;
	sub_823B3B98(ctx, base);
	// 825B1438: 48841D79  bl 0x82df31b0
	ctx.lr = 0x825B143C;
	sub_82DF31B0(ctx, base);
	// 825B143C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 825B1440: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825B1444: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B1448: 48028869  bl 0x825d9cb0
	ctx.lr = 0x825B144C;
	sub_825D9CB0(ctx, base);
	// 825B144C: 806100D4  lwz r3, 0xd4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 825B1450: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B1454: 419A0008  beq cr6, 0x825b145c
	if ctx.cr[6].eq {
	pc = 0x825B145C; continue 'dispatch;
	}
	// 825B1458: 4BD0F439  bl 0x822c0890
	ctx.lr = 0x825B145C;
	sub_822C0890(ctx, base);
	// 825B145C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 825B1460: 48841FC9  bl 0x82df3428
	ctx.lr = 0x825B1464;
	sub_82DF3428(ctx, base);
	// 825B1464: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 825B1468: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 825B146C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825B1470: 48027911  bl 0x825d8d80
	ctx.lr = 0x825B1474;
	sub_825D8D80(ctx, base);
	// 825B1474: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 825B1478: 2F1E000A  cmpwi cr6, r30, 0xa
	ctx.cr[6].compare_i32(ctx.r[30].s32, 10, &mut ctx.xer);
	// 825B147C: 4198FE38  blt cr6, 0x825b12b4
	if ctx.cr[6].lt {
	pc = 0x825B12B4; continue 'dispatch;
	}
	// 825B1480: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825B1484: 809B0000  lwz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B1488: 48027381  bl 0x825d8808
	ctx.lr = 0x825B148C;
	sub_825D8808(ctx, base);
	// 825B148C: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 825B1490: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B1494: 409A003C  bne cr6, 0x825b14d0
	if !ctx.cr[6].eq {
	pc = 0x825B14D0; continue 'dispatch;
	}
	// 825B1498: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 825B149C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B14A0: 419A0008  beq cr6, 0x825b14a8
	if ctx.cr[6].eq {
	pc = 0x825B14A8; continue 'dispatch;
	}
	// 825B14A4: 4BD0F3ED  bl 0x822c0890
	ctx.lr = 0x825B14A8;
	sub_822C0890(ctx, base);
	// 825B14A8: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 825B14AC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B14B0: 419A0008  beq cr6, 0x825b14b8
	if ctx.cr[6].eq {
	pc = 0x825B14B8; continue 'dispatch;
	}
	// 825B14B4: 4BD0F3DD  bl 0x822c0890
	ctx.lr = 0x825B14B8;
	sub_822C0890(ctx, base);
	// 825B14B8: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 825B14BC: 419A000C  beq cr6, 0x825b14c8
	if ctx.cr[6].eq {
	pc = 0x825B14C8; continue 'dispatch;
	}
	// 825B14C0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 825B14C4: 4BD0F3CD  bl 0x822c0890
	ctx.lr = 0x825B14C8;
	sub_822C0890(ctx, base);
	// 825B14C8: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 825B14CC: 4BFFFDB8  b 0x825b1284
	pc = 0x825B1284; continue 'dispatch;
	// 825B14D0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 825B14D4: 480288BD  bl 0x825d9d90
	ctx.lr = 0x825B14D8;
	sub_825D9D90(ctx, base);
	// 825B14D8: 83DA0130  lwz r30, 0x130(r26)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(304 as u32) ) } as u64;
	// 825B14DC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B14E0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 825B14E4: 4800006C  b 0x825b1550
	pc = 0x825B1550; continue 'dispatch;
	// 825B14E8: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825B14EC: 894A0004  lbz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B14F0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B14F4: 41820050  beq 0x825b1544
	if ctx.cr[0].eq {
	pc = 0x825B1544; continue 'dispatch;
	}
	// 825B14F8: 3D40825B  lis r10, -0x7da5
	ctx.r[10].s64 = -2107965440;
	// 825B14FC: 93410050  stw r26, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 825B1500: 3BEB000C  addi r31, r11, 0xc
	ctx.r[31].s64 = ctx.r[11].s64 + 12;
	// 825B1504: 9301006C  stw r24, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[24].u32 ) };
	// 825B1508: 396AF778  addi r11, r10, -0x888
	ctx.r[11].s64 = ctx.r[10].s64 + -2184;
	// 825B150C: 930100E0  stw r24, 0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[24].u32 ) };
	// 825B1510: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 825B1514: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 825B1518: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825B151C: E8810068  ld r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 825B1520: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825B1524: 4BFFF175  bl 0x825b0698
	ctx.lr = 0x825B1528;
	sub_825B0698(ctx, base);
	// 825B1528: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B152C: 3BE100E0  addi r31, r1, 0xe0
	ctx.r[31].s64 = ctx.r[1].s64 + 224;
	// 825B1530: 48841C81  bl 0x82df31b0
	ctx.lr = 0x825B1534;
	sub_82DF31B0(ctx, base);
	// 825B1534: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825B1538: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 825B153C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825B1540: 480289A1  bl 0x825d9ee0
	ctx.lr = 0x825B1544;
	sub_825D9EE0(ctx, base);
	// 825B1544: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825B1548: 4BDF0141  bl 0x823a1688
	ctx.lr = 0x825B154C;
	sub_823A1688(ctx, base);
	// 825B154C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825B1550: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 825B1554: 409AFF94  bne cr6, 0x825b14e8
	if !ctx.cr[6].eq {
	pc = 0x825B14E8; continue 'dispatch;
	}
	// 825B1558: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 825B155C: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 825B1560: 480288A9  bl 0x825d9e08
	ctx.lr = 0x825B1564;
	sub_825D9E08(ctx, base);
	// 825B1564: 387A0138  addi r3, r26, 0x138
	ctx.r[3].s64 = ctx.r[26].s64 + 312;
	// 825B1568: 4BE741E9  bl 0x82425750
	ctx.lr = 0x825B156C;
	sub_82425750(ctx, base);
	// 825B156C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 825B1570: 48028859  bl 0x825d9dc8
	ctx.lr = 0x825B1574;
	sub_825D9DC8(ctx, base);
	// 825B1574: 4BFFFF24  b 0x825b1498
	pc = 0x825B1498; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B1578 size=316
    let mut pc: u32 = 0x825B1578;
    'dispatch: loop {
        match pc {
            0x825B1578 => {
    //   block [0x825B1578..0x825B16B4)
	// 825B1578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B157C: 48BF6BED  bl 0x831a8168
	ctx.lr = 0x825B1580;
	sub_831A8130(ctx, base);
	// 825B1580: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B1584: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B1588: 4BF5FB69  bl 0x825110f0
	ctx.lr = 0x825B158C;
	sub_825110F0(ctx, base);
	// 825B158C: 387F00C4  addi r3, r31, 0xc4
	ctx.r[3].s64 = ctx.r[31].s64 + 196;
	// 825B1590: 48860949  bl 0x82e11ed8
	ctx.lr = 0x825B1594;
	sub_82E11ED8(ctx, base);
	// 825B1594: 3BBF00CC  addi r29, r31, 0xcc
	ctx.r[29].s64 = ctx.r[31].s64 + 204;
	// 825B1598: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825B159C: 488AC335  bl 0x82e5d8d0
	ctx.lr = 0x825B15A0;
	sub_82E5D8D0(ctx, base);
	// 825B15A0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B15A4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825B15A8: 396BB740  addi r11, r11, -0x48c0
	ctx.r[11].s64 = ctx.r[11].s64 + -18624;
	// 825B15AC: 394AB8F0  addi r10, r10, -0x4710
	ctx.r[10].s64 = ctx.r[10].s64 + -18192;
	// 825B15B0: 3D008204  lis r8, -0x7dfc
	ctx.r[8].s64 = -2113667072;
	// 825B15B4: 917F00CC  stw r11, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 825B15B8: 3CE08204  lis r7, -0x7dfc
	ctx.r[7].s64 = -2113667072;
	// 825B15BC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825B15C0: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 825B15C4: 3968B8E0  addi r11, r8, -0x4720
	ctx.r[11].s64 = ctx.r[8].s64 + -18208;
	// 825B15C8: 3929B8BC  addi r9, r9, -0x4744
	ctx.r[9].s64 = ctx.r[9].s64 + -18244;
	// 825B15CC: 3947B8D0  addi r10, r7, -0x4730
	ctx.r[10].s64 = ctx.r[7].s64 + -18224;
	// 825B15D0: 917F00C4  stw r11, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 825B15D4: 913F0028  stw r9, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 825B15D8: 387F012C  addi r3, r31, 0x12c
	ctx.r[3].s64 = ctx.r[31].s64 + 300;
	// 825B15DC: 915F00CC  stw r10, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[10].u32 ) };
	// 825B15E0: 481EA619  bl 0x8279bbf8
	ctx.lr = 0x825B15E4;
	sub_8279BBF8(ctx, base);
	// 825B15E4: 387F0138  addi r3, r31, 0x138
	ctx.r[3].s64 = ctx.r[31].s64 + 312;
	// 825B15E8: 4BEB4EF1  bl 0x824664d8
	ctx.lr = 0x825B15EC;
	sub_824664D8(ctx, base);
	// 825B15EC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825B15F0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825B15F4: 907F013C  stw r3, 0x13c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(316 as u32), ctx.r[3].u32 ) };
	// 825B15F8: 3B9F0170  addi r28, r31, 0x170
	ctx.r[28].s64 = ctx.r[31].s64 + 368;
	// 825B15FC: 93DF0140  stw r30, 0x140(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(320 as u32), ctx.r[30].u32 ) };
	// 825B1600: 815F0130  lwz r10, 0x130(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 825B1604: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825B1608: 915F0144  stw r10, 0x144(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(324 as u32), ctx.r[10].u32 ) };
	// 825B160C: C00B9A8C  lfs f0, -0x6574(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25972 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B1610: D01F0158  stfs f0, 0x158(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), tmp.u32 ) };
	// 825B1614: 93DF0148  stw r30, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[30].u32 ) };
	// 825B1618: 9BDF015C  stb r30, 0x15c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(348 as u32), ctx.r[30].u8 ) };
	// 825B161C: 93DF0160  stw r30, 0x160(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(352 as u32), ctx.r[30].u32 ) };
	// 825B1620: 93DF0164  stw r30, 0x164(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), ctx.r[30].u32 ) };
	// 825B1624: 4800042D  bl 0x825b1a50
	ctx.lr = 0x825B1628;
	sub_825B1A50(ctx, base);
	// 825B1628: 387F01C8  addi r3, r31, 0x1c8
	ctx.r[3].s64 = ctx.r[31].s64 + 456;
	// 825B162C: 48841AC5  bl 0x82df30f0
	ctx.lr = 0x825B1630;
	sub_82DF30F0(ctx, base);
	// 825B1630: 93DF01CC  stw r30, 0x1cc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(460 as u32), ctx.r[30].u32 ) };
	// 825B1634: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B1638: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825B163C: 48AA1CD5  bl 0x83053310
	ctx.lr = 0x825B1640;
	sub_83053310(ctx, base);
	// 825B1640: 3D60825B  lis r11, -0x7da5
	ctx.r[11].s64 = -2107965440;
	// 825B1644: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B1648: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 825B164C: 396B10A0  addi r11, r11, 0x10a0
	ctx.r[11].s64 = ctx.r[11].s64 + 4256;
	// 825B1650: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 825B1654: E8A10068  ld r5, 0x68(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 825B1658: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825B165C: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825B1660: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 825B1664: 4BFFEFB5  bl 0x825b0618
	ctx.lr = 0x825B1668;
	sub_825B0618(ctx, base);
	// 825B1668: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825B166C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 825B1670: 48000171  bl 0x825b17e0
	ctx.lr = 0x825B1674;
	sub_825B17E0(ctx, base);
	// 825B1674: 3D60825B  lis r11, -0x7da5
	ctx.r[11].s64 = -2107965440;
	// 825B1678: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B167C: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 825B1680: 396BFAA8  addi r11, r11, -0x558
	ctx.r[11].s64 = ctx.r[11].s64 + -1368;
	// 825B1684: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 825B1688: E8A10068  ld r5, 0x68(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 825B168C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825B1690: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825B1694: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 825B1698: 4BFFEF81  bl 0x825b0618
	ctx.lr = 0x825B169C;
	sub_825B0618(ctx, base);
	// 825B169C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 825B16A0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825B16A4: 48000175  bl 0x825b1818
	ctx.lr = 0x825B16A8;
	sub_825B1818(ctx, base);
	// 825B16A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B16AC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 825B16B0: 48BF6B08  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B16B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B16B8 size=8
    let mut pc: u32 = 0x825B16B8;
    'dispatch: loop {
        match pc {
            0x825B16B8 => {
    //   block [0x825B16B8..0x825B16C0)
	// 825B16B8: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 825B16BC: 480000D4  b 0x825b1790
	sub_825B1790(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B16C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B16C0 size=8
    let mut pc: u32 = 0x825B16C0;
    'dispatch: loop {
        match pc {
            0x825B16C0 => {
    //   block [0x825B16C0..0x825B16C8)
	// 825B16C0: 3863FF3C  addi r3, r3, -0xc4
	ctx.r[3].s64 = ctx.r[3].s64 + -196;
	// 825B16C4: 480000CC  b 0x825b1790
	sub_825B1790(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B16C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B16C8 size=8
    let mut pc: u32 = 0x825B16C8;
    'dispatch: loop {
        match pc {
            0x825B16C8 => {
    //   block [0x825B16C8..0x825B16D0)
	// 825B16C8: 3863FF34  addi r3, r3, -0xcc
	ctx.r[3].s64 = ctx.r[3].s64 + -204;
	// 825B16CC: 480000C4  b 0x825b1790
	sub_825B1790(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B16D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B16D0 size=188
    let mut pc: u32 = 0x825B16D0;
    'dispatch: loop {
        match pc {
            0x825B16D0 => {
    //   block [0x825B16D0..0x825B178C)
	// 825B16D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B16D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B16D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B16DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B16E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B16E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B16E8: 387F01C8  addi r3, r31, 0x1c8
	ctx.r[3].s64 = ctx.r[31].s64 + 456;
	// 825B16EC: 48841D3D  bl 0x82df3428
	ctx.lr = 0x825B16F0;
	sub_82DF3428(ctx, base);
	// 825B16F0: 3BDF0170  addi r30, r31, 0x170
	ctx.r[30].s64 = ctx.r[31].s64 + 368;
	// 825B16F4: 387E0038  addi r3, r30, 0x38
	ctx.r[3].s64 = ctx.r[30].s64 + 56;
	// 825B16F8: 4BD175C1  bl 0x822c8cb8
	ctx.lr = 0x825B16FC;
	sub_822C8CB8(ctx, base);
	// 825B16FC: 387E0018  addi r3, r30, 0x18
	ctx.r[3].s64 = ctx.r[30].s64 + 24;
	// 825B1700: 4BD175B9  bl 0x822c8cb8
	ctx.lr = 0x825B1704;
	sub_822C8CB8(ctx, base);
	// 825B1704: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B1708: 4BD1A959  bl 0x822cc060
	ctx.lr = 0x825B170C;
	sub_822CC060(ctx, base);
	// 825B170C: 807F0164  lwz r3, 0x164(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(356 as u32) ) } as u64;
	// 825B1710: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B1714: 419A0008  beq cr6, 0x825b171c
	if ctx.cr[6].eq {
	pc = 0x825B171C; continue 'dispatch;
	}
	// 825B1718: 4BD0F179  bl 0x822c0890
	ctx.lr = 0x825B171C;
	sub_822C0890(ctx, base);
	// 825B171C: 387F0138  addi r3, r31, 0x138
	ctx.r[3].s64 = ctx.r[31].s64 + 312;
	// 825B1720: 4BE74031  bl 0x82425750
	ctx.lr = 0x825B1724;
	sub_82425750(ctx, base);
	// 825B1724: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825B1728: 809F013C  lwz r4, 0x13c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(316 as u32) ) } as u64;
	// 825B172C: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 825B1730: 48840A59  bl 0x82df2188
	ctx.lr = 0x825B1734;
	sub_82DF2188(ctx, base);
	// 825B1734: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B1738: 387F012C  addi r3, r31, 0x12c
	ctx.r[3].s64 = ctx.r[31].s64 + 300;
	// 825B173C: 917F013C  stw r11, 0x13c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(316 as u32), ctx.r[11].u32 ) };
	// 825B1740: 4BFFE9D1  bl 0x825b0110
	ctx.lr = 0x825B1744;
	sub_825B0110(ctx, base);
	// 825B1744: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B1748: 387F00CC  addi r3, r31, 0xcc
	ctx.r[3].s64 = ctx.r[31].s64 + 204;
	// 825B174C: 409A0008  bne cr6, 0x825b1754
	if !ctx.cr[6].eq {
	pc = 0x825B1754; continue 'dispatch;
	}
	// 825B1750: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825B1754: 488AC0F5  bl 0x82e5d848
	ctx.lr = 0x825B1758;
	sub_82E5D848(ctx, base);
	// 825B1758: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B175C: 387F00C4  addi r3, r31, 0xc4
	ctx.r[3].s64 = ctx.r[31].s64 + 196;
	// 825B1760: 409A0008  bne cr6, 0x825b1768
	if !ctx.cr[6].eq {
	pc = 0x825B1768; continue 'dispatch;
	}
	// 825B1764: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825B1768: 48860789  bl 0x82e11ef0
	ctx.lr = 0x825B176C;
	sub_82E11EF0(ctx, base);
	// 825B176C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B1770: 4BF5FA29  bl 0x82511198
	ctx.lr = 0x825B1774;
	sub_82511198(ctx, base);
	// 825B1774: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B1778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B177C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B1780: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B1784: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B1788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B1790 size=76
    let mut pc: u32 = 0x825B1790;
    'dispatch: loop {
        match pc {
            0x825B1790 => {
    //   block [0x825B1790..0x825B17DC)
	// 825B1790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B1794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B1798: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B179C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B17A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B17A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B17A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B17AC: 4BFFFF25  bl 0x825b16d0
	ctx.lr = 0x825B17B0;
	sub_825B16D0(ctx, base);
	// 825B17B0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B17B4: 4182000C  beq 0x825b17c0
	if ctx.cr[0].eq {
	pc = 0x825B17C0; continue 'dispatch;
	}
	// 825B17B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B17BC: 48840C1D  bl 0x82df23d8
	ctx.lr = 0x825B17C0;
	sub_82DF23D8(ctx, base);
	// 825B17C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B17C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B17C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B17CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B17D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B17D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B17D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B17E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B17E0 size=56
    let mut pc: u32 = 0x825B17E0;
    'dispatch: loop {
        match pc {
            0x825B17E0 => {
    //   block [0x825B17E0..0x825B1818)
	// 825B17E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B17E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B17E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B17EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B17F0: 38630018  addi r3, r3, 0x18
	ctx.r[3].s64 = ctx.r[3].s64 + 24;
	// 825B17F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B17F8: 4BEBF8D1  bl 0x824710c8
	ctx.lr = 0x825B17FC;
	sub_824710C8(ctx, base);
	// 825B17FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B1800: 4BD174B9  bl 0x822c8cb8
	ctx.lr = 0x825B1804;
	sub_822C8CB8(ctx, base);
	// 825B1804: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B1808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B180C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B1810: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B1814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B1818 size=56
    let mut pc: u32 = 0x825B1818;
    'dispatch: loop {
        match pc {
            0x825B1818 => {
    //   block [0x825B1818..0x825B1850)
	// 825B1818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B181C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B1820: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B1824: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B1828: 38630038  addi r3, r3, 0x38
	ctx.r[3].s64 = ctx.r[3].s64 + 56;
	// 825B182C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B1830: 4BEBF899  bl 0x824710c8
	ctx.lr = 0x825B1834;
	sub_824710C8(ctx, base);
	// 825B1834: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B1838: 4BD17481  bl 0x822c8cb8
	ctx.lr = 0x825B183C;
	sub_822C8CB8(ctx, base);
	// 825B183C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B1840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B1844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B1848: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B184C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B1850 size=116
    let mut pc: u32 = 0x825B1850;
    'dispatch: loop {
        match pc {
            0x825B1850 => {
    //   block [0x825B1850..0x825B18C4)
	// 825B1850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B1854: 48BF6919  bl 0x831a816c
	ctx.lr = 0x825B1858;
	sub_831A8130(ctx, base);
	// 825B1858: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B185C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825B1860: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 825B1864: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B1868: 41980054  blt cr6, 0x825b18bc
	if ctx.cr[6].lt {
	pc = 0x825B18BC; continue 'dispatch;
	}
	// 825B186C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825B1870: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825B1874: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B1878: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B187C: 419A0040  beq cr6, 0x825b18bc
	if ctx.cr[6].eq {
	pc = 0x825B18BC; continue 'dispatch;
	}
	// 825B1880: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B1884: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 825B1888: 7D4A1E70  srawi r10, r10, 3
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 3) as i64;
	// 825B188C: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825B1890: 4098002C  bge cr6, 0x825b18bc
	if !ctx.cr[6].lt {
	pc = 0x825B18BC; continue 'dispatch;
	}
	// 825B1894: 7C9E5A14  add r4, r30, r11
	ctx.r[4].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 825B1898: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B189C: 48847EED  bl 0x82df9788
	ctx.lr = 0x825B18A0;
	sub_82DF9788(ctx, base);
	// 825B18A0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B18A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B18A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B18AC: 4BD17585  bl 0x822c8e30
	ctx.lr = 0x825B18B0;
	sub_822C8E30(ctx, base);
	// 825B18B0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 825B18B4: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 825B18B8: 4BFFFFBC  b 0x825b1874
	pc = 0x825B1874; continue 'dispatch;
	// 825B18BC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825B18C0: 48BF68FC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


