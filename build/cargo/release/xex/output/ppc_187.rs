pub fn sub_82EF9D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9D00 size=136
    let mut pc: u32 = 0x82EF9D00;
    'dispatch: loop {
        match pc {
            0x82EF9D00 => {
    //   block [0x82EF9D00..0x82EF9D88)
	// 82EF9D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9D08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF9D0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9D10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9D14: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF9D18: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82EF9D1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9D20: 4BFFFB69  bl 0x82ef9888
	ctx.lr = 0x82EF9D24;
	sub_82EF9888(ctx, base);
	// 82EF9D24: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF9D28: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EF9D2C: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82EF9D30: 396BC544  addi r11, r11, -0x3abc
	ctx.r[11].s64 = ctx.r[11].s64 + -15036;
	// 82EF9D34: 394ACFF4  addi r10, r10, -0x300c
	ctx.r[10].s64 = ctx.r[10].s64 + -12300;
	// 82EF9D38: 3929CFE0  addi r9, r9, -0x3020
	ctx.r[9].s64 = ctx.r[9].s64 + -12320;
	// 82EF9D3C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EF9D40: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EF9D44: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82EF9D48: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82EF9D4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EF9D50: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF9D54: 48000D75  bl 0x82efaac8
	ctx.lr = 0x82EF9D58;
	sub_82EFAAC8(ctx, base);
	// 82EF9D58: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 82EF9D5C: 48001255  bl 0x82efafb0
	ctx.lr = 0x82EF9D60;
	sub_82EFAFB0(ctx, base);
	// 82EF9D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF9D64: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82EF9D68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9D6C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EF9D70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF9D74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF9D78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9D7C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF9D80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9D88 size=112
    let mut pc: u32 = 0x82EF9D88;
    'dispatch: loop {
        match pc {
            0x82EF9D88 => {
    //   block [0x82EF9D88..0x82EF9DF8)
	// 82EF9D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9D90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF9D94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9D98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9D9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9DA0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF9DA4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EF9DA8: 396BCFF4  addi r11, r11, -0x300c
	ctx.r[11].s64 = ctx.r[11].s64 + -12300;
	// 82EF9DAC: 394ACFE0  addi r10, r10, -0x3020
	ctx.r[10].s64 = ctx.r[10].s64 + -12320;
	// 82EF9DB0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EF9DB4: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 82EF9DB8: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82EF9DBC: 3BDF0014  addi r30, r31, 0x14
	ctx.r[30].s64 = ctx.r[31].s64 + 20;
	// 82EF9DC0: 48001381  bl 0x82efb140
	ctx.lr = 0x82EF9DC4;
	sub_82EFB140(ctx, base);
	// 82EF9DC4: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82EF9DC8: 48001311  bl 0x82efb0d8
	ctx.lr = 0x82EF9DCC;
	sub_82EFB0D8(ctx, base);
	// 82EF9DCC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EF9DD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9DD4: 396BC544  addi r11, r11, -0x3abc
	ctx.r[11].s64 = ctx.r[11].s64 + -15036;
	// 82EF9DD8: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EF9DDC: 4BFFFD55  bl 0x82ef9b30
	ctx.lr = 0x82EF9DE0;
	sub_82EF9B30(ctx, base);
	// 82EF9DE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF9DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF9DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9DEC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF9DF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9DF8 size=164
    let mut pc: u32 = 0x82EF9DF8;
    'dispatch: loop {
        match pc {
            0x82EF9DF8 => {
    //   block [0x82EF9DF8..0x82EF9E9C)
	// 82EF9DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9DFC: 4BDAF60D  bl 0x82ca9408
	ctx.lr = 0x82EF9E00;
	sub_82CA93D0(ctx, base);
	// 82EF9E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9E04: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF9E08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9E0C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EF9E10: 419A0080  beq cr6, 0x82ef9e90
	if ctx.cr[6].eq {
	pc = 0x82EF9E90; continue 'dispatch;
	}
	// 82EF9E14: 3BBF0020  addi r29, r31, 0x20
	ctx.r[29].s64 = ctx.r[31].s64 + 32;
	// 82EF9E18: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF9E1C: 48000D3D  bl 0x82efab58
	ctx.lr = 0x82EF9E20;
	sub_82EFAB58(ctx, base);
	// 82EF9E20: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9E24: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82EF9E28: 7D7E5851  subf. r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF9E2C: 41800014  blt 0x82ef9e40
	if ctx.cr[0].lt {
	pc = 0x82EF9E40; continue 'dispatch;
	}
	// 82EF9E30: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9E34: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82EF9E38: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EF9E3C: 48000008  b 0x82ef9e44
	pc = 0x82EF9E44; continue 'dispatch;
	// 82EF9E40: 939F001C  stw r28, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82EF9E44: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 82EF9E48: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 82EF9E4C: 409A000C  bne cr6, 0x82ef9e58
	if !ctx.cr[6].eq {
	pc = 0x82EF9E58; continue 'dispatch;
	}
	// 82EF9E50: 480011B9  bl 0x82efb008
	ctx.lr = 0x82EF9E54;
	sub_82EFB008(ctx, base);
	// 82EF9E54: 48000008  b 0x82ef9e5c
	pc = 0x82EF9E5C; continue 'dispatch;
	// 82EF9E58: 480011B9  bl 0x82efb010
	ctx.lr = 0x82EF9E5C;
	sub_82EFB010(ctx, base);
	// 82EF9E5C: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82EF9E60: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EF9E64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9E68: 4BFFFC51  bl 0x82ef9ab8
	ctx.lr = 0x82EF9E6C;
	sub_82EF9AB8(ctx, base);
	// 82EF9E6C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF9E70: 48001C31  bl 0x82efbaa0
	ctx.lr = 0x82EF9E74;
	sub_82EFBAA0(ctx, base);
	// 82EF9E74: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF9E78: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EF9E7C: 419A0014  beq cr6, 0x82ef9e90
	if ctx.cr[6].eq {
	pc = 0x82EF9E90; continue 'dispatch;
	}
	// 82EF9E80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9E84: 4BFFFAED  bl 0x82ef9970
	ctx.lr = 0x82EF9E88;
	sub_82EF9970(ctx, base);
	// 82EF9E88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9E8C: 4BFFFBBD  bl 0x82ef9a48
	ctx.lr = 0x82EF9E90;
	sub_82EF9A48(ctx, base);
	// 82EF9E90: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EF9E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF9E98: 4BDAF5C0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9EA0 size=164
    let mut pc: u32 = 0x82EF9EA0;
    'dispatch: loop {
        match pc {
            0x82EF9EA0 => {
    //   block [0x82EF9EA0..0x82EF9F44)
	// 82EF9EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EF9EA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EF9EAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EF9EB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9EB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9EB8: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82EF9EBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9EC0: 48000C99  bl 0x82efab58
	ctx.lr = 0x82EF9EC4;
	sub_82EFAB58(ctx, base);
	// 82EF9EC4: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9EC8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF9ECC: 40990010  ble cr6, 0x82ef9edc
	if !ctx.cr[6].gt {
	pc = 0x82EF9EDC; continue 'dispatch;
	}
	// 82EF9ED0: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9ED4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EF9ED8: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EF9EDC: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 82EF9EE0: 48001129  bl 0x82efb008
	ctx.lr = 0x82EF9EE4;
	sub_82EFB008(ctx, base);
	// 82EF9EE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EF9EE8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EF9EEC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EF9EF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9EF4: 4BFFFBC5  bl 0x82ef9ab8
	ctx.lr = 0x82EF9EF8;
	sub_82EF9AB8(ctx, base);
	// 82EF9EF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9EFC: 48001BA5  bl 0x82efbaa0
	ctx.lr = 0x82EF9F00;
	sub_82EFBAA0(ctx, base);
	// 82EF9F00: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF9F04: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF9F08: 419A000C  beq cr6, 0x82ef9f14
	if ctx.cr[6].eq {
	pc = 0x82EF9F14; continue 'dispatch;
	}
	// 82EF9F0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9F10: 4BFFFA61  bl 0x82ef9970
	ctx.lr = 0x82EF9F14;
	sub_82EF9970(ctx, base);
	// 82EF9F14: 83FF001C  lwz r31, 0x1c(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9F18: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF9F1C: 419A000C  beq cr6, 0x82ef9f28
	if ctx.cr[6].eq {
	pc = 0x82EF9F28; continue 'dispatch;
	}
	// 82EF9F20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9F24: 4BFFFB25  bl 0x82ef9a48
	ctx.lr = 0x82EF9F28;
	sub_82EF9A48(ctx, base);
	// 82EF9F28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9F2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EF9F30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EF9F34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EF9F38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EF9F3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EF9F40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9F48 size=152
    let mut pc: u32 = 0x82EF9F48;
    'dispatch: loop {
        match pc {
            0x82EF9F48 => {
    //   block [0x82EF9F48..0x82EF9FE0)
	// 82EF9F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9F4C: 4BDAF4BD  bl 0x82ca9408
	ctx.lr = 0x82EF9F50;
	sub_82CA93D0(ctx, base);
	// 82EF9F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9F54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EF9F58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EF9F5C: 3BBF0020  addi r29, r31, 0x20
	ctx.r[29].s64 = ctx.r[31].s64 + 32;
	// 82EF9F60: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF9F64: 48000BF5  bl 0x82efab58
	ctx.lr = 0x82EF9F68;
	sub_82EFAB58(ctx, base);
	// 82EF9F68: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9F6C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82EF9F70: 7D7E5851  subf. r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EF9F74: 41800014  blt 0x82ef9f88
	if ctx.cr[0].lt {
	pc = 0x82EF9F88; continue 'dispatch;
	}
	// 82EF9F78: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9F7C: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82EF9F80: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EF9F84: 48000008  b 0x82ef9f8c
	pc = 0x82EF9F8C; continue 'dispatch;
	// 82EF9F88: 939F001C  stw r28, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82EF9F8C: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 82EF9F90: 48001081  bl 0x82efb010
	ctx.lr = 0x82EF9F94;
	sub_82EFB010(ctx, base);
	// 82EF9F94: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82EF9F98: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EF9F9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9FA0: 4BFFFB19  bl 0x82ef9ab8
	ctx.lr = 0x82EF9FA4;
	sub_82EF9AB8(ctx, base);
	// 82EF9FA4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EF9FA8: 48001AF9  bl 0x82efbaa0
	ctx.lr = 0x82EF9FAC;
	sub_82EFBAA0(ctx, base);
	// 82EF9FAC: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EF9FB0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF9FB4: 419A000C  beq cr6, 0x82ef9fc0
	if ctx.cr[6].eq {
	pc = 0x82EF9FC0; continue 'dispatch;
	}
	// 82EF9FB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9FBC: 4BFFF9B5  bl 0x82ef9970
	ctx.lr = 0x82EF9FC0;
	sub_82EF9970(ctx, base);
	// 82EF9FC0: 83FF001C  lwz r31, 0x1c(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EF9FC4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EF9FC8: 419A000C  beq cr6, 0x82ef9fd4
	if ctx.cr[6].eq {
	pc = 0x82EF9FD4; continue 'dispatch;
	}
	// 82EF9FCC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EF9FD0: 4BFFFA79  bl 0x82ef9a48
	ctx.lr = 0x82EF9FD4;
	sub_82EF9A48(ctx, base);
	// 82EF9FD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EF9FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EF9FDC: 4BDAF47C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EF9FE0 size=12
    let mut pc: u32 = 0x82EF9FE0;
    'dispatch: loop {
        match pc {
            0x82EF9FE0 => {
    //   block [0x82EF9FE0..0x82EF9FEC)
	// 82EF9FE0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EF9FE4: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82EF9FE8: 4BFFFE10  b 0x82ef9df8
	sub_82EF9DF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EF9FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EF9FF0 size=108
    let mut pc: u32 = 0x82EF9FF0;
    'dispatch: loop {
        match pc {
            0x82EF9FF0 => {
    //   block [0x82EF9FF0..0x82EFA05C)
	// 82EF9FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EF9FF4: 4BDAF419  bl 0x82ca940c
	ctx.lr = 0x82EF9FF8;
	sub_82CA93D0(ctx, base);
	// 82EF9FF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EF9FFC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFA000: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EFA004: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFA008: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EFA00C: 4BFFF87D  bl 0x82ef9888
	ctx.lr = 0x82EFA010;
	sub_82EF9888(ctx, base);
	// 82EFA010: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFA014: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFA018: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82EFA01C: 396BC544  addi r11, r11, -0x3abc
	ctx.r[11].s64 = ctx.r[11].s64 + -15036;
	// 82EFA020: 394AD014  addi r10, r10, -0x2fec
	ctx.r[10].s64 = ctx.r[10].s64 + -12268;
	// 82EFA024: 3929D000  addi r9, r9, -0x3000
	ctx.r[9].s64 = ctx.r[9].s64 + -12288;
	// 82EFA028: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EFA02C: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82EFA030: 3D6082F0  lis r11, -0x7d10
	ctx.r[11].s64 = -2098200576;
	// 82EFA034: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EFA038: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EFA03C: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82EFA040: 388B9A30  addi r4, r11, -0x65d0
	ctx.r[4].s64 = ctx.r[11].s64 + -26064;
	// 82EFA044: 93BF001C  stw r29, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 82EFA048: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFA04C: 4BFFF555  bl 0x82ef95a0
	ctx.lr = 0x82EFA050;
	sub_82EF95A0(ctx, base);
	// 82EFA050: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFA054: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFA058: 4BDAF404  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFA060 size=8
    let mut pc: u32 = 0x82EFA060;
    'dispatch: loop {
        match pc {
            0x82EFA060 => {
    //   block [0x82EFA060..0x82EFA068)
	// 82EFA060: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82EFA064: 48000864  b 0x82efa8c8
	sub_82EFA8C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA068 size=116
    let mut pc: u32 = 0x82EFA068;
    'dispatch: loop {
        match pc {
            0x82EFA068 => {
    //   block [0x82EFA068..0x82EFA0DC)
	// 82EFA068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFA070: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFA074: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFA078: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA07C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFA080: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFA084: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFA088: 396BD014  addi r11, r11, -0x2fec
	ctx.r[11].s64 = ctx.r[11].s64 + -12268;
	// 82EFA08C: 394AD000  addi r10, r10, -0x3000
	ctx.r[10].s64 = ctx.r[10].s64 + -12288;
	// 82EFA090: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFA094: 3D6082F0  lis r11, -0x7d10
	ctx.r[11].s64 = -2098200576;
	// 82EFA098: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82EFA09C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EFA0A0: 388B9A30  addi r4, r11, -0x65d0
	ctx.r[4].s64 = ctx.r[11].s64 + -26064;
	// 82EFA0A4: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFA0A8: 3BDF0014  addi r30, r31, 0x14
	ctx.r[30].s64 = ctx.r[31].s64 + 20;
	// 82EFA0AC: 4BFFF5E5  bl 0x82ef9690
	ctx.lr = 0x82EFA0B0;
	sub_82EF9690(ctx, base);
	// 82EFA0B0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFA0B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFA0B8: 396BC544  addi r11, r11, -0x3abc
	ctx.r[11].s64 = ctx.r[11].s64 + -15036;
	// 82EFA0BC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EFA0C0: 4BFFFA71  bl 0x82ef9b30
	ctx.lr = 0x82EFA0C4;
	sub_82EF9B30(ctx, base);
	// 82EFA0C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFA0C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFA0CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFA0D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFA0D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFA0D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFA0E0 size=20
    let mut pc: u32 = 0x82EFA0E0;
    'dispatch: loop {
        match pc {
            0x82EFA0E0 => {
    //   block [0x82EFA0E0..0x82EFA0F4)
	// 82EFA0E0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EFA0E4: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA0E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFA0EC: 409A0008  bne cr6, 0x82efa0f4
	if !ctx.cr[6].eq {
		sub_82EFA0F4(ctx, base);
		return;
	}
	// 82EFA0F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA0F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFA0F4 size=8
    let mut pc: u32 = 0x82EFA0F4;
    'dispatch: loop {
        match pc {
            0x82EFA0F4 => {
    //   block [0x82EFA0F4..0x82EFA0FC)
	// 82EFA0F4: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFA0F8: 4BFFFD00  b 0x82ef9df8
	sub_82EF9DF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA100 size=112
    let mut pc: u32 = 0x82EFA100;
    'dispatch: loop {
        match pc {
            0x82EFA100 => {
    //   block [0x82EFA100..0x82EFA170)
	// 82EFA100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFA108: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFA10C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFA110: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA114: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFA118: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82EFA11C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFA120: 48003741  bl 0x82efd860
	ctx.lr = 0x82EFA124;
	sub_82EFD860(ctx, base);
	// 82EFA124: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFA128: 4182002C  beq 0x82efa154
	if ctx.cr[0].eq {
	pc = 0x82EFA154; continue 'dispatch;
	}
	// 82EFA12C: 3D605647  lis r11, 0x5647
	ctx.r[11].s64 = 1447493632;
	// 82EFA130: 3D409FE1  lis r10, -0x601f
	ctx.r[10].s64 = -1612644352;
	// 82EFA134: 616B1E89  ori r11, r11, 0x1e89
	ctx.r[11].u64 = ctx.r[11].u64 | 7817;
	// 82EFA138: 614A234A  ori r10, r10, 0x234a
	ctx.r[10].u64 = ctx.r[10].u64 | 9034;
	// 82EFA13C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EFA140: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EFA144: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EFA148: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFA14C: 4BFFFEA5  bl 0x82ef9ff0
	ctx.lr = 0x82EFA150;
	sub_82EF9FF0(ctx, base);
	// 82EFA150: 48000008  b 0x82efa158
	pc = 0x82EFA158; continue 'dispatch;
	// 82EFA154: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFA158: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFA15C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFA160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFA164: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFA168: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFA16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA170 size=132
    let mut pc: u32 = 0x82EFA170;
    'dispatch: loop {
        match pc {
            0x82EFA170 => {
    //   block [0x82EFA170..0x82EFA1F4)
	// 82EFA170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA174: 4BDAF295  bl 0x82ca9408
	ctx.lr = 0x82EFA178;
	sub_82CA93D0(ctx, base);
	// 82EFA178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA17C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EFA180: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFA184: 578B07BD  rlwinm. r11, r28, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA188: 41820048  beq 0x82efa1d0
	if ctx.cr[0].eq {
	pc = 0x82EFA1D0; continue 'dispatch;
	}
	// 82EFA18C: 815EFFFC  lwz r10, -4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82EFA190: 3BBEFFFC  addi r29, r30, -4
	ctx.r[29].s64 = ctx.r[30].s64 + -4;
	// 82EFA194: 1D6A003C  mulli r11, r10, 0x3c
	ctx.r[11].s64 = ctx.r[10].s64 * 60;
	// 82EFA198: 37EAFFFF  addic. r31, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFA19C: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EFA1A0: 41800018  blt 0x82efa1b8
	if ctx.cr[0].lt {
	pc = 0x82EFA1B8; continue 'dispatch;
	}
	// 82EFA1A4: 3BDEFFC4  addi r30, r30, -0x3c
	ctx.r[30].s64 = ctx.r[30].s64 + -60;
	// 82EFA1A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFA1AC: 4BFFFA5D  bl 0x82ef9c08
	ctx.lr = 0x82EFA1B0;
	sub_82EF9C08(ctx, base);
	// 82EFA1B0: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFA1B4: 4080FFF0  bge 0x82efa1a4
	if !ctx.cr[0].lt {
	pc = 0x82EFA1A4; continue 'dispatch;
	}
	// 82EFA1B8: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA1BC: 4182000C  beq 0x82efa1c8
	if ctx.cr[0].eq {
	pc = 0x82EFA1C8; continue 'dispatch;
	}
	// 82EFA1C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFA1C4: 4B94B5ED  bl 0x828457b0
	ctx.lr = 0x82EFA1C8;
	sub_828457B0(ctx, base);
	// 82EFA1C8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFA1CC: 48000020  b 0x82efa1ec
	pc = 0x82EFA1EC; continue 'dispatch;
	// 82EFA1D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFA1D4: 4BFFFA35  bl 0x82ef9c08
	ctx.lr = 0x82EFA1D8;
	sub_82EF9C08(ctx, base);
	// 82EFA1D8: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA1DC: 4182000C  beq 0x82efa1e8
	if ctx.cr[0].eq {
	pc = 0x82EFA1E8; continue 'dispatch;
	}
	// 82EFA1E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFA1E4: 480036BD  bl 0x82efd8a0
	ctx.lr = 0x82EFA1E8;
	sub_82EFD8A0(ctx, base);
	// 82EFA1E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFA1EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFA1F0: 4BDAF268  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA1F8 size=132
    let mut pc: u32 = 0x82EFA1F8;
    'dispatch: loop {
        match pc {
            0x82EFA1F8 => {
    //   block [0x82EFA1F8..0x82EFA27C)
	// 82EFA1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA1FC: 4BDAF20D  bl 0x82ca9408
	ctx.lr = 0x82EFA200;
	sub_82CA93D0(ctx, base);
	// 82EFA200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA204: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EFA208: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFA20C: 578B07BD  rlwinm. r11, r28, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA210: 41820048  beq 0x82efa258
	if ctx.cr[0].eq {
	pc = 0x82EFA258; continue 'dispatch;
	}
	// 82EFA214: 815EFFFC  lwz r10, -4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82EFA218: 3BBEFFFC  addi r29, r30, -4
	ctx.r[29].s64 = ctx.r[30].s64 + -4;
	// 82EFA21C: 554B3032  slwi r11, r10, 6
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFA220: 37EAFFFF  addic. r31, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFA224: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EFA228: 41800018  blt 0x82efa240
	if ctx.cr[0].lt {
	pc = 0x82EFA240; continue 'dispatch;
	}
	// 82EFA22C: 3BDEFFC0  addi r30, r30, -0x40
	ctx.r[30].s64 = ctx.r[30].s64 + -64;
	// 82EFA230: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFA234: 4BFFFB55  bl 0x82ef9d88
	ctx.lr = 0x82EFA238;
	sub_82EF9D88(ctx, base);
	// 82EFA238: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFA23C: 4080FFF0  bge 0x82efa22c
	if !ctx.cr[0].lt {
	pc = 0x82EFA22C; continue 'dispatch;
	}
	// 82EFA240: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA244: 4182000C  beq 0x82efa250
	if ctx.cr[0].eq {
	pc = 0x82EFA250; continue 'dispatch;
	}
	// 82EFA248: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFA24C: 4B94B565  bl 0x828457b0
	ctx.lr = 0x82EFA250;
	sub_828457B0(ctx, base);
	// 82EFA250: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFA254: 48000020  b 0x82efa274
	pc = 0x82EFA274; continue 'dispatch;
	// 82EFA258: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFA25C: 4BFFFB2D  bl 0x82ef9d88
	ctx.lr = 0x82EFA260;
	sub_82EF9D88(ctx, base);
	// 82EFA260: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA264: 4182000C  beq 0x82efa270
	if ctx.cr[0].eq {
	pc = 0x82EFA270; continue 'dispatch;
	}
	// 82EFA268: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFA26C: 48003635  bl 0x82efd8a0
	ctx.lr = 0x82EFA270;
	sub_82EFD8A0(ctx, base);
	// 82EFA270: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFA274: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFA278: 4BDAF1E0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA280 size=76
    let mut pc: u32 = 0x82EFA280;
    'dispatch: loop {
        match pc {
            0x82EFA280 => {
    //   block [0x82EFA280..0x82EFA2CC)
	// 82EFA280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFA288: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFA28C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFA290: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA294: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFA298: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFA29C: 4BFFF895  bl 0x82ef9b30
	ctx.lr = 0x82EFA2A0;
	sub_82EF9B30(ctx, base);
	// 82EFA2A0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA2A4: 4182000C  beq 0x82efa2b0
	if ctx.cr[0].eq {
	pc = 0x82EFA2B0; continue 'dispatch;
	}
	// 82EFA2A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFA2AC: 480035F5  bl 0x82efd8a0
	ctx.lr = 0x82EFA2B0;
	sub_82EFD8A0(ctx, base);
	// 82EFA2B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFA2B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFA2B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFA2BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFA2C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFA2C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFA2C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA2D0 size=76
    let mut pc: u32 = 0x82EFA2D0;
    'dispatch: loop {
        match pc {
            0x82EFA2D0 => {
    //   block [0x82EFA2D0..0x82EFA31C)
	// 82EFA2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFA2D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFA2DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA2E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFA2E4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFA2E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFA2EC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA2F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFA2F4: 4E800421  bctrl
	ctx.lr = 0x82EFA2F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFA2F8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA2FC: 4182000C  beq 0x82efa308
	if ctx.cr[0].eq {
	pc = 0x82EFA308; continue 'dispatch;
	}
	// 82EFA300: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA304: 4BFFF975  bl 0x82ef9c78
	ctx.lr = 0x82EFA308;
	sub_82EF9C78(ctx, base);
	// 82EFA308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFA30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFA310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFA314: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFA318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA320 size=360
    let mut pc: u32 = 0x82EFA320;
    'dispatch: loop {
        match pc {
            0x82EFA320 => {
    //   block [0x82EFA320..0x82EFA488)
	// 82EFA320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA324: 4BDAF0E1  bl 0x82ca9404
	ctx.lr = 0x82EFA328;
	sub_82CA93D0(ctx, base);
	// 82EFA328: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA32C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EFA330: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EFA334: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFA338: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA33C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFA340: 4E800421  bctrl
	ctx.lr = 0x82EFA344;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFA344: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA348: 41820010  beq 0x82efa358
	if ctx.cr[0].eq {
	pc = 0x82EFA358; continue 'dispatch;
	}
	// 82EFA34C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EFA350: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82EFA354: 4BDAF100  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82EFA358: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFA35C: 409A000C  bne cr6, 0x82efa368
	if !ctx.cr[6].eq {
	pc = 0x82EFA368; continue 'dispatch;
	}
	// 82EFA360: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFA364: 4BFFFFEC  b 0x82efa350
	pc = 0x82EFA350; continue 'dispatch;
	// 82EFA368: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFA36C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFA370: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EFA374: 4BFFF80D  bl 0x82ef9b80
	ctx.lr = 0x82EFA378;
	sub_82EF9B80(ctx, base);
	// 82EFA378: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82EFA37C: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82EFA380: 3D4082F0  lis r10, -0x7d10
	ctx.r[10].s64 = -2098200576;
	// 82EFA384: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82EFA388: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EFA38C: 388AA2D0  addi r4, r10, -0x5d30
	ctx.r[4].s64 = ctx.r[10].s64 + -23856;
	// 82EFA390: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EFA394: 4BFFF20D  bl 0x82ef95a0
	ctx.lr = 0x82EFA398;
	sub_82EF95A0(ctx, base);
	// 82EFA398: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA39C: 40820018  bne 0x82efa3b4
	if !ctx.cr[0].eq {
	pc = 0x82EFA3B4; continue 'dispatch;
	}
	// 82EFA3A0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EFA3A4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EFA3A8: 4BFFF861  bl 0x82ef9c08
	ctx.lr = 0x82EFA3AC;
	sub_82EF9C08(ctx, base);
	// 82EFA3AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFA3B0: 4BFFFFA0  b 0x82efa350
	pc = 0x82EFA350; continue 'dispatch;
	// 82EFA3B4: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFA3B8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EFA3BC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA3C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFA3C4: 4E800421  bctrl
	ctx.lr = 0x82EFA3C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFA3C8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA3CC: 41820020  beq 0x82efa3ec
	if ctx.cr[0].eq {
	pc = 0x82EFA3EC; continue 'dispatch;
	}
	// 82EFA3D0: 3D6082F0  lis r11, -0x7d10
	ctx.r[11].s64 = -2098200576;
	// 82EFA3D4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EFA3D8: 388BA2D0  addi r4, r11, -0x5d30
	ctx.r[4].s64 = ctx.r[11].s64 + -23856;
	// 82EFA3DC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EFA3E0: 4BFFF2B1  bl 0x82ef9690
	ctx.lr = 0x82EFA3E4;
	sub_82EF9690(ctx, base);
	// 82EFA3E4: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82EFA3E8: 4BFFFFBC  b 0x82efa3a4
	pc = 0x82EFA3A4; continue 'dispatch;
	// 82EFA3EC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82EFA3F0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EFA3F4: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82EFA3F8: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82EFA3FC: 419A000C  beq cr6, 0x82efa408
	if ctx.cr[6].eq {
	pc = 0x82EFA408; continue 'dispatch;
	}
	// 82EFA400: 48005171  bl 0x82eff570
	ctx.lr = 0x82EFA404;
	sub_82EFF570(ctx, base);
	// 82EFA404: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EFA408: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFA40C: 48000048  b 0x82efa454
	pc = 0x82EFA454; continue 'dispatch;
	// 82EFA410: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFA414: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EFA418: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA41C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFA420: 4E800421  bctrl
	ctx.lr = 0x82EFA424;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFA424: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA428: 40820040  bne 0x82efa468
	if !ctx.cr[0].eq {
	pc = 0x82EFA468; continue 'dispatch;
	}
	// 82EFA42C: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82EFA430: 419A0020  beq cr6, 0x82efa450
	if ctx.cr[6].eq {
	pc = 0x82EFA450; continue 'dispatch;
	}
	// 82EFA434: 4800513D  bl 0x82eff570
	ctx.lr = 0x82EFA438;
	sub_82EFF570(ctx, base);
	// 82EFA438: 546B003E  slwi r11, r3, 0
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFA43C: 57AA003E  slwi r10, r29, 0
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EFA440: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EFA444: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82EFA448: 40980024  bge cr6, 0x82efa46c
	if !ctx.cr[6].lt {
	pc = 0x82EFA46C; continue 'dispatch;
	}
	// 82EFA44C: 7FCBF850  subf r30, r11, r31
	ctx.r[30].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 82EFA450: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFA454: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EFA458: 4BFFEAE9  bl 0x82ef8f40
	ctx.lr = 0x82EFA45C;
	sub_82EF8F40(ctx, base);
	// 82EFA45C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA460: 4082FFB0  bne 0x82efa410
	if !ctx.cr[0].eq {
	pc = 0x82EFA410; continue 'dispatch;
	}
	// 82EFA464: 48000008  b 0x82efa46c
	pc = 0x82EFA46C; continue 'dispatch;
	// 82EFA468: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82EFA46C: 3D6082F0  lis r11, -0x7d10
	ctx.r[11].s64 = -2098200576;
	// 82EFA470: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EFA474: 388BA2D0  addi r4, r11, -0x5d30
	ctx.r[4].s64 = ctx.r[11].s64 + -23856;
	// 82EFA478: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EFA47C: 4BFFF215  bl 0x82ef9690
	ctx.lr = 0x82EFA480;
	sub_82EF9690(ctx, base);
	// 82EFA480: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 82EFA484: 4BFFFF20  b 0x82efa3a4
	pc = 0x82EFA3A4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA488 size=104
    let mut pc: u32 = 0x82EFA488;
    'dispatch: loop {
        match pc {
            0x82EFA488 => {
    //   block [0x82EFA488..0x82EFA4F0)
	// 82EFA488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA48C: 4BDAEF81  bl 0x82ca940c
	ctx.lr = 0x82EFA490;
	sub_82CA93D0(ctx, base);
	// 82EFA490: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA494: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFA498: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EFA49C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA4A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFA4A4: 4099003C  ble cr6, 0x82efa4e0
	if !ctx.cr[6].gt {
	pc = 0x82EFA4E0; continue 'dispatch;
	}
	// 82EFA4A8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EFA4AC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFA4B0: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EFA4B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFA4B8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA4BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFA4C0: 4E800421  bctrl
	ctx.lr = 0x82EFA4C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFA4C4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA4C8: 41820020  beq 0x82efa4e8
	if ctx.cr[0].eq {
	pc = 0x82EFA4E8; continue 'dispatch;
	}
	// 82EFA4CC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA4D0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82EFA4D4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82EFA4D8: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EFA4DC: 4198FFD0  blt cr6, 0x82efa4ac
	if ctx.cr[6].lt {
	pc = 0x82EFA4AC; continue 'dispatch;
	}
	// 82EFA4E0: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFA4E4: 4BFFF795  bl 0x82ef9c78
	ctx.lr = 0x82EFA4E8;
	sub_82EF9C78(ctx, base);
	// 82EFA4E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFA4EC: 4BDAEF70  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA4F0 size=424
    let mut pc: u32 = 0x82EFA4F0;
    'dispatch: loop {
        match pc {
            0x82EFA4F0 => {
    //   block [0x82EFA4F0..0x82EFA698)
	// 82EFA4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA4F4: 4BDAEF11  bl 0x82ca9404
	ctx.lr = 0x82EFA4F8;
	sub_82CA93D0(ctx, base);
	// 82EFA4F8: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA4FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFA500: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EFA504: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82EFA508: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EFA50C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFA510: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82EFA514: 4BFFE85D  bl 0x82ef8d70
	ctx.lr = 0x82EFA518;
	sub_82EF8D70(ctx, base);
	// 82EFA518: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EFA51C: 806100A0  lwz r3, 0xa0(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82EFA520: 4BFFE8C9  bl 0x82ef8de8
	ctx.lr = 0x82EFA524;
	sub_82EF8DE8(ctx, base);
	// 82EFA524: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA528: 41820020  beq 0x82efa548
	if ctx.cr[0].eq {
	pc = 0x82EFA548; continue 'dispatch;
	}
	// 82EFA52C: 806100A0  lwz r3, 0xa0(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82EFA530: 396100A4  addi r11, r1, 0xa4
	ctx.r[11].s64 = ctx.r[1].s64 + 164;
	// 82EFA534: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EFA538: 419A0008  beq cr6, 0x82efa540
	if ctx.cr[6].eq {
	pc = 0x82EFA540; continue 'dispatch;
	}
	// 82EFA53C: 48003365  bl 0x82efd8a0
	ctx.lr = 0x82EFA540;
	sub_82EFD8A0(ctx, base);
	// 82EFA540: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EFA544: 4800014C  b 0x82efa690
	pc = 0x82EFA690; continue 'dispatch;
	// 82EFA548: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFA54C: 409A0020  bne cr6, 0x82efa56c
	if !ctx.cr[6].eq {
	pc = 0x82EFA56C; continue 'dispatch;
	}
	// 82EFA550: 806100A0  lwz r3, 0xa0(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82EFA554: 396100A4  addi r11, r1, 0xa4
	ctx.r[11].s64 = ctx.r[1].s64 + 164;
	// 82EFA558: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EFA55C: 419A0008  beq cr6, 0x82efa564
	if ctx.cr[6].eq {
	pc = 0x82EFA564; continue 'dispatch;
	}
	// 82EFA560: 48003341  bl 0x82efd8a0
	ctx.lr = 0x82EFA564;
	sub_82EFD8A0(ctx, base);
	// 82EFA564: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFA568: 48000128  b 0x82efa690
	pc = 0x82EFA690; continue 'dispatch;
	// 82EFA56C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFA570: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFA574: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EFA578: 4BFFF609  bl 0x82ef9b80
	ctx.lr = 0x82EFA57C;
	sub_82EF9B80(ctx, base);
	// 82EFA57C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82EFA580: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82EFA584: 3D4082F0  lis r10, -0x7d10
	ctx.r[10].s64 = -2098200576;
	// 82EFA588: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82EFA58C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EFA590: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82EFA594: 388AA488  addi r4, r10, -0x5b78
	ctx.r[4].s64 = ctx.r[10].s64 + -23416;
	// 82EFA598: 816100A0  lwz r11, 0xa0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82EFA59C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82EFA5A0: 4BFFF1A1  bl 0x82ef9740
	ctx.lr = 0x82EFA5A4;
	sub_82EF9740(ctx, base);
	// 82EFA5A4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA5A8: 40820010  bne 0x82efa5b8
	if !ctx.cr[0].eq {
	pc = 0x82EFA5B8; continue 'dispatch;
	}
	// 82EFA5AC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EFA5B0: 4BFFF659  bl 0x82ef9c08
	ctx.lr = 0x82EFA5B4;
	sub_82EF9C08(ctx, base);
	// 82EFA5B4: 4BFFFF9C  b 0x82efa550
	pc = 0x82EFA550; continue 'dispatch;
	// 82EFA5B8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EFA5BC: 806100A0  lwz r3, 0xa0(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82EFA5C0: 4BFFE829  bl 0x82ef8de8
	ctx.lr = 0x82EFA5C4;
	sub_82EF8DE8(ctx, base);
	// 82EFA5C4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA5C8: 41820020  beq 0x82efa5e8
	if ctx.cr[0].eq {
	pc = 0x82EFA5E8; continue 'dispatch;
	}
	// 82EFA5CC: 3D6082F0  lis r11, -0x7d10
	ctx.r[11].s64 = -2098200576;
	// 82EFA5D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EFA5D4: 388BA488  addi r4, r11, -0x5b78
	ctx.r[4].s64 = ctx.r[11].s64 + -23416;
	// 82EFA5D8: 4BFFF209  bl 0x82ef97e0
	ctx.lr = 0x82EFA5DC;
	sub_82EF97E0(ctx, base);
	// 82EFA5DC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EFA5E0: 4BFFF629  bl 0x82ef9c08
	ctx.lr = 0x82EFA5E4;
	sub_82EF9C08(ctx, base);
	// 82EFA5E4: 4BFFFF48  b 0x82efa52c
	pc = 0x82EFA52C; continue 'dispatch;
	// 82EFA5E8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82EFA5EC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EFA5F0: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82EFA5F4: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82EFA5F8: 419A000C  beq cr6, 0x82efa604
	if ctx.cr[6].eq {
	pc = 0x82EFA604; continue 'dispatch;
	}
	// 82EFA5FC: 48004F75  bl 0x82eff570
	ctx.lr = 0x82EFA600;
	sub_82EFF570(ctx, base);
	// 82EFA600: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EFA604: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFA608: 48000040  b 0x82efa648
	pc = 0x82EFA648; continue 'dispatch;
	// 82EFA60C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EFA610: 806100A0  lwz r3, 0xa0(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82EFA614: 4BFFE7D5  bl 0x82ef8de8
	ctx.lr = 0x82EFA618;
	sub_82EF8DE8(ctx, base);
	// 82EFA618: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA61C: 40820040  bne 0x82efa65c
	if !ctx.cr[0].eq {
	pc = 0x82EFA65C; continue 'dispatch;
	}
	// 82EFA620: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82EFA624: 419A0020  beq cr6, 0x82efa644
	if ctx.cr[6].eq {
	pc = 0x82EFA644; continue 'dispatch;
	}
	// 82EFA628: 48004F49  bl 0x82eff570
	ctx.lr = 0x82EFA62C;
	sub_82EFF570(ctx, base);
	// 82EFA62C: 546B003E  slwi r11, r3, 0
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFA630: 57AA003E  slwi r10, r29, 0
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EFA634: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EFA638: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82EFA63C: 40980024  bge cr6, 0x82efa660
	if !ctx.cr[6].lt {
	pc = 0x82EFA660; continue 'dispatch;
	}
	// 82EFA640: 7FCBF850  subf r30, r11, r31
	ctx.r[30].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 82EFA644: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFA648: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EFA64C: 4BFFE8F5  bl 0x82ef8f40
	ctx.lr = 0x82EFA650;
	sub_82EF8F40(ctx, base);
	// 82EFA650: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA654: 4082FFB8  bne 0x82efa60c
	if !ctx.cr[0].eq {
	pc = 0x82EFA60C; continue 'dispatch;
	}
	// 82EFA658: 48000008  b 0x82efa660
	pc = 0x82EFA660; continue 'dispatch;
	// 82EFA65C: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82EFA660: 3D6082F0  lis r11, -0x7d10
	ctx.r[11].s64 = -2098200576;
	// 82EFA664: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EFA668: 388BA488  addi r4, r11, -0x5b78
	ctx.r[4].s64 = ctx.r[11].s64 + -23416;
	// 82EFA66C: 4BFFF175  bl 0x82ef97e0
	ctx.lr = 0x82EFA670;
	sub_82EF97E0(ctx, base);
	// 82EFA670: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EFA674: 4BFFF595  bl 0x82ef9c08
	ctx.lr = 0x82EFA678;
	sub_82EF9C08(ctx, base);
	// 82EFA678: 806100A0  lwz r3, 0xa0(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82EFA67C: 396100A4  addi r11, r1, 0xa4
	ctx.r[11].s64 = ctx.r[1].s64 + 164;
	// 82EFA680: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EFA684: 419A0008  beq cr6, 0x82efa68c
	if ctx.cr[6].eq {
	pc = 0x82EFA68C; continue 'dispatch;
	}
	// 82EFA688: 48003219  bl 0x82efd8a0
	ctx.lr = 0x82EFA68C;
	sub_82EFD8A0(ctx, base);
	// 82EFA68C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EFA690: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 82EFA694: 4BDAEDC0  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA698 size=124
    let mut pc: u32 = 0x82EFA698;
    'dispatch: loop {
        match pc {
            0x82EFA698 => {
    //   block [0x82EFA698..0x82EFA714)
	// 82EFA698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA69C: 4BDAED71  bl 0x82ca940c
	ctx.lr = 0x82EFA6A0;
	sub_82CA93D0(ctx, base);
	// 82EFA6A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA6A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFA6A8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EFA6AC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA6B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFA6B4: 40990058  ble cr6, 0x82efa70c
	if !ctx.cr[6].gt {
	pc = 0x82EFA70C; continue 'dispatch;
	}
	// 82EFA6B8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EFA6BC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFA6C0: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EFA6C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFA6C8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFA6CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFA6D0: 4E800421  bctrl
	ctx.lr = 0x82EFA6D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFA6D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFA6D8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA6DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFA6E0: 4E800421  bctrl
	ctx.lr = 0x82EFA6E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFA6E4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA6E8: 4082001C  bne 0x82efa704
	if !ctx.cr[0].eq {
	pc = 0x82EFA704; continue 'dispatch;
	}
	// 82EFA6EC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA6F0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82EFA6F4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82EFA6F8: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EFA6FC: 4198FFC0  blt cr6, 0x82efa6bc
	if ctx.cr[6].lt {
	pc = 0x82EFA6BC; continue 'dispatch;
	}
	// 82EFA700: 4800000C  b 0x82efa70c
	pc = 0x82EFA70C; continue 'dispatch;
	// 82EFA704: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFA708: 4BFFF571  bl 0x82ef9c78
	ctx.lr = 0x82EFA70C;
	sub_82EF9C78(ctx, base);
	// 82EFA70C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFA710: 4BDAED4C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA718 size=316
    let mut pc: u32 = 0x82EFA718;
    'dispatch: loop {
        match pc {
            0x82EFA718 => {
    //   block [0x82EFA718..0x82EFA854)
	// 82EFA718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA71C: 4BDAECE5  bl 0x82ca9400
	ctx.lr = 0x82EFA720;
	sub_82CA93D0(ctx, base);
	// 82EFA720: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA724: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EFA728: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EFA72C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EFA730: 4BFFE789  bl 0x82ef8eb8
	ctx.lr = 0x82EFA734;
	sub_82EF8EB8(ctx, base);
	// 82EFA734: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82EFA738: 409A000C  bne cr6, 0x82efa744
	if !ctx.cr[6].eq {
	pc = 0x82EFA744; continue 'dispatch;
	}
	// 82EFA73C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EFA740: 409A000C  bne cr6, 0x82efa74c
	if !ctx.cr[6].eq {
	pc = 0x82EFA74C; continue 'dispatch;
	}
	// 82EFA744: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82EFA748: 4BDAED08  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 82EFA74C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFA750: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFA754: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EFA758: 4BFFF429  bl 0x82ef9b80
	ctx.lr = 0x82EFA75C;
	sub_82EF9B80(ctx, base);
	// 82EFA75C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82EFA760: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82EFA764: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82EFA768: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82EFA76C: 3D4082F0  lis r10, -0x7d10
	ctx.r[10].s64 = -2098200576;
	// 82EFA770: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82EFA774: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EFA778: 9341005C  stw r26, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[26].u32 ) };
	// 82EFA77C: 388AA698  addi r4, r10, -0x5968
	ctx.r[4].s64 = ctx.r[10].s64 + -22888;
	// 82EFA780: 4BFFEFC1  bl 0x82ef9740
	ctx.lr = 0x82EFA784;
	sub_82EF9740(ctx, base);
	// 82EFA784: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA788: 40820018  bne 0x82efa7a0
	if !ctx.cr[0].eq {
	pc = 0x82EFA7A0; continue 'dispatch;
	}
	// 82EFA78C: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 82EFA790: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EFA794: 4BFFF475  bl 0x82ef9c08
	ctx.lr = 0x82EFA798;
	sub_82EF9C08(ctx, base);
	// 82EFA798: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFA79C: 4BFFFFA8  b 0x82efa744
	pc = 0x82EFA744; continue 'dispatch;
	// 82EFA7A0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EFA7A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFA7A8: 4BFFE711  bl 0x82ef8eb8
	ctx.lr = 0x82EFA7AC;
	sub_82EF8EB8(ctx, base);
	// 82EFA7AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFA7B0: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82EFA7B4: 419A0018  beq cr6, 0x82efa7cc
	if ctx.cr[6].eq {
	pc = 0x82EFA7CC; continue 'dispatch;
	}
	// 82EFA7B8: 3D6082F0  lis r11, -0x7d10
	ctx.r[11].s64 = -2098200576;
	// 82EFA7BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EFA7C0: 388BA698  addi r4, r11, -0x5968
	ctx.r[4].s64 = ctx.r[11].s64 + -22888;
	// 82EFA7C4: 4BFFF01D  bl 0x82ef97e0
	ctx.lr = 0x82EFA7C8;
	sub_82EF97E0(ctx, base);
	// 82EFA7C8: 4BFFFFC8  b 0x82efa790
	pc = 0x82EFA790; continue 'dispatch;
	// 82EFA7CC: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 82EFA7D0: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82EFA7D4: 419A000C  beq cr6, 0x82efa7e0
	if ctx.cr[6].eq {
	pc = 0x82EFA7E0; continue 'dispatch;
	}
	// 82EFA7D8: 48004D99  bl 0x82eff570
	ctx.lr = 0x82EFA7DC;
	sub_82EFF570(ctx, base);
	// 82EFA7DC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82EFA7E0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EFA7E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFA7E8: 4BFFE6D1  bl 0x82ef8eb8
	ctx.lr = 0x82EFA7EC;
	sub_82EF8EB8(ctx, base);
	// 82EFA7EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFA7F0: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82EFA7F4: 409AFFC4  bne cr6, 0x82efa7b8
	if !ctx.cr[6].eq {
	pc = 0x82EFA7B8; continue 'dispatch;
	}
	// 82EFA7F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFA7FC: 48000044  b 0x82efa840
	pc = 0x82EFA840; continue 'dispatch;
	// 82EFA800: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EFA804: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFA808: 4BFFE6B1  bl 0x82ef8eb8
	ctx.lr = 0x82EFA80C;
	sub_82EF8EB8(ctx, base);
	// 82EFA80C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFA810: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82EFA814: 409AFFA4  bne cr6, 0x82efa7b8
	if !ctx.cr[6].eq {
	pc = 0x82EFA7B8; continue 'dispatch;
	}
	// 82EFA818: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82EFA81C: 419A0020  beq cr6, 0x82efa83c
	if ctx.cr[6].eq {
	pc = 0x82EFA83C; continue 'dispatch;
	}
	// 82EFA820: 48004D51  bl 0x82eff570
	ctx.lr = 0x82EFA824;
	sub_82EFF570(ctx, base);
	// 82EFA824: 546B003E  slwi r11, r3, 0
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFA828: 574A003E  slwi r10, r26, 0
	ctx.r[10].u32 = ctx.r[26].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EFA82C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EFA830: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82EFA834: 4098FF84  bge cr6, 0x82efa7b8
	if !ctx.cr[6].lt {
	pc = 0x82EFA7B8; continue 'dispatch;
	}
	// 82EFA838: 7F6BF050  subf r27, r11, r30
	ctx.r[27].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 82EFA83C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EFA840: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EFA844: 4BFFE6FD  bl 0x82ef8f40
	ctx.lr = 0x82EFA848;
	sub_82EF8F40(ctx, base);
	// 82EFA848: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA84C: 4082FFB4  bne 0x82efa800
	if !ctx.cr[0].eq {
	pc = 0x82EFA800; continue 'dispatch;
	}
	// 82EFA850: 4BFFFF68  b 0x82efa7b8
	pc = 0x82EFA7B8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA858 size=112
    let mut pc: u32 = 0x82EFA858;
    'dispatch: loop {
        match pc {
            0x82EFA858 => {
    //   block [0x82EFA858..0x82EFA8C8)
	// 82EFA858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA85C: 4BDAEBB1  bl 0x82ca940c
	ctx.lr = 0x82EFA860;
	sub_82CA93D0(ctx, base);
	// 82EFA860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA864: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFA868: 3BDF001C  addi r30, r31, 0x1c
	ctx.r[30].s64 = ctx.r[31].s64 + 28;
	// 82EFA86C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFA870: 480002E9  bl 0x82efab58
	ctx.lr = 0x82EFA874;
	sub_82EFAB58(ctx, base);
	// 82EFA874: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFA878: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EFA87C: 997F0018  stb r11, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 82EFA880: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 82EFA884: 9BBF0019  stb r29, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[29].u8 ) };
	// 82EFA888: 48000789  bl 0x82efb010
	ctx.lr = 0x82EFA88C;
	sub_82EFB010(ctx, base);
	// 82EFA88C: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82EFA890: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EFA894: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFA898: 4BFFF221  bl 0x82ef9ab8
	ctx.lr = 0x82EFA89C;
	sub_82EF9AB8(ctx, base);
	// 82EFA89C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFA8A0: 48001201  bl 0x82efbaa0
	ctx.lr = 0x82EFA8A4;
	sub_82EFBAA0(ctx, base);
	// 82EFA8A4: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFA8A8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFA8AC: 419A0014  beq cr6, 0x82efa8c0
	if ctx.cr[6].eq {
	pc = 0x82EFA8C0; continue 'dispatch;
	}
	// 82EFA8B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFA8B4: 4BFFF0BD  bl 0x82ef9970
	ctx.lr = 0x82EFA8B8;
	sub_82EF9970(ctx, base);
	// 82EFA8B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFA8BC: 4BFFF18D  bl 0x82ef9a48
	ctx.lr = 0x82EFA8C0;
	sub_82EF9A48(ctx, base);
	// 82EFA8C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFA8C4: 4BDAEB98  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA8C8 size=76
    let mut pc: u32 = 0x82EFA8C8;
    'dispatch: loop {
        match pc {
            0x82EFA8C8 => {
    //   block [0x82EFA8C8..0x82EFA914)
	// 82EFA8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFA8D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFA8D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFA8D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA8DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFA8E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFA8E4: 4BFFF785  bl 0x82efa068
	ctx.lr = 0x82EFA8E8;
	sub_82EFA068(ctx, base);
	// 82EFA8E8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFA8EC: 4182000C  beq 0x82efa8f8
	if ctx.cr[0].eq {
	pc = 0x82EFA8F8; continue 'dispatch;
	}
	// 82EFA8F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFA8F4: 48002FAD  bl 0x82efd8a0
	ctx.lr = 0x82EFA8F8;
	sub_82EFD8A0(ctx, base);
	// 82EFA8F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFA8FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFA900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFA904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFA908: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFA90C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFA910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFA918 size=8
    let mut pc: u32 = 0x82EFA918;
    'dispatch: loop {
        match pc {
            0x82EFA918 => {
    //   block [0x82EFA918..0x82EFA920)
	// 82EFA918: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82EFA91C: 4BFFF8DC  b 0x82efa1f8
	sub_82EFA1F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFA920 size=8
    let mut pc: u32 = 0x82EFA920;
    'dispatch: loop {
        match pc {
            0x82EFA920 => {
    //   block [0x82EFA920..0x82EFA928)
	// 82EFA920: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82EFA924: 4BFFF84C  b 0x82efa170
	sub_82EFA170(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA928 size=56
    let mut pc: u32 = 0x82EFA928;
    'dispatch: loop {
        match pc {
            0x82EFA928 => {
    //   block [0x82EFA928..0x82EFA960)
	// 82EFA928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFA930: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA934: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82EFA938: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82EFA93C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EFA940: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EFA944: 4BFFFDD5  bl 0x82efa718
	ctx.lr = 0x82EFA948;
	sub_82EFA718(ctx, base);
	// 82EFA948: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82EFA94C: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82EFA950: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFA954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFA958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFA95C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFA960 size=24
    let mut pc: u32 = 0x82EFA960;
    'dispatch: loop {
        match pc {
            0x82EFA960 => {
    //   block [0x82EFA960..0x82EFA978)
	// 82EFA960: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA964: 386B0014  addi r3, r11, 0x14
	ctx.r[3].s64 = ctx.r[11].s64 + 20;
	// 82EFA968: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFA96C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFA970: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFA974: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFA978 size=24
    let mut pc: u32 = 0x82EFA978;
    'dispatch: loop {
        match pc {
            0x82EFA978 => {
    //   block [0x82EFA978..0x82EFA990)
	// 82EFA978: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA97C: 386B0014  addi r3, r11, 0x14
	ctx.r[3].s64 = ctx.r[11].s64 + 20;
	// 82EFA980: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFA984: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFA988: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFA98C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFA990 size=24
    let mut pc: u32 = 0x82EFA990;
    'dispatch: loop {
        match pc {
            0x82EFA990 => {
    //   block [0x82EFA990..0x82EFA9A8)
	// 82EFA990: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFA994: 386B0014  addi r3, r11, 0x14
	ctx.r[3].s64 = ctx.r[11].s64 + 20;
	// 82EFA998: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFA99C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFA9A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFA9A4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFA9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFA9A8 size=124
    let mut pc: u32 = 0x82EFA9A8;
    'dispatch: loop {
        match pc {
            0x82EFA9A8 => {
    //   block [0x82EFA9A8..0x82EFAA24)
	// 82EFA9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFA9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFA9B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFA9B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFA9B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFA9BC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFA9C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFA9C4: 394AD02C  addi r10, r10, -0x2fd4
	ctx.r[10].s64 = ctx.r[10].s64 + -12244;
	// 82EFA9C8: 54A9063F  clrlwi. r9, r5, 0x18
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EFA9CC: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EFA9D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFA9D4: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82EFA9D8: 909F0010  stw r4, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 82EFA9DC: 98BF0004  stb r5, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 82EFA9E0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFA9E4: 41820014  beq 0x82efa9f8
	if ctx.cr[0].eq {
	pc = 0x82EFA9F8; continue 'dispatch;
	}
	// 82EFA9E8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFA9EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFA9F0: 48110F49  bl 0x8300b938
	ctx.lr = 0x82EFA9F4;
	sub_8300B938(ctx, base);
	// 82EFA9F4: 48000014  b 0x82efaa08
	pc = 0x82EFAA08; continue 'dispatch;
	// 82EFA9F8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82EFA9FC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EFAA00: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EFAA04: 4BDCC4AD  bl 0x82cc6eb0
	ctx.lr = 0x82EFAA08;
	sub_82CC6EB0(ctx, base);
	// 82EFAA08: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EFAA0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFAA10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFAA14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFAA18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFAA1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFAA20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFAA28 size=72
    let mut pc: u32 = 0x82EFAA28;
    'dispatch: loop {
        match pc {
            0x82EFAA28 => {
    //   block [0x82EFAA28..0x82EFAA70)
	// 82EFAA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFAA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFAA30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFAA34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFAA38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFAA3C: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82EFAA40: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFAA44: 4B29BF85  bl 0x821969c8
	ctx.lr = 0x82EFAA48;
	sub_821969C8(ctx, base);
	// 82EFAA48: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFAA4C: 40820010  bne 0x82efaa5c
	if !ctx.cr[0].eq {
	pc = 0x82EFAA5C; continue 'dispatch;
	}
	// 82EFAA50: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFAA54: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EFAA58: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFAA5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFAA60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFAA64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFAA68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFAA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFAA70 size=84
    let mut pc: u32 = 0x82EFAA70;
    'dispatch: loop {
        match pc {
            0x82EFAA70 => {
    //   block [0x82EFAA70..0x82EFAAC4)
	// 82EFAA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFAA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFAA78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFAA7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFAA80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFAA84: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFAA88: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFAA8C: 4B29BF3D  bl 0x821969c8
	ctx.lr = 0x82EFAA90;
	sub_821969C8(ctx, base);
	// 82EFAA90: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFAA94: 4182000C  beq 0x82efaaa0
	if ctx.cr[0].eq {
	pc = 0x82EFAAA0; continue 'dispatch;
	}
	// 82EFAA98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFAA9C: 48000014  b 0x82efaab0
	pc = 0x82EFAAB0; continue 'dispatch;
	// 82EFAAA0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFAAA4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EFAAA8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EFAAAC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFAAB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFAAB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFAAB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFAABC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFAAC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFAAC8 size=140
    let mut pc: u32 = 0x82EFAAC8;
    'dispatch: loop {
        match pc {
            0x82EFAAC8 => {
    //   block [0x82EFAAC8..0x82EFAB54)
	// 82EFAAC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFAACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFAAD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFAAD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFAAD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFAADC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFAAE0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82EFAAE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFAAE8: 4BFFEDA1  bl 0x82ef9888
	ctx.lr = 0x82EFAAEC;
	sub_82EF9888(ctx, base);
	// 82EFAAEC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFAAF0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFAAF4: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82EFAAF8: 396BC544  addi r11, r11, -0x3abc
	ctx.r[11].s64 = ctx.r[11].s64 + -15036;
	// 82EFAAFC: 394AD060  addi r10, r10, -0x2fa0
	ctx.r[10].s64 = ctx.r[10].s64 + -12192;
	// 82EFAB00: 3929D04C  addi r9, r9, -0x2fb4
	ctx.r[9].s64 = ctx.r[9].s64 + -12212;
	// 82EFAB04: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EFAB08: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EFAB0C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82EFAB10: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82EFAB14: 48002D4D  bl 0x82efd860
	ctx.lr = 0x82EFAB18;
	sub_82EFD860(ctx, base);
	// 82EFAB18: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFAB1C: 41820014  beq 0x82efab30
	if ctx.cr[0].eq {
	pc = 0x82EFAB30; continue 'dispatch;
	}
	// 82EFAB20: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EFAB24: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFAB28: 4BFFFE81  bl 0x82efa9a8
	ctx.lr = 0x82EFAB2C;
	sub_82EFA9A8(ctx, base);
	// 82EFAB2C: 48000008  b 0x82efab34
	pc = 0x82EFAB34; continue 'dispatch;
	// 82EFAB30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFAB34: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82EFAB38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFAB3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFAB40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFAB44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFAB48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFAB4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFAB50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFAB58 size=8
    let mut pc: u32 = 0x82EFAB58;
    'dispatch: loop {
        match pc {
            0x82EFAB58 => {
    //   block [0x82EFAB58..0x82EFAB60)
	// 82EFAB58: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFAB5C: 4BFFFECC  b 0x82efaa28
	sub_82EFAA28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFAB60 size=8
    let mut pc: u32 = 0x82EFAB60;
    'dispatch: loop {
        match pc {
            0x82EFAB60 => {
    //   block [0x82EFAB60..0x82EFAB68)
	// 82EFAB60: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFAB64: 4BFFFF0C  b 0x82efaa70
	sub_82EFAA70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFAB68 size=20
    let mut pc: u32 = 0x82EFAB68;
    'dispatch: loop {
        match pc {
            0x82EFAB68 => {
    //   block [0x82EFAB68..0x82EFAB7C)
	// 82EFAB68: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFAB6C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFAB70: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82EFAB74: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82EFAB78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFAB80 size=8
    let mut pc: u32 = 0x82EFAB80;
    'dispatch: loop {
        match pc {
            0x82EFAB80 => {
    //   block [0x82EFAB80..0x82EFAB88)
	// 82EFAB80: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFAB84: 4BFFFEEC  b 0x82efaa70
	sub_82EFAA70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFAB88 size=68
    let mut pc: u32 = 0x82EFAB88;
    'dispatch: loop {
        match pc {
            0x82EFAB88 => {
    //   block [0x82EFAB88..0x82EFABCC)
	// 82EFAB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFAB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFAB90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFAB94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFAB98: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFAB9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFABA0: 4BFFE1A1  bl 0x82ef8d40
	ctx.lr = 0x82EFABA4;
	sub_82EF8D40(ctx, base);
	// 82EFABA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFABA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFABAC: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EFABB0: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82EFABB4: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82EFABB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFABBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFABC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFABC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFABC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFABD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFABD0 size=88
    let mut pc: u32 = 0x82EFABD0;
    'dispatch: loop {
        match pc {
            0x82EFABD0 => {
    //   block [0x82EFABD0..0x82EFAC28)
	// 82EFABD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFABD4: 4BDAE839  bl 0x82ca940c
	ctx.lr = 0x82EFABD8;
	sub_82CA93D0(ctx, base);
	// 82EFABD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFABDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFABE0: 83DF001C  lwz r30, 0x1c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFABE4: 4800001C  b 0x82efac00
	pc = 0x82EFAC00; continue 'dispatch;
	// 82EFABE8: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82EFABEC: 83DE0004  lwz r30, 4(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFABF0: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFABF4: 4BDC7BBD  bl 0x82cc27b0
	ctx.lr = 0x82EFABF8;
	sub_82CC27B0(ctx, base);
	// 82EFABF8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFABFC: 48002CA5  bl 0x82efd8a0
	ctx.lr = 0x82EFAC00;
	sub_82EFD8A0(ctx, base);
	// 82EFAC00: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EFAC04: 409AFFE4  bne cr6, 0x82efabe8
	if !ctx.cr[6].eq {
	pc = 0x82EFABE8; continue 'dispatch;
	}
	// 82EFAC08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFAC0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFAC10: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EFAC14: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82EFAC18: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82EFAC1C: 4800041D  bl 0x82efb038
	ctx.lr = 0x82EFAC20;
	sub_82EFB038(ctx, base);
	// 82EFAC20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFAC24: 4BDAE838  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFAC28 size=112
    let mut pc: u32 = 0x82EFAC28;
    'dispatch: loop {
        match pc {
            0x82EFAC28 => {
    //   block [0x82EFAC28..0x82EFAC98)
	// 82EFAC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFAC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFAC30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFAC34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFAC38: 83E3001C  lwz r31, 0x1c(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFAC3C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFAC40: 419A0010  beq cr6, 0x82efac50
	if ctx.cr[6].eq {
	pc = 0x82EFAC50; continue 'dispatch;
	}
	// 82EFAC44: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFAC48: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EFAC4C: 48000034  b 0x82efac80
	pc = 0x82EFAC80; continue 'dispatch;
	// 82EFAC50: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82EFAC54: 48002C0D  bl 0x82efd860
	ctx.lr = 0x82EFAC58;
	sub_82EFD860(ctx, base);
	// 82EFAC58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFAC5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFAC60: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82EFAC64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFAC68: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EFAC6C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EFAC70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFAC74: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFAC78: 4BDC8069  bl 0x82cc2ce0
	ctx.lr = 0x82EFAC7C;
	sub_82CC2CE0(ctx, base);
	// 82EFAC7C: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EFAC80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFAC84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFAC88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFAC8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFAC90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFAC94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFAC98 size=80
    let mut pc: u32 = 0x82EFAC98;
    'dispatch: loop {
        match pc {
            0x82EFAC98 => {
    //   block [0x82EFAC98..0x82EFACE8)
	// 82EFAC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFAC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFACA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFACA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFACA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFACAC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFACB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFACB4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFACB8: 4B292F61  bl 0x8218dc18
	ctx.lr = 0x82EFACBC;
	sub_8218DC18(ctx, base);
	// 82EFACBC: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFACC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFACC4: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFACC8: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EFACCC: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82EFACD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFACD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFACD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFACDC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFACE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFACE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFACE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFACE8 size=8
    let mut pc: u32 = 0x82EFACE8;
    'dispatch: loop {
        match pc {
            0x82EFACE8 => {
    //   block [0x82EFACE8..0x82EFACF0)
	// 82EFACE8: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFACEC: 48000010  b 0x82efacfc
	sub_82EFACF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFACF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFACF0 size=24
    let mut pc: u32 = 0x82EFACF0;
    'dispatch: loop {
        match pc {
            0x82EFACF0 => {
    //   block [0x82EFACF0..0x82EFAD08)
	// 82EFACF0: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82EFACF4: 419A0014  beq cr6, 0x82efad08
	if ctx.cr[6].eq {
		sub_82EFAD08(ctx, base);
		return;
	}
	// 82EFACF8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFACFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFAD00: 409AFFF0  bne cr6, 0x82efacf0
	if !ctx.cr[6].eq {
	pc = 0x82EFACF0; continue 'dispatch;
	}
	// 82EFAD04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFAD08 size=24
    let mut pc: u32 = 0x82EFAD08;
    'dispatch: loop {
        match pc {
            0x82EFAD08 => {
    //   block [0x82EFAD08..0x82EFAD20)
	// 82EFAD08: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFAD0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFAD10: 419A0010  beq cr6, 0x82efad20
	if ctx.cr[6].eq {
		sub_82EFAD20(ctx, base);
		return;
	}
	// 82EFAD14: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFAD18: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EFAD1C: 4800000C  b 0x82efad28
	sub_82EFAD20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFAD20 size=32
    let mut pc: u32 = 0x82EFAD20;
    'dispatch: loop {
        match pc {
            0x82EFAD20 => {
    //   block [0x82EFAD20..0x82EFAD40)
	// 82EFAD20: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFAD24: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82EFAD28: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFAD2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFAD30: 419A0010  beq cr6, 0x82efad40
	if ctx.cr[6].eq {
		sub_82EFAD40(ctx, base);
		return;
	}
	// 82EFAD34: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFAD38: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EFAD3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFAD40 size=12
    let mut pc: u32 = 0x82EFAD40;
    'dispatch: loop {
        match pc {
            0x82EFAD40 => {
    //   block [0x82EFAD40..0x82EFAD4C)
	// 82EFAD40: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFAD44: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82EFAD48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFAD50 size=348
    let mut pc: u32 = 0x82EFAD50;
    'dispatch: loop {
        match pc {
            0x82EFAD50 => {
    //   block [0x82EFAD50..0x82EFAEAC)
	// 82EFAD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFAD54: 4BDAE6A9  bl 0x82ca93fc
	ctx.lr = 0x82EFAD58;
	sub_82CA93D0(ctx, base);
	// 82EFAD58: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFAD5C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EFAD60: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EFAD64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFAD68: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82EFAD6C: 7FB9EB78  mr r25, r29
	ctx.r[25].u64 = ctx.r[29].u64;
	// 82EFAD70: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFAD74: 836B0008  lwz r27, 8(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFAD78: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82EFAD7C: 409A000C  bne cr6, 0x82efad88
	if !ctx.cr[6].eq {
	pc = 0x82EFAD88; continue 'dispatch;
	}
	// 82EFAD80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFAD84: 48000120  b 0x82efaea4
	pc = 0x82EFAEA4; continue 'dispatch;
	// 82EFAD88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFAD8C: 483BEBD9  bl 0x832b9964
	ctx.lr = 0x82EFAD90;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EFAD90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFAD94: 4BFFFE95  bl 0x82efac28
	ctx.lr = 0x82EFAD98;
	sub_82EFAC28(ctx, base);
	// 82EFAD98: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFAD9C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFADA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFADA4: 419A0018  beq cr6, 0x82efadbc
	if ctx.cr[6].eq {
	pc = 0x82EFADBC; continue 'dispatch;
	}
	// 82EFADA8: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFADAC: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFADB0: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82EFADB4: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82EFADB8: 48000010  b 0x82efadc8
	pc = 0x82EFADC8; continue 'dispatch;
	// 82EFADBC: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82EFADC0: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82EFADC4: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82EFADC8: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82EFADCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFADD0: 483BEB85  bl 0x832b9954
	ctx.lr = 0x82EFADD4;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EFADD4: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFADD8: 894B0004  lbz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFADDC: 93AB0008  stw r29, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82EFADE0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFADE4: 41820028  beq 0x82efae0c
	if ctx.cr[0].eq {
	pc = 0x82EFAE0C; continue 'dispatch;
	}
	// 82EFADE8: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82EFADEC: 419A0034  beq cr6, 0x82efae20
	if ctx.cr[6].eq {
	pc = 0x82EFAE20; continue 'dispatch;
	}
	// 82EFADF0: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 82EFADF4: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFADF8: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFADFC: 48110BC5  bl 0x8300b9c0
	ctx.lr = 0x82EFAE00;
	sub_8300B9C0(ctx, base);
	// 82EFAE00: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82EFAE04: 4082FFF0  bne 0x82efadf4
	if !ctx.cr[0].eq {
	pc = 0x82EFADF4; continue 'dispatch;
	}
	// 82EFAE08: 48000018  b 0x82efae20
	pc = 0x82EFAE20; continue 'dispatch;
	// 82EFAE0C: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFAE10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFAE14: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EFAE18: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFAE1C: 4BDCC12D  bl 0x82cc6f48
	ctx.lr = 0x82EFAE20;
	sub_82CC6F48(ctx, base);
	// 82EFAE20: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EFAE24: 4BFFEC0D  bl 0x82ef9a30
	ctx.lr = 0x82EFAE28;
	sub_82EF9A30(ctx, base);
	// 82EFAE28: 2F1AFFFF  cmpwi cr6, r26, -1
	ctx.cr[6].compare_i32(ctx.r[26].s32, -1, &mut ctx.xer);
	// 82EFAE2C: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82EFAE30: 419A0008  beq cr6, 0x82efae38
	if ctx.cr[6].eq {
	pc = 0x82EFAE38; continue 'dispatch;
	}
	// 82EFAE34: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82EFAE38: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFAE3C: 4B29BB8D  bl 0x821969c8
	ctx.lr = 0x82EFAE40;
	sub_821969C8(ctx, base);
	// 82EFAE40: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EFAE44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFAE48: 483BEB1D  bl 0x832b9964
	ctx.lr = 0x82EFAE4C;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EFAE4C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EFAE50: 419A001C  beq cr6, 0x82efae6c
	if ctx.cr[6].eq {
	pc = 0x82EFAE6C; continue 'dispatch;
	}
	// 82EFAE54: 2B1D0080  cmplwi cr6, r29, 0x80
	ctx.cr[6].compare_u32(ctx.r[29].u32, 128 as u32, &mut ctx.xer);
	// 82EFAE58: 419A0014  beq cr6, 0x82efae6c
	if ctx.cr[6].eq {
	pc = 0x82EFAE6C; continue 'dispatch;
	}
	// 82EFAE5C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFAE60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFAE64: 4BFFFE85  bl 0x82eface8
	ctx.lr = 0x82EFAE68;
	sub_82EFACE8(ctx, base);
	// 82EFAE68: 48000008  b 0x82efae70
	pc = 0x82EFAE70; continue 'dispatch;
	// 82EFAE6C: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 82EFAE70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFAE74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFAE78: 4BFFFE21  bl 0x82efac98
	ctx.lr = 0x82EFAE7C;
	sub_82EFAC98(ctx, base);
	// 82EFAE7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFAE80: 483BEAD5  bl 0x832b9954
	ctx.lr = 0x82EFAE84;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EFAE84: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82EFAE88: 419A0018  beq cr6, 0x82efaea0
	if ctx.cr[6].eq {
	pc = 0x82EFAEA0; continue 'dispatch;
	}
	// 82EFAE8C: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 82EFAE90: 807C0018  lwz r3, 0x18(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFAE94: 4BFFFB95  bl 0x82efaa28
	ctx.lr = 0x82EFAE98;
	sub_82EFAA28(ctx, base);
	// 82EFAE98: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFAE9C: 4082FFF4  bne 0x82efae90
	if !ctx.cr[0].eq {
	pc = 0x82EFAE90; continue 'dispatch;
	}
	// 82EFAEA0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82EFAEA4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EFAEA8: 4BDAE5A4  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFAEB0 size=116
    let mut pc: u32 = 0x82EFAEB0;
    'dispatch: loop {
        match pc {
            0x82EFAEB0 => {
    //   block [0x82EFAEB0..0x82EFAF24)
	// 82EFAEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFAEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFAEB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFAEBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFAEC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFAEC4: 483BEAA1  bl 0x832b9964
	ctx.lr = 0x82EFAEC8;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EFAEC8: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFAECC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFAED0: 41820038  beq 0x82efaf08
	if ctx.cr[0].eq {
	pc = 0x82EFAF08; continue 'dispatch;
	}
	// 82EFAED4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFAED8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EFAEDC: 419A0018  beq cr6, 0x82efaef4
	if ctx.cr[6].eq {
	pc = 0x82EFAEF4; continue 'dispatch;
	}
	// 82EFAEE0: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EFAEE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82EFAEE8: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82EFAEEC: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82EFAEF0: 48000010  b 0x82efaf00
	pc = 0x82EFAF00; continue 'dispatch;
	// 82EFAEF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EFAEF8: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82EFAEFC: 915F0024  stw r10, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 82EFAF00: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFAF04: 4B3CE485  bl 0x822c9388
	ctx.lr = 0x82EFAF08;
	sub_822C9388(ctx, base);
	// 82EFAF08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFAF0C: 483BEA49  bl 0x832b9954
	ctx.lr = 0x82EFAF10;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EFAF10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFAF14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFAF18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFAF1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFAF20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFAF28 size=132
    let mut pc: u32 = 0x82EFAF28;
    'dispatch: loop {
        match pc {
            0x82EFAF28 => {
    //   block [0x82EFAF28..0x82EFAFAC)
	// 82EFAF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFAF2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFAF30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFAF34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFAF38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFAF3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFAF40: 483BEA25  bl 0x832b9964
	ctx.lr = 0x82EFAF44;
	// extern call 0x832B9964 → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82EFAF44: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EFAF48: 4800000C  b 0x82efaf54
	pc = 0x82EFAF54; continue 'dispatch;
	// 82EFAF4C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFAF50: 4B3CE439  bl 0x822c9388
	ctx.lr = 0x82EFAF54;
	sub_822C9388(ctx, base);
	// 82EFAF54: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFAF58: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFAF5C: 41820028  beq 0x82efaf84
	if ctx.cr[0].eq {
	pc = 0x82EFAF84; continue 'dispatch;
	}
	// 82EFAF60: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFAF64: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EFAF68: 419A0014  beq cr6, 0x82efaf7c
	if ctx.cr[6].eq {
	pc = 0x82EFAF7C; continue 'dispatch;
	}
	// 82EFAF6C: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EFAF70: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82EFAF74: 93CA0008  stw r30, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82EFAF78: 4800000C  b 0x82efaf84
	pc = 0x82EFAF84; continue 'dispatch;
	// 82EFAF7C: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82EFAF80: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82EFAF84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFAF88: 409AFFC4  bne cr6, 0x82efaf4c
	if !ctx.cr[6].eq {
	pc = 0x82EFAF4C; continue 'dispatch;
	}
	// 82EFAF8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFAF90: 483BE9C5  bl 0x832b9954
	ctx.lr = 0x82EFAF94;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EFAF94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFAF98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFAF9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFAFA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFAFA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFAFA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFAFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFAFB0 size=76
    let mut pc: u32 = 0x82EFAFB0;
    'dispatch: loop {
        match pc {
            0x82EFAFB0 => {
    //   block [0x82EFAFB0..0x82EFAFFC)
	// 82EFAFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFAFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFAFB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFAFBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFAFC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFAFC4: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 82EFAFC8: 48002899  bl 0x82efd860
	ctx.lr = 0x82EFAFCC;
	sub_82EFD860(ctx, base);
	// 82EFAFCC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFAFD0: 4182000C  beq 0x82efafdc
	if ctx.cr[0].eq {
	pc = 0x82EFAFDC; continue 'dispatch;
	}
	// 82EFAFD4: 4BFFFBB5  bl 0x82efab88
	ctx.lr = 0x82EFAFD8;
	sub_82EFAB88(ctx, base);
	// 82EFAFD8: 48000008  b 0x82efafe0
	pc = 0x82EFAFE0; continue 'dispatch;
	// 82EFAFDC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFAFE0: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EFAFE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFAFE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFAFEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFAFF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFAFF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFAFF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB000 size=8
    let mut pc: u32 = 0x82EFB000;
    'dispatch: loop {
        match pc {
            0x82EFB000 => {
    //   block [0x82EFB000..0x82EFB008)
	// 82EFB000: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB004: 4BFFFD4C  b 0x82efad50
	sub_82EFAD50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB008 size=8
    let mut pc: u32 = 0x82EFB008;
    'dispatch: loop {
        match pc {
            0x82EFB008 => {
    //   block [0x82EFB008..0x82EFB010)
	// 82EFB008: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB00C: 4BFFFEA4  b 0x82efaeb0
	sub_82EFAEB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB010 size=8
    let mut pc: u32 = 0x82EFB010;
    'dispatch: loop {
        match pc {
            0x82EFB010 => {
    //   block [0x82EFB010..0x82EFB018)
	// 82EFB010: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB014: 4BFFFF14  b 0x82efaf28
	sub_82EFAF28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB018 size=24
    let mut pc: u32 = 0x82EFB018;
    'dispatch: loop {
        match pc {
            0x82EFB018 => {
    //   block [0x82EFB018..0x82EFB030)
	// 82EFB018: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EFB01C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFB020: 419A0010  beq cr6, 0x82efb030
	if ctx.cr[6].eq {
		sub_82EFB030(ctx, base);
		return;
	}
	// 82EFB024: 80830034  lwz r4, 0x34(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82EFB028: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFB02C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB030 size=8
    let mut pc: u32 = 0x82EFB030;
    'dispatch: loop {
        match pc {
            0x82EFB030 => {
    //   block [0x82EFB030..0x82EFB038)
	// 82EFB030: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFB034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB038 size=4
    let mut pc: u32 = 0x82EFB038;
    'dispatch: loop {
        match pc {
            0x82EFB038 => {
    //   block [0x82EFB038..0x82EFB03C)
	// 82EFB038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB040 size=40
    let mut pc: u32 = 0x82EFB040;
    'dispatch: loop {
        match pc {
            0x82EFB040 => {
    //   block [0x82EFB040..0x82EFB068)
	// 82EFB040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFB048: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB04C: 1C6403E8  mulli r3, r4, 0x3e8
	ctx.r[3].s64 = ctx.r[4].s64 * 1000;
	// 82EFB050: 4BDC7E31  bl 0x82cc2e80
	ctx.lr = 0x82EFB054;
	sub_82CC2E80(ctx, base);
	// 82EFB054: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EFB058: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFB05C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFB060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFB064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB068 size=40
    let mut pc: u32 = 0x82EFB068;
    'dispatch: loop {
        match pc {
            0x82EFB068 => {
    //   block [0x82EFB068..0x82EFB090)
	// 82EFB068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFB070: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB074: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82EFB078: 4BDC7E09  bl 0x82cc2e80
	ctx.lr = 0x82EFB07C;
	sub_82CC2E80(ctx, base);
	// 82EFB07C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EFB080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFB084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFB088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFB08C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB090 size=68
    let mut pc: u32 = 0x82EFB090;
    'dispatch: loop {
        match pc {
            0x82EFB090 => {
    //   block [0x82EFB090..0x82EFB0D4)
	// 82EFB090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFB098: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFB09C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB0A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFB0A4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFB0A8: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EFB0AC: 396BC544  addi r11, r11, -0x3abc
	ctx.r[11].s64 = ctx.r[11].s64 + -15036;
	// 82EFB0B0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFB0B4: 41820008  beq 0x82efb0bc
	if ctx.cr[0].eq {
	pc = 0x82EFB0BC; continue 'dispatch;
	}
	// 82EFB0B8: 4B94A6F9  bl 0x828457b0
	ctx.lr = 0x82EFB0BC;
	sub_828457B0(ctx, base);
	// 82EFB0BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFB0C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFB0C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFB0C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFB0CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFB0D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB0D8 size=104
    let mut pc: u32 = 0x82EFB0D8;
    'dispatch: loop {
        match pc {
            0x82EFB0D8 => {
    //   block [0x82EFB0D8..0x82EFB140)
	// 82EFB0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB0DC: 4BDAE32D  bl 0x82ca9408
	ctx.lr = 0x82EFB0E0;
	sub_82CA93D0(ctx, base);
	// 82EFB0E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB0E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFB0E8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFB0EC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFB0F0: 392BD060  addi r9, r11, -0x2fa0
	ctx.r[9].s64 = ctx.r[11].s64 + -12192;
	// 82EFB0F4: 394AD04C  addi r10, r10, -0x2fb4
	ctx.r[10].s64 = ctx.r[10].s64 + -12212;
	// 82EFB0F8: 83DF0018  lwz r30, 0x18(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFB0FC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFB100: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EFB104: 3BBF0014  addi r29, r31, 0x14
	ctx.r[29].s64 = ctx.r[31].s64 + 20;
	// 82EFB108: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82EFB10C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EFB110: 3B8BC544  addi r28, r11, -0x3abc
	ctx.r[28].s64 = ctx.r[11].s64 + -15036;
	// 82EFB114: 419A0018  beq cr6, 0x82efb12c
	if ctx.cr[6].eq {
	pc = 0x82EFB12C; continue 'dispatch;
	}
	// 82EFB118: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB11C: 4BDC7695  bl 0x82cc27b0
	ctx.lr = 0x82EFB120;
	sub_82CC27B0(ctx, base);
	// 82EFB120: 939E000C  stw r28, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82EFB124: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFB128: 48002779  bl 0x82efd8a0
	ctx.lr = 0x82EFB12C;
	sub_82EFD8A0(ctx, base);
	// 82EFB12C: 939D0000  stw r28, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82EFB130: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFB134: 4BFFE9FD  bl 0x82ef9b30
	ctx.lr = 0x82EFB138;
	sub_82EF9B30(ctx, base);
	// 82EFB138: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFB13C: 4BDAE31C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB140 size=64
    let mut pc: u32 = 0x82EFB140;
    'dispatch: loop {
        match pc {
            0x82EFB140 => {
    //   block [0x82EFB140..0x82EFB180)
	// 82EFB140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFB148: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFB14C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB150: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB154: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFB158: 419A0014  beq cr6, 0x82efb16c
	if ctx.cr[6].eq {
	pc = 0x82EFB16C; continue 'dispatch;
	}
	// 82EFB15C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFB160: 4BFFFA71  bl 0x82efabd0
	ctx.lr = 0x82EFB164;
	sub_82EFABD0(ctx, base);
	// 82EFB164: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFB168: 48002739  bl 0x82efd8a0
	ctx.lr = 0x82EFB16C;
	sub_82EFD8A0(ctx, base);
	// 82EFB16C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFB170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFB174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFB178: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFB17C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB180 size=120
    let mut pc: u32 = 0x82EFB180;
    'dispatch: loop {
        match pc {
            0x82EFB180 => {
    //   block [0x82EFB180..0x82EFB1F8)
	// 82EFB180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFB188: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFB18C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFB190: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB194: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFB198: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFB19C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFB1A0: 396BD080  addi r11, r11, -0x2f80
	ctx.r[11].s64 = ctx.r[11].s64 + -12160;
	// 82EFB1A4: 394AD06C  addi r10, r10, -0x2f94
	ctx.r[10].s64 = ctx.r[10].s64 + -12180;
	// 82EFB1A8: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFB1AC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EFB1B0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFB1B4: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82EFB1B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFB1BC: 419A000C  beq cr6, 0x82efb1c8
	if ctx.cr[6].eq {
	pc = 0x82EFB1C8; continue 'dispatch;
	}
	// 82EFB1C0: 4BDC75F1  bl 0x82cc27b0
	ctx.lr = 0x82EFB1C4;
	sub_82CC27B0(ctx, base);
	// 82EFB1C4: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82EFB1C8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFB1CC: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82EFB1D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFB1D4: 396BC544  addi r11, r11, -0x3abc
	ctx.r[11].s64 = ctx.r[11].s64 + -15036;
	// 82EFB1D8: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EFB1DC: 4BFFE955  bl 0x82ef9b30
	ctx.lr = 0x82EFB1E0;
	sub_82EF9B30(ctx, base);
	// 82EFB1E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFB1E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFB1E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFB1EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFB1F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFB1F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB1F8 size=12
    let mut pc: u32 = 0x82EFB1F8;
    'dispatch: loop {
        match pc {
            0x82EFB1F8 => {
    //   block [0x82EFB1F8..0x82EFB204)
	// 82EFB1F8: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFB1FC: 5563E7FE  rlwinm r3, r11, 0x1c, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82EFB200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB208 size=12
    let mut pc: u32 = 0x82EFB208;
    'dispatch: loop {
        match pc {
            0x82EFB208 => {
    //   block [0x82EFB208..0x82EFB214)
	// 82EFB208: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFB20C: 5563FFFE  rlwinm r3, r11, 0x1f, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82EFB210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB218 size=28
    let mut pc: u32 = 0x82EFB218;
    'dispatch: loop {
        match pc {
            0x82EFB218 => {
    //   block [0x82EFB218..0x82EFB234)
	// 82EFB218: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFB21C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFB220: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFB224: 41990008  bgt cr6, 0x82efb22c
	if ctx.cr[6].gt {
	pc = 0x82EFB22C; continue 'dispatch;
	}
	// 82EFB228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFB22C: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82EFB230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB238 size=36
    let mut pc: u32 = 0x82EFB238;
    'dispatch: loop {
        match pc {
            0x82EFB238 => {
    //   block [0x82EFB238..0x82EFB25C)
	// 82EFB238: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFB23C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFB240: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFB244: 41990008  bgt cr6, 0x82efb24c
	if ctx.cr[6].gt {
	pc = 0x82EFB24C; continue 'dispatch;
	}
	// 82EFB248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFB24C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFB250: 4182000C  beq 0x82efb25c
	if ctx.cr[0].eq {
		sub_82EFB25C(ctx, base);
		return;
	}
	// 82EFB254: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82EFB258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB25C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB25C size=12
    let mut pc: u32 = 0x82EFB25C;
    'dispatch: loop {
        match pc {
            0x82EFB25C => {
    //   block [0x82EFB25C..0x82EFB268)
	// 82EFB25C: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFB260: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82EFB264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB268 size=352
    let mut pc: u32 = 0x82EFB268;
    'dispatch: loop {
        match pc {
            0x82EFB268 => {
    //   block [0x82EFB268..0x82EFB3C8)
	// 82EFB268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB26C: 4BDAE1A1  bl 0x82ca940c
	ctx.lr = 0x82EFB270;
	sub_82CA93D0(ctx, base);
	// 82EFB270: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB274: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFB278: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EFB27C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EFB280: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB284: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFB288: 409A000C  bne cr6, 0x82efb294
	if !ctx.cr[6].eq {
	pc = 0x82EFB294; continue 'dispatch;
	}
	// 82EFB28C: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82EFB290: 4800002C  b 0x82efb2bc
	pc = 0x82EFB2BC; continue 'dispatch;
	// 82EFB294: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFB298: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB29C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EFB2A0: 1D290005  mulli r9, r9, 5
	ctx.r[9].s64 = ctx.r[9].s64 * 5;
	// 82EFB2A4: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EFB2A8: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EFB2AC: 40990018  ble cr6, 0x82efb2c4
	if !ctx.cr[6].gt {
	pc = 0x82EFB2C4; continue 'dispatch;
	}
	// 82EFB2B0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFB2B4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EFB2B8: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EFB2BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFB2C0: 48000219  bl 0x82efb4d8
	ctx.lr = 0x82EFB2C4;
	sub_82EFB4D8(ctx, base);
	// 82EFB2C4: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB2C8: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFB2CC: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB2D0: 7D48F038  and r8, r10, r30
	ctx.r[8].u64 = ctx.r[10].u64 & ctx.r[30].u64;
	// 82EFB2D4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EFB2D8: 1D48000C  mulli r10, r8, 0xc
	ctx.r[10].s64 = ctx.r[8].s64 * 12;
	// 82EFB2DC: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFB2E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB2E4: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EFB2E8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82EFB2EC: 80AA0000  lwz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB2F0: 2F05FFFE  cmpwi cr6, r5, -2
	ctx.cr[6].compare_i32(ctx.r[5].s32, -2, &mut ctx.xer);
	// 82EFB2F4: 409A0014  bne cr6, 0x82efb308
	if !ctx.cr[6].eq {
	pc = 0x82EFB308; continue 'dispatch;
	}
	// 82EFB2F8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EFB2FC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFB300: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB304: 48000064  b 0x82efb368
	pc = 0x82EFB368; continue 'dispatch;
	// 82EFB308: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFB30C: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 82EFB310: 38E60001  addi r7, r6, 1
	ctx.r[7].s64 = ctx.r[6].s64 + 1;
	// 82EFB314: 7CE64838  and r6, r7, r9
	ctx.r[6].u64 = ctx.r[7].u64 & ctx.r[9].u64;
	// 82EFB318: 1CE6000C  mulli r7, r6, 0xc
	ctx.r[7].s64 = ctx.r[6].s64 * 12;
	// 82EFB31C: 7CE75A14  add r7, r7, r11
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82EFB320: 80E70008  lwz r7, 8(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFB324: 2F07FFFE  cmpwi cr6, r7, -2
	ctx.cr[6].compare_i32(ctx.r[7].s32, -2, &mut ctx.xer);
	// 82EFB328: 409AFFE8  bne cr6, 0x82efb310
	if !ctx.cr[6].eq {
	pc = 0x82EFB310; continue 'dispatch;
	}
	// 82EFB32C: 1CE6000C  mulli r7, r6, 0xc
	ctx.r[7].s64 = ctx.r[6].s64 * 12;
	// 82EFB330: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFB334: 7CE75A14  add r7, r7, r11
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82EFB338: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82EFB33C: 38E70008  addi r7, r7, 8
	ctx.r[7].s64 = ctx.r[7].s64 + 8;
	// 82EFB340: 409A0034  bne cr6, 0x82efb374
	if !ctx.cr[6].eq {
	pc = 0x82EFB374; continue 'dispatch;
	}
	// 82EFB344: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82EFB348: 419A0018  beq cr6, 0x82efb360
	if ctx.cr[6].eq {
	pc = 0x82EFB360; continue 'dispatch;
	}
	// 82EFB34C: 90A70000  stw r5, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82EFB350: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFB354: 91670004  stw r11, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EFB358: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFB35C: 91670008  stw r11, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFB360: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB364: 90CA0000  stw r6, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82EFB368: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFB36C: 48000050  b 0x82efb3bc
	pc = 0x82EFB3BC; continue 'dispatch;
	// 82EFB370: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB374: 1D29000C  mulli r9, r9, 0xc
	ctx.r[9].s64 = ctx.r[9].s64 * 12;
	// 82EFB378: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82EFB37C: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82EFB380: 80890000  lwz r4, 0(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB384: 7F044000  cmpw cr6, r4, r8
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EFB388: 409AFFE8  bne cr6, 0x82efb370
	if !ctx.cr[6].eq {
	pc = 0x82EFB370; continue 'dispatch;
	}
	// 82EFB38C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82EFB390: 419A0018  beq cr6, 0x82efb3a8
	if ctx.cr[6].eq {
	pc = 0x82EFB3A8; continue 'dispatch;
	}
	// 82EFB394: 90A70000  stw r5, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82EFB398: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFB39C: 91670004  stw r11, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EFB3A0: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFB3A4: 91670008  stw r11, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFB3A8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EFB3AC: 90C90000  stw r6, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82EFB3B0: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB3B4: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFB3B8: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82EFB3BC: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82EFB3C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFB3C4: 4BDAE098  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB3C8 size=132
    let mut pc: u32 = 0x82EFB3C8;
    'dispatch: loop {
        match pc {
            0x82EFB3C8 => {
    //   block [0x82EFB3C8..0x82EFB44C)
	// 82EFB3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB3CC: 4BDAE03D  bl 0x82ca9408
	ctx.lr = 0x82EFB3D0;
	sub_82CA93D0(ctx, base);
	// 82EFB3D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB3D4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EFB3D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFB3DC: 578B07BD  rlwinm. r11, r28, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFB3E0: 41820048  beq 0x82efb428
	if ctx.cr[0].eq {
	pc = 0x82EFB428; continue 'dispatch;
	}
	// 82EFB3E4: 815EFFFC  lwz r10, -4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82EFB3E8: 3BBEFFFC  addi r29, r30, -4
	ctx.r[29].s64 = ctx.r[30].s64 + -4;
	// 82EFB3EC: 1D6A001C  mulli r11, r10, 0x1c
	ctx.r[11].s64 = ctx.r[10].s64 * 28;
	// 82EFB3F0: 37EAFFFF  addic. r31, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFB3F4: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EFB3F8: 41800018  blt 0x82efb410
	if ctx.cr[0].lt {
	pc = 0x82EFB410; continue 'dispatch;
	}
	// 82EFB3FC: 3BDEFFE4  addi r30, r30, -0x1c
	ctx.r[30].s64 = ctx.r[30].s64 + -28;
	// 82EFB400: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFB404: 4BFFFCD5  bl 0x82efb0d8
	ctx.lr = 0x82EFB408;
	sub_82EFB0D8(ctx, base);
	// 82EFB408: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFB40C: 4080FFF0  bge 0x82efb3fc
	if !ctx.cr[0].lt {
	pc = 0x82EFB3FC; continue 'dispatch;
	}
	// 82EFB410: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFB414: 4182000C  beq 0x82efb420
	if ctx.cr[0].eq {
	pc = 0x82EFB420; continue 'dispatch;
	}
	// 82EFB418: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFB41C: 4B94A395  bl 0x828457b0
	ctx.lr = 0x82EFB420;
	sub_828457B0(ctx, base);
	// 82EFB420: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFB424: 48000020  b 0x82efb444
	pc = 0x82EFB444; continue 'dispatch;
	// 82EFB428: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFB42C: 4BFFFCAD  bl 0x82efb0d8
	ctx.lr = 0x82EFB430;
	sub_82EFB0D8(ctx, base);
	// 82EFB430: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFB434: 4182000C  beq 0x82efb440
	if ctx.cr[0].eq {
	pc = 0x82EFB440; continue 'dispatch;
	}
	// 82EFB438: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFB43C: 48002465  bl 0x82efd8a0
	ctx.lr = 0x82EFB440;
	sub_82EFD8A0(ctx, base);
	// 82EFB440: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFB444: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFB448: 4BDAE010  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB450 size=132
    let mut pc: u32 = 0x82EFB450;
    'dispatch: loop {
        match pc {
            0x82EFB450 => {
    //   block [0x82EFB450..0x82EFB4D4)
	// 82EFB450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB454: 4BDADFB5  bl 0x82ca9408
	ctx.lr = 0x82EFB458;
	sub_82CA93D0(ctx, base);
	// 82EFB458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB45C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EFB460: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFB464: 578B07BD  rlwinm. r11, r28, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFB468: 41820048  beq 0x82efb4b0
	if ctx.cr[0].eq {
	pc = 0x82EFB4B0; continue 'dispatch;
	}
	// 82EFB46C: 815EFFFC  lwz r10, -4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82EFB470: 3BBEFFFC  addi r29, r30, -4
	ctx.r[29].s64 = ctx.r[30].s64 + -4;
	// 82EFB474: 1D6A0038  mulli r11, r10, 0x38
	ctx.r[11].s64 = ctx.r[10].s64 * 56;
	// 82EFB478: 37EAFFFF  addic. r31, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFB47C: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EFB480: 41800018  blt 0x82efb498
	if ctx.cr[0].lt {
	pc = 0x82EFB498; continue 'dispatch;
	}
	// 82EFB484: 3BDEFFC8  addi r30, r30, -0x38
	ctx.r[30].s64 = ctx.r[30].s64 + -56;
	// 82EFB488: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFB48C: 4BFFFCF5  bl 0x82efb180
	ctx.lr = 0x82EFB490;
	sub_82EFB180(ctx, base);
	// 82EFB490: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFB494: 4080FFF0  bge 0x82efb484
	if !ctx.cr[0].lt {
	pc = 0x82EFB484; continue 'dispatch;
	}
	// 82EFB498: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFB49C: 4182000C  beq 0x82efb4a8
	if ctx.cr[0].eq {
	pc = 0x82EFB4A8; continue 'dispatch;
	}
	// 82EFB4A0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFB4A4: 4B94A30D  bl 0x828457b0
	ctx.lr = 0x82EFB4A8;
	sub_828457B0(ctx, base);
	// 82EFB4A8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFB4AC: 48000020  b 0x82efb4cc
	pc = 0x82EFB4CC; continue 'dispatch;
	// 82EFB4B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFB4B4: 4BFFFCCD  bl 0x82efb180
	ctx.lr = 0x82EFB4B8;
	sub_82EFB180(ctx, base);
	// 82EFB4B8: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFB4BC: 4182000C  beq 0x82efb4c8
	if ctx.cr[0].eq {
	pc = 0x82EFB4C8; continue 'dispatch;
	}
	// 82EFB4C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFB4C4: 480023DD  bl 0x82efd8a0
	ctx.lr = 0x82EFB4C8;
	sub_82EFD8A0(ctx, base);
	// 82EFB4C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFB4CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFB4D0: 4BDADF88  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EFB4D8 size=320
    let mut pc: u32 = 0x82EFB4D8;
    'dispatch: loop {
        match pc {
            0x82EFB4D8 => {
    //   block [0x82EFB4D8..0x82EFB618)
	// 82EFB4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB4DC: 4BDADF25  bl 0x82ca9400
	ctx.lr = 0x82EFB4E0;
	sub_82CA93D0(ctx, base);
	// 82EFB4E0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB4E4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82EFB4E8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EFB4EC: 409A0008  bne cr6, 0x82efb4f4
	if !ctx.cr[6].eq {
	pc = 0x82EFB4F4; continue 'dispatch;
	}
	// 82EFB4F0: 4800011C  b 0x82efb60c
	pc = 0x82EFB60C; continue 'dispatch;
	// 82EFB4F4: 2B040008  cmplwi cr6, r4, 8
	ctx.cr[6].compare_u32(ctx.r[4].u32, 8 as u32, &mut ctx.xer);
	// 82EFB4F8: 4098000C  bge cr6, 0x82efb504
	if !ctx.cr[6].lt {
	pc = 0x82EFB504; continue 'dispatch;
	}
	// 82EFB4FC: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 82EFB500: 4800004C  b 0x82efb54c
	pc = 0x82EFB54C; continue 'dispatch;
	// 82EFB504: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 82EFB508: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82EFB50C: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82EFB510: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82EFB514: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82EFB518: FC200018  frsp f1, f0
	ctx.f[1].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82EFB51C: 4B2F8895  bl 0x821f3db0
	ctx.lr = 0x82EFB520;
	sub_821F3DB0(ctx, base);
	// 82EFB520: FD800818  frsp f12, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82EFB524: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EFB528: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82EFB52C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EFB530: C00B3FA8  lfs f0, 0x3fa8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16296 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EFB534: C1AA0C14  lfs f13, 0xc14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EFB538: EC0C683A  fmadds f0, f12, f0, f13
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82EFB53C: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82EFB540: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 82EFB544: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EFB548: 7D3F5830  slw r31, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[31].u64 = 0;
	} else {
		ctx.r[31].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82EFB54C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82EFB550: 1D7F000C  mulli r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 * 12;
	// 82EFB554: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82EFB558: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82EFB55C: 48002305  bl 0x82efd860
	ctx.lr = 0x82EFB560;
	sub_82EFD860(ctx, base);
	// 82EFB560: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82EFB564: 397FFFFF  addi r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	// 82EFB568: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82EFB56C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFB570: 3B60FFFE  li r27, -2
	ctx.r[27].s64 = -2;
	// 82EFB574: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFB578: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EFB57C: 419A0020  beq cr6, 0x82efb59c
	if ctx.cr[6].eq {
	pc = 0x82EFB59C; continue 'dispatch;
	}
	// 82EFB580: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82EFB584: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFB588: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFB58C: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EFB590: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82EFB594: 936A0008  stw r27, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82EFB598: 4082FFEC  bne 0x82efb584
	if !ctx.cr[0].eq {
	pc = 0x82EFB584; continue 'dispatch;
	}
	// 82EFB59C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB5A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFB5A4: 419A0058  beq cr6, 0x82efb5fc
	if ctx.cr[6].eq {
	pc = 0x82EFB5FC; continue 'dispatch;
	}
	// 82EFB5A8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFB5AC: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82EFB5B0: 3BAB0001  addi r29, r11, 1
	ctx.r[29].s64 = ctx.r[11].s64 + 1;
	// 82EFB5B4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB5B8: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82EFB5BC: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 82EFB5C0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFB5C4: 2F0BFFFE  cmpwi cr6, r11, -2
	ctx.cr[6].compare_i32(ctx.r[11].s32, -2, &mut ctx.xer);
	// 82EFB5C8: 419A0020  beq cr6, 0x82efb5e8
	if ctx.cr[6].eq {
	pc = 0x82EFB5E8; continue 'dispatch;
	}
	// 82EFB5CC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFB5D0: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82EFB5D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EFB5D8: 556AD1BE  srwi r10, r11, 6
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EFB5DC: 7D455A78  xor r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	// 82EFB5E0: 4BFFFC89  bl 0x82efb268
	ctx.lr = 0x82EFB5E4;
	sub_82EFB268(ctx, base);
	// 82EFB5E4: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82EFB5E8: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82EFB5EC: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 82EFB5F0: 4082FFC4  bne 0x82efb5b4
	if !ctx.cr[0].eq {
	pc = 0x82EFB5B4; continue 'dispatch;
	}
	// 82EFB5F4: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB5F8: 480022A9  bl 0x82efd8a0
	ctx.lr = 0x82EFB5FC;
	sub_82EFD8A0(ctx, base);
	// 82EFB5FC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFB600: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EFB604: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82EFB608: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFB60C: 4809496D  bl 0x82f8ff78
	ctx.lr = 0x82EFB610;
	sub_82F8FF78(ctx, base);
	// 82EFB610: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EFB614: 4BDADE3C  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB618 size=8
    let mut pc: u32 = 0x82EFB618;
    'dispatch: loop {
        match pc {
            0x82EFB618 => {
    //   block [0x82EFB618..0x82EFB620)
	// 82EFB618: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82EFB61C: 4BFFFDAC  b 0x82efb3c8
	sub_82EFB3C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB620 size=8
    let mut pc: u32 = 0x82EFB620;
    'dispatch: loop {
        match pc {
            0x82EFB620 => {
    //   block [0x82EFB620..0x82EFB628)
	// 82EFB620: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82EFB624: 4BFFFE2C  b 0x82efb450
	sub_82EFB450(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB628 size=84
    let mut pc: u32 = 0x82EFB628;
    'dispatch: loop {
        match pc {
            0x82EFB628 => {
    //   block [0x82EFB628..0x82EFB67C)
	// 82EFB628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB62C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFB630: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFB634: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB638: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFB63C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFB640: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82EFB644: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFB648: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EFB64C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFB650: 4BFFF479  bl 0x82efaac8
	ctx.lr = 0x82EFB654;
	sub_82EFAAC8(ctx, base);
	// 82EFB654: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82EFB658: 4BFFF959  bl 0x82efafb0
	ctx.lr = 0x82EFB65C;
	sub_82EFAFB0(ctx, base);
	// 82EFB65C: 4B3236AD  bl 0x8221ed08
	ctx.lr = 0x82EFB660;
	sub_8221ED08(ctx, base);
	// 82EFB660: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 82EFB664: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFB668: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFB66C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFB670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFB674: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFB678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB680 size=92
    let mut pc: u32 = 0x82EFB680;
    'dispatch: loop {
        match pc {
            0x82EFB680 => {
    //   block [0x82EFB680..0x82EFB6DC)
	// 82EFB680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFB688: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFB68C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFB690: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB694: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFB698: 83FE0020  lwz r31, 0x20(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFB69C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFB6A0: 419A0014  beq cr6, 0x82efb6b4
	if ctx.cr[6].eq {
	pc = 0x82EFB6B4; continue 'dispatch;
	}
	// 82EFB6A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFB6A8: 4BFFF529  bl 0x82efabd0
	ctx.lr = 0x82EFB6AC;
	sub_82EFABD0(ctx, base);
	// 82EFB6AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFB6B0: 480021F1  bl 0x82efd8a0
	ctx.lr = 0x82EFB6B4;
	sub_82EFD8A0(ctx, base);
	// 82EFB6B4: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82EFB6B8: 4BFFFA21  bl 0x82efb0d8
	ctx.lr = 0x82EFB6BC;
	sub_82EFB0D8(ctx, base);
	// 82EFB6BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFB6C0: 480948B9  bl 0x82f8ff78
	ctx.lr = 0x82EFB6C4;
	sub_82F8FF78(ctx, base);
	// 82EFB6C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFB6C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFB6CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFB6D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFB6D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFB6D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB6E0 size=124
    let mut pc: u32 = 0x82EFB6E0;
    'dispatch: loop {
        match pc {
            0x82EFB6E0 => {
    //   block [0x82EFB6E0..0x82EFB75C)
	// 82EFB6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB6E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFB6E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB6EC: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFB6F0: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFB6F4: 41820054  beq 0x82efb748
	if ctx.cr[0].eq {
	pc = 0x82EFB748; continue 'dispatch;
	}
	// 82EFB6F8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EFB6FC: 38E3001C  addi r7, r3, 0x1c
	ctx.r[7].s64 = ctx.r[3].s64 + 28;
	// 82EFB700: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EFB704: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFB708: 7D403828  lwarx r10, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EFB70C: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EFB710: 7D20392D  stwcx. r9, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFB714: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFB718: 4082FFE8  bne 0x82efb700
	if !ctx.cr[0].eq {
	pc = 0x82EFB700; continue 'dispatch;
	}
	// 82EFB71C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EFB720: 7C2004AC  lwsync
	// 82EFB724: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EFB728: 41980020  blt cr6, 0x82efb748
	if ctx.cr[6].lt {
	pc = 0x82EFB748; continue 'dispatch;
	}
	// 82EFB72C: 409A0014  bne cr6, 0x82efb740
	if !ctx.cr[6].eq {
	pc = 0x82EFB740; continue 'dispatch;
	}
	// 82EFB730: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFB734: 4BDCC735  bl 0x82cc7e68
	ctx.lr = 0x82EFB738;
	sub_82CC7E68(ctx, base);
	// 82EFB738: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82EFB73C: 419A000C  beq cr6, 0x82efb748
	if ctx.cr[6].eq {
	pc = 0x82EFB748; continue 'dispatch;
	}
	// 82EFB740: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EFB744: 48000008  b 0x82efb74c
	pc = 0x82EFB74C; continue 'dispatch;
	// 82EFB748: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFB74C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFB750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFB754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFB758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB760 size=52
    let mut pc: u32 = 0x82EFB760;
    'dispatch: loop {
        match pc {
            0x82EFB760 => {
    //   block [0x82EFB760..0x82EFB794)
	// 82EFB760: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EFB764: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB768: 7D232038  and r3, r9, r4
	ctx.r[3].u64 = ctx.r[9].u64 & ctx.r[4].u64;
	// 82EFB76C: 7C2004AC  lwsync
	// 82EFB770: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EFB774: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFB778: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EFB77C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EFB780: 409A0014  bne cr6, 0x82efb794
	if !ctx.cr[6].eq {
		sub_82EFB794(ctx, base);
		return;
	}
	// 82EFB784: 7C60592D  stwcx. r3, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[3].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFB788: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFB78C: 4082FFE4  bne 0x82efb770
	if !ctx.cr[0].eq {
	pc = 0x82EFB770; continue 'dispatch;
	}
	// 82EFB790: 4800000C  b 0x82efb79c
	sub_82EFB794(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB794(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB794 size=28
    let mut pc: u32 = 0x82EFB794;
    'dispatch: loop {
        match pc {
            0x82EFB794 => {
    //   block [0x82EFB794..0x82EFB7B0)
	// 82EFB794: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFB798: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFB79C: 7D4A5378  mr r10, r10
	ctx.r[10].u64 = ctx.r[10].u64;
	// 82EFB7A0: 7C2004AC  lwsync
	// 82EFB7A4: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EFB7A8: 409AFFBC  bne cr6, 0x82efb764
	if !ctx.cr[6].eq {
		sub_82EFB760(ctx, base);
		return;
	}
	// 82EFB7AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB7B0 size=52
    let mut pc: u32 = 0x82EFB7B0;
    'dispatch: loop {
        match pc {
            0x82EFB7B0 => {
    //   block [0x82EFB7B0..0x82EFB7E4)
	// 82EFB7B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EFB7B4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB7B8: 7D232378  or r3, r9, r4
	ctx.r[3].u64 = ctx.r[9].u64 | ctx.r[4].u64;
	// 82EFB7BC: 7C2004AC  lwsync
	// 82EFB7C0: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EFB7C4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFB7C8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EFB7CC: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EFB7D0: 409A0014  bne cr6, 0x82efb7e4
	if !ctx.cr[6].eq {
		sub_82EFB7E4(ctx, base);
		return;
	}
	// 82EFB7D4: 7C60592D  stwcx. r3, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[3].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFB7D8: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFB7DC: 4082FFE4  bne 0x82efb7c0
	if !ctx.cr[0].eq {
	pc = 0x82EFB7C0; continue 'dispatch;
	}
	// 82EFB7E0: 4800000C  b 0x82efb7ec
	sub_82EFB7E4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB7E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB7E4 size=28
    let mut pc: u32 = 0x82EFB7E4;
    'dispatch: loop {
        match pc {
            0x82EFB7E4 => {
    //   block [0x82EFB7E4..0x82EFB800)
	// 82EFB7E4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFB7E8: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFB7EC: 7D4A5378  mr r10, r10
	ctx.r[10].u64 = ctx.r[10].u64;
	// 82EFB7F0: 7C2004AC  lwsync
	// 82EFB7F4: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EFB7F8: 409AFFBC  bne cr6, 0x82efb7b4
	if !ctx.cr[6].eq {
		sub_82EFB7B0(ctx, base);
		return;
	}
	// 82EFB7FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB800 size=124
    let mut pc: u32 = 0x82EFB800;
    'dispatch: loop {
        match pc {
            0x82EFB800 => {
    //   block [0x82EFB800..0x82EFB87C)
	// 82EFB800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB804: 4BDADC09  bl 0x82ca940c
	ctx.lr = 0x82EFB808;
	sub_82CA93D0(ctx, base);
	// 82EFB808: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB80C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFB810: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EFB814: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFB818: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EFB81C: 4BFFE06D  bl 0x82ef9888
	ctx.lr = 0x82EFB820;
	sub_82EF9888(ctx, base);
	// 82EFB820: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFB824: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFB828: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82EFB82C: 396BC544  addi r11, r11, -0x3abc
	ctx.r[11].s64 = ctx.r[11].s64 + -15036;
	// 82EFB830: 394AD080  addi r10, r10, -0x2f80
	ctx.r[10].s64 = ctx.r[10].s64 + -12160;
	// 82EFB834: 3929D06C  addi r9, r9, -0x2f94
	ctx.r[9].s64 = ctx.r[9].s64 + -12180;
	// 82EFB838: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EFB83C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EFB840: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82EFB844: 7C2004AC  lwsync
	// 82EFB848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFB84C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82EFB850: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82EFB854: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82EFB858: 7C2004AC  lwsync
	// 82EFB85C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EFB860: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFB864: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82EFB868: 93BF002C  stw r29, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 82EFB86C: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82EFB870: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82EFB874: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFB878: 4BDADBE4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB880 size=20
    let mut pc: u32 = 0x82EFB880;
    'dispatch: loop {
        match pc {
            0x82EFB880 => {
    //   block [0x82EFB880..0x82EFB894)
	// 82EFB880: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFB884: 38630018  addi r3, r3, 0x18
	ctx.r[3].s64 = ctx.r[3].s64 + 24;
	// 82EFB888: 4182000C  beq 0x82efb894
	if ctx.cr[0].eq {
		sub_82EFB894(ctx, base);
		return;
	}
	// 82EFB88C: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82EFB890: 4BFFFF20  b 0x82efb7b0
	sub_82EFB7B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB894(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFB894 size=8
    let mut pc: u32 = 0x82EFB894;
    'dispatch: loop {
        match pc {
            0x82EFB894 => {
    //   block [0x82EFB894..0x82EFB89C)
	// 82EFB894: 3880FFEF  li r4, -0x11
	ctx.r[4].s64 = -17;
	// 82EFB898: 4BFFFEC8  b 0x82efb760
	sub_82EFB760(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB8A0 size=124
    let mut pc: u32 = 0x82EFB8A0;
    'dispatch: loop {
        match pc {
            0x82EFB8A0 => {
    //   block [0x82EFB8A0..0x82EFB91C)
	// 82EFB8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFB8A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFB8AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB8B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFB8B4: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFB8B8: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFB8BC: 41820048  beq 0x82efb904
	if ctx.cr[0].eq {
	pc = 0x82EFB904; continue 'dispatch;
	}
	// 82EFB8C0: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFB8C4: 4BDC6C7D  bl 0x82cc2540
	ctx.lr = 0x82EFB8C8;
	sub_82CC2540(ctx, base);
	// 82EFB8C8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82EFB8CC: 419A0038  beq cr6, 0x82efb904
	if ctx.cr[6].eq {
	pc = 0x82EFB904; continue 'dispatch;
	}
	// 82EFB8D0: 7C2004AC  lwsync
	// 82EFB8D4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFB8D8: 393F001C  addi r9, r31, 0x1c
	ctx.r[9].s64 = ctx.r[31].s64 + 28;
	// 82EFB8DC: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82EFB8E0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFB8E4: 7D404828  lwarx r10, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EFB8E8: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EFB8EC: 7D00492D  stwcx. r8, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFB8F0: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFB8F4: 4082FFE8  bne 0x82efb8dc
	if !ctx.cr[0].eq {
	pc = 0x82EFB8DC; continue 'dispatch;
	}
	// 82EFB8F8: 7C2004AC  lwsync
	// 82EFB8FC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EFB900: 48000008  b 0x82efb908
	pc = 0x82EFB908; continue 'dispatch;
	// 82EFB904: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFB908: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFB90C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFB910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFB914: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFB918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB920 size=172
    let mut pc: u32 = 0x82EFB920;
    'dispatch: loop {
        match pc {
            0x82EFB920 => {
    //   block [0x82EFB920..0x82EFB9CC)
	// 82EFB920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFB928: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFB92C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFB930: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB934: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFB938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EFB93C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82EFB940: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82EFB944: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EFB948: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFB94C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EFB950: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFB954: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFB958: 4BFFE161  bl 0x82ef9ab8
	ctx.lr = 0x82EFB95C;
	sub_82EF9AB8(ctx, base);
	// 82EFB95C: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFB960: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFB964: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFB968: 4182000C  beq 0x82efb974
	if ctx.cr[0].eq {
	pc = 0x82EFB974; continue 'dispatch;
	}
	// 82EFB96C: 48110055  bl 0x8300b9c0
	ctx.lr = 0x82EFB970;
	sub_8300B9C0(ctx, base);
	// 82EFB970: 48000010  b 0x82efb980
	pc = 0x82EFB980; continue 'dispatch;
	// 82EFB974: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFB978: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EFB97C: 4BDCB5CD  bl 0x82cc6f48
	ctx.lr = 0x82EFB980;
	sub_82CC6F48(ctx, base);
	// 82EFB980: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFB984: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EFB988: 419A001C  beq cr6, 0x82efb9a4
	if ctx.cr[6].eq {
	pc = 0x82EFB9A4; continue 'dispatch;
	}
	// 82EFB98C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EFB990: 409A0014  bne cr6, 0x82efb9a4
	if !ctx.cr[6].eq {
	pc = 0x82EFB9A4; continue 'dispatch;
	}
	// 82EFB994: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFB998: 419A001C  beq cr6, 0x82efb9b4
	if ctx.cr[6].eq {
	pc = 0x82EFB9B4; continue 'dispatch;
	}
	// 82EFB99C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFB9A0: 4BFFDFD1  bl 0x82ef9970
	ctx.lr = 0x82EFB9A4;
	sub_82EF9970(ctx, base);
	// 82EFB9A4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFB9A8: 419A000C  beq cr6, 0x82efb9b4
	if ctx.cr[6].eq {
	pc = 0x82EFB9B4; continue 'dispatch;
	}
	// 82EFB9AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFB9B0: 4BFFE099  bl 0x82ef9a48
	ctx.lr = 0x82EFB9B4;
	sub_82EF9A48(ctx, base);
	// 82EFB9B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFB9B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFB9BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFB9C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFB9C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFB9C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFB9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFB9D0 size=100
    let mut pc: u32 = 0x82EFB9D0;
    'dispatch: loop {
        match pc {
            0x82EFB9D0 => {
    //   block [0x82EFB9D0..0x82EFBA34)
	// 82EFB9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFB9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFB9D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFB9DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFB9E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFB9E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFB9E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFB9EC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFB9F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFB9F4: 419A0024  beq cr6, 0x82efba18
	if ctx.cr[6].eq {
	pc = 0x82EFBA18; continue 'dispatch;
	}
	// 82EFB9F8: 4BFFF079  bl 0x82efaa70
	ctx.lr = 0x82EFB9FC;
	sub_82EFAA70(ctx, base);
	// 82EFB9FC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFBA00: 4082000C  bne 0x82efba0c
	if !ctx.cr[0].eq {
	pc = 0x82EFBA0C; continue 'dispatch;
	}
	// 82EFBA04: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EFBA08: 48000014  b 0x82efba1c
	pc = 0x82EFBA1C; continue 'dispatch;
	// 82EFBA0C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFBA10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBA14: 4BFFFF0D  bl 0x82efb920
	ctx.lr = 0x82EFBA18;
	sub_82EFB920(ctx, base);
	// 82EFBA18: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFBA1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFBA20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFBA24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFBA28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFBA2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFBA30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBA38 size=100
    let mut pc: u32 = 0x82EFBA38;
    'dispatch: loop {
        match pc {
            0x82EFBA38 => {
    //   block [0x82EFBA38..0x82EFBA9C)
	// 82EFBA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFBA40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFBA44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFBA48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBA4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFBA50: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFBA54: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFBA58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFBA5C: 419A0018  beq cr6, 0x82efba74
	if ctx.cr[6].eq {
	pc = 0x82EFBA74; continue 'dispatch;
	}
	// 82EFBA60: 4BFFFF71  bl 0x82efb9d0
	ctx.lr = 0x82EFBA64;
	sub_82EFB9D0(ctx, base);
	// 82EFBA64: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFBA68: 4082000C  bne 0x82efba74
	if !ctx.cr[0].eq {
	pc = 0x82EFBA74; continue 'dispatch;
	}
	// 82EFBA6C: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82EFBA70: 48000014  b 0x82efba84
	pc = 0x82EFBA84; continue 'dispatch;
	// 82EFBA74: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EFBA78: 387E0014  addi r3, r30, 0x14
	ctx.r[3].s64 = ctx.r[30].s64 + 20;
	// 82EFBA7C: 409A0008  bne cr6, 0x82efba84
	if !ctx.cr[6].eq {
	pc = 0x82EFBA84; continue 'dispatch;
	}
	// 82EFBA80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFBA84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFBA88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFBA8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFBA90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFBA94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFBA98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFBAA0 size=12
    let mut pc: u32 = 0x82EFBAA0;
    'dispatch: loop {
        match pc {
            0x82EFBAA0 => {
    //   block [0x82EFBAA0..0x82EFBAAC)
	// 82EFBAA0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EFBAA4: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFBAA8: 4BFFFE78  b 0x82efb920
	sub_82EFB920(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFBAB0 size=12
    let mut pc: u32 = 0x82EFBAB0;
    'dispatch: loop {
        match pc {
            0x82EFBAB0 => {
    //   block [0x82EFBAB0..0x82EFBABC)
	// 82EFBAB0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EFBAB4: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFBAB8: 4BFFFF18  b 0x82efb9d0
	sub_82EFB9D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFBAC0 size=12
    let mut pc: u32 = 0x82EFBAC0;
    'dispatch: loop {
        match pc {
            0x82EFBAC0 => {
    //   block [0x82EFBAC0..0x82EFBACC)
	// 82EFBAC0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EFBAC4: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFBAC8: 4BFFFF70  b 0x82efba38
	sub_82EFBA38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBAD0 size=52
    let mut pc: u32 = 0x82EFBAD0;
    'dispatch: loop {
        match pc {
            0x82EFBAD0 => {
    //   block [0x82EFBAD0..0x82EFBB04)
	// 82EFBAD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBAD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFBAD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBADC: 3883FFEC  addi r4, r3, -0x14
	ctx.r[4].s64 = ctx.r[3].s64 + -20;
	// 82EFBAE0: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFBAE4: 4BFFFEED  bl 0x82efb9d0
	ctx.lr = 0x82EFBAE8;
	sub_82EFB9D0(ctx, base);
	// 82EFBAE8: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82EFBAEC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82EFBAF0: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82EFBAF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFBAF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFBAFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFBB00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBB08 size=44
    let mut pc: u32 = 0x82EFBB08;
    'dispatch: loop {
        match pc {
            0x82EFBB08 => {
    //   block [0x82EFBB08..0x82EFBB34)
	// 82EFBB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFBB10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBB14: 3883FFEC  addi r4, r3, -0x14
	ctx.r[4].s64 = ctx.r[3].s64 + -20;
	// 82EFBB18: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFBB1C: 4BFFFE05  bl 0x82efb920
	ctx.lr = 0x82EFBB20;
	sub_82EFB920(ctx, base);
	// 82EFBB20: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EFBB24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFBB28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFBB2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFBB30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBB38 size=144
    let mut pc: u32 = 0x82EFBB38;
    'dispatch: loop {
        match pc {
            0x82EFBB38 => {
    //   block [0x82EFBB38..0x82EFBBC8)
	// 82EFBB38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBB3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFBB40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFBB44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFBB48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBB4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFBB50: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EFBB54: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EFBB58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFBB5C: 4BFFDF5D  bl 0x82ef9ab8
	ctx.lr = 0x82EFBB60;
	sub_82EF9AB8(ctx, base);
	// 82EFBB60: 3BDF0018  addi r30, r31, 0x18
	ctx.r[30].s64 = ctx.r[31].s64 + 24;
	// 82EFBB64: 3880FFFE  li r4, -2
	ctx.r[4].s64 = -2;
	// 82EFBB68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFBB6C: 4BFFFBF5  bl 0x82efb760
	ctx.lr = 0x82EFBB70;
	sub_82EFB760(ctx, base);
	// 82EFBB70: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82EFBB74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFBB78: 4BFFFC39  bl 0x82efb7b0
	ctx.lr = 0x82EFBB7C;
	sub_82EFB7B0(ctx, base);
	// 82EFBB7C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFBB80: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFBB84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBB88: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFBB8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFBB90: 4E800421  bctrl
	ctx.lr = 0x82EFBB94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFBB94: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFBB98: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFBB9C: 419A0014  beq cr6, 0x82efbbb0
	if ctx.cr[6].eq {
	pc = 0x82EFBBB0; continue 'dispatch;
	}
	// 82EFBBA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBBA4: 4BFFDDCD  bl 0x82ef9970
	ctx.lr = 0x82EFBBA8;
	sub_82EF9970(ctx, base);
	// 82EFBBA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBBAC: 4BFFDE9D  bl 0x82ef9a48
	ctx.lr = 0x82EFBBB0;
	sub_82EF9A48(ctx, base);
	// 82EFBBB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFBBB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFBBB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFBBBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFBBC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFBBC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBBC8 size=104
    let mut pc: u32 = 0x82EFBBC8;
    'dispatch: loop {
        match pc {
            0x82EFBBC8 => {
    //   block [0x82EFBBC8..0x82EFBC30)
	// 82EFBBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBBCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFBBD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFBBD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFBBD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBBDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFBBE0: 3BDF0018  addi r30, r31, 0x18
	ctx.r[30].s64 = ctx.r[31].s64 + 24;
	// 82EFBBE4: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFBBE8: 556B0739  rlwinm. r11, r11, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFBBEC: 41820014  beq 0x82efbc00
	if ctx.cr[0].eq {
	pc = 0x82EFBC00; continue 'dispatch;
	}
	// 82EFBBF0: 4BFFFCB1  bl 0x82efb8a0
	ctx.lr = 0x82EFBBF4;
	sub_82EFB8A0(ctx, base);
	// 82EFBBF4: 3880FFF7  li r4, -9
	ctx.r[4].s64 = -9;
	// 82EFBBF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFBBFC: 4BFFFB65  bl 0x82efb760
	ctx.lr = 0x82EFBC00;
	sub_82EFB760(ctx, base);
	// 82EFBC00: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFBC04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBC08: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFBC0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFBC10: 4E800421  bctrl
	ctx.lr = 0x82EFBC14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFBC14: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 82EFBC18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFBC1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFBC20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFBC24: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFBC28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFBC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBC30 size=120
    let mut pc: u32 = 0x82EFBC30;
    'dispatch: loop {
        match pc {
            0x82EFBC30 => {
    //   block [0x82EFBC30..0x82EFBCA8)
	// 82EFBC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFBC38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFBC3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFBC40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBC44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFBC48: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82EFBC4C: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82EFBC50: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFBC54: 4BFFEDD5  bl 0x82efaa28
	ctx.lr = 0x82EFBC58;
	sub_82EFAA28(ctx, base);
	// 82EFBC58: 3881008C  addi r4, r1, 0x8c
	ctx.r[4].s64 = ctx.r[1].s64 + 140;
	// 82EFBC5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBC60: 48066DB1  bl 0x82f62a10
	ctx.lr = 0x82EFBC64;
	sub_82F62A10(ctx, base);
	// 82EFBC64: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFBC68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFBC6C: 419A0010  beq cr6, 0x82efbc7c
	if ctx.cr[6].eq {
	pc = 0x82EFBC7C; continue 'dispatch;
	}
	// 82EFBC70: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFBC74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFBC78: 409A000C  bne cr6, 0x82efbc84
	if !ctx.cr[6].eq {
	pc = 0x82EFBC84; continue 'dispatch;
	}
	// 82EFBC7C: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFBC80: 4BFFF231  bl 0x82efaeb0
	ctx.lr = 0x82EFBC84;
	sub_82EFAEB0(ctx, base);
	// 82EFBC84: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFBC88: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFBC8C: 4BFFFC95  bl 0x82efb920
	ctx.lr = 0x82EFBC90;
	sub_82EFB920(ctx, base);
	// 82EFBC90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFBC94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFBC98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFBC9C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFBCA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFBCA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBCA8 size=116
    let mut pc: u32 = 0x82EFBCA8;
    'dispatch: loop {
        match pc {
            0x82EFBCA8 => {
    //   block [0x82EFBCA8..0x82EFBD1C)
	// 82EFBCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBCAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFBCB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFBCB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFBCB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBCBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFBCC0: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82EFBCC4: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFBCC8: 4BFFED61  bl 0x82efaa28
	ctx.lr = 0x82EFBCCC;
	sub_82EFAA28(ctx, base);
	// 82EFBCCC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFBCD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFBCD4: 419A0024  beq cr6, 0x82efbcf8
	if ctx.cr[6].eq {
	pc = 0x82EFBCF8; continue 'dispatch;
	}
	// 82EFBCD8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFBCDC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFBCE0: 419A0018  beq cr6, 0x82efbcf8
	if ctx.cr[6].eq {
	pc = 0x82EFBCF8; continue 'dispatch;
	}
	// 82EFBCE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82EFBCE8: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFBCEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFBCF0: 4BFFF061  bl 0x82efad50
	ctx.lr = 0x82EFBCF4;
	sub_82EFAD50(ctx, base);
	// 82EFBCF4: 4BFFFFD8  b 0x82efbccc
	pc = 0x82EFBCCC; continue 'dispatch;
	// 82EFBCF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFBCFC: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFBD00: 4BFFFC21  bl 0x82efb920
	ctx.lr = 0x82EFBD04;
	sub_82EFB920(ctx, base);
	// 82EFBD04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFBD08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFBD0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFBD10: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFBD14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFBD18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBD20 size=124
    let mut pc: u32 = 0x82EFBD20;
    'dispatch: loop {
        match pc {
            0x82EFBD20 => {
    //   block [0x82EFBD20..0x82EFBD9C)
	// 82EFBD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBD24: 4BDAD6E9  bl 0x82ca940c
	ctx.lr = 0x82EFBD28;
	sub_82CA93D0(ctx, base);
	// 82EFBD28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBD2C: 3FE08336  lis r31, -0x7cca
	ctx.r[31].s64 = -2093613056;
	// 82EFBD30: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EFBD34: 817F8FB8  lwz r11, -0x7048(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-28744 as u32) ) } as u64;
	// 82EFBD38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFBD3C: 409A0024  bne cr6, 0x82efbd60
	if !ctx.cr[6].eq {
	pc = 0x82EFBD60; continue 'dispatch;
	}
	// 82EFBD40: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 82EFBD44: 48001B1D  bl 0x82efd860
	ctx.lr = 0x82EFBD48;
	sub_82EFD860(ctx, base);
	// 82EFBD48: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFBD4C: 4182000C  beq 0x82efbd58
	if ctx.cr[0].eq {
	pc = 0x82EFBD58; continue 'dispatch;
	}
	// 82EFBD50: 4BFFF8D9  bl 0x82efb628
	ctx.lr = 0x82EFBD54;
	sub_82EFB628(ctx, base);
	// 82EFBD54: 48000008  b 0x82efbd5c
	pc = 0x82EFBD5C; continue 'dispatch;
	// 82EFBD58: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFBD5C: 907F8FB8  stw r3, -0x7048(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-28744 as u32), ctx.r[3].u32 ) };
	// 82EFBD60: 83FF8FB8  lwz r31, -0x7048(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-28744 as u32) ) } as u64;
	// 82EFBD64: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82EFBD68: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82EFBD6C: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFBD70: 4BFFECB9  bl 0x82efaa28
	ctx.lr = 0x82EFBD74;
	sub_82EFAA28(ctx, base);
	// 82EFBD74: 57ABD1BE  srwi r11, r29, 6
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shr(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFBD78: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EFBD7C: 7D65EA78  xor r5, r11, r29
	ctx.r[5].u64 = ctx.r[11].u64 ^ ctx.r[29].u64;
	// 82EFBD80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBD84: 4BFFF4E5  bl 0x82efb268
	ctx.lr = 0x82EFBD88;
	sub_82EFB268(ctx, base);
	// 82EFBD88: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFBD8C: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFBD90: 4BFFFB91  bl 0x82efb920
	ctx.lr = 0x82EFBD94;
	sub_82EFB920(ctx, base);
	// 82EFBD94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFBD98: 4BDAD6C4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBDA0 size=124
    let mut pc: u32 = 0x82EFBDA0;
    'dispatch: loop {
        match pc {
            0x82EFBDA0 => {
    //   block [0x82EFBDA0..0x82EFBE1C)
	// 82EFBDA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBDA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFBDA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFBDAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFBDB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBDB4: 3FE08336  lis r31, -0x7cca
	ctx.r[31].s64 = -2093613056;
	// 82EFBDB8: 817F8FB8  lwz r11, -0x7048(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-28744 as u32) ) } as u64;
	// 82EFBDBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFBDC0: 419A0044  beq cr6, 0x82efbe04
	if ctx.cr[6].eq {
	pc = 0x82EFBE04; continue 'dispatch;
	}
	// 82EFBDC4: 807F8FB8  lwz r3, -0x7048(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-28744 as u32) ) } as u64;
	// 82EFBDC8: 4BFFFEE1  bl 0x82efbca8
	ctx.lr = 0x82EFBDCC;
	sub_82EFBCA8(ctx, base);
	// 82EFBDCC: 817F8FB8  lwz r11, -0x7048(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-28744 as u32) ) } as u64;
	// 82EFBDD0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EFBDD4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFBDD8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EFBDDC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFBDE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFBDE4: 419A0018  beq cr6, 0x82efbdfc
	if ctx.cr[6].eq {
	pc = 0x82EFBDFC; continue 'dispatch;
	}
	// 82EFBDE8: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFBDEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFBDF0: 4BFFF891  bl 0x82efb680
	ctx.lr = 0x82EFBDF4;
	sub_82EFB680(ctx, base);
	// 82EFBDF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFBDF8: 48001AA9  bl 0x82efd8a0
	ctx.lr = 0x82EFBDFC;
	sub_82EFD8A0(ctx, base);
	// 82EFBDFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFBE00: 917F8FB8  stw r11, -0x7048(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-28744 as u32), ctx.r[11].u32 ) };
	// 82EFBE04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFBE08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFBE0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFBE10: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFBE14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFBE18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFBE20 size=4
    let mut pc: u32 = 0x82EFBE20;
    'dispatch: loop {
        match pc {
            0x82EFBE20 => {
    //   block [0x82EFBE20..0x82EFBE24)
	// 82EFBE20: 4BFFFF80  b 0x82efbda0
	sub_82EFBDA0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBE28 size=108
    let mut pc: u32 = 0x82EFBE28;
    'dispatch: loop {
        match pc {
            0x82EFBE28 => {
    //   block [0x82EFBE28..0x82EFBE94)
	// 82EFBE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFBE30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFBE34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFBE38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBE3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFBE40: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EFBE44: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 82EFBE48: 419A000C  beq cr6, 0x82efbe54
	if ctx.cr[6].eq {
	pc = 0x82EFBE54; continue 'dispatch;
	}
	// 82EFBE4C: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 82EFBE50: 4BDC6821  bl 0x82cc2670
	ctx.lr = 0x82EFBE54;
	sub_82CC2670(ctx, base);
	// 82EFBE54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBE58: 4BFFFD71  bl 0x82efbbc8
	ctx.lr = 0x82EFBE5C;
	sub_82EFBBC8(ctx, base);
	// 82EFBE5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFBE60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBE64: 4BFFFCD5  bl 0x82efbb38
	ctx.lr = 0x82EFBE68;
	sub_82EFBB38(ctx, base);
	// 82EFBE68: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EFBE6C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFBE70: 806B8FB8  lwz r3, -0x7048(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28744 as u32) ) } as u64;
	// 82EFBE74: 4BFFFDBD  bl 0x82efbc30
	ctx.lr = 0x82EFBE78;
	sub_82EFBC30(ctx, base);
	// 82EFBE78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFBE7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFBE80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFBE84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFBE88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFBE8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFBE90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBE98 size=296
    let mut pc: u32 = 0x82EFBE98;
    'dispatch: loop {
        match pc {
            0x82EFBE98 => {
    //   block [0x82EFBE98..0x82EFBFC0)
	// 82EFBE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBE9C: 4BDAD56D  bl 0x82ca9408
	ctx.lr = 0x82EFBEA0;
	sub_82CA93D0(ctx, base);
	// 82EFBEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBEA4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EFBEA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFBEAC: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82EFBEB0: 409A000C  bne cr6, 0x82efbebc
	if !ctx.cr[6].eq {
	pc = 0x82EFBEBC; continue 'dispatch;
	}
	// 82EFBEB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFBEB8: 48000100  b 0x82efbfb8
	pc = 0x82EFBFB8; continue 'dispatch;
	// 82EFBEBC: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFBEC0: 3BBF001C  addi r29, r31, 0x1c
	ctx.r[29].s64 = ctx.r[31].s64 + 28;
	// 82EFBEC4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EFBEC8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFBECC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFBED0: 41990008  bgt cr6, 0x82efbed8
	if ctx.cr[6].gt {
	pc = 0x82EFBED8; continue 'dispatch;
	}
	// 82EFBED4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82EFBED8: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFBEDC: 40820010  bne 0x82efbeec
	if !ctx.cr[0].eq {
	pc = 0x82EFBEEC; continue 'dispatch;
	}
	// 82EFBEE0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFBEE4: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFBEE8: 41820018  beq 0x82efbf00
	if ctx.cr[0].eq {
	pc = 0x82EFBF00; continue 'dispatch;
	}
	// 82EFBEEC: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82EFBEF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBEF4: 4BFFE42D  bl 0x82efa320
	ctx.lr = 0x82EFBEF8;
	sub_82EFA320(ctx, base);
	// 82EFBEF8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFBEFC: 4182FFB8  beq 0x82efbeb4
	if ctx.cr[0].eq {
	pc = 0x82EFBEB4; continue 'dispatch;
	}
	// 82EFBF00: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFBF04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFBF08: 419A000C  beq cr6, 0x82efbf14
	if ctx.cr[6].eq {
	pc = 0x82EFBF14; continue 'dispatch;
	}
	// 82EFBF0C: 4BDC68A5  bl 0x82cc27b0
	ctx.lr = 0x82EFBF10;
	sub_82CC27B0(ctx, base);
	// 82EFBF10: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82EFBF14: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFBF18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBF1C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFBF20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFBF24: 4E800421  bctrl
	ctx.lr = 0x82EFBF28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFBF28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBF2C: 4BFFFDF5  bl 0x82efbd20
	ctx.lr = 0x82EFBF30;
	sub_82EFBD20(ctx, base);
	// 82EFBF30: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 82EFBF34: 7C2004AC  lwsync
	// 82EFBF38: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82EFBF3C: 7C2004AC  lwsync
	// 82EFBF40: 397CFFFF  addi r11, r28, -1
	ctx.r[11].s64 = ctx.r[28].s64 + -1;
	// 82EFBF44: 3D4082F0  lis r10, -0x7d10
	ctx.r[10].s64 = -2098200576;
	// 82EFBF48: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82EFBF4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EFBF50: 556BF738  rlwinm r11, r11, 0x1e, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82EFBF54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82EFBF58: 696B0008  xori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 ^ 8;
	// 82EFBF5C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82EFBF60: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82EFBF64: 38AABE28  addi r5, r10, -0x41d8
	ctx.r[5].s64 = ctx.r[10].s64 + -16856;
	// 82EFBF68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFBF6C: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFBF70: 4810D589  bl 0x830094f8
	ctx.lr = 0x82EFBF74;
	sub_830094F8(ctx, base);
	// 82EFBF74: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 82EFBF78: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFBF7C: 40820038  bne 0x82efbfb4
	if !ctx.cr[0].eq {
	pc = 0x82EFBFB4; continue 'dispatch;
	}
	// 82EFBF80: 7C2004AC  lwsync
	// 82EFBF84: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82EFBF88: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFBF8C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFBF90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBF94: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFBF98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFBF9C: 4E800421  bctrl
	ctx.lr = 0x82EFBFA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFBFA0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EFBFA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFBFA8: 806B8FB8  lwz r3, -0x7048(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28744 as u32) ) } as u64;
	// 82EFBFAC: 4BFFFC85  bl 0x82efbc30
	ctx.lr = 0x82EFBFB0;
	sub_82EFBC30(ctx, base);
	// 82EFBFB0: 4BFFFF04  b 0x82efbeb4
	pc = 0x82EFBEB4; continue 'dispatch;
	// 82EFBFB4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EFBFB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFBFBC: 4BDAD49C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFBFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFBFC0 size=100
    let mut pc: u32 = 0x82EFBFC0;
    'dispatch: loop {
        match pc {
            0x82EFBFC0 => {
    //   block [0x82EFBFC0..0x82EFC024)
	// 82EFBFC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFBFC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFBFC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFBFCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFBFD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFBFD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFBFD8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFBFDC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFBFE0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFBFE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFBFE8: 4E800421  bctrl
	ctx.lr = 0x82EFBFEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFBFEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFBFF0: 4BFFFB49  bl 0x82efbb38
	ctx.lr = 0x82EFBFF4;
	sub_82EFBB38(ctx, base);
	// 82EFBFF4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EFBFF8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFBFFC: 806B8FB8  lwz r3, -0x7048(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28744 as u32) ) } as u64;
	// 82EFC000: 4BFFFC31  bl 0x82efbc30
	ctx.lr = 0x82EFC004;
	sub_82EFBC30(ctx, base);
	// 82EFC004: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFC008: 4810D3C9  bl 0x830093d0
	ctx.lr = 0x82EFC00C;
	sub_830093D0(ctx, base);
	// 82EFC00C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFC010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFC014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFC018: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFC01C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFC020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC028 size=156
    let mut pc: u32 = 0x82EFC028;
    'dispatch: loop {
        match pc {
            0x82EFC028 => {
    //   block [0x82EFC028..0x82EFC0C4)
	// 82EFC028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC02C: 4BDAD3D5  bl 0x82ca9400
	ctx.lr = 0x82EFC030;
	sub_82CA93D0(ctx, base);
	// 82EFC030: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC034: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EFC038: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EFC03C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFC040: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82EFC044: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82EFC048: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82EFC04C: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 82EFC050: 4BFFD839  bl 0x82ef9888
	ctx.lr = 0x82EFC054;
	sub_82EF9888(ctx, base);
	// 82EFC054: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFC058: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFC05C: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82EFC060: 396BC544  addi r11, r11, -0x3abc
	ctx.r[11].s64 = ctx.r[11].s64 + -15036;
	// 82EFC064: 394AD080  addi r10, r10, -0x2f80
	ctx.r[10].s64 = ctx.r[10].s64 + -12160;
	// 82EFC068: 3929D06C  addi r9, r9, -0x2f94
	ctx.r[9].s64 = ctx.r[9].s64 + -12180;
	// 82EFC06C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EFC070: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EFC074: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82EFC078: 7C2004AC  lwsync
	// 82EFC07C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFC080: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82EFC084: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82EFC088: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82EFC08C: 7C2004AC  lwsync
	// 82EFC090: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EFC094: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EFC098: 937F0020  stw r27, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[27].u32 ) };
	// 82EFC09C: 935F002C  stw r26, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[26].u32 ) };
	// 82EFC0A0: 93BF0030  stw r29, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 82EFC0A4: 939F0034  stw r28, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[28].u32 ) };
	// 82EFC0A8: 419A0010  beq cr6, 0x82efc0b8
	if ctx.cr[6].eq {
	pc = 0x82EFC0B8; continue 'dispatch;
	}
	// 82EFC0AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFC0B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC0B4: 4BFFFDE5  bl 0x82efbe98
	ctx.lr = 0x82EFC0B8;
	sub_82EFBE98(ctx, base);
	// 82EFC0B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC0BC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EFC0C0: 4BDAD390  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC0C8 size=68
    let mut pc: u32 = 0x82EFC0C8;
    'dispatch: loop {
        match pc {
            0x82EFC0C8 => {
    //   block [0x82EFC0C8..0x82EFC10C)
	// 82EFC0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFC0D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFC0D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC0D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFC0DC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFC0E0: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EFC0E4: 396BD0A4  addi r11, r11, -0x2f5c
	ctx.r[11].s64 = ctx.r[11].s64 + -12124;
	// 82EFC0E8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFC0EC: 41820008  beq 0x82efc0f4
	if ctx.cr[0].eq {
	pc = 0x82EFC0F4; continue 'dispatch;
	}
	// 82EFC0F0: 4B9496C1  bl 0x828457b0
	ctx.lr = 0x82EFC0F4;
	sub_828457B0(ctx, base);
	// 82EFC0F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC0F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFC0FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFC100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFC104: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFC108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC110 size=84
    let mut pc: u32 = 0x82EFC110;
    'dispatch: loop {
        match pc {
            0x82EFC110 => {
    //   block [0x82EFC110..0x82EFC164)
	// 82EFC110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFC118: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFC11C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC120: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFC124: 48001825  bl 0x82efd948
	ctx.lr = 0x82EFC128;
	sub_82EFD948(ctx, base);
	// 82EFC128: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFC12C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFC130: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82EFC134: 396BD0A4  addi r11, r11, -0x2f5c
	ctx.r[11].s64 = ctx.r[11].s64 + -12124;
	// 82EFC138: 394AD0BC  addi r10, r10, -0x2f44
	ctx.r[10].s64 = ctx.r[10].s64 + -12100;
	// 82EFC13C: 3929D0B0  addi r9, r9, -0x2f50
	ctx.r[9].s64 = ctx.r[9].s64 + -12112;
	// 82EFC140: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EFC144: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EFC148: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC14C: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82EFC150: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFC154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFC158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFC15C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFC160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFC168 size=8
    let mut pc: u32 = 0x82EFC168;
    'dispatch: loop {
        match pc {
            0x82EFC168 => {
    //   block [0x82EFC168..0x82EFC170)
	// 82EFC168: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82EFC16C: 48000004  b 0x82efc170
	sub_82EFC170(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC170 size=108
    let mut pc: u32 = 0x82EFC170;
    'dispatch: loop {
        match pc {
            0x82EFC170 => {
    //   block [0x82EFC170..0x82EFC1DC)
	// 82EFC170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFC178: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFC17C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFC180: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC184: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFC188: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFC18C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFC190: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 82EFC194: 409A0008  bne cr6, 0x82efc19c
	if !ctx.cr[6].eq {
	pc = 0x82EFC19C; continue 'dispatch;
	}
	// 82EFC198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFC19C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFC1A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC1A4: 394AD0A4  addi r10, r10, -0x2f5c
	ctx.r[10].s64 = ctx.r[10].s64 + -12124;
	// 82EFC1A8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EFC1AC: 4800186D  bl 0x82efda18
	ctx.lr = 0x82EFC1B0;
	sub_82EFDA18(ctx, base);
	// 82EFC1B0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFC1B4: 4182000C  beq 0x82efc1c0
	if ctx.cr[0].eq {
	pc = 0x82EFC1C0; continue 'dispatch;
	}
	// 82EFC1B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC1BC: 480016E5  bl 0x82efd8a0
	ctx.lr = 0x82EFC1C0;
	sub_82EFD8A0(ctx, base);
	// 82EFC1C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC1C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFC1C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFC1CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFC1D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFC1D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFC1D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC1E0 size=128
    let mut pc: u32 = 0x82EFC1E0;
    'dispatch: loop {
        match pc {
            0x82EFC1E0 => {
    //   block [0x82EFC1E0..0x82EFC260)
	// 82EFC1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC1E4: 4BDAD229  bl 0x82ca940c
	ctx.lr = 0x82EFC1E8;
	sub_82CA93D0(ctx, base);
	// 82EFC1E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC1EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFC1F0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EFC1F4: 4BFFFF1D  bl 0x82efc110
	ctx.lr = 0x82EFC1F8;
	sub_82EFC110(ctx, base);
	// 82EFC1F8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFC1FC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFC200: 396BD0DC  addi r11, r11, -0x2f24
	ctx.r[11].s64 = ctx.r[11].s64 + -12068;
	// 82EFC204: 394AD0D0  addi r10, r10, -0x2f30
	ctx.r[10].s64 = ctx.r[10].s64 + -12080;
	// 82EFC208: 3BBE0010  addi r29, r30, 0x10
	ctx.r[29].s64 = ctx.r[30].s64 + 16;
	// 82EFC20C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFC210: 915E0010  stw r10, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82EFC214: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFC218: 419A0018  beq cr6, 0x82efc230
	if ctx.cr[6].eq {
	pc = 0x82EFC230; continue 'dispatch;
	}
	// 82EFC21C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC220: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC224: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC228: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC22C: 4E800421  bctrl
	ctx.lr = 0x82EFC230;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC230: 93FE0014  stw r31, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 82EFC234: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFC238: 419A001C  beq cr6, 0x82efc254
	if ctx.cr[6].eq {
	pc = 0x82EFC254; continue 'dispatch;
	}
	// 82EFC23C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC240: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EFC244: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC248: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFC24C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC250: 4E800421  bctrl
	ctx.lr = 0x82EFC254;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC254: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFC258: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFC25C: 4BDAD200  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFC260 size=8
    let mut pc: u32 = 0x82EFC260;
    'dispatch: loop {
        match pc {
            0x82EFC260 => {
    //   block [0x82EFC260..0x82EFC268)
	// 82EFC260: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82EFC264: 4800069C  b 0x82efc900
	sub_82EFC900(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC268 size=160
    let mut pc: u32 = 0x82EFC268;
    'dispatch: loop {
        match pc {
            0x82EFC268 => {
    //   block [0x82EFC268..0x82EFC308)
	// 82EFC268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC26C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFC270: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFC274: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFC278: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC27C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFC280: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFC284: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFC288: 396BD0DC  addi r11, r11, -0x2f24
	ctx.r[11].s64 = ctx.r[11].s64 + -12068;
	// 82EFC28C: 394AD0D0  addi r10, r10, -0x2f30
	ctx.r[10].s64 = ctx.r[10].s64 + -12080;
	// 82EFC290: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFC294: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 82EFC298: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFC29C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82EFC2A0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EFC2A4: 419A0018  beq cr6, 0x82efc2bc
	if ctx.cr[6].eq {
	pc = 0x82EFC2BC; continue 'dispatch;
	}
	// 82EFC2A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC2AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFC2B0: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EFC2B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC2B8: 4E800421  bctrl
	ctx.lr = 0x82EFC2BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC2BC: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFC2C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFC2C4: 419A0018  beq cr6, 0x82efc2dc
	if ctx.cr[6].eq {
	pc = 0x82EFC2DC; continue 'dispatch;
	}
	// 82EFC2C8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC2CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFC2D0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFC2D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC2D8: 4E800421  bctrl
	ctx.lr = 0x82EFC2DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC2DC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFC2E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC2E4: 396BD0A4  addi r11, r11, -0x2f5c
	ctx.r[11].s64 = ctx.r[11].s64 + -12124;
	// 82EFC2E8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFC2EC: 4800172D  bl 0x82efda18
	ctx.lr = 0x82EFC2F0;
	sub_82EFDA18(ctx, base);
	// 82EFC2F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFC2F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFC2F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFC2FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFC300: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFC304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC308 size=220
    let mut pc: u32 = 0x82EFC308;
    'dispatch: loop {
        match pc {
            0x82EFC308 => {
    //   block [0x82EFC308..0x82EFC3E4)
	// 82EFC308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC30C: 4BDAD101  bl 0x82ca940c
	ctx.lr = 0x82EFC310;
	sub_82CA93D0(ctx, base);
	// 82EFC310: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC314: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFC318: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EFC31C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFC320: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EFC324: 409A00B8  bne cr6, 0x82efc3dc
	if !ctx.cr[6].eq {
	pc = 0x82EFC3DC; continue 'dispatch;
	}
	// 82EFC328: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC32C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFC330: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC334: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC338: 4E800421  bctrl
	ctx.lr = 0x82EFC33C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC33C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFC340: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFC344: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFC348: 419A001C  beq cr6, 0x82efc364
	if ctx.cr[6].eq {
	pc = 0x82EFC364; continue 'dispatch;
	}
	// 82EFC34C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC350: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EFC354: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFC358: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFC35C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC360: 4E800421  bctrl
	ctx.lr = 0x82EFC364;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC364: 57CB003E  slwi r11, r30, 0
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFC368: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82EFC36C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFC370: 419A0068  beq cr6, 0x82efc3d8
	if ctx.cr[6].eq {
	pc = 0x82EFC3D8; continue 'dispatch;
	}
	// 82EFC374: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFC378: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 82EFC37C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EFC380: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFC384: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC388: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC38C: 4E800421  bctrl
	ctx.lr = 0x82EFC390;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC390: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFC394: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFC398: 4182001C  beq 0x82efc3b4
	if ctx.cr[0].eq {
	pc = 0x82EFC3B4; continue 'dispatch;
	}
	// 82EFC39C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC3A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFC3A4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFC3A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC3AC: 4E800421  bctrl
	ctx.lr = 0x82EFC3B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC3B0: 48000028  b 0x82efc3d8
	pc = 0x82EFC3D8; continue 'dispatch;
	// 82EFC3B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFC3B8: 419A0018  beq cr6, 0x82efc3d0
	if ctx.cr[6].eq {
	pc = 0x82EFC3D0; continue 'dispatch;
	}
	// 82EFC3BC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC3C0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFC3C4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFC3C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC3CC: 4E800421  bctrl
	ctx.lr = 0x82EFC3D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC3D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFC3D4: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EFC3D8: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFC3DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFC3E0: 4BDAD07C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC3E8 size=132
    let mut pc: u32 = 0x82EFC3E8;
    'dispatch: loop {
        match pc {
            0x82EFC3E8 => {
    //   block [0x82EFC3E8..0x82EFC46C)
	// 82EFC3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFC3F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFC3F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC3F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFC3FC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFC400: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EFC404: 419A0054  beq cr6, 0x82efc458
	if ctx.cr[6].eq {
	pc = 0x82EFC458; continue 'dispatch;
	}
	// 82EFC408: 2F050002  cmpwi cr6, r5, 2
	ctx.cr[6].compare_i32(ctx.r[5].s32, 2, &mut ctx.xer);
	// 82EFC40C: 409A004C  bne cr6, 0x82efc458
	if !ctx.cr[6].eq {
	pc = 0x82EFC458; continue 'dispatch;
	}
	// 82EFC410: 357FFFF0  addic. r11, r31, -0x10
	ctx.xer.ca = (ctx.r[31].u32 > (!(-16 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + -16;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFC414: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFC418: 40820008  bne 0x82efc420
	if !ctx.cr[0].eq {
	pc = 0x82EFC420; continue 'dispatch;
	}
	// 82EFC41C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFC420: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC424: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EFC428: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC42C: 4E800421  bctrl
	ctx.lr = 0x82EFC430;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC430: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFC434: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFC438: 419A0018  beq cr6, 0x82efc450
	if ctx.cr[6].eq {
	pc = 0x82EFC450; continue 'dispatch;
	}
	// 82EFC43C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC440: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFC444: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFC448: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC44C: 4E800421  bctrl
	ctx.lr = 0x82EFC450;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFC454: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EFC458: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFC45C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFC460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFC464: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFC468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC470 size=84
    let mut pc: u32 = 0x82EFC470;
    'dispatch: loop {
        match pc {
            0x82EFC470 => {
    //   block [0x82EFC470..0x82EFC4C4)
	// 82EFC470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC474: 4BDACF99  bl 0x82ca940c
	ctx.lr = 0x82EFC478;
	sub_82CA93D0(ctx, base);
	// 82EFC478: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC47C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFC480: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EFC484: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82EFC488: 4BFFFD59  bl 0x82efc1e0
	ctx.lr = 0x82EFC48C;
	sub_82EFC1E0(ctx, base);
	// 82EFC48C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFC490: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFC494: 392BD0FC  addi r9, r11, -0x2f04
	ctx.r[9].s64 = ctx.r[11].s64 + -12036;
	// 82EFC498: 394AD0F0  addi r10, r10, -0x2f10
	ctx.r[10].s64 = ctx.r[10].s64 + -12048;
	// 82EFC49C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFC4A0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EFC4A4: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82EFC4A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC4AC: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82EFC4B0: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82EFC4B4: 93BF0020  stw r29, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 82EFC4B8: 997F0024  stb r11, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 82EFC4BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFC4C0: 4BDACF9C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFC4C8 size=8
    let mut pc: u32 = 0x82EFC4C8;
    'dispatch: loop {
        match pc {
            0x82EFC4C8 => {
    //   block [0x82EFC4C8..0x82EFC4D0)
	// 82EFC4C8: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82EFC4CC: 480004F4  b 0x82efc9c0
	sub_82EFC9C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC4D0 size=104
    let mut pc: u32 = 0x82EFC4D0;
    'dispatch: loop {
        match pc {
            0x82EFC4D0 => {
    //   block [0x82EFC4D0..0x82EFC538)
	// 82EFC4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFC4D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFC4DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC4E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFC4E4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFC4E8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFC4EC: 396BD0FC  addi r11, r11, -0x2f04
	ctx.r[11].s64 = ctx.r[11].s64 + -12036;
	// 82EFC4F0: 394AD0F0  addi r10, r10, -0x2f10
	ctx.r[10].s64 = ctx.r[10].s64 + -12048;
	// 82EFC4F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFC4F8: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82EFC4FC: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFC500: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFC504: 419A0018  beq cr6, 0x82efc51c
	if ctx.cr[6].eq {
	pc = 0x82EFC51C; continue 'dispatch;
	}
	// 82EFC508: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC50C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFC510: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFC514: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC518: 4E800421  bctrl
	ctx.lr = 0x82EFC51C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC51C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC520: 4BFFFD49  bl 0x82efc268
	ctx.lr = 0x82EFC524;
	sub_82EFC268(ctx, base);
	// 82EFC524: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFC528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFC52C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFC530: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFC534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFC538 size=16
    let mut pc: u32 = 0x82EFC538;
    'dispatch: loop {
        match pc {
            0x82EFC538 => {
    //   block [0x82EFC538..0x82EFC548)
	// 82EFC538: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EFC53C: 806B001C  lwz r3, 0x1c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFC540: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFC544: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFC548 size=20
    let mut pc: u32 = 0x82EFC548;
    'dispatch: loop {
        match pc {
            0x82EFC548 => {
    //   block [0x82EFC548..0x82EFC55C)
	// 82EFC548: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFC54C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFC550: 419A000C  beq cr6, 0x82efc55c
	if ctx.cr[6].eq {
		sub_82EFC55C(ctx, base);
		return;
	}
	// 82EFC554: 806B0014  lwz r3, 0x14(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFC558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC55C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFC55C size=8
    let mut pc: u32 = 0x82EFC55C;
    'dispatch: loop {
        match pc {
            0x82EFC55C => {
    //   block [0x82EFC55C..0x82EFC564)
	// 82EFC55C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFC560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFC568 size=16
    let mut pc: u32 = 0x82EFC568;
    'dispatch: loop {
        match pc {
            0x82EFC568 => {
    //   block [0x82EFC568..0x82EFC578)
	// 82EFC568: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EFC56C: 806B0020  lwz r3, 0x20(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFC570: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFC574: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFC578 size=20
    let mut pc: u32 = 0x82EFC578;
    'dispatch: loop {
        match pc {
            0x82EFC578 => {
    //   block [0x82EFC578..0x82EFC58C)
	// 82EFC578: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFC57C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFC580: 419A000C  beq cr6, 0x82efc58c
	if ctx.cr[6].eq {
		sub_82EFC58C(ctx, base);
		return;
	}
	// 82EFC584: 806B0018  lwz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFC588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC58C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFC58C size=8
    let mut pc: u32 = 0x82EFC58C;
    'dispatch: loop {
        match pc {
            0x82EFC58C => {
    //   block [0x82EFC58C..0x82EFC594)
	// 82EFC58C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFC590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC598 size=232
    let mut pc: u32 = 0x82EFC598;
    'dispatch: loop {
        match pc {
            0x82EFC598 => {
    //   block [0x82EFC598..0x82EFC680)
	// 82EFC598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFC5A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFC5A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFC5A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC5AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFC5B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFC5B4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFC5B8: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82EFC5BC: 419A00AC  beq cr6, 0x82efc668
	if ctx.cr[6].eq {
	pc = 0x82EFC668; continue 'dispatch;
	}
	// 82EFC5C0: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFC5C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFC5C8: 419A0018  beq cr6, 0x82efc5e0
	if ctx.cr[6].eq {
	pc = 0x82EFC5E0; continue 'dispatch;
	}
	// 82EFC5CC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC5D0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFC5D4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFC5D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC5DC: 4E800421  bctrl
	ctx.lr = 0x82EFC5E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC5E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFC5E4: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82EFC5E8: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFC5EC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EFC5F0: 419A0018  beq cr6, 0x82efc608
	if ctx.cr[6].eq {
	pc = 0x82EFC608; continue 'dispatch;
	}
	// 82EFC5F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC5F8: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82EFC5FC: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EFC600: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC604: 4E800421  bctrl
	ctx.lr = 0x82EFC608;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC608: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EFC60C: 419A0038  beq cr6, 0x82efc644
	if ctx.cr[6].eq {
	pc = 0x82EFC644; continue 'dispatch;
	}
	// 82EFC610: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC614: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82EFC618: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFC61C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFC620: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC624: 4E800421  bctrl
	ctx.lr = 0x82EFC628;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC628: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EFC62C: 419A0018  beq cr6, 0x82efc644
	if ctx.cr[6].eq {
	pc = 0x82EFC644; continue 'dispatch;
	}
	// 82EFC630: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC634: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFC638: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC63C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC640: 4E800421  bctrl
	ctx.lr = 0x82EFC644;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC644: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFC648: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFC64C: 419A0018  beq cr6, 0x82efc664
	if ctx.cr[6].eq {
	pc = 0x82EFC664; continue 'dispatch;
	}
	// 82EFC650: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC654: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFC658: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFC65C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC660: 4E800421  bctrl
	ctx.lr = 0x82EFC664;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC664: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82EFC668: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFC66C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFC670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFC674: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFC678: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFC67C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC680 size=292
    let mut pc: u32 = 0x82EFC680;
    'dispatch: loop {
        match pc {
            0x82EFC680 => {
    //   block [0x82EFC680..0x82EFC7A4)
	// 82EFC680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC684: 4BDACD85  bl 0x82ca9408
	ctx.lr = 0x82EFC688;
	sub_82CA93D0(ctx, base);
	// 82EFC688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC68C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFC690: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EFC694: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFC698: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EFC69C: 409A0100  bne cr6, 0x82efc79c
	if !ctx.cr[6].eq {
	pc = 0x82EFC79C; continue 'dispatch;
	}
	// 82EFC6A0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFC6A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFC6A8: 419A00F4  beq cr6, 0x82efc79c
	if ctx.cr[6].eq {
	pc = 0x82EFC79C; continue 'dispatch;
	}
	// 82EFC6AC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC6B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFC6B4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC6B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC6BC: 4E800421  bctrl
	ctx.lr = 0x82EFC6C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC6C0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFC6C4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFC6C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFC6CC: 419A001C  beq cr6, 0x82efc6e8
	if ctx.cr[6].eq {
	pc = 0x82EFC6E8; continue 'dispatch;
	}
	// 82EFC6D0: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC6D4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EFC6D8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFC6DC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFC6E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC6E4: 4E800421  bctrl
	ctx.lr = 0x82EFC6E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC6E8: 57CB003E  slwi r11, r30, 0
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFC6EC: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82EFC6F0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82EFC6F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFC6F8: 419A0064  beq cr6, 0x82efc75c
	if ctx.cr[6].eq {
	pc = 0x82EFC75C; continue 'dispatch;
	}
	// 82EFC6FC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFC700: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 82EFC704: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EFC708: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFC70C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC710: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC714: 4E800421  bctrl
	ctx.lr = 0x82EFC718;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC718: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFC71C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFC720: 4182001C  beq 0x82efc73c
	if ctx.cr[0].eq {
	pc = 0x82EFC73C; continue 'dispatch;
	}
	// 82EFC724: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC728: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFC72C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFC730: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC734: 4E800421  bctrl
	ctx.lr = 0x82EFC738;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC738: 48000024  b 0x82efc75c
	pc = 0x82EFC75C; continue 'dispatch;
	// 82EFC73C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFC740: 419A0018  beq cr6, 0x82efc758
	if ctx.cr[6].eq {
	pc = 0x82EFC758; continue 'dispatch;
	}
	// 82EFC744: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC748: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFC74C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFC750: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC754: 4E800421  bctrl
	ctx.lr = 0x82EFC758;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC758: 939F0014  stw r28, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 82EFC75C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFC760: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFC764: 419A0034  beq cr6, 0x82efc798
	if ctx.cr[6].eq {
	pc = 0x82EFC798; continue 'dispatch;
	}
	// 82EFC768: 897F0024  lbz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFC76C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFC770: 41820028  beq 0x82efc798
	if ctx.cr[0].eq {
	pc = 0x82EFC798; continue 'dispatch;
	}
	// 82EFC774: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFC778: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFC77C: 419A0018  beq cr6, 0x82efc794
	if ctx.cr[6].eq {
	pc = 0x82EFC794; continue 'dispatch;
	}
	// 82EFC780: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC784: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFC788: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFC78C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC790: 4E800421  bctrl
	ctx.lr = 0x82EFC794;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC794: 939F0018  stw r28, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[28].u32 ) };
	// 82EFC798: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFC79C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFC7A0: 4BDACCB8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC7A8 size=160
    let mut pc: u32 = 0x82EFC7A8;
    'dispatch: loop {
        match pc {
            0x82EFC7A8 => {
    //   block [0x82EFC7A8..0x82EFC848)
	// 82EFC7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC7AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFC7B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFC7B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFC7B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC7BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFC7C0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EFC7C4: 4BFFFC25  bl 0x82efc3e8
	ctx.lr = 0x82EFC7C8;
	sub_82EFC3E8(ctx, base);
	// 82EFC7C8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFC7CC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EFC7D0: 419A0060  beq cr6, 0x82efc830
	if ctx.cr[6].eq {
	pc = 0x82EFC830; continue 'dispatch;
	}
	// 82EFC7D4: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 82EFC7D8: 409A0058  bne cr6, 0x82efc830
	if !ctx.cr[6].eq {
	pc = 0x82EFC830; continue 'dispatch;
	}
	// 82EFC7DC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC7E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFC7E4: 409A004C  bne cr6, 0x82efc830
	if !ctx.cr[6].eq {
	pc = 0x82EFC830; continue 'dispatch;
	}
	// 82EFC7E8: 357FFFF0  addic. r11, r31, -0x10
	ctx.xer.ca = (ctx.r[31].u32 > (!(-16 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + -16;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFC7EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFC7F0: 40820008  bne 0x82efc7f8
	if !ctx.cr[0].eq {
	pc = 0x82EFC7F8; continue 'dispatch;
	}
	// 82EFC7F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFC7F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC7FC: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EFC800: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC804: 4E800421  bctrl
	ctx.lr = 0x82EFC808;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC808: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFC80C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFC810: 419A0018  beq cr6, 0x82efc828
	if ctx.cr[6].eq {
	pc = 0x82EFC828; continue 'dispatch;
	}
	// 82EFC814: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC818: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFC81C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFC820: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC824: 4E800421  bctrl
	ctx.lr = 0x82EFC828;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFC82C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EFC830: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFC834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFC838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFC83C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFC840: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFC844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC848 size=100
    let mut pc: u32 = 0x82EFC848;
    'dispatch: loop {
        match pc {
            0x82EFC848 => {
    //   block [0x82EFC848..0x82EFC8AC)
	// 82EFC848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFC850: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC854: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EFC858: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFC85C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EFC860: 419A0038  beq cr6, 0x82efc898
	if ctx.cr[6].eq {
	pc = 0x82EFC898; continue 'dispatch;
	}
	// 82EFC864: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC868: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EFC86C: 419A002C  beq cr6, 0x82efc898
	if ctx.cr[6].eq {
	pc = 0x82EFC898; continue 'dispatch;
	}
	// 82EFC870: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC874: 388A0010  addi r4, r10, 0x10
	ctx.r[4].s64 = ctx.r[10].s64 + 16;
	// 82EFC878: 80CB0010  lwz r6, 0x10(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFC87C: 80AB000C  lwz r5, 0xc(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFC880: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC884: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC888: 4E800421  bctrl
	ctx.lr = 0x82EFC88C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC88C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFC890: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EFC894: 40820008  bne 0x82efc89c
	if !ctx.cr[0].eq {
	pc = 0x82EFC89C; continue 'dispatch;
	}
	// 82EFC898: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFC89C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFC8A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFC8A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFC8A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC8B0 size=80
    let mut pc: u32 = 0x82EFC8B0;
    'dispatch: loop {
        match pc {
            0x82EFC8B0 => {
    //   block [0x82EFC8B0..0x82EFC900)
	// 82EFC8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFC8B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFC8BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC8C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFC8C4: 4BFFF84D  bl 0x82efc110
	ctx.lr = 0x82EFC8C8;
	sub_82EFC110(ctx, base);
	// 82EFC8C8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFC8CC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFC8D0: 396BD0DC  addi r11, r11, -0x2f24
	ctx.r[11].s64 = ctx.r[11].s64 + -12068;
	// 82EFC8D4: 394AD0D0  addi r10, r10, -0x2f30
	ctx.r[10].s64 = ctx.r[10].s64 + -12080;
	// 82EFC8D8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82EFC8DC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFC8E0: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82EFC8E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC8E8: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82EFC8EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFC8F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFC8F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFC8F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFC8FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC900 size=76
    let mut pc: u32 = 0x82EFC900;
    'dispatch: loop {
        match pc {
            0x82EFC900 => {
    //   block [0x82EFC900..0x82EFC94C)
	// 82EFC900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFC908: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFC90C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFC910: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC914: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFC918: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFC91C: 4BFFF94D  bl 0x82efc268
	ctx.lr = 0x82EFC920;
	sub_82EFC268(ctx, base);
	// 82EFC920: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFC924: 4182000C  beq 0x82efc930
	if ctx.cr[0].eq {
	pc = 0x82EFC930; continue 'dispatch;
	}
	// 82EFC928: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC92C: 48000F75  bl 0x82efd8a0
	ctx.lr = 0x82EFC930;
	sub_82EFD8A0(ctx, base);
	// 82EFC930: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC934: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFC938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFC93C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFC940: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFC944: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFC948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC950 size=112
    let mut pc: u32 = 0x82EFC950;
    'dispatch: loop {
        match pc {
            0x82EFC950 => {
    //   block [0x82EFC950..0x82EFC9C0)
	// 82EFC950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC954: 4BDACAB9  bl 0x82ca940c
	ctx.lr = 0x82EFC958;
	sub_82CA93D0(ctx, base);
	// 82EFC958: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC95C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFC960: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFC964: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EFC968: 4BFFFF49  bl 0x82efc8b0
	ctx.lr = 0x82EFC96C;
	sub_82EFC8B0(ctx, base);
	// 82EFC96C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFC970: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFC974: 396BD0FC  addi r11, r11, -0x2f04
	ctx.r[11].s64 = ctx.r[11].s64 + -12036;
	// 82EFC978: 394AD0F0  addi r10, r10, -0x2f10
	ctx.r[10].s64 = ctx.r[10].s64 + -12048;
	// 82EFC97C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFC980: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EFC984: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82EFC988: 419A0018  beq cr6, 0x82efc9a0
	if ctx.cr[6].eq {
	pc = 0x82EFC9A0; continue 'dispatch;
	}
	// 82EFC98C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFC990: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFC994: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFC998: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFC99C: 4E800421  bctrl
	ctx.lr = 0x82EFC9A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFC9A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFC9A4: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82EFC9A8: 9BBF0024  stb r29, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[29].u8 ) };
	// 82EFC9AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC9B0: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EFC9B4: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82EFC9B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFC9BC: 4BDACAA0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFC9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFC9C0 size=76
    let mut pc: u32 = 0x82EFC9C0;
    'dispatch: loop {
        match pc {
            0x82EFC9C0 => {
    //   block [0x82EFC9C0..0x82EFCA0C)
	// 82EFC9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFC9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFC9C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFC9CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFC9D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFC9D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFC9D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFC9DC: 4BFFFAF5  bl 0x82efc4d0
	ctx.lr = 0x82EFC9E0;
	sub_82EFC4D0(ctx, base);
	// 82EFC9E0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFC9E4: 4182000C  beq 0x82efc9f0
	if ctx.cr[0].eq {
	pc = 0x82EFC9F0; continue 'dispatch;
	}
	// 82EFC9E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC9EC: 48000EB5  bl 0x82efd8a0
	ctx.lr = 0x82EFC9F0;
	sub_82EFD8A0(ctx, base);
	// 82EFC9F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFC9F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFC9F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFC9FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFCA00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFCA04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFCA08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFCA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFCA10 size=116
    let mut pc: u32 = 0x82EFCA10;
    'dispatch: loop {
        match pc {
            0x82EFCA10 => {
    //   block [0x82EFCA10..0x82EFCA84)
	// 82EFCA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFCA14: 4BDAC9F1  bl 0x82ca9404
	ctx.lr = 0x82EFCA18;
	sub_82CA93D0(ctx, base);
	// 82EFCA18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFCA1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFCA20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFCA24: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EFCA28: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82EFCA2C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82EFCA30: 4BFFFE81  bl 0x82efc8b0
	ctx.lr = 0x82EFCA34;
	sub_82EFC8B0(ctx, base);
	// 82EFCA34: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFCA38: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFCA3C: 396BD0FC  addi r11, r11, -0x2f04
	ctx.r[11].s64 = ctx.r[11].s64 + -12036;
	// 82EFCA40: 394AD0F0  addi r10, r10, -0x2f10
	ctx.r[10].s64 = ctx.r[10].s64 + -12048;
	// 82EFCA44: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFCA48: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EFCA4C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82EFCA50: 419A0018  beq cr6, 0x82efca68
	if ctx.cr[6].eq {
	pc = 0x82EFCA68; continue 'dispatch;
	}
	// 82EFCA54: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFCA58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFCA5C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCA60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFCA64: 4E800421  bctrl
	ctx.lr = 0x82EFCA68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFCA68: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82EFCA6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFCA70: 93BF001C  stw r29, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 82EFCA74: 939F0020  stw r28, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[28].u32 ) };
	// 82EFCA78: 9B7F0024  stb r27, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[27].u8 ) };
	// 82EFCA7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFCA80: 4BDAC9D4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFCA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFCA88 size=100
    let mut pc: u32 = 0x82EFCA88;
    'dispatch: loop {
        match pc {
            0x82EFCA88 => {
    //   block [0x82EFCA88..0x82EFCAEC)
	// 82EFCA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFCA8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFCA90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFCA94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFCA98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFCA9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCAA0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EFCAA4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EFCAA8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCAAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFCAB0: 4E800421  bctrl
	ctx.lr = 0x82EFCAB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFCAB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFCAB8: 4182001C  beq 0x82efcad4
	if ctx.cr[0].eq {
	pc = 0x82EFCAD4; continue 'dispatch;
	}
	// 82EFCABC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCAC0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EFCAC4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFCAC8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFCACC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFCAD0: 4E800421  bctrl
	ctx.lr = 0x82EFCAD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFCAD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFCAD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFCADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFCAE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFCAE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFCAE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFCAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFCAF0 size=92
    let mut pc: u32 = 0x82EFCAF0;
    'dispatch: loop {
        match pc {
            0x82EFCAF0 => {
    //   block [0x82EFCAF0..0x82EFCB4C)
	// 82EFCAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFCAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFCAF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFCAFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFCB00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCB04: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EFCB08: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCB0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFCB10: 4E800421  bctrl
	ctx.lr = 0x82EFCB14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFCB14: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFCB18: 4182001C  beq 0x82efcb34
	if ctx.cr[0].eq {
	pc = 0x82EFCB34; continue 'dispatch;
	}
	// 82EFCB1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCB20: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFCB24: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFCB28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFCB2C: 4E800421  bctrl
	ctx.lr = 0x82EFCB30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFCB30: 48000008  b 0x82efcb38
	pc = 0x82EFCB38; continue 'dispatch;
	// 82EFCB34: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFCB38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFCB3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFCB40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFCB44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFCB48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFCB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFCB50 size=84
    let mut pc: u32 = 0x82EFCB50;
    'dispatch: loop {
        match pc {
            0x82EFCB50 => {
    //   block [0x82EFCB50..0x82EFCBA4)
	// 82EFCB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFCB54: 4BDAC8B9  bl 0x82ca940c
	ctx.lr = 0x82EFCB58;
	sub_82CA93D0(ctx, base);
	// 82EFCB58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFCB5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCB60: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EFCB64: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EFCB68: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82EFCB6C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCB70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFCB74: 4E800421  bctrl
	ctx.lr = 0x82EFCB78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFCB78: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFCB7C: 41820020  beq 0x82efcb9c
	if ctx.cr[0].eq {
	pc = 0x82EFCB9C; continue 'dispatch;
	}
	// 82EFCB80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCB84: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82EFCB88: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EFCB8C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFCB90: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFCB94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFCB98: 4E800421  bctrl
	ctx.lr = 0x82EFCB9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFCB9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFCBA0: 4BDAC8BC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFCBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFCBA8 size=68
    let mut pc: u32 = 0x82EFCBA8;
    'dispatch: loop {
        match pc {
            0x82EFCBA8 => {
    //   block [0x82EFCBA8..0x82EFCBEC)
	// 82EFCBA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFCBAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFCBB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFCBB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFCBB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFCBBC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFCBC0: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EFCBC4: 396BD110  addi r11, r11, -0x2ef0
	ctx.r[11].s64 = ctx.r[11].s64 + -12016;
	// 82EFCBC8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFCBCC: 41820008  beq 0x82efcbd4
	if ctx.cr[0].eq {
	pc = 0x82EFCBD4; continue 'dispatch;
	}
	// 82EFCBD0: 4B948BE1  bl 0x828457b0
	ctx.lr = 0x82EFCBD4;
	sub_828457B0(ctx, base);
	// 82EFCBD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFCBD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFCBDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFCBE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFCBE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFCBE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFCBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFCBF0 size=28
    let mut pc: u32 = 0x82EFCBF0;
    'dispatch: loop {
        match pc {
            0x82EFCBF0 => {
    //   block [0x82EFCBF0..0x82EFCC0C)
	// 82EFCBF0: 2B030055  cmplwi cr6, r3, 0x55
	ctx.cr[6].compare_u32(ctx.r[3].u32, 85 as u32, &mut ctx.xer);
	// 82EFCBF4: 40980018  bge cr6, 0x82efcc0c
	if !ctx.cr[6].lt {
		sub_82EFCC0C(ctx, base);
		return;
	}
	// 82EFCBF8: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82EFCBFC: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EFCC00: 396BE2D0  addi r11, r11, -0x1d30
	ctx.r[11].s64 = ctx.r[11].s64 + -7472;
	// 82EFCC04: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EFCC08: 48000030  b 0x82efcc38
	sub_82EFCC34(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFCC0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFCC0C size=40
    let mut pc: u32 = 0x82EFCC0C;
    'dispatch: loop {
        match pc {
            0x82EFCC0C => {
    //   block [0x82EFCC0C..0x82EFCC34)
	// 82EFCC0C: 2B0303E8  cmplwi cr6, r3, 0x3e8
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1000 as u32, &mut ctx.xer);
	// 82EFCC10: 41980024  blt cr6, 0x82efcc34
	if ctx.cr[6].lt {
		sub_82EFCC34(ctx, base);
		return;
	}
	// 82EFCC14: 2B0303ED  cmplwi cr6, r3, 0x3ed
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1005 as u32, &mut ctx.xer);
	// 82EFCC18: 4098001C  bge cr6, 0x82efcc34
	if !ctx.cr[6].lt {
		sub_82EFCC34(ctx, base);
		return;
	}
	// 82EFCC1C: 3963FC18  addi r11, r3, -0x3e8
	ctx.r[11].s64 = ctx.r[3].s64 + -1000;
	// 82EFCC20: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 82EFCC24: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFCC28: 394AE424  addi r10, r10, -0x1bdc
	ctx.r[10].s64 = ctx.r[10].s64 + -7132;
	// 82EFCC2C: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EFCC30: 48000008  b 0x82efcc38
	sub_82EFCC34(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFCC34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFCC34 size=28
    let mut pc: u32 = 0x82EFCC34;
    'dispatch: loop {
        match pc {
            0x82EFCC34 => {
    //   block [0x82EFCC34..0x82EFCC50)
	// 82EFCC34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFCC38: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFCC3C: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFCC40: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82EFCC44: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82EFCC48: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82EFCC4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFCC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFCC50 size=52
    let mut pc: u32 = 0x82EFCC50;
    'dispatch: loop {
        match pc {
            0x82EFCC50 => {
    //   block [0x82EFCC50..0x82EFCC84)
	// 82EFCC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFCC54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFCC58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFCC5C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EFCC60: 480068D9  bl 0x82f03538
	ctx.lr = 0x82EFCC64;
	sub_82F03538(ctx, base);
	// 82EFCC64: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFCC68: E8610050  ld r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82EFCC6C: 40820008  bne 0x82efcc74
	if !ctx.cr[0].eq {
	pc = 0x82EFCC74; continue 'dispatch;
	}
	// 82EFCC70: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82EFCC74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFCC78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFCC7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFCC80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFCC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFCC88 size=160
    let mut pc: u32 = 0x82EFCC88;
    'dispatch: loop {
        match pc {
            0x82EFCC88 => {
    //   block [0x82EFCC88..0x82EFCD28)
	// 82EFCC88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFCC8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFCC90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFCC94: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82EFCC98: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFCC9C: 419A0050  beq cr6, 0x82efccec
	if ctx.cr[6].eq {
	pc = 0x82EFCCEC; continue 'dispatch;
	}
	// 82EFCCA0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCCA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFCCA8: 419A0044  beq cr6, 0x82efccec
	if ctx.cr[6].eq {
	pc = 0x82EFCCEC; continue 'dispatch;
	}
	// 82EFCCAC: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 82EFCCB0: 48007E69  bl 0x82f04b18
	ctx.lr = 0x82EFCCB4;
	sub_82F04B18(ctx, base);
	// 82EFCCB4: 2B03002F  cmplwi cr6, r3, 0x2f
	ctx.cr[6].compare_u32(ctx.r[3].u32, 47 as u32, &mut ctx.xer);
	// 82EFCCB8: 419A0034  beq cr6, 0x82efccec
	if ctx.cr[6].eq {
	pc = 0x82EFCCEC; continue 'dispatch;
	}
	// 82EFCCBC: 2B03005C  cmplwi cr6, r3, 0x5c
	ctx.cr[6].compare_u32(ctx.r[3].u32, 92 as u32, &mut ctx.xer);
	// 82EFCCC0: 419A002C  beq cr6, 0x82efccec
	if ctx.cr[6].eq {
	pc = 0x82EFCCEC; continue 'dispatch;
	}
	// 82EFCCC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFCCC8: 419A0058  beq cr6, 0x82efcd20
	if ctx.cr[6].eq {
	pc = 0x82EFCD20; continue 'dispatch;
	}
	// 82EFCCCC: 2B03003A  cmplwi cr6, r3, 0x3a
	ctx.cr[6].compare_u32(ctx.r[3].u32, 58 as u32, &mut ctx.xer);
	// 82EFCCD0: 409A0030  bne cr6, 0x82efcd00
	if !ctx.cr[6].eq {
	pc = 0x82EFCD00; continue 'dispatch;
	}
	// 82EFCCD4: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 82EFCCD8: 48007E41  bl 0x82f04b18
	ctx.lr = 0x82EFCCDC;
	sub_82F04B18(ctx, base);
	// 82EFCCDC: 2B03002F  cmplwi cr6, r3, 0x2f
	ctx.cr[6].compare_u32(ctx.r[3].u32, 47 as u32, &mut ctx.xer);
	// 82EFCCE0: 419A000C  beq cr6, 0x82efccec
	if ctx.cr[6].eq {
	pc = 0x82EFCCEC; continue 'dispatch;
	}
	// 82EFCCE4: 2B03005C  cmplwi cr6, r3, 0x5c
	ctx.cr[6].compare_u32(ctx.r[3].u32, 92 as u32, &mut ctx.xer);
	// 82EFCCE8: 409A0028  bne cr6, 0x82efcd10
	if !ctx.cr[6].eq {
	pc = 0x82EFCD10; continue 'dispatch;
	}
	// 82EFCCEC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EFCCF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFCCF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFCCF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFCCFC: 4E800020  blr
	return;
	// 82EFCD00: 2B03002F  cmplwi cr6, r3, 0x2f
	ctx.cr[6].compare_u32(ctx.r[3].u32, 47 as u32, &mut ctx.xer);
	// 82EFCD04: 419A001C  beq cr6, 0x82efcd20
	if ctx.cr[6].eq {
	pc = 0x82EFCD20; continue 'dispatch;
	}
	// 82EFCD08: 2B03005C  cmplwi cr6, r3, 0x5c
	ctx.cr[6].compare_u32(ctx.r[3].u32, 92 as u32, &mut ctx.xer);
	// 82EFCD0C: 419A0014  beq cr6, 0x82efcd20
	if ctx.cr[6].eq {
	pc = 0x82EFCD20; continue 'dispatch;
	}
	// 82EFCD10: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 82EFCD14: 48007E05  bl 0x82f04b18
	ctx.lr = 0x82EFCD18;
	sub_82F04B18(ctx, base);
	// 82EFCD18: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFCD1C: 4082FFB0  bne 0x82efcccc
	if !ctx.cr[0].eq {
	pc = 0x82EFCCCC; continue 'dispatch;
	}
	// 82EFCD20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFCD24: 4BFFFFCC  b 0x82efccf0
	pc = 0x82EFCCF0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFCD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFCD28 size=100
    let mut pc: u32 = 0x82EFCD28;
    'dispatch: loop {
        match pc {
            0x82EFCD28 => {
    //   block [0x82EFCD28..0x82EFCD8C)
	// 82EFCD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFCD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFCD30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFCD34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFCD38: 2F04000A  cmpwi cr6, r4, 0xa
	ctx.cr[6].compare_i32(ctx.r[4].s32, 10, &mut ctx.xer);
	// 82EFCD3C: 419A0034  beq cr6, 0x82efcd70
	if ctx.cr[6].eq {
	pc = 0x82EFCD70; continue 'dispatch;
	}
	// 82EFCD40: 2F04000D  cmpwi cr6, r4, 0xd
	ctx.cr[6].compare_i32(ctx.r[4].s32, 13, &mut ctx.xer);
	// 82EFCD44: 419A0014  beq cr6, 0x82efcd58
	if ctx.cr[6].eq {
	pc = 0x82EFCD58; continue 'dispatch;
	}
	// 82EFCD48: 2F04000E  cmpwi cr6, r4, 0xe
	ctx.cr[6].compare_i32(ctx.r[4].s32, 14, &mut ctx.xer);
	// 82EFCD4C: 409A002C  bne cr6, 0x82efcd78
	if !ctx.cr[6].eq {
	pc = 0x82EFCD78; continue 'dispatch;
	}
	// 82EFCD50: 4800A039  bl 0x82f06d88
	ctx.lr = 0x82EFCD54;
	sub_82F06D88(ctx, base);
	// 82EFCD54: 48000020  b 0x82efcd74
	pc = 0x82EFCD74; continue 'dispatch;
	// 82EFCD58: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 82EFCD5C: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 82EFCD60: 419A0008  beq cr6, 0x82efcd68
	if ctx.cr[6].eq {
	pc = 0x82EFCD68; continue 'dispatch;
	}
	// 82EFCD64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFCD68: 480093B1  bl 0x82f06118
	ctx.lr = 0x82EFCD6C;
	sub_82F06118(ctx, base);
	// 82EFCD6C: 48000008  b 0x82efcd74
	pc = 0x82EFCD74; continue 'dispatch;
	// 82EFCD70: 48009A71  bl 0x82f067e0
	ctx.lr = 0x82EFCD74;
	sub_82F067E0(ctx, base);
	// 82EFCD74: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EFCD78: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EFCD7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFCD80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFCD84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFCD88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFCD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFCD90 size=128
    let mut pc: u32 = 0x82EFCD90;
    'dispatch: loop {
        match pc {
            0x82EFCD90 => {
    //   block [0x82EFCD90..0x82EFCE10)
	// 82EFCD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFCD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFCD98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFCD9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFCDA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFCDA4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFCDA8: 396BD128  addi r11, r11, -0x2ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -11992;
	// 82EFCDAC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFCDB0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFCDB4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFCDB8: 419A0018  beq cr6, 0x82efcdd0
	if ctx.cr[6].eq {
	pc = 0x82EFCDD0; continue 'dispatch;
	}
	// 82EFCDBC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFCDC0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFCDC4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFCDC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFCDCC: 4E800421  bctrl
	ctx.lr = 0x82EFCDD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFCDD0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFCDD4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFCDD8: 419A0018  beq cr6, 0x82efcdf0
	if ctx.cr[6].eq {
	pc = 0x82EFCDF0; continue 'dispatch;
	}
	// 82EFCDDC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFCDE0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFCDE4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFCDE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFCDEC: 4E800421  bctrl
	ctx.lr = 0x82EFCDF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFCDF0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFCDF4: 396BD110  addi r11, r11, -0x2ef0
	ctx.r[11].s64 = ctx.r[11].s64 + -12016;
	// 82EFCDF8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFCDFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFCE00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFCE04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFCE08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFCE0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFCE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFCE10 size=16
    let mut pc: u32 = 0x82EFCE10;
    'dispatch: loop {
        match pc {
            0x82EFCE10 => {
    //   block [0x82EFCE10..0x82EFCE20)
	// 82EFCE10: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFCE14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFCE18: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82EFCE1C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFCE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFCE20 size=8
    let mut pc: u32 = 0x82EFCE20;
    'dispatch: loop {
        match pc {
            0x82EFCE20 => {
    //   block [0x82EFCE20..0x82EFCE28)
	// 82EFCE20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFCE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFCE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFCE28 size=72
    let mut pc: u32 = 0x82EFCE28;
    'dispatch: loop {
        match pc {
            0x82EFCE28 => {
    //   block [0x82EFCE28..0x82EFCE70)
	// 82EFCE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFCE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFCE30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFCE34: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFCE38: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82EFCE3C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EFCE40: 419A0018  beq cr6, 0x82efce58
	if ctx.cr[6].eq {
	pc = 0x82EFCE58; continue 'dispatch;
	}
	// 82EFCE44: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EFCE48: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EFCE4C: 4BFFFDA5  bl 0x82efcbf0
	ctx.lr = 0x82EFCE50;
	sub_82EFCBF0(ctx, base);
	// 82EFCE50: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82EFCE54: 48000008  b 0x82efce5c
	pc = 0x82EFCE5C; continue 'dispatch;
	// 82EFCE58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFCE5C: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82EFCE60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFCE64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFCE68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFCE6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFCE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFCE70 size=96
    let mut pc: u32 = 0x82EFCE70;
    'dispatch: loop {
        match pc {
            0x82EFCE70 => {
    //   block [0x82EFCE70..0x82EFCED0)
	// 82EFCE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFCE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFCE78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFCE7C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EFCE80: 419A003C  beq cr6, 0x82efcebc
	if ctx.cr[6].eq {
	pc = 0x82EFCEBC; continue 'dispatch;
	}
	// 82EFCE84: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCE88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFCE8C: 419A0030  beq cr6, 0x82efcebc
	if ctx.cr[6].eq {
	pc = 0x82EFCEBC; continue 'dispatch;
	}
	// 82EFCE90: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82EFCE94: 419A0028  beq cr6, 0x82efcebc
	if ctx.cr[6].eq {
	pc = 0x82EFCEBC; continue 'dispatch;
	}
	// 82EFCE98: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFCE9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFCEA0: 419A0010  beq cr6, 0x82efceb0
	if ctx.cr[6].eq {
	pc = 0x82EFCEB0; continue 'dispatch;
	}
	// 82EFCEA4: 4800BAB5  bl 0x82f08958
	ctx.lr = 0x82EFCEA8;
	sub_82F08958(ctx, base);
	// 82EFCEA8: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82EFCEAC: 48000008  b 0x82efceb4
	pc = 0x82EFCEB4; continue 'dispatch;
	// 82EFCEB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFCEB4: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82EFCEB8: 48000008  b 0x82efcec0
	pc = 0x82EFCEC0; continue 'dispatch;
	// 82EFCEBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFCEC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFCEC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFCEC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFCECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFCED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFCED0 size=72
    let mut pc: u32 = 0x82EFCED0;
    'dispatch: loop {
        match pc {
            0x82EFCED0 => {
    //   block [0x82EFCED0..0x82EFCF18)
	// 82EFCED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFCED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFCED8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFCEDC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EFCEE0: 419A0024  beq cr6, 0x82efcf04
	if ctx.cr[6].eq {
	pc = 0x82EFCF04; continue 'dispatch;
	}
	// 82EFCEE4: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCEE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFCEEC: 419A0018  beq cr6, 0x82efcf04
	if ctx.cr[6].eq {
	pc = 0x82EFCF04; continue 'dispatch;
	}
	// 82EFCEF0: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFCEF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFCEF8: 419A000C  beq cr6, 0x82efcf04
	if ctx.cr[6].eq {
	pc = 0x82EFCF04; continue 'dispatch;
	}
	// 82EFCEFC: 4800D11D  bl 0x82f0a018
	ctx.lr = 0x82EFCF00;
	sub_82F0A018(ctx, base);
	// 82EFCF00: 48000008  b 0x82efcf08
	pc = 0x82EFCF08; continue 'dispatch;
	// 82EFCF04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFCF08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFCF0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFCF10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFCF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFCF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFCF18 size=108
    let mut pc: u32 = 0x82EFCF18;
    'dispatch: loop {
        match pc {
            0x82EFCF18 => {
    //   block [0x82EFCF18..0x82EFCF84)
	// 82EFCF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFCF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFCF20: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 82EFCF24: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82EFCF28: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 82EFCF2C: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82EFCF30: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82EFCF34: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82EFCF38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFCF3C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EFCF40: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82EFCF44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFCF48: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82EFCF4C: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82EFCF50: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EFCF54: 409A0008  bne cr6, 0x82efcf5c
	if !ctx.cr[6].eq {
	pc = 0x82EFCF5C; continue 'dispatch;
	}
	// 82EFCF58: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFCF5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFCF60: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82EFCF64: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFCF68: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFCF6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFCF70: 4E800421  bctrl
	ctx.lr = 0x82EFCF74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFCF74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFCF78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFCF7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFCF80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFCF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFCF88 size=132
    let mut pc: u32 = 0x82EFCF88;
    'dispatch: loop {
        match pc {
            0x82EFCF88 => {
    //   block [0x82EFCF88..0x82EFD00C)
	// 82EFCF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFCF8C: 4BDAC47D  bl 0x82ca9408
	ctx.lr = 0x82EFCF90;
	sub_82CA93D0(ctx, base);
	// 82EFCF90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFCF94: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EFCF98: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFCF9C: 578B07BD  rlwinm. r11, r28, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFCFA0: 41820048  beq 0x82efcfe8
	if ctx.cr[0].eq {
	pc = 0x82EFCFE8; continue 'dispatch;
	}
	// 82EFCFA4: 815EFFFC  lwz r10, -4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82EFCFA8: 3BBEFFFC  addi r29, r30, -4
	ctx.r[29].s64 = ctx.r[30].s64 + -4;
	// 82EFCFAC: 554B2036  slwi r11, r10, 4
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFCFB0: 37EAFFFF  addic. r31, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFCFB4: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EFCFB8: 41800018  blt 0x82efcfd0
	if ctx.cr[0].lt {
	pc = 0x82EFCFD0; continue 'dispatch;
	}
	// 82EFCFBC: 3BDEFFF0  addi r30, r30, -0x10
	ctx.r[30].s64 = ctx.r[30].s64 + -16;
	// 82EFCFC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFCFC4: 48000A55  bl 0x82efda18
	ctx.lr = 0x82EFCFC8;
	sub_82EFDA18(ctx, base);
	// 82EFCFC8: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFCFCC: 4080FFF0  bge 0x82efcfbc
	if !ctx.cr[0].lt {
	pc = 0x82EFCFBC; continue 'dispatch;
	}
	// 82EFCFD0: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFCFD4: 4182000C  beq 0x82efcfe0
	if ctx.cr[0].eq {
	pc = 0x82EFCFE0; continue 'dispatch;
	}
	// 82EFCFD8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFCFDC: 4B9487D5  bl 0x828457b0
	ctx.lr = 0x82EFCFE0;
	sub_828457B0(ctx, base);
	// 82EFCFE0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFCFE4: 48000020  b 0x82efd004
	pc = 0x82EFD004; continue 'dispatch;
	// 82EFCFE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFCFEC: 48000A2D  bl 0x82efda18
	ctx.lr = 0x82EFCFF0;
	sub_82EFDA18(ctx, base);
	// 82EFCFF0: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFCFF4: 4182000C  beq 0x82efd000
	if ctx.cr[0].eq {
	pc = 0x82EFD000; continue 'dispatch;
	}
	// 82EFCFF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFCFFC: 480008A5  bl 0x82efd8a0
	ctx.lr = 0x82EFD000;
	sub_82EFD8A0(ctx, base);
	// 82EFD000: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFD004: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFD008: 4BDAC450  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFD010 size=88
    let mut pc: u32 = 0x82EFD010;
    'dispatch: loop {
        match pc {
            0x82EFD010 => {
    //   block [0x82EFD010..0x82EFD068)
	// 82EFD010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFD014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFD018: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFD01C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFD020: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFD024: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFD028: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFD02C: 4800091D  bl 0x82efd948
	ctx.lr = 0x82EFD030;
	sub_82EFD948(ctx, base);
	// 82EFD030: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFD034: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82EFD038: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFD03C: 396BD140  addi r11, r11, -0x2ec0
	ctx.r[11].s64 = ctx.r[11].s64 + -11968;
	// 82EFD040: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EFD044: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFD048: 48000A11  bl 0x82efda58
	ctx.lr = 0x82EFD04C;
	sub_82EFDA58(ctx, base);
	// 82EFD04C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFD050: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFD054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFD058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFD05C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFD060: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFD064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFD068 size=80
    let mut pc: u32 = 0x82EFD068;
    'dispatch: loop {
        match pc {
            0x82EFD068 => {
    //   block [0x82EFD068..0x82EFD0B8)
	// 82EFD068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFD06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFD070: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFD074: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFD078: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFD07C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFD080: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 82EFD084: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFD088: 4BFFFF89  bl 0x82efd010
	ctx.lr = 0x82EFD08C;
	sub_82EFD010(ctx, base);
	// 82EFD08C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFD090: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82EFD094: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFD098: 396BD140  addi r11, r11, -0x2ec0
	ctx.r[11].s64 = ctx.r[11].s64 + -11968;
	// 82EFD09C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFD0A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFD0A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFD0A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFD0AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFD0B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFD0B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFD0B8 size=132
    let mut pc: u32 = 0x82EFD0B8;
    'dispatch: loop {
        match pc {
            0x82EFD0B8 => {
    //   block [0x82EFD0B8..0x82EFD13C)
	// 82EFD0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFD0BC: 4BDAC34D  bl 0x82ca9408
	ctx.lr = 0x82EFD0C0;
	sub_82CA93D0(ctx, base);
	// 82EFD0C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFD0C4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EFD0C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFD0CC: 578B07BD  rlwinm. r11, r28, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFD0D0: 41820048  beq 0x82efd118
	if ctx.cr[0].eq {
	pc = 0x82EFD118; continue 'dispatch;
	}
	// 82EFD0D4: 815EFFFC  lwz r10, -4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82EFD0D8: 3BBEFFFC  addi r29, r30, -4
	ctx.r[29].s64 = ctx.r[30].s64 + -4;
	// 82EFD0DC: 1D6A000C  mulli r11, r10, 0xc
	ctx.r[11].s64 = ctx.r[10].s64 * 12;
	// 82EFD0E0: 37EAFFFF  addic. r31, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFD0E4: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EFD0E8: 41800018  blt 0x82efd100
	if ctx.cr[0].lt {
	pc = 0x82EFD100; continue 'dispatch;
	}
	// 82EFD0EC: 3BDEFFF4  addi r30, r30, -0xc
	ctx.r[30].s64 = ctx.r[30].s64 + -12;
	// 82EFD0F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFD0F4: 4BFFFC9D  bl 0x82efcd90
	ctx.lr = 0x82EFD0F8;
	sub_82EFCD90(ctx, base);
	// 82EFD0F8: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFD0FC: 4080FFF0  bge 0x82efd0ec
	if !ctx.cr[0].lt {
	pc = 0x82EFD0EC; continue 'dispatch;
	}
	// 82EFD100: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFD104: 4182000C  beq 0x82efd110
	if ctx.cr[0].eq {
	pc = 0x82EFD110; continue 'dispatch;
	}
	// 82EFD108: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFD10C: 4B9486A5  bl 0x828457b0
	ctx.lr = 0x82EFD110;
	sub_828457B0(ctx, base);
	// 82EFD110: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFD114: 48000020  b 0x82efd134
	pc = 0x82EFD134; continue 'dispatch;
	// 82EFD118: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFD11C: 4BFFFC75  bl 0x82efcd90
	ctx.lr = 0x82EFD120;
	sub_82EFCD90(ctx, base);
	// 82EFD120: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFD124: 4182000C  beq 0x82efd130
	if ctx.cr[0].eq {
	pc = 0x82EFD130; continue 'dispatch;
	}
	// 82EFD128: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFD12C: 4B948685  bl 0x828457b0
	ctx.lr = 0x82EFD130;
	sub_828457B0(ctx, base);
	// 82EFD130: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFD134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFD138: 4BDAC320  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFD140 size=152
    let mut pc: u32 = 0x82EFD140;
    'dispatch: loop {
        match pc {
            0x82EFD140 => {
    //   block [0x82EFD140..0x82EFD1D8)
	// 82EFD140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFD144: 4BDAC2C9  bl 0x82ca940c
	ctx.lr = 0x82EFD148;
	sub_82CA93D0(ctx, base);
	// 82EFD148: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFD14C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD150: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFD154: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EFD158: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD15C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFD160: 4E800421  bctrl
	ctx.lr = 0x82EFD164;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFD164: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFD168: 41820028  beq 0x82efd190
	if ctx.cr[0].eq {
	pc = 0x82EFD190; continue 'dispatch;
	}
	// 82EFD16C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD170: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFD174: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EFD178: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFD17C: 4E800421  bctrl
	ctx.lr = 0x82EFD180;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFD180: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EFD184: 4082000C  bne 0x82efd190
	if !ctx.cr[0].eq {
	pc = 0x82EFD190; continue 'dispatch;
	}
	// 82EFD188: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFD18C: 48000044  b 0x82efd1d0
	pc = 0x82EFD1D0; continue 'dispatch;
	// 82EFD190: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EFD194: 419A0018  beq cr6, 0x82efd1ac
	if ctx.cr[6].eq {
	pc = 0x82EFD1AC; continue 'dispatch;
	}
	// 82EFD198: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFD19C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EFD1A0: 388BD144  addi r4, r11, -0x2ebc
	ctx.r[4].s64 = ctx.r[11].s64 + -11964;
	// 82EFD1A4: 387D0014  addi r3, r29, 0x14
	ctx.r[3].s64 = ctx.r[29].s64 + 20;
	// 82EFD1A8: 4BFFFD71  bl 0x82efcf18
	ctx.lr = 0x82EFD1AC;
	sub_82EFCF18(ctx, base);
	// 82EFD1AC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFD1B0: 419A001C  beq cr6, 0x82efd1cc
	if ctx.cr[6].eq {
	pc = 0x82EFD1CC; continue 'dispatch;
	}
	// 82EFD1B4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD1B8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFD1BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFD1C0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD1C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFD1C8: 4E800421  bctrl
	ctx.lr = 0x82EFD1CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFD1CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFD1D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFD1D4: 4BDAC288  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFD1D8 size=160
    let mut pc: u32 = 0x82EFD1D8;
    'dispatch: loop {
        match pc {
            0x82EFD1D8 => {
    //   block [0x82EFD1D8..0x82EFD278)
	// 82EFD1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFD1DC: 4BDAC22D  bl 0x82ca9408
	ctx.lr = 0x82EFD1E0;
	sub_82CA93D0(ctx, base);
	// 82EFD1E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFD1E4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EFD1E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFD1EC: 3B9F0004  addi r28, r31, 4
	ctx.r[28].s64 = ctx.r[31].s64 + 4;
	// 82EFD1F0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD1F4: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82EFD1F8: 4BFFFA91  bl 0x82efcc88
	ctx.lr = 0x82EFD1FC;
	sub_82EFCC88(ctx, base);
	// 82EFD1FC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFD200: 40820064  bne 0x82efd264
	if !ctx.cr[0].eq {
	pc = 0x82EFD264; continue 'dispatch;
	}
	// 82EFD204: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82EFD208: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD20C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD210: 557D007F  clrlwi. r29, r11, 1
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82EFD214: 41820050  beq 0x82efd264
	if ctx.cr[0].eq {
	pc = 0x82EFD264; continue 'dispatch;
	}
	// 82EFD218: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFD21C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFD220: 48001579  bl 0x82efe798
	ctx.lr = 0x82EFD224;
	sub_82EFE798(ctx, base);
	// 82EFD224: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD228: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82EFD22C: 896B000B  lbz r11, 0xb(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(11 as u32) ) } as u64;
	// 82EFD230: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82EFD234: 2B0B005C  cmplwi cr6, r11, 0x5c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 92 as u32, &mut ctx.xer);
	// 82EFD238: 419A001C  beq cr6, 0x82efd254
	if ctx.cr[6].eq {
	pc = 0x82EFD254; continue 'dispatch;
	}
	// 82EFD23C: 2B0B002F  cmplwi cr6, r11, 0x2f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 47 as u32, &mut ctx.xer);
	// 82EFD240: 419A0014  beq cr6, 0x82efd254
	if ctx.cr[6].eq {
	pc = 0x82EFD254; continue 'dispatch;
	}
	// 82EFD244: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EFD248: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFD24C: 388B0C04  addi r4, r11, 0xc04
	ctx.r[4].s64 = ctx.r[11].s64 + 3076;
	// 82EFD250: 48001731  bl 0x82efe980
	ctx.lr = 0x82EFD254;
	sub_82EFE980(ctx, base);
	// 82EFD254: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EFD258: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFD25C: 480015DD  bl 0x82efe838
	ctx.lr = 0x82EFD260;
	sub_82EFE838(ctx, base);
	// 82EFD260: 48000010  b 0x82efd270
	pc = 0x82EFD270; continue 'dispatch;
	// 82EFD264: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EFD268: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFD26C: 4800152D  bl 0x82efe798
	ctx.lr = 0x82EFD270;
	sub_82EFE798(ctx, base);
	// 82EFD270: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFD274: 4BDAC1E4  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFD278 size=208
    let mut pc: u32 = 0x82EFD278;
    'dispatch: loop {
        match pc {
            0x82EFD278 => {
    //   block [0x82EFD278..0x82EFD348)
	// 82EFD278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFD27C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFD280: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFD284: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFD288: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFD28C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFD290: 480008E9  bl 0x82efdb78
	ctx.lr = 0x82EFD294;
	sub_82EFDB78(ctx, base);
	// 82EFD294: 37C3FFFF  addic. r30, r3, -1
	ctx.xer.ca = (ctx.r[3].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[3].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EFD298: 41800088  blt 0x82efd320
	if ctx.cr[0].lt {
	pc = 0x82EFD320; continue 'dispatch;
	}
	// 82EFD29C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFD2A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFD2A4: 48000945  bl 0x82efdbe8
	ctx.lr = 0x82EFD2A8;
	sub_82EFDBE8(ctx, base);
	// 82EFD2A8: 2B03002F  cmplwi cr6, r3, 0x2f
	ctx.cr[6].compare_u32(ctx.r[3].u32, 47 as u32, &mut ctx.xer);
	// 82EFD2AC: 419A0018  beq cr6, 0x82efd2c4
	if ctx.cr[6].eq {
	pc = 0x82EFD2C4; continue 'dispatch;
	}
	// 82EFD2B0: 2B03005C  cmplwi cr6, r3, 0x5c
	ctx.cr[6].compare_u32(ctx.r[3].u32, 92 as u32, &mut ctx.xer);
	// 82EFD2B4: 419A0010  beq cr6, 0x82efd2c4
	if ctx.cr[6].eq {
	pc = 0x82EFD2C4; continue 'dispatch;
	}
	// 82EFD2B8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EFD2BC: 4080FFE0  bge 0x82efd29c
	if !ctx.cr[0].lt {
	pc = 0x82EFD29C; continue 'dispatch;
	}
	// 82EFD2C0: 48000060  b 0x82efd320
	pc = 0x82EFD320; continue 'dispatch;
	// 82EFD2C4: 38DE0001  addi r6, r30, 1
	ctx.r[6].s64 = ctx.r[30].s64 + 1;
	// 82EFD2C8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFD2CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFD2D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EFD2D4: 48001AF5  bl 0x82efedc8
	ctx.lr = 0x82EFD2D8;
	sub_82EFEDC8(ctx, base);
	// 82EFD2D8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EFD2DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFD2E0: 480014B9  bl 0x82efe798
	ctx.lr = 0x82EFD2E4;
	sub_82EFE798(ctx, base);
	// 82EFD2E4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFD2E8: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82EFD2EC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EFD2F0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82EFD2F4: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82EFD2F8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFD2FC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EFD300: 7D285214  add r9, r8, r10
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82EFD304: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFD308: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFD30C: 4082FFE8  bne 0x82efd2f4
	if !ctx.cr[0].eq {
	pc = 0x82EFD2F4; continue 'dispatch;
	}
	// 82EFD310: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EFD314: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFD318: 40820008  bne 0x82efd320
	if !ctx.cr[0].eq {
	pc = 0x82EFD320; continue 'dispatch;
	}
	// 82EFD31C: 48000585  bl 0x82efd8a0
	ctx.lr = 0x82EFD320;
	sub_82EFD8A0(ctx, base);
	// 82EFD320: 7FCB0034  cntlzw r11, r30
	ctx.r[11].u64 = if ctx.r[30].u32 == 0 { 32 } else { ctx.r[30].u32.leading_zeros() as u64 };
	// 82EFD324: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82EFD328: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82EFD32C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82EFD330: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFD334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFD338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFD33C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFD340: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFD344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFD348 size=712
    let mut pc: u32 = 0x82EFD348;
    'dispatch: loop {
        match pc {
            0x82EFD348 => {
    //   block [0x82EFD348..0x82EFD610)
	// 82EFD348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFD34C: 4BDAC0B5  bl 0x82ca9400
	ctx.lr = 0x82EFD350;
	sub_82CA93D0(ctx, base);
	// 82EFD350: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFD354: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFD358: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82EFD35C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD360: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EFD364: 419A00BC  beq cr6, 0x82efd420
	if ctx.cr[6].eq {
	pc = 0x82EFD420; continue 'dispatch;
	}
	// 82EFD368: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82EFD36C: 419A003C  beq cr6, 0x82efd3a8
	if ctx.cr[6].eq {
	pc = 0x82EFD3A8; continue 'dispatch;
	}
	// 82EFD370: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 82EFD374: 480004ED  bl 0x82efd860
	ctx.lr = 0x82EFD378;
	sub_82EFD860(ctx, base);
	// 82EFD378: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFD37C: 41820288  beq 0x82efd604
	if ctx.cr[0].eq {
	pc = 0x82EFD604; continue 'dispatch;
	}
	// 82EFD380: 3D605647  lis r11, 0x5647
	ctx.r[11].s64 = 1447493632;
	// 82EFD384: 3D409FE1  lis r10, -0x601f
	ctx.r[10].s64 = -1612644352;
	// 82EFD388: 616B1E89  ori r11, r11, 0x1e89
	ctx.r[11].u64 = ctx.r[11].u64 | 7817;
	// 82EFD38C: 614A234A  ori r10, r10, 0x234a
	ctx.r[10].u64 = ctx.r[10].u64 | 9034;
	// 82EFD390: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EFD394: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFD398: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EFD39C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFD3A0: 4BFFF5B1  bl 0x82efc950
	ctx.lr = 0x82EFD3A4;
	sub_82EFC950(ctx, base);
	// 82EFD3A4: 48000264  b 0x82efd608
	pc = 0x82EFD608; continue 'dispatch;
	// 82EFD3A8: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFD3AC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFD3B0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD3B4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFD3B8: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82EFD3BC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD3C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFD3C4: 4E800421  bctrl
	ctx.lr = 0x82EFD3C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFD3C8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFD3CC: 41820238  beq 0x82efd604
	if ctx.cr[0].eq {
	pc = 0x82EFD604; continue 'dispatch;
	}
	// 82EFD3D0: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFD3D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFD3D8: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD3DC: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFD3E0: 4BFFF949  bl 0x82efcd28
	ctx.lr = 0x82EFD3E4;
	sub_82EFCD28(ctx, base);
	// 82EFD3E4: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82EFD3E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFD3EC: 40820010  bne 0x82efd3fc
	if !ctx.cr[0].eq {
	pc = 0x82EFD3FC; continue 'dispatch;
	}
	// 82EFD3F0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD3F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFD3F8: 48000200  b 0x82efd5f8
	pc = 0x82EFD5F8; continue 'dispatch;
	// 82EFD3FC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFD400: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFD404: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD408: A3AB001C  lhz r29, 0x1c(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFD40C: A38B001E  lhz r28, 0x1e(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(30 as u32) ) } as u64;
	// 82EFD410: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD414: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFD418: 4E800421  bctrl
	ctx.lr = 0x82EFD41C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFD41C: 48000030  b 0x82efd44c
	pc = 0x82EFD44C; continue 'dispatch;
	// 82EFD420: 83FE000C  lwz r31, 0xc(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFD424: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFD428: 419A0018  beq cr6, 0x82efd440
	if ctx.cr[6].eq {
	pc = 0x82EFD440; continue 'dispatch;
	}
	// 82EFD42C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD430: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFD434: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD438: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFD43C: 4E800421  bctrl
	ctx.lr = 0x82EFD440;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFD440: 83BF0014  lwz r29, 0x14(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFD444: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 82EFD448: 839F0018  lwz r28, 0x18(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFD44C: 897A0014  lbz r11, 0x14(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFD450: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFD454: 4182006C  beq 0x82efd4c0
	if ctx.cr[0].eq {
	pc = 0x82EFD4C0; continue 'dispatch;
	}
	// 82EFD458: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 82EFD45C: 48000405  bl 0x82efd860
	ctx.lr = 0x82EFD460;
	sub_82EFD860(ctx, base);
	// 82EFD460: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFD464: 41820034  beq 0x82efd498
	if ctx.cr[0].eq {
	pc = 0x82EFD498; continue 'dispatch;
	}
	// 82EFD468: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82EFD46C: 3D605647  lis r11, 0x5647
	ctx.r[11].s64 = 1447493632;
	// 82EFD470: 3D409FE1  lis r10, -0x601f
	ctx.r[10].s64 = -1612644352;
	// 82EFD474: 616B1E89  ori r11, r11, 0x1e89
	ctx.r[11].u64 = ctx.r[11].u64 | 7817;
	// 82EFD478: 614A234A  ori r10, r10, 0x234a
	ctx.r[10].u64 = ctx.r[10].u64 | 9034;
	// 82EFD47C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EFD480: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82EFD484: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EFD488: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EFD48C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EFD490: 4BFFF581  bl 0x82efca10
	ctx.lr = 0x82EFD494;
	sub_82EFCA10(ctx, base);
	// 82EFD494: 48000008  b 0x82efd49c
	pc = 0x82EFD49C; continue 'dispatch;
	// 82EFD498: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFD49C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFD4A0: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD4A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFD4A8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EFD4AC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD4B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFD4B4: 4E800421  bctrl
	ctx.lr = 0x82EFD4B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFD4B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFD4BC: 4800014C  b 0x82efd608
	pc = 0x82EFD608; continue 'dispatch;
	// 82EFD4C0: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFD4C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFD4C8: 419A0124  beq cr6, 0x82efd5ec
	if ctx.cr[6].eq {
	pc = 0x82EFD5EC; continue 'dispatch;
	}
	// 82EFD4CC: 806B0014  lwz r3, 0x14(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFD4D0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82EFD4D4: 419A0118  beq cr6, 0x82efd5ec
	if ctx.cr[6].eq {
	pc = 0x82EFD5EC; continue 'dispatch;
	}
	// 82EFD4D8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFD4DC: 556A03DF  rlwinm. r10, r11, 0, 0xf, 0xf
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EFD4E0: 4082FF78  bne 0x82efd458
	if !ctx.cr[0].eq {
	pc = 0x82EFD458; continue 'dispatch;
	}
	// 82EFD4E4: 895E0018  lbz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFD4E8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFD4EC: 41820024  beq 0x82efd510
	if ctx.cr[0].eq {
	pc = 0x82EFD510; continue 'dispatch;
	}
	// 82EFD4F0: 556B02D7  rlwinm. r11, r11, 0, 0xb, 0xb
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFD4F4: 4082001C  bne 0x82efd510
	if !ctx.cr[0].eq {
	pc = 0x82EFD510; continue 'dispatch;
	}
	// 82EFD4F8: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 82EFD4FC: 48000365  bl 0x82efd860
	ctx.lr = 0x82EFD500;
	sub_82EFD860(ctx, base);
	// 82EFD500: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFD504: 4182FF94  beq 0x82efd498
	if ctx.cr[0].eq {
	pc = 0x82EFD498; continue 'dispatch;
	}
	// 82EFD508: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82EFD50C: 4BFFFF60  b 0x82efd46c
	pc = 0x82EFD46C; continue 'dispatch;
	// 82EFD510: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD514: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD518: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFD51C: 4E800421  bctrl
	ctx.lr = 0x82EFD520;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFD520: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFD524: 418200C0  beq 0x82efd5e4
	if ctx.cr[0].eq {
	pc = 0x82EFD5E4; continue 'dispatch;
	}
	// 82EFD528: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD52C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82EFD530: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EFD534: 389B0010  addi r4, r27, 0x10
	ctx.r[4].s64 = ctx.r[27].s64 + 16;
	// 82EFD538: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFD53C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD540: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFD544: 4E800421  bctrl
	ctx.lr = 0x82EFD548;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFD548: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFD54C: 41820080  beq 0x82efd5cc
	if ctx.cr[0].eq {
	pc = 0x82EFD5CC; continue 'dispatch;
	}
	// 82EFD550: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 82EFD554: 4800030D  bl 0x82efd860
	ctx.lr = 0x82EFD558;
	sub_82EFD860(ctx, base);
	// 82EFD558: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFD55C: 41820034  beq 0x82efd590
	if ctx.cr[0].eq {
	pc = 0x82EFD590; continue 'dispatch;
	}
	// 82EFD560: 3D605647  lis r11, 0x5647
	ctx.r[11].s64 = 1447493632;
	// 82EFD564: 3D409FE1  lis r10, -0x601f
	ctx.r[10].s64 = -1612644352;
	// 82EFD568: 616B1E89  ori r11, r11, 0x1e89
	ctx.r[11].u64 = ctx.r[11].u64 | 7817;
	// 82EFD56C: 614A234A  ori r10, r10, 0x234a
	ctx.r[10].u64 = ctx.r[10].u64 | 9034;
	// 82EFD570: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EFD574: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82EFD578: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82EFD57C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EFD580: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFD584: 4BFFEEED  bl 0x82efc470
	ctx.lr = 0x82EFD588;
	sub_82EFC470(ctx, base);
	// 82EFD588: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFD58C: 48000008  b 0x82efd594
	pc = 0x82EFD594; continue 'dispatch;
	// 82EFD590: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EFD594: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD598: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFD59C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFD5A0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD5A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFD5A8: 4E800421  bctrl
	ctx.lr = 0x82EFD5AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFD5AC: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD5B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFD5B4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EFD5B8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD5BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFD5C0: 4E800421  bctrl
	ctx.lr = 0x82EFD5C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFD5C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFD5C8: 48000040  b 0x82efd608
	pc = 0x82EFD608; continue 'dispatch;
	// 82EFD5CC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD5D0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFD5D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFD5D8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD5DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFD5E0: 4E800421  bctrl
	ctx.lr = 0x82EFD5E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFD5E4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EFD5E8: 4BFFFEB8  b 0x82efd4a0
	pc = 0x82EFD4A0; continue 'dispatch;
	// 82EFD5EC: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD5F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFD5F4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EFD5F8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD5FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFD600: 4E800421  bctrl
	ctx.lr = 0x82EFD604;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFD604: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFD608: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EFD60C: 4BDABE44  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFD610 size=292
    let mut pc: u32 = 0x82EFD610;
    'dispatch: loop {
        match pc {
            0x82EFD610 => {
    //   block [0x82EFD610..0x82EFD734)
	// 82EFD610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFD614: 4BDABDF1  bl 0x82ca9404
	ctx.lr = 0x82EFD618;
	sub_82CA93D0(ctx, base);
	// 82EFD618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFD61C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFD620: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFD624: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82EFD628: 396BD128  addi r11, r11, -0x2ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -11992;
	// 82EFD62C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EFD630: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFD634: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EFD638: 48000229  bl 0x82efd860
	ctx.lr = 0x82EFD63C;
	sub_82EFD860(ctx, base);
	// 82EFD63C: 3D605647  lis r11, 0x5647
	ctx.r[11].s64 = 1447493632;
	// 82EFD640: 3D409FE1  lis r10, -0x601f
	ctx.r[10].s64 = -1612644352;
	// 82EFD644: 617D1E89  ori r29, r11, 0x1e89
	ctx.r[29].u64 = ctx.r[11].u64 | 7817;
	// 82EFD648: 615E234A  ori r30, r10, 0x234a
	ctx.r[30].u64 = ctx.r[10].u64 | 9034;
	// 82EFD64C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFD650: 41820018  beq 0x82efd668
	if ctx.cr[0].eq {
	pc = 0x82EFD668; continue 'dispatch;
	}
	// 82EFD654: 93A30004  stw r29, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82EFD658: 93C30008  stw r30, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82EFD65C: 4BFFB4F5  bl 0x82ef8b50
	ctx.lr = 0x82EFD660;
	sub_82EF8B50(ctx, base);
	// 82EFD660: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EFD664: 48000008  b 0x82efd66c
	pc = 0x82EFD66C; continue 'dispatch;
	// 82EFD668: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFD66C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFD670: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 82EFD674: 480001ED  bl 0x82efd860
	ctx.lr = 0x82EFD678;
	sub_82EFD860(ctx, base);
	// 82EFD678: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFD67C: 41820018  beq 0x82efd694
	if ctx.cr[0].eq {
	pc = 0x82EFD694; continue 'dispatch;
	}
	// 82EFD680: 93A30004  stw r29, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82EFD684: 93C30008  stw r30, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82EFD688: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD68C: 4800D2A5  bl 0x82f0a930
	ctx.lr = 0x82EFD690;
	sub_82F0A930(ctx, base);
	// 82EFD690: 48000008  b 0x82efd698
	pc = 0x82EFD698; continue 'dispatch;
	// 82EFD694: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFD698: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82EFD69C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFD6A0: 419A0088  beq cr6, 0x82efd728
	if ctx.cr[6].eq {
	pc = 0x82EFD728; continue 'dispatch;
	}
	// 82EFD6A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD6A8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EFD6AC: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 82EFD6B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFD6B4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD6B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFD6BC: 4E800421  bctrl
	ctx.lr = 0x82EFD6C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFD6C0: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82EFD6C4: 4800019D  bl 0x82efd860
	ctx.lr = 0x82EFD6C8;
	sub_82EFD860(ctx, base);
	// 82EFD6C8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFD6CC: 4182001C  beq 0x82efd6e8
	if ctx.cr[0].eq {
	pc = 0x82EFD6E8; continue 'dispatch;
	}
	// 82EFD6D0: 93A30004  stw r29, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82EFD6D4: 5764063E  clrlwi r4, r27, 0x18
	ctx.r[4].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	// 82EFD6D8: 93C30008  stw r30, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82EFD6DC: 4BFFF98D  bl 0x82efd068
	ctx.lr = 0x82EFD6E0;
	sub_82EFD068(ctx, base);
	// 82EFD6E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFD6E4: 48000008  b 0x82efd6ec
	pc = 0x82EFD6EC; continue 'dispatch;
	// 82EFD6E8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EFD6EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD6F0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EFD6F4: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 82EFD6F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFD6FC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD700: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFD704: 4E800421  bctrl
	ctx.lr = 0x82EFD708;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFD708: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82EFD70C: 419A001C  beq cr6, 0x82efd728
	if ctx.cr[6].eq {
	pc = 0x82EFD728; continue 'dispatch;
	}
	// 82EFD710: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD714: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFD718: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFD71C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD720: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFD724: 4E800421  bctrl
	ctx.lr = 0x82EFD728;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFD728: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFD72C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFD730: 4BDABD24  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFD738 size=76
    let mut pc: u32 = 0x82EFD738;
    'dispatch: loop {
        match pc {
            0x82EFD738 => {
    //   block [0x82EFD738..0x82EFD784)
	// 82EFD738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFD73C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFD740: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFD744: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFD748: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFD74C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFD750: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFD754: 48009FC5  bl 0x82f07718
	ctx.lr = 0x82EFD758;
	sub_82F07718(ctx, base);
	// 82EFD758: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFD75C: 4182000C  beq 0x82efd768
	if ctx.cr[0].eq {
	pc = 0x82EFD768; continue 'dispatch;
	}
	// 82EFD760: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFD764: 4800013D  bl 0x82efd8a0
	ctx.lr = 0x82EFD768;
	sub_82EFD8A0(ctx, base);
	// 82EFD768: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFD76C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFD770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFD774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFD778: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFD77C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFD780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFD788 size=8
    let mut pc: u32 = 0x82EFD788;
    'dispatch: loop {
        match pc {
            0x82EFD788 => {
    //   block [0x82EFD788..0x82EFD790)
	// 82EFD788: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82EFD78C: 4BFFFFAC  b 0x82efd738
	sub_82EFD738(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFD790 size=8
    let mut pc: u32 = 0x82EFD790;
    'dispatch: loop {
        match pc {
            0x82EFD790 => {
    //   block [0x82EFD790..0x82EFD798)
	// 82EFD790: 3863FFE8  addi r3, r3, -0x18
	ctx.r[3].s64 = ctx.r[3].s64 + -24;
	// 82EFD794: 4BFFFFA4  b 0x82efd738
	sub_82EFD738(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFD798 size=16
    let mut pc: u32 = 0x82EFD798;
    'dispatch: loop {
        match pc {
            0x82EFD798 => {
    //   block [0x82EFD798..0x82EFD7A8)
	// 82EFD798: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFD79C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFD7A0: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82EFD7A4: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFD7A8 size=8
    let mut pc: u32 = 0x82EFD7A8;
    'dispatch: loop {
        match pc {
            0x82EFD7A8 => {
    //   block [0x82EFD7A8..0x82EFD7B0)
	// 82EFD7A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFD7AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFD7B0 size=68
    let mut pc: u32 = 0x82EFD7B0;
    'dispatch: loop {
        match pc {
            0x82EFD7B0 => {
    //   block [0x82EFD7B0..0x82EFD7F4)
	// 82EFD7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFD7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFD7B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFD7BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFD7C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFD7C4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFD7C8: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EFD7CC: 396BD1D4  addi r11, r11, -0x2e2c
	ctx.r[11].s64 = ctx.r[11].s64 + -11820;
	// 82EFD7D0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFD7D4: 41820008  beq 0x82efd7dc
	if ctx.cr[0].eq {
	pc = 0x82EFD7DC; continue 'dispatch;
	}
	// 82EFD7D8: 4B947FD9  bl 0x828457b0
	ctx.lr = 0x82EFD7DC;
	sub_828457B0(ctx, base);
	// 82EFD7DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFD7E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFD7E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFD7E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFD7EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFD7F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFD7F8 size=12
    let mut pc: u32 = 0x82EFD7F8;
    'dispatch: loop {
        match pc {
            0x82EFD7F8 => {
    //   block [0x82EFD7F8..0x82EFD804)
	// 82EFD7F8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EFD7FC: 906B8FC0  stw r3, -0x7040(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-28736 as u32), ctx.r[3].u32 ) };
	// 82EFD800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFD808 size=12
    let mut pc: u32 = 0x82EFD808;
    'dispatch: loop {
        match pc {
            0x82EFD808 => {
    //   block [0x82EFD808..0x82EFD814)
	// 82EFD808: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EFD80C: 806B8FC0  lwz r3, -0x7040(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28736 as u32) ) } as u64;
	// 82EFD810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFD818 size=20
    let mut pc: u32 = 0x82EFD818;
    'dispatch: loop {
        match pc {
            0x82EFD818 => {
    //   block [0x82EFD818..0x82EFD82C)
	// 82EFD818: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFD81C: 419A0010  beq cr6, 0x82efd82c
	if ctx.cr[6].eq {
		sub_82EFD82C(ctx, base);
		return;
	}
	// 82EFD820: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82EFD824: 906BE294  stw r3, -0x1d6c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-7532 as u32), ctx.r[3].u32 ) };
	// 82EFD828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD82C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFD82C size=20
    let mut pc: u32 = 0x82EFD82C;
    'dispatch: loop {
        match pc {
            0x82EFD82C => {
    //   block [0x82EFD82C..0x82EFD840)
	// 82EFD82C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82EFD830: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 82EFD834: 396B9010  addi r11, r11, -0x6ff0
	ctx.r[11].s64 = ctx.r[11].s64 + -28656;
	// 82EFD838: 916AE294  stw r11, -0x1d6c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7532 as u32), ctx.r[11].u32 ) };
	// 82EFD83C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFD840 size=12
    let mut pc: u32 = 0x82EFD840;
    'dispatch: loop {
        match pc {
            0x82EFD840 => {
    //   block [0x82EFD840..0x82EFD84C)
	// 82EFD840: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82EFD844: 806BE294  lwz r3, -0x1d6c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7532 as u32) ) } as u64;
	// 82EFD848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFD850 size=12
    let mut pc: u32 = 0x82EFD850;
    'dispatch: loop {
        match pc {
            0x82EFD850 => {
    //   block [0x82EFD850..0x82EFD85C)
	// 82EFD850: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82EFD854: 806BE29C  lwz r3, -0x1d64(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7524 as u32) ) } as u64;
	// 82EFD858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFD860 size=28
    let mut pc: u32 = 0x82EFD860;
    'dispatch: loop {
        match pc {
            0x82EFD860 => {
    //   block [0x82EFD860..0x82EFD87C)
	// 82EFD860: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82EFD864: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EFD868: 806BE294  lwz r3, -0x1d6c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7532 as u32) ) } as u64;
	// 82EFD86C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD870: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD874: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFD878: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFD880 size=32
    let mut pc: u32 = 0x82EFD880;
    'dispatch: loop {
        match pc {
            0x82EFD880 => {
    //   block [0x82EFD880..0x82EFD8A0)
	// 82EFD880: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82EFD884: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82EFD888: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EFD88C: 806BE294  lwz r3, -0x1d6c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7532 as u32) ) } as u64;
	// 82EFD890: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD894: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFD898: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFD89C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFD8A0 size=28
    let mut pc: u32 = 0x82EFD8A0;
    'dispatch: loop {
        match pc {
            0x82EFD8A0 => {
    //   block [0x82EFD8A0..0x82EFD8BC)
	// 82EFD8A0: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82EFD8A4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EFD8A8: 806BE294  lwz r3, -0x1d6c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7532 as u32) ) } as u64;
	// 82EFD8AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD8B0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD8B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFD8B8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFD8C0 size=36
    let mut pc: u32 = 0x82EFD8C0;
    'dispatch: loop {
        match pc {
            0x82EFD8C0 => {
    //   block [0x82EFD8C0..0x82EFD8E4)
	// 82EFD8C0: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82EFD8C4: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82EFD8C8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82EFD8CC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EFD8D0: 806BE294  lwz r3, -0x1d6c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7532 as u32) ) } as u64;
	// 82EFD8D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD8D8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFD8DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFD8E0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFD8E8 size=28
    let mut pc: u32 = 0x82EFD8E8;
    'dispatch: loop {
        match pc {
            0x82EFD8E8 => {
    //   block [0x82EFD8E8..0x82EFD904)
	// 82EFD8E8: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82EFD8EC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EFD8F0: 806BE294  lwz r3, -0x1d6c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7532 as u32) ) } as u64;
	// 82EFD8F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD8F8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFD8FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFD900: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFD908 size=16
    let mut pc: u32 = 0x82EFD908;
    'dispatch: loop {
        match pc {
            0x82EFD908 => {
    //   block [0x82EFD908..0x82EFD918)
	// 82EFD908: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD90C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EFD910: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EFD914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFD918 size=24
    let mut pc: u32 = 0x82EFD918;
    'dispatch: loop {
        match pc {
            0x82EFD918 => {
    //   block [0x82EFD918..0x82EFD930)
	// 82EFD918: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD91C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EFD920: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EFD924: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD928: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFD92C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFD930 size=20
    let mut pc: u32 = 0x82EFD930;
    'dispatch: loop {
        match pc {
            0x82EFD930 => {
    //   block [0x82EFD930..0x82EFD944)
	// 82EFD930: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD934: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EFD938: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFD93C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFD940: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD944(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFD944 size=4
    let mut pc: u32 = 0x82EFD944;
    'dispatch: loop {
        match pc {
            0x82EFD944 => {
    //   block [0x82EFD944..0x82EFD948)
	// 82EFD944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFD948 size=84
    let mut pc: u32 = 0x82EFD948;
    'dispatch: loop {
        match pc {
            0x82EFD948 => {
    //   block [0x82EFD948..0x82EFD99C)
	// 82EFD948: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFD94C: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD950: 3D005647  lis r8, 0x5647
	ctx.r[8].s64 = 1447493632;
	// 82EFD954: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFD958: 394AD124  addi r10, r10, -0x2edc
	ctx.r[10].s64 = ctx.r[10].s64 + -11996;
	// 82EFD95C: 61081E89  ori r8, r8, 0x1e89
	ctx.r[8].u64 = ctx.r[8].u64 | 7817;
	// 82EFD960: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EFD964: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EFD968: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EFD96C: 409A0030  bne cr6, 0x82efd99c
	if !ctx.cr[6].eq {
		sub_82EFD99C(ctx, base);
		return;
	}
	// 82EFD970: 3D409FE1  lis r10, -0x601f
	ctx.r[10].s64 = -1612644352;
	// 82EFD974: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD978: 614A234A  ori r10, r10, 0x234a
	ctx.r[10].u64 = ctx.r[10].u64 | 9034;
	// 82EFD97C: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EFD980: 409A001C  bne cr6, 0x82efd99c
	if !ctx.cr[6].eq {
		sub_82EFD99C(ctx, base);
		return;
	}
	// 82EFD984: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82EFD988: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EFD98C: 396BE2A4  addi r11, r11, -0x1d5c
	ctx.r[11].s64 = ctx.r[11].s64 + -7516;
	// 82EFD990: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EFD994: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82EFD998: 48000010  b 0x82efd9a8
	sub_82EFD99C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD99C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFD99C size=20
    let mut pc: u32 = 0x82EFD99C;
    'dispatch: loop {
        match pc {
            0x82EFD99C => {
    //   block [0x82EFD99C..0x82EFD9B0)
	// 82EFD99C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 82EFD9A0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EFD9A4: 396AE2A4  addi r11, r10, -0x1d5c
	ctx.r[11].s64 = ctx.r[10].s64 + -7516;
	// 82EFD9A8: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFD9AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFD9B0 size=76
    let mut pc: u32 = 0x82EFD9B0;
    'dispatch: loop {
        match pc {
            0x82EFD9B0 => {
    //   block [0x82EFD9B0..0x82EFD9FC)
	// 82EFD9B0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFD9B4: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFD9B8: 3D005647  lis r8, 0x5647
	ctx.r[8].s64 = 1447493632;
	// 82EFD9BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFD9C0: 394AD124  addi r10, r10, -0x2edc
	ctx.r[10].s64 = ctx.r[10].s64 + -11996;
	// 82EFD9C4: 61081E89  ori r8, r8, 0x1e89
	ctx.r[8].u64 = ctx.r[8].u64 | 7817;
	// 82EFD9C8: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EFD9CC: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EFD9D0: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EFD9D4: 409A0028  bne cr6, 0x82efd9fc
	if !ctx.cr[6].eq {
		sub_82EFD9FC(ctx, base);
		return;
	}
	// 82EFD9D8: 3D409FE1  lis r10, -0x601f
	ctx.r[10].s64 = -1612644352;
	// 82EFD9DC: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFD9E0: 614A234A  ori r10, r10, 0x234a
	ctx.r[10].u64 = ctx.r[10].u64 | 9034;
	// 82EFD9E4: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EFD9E8: 409A0014  bne cr6, 0x82efd9fc
	if !ctx.cr[6].eq {
		sub_82EFD9FC(ctx, base);
		return;
	}
	// 82EFD9EC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFD9F0: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82EFD9F4: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EFD9F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFD9FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFD9FC size=28
    let mut pc: u32 = 0x82EFD9FC;
    'dispatch: loop {
        match pc {
            0x82EFD9FC => {
    //   block [0x82EFD9FC..0x82EFDA18)
	// 82EFD9FC: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82EFDA00: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EFDA04: 409A000C  bne cr6, 0x82efda10
	if !ctx.cr[6].eq {
	pc = 0x82EFDA10; continue 'dispatch;
	}
	// 82EFDA08: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82EFDA0C: 38ABE2A4  addi r5, r11, -0x1d5c
	ctx.r[5].s64 = ctx.r[11].s64 + -7516;
	// 82EFDA10: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82EFDA14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFDA18 size=24
    let mut pc: u32 = 0x82EFDA18;
    'dispatch: loop {
        match pc {
            0x82EFDA18 => {
    //   block [0x82EFDA18..0x82EFDA30)
	// 82EFDA18: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFDA1C: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFDA20: 394AD124  addi r10, r10, -0x2edc
	ctx.r[10].s64 = ctx.r[10].s64 + -11996;
	// 82EFDA24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFDA28: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EFDA2C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFDA30 size=28
    let mut pc: u32 = 0x82EFDA30;
    'dispatch: loop {
        match pc {
            0x82EFDA30 => {
    //   block [0x82EFDA30..0x82EFDA4C)
	// 82EFDA30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EFDA34: 994B0004  stb r10, 4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 82EFDA38: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFDA3C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDA40: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFDA44: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFDA48: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDA4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFDA4C size=8
    let mut pc: u32 = 0x82EFDA4C;
    'dispatch: loop {
        match pc {
            0x82EFDA4C => {
    //   block [0x82EFDA4C..0x82EFDA54)
	// 82EFDA4C: 4BFFFE54  b 0x82efd8a0
	sub_82EFD8A0(ctx, base);
	return;
	// 82EFDA50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFDA58 size=32
    let mut pc: u32 = 0x82EFDA58;
    'dispatch: loop {
        match pc {
            0x82EFDA58 => {
    //   block [0x82EFDA58..0x82EFDA78)
	// 82EFDA58: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82EFDA5C: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82EFDA60: 396BE2A4  addi r11, r11, -0x1d5c
	ctx.r[11].s64 = ctx.r[11].s64 + -7516;
	// 82EFDA64: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFDA68: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EFDA6C: 409A000C  bne cr6, 0x82efda78
	if !ctx.cr[6].eq {
		sub_82EFDA78(ctx, base);
		return;
	}
	// 82EFDA70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFDA74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFDA78 size=20
    let mut pc: u32 = 0x82EFDA78;
    'dispatch: loop {
        match pc {
            0x82EFDA78 => {
    //   block [0x82EFDA78..0x82EFDA8C)
	// 82EFDA78: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 82EFDA7C: 41980010  blt cr6, 0x82efda8c
	if ctx.cr[6].lt {
		sub_82EFDA8C(ctx, base);
		return;
	}
	// 82EFDA80: 409AFFF0  bne cr6, 0x82efda70
	if !ctx.cr[6].eq {
		sub_82EFDA58(ctx, base);
		return;
	}
	// 82EFDA84: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82EFDA88: 48000008  b 0x82efda90
	sub_82EFDA8C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDA8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFDA8C size=16
    let mut pc: u32 = 0x82EFDA8C;
    'dispatch: loop {
        match pc {
            0x82EFDA8C => {
    //   block [0x82EFDA8C..0x82EFDA9C)
	// 82EFDA8C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82EFDA90: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFDA94: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EFDA98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFDAA0 size=132
    let mut pc: u32 = 0x82EFDAA0;
    'dispatch: loop {
        match pc {
            0x82EFDAA0 => {
    //   block [0x82EFDAA0..0x82EFDB24)
	// 82EFDAA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFDAA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFDAA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFDAAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFDAB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFDAB4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFDAB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFDABC: 409A0040  bne cr6, 0x82efdafc
	if !ctx.cr[6].eq {
	pc = 0x82EFDAFC; continue 'dispatch;
	}
	// 82EFDAC0: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82EFDAC4: 4BFFFD9D  bl 0x82efd860
	ctx.lr = 0x82EFDAC8;
	sub_82EFD860(ctx, base);
	// 82EFDAC8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFDACC: 41820018  beq 0x82efdae4
	if ctx.cr[0].eq {
	pc = 0x82EFDAE4; continue 'dispatch;
	}
	// 82EFDAD0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EFDAD4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EFDAD8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EFDADC: 99430004  stb r10, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 82EFDAE0: 48000008  b 0x82efdae8
	pc = 0x82EFDAE8; continue 'dispatch;
	// 82EFDAE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFDAE8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EFDAEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFDAF0: 409A000C  bne cr6, 0x82efdafc
	if !ctx.cr[6].eq {
	pc = 0x82EFDAFC; continue 'dispatch;
	}
	// 82EFDAF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFDAF8: 48000018  b 0x82efdb10
	pc = 0x82EFDB10; continue 'dispatch;
	// 82EFDAFC: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFDB00: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDB04: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EFDB08: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFDB0C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFDB10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFDB14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFDB18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFDB1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFDB20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFDB28 size=48
    let mut pc: u32 = 0x82EFDB28;
    'dispatch: loop {
        match pc {
            0x82EFDB28 => {
    //   block [0x82EFDB28..0x82EFDB58)
	// 82EFDB28: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EFDB2C: 38E30004  addi r7, r3, 4
	ctx.r[7].s64 = ctx.r[3].s64 + 4;
	// 82EFDB30: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EFDB34: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFDB38: 7D403828  lwarx r10, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EFDB3C: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EFDB40: 7D20392D  stwcx. r9, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFDB44: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFDB48: 4082FFE8  bne 0x82efdb30
	if !ctx.cr[0].eq {
	pc = 0x82EFDB30; continue 'dispatch;
	}
	// 82EFDB4C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EFDB50: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFDB54: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFDB58 size=8
    let mut pc: u32 = 0x82EFDB58;
    'dispatch: loop {
        match pc {
            0x82EFDB58 => {
    //   block [0x82EFDB58..0x82EFDB60)
	// 82EFDB58: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFDB5C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFDB60 size=20
    let mut pc: u32 = 0x82EFDB60;
    'dispatch: loop {
        match pc {
            0x82EFDB60 => {
    //   block [0x82EFDB60..0x82EFDB74)
	// 82EFDB60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDB64: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EFDB68: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDB6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFDB70: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDB74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFDB74 size=4
    let mut pc: u32 = 0x82EFDB74;
    'dispatch: loop {
        match pc {
            0x82EFDB74 => {
    //   block [0x82EFDB74..0x82EFDB78)
	// 82EFDB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFDB78 size=112
    let mut pc: u32 = 0x82EFDB78;
    'dispatch: loop {
        match pc {
            0x82EFDB78 => {
    //   block [0x82EFDB78..0x82EFDBE8)
	// 82EFDB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFDB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFDB80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFDB84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFDB88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFDB8C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDB90: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDB94: 55490001  rlwinm. r9, r10, 0, 0, 0
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EFDB98: 4182000C  beq 0x82efdba4
	if ctx.cr[0].eq {
	pc = 0x82EFDBA4; continue 'dispatch;
	}
	// 82EFDB9C: 5543007E  clrlwi r3, r10, 1
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0x7FFFFFFFu64;
	// 82EFDBA0: 48000034  b 0x82efdbd4
	pc = 0x82EFDBD4; continue 'dispatch;
	// 82EFDBA4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDBA8: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82EFDBAC: 5544007E  clrlwi r4, r10, 1
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x7FFFFFFFu64;
	// 82EFDBB0: 48007641  bl 0x82f051f0
	ctx.lr = 0x82EFDBB4;
	sub_82F051F0(ctx, base);
	// 82EFDBB4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDBB8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDBBC: 554A007E  clrlwi r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x7FFFFFFFu64;
	// 82EFDBC0: 7F035040  cmplw cr6, r3, r10
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82EFDBC4: 409A0010  bne cr6, 0x82efdbd4
	if !ctx.cr[6].eq {
	pc = 0x82EFDBD4; continue 'dispatch;
	}
	// 82EFDBC8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDBCC: 654A8000  oris r10, r10, 0x8000
	ctx.r[10].u64 = ctx.r[10].u64 | 2147483648;
	// 82EFDBD0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EFDBD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFDBD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFDBDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFDBE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFDBE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFDBE8 size=88
    let mut pc: u32 = 0x82EFDBE8;
    'dispatch: loop {
        match pc {
            0x82EFDBE8 => {
    //   block [0x82EFDBE8..0x82EFDC40)
	// 82EFDBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFDBEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFDBF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFDBF4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFDBF8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDBFC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EFDC00: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82EFDC04: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EFDC08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EFDC0C: 48006F0D  bl 0x82f04b18
	ctx.lr = 0x82EFDC10;
	sub_82F04B18(ctx, base);
	// 82EFDC10: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82EFDC14: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFDC18: 41820010  beq 0x82efdc28
	if ctx.cr[0].eq {
	pc = 0x82EFDC28; continue 'dispatch;
	}
	// 82EFDC1C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EFDC20: 4098FFE8  bge cr6, 0x82efdc08
	if !ctx.cr[6].lt {
	pc = 0x82EFDC08; continue 'dispatch;
	}
	// 82EFDC24: 48000008  b 0x82efdc2c
	pc = 0x82EFDC2C; continue 'dispatch;
	// 82EFDC28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFDC2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFDC30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFDC34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFDC38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFDC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFDC40 size=116
    let mut pc: u32 = 0x82EFDC40;
    'dispatch: loop {
        match pc {
            0x82EFDC40 => {
    //   block [0x82EFDC40..0x82EFDCB4)
	// 82EFDC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFDC44: 4BDAB7C5  bl 0x82ca9408
	ctx.lr = 0x82EFDC48;
	sub_82CA93D0(ctx, base);
	// 82EFDC48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFDC4C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFDC50: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EFDC54: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82EFDC58: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDC5C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFDC60: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EFDC64: 4098001C  bge cr6, 0x82efdc80
	if !ctx.cr[6].lt {
	pc = 0x82EFDC80; continue 'dispatch;
	}
	// 82EFDC68: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFDC6C: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82EFDC70: 4182003C  beq 0x82efdcac
	if ctx.cr[0].eq {
	pc = 0x82EFDCAC; continue 'dispatch;
	}
	// 82EFDC74: 67EB8000  oris r11, r31, 0x8000
	ctx.r[11].u64 = ctx.r[31].u64 | 2147483648;
	// 82EFDC78: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFDC7C: 48000030  b 0x82efdcac
	pc = 0x82EFDCAC; continue 'dispatch;
	// 82EFDC80: 57FD083C  slwi r29, r31, 1
	ctx.r[29].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82EFDC84: 389D0010  addi r4, r29, 0x10
	ctx.r[4].s64 = ctx.r[29].s64 + 16;
	// 82EFDC88: 4BFFFBF9  bl 0x82efd880
	ctx.lr = 0x82EFDC8C;
	sub_82EFD880(ctx, base);
	// 82EFDC8C: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EFDC90: 93A30004  stw r29, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82EFDC94: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFDC98: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDC9C: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82EFDCA0: 4182000C  beq 0x82efdcac
	if ctx.cr[0].eq {
	pc = 0x82EFDCAC; continue 'dispatch;
	}
	// 82EFDCA4: 67EA8000  oris r10, r31, 0x8000
	ctx.r[10].u64 = ctx.r[31].u64 | 2147483648;
	// 82EFDCA8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EFDCAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFDCB0: 4BDAB7A8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFDCB8 size=4
    let mut pc: u32 = 0x82EFDCB8;
    'dispatch: loop {
        match pc {
            0x82EFDCB8 => {
    //   block [0x82EFDCB8..0x82EFDCBC)
	// 82EFDCB8: 4800CFD8  b 0x82f0ac90
	sub_82F0AC90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFDCC0 size=232
    let mut pc: u32 = 0x82EFDCC0;
    'dispatch: loop {
        match pc {
            0x82EFDCC0 => {
    //   block [0x82EFDCC0..0x82EFDDA8)
	// 82EFDCC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFDCC4: 4BDAB73D  bl 0x82ca9400
	ctx.lr = 0x82EFDCC8;
	sub_82CA93D0(ctx, base);
	// 82EFDCC8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFDCCC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EFDCD0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EFDCD4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EFDCD8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EFDCDC: 419A00A0  beq cr6, 0x82efdd7c
	if ctx.cr[6].eq {
	pc = 0x82EFDD7C; continue 'dispatch;
	}
	// 82EFDCE0: 7FDAF378  mr r26, r30
	ctx.r[26].u64 = ctx.r[30].u64;
	// 82EFDCE4: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 82EFDCE8: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDCEC: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82EFDCF0: 4B2F0E31  bl 0x821eeb20
	ctx.lr = 0x82EFDCF4;
	sub_821EEB20(ctx, base);
	// 82EFDCF4: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDCF8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EFDCFC: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82EFDD00: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82EFDD04: 4B2F0E1D  bl 0x821eeb20
	ctx.lr = 0x82EFDD08;
	sub_821EEB20(ctx, base);
	// 82EFDD08: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EFDD0C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82EFDD10: 41820020  beq 0x82efdd30
	if ctx.cr[0].eq {
	pc = 0x82EFDD30; continue 'dispatch;
	}
	// 82EFDD14: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82EFDD18: 419A0018  beq cr6, 0x82efdd30
	if ctx.cr[6].eq {
	pc = 0x82EFDD30; continue 'dispatch;
	}
	// 82EFDD1C: 7F1C1800  cmpw cr6, r28, r3
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82EFDD20: 409A0054  bne cr6, 0x82efdd74
	if !ctx.cr[6].eq {
	pc = 0x82EFDD74; continue 'dispatch;
	}
	// 82EFDD24: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDD28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFDD2C: 409AFFBC  bne cr6, 0x82efdce8
	if !ctx.cr[6].eq {
	pc = 0x82EFDCE8; continue 'dispatch;
	}
	// 82EFDD30: 7F1C1800  cmpw cr6, r28, r3
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82EFDD34: 409A0040  bne cr6, 0x82efdd74
	if !ctx.cr[6].eq {
	pc = 0x82EFDD74; continue 'dispatch;
	}
	// 82EFDD38: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EFDD3C: 409A0010  bne cr6, 0x82efdd4c
	if !ctx.cr[6].eq {
	pc = 0x82EFDD4C; continue 'dispatch;
	}
	// 82EFDD40: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDD44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFDD48: 419A002C  beq cr6, 0x82efdd74
	if ctx.cr[6].eq {
	pc = 0x82EFDD74; continue 'dispatch;
	}
	// 82EFDD4C: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82EFDD50: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDD54: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EFDD58: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EFDD5C: 409AFFF4  bne cr6, 0x82efdd50
	if !ctx.cr[6].eq {
	pc = 0x82EFDD50; continue 'dispatch;
	}
	// 82EFDD60: 7D7B5850  subf r11, r27, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 82EFDD64: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EFDD68: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFDD6C: 7C6BD050  subf r3, r11, r26
	ctx.r[3].s64 = ctx.r[26].s64 - ctx.r[11].s64;
	// 82EFDD70: 48000030  b 0x82efdda0
	pc = 0x82EFDDA0; continue 'dispatch;
	// 82EFDD74: 7C63E050  subf r3, r3, r28
	ctx.r[3].s64 = ctx.r[28].s64 - ctx.r[3].s64;
	// 82EFDD78: 48000028  b 0x82efdda0
	pc = 0x82EFDDA0; continue 'dispatch;
	// 82EFDD7C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82EFDD80: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDD84: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EFDD88: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EFDD8C: 409AFFF4  bne cr6, 0x82efdd80
	if !ctx.cr[6].eq {
	pc = 0x82EFDD80; continue 'dispatch;
	}
	// 82EFDD90: 7D7F5850  subf r11, r31, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 82EFDD94: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EFDD98: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFDD9C: 7C6B00D0  neg r3, r11
	ctx.r[3].s64 = -ctx.r[11].s64;
	// 82EFDDA0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EFDDA4: 4BDAB6AC  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFDDA8 size=36
    let mut pc: u32 = 0x82EFDDA8;
    'dispatch: loop {
        match pc {
            0x82EFDDA8 => {
    //   block [0x82EFDDA8..0x82EFDDCC)
	// 82EFDDA8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EFDDAC: 419A0018  beq cr6, 0x82efddc4
	if ctx.cr[6].eq {
	pc = 0x82EFDDC4; continue 'dispatch;
	}
	// 82EFDDB0: 3484FFFF  addic. r4, r4, -1
	ctx.xer.ca = (ctx.r[4].u32 > (!(-1 as u32)));
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82EFDDB4: 1D650021  mulli r11, r5, 0x21
	ctx.r[11].s64 = ctx.r[5].s64 * 33;
	// 82EFDDB8: 7D4320AE  lbzx r10, r3, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82EFDDBC: 7D455A78  xor r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	// 82EFDDC0: 4082FFF0  bne 0x82efddb0
	if !ctx.cr[0].eq {
	pc = 0x82EFDDB0; continue 'dispatch;
	}
	// 82EFDDC4: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82EFDDC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFDDD0 size=68
    let mut pc: u32 = 0x82EFDDD0;
    'dispatch: loop {
        match pc {
            0x82EFDDD0 => {
    //   block [0x82EFDDD0..0x82EFDE14)
	// 82EFDDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFDDD4: 4BDAB639  bl 0x82ca940c
	ctx.lr = 0x82EFDDD8;
	sub_82CA93D0(ctx, base);
	// 82EFDDD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFDDDC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EFDDE0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EFDDE4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EFDDE8: 48000018  b 0x82efde00
	pc = 0x82EFDE00; continue 'dispatch;
	// 82EFDDEC: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82EFDDF0: 7C7DF8AE  lbzx r3, r29, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EFDDF4: 4B2F0D2D  bl 0x821eeb20
	ctx.lr = 0x82EFDDF8;
	sub_821EEB20(ctx, base);
	// 82EFDDF8: 1D7E0021  mulli r11, r30, 0x21
	ctx.r[11].s64 = ctx.r[30].s64 * 33;
	// 82EFDDFC: 7C7E5A78  xor r30, r3, r11
	ctx.r[30].u64 = ctx.r[3].u64 ^ ctx.r[11].u64;
	// 82EFDE00: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFDE04: 409AFFE8  bne cr6, 0x82efddec
	if !ctx.cr[6].eq {
	pc = 0x82EFDDEC; continue 'dispatch;
	}
	// 82EFDE08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFDE0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFDE10: 4BDAB64C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFDE18 size=20
    let mut pc: u32 = 0x82EFDE18;
    'dispatch: loop {
        match pc {
            0x82EFDE18 => {
    //   block [0x82EFDE18..0x82EFDE2C)
	// 82EFDE18: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EFDE1C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDE20: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFDE24: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EFDE28: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDE2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFDE2C size=8
    let mut pc: u32 = 0x82EFDE2C;
    'dispatch: loop {
        match pc {
            0x82EFDE2C => {
    //   block [0x82EFDE2C..0x82EFDE34)
	// 82EFDE2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFDE30: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDE34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFDE34 size=8
    let mut pc: u32 = 0x82EFDE34;
    'dispatch: loop {
        match pc {
            0x82EFDE34 => {
    //   block [0x82EFDE34..0x82EFDE3C)
	// 82EFDE34: 4BFFFA6C  b 0x82efd8a0
	sub_82EFD8A0(ctx, base);
	return;
	// 82EFDE38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFDE40 size=188
    let mut pc: u32 = 0x82EFDE40;
    'dispatch: loop {
        match pc {
            0x82EFDE40 => {
    //   block [0x82EFDE40..0x82EFDEFC)
	// 82EFDE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFDE44: 4BDAB5C5  bl 0x82ca9408
	ctx.lr = 0x82EFDE48;
	sub_82CA93D0(ctx, base);
	// 82EFDE48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFDE4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFDE50: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EFDE54: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFDE58: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EFDE5C: 40990078  ble cr6, 0x82efded4
	if !ctx.cr[6].gt {
	pc = 0x82EFDED4; continue 'dispatch;
	}
	// 82EFDE60: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFDE64: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EFDE68: 4198006C  blt cr6, 0x82efded4
	if ctx.cr[6].lt {
	pc = 0x82EFDED4; continue 'dispatch;
	}
	// 82EFDE6C: 579D083C  slwi r29, r28, 1
	ctx.r[29].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82EFDE70: 387D0002  addi r3, r29, 2
	ctx.r[3].s64 = ctx.r[29].s64 + 2;
	// 82EFDE74: 4BFFF9ED  bl 0x82efd860
	ctx.lr = 0x82EFDE78;
	sub_82EFD860(ctx, base);
	// 82EFDE78: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EFDE7C: 41820050  beq 0x82efdecc
	if ctx.cr[0].eq {
	pc = 0x82EFDECC; continue 'dispatch;
	}
	// 82EFDE80: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDE84: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EFDE88: 419A0018  beq cr6, 0x82efdea0
	if ctx.cr[6].eq {
	pc = 0x82EFDEA0; continue 'dispatch;
	}
	// 82EFDE8C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFDE90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFDE94: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EFDE98: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EFDE9C: 4BDAB5E5  bl 0x82ca9480
	ctx.lr = 0x82EFDEA0;
	sub_82CA9480(ctx, base);
	// 82EFDEA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFDEA4: 7D7DF32E  sthx r11, r29, r30
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32), ctx.r[11].u16) };
	// 82EFDEA8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDEAC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFDEB0: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EFDEB4: 419A0010  beq cr6, 0x82efdec4
	if ctx.cr[6].eq {
	pc = 0x82EFDEC4; continue 'dispatch;
	}
	// 82EFDEB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFDEBC: 419A0008  beq cr6, 0x82efdec4
	if ctx.cr[6].eq {
	pc = 0x82EFDEC4; continue 'dispatch;
	}
	// 82EFDEC0: 4BFFF9E1  bl 0x82efd8a0
	ctx.lr = 0x82EFDEC4;
	sub_82EFD8A0(ctx, base);
	// 82EFDEC4: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82EFDEC8: 48000024  b 0x82efdeec
	pc = 0x82EFDEEC; continue 'dispatch;
	// 82EFDECC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EFDED0: 48000024  b 0x82efdef4
	pc = 0x82EFDEF4; continue 'dispatch;
	// 82EFDED4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDED8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFDEDC: 419A0010  beq cr6, 0x82efdeec
	if ctx.cr[6].eq {
	pc = 0x82EFDEEC; continue 'dispatch;
	}
	// 82EFDEE0: 578A083C  slwi r10, r28, 1
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EFDEE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82EFDEE8: 7D2A5B2E  sthx r9, r10, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u16) };
	// 82EFDEEC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EFDEF0: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82EFDEF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFDEF8: 4BDAB560  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFDF00 size=100
    let mut pc: u32 = 0x82EFDF00;
    'dispatch: loop {
        match pc {
            0x82EFDF00 => {
    //   block [0x82EFDF00..0x82EFDF64)
	// 82EFDF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFDF04: 4BDAB509  bl 0x82ca940c
	ctx.lr = 0x82EFDF08;
	sub_82CA93D0(ctx, base);
	// 82EFDF08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFDF0C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82EFDF10: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFDF14: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EFDF18: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82EFDF1C: 409A0010  bne cr6, 0x82efdf2c
	if !ctx.cr[6].eq {
	pc = 0x82EFDF2C; continue 'dispatch;
	}
	// 82EFDF20: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFDF24: 4800CD75  bl 0x82f0ac98
	ctx.lr = 0x82EFDF28;
	sub_82F0AC98(ctx, base);
	// 82EFDF28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFDF2C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFDF30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFDF34: 4BFFFF0D  bl 0x82efde40
	ctx.lr = 0x82EFDF38;
	sub_82EFDE40(ctx, base);
	// 82EFDF38: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFDF3C: 41820020  beq 0x82efdf5c
	if ctx.cr[0].eq {
	pc = 0x82EFDF5C; continue 'dispatch;
	}
	// 82EFDF40: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFDF44: 419A0018  beq cr6, 0x82efdf5c
	if ctx.cr[6].eq {
	pc = 0x82EFDF5C; continue 'dispatch;
	}
	// 82EFDF48: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 82EFDF4C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDF50: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EFDF54: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EFDF58: 4BDAB529  bl 0x82ca9480
	ctx.lr = 0x82EFDF5C;
	sub_82CA9480(ctx, base);
	// 82EFDF5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFDF60: 4BDAB4FC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFDF68 size=52
    let mut pc: u32 = 0x82EFDF68;
    'dispatch: loop {
        match pc {
            0x82EFDF68 => {
    //   block [0x82EFDF68..0x82EFDF9C)
	// 82EFDF68: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82EFDF6C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82EFDF70: 396BE2BC  addi r11, r11, -0x1d44
	ctx.r[11].s64 = ctx.r[11].s64 + -7492;
	// 82EFDF74: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFDF78: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82EFDF7C: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EFDF80: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFDF84: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EFDF88: 7D275214  add r9, r7, r10
	ctx.r[9].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 82EFDF8C: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFDF90: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFDF94: 4082FFE8  bne 0x82efdf7c
	if !ctx.cr[0].eq {
	pc = 0x82EFDF7C; continue 'dispatch;
	}
	// 82EFDF98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFDFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFDFA0 size=196
    let mut pc: u32 = 0x82EFDFA0;
    'dispatch: loop {
        match pc {
            0x82EFDFA0 => {
    //   block [0x82EFDFA0..0x82EFE064)
	// 82EFDFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFDFA4: 4BDAB469  bl 0x82ca940c
	ctx.lr = 0x82EFDFA8;
	sub_82CA93D0(ctx, base);
	// 82EFDFA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFDFAC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EFDFB0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFDFB4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EFDFB8: 409A0038  bne cr6, 0x82efdff0
	if !ctx.cr[6].eq {
	pc = 0x82EFDFF0; continue 'dispatch;
	}
	// 82EFDFBC: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82EFDFC0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82EFDFC4: 396BE2BC  addi r11, r11, -0x1d44
	ctx.r[11].s64 = ctx.r[11].s64 + -7492;
	// 82EFDFC8: 38CB0008  addi r6, r11, 8
	ctx.r[6].s64 = ctx.r[11].s64 + 8;
	// 82EFDFCC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFDFD0: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EFDFD4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFDFD8: 7D403028  lwarx r10, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EFDFDC: 7D275214  add r9, r7, r10
	ctx.r[9].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 82EFDFE0: 7D20312D  stwcx. r9, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFDFE4: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFDFE8: 4082FFE8  bne 0x82efdfd0
	if !ctx.cr[0].eq {
	pc = 0x82EFDFD0; continue 'dispatch;
	}
	// 82EFDFEC: 4800006C  b 0x82efe058
	pc = 0x82EFE058; continue 'dispatch;
	// 82EFDFF0: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82EFDFF4: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFDFF8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EFDFFC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EFE000: 409AFFF4  bne cr6, 0x82efdff4
	if !ctx.cr[6].eq {
	pc = 0x82EFDFF4; continue 'dispatch;
	}
	// 82EFE004: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82EFE008: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EFE00C: 557F003E  slwi r31, r11, 0
	ctx.r[31].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82EFE010: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82EFE014: 4BFFF84D  bl 0x82efd860
	ctx.lr = 0x82EFE018;
	sub_82EFD860(ctx, base);
	// 82EFE018: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EFE01C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EFE020: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EFE024: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 82EFE028: 4BDAB459  bl 0x82ca9480
	ctx.lr = 0x82EFE02C;
	sub_82CA9480(ctx, base);
	// 82EFE02C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EFE034: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EFE038: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EFE03C: 994B000C  stb r10, 0xc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 82EFE040: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE044: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82EFE048: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE04C: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82EFE050: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE054: 93EB0004  stw r31, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82EFE058: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFE05C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFE060: 4BDAB3FC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFE068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFE068 size=168
    let mut pc: u32 = 0x82EFE068;
    'dispatch: loop {
        match pc {
            0x82EFE068 => {
    //   block [0x82EFE068..0x82EFE110)
	// 82EFE068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFE06C: 4BDAB3A1  bl 0x82ca940c
	ctx.lr = 0x82EFE070;
	sub_82CA93D0(ctx, base);
	// 82EFE070: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFE074: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EFE078: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFE07C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EFE080: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EFE084: 409A0038  bne cr6, 0x82efe0bc
	if !ctx.cr[6].eq {
	pc = 0x82EFE0BC; continue 'dispatch;
	}
	// 82EFE088: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82EFE08C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82EFE090: 396BE2BC  addi r11, r11, -0x1d44
	ctx.r[11].s64 = ctx.r[11].s64 + -7492;
	// 82EFE094: 38CB0008  addi r6, r11, 8
	ctx.r[6].s64 = ctx.r[11].s64 + 8;
	// 82EFE098: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFE09C: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EFE0A0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFE0A4: 7D403028  lwarx r10, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EFE0A8: 7D275214  add r9, r7, r10
	ctx.r[9].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 82EFE0AC: 7D20312D  stwcx. r9, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFE0B0: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFE0B4: 4082FFE8  bne 0x82efe09c
	if !ctx.cr[0].eq {
	pc = 0x82EFE09C; continue 'dispatch;
	}
	// 82EFE0B8: 4800004C  b 0x82efe104
	pc = 0x82EFE104; continue 'dispatch;
	// 82EFE0BC: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 82EFE0C0: 4BFFF7A1  bl 0x82efd860
	ctx.lr = 0x82EFE0C4;
	sub_82EFD860(ctx, base);
	// 82EFE0C4: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EFE0C8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EFE0CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EFE0D0: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 82EFE0D4: 4BDAB3AD  bl 0x82ca9480
	ctx.lr = 0x82EFE0D8;
	sub_82CA9480(ctx, base);
	// 82EFE0D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE0DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EFE0E0: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EFE0E4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EFE0E8: 994B000C  stb r10, 0xc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 82EFE0EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE0F0: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82EFE0F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE0F8: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82EFE0FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE100: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82EFE104: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFE108: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFE10C: 4BDAB350  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFE110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFE110 size=244
    let mut pc: u32 = 0x82EFE110;
    'dispatch: loop {
        match pc {
            0x82EFE110 => {
    //   block [0x82EFE110..0x82EFE204)
	// 82EFE110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFE114: 4BDAB2F5  bl 0x82ca9408
	ctx.lr = 0x82EFE118;
	sub_82CA93D0(ctx, base);
	// 82EFE118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFE11C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EFE120: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EFE124: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE128: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFE12C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EFE130: 409A0050  bne cr6, 0x82efe180
	if !ctx.cr[6].eq {
	pc = 0x82EFE180; continue 'dispatch;
	}
	// 82EFE134: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE138: 556B007E  clrlwi r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82EFE13C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EFE140: 40980014  bge cr6, 0x82efe154
	if !ctx.cr[6].lt {
	pc = 0x82EFE154; continue 'dispatch;
	}
	// 82EFE144: 7D63FA14  add r11, r3, r31
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 82EFE148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EFE14C: 994B000C  stb r10, 0xc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 82EFE150: 48000024  b 0x82efe174
	pc = 0x82EFE174; continue 'dispatch;
	// 82EFE154: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFE158: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EFE15C: 409900A0  ble cr6, 0x82efe1fc
	if !ctx.cr[6].gt {
	pc = 0x82EFE1FC; continue 'dispatch;
	}
	// 82EFE160: 57FE083C  slwi r30, r31, 1
	ctx.r[30].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82EFE164: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 82EFE168: 4BFFF719  bl 0x82efd880
	ctx.lr = 0x82EFE16C;
	sub_82EFD880(ctx, base);
	// 82EFE16C: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EFE170: 93C30004  stw r30, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82EFE174: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE178: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82EFE17C: 48000080  b 0x82efe1fc
	pc = 0x82EFE1FC; continue 'dispatch;
	// 82EFE180: 57FC083C  slwi r28, r31, 1
	ctx.r[28].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82EFE184: 387C0010  addi r3, r28, 0x10
	ctx.r[3].s64 = ctx.r[28].s64 + 16;
	// 82EFE188: 4BFFF6D9  bl 0x82efd860
	ctx.lr = 0x82EFE18C;
	sub_82EFD860(ctx, base);
	// 82EFE18C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFE190: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFE194: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EFE198: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82EFE19C: 3B9E000C  addi r28, r30, 0xc
	ctx.r[28].s64 = ctx.r[30].s64 + 12;
	// 82EFE1A0: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82EFE1A4: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFE1A8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EFE1AC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE1B0: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82EFE1B4: 4BDAB2CD  bl 0x82ca9480
	ctx.lr = 0x82EFE1B8;
	sub_82CA9480(ctx, base);
	// 82EFE1B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFE1BC: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82EFE1C0: 7D7CF9AE  stbx r11, r28, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[31].u32), ctx.r[11].u8) };
	// 82EFE1C4: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE1C8: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 82EFE1CC: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82EFE1D0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFE1D4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EFE1D8: 7D285214  add r9, r8, r10
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82EFE1DC: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFE1E0: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFE1E4: 4082FFE8  bne 0x82efe1cc
	if !ctx.cr[0].eq {
	pc = 0x82EFE1CC; continue 'dispatch;
	}
	// 82EFE1E8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EFE1EC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFE1F0: 40820008  bne 0x82efe1f8
	if !ctx.cr[0].eq {
	pc = 0x82EFE1F8; continue 'dispatch;
	}
	// 82EFE1F4: 4BFFF6AD  bl 0x82efd8a0
	ctx.lr = 0x82EFE1F8;
	sub_82EFD8A0(ctx, base);
	// 82EFE1F8: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82EFE1FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFE200: 4BDAB258  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFE208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFE208 size=312
    let mut pc: u32 = 0x82EFE208;
    'dispatch: loop {
        match pc {
            0x82EFE208 => {
    //   block [0x82EFE208..0x82EFE340)
	// 82EFE208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFE20C: 4BDAB1F5  bl 0x82ca9400
	ctx.lr = 0x82EFE210;
	sub_82CA93D0(ctx, base);
	// 82EFE210: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFE214: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFE218: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82EFE21C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82EFE220: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EFE224: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82EFE228: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE22C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE230: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82EFE234: 557F007E  clrlwi r31, r11, 1
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82EFE238: 48006C01  bl 0x82f04e38
	ctx.lr = 0x82EFE23C;
	sub_82F04E38(ctx, base);
	// 82EFE23C: 39610058  addi r11, r1, 0x58
	ctx.r[11].s64 = ctx.r[1].s64 + 88;
	// 82EFE240: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE244: 83A10050  lwz r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFE248: 7F7DFA14  add r27, r29, r31
	ctx.r[27].u64 = ctx.r[29].u64 + ctx.r[31].u64;
	// 82EFE24C: 7F9D59AE  stbx r28, r29, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32), ctx.r[28].u8) };
	// 82EFE250: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFE254: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EFE258: 409A0038  bne cr6, 0x82efe290
	if !ctx.cr[6].eq {
	pc = 0x82EFE290; continue 'dispatch;
	}
	// 82EFE25C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFE260: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EFE264: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFE268: 4BFFF9D9  bl 0x82efdc40
	ctx.lr = 0x82EFE26C;
	sub_82EFDC40(ctx, base);
	// 82EFE26C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE270: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EFE274: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EFE278: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82EFE27C: 3BEB000C  addi r31, r11, 0xc
	ctx.r[31].s64 = ctx.r[11].s64 + 12;
	// 82EFE280: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFE284: 4BDAB1FD  bl 0x82ca9480
	ctx.lr = 0x82EFE288;
	sub_82CA9480(ctx, base);
	// 82EFE288: 7F9FE9AE  stbx r28, r31, r29
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32), ctx.r[28].u8) };
	// 82EFE28C: 480000AC  b 0x82efe338
	pc = 0x82EFE338; continue 'dispatch;
	// 82EFE290: 577A083C  slwi r26, r27, 1
	ctx.r[26].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 82EFE294: 387A0010  addi r3, r26, 0x10
	ctx.r[3].s64 = ctx.r[26].s64 + 16;
	// 82EFE298: 4BFFF5C9  bl 0x82efd860
	ctx.lr = 0x82EFE29C;
	sub_82EFD860(ctx, base);
	// 82EFE29C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFE2A0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFE2A4: 935F0004  stw r26, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82EFE2A8: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82EFE2AC: 3B7F000C  addi r27, r31, 0xc
	ctx.r[27].s64 = ctx.r[31].s64 + 12;
	// 82EFE2B0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFE2B4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE2B8: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82EFE2BC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE2C0: 557A007E  clrlwi r26, r11, 1
	ctx.r[26].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82EFE2C4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EFE2C8: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82EFE2CC: 4BDAB1B5  bl 0x82ca9480
	ctx.lr = 0x82EFE2D0;
	sub_82CA9480(ctx, base);
	// 82EFE2D0: 7F9BD1AE  stbx r28, r27, r26
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32), ctx.r[28].u8) };
	// 82EFE2D4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE2D8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EFE2DC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82EFE2E0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE2E4: 556B007E  clrlwi r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82EFE2E8: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EFE2EC: 3B6B000C  addi r27, r11, 0xc
	ctx.r[27].s64 = ctx.r[11].s64 + 12;
	// 82EFE2F0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EFE2F4: 4BDAB18D  bl 0x82ca9480
	ctx.lr = 0x82EFE2F8;
	sub_82CA9480(ctx, base);
	// 82EFE2F8: 7F9BE9AE  stbx r28, r27, r29
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[29].u32), ctx.r[28].u8) };
	// 82EFE2FC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE300: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82EFE304: 38E30008  addi r7, r3, 8
	ctx.r[7].s64 = ctx.r[3].s64 + 8;
	// 82EFE308: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EFE30C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFE310: 7D603828  lwarx r11, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 82EFE314: 7D495A14  add r10, r9, r11
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82EFE318: 7D40392D  stwcx. r10, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFE31C: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFE320: 4082FFE8  bne 0x82efe308
	if !ctx.cr[0].eq {
	pc = 0x82EFE308; continue 'dispatch;
	}
	// 82EFE324: 7D6B5B78  mr r11, r11
	ctx.r[11].u64 = ctx.r[11].u64;
	// 82EFE328: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFE32C: 40820008  bne 0x82efe334
	if !ctx.cr[0].eq {
	pc = 0x82EFE334; continue 'dispatch;
	}
	// 82EFE330: 4BFFF571  bl 0x82efd8a0
	ctx.lr = 0x82EFE334;
	sub_82EFD8A0(ctx, base);
	// 82EFE334: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82EFE338: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EFE33C: 4BDAB114  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFE340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFE340 size=268
    let mut pc: u32 = 0x82EFE340;
    'dispatch: loop {
        match pc {
            0x82EFE340 => {
    //   block [0x82EFE340..0x82EFE44C)
	// 82EFE340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFE344: 4BDAB0B9  bl 0x82ca93fc
	ctx.lr = 0x82EFE348;
	sub_82CA93D0(ctx, base);
	// 82EFE348: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFE34C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82EFE350: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EFE354: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82EFE358: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82EFE35C: 419A00E8  beq cr6, 0x82efe444
	if ctx.cr[6].eq {
	pc = 0x82EFE444; continue 'dispatch;
	}
	// 82EFE360: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82EFE364: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EFE368: 48006CD1  bl 0x82f05038
	ctx.lr = 0x82EFE36C;
	sub_82F05038(ctx, base);
	// 82EFE36C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE370: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE374: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFE378: 555E007E  clrlwi r30, r10, 1
	ctx.r[30].u64 = ctx.r[10].u32 as u64 & 0x7FFFFFFFu64;
	// 82EFE37C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EFE380: 7F9E1A14  add r28, r30, r3
	ctx.r[28].u64 = ctx.r[30].u64 + ctx.r[3].u64;
	// 82EFE384: 409A0030  bne cr6, 0x82efe3b4
	if !ctx.cr[6].eq {
	pc = 0x82EFE3B4; continue 'dispatch;
	}
	// 82EFE388: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFE38C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EFE390: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFE394: 4BFFF8AD  bl 0x82efdc40
	ctx.lr = 0x82EFE398;
	sub_82EFDC40(ctx, base);
	// 82EFE398: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE39C: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82EFE3A0: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EFE3A4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82EFE3A8: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82EFE3AC: 48006D15  bl 0x82f050c0
	ctx.lr = 0x82EFE3B0;
	sub_82F050C0(ctx, base);
	// 82EFE3B0: 48000094  b 0x82efe444
	pc = 0x82EFE444; continue 'dispatch;
	// 82EFE3B4: 579B083C  slwi r27, r28, 1
	ctx.r[27].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82EFE3B8: 387B0010  addi r3, r27, 0x10
	ctx.r[3].s64 = ctx.r[27].s64 + 16;
	// 82EFE3BC: 4BFFF4A5  bl 0x82efd860
	ctx.lr = 0x82EFE3C0;
	sub_82EFD860(ctx, base);
	// 82EFE3C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFE3C4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFE3C8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EFE3CC: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82EFE3D0: 3B9F000C  addi r28, r31, 0xc
	ctx.r[28].s64 = ctx.r[31].s64 + 12;
	// 82EFE3D4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFE3D8: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82EFE3DC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EFE3E0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE3E4: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82EFE3E8: 4BDAB099  bl 0x82ca9480
	ctx.lr = 0x82EFE3EC;
	sub_82CA9480(ctx, base);
	// 82EFE3EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EFE3F0: 7D7FF214  add r11, r31, r30
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 82EFE3F4: 7D5CF1AE  stbx r10, r28, r30
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u8) };
	// 82EFE3F8: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82EFE3FC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82EFE400: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82EFE404: 48006CBD  bl 0x82f050c0
	ctx.lr = 0x82EFE408;
	sub_82F050C0(ctx, base);
	// 82EFE408: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE40C: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82EFE410: 38E30008  addi r7, r3, 8
	ctx.r[7].s64 = ctx.r[3].s64 + 8;
	// 82EFE414: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EFE418: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFE41C: 7D603828  lwarx r11, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 82EFE420: 7D495A14  add r10, r9, r11
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82EFE424: 7D40392D  stwcx. r10, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFE428: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFE42C: 4082FFE8  bne 0x82efe414
	if !ctx.cr[0].eq {
	pc = 0x82EFE414; continue 'dispatch;
	}
	// 82EFE430: 7D6B5B78  mr r11, r11
	ctx.r[11].u64 = ctx.r[11].u64;
	// 82EFE434: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFE438: 40820008  bne 0x82efe440
	if !ctx.cr[0].eq {
	pc = 0x82EFE440; continue 'dispatch;
	}
	// 82EFE43C: 4BFFF465  bl 0x82efd8a0
	ctx.lr = 0x82EFE440;
	sub_82EFD8A0(ctx, base);
	// 82EFE440: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82EFE444: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EFE448: 4BDAB004  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFE450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFE450 size=324
    let mut pc: u32 = 0x82EFE450;
    'dispatch: loop {
        match pc {
            0x82EFE450 => {
    //   block [0x82EFE450..0x82EFE594)
	// 82EFE450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFE454: 4BDAAFA9  bl 0x82ca93fc
	ctx.lr = 0x82EFE458;
	sub_82CA93D0(ctx, base);
	// 82EFE458: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFE45C: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82EFE460: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EFE464: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EFE468: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82EFE46C: 419A0120  beq cr6, 0x82efe58c
	if ctx.cr[6].eq {
	pc = 0x82EFE58C; continue 'dispatch;
	}
	// 82EFE470: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82EFE474: 419A0118  beq cr6, 0x82efe58c
	if ctx.cr[6].eq {
	pc = 0x82EFE58C; continue 'dispatch;
	}
	// 82EFE478: 2F1DFFFF  cmpwi cr6, r29, -1
	ctx.cr[6].compare_i32(ctx.r[29].s32, -1, &mut ctx.xer);
	// 82EFE47C: 409A0024  bne cr6, 0x82efe4a0
	if !ctx.cr[6].eq {
	pc = 0x82EFE4A0; continue 'dispatch;
	}
	// 82EFE480: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82EFE484: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE488: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EFE48C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EFE490: 409AFFF4  bne cr6, 0x82efe484
	if !ctx.cr[6].eq {
	pc = 0x82EFE484; continue 'dispatch;
	}
	// 82EFE494: 7D795850  subf r11, r25, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[25].s64;
	// 82EFE498: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EFE49C: 557D003E  slwi r29, r11, 0
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82EFE4A0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE4A4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE4A8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFE4AC: 555E007E  clrlwi r30, r10, 1
	ctx.r[30].u64 = ctx.r[10].u32 as u64 & 0x7FFFFFFFu64;
	// 82EFE4B0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EFE4B4: 7F5EEA14  add r26, r30, r29
	ctx.r[26].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 82EFE4B8: 409A003C  bne cr6, 0x82efe4f4
	if !ctx.cr[6].eq {
	pc = 0x82EFE4F4; continue 'dispatch;
	}
	// 82EFE4BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFE4C0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82EFE4C4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EFE4C8: 4BFFF779  bl 0x82efdc40
	ctx.lr = 0x82EFE4CC;
	sub_82EFDC40(ctx, base);
	// 82EFE4CC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE4D0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EFE4D4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EFE4D8: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82EFE4DC: 3BEB000C  addi r31, r11, 0xc
	ctx.r[31].s64 = ctx.r[11].s64 + 12;
	// 82EFE4E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFE4E4: 4BDAAF9D  bl 0x82ca9480
	ctx.lr = 0x82EFE4E8;
	sub_82CA9480(ctx, base);
	// 82EFE4E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFE4EC: 7D7FE9AE  stbx r11, r31, r29
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32), ctx.r[11].u8) };
	// 82EFE4F0: 4800009C  b 0x82efe58c
	pc = 0x82EFE58C; continue 'dispatch;
	// 82EFE4F4: 575B083C  slwi r27, r26, 1
	ctx.r[27].u32 = ctx.r[26].u32.wrapping_shl(1);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82EFE4F8: 387B0010  addi r3, r27, 0x10
	ctx.r[3].s64 = ctx.r[27].s64 + 16;
	// 82EFE4FC: 4BFFF365  bl 0x82efd860
	ctx.lr = 0x82EFE500;
	sub_82EFD860(ctx, base);
	// 82EFE500: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFE504: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFE508: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EFE50C: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82EFE510: 3B7F000C  addi r27, r31, 0xc
	ctx.r[27].s64 = ctx.r[31].s64 + 12;
	// 82EFE514: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFE518: 935F0000  stw r26, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82EFE51C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EFE520: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE524: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82EFE528: 4BDAAF59  bl 0x82ca9480
	ctx.lr = 0x82EFE52C;
	sub_82CA9480(ctx, base);
	// 82EFE52C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82EFE530: 7D7FF214  add r11, r31, r30
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 82EFE534: 7F5BF1AE  stbx r26, r27, r30
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[30].u32), ctx.r[26].u8) };
	// 82EFE538: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EFE53C: 3BCB000C  addi r30, r11, 0xc
	ctx.r[30].s64 = ctx.r[11].s64 + 12;
	// 82EFE540: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82EFE544: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFE548: 4BDAAF39  bl 0x82ca9480
	ctx.lr = 0x82EFE54C;
	sub_82CA9480(ctx, base);
	// 82EFE54C: 7F5EE9AE  stbx r26, r30, r29
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32), ctx.r[26].u8) };
	// 82EFE550: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE554: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82EFE558: 38E30008  addi r7, r3, 8
	ctx.r[7].s64 = ctx.r[3].s64 + 8;
	// 82EFE55C: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EFE560: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFE564: 7D603828  lwarx r11, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 82EFE568: 7D495A14  add r10, r9, r11
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82EFE56C: 7D40392D  stwcx. r10, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFE570: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFE574: 4082FFE8  bne 0x82efe55c
	if !ctx.cr[0].eq {
	pc = 0x82EFE55C; continue 'dispatch;
	}
	// 82EFE578: 7D6B5B78  mr r11, r11
	ctx.r[11].u64 = ctx.r[11].u64;
	// 82EFE57C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFE580: 40820008  bne 0x82efe588
	if !ctx.cr[0].eq {
	pc = 0x82EFE588; continue 'dispatch;
	}
	// 82EFE584: 4BFFF31D  bl 0x82efd8a0
	ctx.lr = 0x82EFE588;
	sub_82EFD8A0(ctx, base);
	// 82EFE588: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82EFE58C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EFE590: 4BDAAEBC  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFE598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFE598 size=284
    let mut pc: u32 = 0x82EFE598;
    'dispatch: loop {
        match pc {
            0x82EFE598 => {
    //   block [0x82EFE598..0x82EFE6B4)
	// 82EFE598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFE59C: 4BDAAE69  bl 0x82ca9404
	ctx.lr = 0x82EFE5A0;
	sub_82CA93D0(ctx, base);
	// 82EFE5A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFE5A4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EFE5A8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82EFE5AC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82EFE5B0: 409A000C  bne cr6, 0x82efe5bc
	if !ctx.cr[6].eq {
	pc = 0x82EFE5BC; continue 'dispatch;
	}
	// 82EFE5B4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EFE5B8: 3BAB0CA0  addi r29, r11, 0xca0
	ctx.r[29].s64 = ctx.r[11].s64 + 3232;
	// 82EFE5BC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82EFE5C0: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE5C4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EFE5C8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EFE5CC: 409AFFF4  bne cr6, 0x82efe5c0
	if !ctx.cr[6].eq {
	pc = 0x82EFE5C0; continue 'dispatch;
	}
	// 82EFE5D0: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82EFE5D4: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE5D8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EFE5DC: 557E003E  slwi r30, r11, 0
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82EFE5E0: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFE5E4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EFE5E8: 409A004C  bne cr6, 0x82efe634
	if !ctx.cr[6].eq {
	pc = 0x82EFE634; continue 'dispatch;
	}
	// 82EFE5EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFE5F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFE5F4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EFE5F8: 4BFFF649  bl 0x82efdc40
	ctx.lr = 0x82EFE5FC;
	sub_82EFDC40(ctx, base);
	// 82EFE5FC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82EFE600: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE604: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EFE608: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EFE60C: 409AFFF4  bne cr6, 0x82efe600
	if !ctx.cr[6].eq {
	pc = 0x82EFE600; continue 'dispatch;
	}
	// 82EFE610: 7D5D5850  subf r10, r29, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82EFE614: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE618: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EFE61C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82EFE620: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82EFE624: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFE628: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 82EFE62C: 4BDAAE55  bl 0x82ca9480
	ctx.lr = 0x82EFE630;
	sub_82CA9480(ctx, base);
	// 82EFE630: 4800007C  b 0x82efe6ac
	pc = 0x82EFE6AC; continue 'dispatch;
	// 82EFE634: 57DC083C  slwi r28, r30, 1
	ctx.r[28].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82EFE638: 387C0010  addi r3, r28, 0x10
	ctx.r[3].s64 = ctx.r[28].s64 + 16;
	// 82EFE63C: 4BFFF225  bl 0x82efd860
	ctx.lr = 0x82EFE640;
	sub_82EFD860(ctx, base);
	// 82EFE640: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFE644: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFE648: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EFE64C: 3BBF000C  addi r29, r31, 0xc
	ctx.r[29].s64 = ctx.r[31].s64 + 12;
	// 82EFE650: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EFE654: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFE658: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFE65C: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82EFE660: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82EFE664: 4BDAAE1D  bl 0x82ca9480
	ctx.lr = 0x82EFE668;
	sub_82CA9480(ctx, base);
	// 82EFE668: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFE66C: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82EFE670: 7D7DF1AE  stbx r11, r29, r30
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32), ctx.r[11].u8) };
	// 82EFE674: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE678: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 82EFE67C: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82EFE680: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFE684: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EFE688: 7D285214  add r9, r8, r10
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82EFE68C: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFE690: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFE694: 4082FFE8  bne 0x82efe67c
	if !ctx.cr[0].eq {
	pc = 0x82EFE67C; continue 'dispatch;
	}
	// 82EFE698: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EFE69C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFE6A0: 40820008  bne 0x82efe6a8
	if !ctx.cr[0].eq {
	pc = 0x82EFE6A8; continue 'dispatch;
	}
	// 82EFE6A4: 4BFFF1FD  bl 0x82efd8a0
	ctx.lr = 0x82EFE6A8;
	sub_82EFD8A0(ctx, base);
	// 82EFE6A8: 93FB0000  stw r31, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82EFE6AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFE6B0: 4BDAADA4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFE6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFE6B8 size=224
    let mut pc: u32 = 0x82EFE6B8;
    'dispatch: loop {
        match pc {
            0x82EFE6B8 => {
    //   block [0x82EFE6B8..0x82EFE798)
	// 82EFE6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFE6BC: 4BDAAD49  bl 0x82ca9404
	ctx.lr = 0x82EFE6C0;
	sub_82CA93D0(ctx, base);
	// 82EFE6C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFE6C4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EFE6C8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82EFE6CC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82EFE6D0: 409A000C  bne cr6, 0x82efe6dc
	if !ctx.cr[6].eq {
	pc = 0x82EFE6DC; continue 'dispatch;
	}
	// 82EFE6D4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EFE6D8: 3B8B0AF8  addi r28, r11, 0xaf8
	ctx.r[28].s64 = ctx.r[11].s64 + 2808;
	// 82EFE6DC: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82EFE6E0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EFE6E4: 48006955  bl 0x82f05038
	ctx.lr = 0x82EFE6E8;
	sub_82F05038(ctx, base);
	// 82EFE6E8: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE6EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFE6F0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFE6F4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EFE6F8: 409A002C  bne cr6, 0x82efe724
	if !ctx.cr[6].eq {
	pc = 0x82EFE724; continue 'dispatch;
	}
	// 82EFE6FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFE700: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFE704: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EFE708: 4BFFF539  bl 0x82efdc40
	ctx.lr = 0x82EFE70C;
	sub_82EFDC40(ctx, base);
	// 82EFE70C: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE710: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82EFE714: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EFE718: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82EFE71C: 480069A5  bl 0x82f050c0
	ctx.lr = 0x82EFE720;
	sub_82F050C0(ctx, base);
	// 82EFE720: 48000070  b 0x82efe790
	pc = 0x82EFE790; continue 'dispatch;
	// 82EFE724: 57DD083C  slwi r29, r30, 1
	ctx.r[29].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82EFE728: 387D0010  addi r3, r29, 0x10
	ctx.r[3].s64 = ctx.r[29].s64 + 16;
	// 82EFE72C: 4BFFF135  bl 0x82efd860
	ctx.lr = 0x82EFE730;
	sub_82EFD860(ctx, base);
	// 82EFE730: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFE734: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFE738: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82EFE73C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EFE740: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82EFE744: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFE748: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82EFE74C: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82EFE750: 48006971  bl 0x82f050c0
	ctx.lr = 0x82EFE754;
	sub_82F050C0(ctx, base);
	// 82EFE754: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE758: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82EFE75C: 38E30008  addi r7, r3, 8
	ctx.r[7].s64 = ctx.r[3].s64 + 8;
	// 82EFE760: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EFE764: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFE768: 7D603828  lwarx r11, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 82EFE76C: 7D495A14  add r10, r9, r11
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82EFE770: 7D40392D  stwcx. r10, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFE774: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFE778: 4082FFE8  bne 0x82efe760
	if !ctx.cr[0].eq {
	pc = 0x82EFE760; continue 'dispatch;
	}
	// 82EFE77C: 7D6B5B78  mr r11, r11
	ctx.r[11].u64 = ctx.r[11].u64;
	// 82EFE780: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFE784: 40820008  bne 0x82efe78c
	if !ctx.cr[0].eq {
	pc = 0x82EFE78C; continue 'dispatch;
	}
	// 82EFE788: 4BFFF119  bl 0x82efd8a0
	ctx.lr = 0x82EFE78C;
	sub_82EFD8A0(ctx, base);
	// 82EFE78C: 93FB0000  stw r31, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82EFE790: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFE794: 4BDAACC0  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFE798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFE798 size=156
    let mut pc: u32 = 0x82EFE798;
    'dispatch: loop {
        match pc {
            0x82EFE798 => {
    //   block [0x82EFE798..0x82EFE834)
	// 82EFE798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFE79C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFE7A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFE7A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFE7A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFE7AC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFE7B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFE7B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EFE7B8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE7BC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82EFE7C0: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82EFE7C4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFE7C8: 7D205828  lwarx r9, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82EFE7CC: 7D0A4A14  add r8, r10, r9
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EFE7D0: 7D00592D  stwcx. r8, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFE7D4: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFE7D8: 4082FFE8  bne 0x82efe7c0
	if !ctx.cr[0].eq {
	pc = 0x82EFE7C0; continue 'dispatch;
	}
	// 82EFE7DC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE7E0: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82EFE7E4: 38E30008  addi r7, r3, 8
	ctx.r[7].s64 = ctx.r[3].s64 + 8;
	// 82EFE7E8: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EFE7EC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFE7F0: 7D603828  lwarx r11, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 82EFE7F4: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EFE7F8: 7D20392D  stwcx. r9, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFE7FC: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFE800: 4082FFE8  bne 0x82efe7e8
	if !ctx.cr[0].eq {
	pc = 0x82EFE7E8; continue 'dispatch;
	}
	// 82EFE804: 7D6B5B78  mr r11, r11
	ctx.r[11].u64 = ctx.r[11].u64;
	// 82EFE808: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFE80C: 40820008  bne 0x82efe814
	if !ctx.cr[0].eq {
	pc = 0x82EFE814; continue 'dispatch;
	}
	// 82EFE810: 4BFFF091  bl 0x82efd8a0
	ctx.lr = 0x82EFE814;
	sub_82EFD8A0(ctx, base);
	// 82EFE814: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE818: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFE81C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFE820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFE824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFE828: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFE82C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFE830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFE838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFE838 size=328
    let mut pc: u32 = 0x82EFE838;
    'dispatch: loop {
        match pc {
            0x82EFE838 => {
    //   block [0x82EFE838..0x82EFE980)
	// 82EFE838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFE83C: 4BDAABC1  bl 0x82ca93fc
	ctx.lr = 0x82EFE840;
	sub_82CA93D0(ctx, base);
	// 82EFE840: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFE844: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFE848: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82EFE84C: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82EFE850: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE854: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE858: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE85C: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE860: 551F007E  clrlwi r31, r8, 1
	ctx.r[31].u64 = ctx.r[8].u32 as u64 & 0x7FFFFFFFu64;
	// 82EFE864: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFE868: 7D084838  and r8, r8, r9
	ctx.r[8].u64 = ctx.r[8].u64 & ctx.r[9].u64;
	// 82EFE86C: 553D007E  clrlwi r29, r9, 1
	ctx.r[29].u64 = ctx.r[9].u32 as u64 & 0x7FFFFFFFu64;
	// 82EFE870: 55090000  rlwinm r9, r8, 0, 0, 0
	ctx.r[9].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82EFE874: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EFE878: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82EFE87C: 7F9FEA14  add r28, r31, r29
	ctx.r[28].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 82EFE880: 7D4B0034  cntlzw r11, r10
	ctx.r[11].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82EFE884: 557ADFFE  rlwinm r26, r11, 0x1b, 0x1f, 0x1f
	ctx.r[26].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82EFE888: 409A003C  bne cr6, 0x82efe8c4
	if !ctx.cr[6].eq {
	pc = 0x82EFE8C4; continue 'dispatch;
	}
	// 82EFE88C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82EFE890: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EFE894: 4BFFF3AD  bl 0x82efdc40
	ctx.lr = 0x82EFE898;
	sub_82EFDC40(ctx, base);
	// 82EFE898: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE89C: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE8A0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EFE8A4: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EFE8A8: 388A000C  addi r4, r10, 0xc
	ctx.r[4].s64 = ctx.r[10].s64 + 12;
	// 82EFE8AC: 3BEB000C  addi r31, r11, 0xc
	ctx.r[31].s64 = ctx.r[11].s64 + 12;
	// 82EFE8B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFE8B4: 4BDAABCD  bl 0x82ca9480
	ctx.lr = 0x82EFE8B8;
	sub_82CA9480(ctx, base);
	// 82EFE8B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFE8BC: 7D7FE9AE  stbx r11, r31, r29
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32), ctx.r[11].u8) };
	// 82EFE8C0: 480000B8  b 0x82efe978
	pc = 0x82EFE978; continue 'dispatch;
	// 82EFE8C4: 5799083C  slwi r25, r28, 1
	ctx.r[25].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 82EFE8C8: 38790010  addi r3, r25, 0x10
	ctx.r[3].s64 = ctx.r[25].s64 + 16;
	// 82EFE8CC: 4BFFEF95  bl 0x82efd860
	ctx.lr = 0x82EFE8D0;
	sub_82EFD860(ctx, base);
	// 82EFE8D0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EFE8D4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFE8D8: 574A063F  clrlwi. r10, r26, 0x18
	ctx.r[10].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EFE8DC: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFE8E0: 933D0004  stw r25, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82EFE8E4: 939D0000  stw r28, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82EFE8E8: 4182000C  beq 0x82efe8f4
	if ctx.cr[0].eq {
	pc = 0x82EFE8F4; continue 'dispatch;
	}
	// 82EFE8EC: 678B8000  oris r11, r28, 0x8000
	ctx.r[11].u64 = ctx.r[28].u64 | 2147483648;
	// 82EFE8F0: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFE8F4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE8F8: 3B9D000C  addi r28, r29, 0xc
	ctx.r[28].s64 = ctx.r[29].s64 + 12;
	// 82EFE8FC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EFE900: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82EFE904: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EFE908: 4BDAAB79  bl 0x82ca9480
	ctx.lr = 0x82EFE90C;
	sub_82CA9480(ctx, base);
	// 82EFE90C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82EFE910: 7D7DFA14  add r11, r29, r31
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[31].u64;
	// 82EFE914: 7F5CF9AE  stbx r26, r28, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[31].u32), ctx.r[26].u8) };
	// 82EFE918: 3BEB000C  addi r31, r11, 0xc
	ctx.r[31].s64 = ctx.r[11].s64 + 12;
	// 82EFE91C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFE920: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE924: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82EFE928: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE92C: 557C007E  clrlwi r28, r11, 1
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82EFE930: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EFE934: 4BDAAB4D  bl 0x82ca9480
	ctx.lr = 0x82EFE938;
	sub_82CA9480(ctx, base);
	// 82EFE938: 7F5FE1AE  stbx r26, r31, r28
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[28].u32), ctx.r[26].u8) };
	// 82EFE93C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE940: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82EFE944: 38E30008  addi r7, r3, 8
	ctx.r[7].s64 = ctx.r[3].s64 + 8;
	// 82EFE948: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EFE94C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFE950: 7D603828  lwarx r11, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 82EFE954: 7D495A14  add r10, r9, r11
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82EFE958: 7D40392D  stwcx. r10, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFE95C: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFE960: 4082FFE8  bne 0x82efe948
	if !ctx.cr[0].eq {
	pc = 0x82EFE948; continue 'dispatch;
	}
	// 82EFE964: 7D6B5B78  mr r11, r11
	ctx.r[11].u64 = ctx.r[11].u64;
	// 82EFE968: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFE96C: 40820008  bne 0x82efe974
	if !ctx.cr[0].eq {
	pc = 0x82EFE974; continue 'dispatch;
	}
	// 82EFE970: 4BFFEF31  bl 0x82efd8a0
	ctx.lr = 0x82EFE974;
	sub_82EFD8A0(ctx, base);
	// 82EFE974: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82EFE978: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EFE97C: 4BDAAAD0  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFE980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFE980 size=312
    let mut pc: u32 = 0x82EFE980;
    'dispatch: loop {
        match pc {
            0x82EFE980 => {
    //   block [0x82EFE980..0x82EFEAB8)
	// 82EFE980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFE984: 4BDAAA79  bl 0x82ca93fc
	ctx.lr = 0x82EFE988;
	sub_82CA93D0(ctx, base);
	// 82EFE988: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFE98C: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82EFE990: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82EFE994: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82EFE998: 409A000C  bne cr6, 0x82efe9a4
	if !ctx.cr[6].eq {
	pc = 0x82EFE9A4; continue 'dispatch;
	}
	// 82EFE99C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EFE9A0: 3B2B0CA0  addi r25, r11, 0xca0
	ctx.r[25].s64 = ctx.r[11].s64 + 3232;
	// 82EFE9A4: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82EFE9A8: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE9AC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EFE9B0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EFE9B4: 409AFFF4  bne cr6, 0x82efe9a8
	if !ctx.cr[6].eq {
	pc = 0x82EFE9A8; continue 'dispatch;
	}
	// 82EFE9B8: 7D795850  subf r11, r25, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[25].s64;
	// 82EFE9BC: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE9C0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EFE9C4: 557D003E  slwi r29, r11, 0
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82EFE9C8: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE9CC: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFE9D0: 557E007E  clrlwi r30, r11, 1
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82EFE9D4: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82EFE9D8: 7F7EEA14  add r27, r30, r29
	ctx.r[27].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 82EFE9DC: 409A003C  bne cr6, 0x82efea18
	if !ctx.cr[6].eq {
	pc = 0x82EFEA18; continue 'dispatch;
	}
	// 82EFE9E0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFE9E4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EFE9E8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EFE9EC: 4BFFF255  bl 0x82efdc40
	ctx.lr = 0x82EFE9F0;
	sub_82EFDC40(ctx, base);
	// 82EFE9F0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFE9F4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EFE9F8: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EFE9FC: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82EFEA00: 3BEB000C  addi r31, r11, 0xc
	ctx.r[31].s64 = ctx.r[11].s64 + 12;
	// 82EFEA04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFEA08: 4BDAAA79  bl 0x82ca9480
	ctx.lr = 0x82EFEA0C;
	sub_82CA9480(ctx, base);
	// 82EFEA0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFEA10: 7D7FE9AE  stbx r11, r31, r29
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32), ctx.r[11].u8) };
	// 82EFEA14: 4800009C  b 0x82efeab0
	pc = 0x82EFEAB0; continue 'dispatch;
	// 82EFEA18: 577A083C  slwi r26, r27, 1
	ctx.r[26].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 82EFEA1C: 387A0010  addi r3, r26, 0x10
	ctx.r[3].s64 = ctx.r[26].s64 + 16;
	// 82EFEA20: 4BFFEE41  bl 0x82efd860
	ctx.lr = 0x82EFEA24;
	sub_82EFD860(ctx, base);
	// 82EFEA24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFEA28: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFEA2C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EFEA30: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82EFEA34: 3B7F000C  addi r27, r31, 0xc
	ctx.r[27].s64 = ctx.r[31].s64 + 12;
	// 82EFEA38: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFEA3C: 935F0004  stw r26, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82EFEA40: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82EFEA44: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFEA48: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82EFEA4C: 4BDAAA35  bl 0x82ca9480
	ctx.lr = 0x82EFEA50;
	sub_82CA9480(ctx, base);
	// 82EFEA50: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82EFEA54: 7D7FF214  add r11, r31, r30
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 82EFEA58: 7F5BF1AE  stbx r26, r27, r30
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[30].u32), ctx.r[26].u8) };
	// 82EFEA5C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EFEA60: 3BCB000C  addi r30, r11, 0xc
	ctx.r[30].s64 = ctx.r[11].s64 + 12;
	// 82EFEA64: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82EFEA68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFEA6C: 4BDAAA15  bl 0x82ca9480
	ctx.lr = 0x82EFEA70;
	sub_82CA9480(ctx, base);
	// 82EFEA70: 7F5EE9AE  stbx r26, r30, r29
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32), ctx.r[26].u8) };
	// 82EFEA74: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFEA78: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82EFEA7C: 38E30008  addi r7, r3, 8
	ctx.r[7].s64 = ctx.r[3].s64 + 8;
	// 82EFEA80: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EFEA84: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFEA88: 7D603828  lwarx r11, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 82EFEA8C: 7D495A14  add r10, r9, r11
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82EFEA90: 7D40392D  stwcx. r10, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFEA94: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFEA98: 4082FFE8  bne 0x82efea80
	if !ctx.cr[0].eq {
	pc = 0x82EFEA80; continue 'dispatch;
	}
	// 82EFEA9C: 7D6B5B78  mr r11, r11
	ctx.r[11].u64 = ctx.r[11].u64;
	// 82EFEAA0: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFEAA4: 40820008  bne 0x82efeaac
	if !ctx.cr[0].eq {
	pc = 0x82EFEAAC; continue 'dispatch;
	}
	// 82EFEAA8: 4BFFEDF9  bl 0x82efd8a0
	ctx.lr = 0x82EFEAAC;
	sub_82EFD8A0(ctx, base);
	// 82EFEAAC: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82EFEAB0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EFEAB4: 4BDAA998  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFEAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFEAB8 size=272
    let mut pc: u32 = 0x82EFEAB8;
    'dispatch: loop {
        match pc {
            0x82EFEAB8 => {
    //   block [0x82EFEAB8..0x82EFEBC8)
	// 82EFEAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFEABC: 4BDAA945  bl 0x82ca9400
	ctx.lr = 0x82EFEAC0;
	sub_82CA93D0(ctx, base);
	// 82EFEAC0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFEAC4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EFEAC8: 988100AF  stb r4, 0xaf(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(175 as u32), ctx.r[4].u8 ) };
	// 82EFEACC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFEAD0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFEAD4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFEAD8: 555E007E  clrlwi r30, r10, 1
	ctx.r[30].u64 = ctx.r[10].u32 as u64 & 0x7FFFFFFFu64;
	// 82EFEADC: 555B0FFE  srwi r27, r10, 0x1f
	ctx.r[27].u32 = ctx.r[10].u32.wrapping_shr(31);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82EFEAE0: 3B9E0001  addi r28, r30, 1
	ctx.r[28].s64 = ctx.r[30].s64 + 1;
	// 82EFEAE4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EFEAE8: 409A0034  bne cr6, 0x82efeb1c
	if !ctx.cr[6].eq {
	pc = 0x82EFEB1C; continue 'dispatch;
	}
	// 82EFEAEC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82EFEAF0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EFEAF4: 4BFFF14D  bl 0x82efdc40
	ctx.lr = 0x82EFEAF8;
	sub_82EFDC40(ctx, base);
	// 82EFEAF8: 394100AF  addi r10, r1, 0xaf
	ctx.r[10].s64 = ctx.r[1].s64 + 175;
	// 82EFEAFC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFEB00: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82EFEB04: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EFEB08: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFEB0C: 394B000C  addi r10, r11, 0xc
	ctx.r[10].s64 = ctx.r[11].s64 + 12;
	// 82EFEB10: 990B000C  stb r8, 0xc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u8 ) };
	// 82EFEB14: 992B000D  stb r9, 0xd(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(13 as u32), ctx.r[9].u8 ) };
	// 82EFEB18: 480000A8  b 0x82efebc0
	pc = 0x82EFEBC0; continue 'dispatch;
	// 82EFEB1C: 579A083C  slwi r26, r28, 1
	ctx.r[26].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 82EFEB20: 387A0010  addi r3, r26, 0x10
	ctx.r[3].s64 = ctx.r[26].s64 + 16;
	// 82EFEB24: 4BFFED3D  bl 0x82efd860
	ctx.lr = 0x82EFEB28;
	sub_82EFD860(ctx, base);
	// 82EFEB28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFEB2C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFEB30: 576A063F  clrlwi. r10, r27, 0x18
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EFEB34: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFEB38: 935F0004  stw r26, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82EFEB3C: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82EFEB40: 4182000C  beq 0x82efeb4c
	if ctx.cr[0].eq {
	pc = 0x82EFEB4C; continue 'dispatch;
	}
	// 82EFEB44: 678B8000  oris r11, r28, 0x8000
	ctx.r[11].u64 = ctx.r[28].u64 | 2147483648;
	// 82EFEB48: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFEB4C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFEB50: 3B9F000C  addi r28, r31, 0xc
	ctx.r[28].s64 = ctx.r[31].s64 + 12;
	// 82EFEB54: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EFEB58: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82EFEB5C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EFEB60: 4BDAA921  bl 0x82ca9480
	ctx.lr = 0x82EFEB64;
	sub_82CA9480(ctx, base);
	// 82EFEB64: 390100AF  addi r8, r1, 0xaf
	ctx.r[8].s64 = ctx.r[1].s64 + 175;
	// 82EFEB68: 7D7FF214  add r11, r31, r30
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 82EFEB6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82EFEB70: 394B000C  addi r10, r11, 0xc
	ctx.r[10].s64 = ctx.r[11].s64 + 12;
	// 82EFEB74: 7D3CF1AE  stbx r9, r28, r30
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[30].u32), ctx.r[9].u8) };
	// 82EFEB78: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82EFEB7C: 89080000  lbz r8, 0(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFEB80: 990B000C  stb r8, 0xc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u8 ) };
	// 82EFEB84: 992B000D  stb r9, 0xd(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(13 as u32), ctx.r[9].u8 ) };
	// 82EFEB88: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFEB8C: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 82EFEB90: 7CA000A6  mfmsr r5
	ctx.r[5].u64 = ctx.msr;
	// 82EFEB94: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFEB98: 7CE05828  lwarx r7, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[7].u64 = ctx.reserved.u32 as u64;
	// 82EFEB9C: 7CCA3A14  add r6, r10, r7
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82EFEBA0: 7CC0592D  stwcx. r6, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[6].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFEBA4: 7CA10164  mtmsrd r5, 1
	ctx.msr = (ctx.r[5].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFEBA8: 4082FFE8  bne 0x82efeb90
	if !ctx.cr[0].eq {
	pc = 0x82EFEB90; continue 'dispatch;
	}
	// 82EFEBAC: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 82EFEBB0: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFEBB4: 40820008  bne 0x82efebbc
	if !ctx.cr[0].eq {
	pc = 0x82EFEBBC; continue 'dispatch;
	}
	// 82EFEBB8: 4BFFECE9  bl 0x82efd8a0
	ctx.lr = 0x82EFEBBC;
	sub_82EFD8A0(ctx, base);
	// 82EFEBBC: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82EFEBC0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EFEBC4: 4BDAA88C  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFEBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFEBC8 size=116
    let mut pc: u32 = 0x82EFEBC8;
    'dispatch: loop {
        match pc {
            0x82EFEBC8 => {
    //   block [0x82EFEBC8..0x82EFEC3C)
	// 82EFEBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFEBCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFEBD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFEBD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFEBD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFEBDC: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82EFEBE0: 409A000C  bne cr6, 0x82efebec
	if !ctx.cr[6].eq {
	pc = 0x82EFEBEC; continue 'dispatch;
	}
	// 82EFEBE4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EFEBE8: 38AB0CA0  addi r5, r11, 0xca0
	ctx.r[5].s64 = ctx.r[11].s64 + 3232;
	// 82EFEBEC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFEBF0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EFEBF4: 38CB0008  addi r6, r11, 8
	ctx.r[6].s64 = ctx.r[11].s64 + 8;
	// 82EFEBF8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFEBFC: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82EFEC00: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFEC04: 7D203028  lwarx r9, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82EFEC08: 7D0A4A14  add r8, r10, r9
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EFEC0C: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFEC10: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFEC14: 4082FFE8  bne 0x82efebfc
	if !ctx.cr[0].eq {
	pc = 0x82EFEBFC; continue 'dispatch;
	}
	// 82EFEC18: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82EFEC1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFEC20: 4BFFFD61  bl 0x82efe980
	ctx.lr = 0x82EFEC24;
	sub_82EFE980(ctx, base);
	// 82EFEC24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFEC28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFEC2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFEC30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFEC34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFEC38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFEC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFEC40 size=100
    let mut pc: u32 = 0x82EFEC40;
    'dispatch: loop {
        match pc {
            0x82EFEC40 => {
    //   block [0x82EFEC40..0x82EFECA4)
	// 82EFEC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFEC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFEC48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFEC4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFEC50: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFEC54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFEC58: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EFEC5C: 38CB0008  addi r6, r11, 8
	ctx.r[6].s64 = ctx.r[11].s64 + 8;
	// 82EFEC60: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFEC64: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82EFEC68: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFEC6C: 7D203028  lwarx r9, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82EFEC70: 7D0A4A14  add r8, r10, r9
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EFEC74: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFEC78: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFEC7C: 4082FFE8  bne 0x82efec64
	if !ctx.cr[0].eq {
	pc = 0x82EFEC64; continue 'dispatch;
	}
	// 82EFEC80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFEC84: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82EFEC88: 4BFFFBB1  bl 0x82efe838
	ctx.lr = 0x82EFEC8C;
	sub_82EFE838(ctx, base);
	// 82EFEC8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFEC90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFEC94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFEC98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFEC9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFECA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFECA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFECA8 size=284
    let mut pc: u32 = 0x82EFECA8;
    'dispatch: loop {
        match pc {
            0x82EFECA8 => {
    //   block [0x82EFECA8..0x82EFEDC4)
	// 82EFECA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFECAC: 4BDAA75D  bl 0x82ca9408
	ctx.lr = 0x82EFECB0;
	sub_82CA93D0(ctx, base);
	// 82EFECB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFECB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFECB8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EFECBC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82EFECC0: 4BFFEEB9  bl 0x82efdb78
	ctx.lr = 0x82EFECC4;
	sub_82EFDB78(ctx, base);
	// 82EFECC4: 7D7C1850  subf r11, r28, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[28].s64;
	// 82EFECC8: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82EFECCC: 419800F0  blt cr6, 0x82efedbc
	if ctx.cr[6].lt {
	pc = 0x82EFEDBC; continue 'dispatch;
	}
	// 82EFECD0: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82EFECD4: 409A0040  bne cr6, 0x82efed14
	if !ctx.cr[6].eq {
	pc = 0x82EFED14; continue 'dispatch;
	}
	// 82EFECD8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFECDC: 4BFFEBC5  bl 0x82efd8a0
	ctx.lr = 0x82EFECE0;
	sub_82EFD8A0(ctx, base);
	// 82EFECE0: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82EFECE4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82EFECE8: 396BE2BC  addi r11, r11, -0x1d44
	ctx.r[11].s64 = ctx.r[11].s64 + -7492;
	// 82EFECEC: 38CB0008  addi r6, r11, 8
	ctx.r[6].s64 = ctx.r[11].s64 + 8;
	// 82EFECF0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFECF4: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EFECF8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFECFC: 7D403028  lwarx r10, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EFED00: 7D275214  add r9, r7, r10
	ctx.r[9].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 82EFED04: 7D20312D  stwcx. r9, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFED08: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFED0C: 4082FFE8  bne 0x82efecf4
	if !ctx.cr[0].eq {
	pc = 0x82EFECF4; continue 'dispatch;
	}
	// 82EFED10: 480000AC  b 0x82efedbc
	pc = 0x82EFEDBC; continue 'dispatch;
	// 82EFED14: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFED18: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFED1C: 55490001  rlwinm. r9, r10, 0, 0, 0
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EFED20: 41820028  beq 0x82efed48
	if ctx.cr[0].eq {
	pc = 0x82EFED48; continue 'dispatch;
	}
	// 82EFED24: 554A007E  clrlwi r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x7FFFFFFFu64;
	// 82EFED28: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82EFED2C: 7D3C5050  subf r9, r28, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[28].s64;
	// 82EFED30: 7D4BEA14  add r10, r11, r29
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82EFED34: 7D3D4850  subf r9, r29, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[29].s64;
	// 82EFED38: 388A000C  addi r4, r10, 0xc
	ctx.r[4].s64 = ctx.r[10].s64 + 12;
	// 82EFED3C: 38A90001  addi r5, r9, 1
	ctx.r[5].s64 = ctx.r[9].s64 + 1;
	// 82EFED40: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82EFED44: 48000064  b 0x82efeda8
	pc = 0x82EFEDA8; continue 'dispatch;
	// 82EFED48: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82EFED4C: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82EFED50: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EFED54: 48006515  bl 0x82f05268
	ctx.lr = 0x82EFED58;
	sub_82F05268(ctx, base);
	// 82EFED58: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFED5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFED60: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82EFED64: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82EFED68: 7C7CEA14  add r3, r28, r29
	ctx.r[3].u64 = ctx.r[28].u64 + ctx.r[29].u64;
	// 82EFED6C: 480064FD  bl 0x82f05268
	ctx.lr = 0x82EFED70;
	sub_82F05268(ctx, base);
	// 82EFED70: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82EFED74: 7F0AF000  cmpw cr6, r10, r30
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82EFED78: 419A0044  beq cr6, 0x82efedbc
	if ctx.cr[6].eq {
	pc = 0x82EFEDBC; continue 'dispatch;
	}
	// 82EFED7C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFED80: 7FBE5050  subf r29, r30, r10
	ctx.r[29].s64 = ctx.r[10].s64 - ctx.r[30].s64;
	// 82EFED84: 7D2BEA14  add r9, r11, r29
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82EFED88: 7D0BF214  add r8, r11, r30
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EFED8C: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 82EFED90: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFED94: 3868000C  addi r3, r8, 0xc
	ctx.r[3].s64 = ctx.r[8].s64 + 12;
	// 82EFED98: 3889000C  addi r4, r9, 0xc
	ctx.r[4].s64 = ctx.r[9].s64 + 12;
	// 82EFED9C: 556B007E  clrlwi r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82EFEDA0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82EFEDA4: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 82EFEDA8: 4BDAA6D9  bl 0x82ca9480
	ctx.lr = 0x82EFEDAC;
	sub_82CA9480(ctx, base);
	// 82EFEDAC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFEDB0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFEDB4: 7D5D5050  subf r10, r29, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[29].s64;
	// 82EFEDB8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EFEDBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFEDC0: 4BDAA698  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFEDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFEDC8 size=380
    let mut pc: u32 = 0x82EFEDC8;
    'dispatch: loop {
        match pc {
            0x82EFEDC8 => {
    //   block [0x82EFEDC8..0x82EFEF44)
	// 82EFEDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFEDCC: 4BDAA63D  bl 0x82ca9408
	ctx.lr = 0x82EFEDD0;
	sub_82CA93D0(ctx, base);
	// 82EFEDD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFEDD4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFEDD8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EFEDDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFEDE0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82EFEDE4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82EFEDE8: 4BFFED91  bl 0x82efdb78
	ctx.lr = 0x82EFEDEC;
	sub_82EFDB78(ctx, base);
	// 82EFEDEC: 7F1C1840  cmplw cr6, r28, r3
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82EFEDF0: 40990038  ble cr6, 0x82efee28
	if !ctx.cr[6].gt {
	pc = 0x82EFEE28; continue 'dispatch;
	}
	// 82EFEDF4: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82EFEDF8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82EFEDFC: 396BE2BC  addi r11, r11, -0x1d44
	ctx.r[11].s64 = ctx.r[11].s64 + -7492;
	// 82EFEE00: 38CB0008  addi r6, r11, 8
	ctx.r[6].s64 = ctx.r[11].s64 + 8;
	// 82EFEE04: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFEE08: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EFEE0C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFEE10: 7D403028  lwarx r10, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EFEE14: 7D275214  add r9, r7, r10
	ctx.r[9].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 82EFEE18: 7D20312D  stwcx. r9, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFEE1C: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFEE20: 4082FFE8  bne 0x82efee08
	if !ctx.cr[0].eq {
	pc = 0x82EFEE08; continue 'dispatch;
	}
	// 82EFEE24: 48000114  b 0x82efef38
	pc = 0x82EFEF38; continue 'dispatch;
	// 82EFEE28: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFEE2C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFEE30: 554A0001  rlwinm. r10, r10, 0, 0, 0
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EFEE34: 41820068  beq 0x82efee9c
	if ctx.cr[0].eq {
	pc = 0x82EFEE9C; continue 'dispatch;
	}
	// 82EFEE38: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EFEE3C: 7CBFE050  subf r5, r31, r28
	ctx.r[5].s64 = ctx.r[28].s64 - ctx.r[31].s64;
	// 82EFEE40: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82EFEE44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EFEE48: 4BFFF221  bl 0x82efe068
	ctx.lr = 0x82EFEE4C;
	sub_82EFE068(ctx, base);
	// 82EFEE4C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFEE50: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82EFEE54: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 82EFEE58: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EFEE5C: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82EFEE60: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFEE64: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EFEE68: 7D285214  add r9, r8, r10
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82EFEE6C: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFEE70: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFEE74: 4082FFE8  bne 0x82efee5c
	if !ctx.cr[0].eq {
	pc = 0x82EFEE5C; continue 'dispatch;
	}
	// 82EFEE78: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82EFEE7C: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82EFEE80: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFEE84: 7D205828  lwarx r9, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82EFEE88: 7D0A4A14  add r8, r10, r9
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EFEE8C: 7D00592D  stwcx. r8, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFEE90: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFEE94: 4082FFE8  bne 0x82efee7c
	if !ctx.cr[0].eq {
	pc = 0x82EFEE7C; continue 'dispatch;
	}
	// 82EFEE98: 48000090  b 0x82efef28
	pc = 0x82EFEF28; continue 'dispatch;
	// 82EFEE9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82EFEEA0: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82EFEEA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFEEA8: 480063C1  bl 0x82f05268
	ctx.lr = 0x82EFEEAC;
	sub_82F05268(ctx, base);
	// 82EFEEAC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFEEB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFEEB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82EFEEB8: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82EFEEBC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EFEEC0: 480063A9  bl 0x82f05268
	ctx.lr = 0x82EFEEC4;
	sub_82F05268(ctx, base);
	// 82EFEEC4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFEEC8: 7CBF1850  subf r5, r31, r3
	ctx.r[5].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	// 82EFEECC: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EFEED0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EFEED4: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82EFEED8: 4BFFF191  bl 0x82efe068
	ctx.lr = 0x82EFEEDC;
	sub_82EFE068(ctx, base);
	// 82EFEEDC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EFEEE0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82EFEEE4: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 82EFEEE8: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EFEEEC: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82EFEEF0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFEEF4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EFEEF8: 7D285214  add r9, r8, r10
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82EFEEFC: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFEF00: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFEF04: 4082FFE8  bne 0x82efeeec
	if !ctx.cr[0].eq {
	pc = 0x82EFEEEC; continue 'dispatch;
	}
	// 82EFEF08: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82EFEF0C: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82EFEF10: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFEF14: 7D205828  lwarx r9, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82EFEF18: 7D0A4A14  add r8, r10, r9
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82EFEF1C: 7D00592D  stwcx. r8, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFEF20: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFEF24: 4082FFE8  bne 0x82efef0c
	if !ctx.cr[0].eq {
	pc = 0x82EFEF0C; continue 'dispatch;
	}
	// 82EFEF28: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82EFEF2C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFEF30: 40820008  bne 0x82efef38
	if !ctx.cr[0].eq {
	pc = 0x82EFEF38; continue 'dispatch;
	}
	// 82EFEF34: 4BFFE96D  bl 0x82efd8a0
	ctx.lr = 0x82EFEF38;
	sub_82EFD8A0(ctx, base);
	// 82EFEF38: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFEF3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EFEF40: 4BDAA518  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFEF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFEF48 size=152
    let mut pc: u32 = 0x82EFEF48;
    'dispatch: loop {
        match pc {
            0x82EFEF48 => {
    //   block [0x82EFEF48..0x82EFEFE0)
	// 82EFEF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFEF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFEF50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFEF54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFEF58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFEF5C: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82EFEF60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFEF64: 3BCBE2BC  addi r30, r11, -0x1d44
	ctx.r[30].s64 = ctx.r[11].s64 + -7492;
	// 82EFEF68: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82EFEF6C: 397E0008  addi r11, r30, 8
	ctx.r[11].s64 = ctx.r[30].s64 + 8;
	// 82EFEF70: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EFEF74: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFEF78: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EFEF7C: 7D275214  add r9, r7, r10
	ctx.r[9].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 82EFEF80: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFEF84: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFEF88: 4082FFE8  bne 0x82efef70
	if !ctx.cr[0].eq {
	pc = 0x82EFEF70; continue 'dispatch;
	}
	// 82EFEF8C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFEF90: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EFEF94: 38E30008  addi r7, r3, 8
	ctx.r[7].s64 = ctx.r[3].s64 + 8;
	// 82EFEF98: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EFEF9C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFEFA0: 7D403828  lwarx r10, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EFEFA4: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EFEFA8: 7D20392D  stwcx. r9, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFEFAC: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFEFB0: 4082FFE8  bne 0x82efef98
	if !ctx.cr[0].eq {
	pc = 0x82EFEF98; continue 'dispatch;
	}
	// 82EFEFB4: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EFEFB8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFEFBC: 40820008  bne 0x82efefc4
	if !ctx.cr[0].eq {
	pc = 0x82EFEFC4; continue 'dispatch;
	}
	// 82EFEFC0: 4BFFE8E1  bl 0x82efd8a0
	ctx.lr = 0x82EFEFC4;
	sub_82EFD8A0(ctx, base);
	// 82EFEFC4: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82EFEFC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFEFCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFEFD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFEFD4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFEFD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFEFDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFEFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFEFE0 size=144
    let mut pc: u32 = 0x82EFEFE0;
    'dispatch: loop {
        match pc {
            0x82EFEFE0 => {
    //   block [0x82EFEFE0..0x82EFF070)
	// 82EFEFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFEFE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFEFE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFEFEC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFEFF0: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFEFF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFEFF8: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82EFEFFC: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82EFF000: 396BE2BC  addi r11, r11, -0x1d44
	ctx.r[11].s64 = ctx.r[11].s64 + -7492;
	// 82EFF004: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82EFF008: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82EFF00C: 38AB0008  addi r5, r11, 8
	ctx.r[5].s64 = ctx.r[11].s64 + 8;
	// 82EFF010: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFF014: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82EFF018: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFF01C: 7D202828  lwarx r9, 0, r5
	// lwarx
	let ea = ctx.r[5].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82EFF020: 7D064A14  add r8, r6, r9
	ctx.r[8].u64 = ctx.r[6].u64 + ctx.r[9].u64;
	// 82EFF024: 7D00292D  stwcx. r8, 0, r5
	// stwcx.
	let addr = ctx.r[5].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFF028: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFF02C: 4082FFE8  bne 0x82eff014
	if !ctx.cr[0].eq {
	pc = 0x82EFF014; continue 'dispatch;
	}
	// 82EFF030: 48000018  b 0x82eff048
	pc = 0x82EFF048; continue 'dispatch;
	// 82EFF034: 5463043E  clrlwi r3, r3, 0x10
	ctx.r[3].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82EFF038: 4810A5B1  bl 0x830095e8
	ctx.lr = 0x82EFF03C;
	sub_830095E8(ctx, base);
	// 82EFF03C: 5464043E  clrlwi r4, r3, 0x10
	ctx.r[4].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82EFF040: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFF044: 4BFFF1C5  bl 0x82efe208
	ctx.lr = 0x82EFF048;
	sub_82EFE208(ctx, base);
	// 82EFF048: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EFF04C: 48005ACD  bl 0x82f04b18
	ctx.lr = 0x82EFF050;
	sub_82F04B18(ctx, base);
	// 82EFF050: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFF054: 4082FFE0  bne 0x82eff034
	if !ctx.cr[0].eq {
	pc = 0x82EFF034; continue 'dispatch;
	}
	// 82EFF058: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFF05C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFF060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFF064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFF068: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFF06C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFF070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFF070 size=144
    let mut pc: u32 = 0x82EFF070;
    'dispatch: loop {
        match pc {
            0x82EFF070 => {
    //   block [0x82EFF070..0x82EFF100)
	// 82EFF070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFF074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFF078: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFF07C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFF080: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFF084: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFF088: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82EFF08C: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82EFF090: 396BE2BC  addi r11, r11, -0x1d44
	ctx.r[11].s64 = ctx.r[11].s64 + -7492;
	// 82EFF094: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82EFF098: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82EFF09C: 38AB0008  addi r5, r11, 8
	ctx.r[5].s64 = ctx.r[11].s64 + 8;
	// 82EFF0A0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFF0A4: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82EFF0A8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFF0AC: 7D202828  lwarx r9, 0, r5
	// lwarx
	let ea = ctx.r[5].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82EFF0B0: 7D064A14  add r8, r6, r9
	ctx.r[8].u64 = ctx.r[6].u64 + ctx.r[9].u64;
	// 82EFF0B4: 7D00292D  stwcx. r8, 0, r5
	// stwcx.
	let addr = ctx.r[5].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFF0B8: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFF0BC: 4082FFE8  bne 0x82eff0a4
	if !ctx.cr[0].eq {
	pc = 0x82EFF0A4; continue 'dispatch;
	}
	// 82EFF0C0: 48000018  b 0x82eff0d8
	pc = 0x82EFF0D8; continue 'dispatch;
	// 82EFF0C4: 5463043E  clrlwi r3, r3, 0x10
	ctx.r[3].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82EFF0C8: 4B27BE09  bl 0x8217aed0
	ctx.lr = 0x82EFF0CC;
	sub_8217AED0(ctx, base);
	// 82EFF0CC: 5464043E  clrlwi r4, r3, 0x10
	ctx.r[4].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82EFF0D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFF0D4: 4BFFF135  bl 0x82efe208
	ctx.lr = 0x82EFF0D8;
	sub_82EFE208(ctx, base);
	// 82EFF0D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EFF0DC: 48005A3D  bl 0x82f04b18
	ctx.lr = 0x82EFF0E0;
	sub_82F04B18(ctx, base);
	// 82EFF0E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFF0E4: 4082FFE0  bne 0x82eff0c4
	if !ctx.cr[0].eq {
	pc = 0x82EFF0C4; continue 'dispatch;
	}
	// 82EFF0E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFF0EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFF0F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFF0F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFF0F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFF0FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFF100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFF100 size=380
    let mut pc: u32 = 0x82EFF100;
    'dispatch: loop {
        match pc {
            0x82EFF100 => {
    //   block [0x82EFF100..0x82EFF27C)
	// 82EFF100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFF104: 4BDAA2F1  bl 0x82ca93f4
	ctx.lr = 0x82EFF108;
	sub_82CA93D0(ctx, base);
	// 82EFF108: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFF10C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82EFF110: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82EFF114: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82EFF118: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82EFF11C: 40980028  bge cr6, 0x82eff144
	if !ctx.cr[6].lt {
	pc = 0x82EFF144; continue 'dispatch;
	}
	// 82EFF120: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82EFF124: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFF128: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EFF12C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EFF130: 409AFFF4  bne cr6, 0x82eff124
	if !ctx.cr[6].eq {
	pc = 0x82EFF124; continue 'dispatch;
	}
	// 82EFF134: 7D785850  subf r11, r24, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[24].s64;
	// 82EFF138: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EFF13C: 557C003E  slwi r28, r11, 0
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82EFF140: 48000008  b 0x82eff148
	pc = 0x82EFF148; continue 'dispatch;
	// 82EFF144: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82EFF148: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFF14C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFF150: 5559007E  clrlwi r25, r10, 1
	ctx.r[25].u64 = ctx.r[10].u32 as u64 & 0x7FFFFFFFu64;
	// 82EFF154: 554A0001  rlwinm. r10, r10, 0, 0, 0
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EFF158: 7FB9E214  add r29, r25, r28
	ctx.r[29].u64 = ctx.r[25].u64 + ctx.r[28].u64;
	// 82EFF15C: 40820010  bne 0x82eff16c
	if !ctx.cr[0].eq {
	pc = 0x82EFF16C; continue 'dispatch;
	}
	// 82EFF160: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82EFF164: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82EFF168: 48006101  bl 0x82f05268
	ctx.lr = 0x82EFF16C;
	sub_82F05268(ctx, base);
	// 82EFF16C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFF170: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82EFF174: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFF178: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EFF17C: 409A0048  bne cr6, 0x82eff1c4
	if !ctx.cr[6].eq {
	pc = 0x82EFF1C4; continue 'dispatch;
	}
	// 82EFF180: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82EFF184: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EFF188: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EFF18C: 4BFFEAB5  bl 0x82efdc40
	ctx.lr = 0x82EFF190;
	sub_82EFDC40(ctx, base);
	// 82EFF190: 7D7BC850  subf r11, r27, r25
	ctx.r[11].s64 = ctx.r[25].s64 - ctx.r[27].s64;
	// 82EFF194: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 82EFF198: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFF19C: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82EFF1A0: 7FEBDA14  add r31, r11, r27
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82EFF1A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFF1A8: 7C7FE214  add r3, r31, r28
	ctx.r[3].u64 = ctx.r[31].u64 + ctx.r[28].u64;
	// 82EFF1AC: 4BDB1985  bl 0x82cb0b30
	ctx.lr = 0x82EFF1B0;
	sub_82CB0B30(ctx, base);
	// 82EFF1B0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EFF1B4: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82EFF1B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFF1BC: 4BDAA2C5  bl 0x82ca9480
	ctx.lr = 0x82EFF1C0;
	sub_82CA9480(ctx, base);
	// 82EFF1C0: 480000B0  b 0x82eff270
	pc = 0x82EFF270; continue 'dispatch;
	// 82EFF1C4: 57BE083C  slwi r30, r29, 1
	ctx.r[30].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82EFF1C8: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 82EFF1CC: 4BFFE695  bl 0x82efd860
	ctx.lr = 0x82EFF1D0;
	sub_82EFD860(ctx, base);
	// 82EFF1D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFF1D4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFF1D8: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82EFF1DC: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82EFF1E0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFF1E4: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82EFF1E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFF1EC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFF1F0: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82EFF1F4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFF1F8: 5577007E  clrlwi r23, r11, 1
	ctx.r[23].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82EFF1FC: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82EFF200: 4BDAA281  bl 0x82ca9480
	ctx.lr = 0x82EFF204;
	sub_82CA9480(ctx, base);
	// 82EFF204: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFF208: 7FBEDA14  add r29, r30, r27
	ctx.r[29].u64 = ctx.r[30].u64 + ctx.r[27].u64;
	// 82EFF20C: 7D7EB9AE  stbx r11, r30, r23
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[23].u32), ctx.r[11].u8) };
	// 82EFF210: 7D7BC850  subf r11, r27, r25
	ctx.r[11].s64 = ctx.r[25].s64 - ctx.r[27].s64;
	// 82EFF214: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EFF218: 7C7DE214  add r3, r29, r28
	ctx.r[3].u64 = ctx.r[29].u64 + ctx.r[28].u64;
	// 82EFF21C: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 82EFF220: 4BDB1911  bl 0x82cb0b30
	ctx.lr = 0x82EFF224;
	sub_82CB0B30(ctx, base);
	// 82EFF224: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EFF228: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82EFF22C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFF230: 4BDAA251  bl 0x82ca9480
	ctx.lr = 0x82EFF234;
	sub_82CA9480(ctx, base);
	// 82EFF234: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFF238: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82EFF23C: 38E30008  addi r7, r3, 8
	ctx.r[7].s64 = ctx.r[3].s64 + 8;
	// 82EFF240: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EFF244: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFF248: 7D603828  lwarx r11, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 82EFF24C: 7D495A14  add r10, r9, r11
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82EFF250: 7D40392D  stwcx. r10, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFF254: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFF258: 4082FFE8  bne 0x82eff240
	if !ctx.cr[0].eq {
	pc = 0x82EFF240; continue 'dispatch;
	}
	// 82EFF25C: 7D6B5B78  mr r11, r11
	ctx.r[11].u64 = ctx.r[11].u64;
	// 82EFF260: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFF264: 40820008  bne 0x82eff26c
	if !ctx.cr[0].eq {
	pc = 0x82EFF26C; continue 'dispatch;
	}
	// 82EFF268: 4BFFE639  bl 0x82efd8a0
	ctx.lr = 0x82EFF26C;
	sub_82EFD8A0(ctx, base);
	// 82EFF26C: 93FA0000  stw r31, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82EFF270: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EFF274: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EFF278: 4BDAA1CC  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFF280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFF280 size=212
    let mut pc: u32 = 0x82EFF280;
    'dispatch: loop {
        match pc {
            0x82EFF280 => {
    //   block [0x82EFF280..0x82EFF354)
	// 82EFF280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFF284: 4BDAA17D  bl 0x82ca9400
	ctx.lr = 0x82EFF288;
	sub_82CA93D0(ctx, base);
	// 82EFF288: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFF28C: 906100A4  stw r3, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[3].u32 ) };
	// 82EFF290: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 82EFF294: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82EFF298: 48005881  bl 0x82f04b18
	ctx.lr = 0x82EFF29C;
	sub_82F04B18(ctx, base);
	// 82EFF29C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFF2A0: 418200AC  beq 0x82eff34c
	if ctx.cr[0].eq {
	pc = 0x82EFF34C; continue 'dispatch;
	}
	// 82EFF2A4: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 82EFF2A8: 3D008204  lis r8, -0x7dfc
	ctx.r[8].s64 = -2113667072;
	// 82EFF2AC: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 82EFF2B0: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 82EFF2B4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82EFF2B8: 3BC7EF9C  addi r30, r7, -0x1064
	ctx.r[30].s64 = ctx.r[7].s64 + -4196;
	// 82EFF2BC: 3BA8D1F4  addi r29, r8, -0x2e0c
	ctx.r[29].s64 = ctx.r[8].s64 + -11788;
	// 82EFF2C0: 3B89EF84  addi r28, r9, -0x107c
	ctx.r[28].s64 = ctx.r[9].s64 + -4220;
	// 82EFF2C4: 3B6AEF8C  addi r27, r10, -0x1074
	ctx.r[27].s64 = ctx.r[10].s64 + -4212;
	// 82EFF2C8: 3B4BEF94  addi r26, r11, -0x106c
	ctx.r[26].s64 = ctx.r[11].s64 + -4204;
	// 82EFF2CC: 2B03003C  cmplwi cr6, r3, 0x3c
	ctx.cr[6].compare_u32(ctx.r[3].u32, 60 as u32, &mut ctx.xer);
	// 82EFF2D0: 409A000C  bne cr6, 0x82eff2dc
	if !ctx.cr[6].eq {
	pc = 0x82EFF2DC; continue 'dispatch;
	}
	// 82EFF2D4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82EFF2D8: 48000010  b 0x82eff2e8
	pc = 0x82EFF2E8; continue 'dispatch;
	// 82EFF2DC: 2B03003E  cmplwi cr6, r3, 0x3e
	ctx.cr[6].compare_u32(ctx.r[3].u32, 62 as u32, &mut ctx.xer);
	// 82EFF2E0: 409A0010  bne cr6, 0x82eff2f0
	if !ctx.cr[6].eq {
	pc = 0x82EFF2F0; continue 'dispatch;
	}
	// 82EFF2E4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EFF2E8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82EFF2EC: 48000038  b 0x82eff324
	pc = 0x82EFF324; continue 'dispatch;
	// 82EFF2F0: 2B030022  cmplwi cr6, r3, 0x22
	ctx.cr[6].compare_u32(ctx.r[3].u32, 34 as u32, &mut ctx.xer);
	// 82EFF2F4: 409A000C  bne cr6, 0x82eff300
	if !ctx.cr[6].eq {
	pc = 0x82EFF300; continue 'dispatch;
	}
	// 82EFF2F8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EFF2FC: 48000010  b 0x82eff30c
	pc = 0x82EFF30C; continue 'dispatch;
	// 82EFF300: 2B030027  cmplwi cr6, r3, 0x27
	ctx.cr[6].compare_u32(ctx.r[3].u32, 39 as u32, &mut ctx.xer);
	// 82EFF304: 409A0010  bne cr6, 0x82eff314
	if !ctx.cr[6].eq {
	pc = 0x82EFF314; continue 'dispatch;
	}
	// 82EFF308: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EFF30C: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82EFF310: 48000014  b 0x82eff324
	pc = 0x82EFF324; continue 'dispatch;
	// 82EFF314: 2B030026  cmplwi cr6, r3, 0x26
	ctx.cr[6].compare_u32(ctx.r[3].u32, 38 as u32, &mut ctx.xer);
	// 82EFF318: 409A0018  bne cr6, 0x82eff330
	if !ctx.cr[6].eq {
	pc = 0x82EFF330; continue 'dispatch;
	}
	// 82EFF31C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFF320: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82EFF324: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFF328: 4BFFF129  bl 0x82efe450
	ctx.lr = 0x82EFF32C;
	sub_82EFE450(ctx, base);
	// 82EFF32C: 48000010  b 0x82eff33c
	pc = 0x82EFF33C; continue 'dispatch;
	// 82EFF330: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EFF334: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFF338: 4BFFEED1  bl 0x82efe208
	ctx.lr = 0x82EFF33C;
	sub_82EFE208(ctx, base);
	// 82EFF33C: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 82EFF340: 480057D9  bl 0x82f04b18
	ctx.lr = 0x82EFF344;
	sub_82F04B18(ctx, base);
	// 82EFF344: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFF348: 4082FF84  bne 0x82eff2cc
	if !ctx.cr[0].eq {
	pc = 0x82EFF2CC; continue 'dispatch;
	}
	// 82EFF34C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EFF350: 4BDAA100  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFF358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFF358 size=300
    let mut pc: u32 = 0x82EFF358;
    'dispatch: loop {
        match pc {
            0x82EFF358 => {
    //   block [0x82EFF358..0x82EFF484)
	// 82EFF358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFF35C: 4BDAA0A5  bl 0x82ca9400
	ctx.lr = 0x82EFF360;
	sub_82CA93D0(ctx, base);
	// 82EFF360: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFF364: 906100A4  stw r3, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[3].u32 ) };
	// 82EFF368: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 82EFF36C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82EFF370: 480057A9  bl 0x82f04b18
	ctx.lr = 0x82EFF374;
	sub_82F04B18(ctx, base);
	// 82EFF374: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFF378: 41820104  beq 0x82eff47c
	if ctx.cr[0].eq {
	pc = 0x82EFF47C; continue 'dispatch;
	}
	// 82EFF37C: 3CE08204  lis r7, -0x7dfc
	ctx.r[7].s64 = -2113667072;
	// 82EFF380: 3D008204  lis r8, -0x7dfc
	ctx.r[8].s64 = -2113667072;
	// 82EFF384: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 82EFF388: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82EFF38C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFF390: 3BE7D218  addi r31, r7, -0x2de8
	ctx.r[31].s64 = ctx.r[7].s64 + -11752;
	// 82EFF394: 3BC8D214  addi r30, r8, -0x2dec
	ctx.r[30].s64 = ctx.r[8].s64 + -11756;
	// 82EFF398: 3BA9D20C  addi r29, r9, -0x2df4
	ctx.r[29].s64 = ctx.r[9].s64 + -11764;
	// 82EFF39C: 3B8AD204  addi r28, r10, -0x2dfc
	ctx.r[28].s64 = ctx.r[10].s64 + -11772;
	// 82EFF3A0: 3B6BD1FC  addi r27, r11, -0x2e04
	ctx.r[27].s64 = ctx.r[11].s64 + -11780;
	// 82EFF3A4: 2B030026  cmplwi cr6, r3, 0x26
	ctx.cr[6].compare_u32(ctx.r[3].u32, 38 as u32, &mut ctx.xer);
	// 82EFF3A8: 409A0088  bne cr6, 0x82eff430
	if !ctx.cr[6].eq {
	pc = 0x82EFF430; continue 'dispatch;
	}
	// 82EFF3AC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82EFF3B0: 806100A4  lwz r3, 0xa4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EFF3B4: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82EFF3B8: 4BDAAD39  bl 0x82caa0f0
	ctx.lr = 0x82EFF3BC;
	sub_82CAA0F0(ctx, base);
	// 82EFF3BC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EFF3C0: 41820090  beq 0x82eff450
	if ctx.cr[0].eq {
	pc = 0x82EFF450; continue 'dispatch;
	}
	// 82EFF3C4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EFF3C8: 806100A4  lwz r3, 0xa4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EFF3CC: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82EFF3D0: 4BDAAD21  bl 0x82caa0f0
	ctx.lr = 0x82EFF3D4;
	sub_82CAA0F0(ctx, base);
	// 82EFF3D4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EFF3D8: 41820084  beq 0x82eff45c
	if ctx.cr[0].eq {
	pc = 0x82EFF45C; continue 'dispatch;
	}
	// 82EFF3DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EFF3E0: 806100A4  lwz r3, 0xa4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EFF3E4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82EFF3E8: 4BDAAD09  bl 0x82caa0f0
	ctx.lr = 0x82EFF3EC;
	sub_82CAA0F0(ctx, base);
	// 82EFF3EC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EFF3F0: 41820074  beq 0x82eff464
	if ctx.cr[0].eq {
	pc = 0x82EFF464; continue 'dispatch;
	}
	// 82EFF3F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFF3F8: 806100A4  lwz r3, 0xa4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EFF3FC: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82EFF400: 4BDAACF1  bl 0x82caa0f0
	ctx.lr = 0x82EFF404;
	sub_82CAA0F0(ctx, base);
	// 82EFF404: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EFF408: 41820064  beq 0x82eff46c
	if ctx.cr[0].eq {
	pc = 0x82EFF46C; continue 'dispatch;
	}
	// 82EFF40C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EFF410: 806100A4  lwz r3, 0xa4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82EFF414: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82EFF418: 4BDAACD9  bl 0x82caa0f0
	ctx.lr = 0x82EFF41C;
	sub_82CAA0F0(ctx, base);
	// 82EFF41C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82EFF420: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EFF424: 41820050  beq 0x82eff474
	if ctx.cr[0].eq {
	pc = 0x82EFF474; continue 'dispatch;
	}
	// 82EFF428: 38800026  li r4, 0x26
	ctx.r[4].s64 = 38;
	// 82EFF42C: 4800000C  b 0x82eff438
	pc = 0x82EFF438; continue 'dispatch;
	// 82EFF430: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82EFF434: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EFF438: 4BFFEDD1  bl 0x82efe208
	ctx.lr = 0x82EFF43C;
	sub_82EFE208(ctx, base);
	// 82EFF43C: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 82EFF440: 480056D9  bl 0x82f04b18
	ctx.lr = 0x82EFF444;
	sub_82F04B18(ctx, base);
	// 82EFF444: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82EFF448: 4082FF5C  bne 0x82eff3a4
	if !ctx.cr[0].eq {
	pc = 0x82EFF3A4; continue 'dispatch;
	}
	// 82EFF44C: 48000030  b 0x82eff47c
	pc = 0x82EFF47C; continue 'dispatch;
	// 82EFF450: 38800022  li r4, 0x22
	ctx.r[4].s64 = 34;
	// 82EFF454: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EFF458: 48000020  b 0x82eff478
	pc = 0x82EFF478; continue 'dispatch;
	// 82EFF45C: 38800027  li r4, 0x27
	ctx.r[4].s64 = 39;
	// 82EFF460: 4BFFFFF4  b 0x82eff454
	pc = 0x82EFF454; continue 'dispatch;
	// 82EFF464: 38800026  li r4, 0x26
	ctx.r[4].s64 = 38;
	// 82EFF468: 4BFFFFEC  b 0x82eff454
	pc = 0x82EFF454; continue 'dispatch;
	// 82EFF46C: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 82EFF470: 4BFFFFE4  b 0x82eff454
	pc = 0x82EFF454; continue 'dispatch;
	// 82EFF474: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 82EFF478: 4BFFED91  bl 0x82efe208
	ctx.lr = 0x82EFF47C;
	sub_82EFE208(ctx, base);
	// 82EFF47C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EFF480: 4BDA9FD0  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFF488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFF488 size=228
    let mut pc: u32 = 0x82EFF488;
    'dispatch: loop {
        match pc {
            0x82EFF488 => {
    //   block [0x82EFF488..0x82EFF56C)
	// 82EFF488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFF48C: 4BDA9F71  bl 0x82ca93fc
	ctx.lr = 0x82EFF490;
	sub_82CA93D0(ctx, base);
	// 82EFF490: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFF494: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFF498: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82EFF49C: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82EFF4A0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFF4A4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFF4A8: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFF4AC: 555B0FFE  srwi r27, r10, 0x1f
	ctx.r[27].u32 = ctx.r[10].u32.wrapping_shr(31);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82EFF4B0: 2F090001  cmpwi cr6, r9, 1
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1, &mut ctx.xer);
	// 82EFF4B4: 419A00A0  beq cr6, 0x82eff554
	if ctx.cr[6].eq {
	pc = 0x82EFF554; continue 'dispatch;
	}
	// 82EFF4B8: 83AB0004  lwz r29, 4(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFF4BC: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFF4C0: 387D0010  addi r3, r29, 0x10
	ctx.r[3].s64 = ctx.r[29].s64 + 16;
	// 82EFF4C4: 557C007E  clrlwi r28, r11, 1
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82EFF4C8: 4BFFE399  bl 0x82efd860
	ctx.lr = 0x82EFF4CC;
	sub_82EFD860(ctx, base);
	// 82EFF4CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFF4D0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFF4D4: 576A063F  clrlwi. r10, r27, 0x18
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EFF4D8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EFF4DC: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82EFF4E0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82EFF4E4: 4182000C  beq 0x82eff4f0
	if ctx.cr[0].eq {
	pc = 0x82EFF4F0; continue 'dispatch;
	}
	// 82EFF4E8: 678B8000  oris r11, r28, 0x8000
	ctx.r[11].u64 = ctx.r[28].u64 | 2147483648;
	// 82EFF4EC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFF4F0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFF4F4: 3BBF000C  addi r29, r31, 0xc
	ctx.r[29].s64 = ctx.r[31].s64 + 12;
	// 82EFF4F8: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82EFF4FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EFF500: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFF504: 557C007E  clrlwi r28, r11, 1
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82EFF508: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82EFF50C: 4BDA9F75  bl 0x82ca9480
	ctx.lr = 0x82EFF510;
	sub_82CA9480(ctx, base);
	// 82EFF510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFF514: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82EFF518: 7D7DE1AE  stbx r11, r29, r28
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32), ctx.r[11].u8) };
	// 82EFF51C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFF520: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 82EFF524: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82EFF528: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFF52C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EFF530: 7D285214  add r9, r8, r10
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82EFF534: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFF538: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFF53C: 4082FFE8  bne 0x82eff524
	if !ctx.cr[0].eq {
	pc = 0x82EFF524; continue 'dispatch;
	}
	// 82EFF540: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EFF544: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFF548: 40820008  bne 0x82eff550
	if !ctx.cr[0].eq {
	pc = 0x82EFF550; continue 'dispatch;
	}
	// 82EFF54C: 4BFFE355  bl 0x82efd8a0
	ctx.lr = 0x82EFF550;
	sub_82EFD8A0(ctx, base);
	// 82EFF550: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82EFF554: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82EFF558: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82EFF55C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFF560: 4BFFF749  bl 0x82efeca8
	ctx.lr = 0x82EFF564;
	sub_82EFECA8(ctx, base);
	// 82EFF564: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EFF568: 4BDA9EE4  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFF570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFF570 size=36
    let mut pc: u32 = 0x82EFF570;
    'dispatch: loop {
        match pc {
            0x82EFF570 => {
    //   block [0x82EFF570..0x82EFF594)
	// 82EFF570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFF574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFF578: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFF57C: 4B366B35  bl 0x822660b0
	ctx.lr = 0x82EFF580;
	sub_822660B0(ctx, base);
	// 82EFF580: 78630020  clrldi r3, r3, 0x20
	ctx.r[3].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 82EFF584: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFF588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFF58C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFF590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFF598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFF598 size=8
    let mut pc: u32 = 0x82EFF598;
    'dispatch: loop {
        match pc {
            0x82EFF598 => {
    //   block [0x82EFF598..0x82EFF5A0)
	// 82EFF598: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFF59C: 48030464  b 0x82f2fa00
	sub_82F2FA00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFF5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFF5A0 size=84
    let mut pc: u32 = 0x82EFF5A0;
    'dispatch: loop {
        match pc {
            0x82EFF5A0 => {
    //   block [0x82EFF5A0..0x82EFF5F4)
	// 82EFF5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFF5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFF5A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFF5AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFF5B0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFF5B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFF5B8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFF5BC: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EFF5C0: 80640004  lwz r3, 4(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFF5C4: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82EFF5C8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFF5CC: 409A0010  bne cr6, 0x82eff5dc
	if !ctx.cr[6].eq {
	pc = 0x82EFF5DC; continue 'dispatch;
	}
	// 82EFF5D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFF5D4: 419A0008  beq cr6, 0x82eff5dc
	if ctx.cr[6].eq {
	pc = 0x82EFF5DC; continue 'dispatch;
	}
	// 82EFF5D8: 4BFF87E1  bl 0x82ef7db8
	ctx.lr = 0x82EFF5DC;
	sub_82EF7DB8(ctx, base);
	// 82EFF5DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFF5E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFF5E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFF5E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFF5EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFF5F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFF5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFF5F8 size=128
    let mut pc: u32 = 0x82EFF5F8;
    'dispatch: loop {
        match pc {
            0x82EFF5F8 => {
    //   block [0x82EFF5F8..0x82EFF678)
	// 82EFF5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFF5FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFF600: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFF604: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFF608: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFF60C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFF610: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFF614: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFF618: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFF61C: 409A0014  bne cr6, 0x82eff630
	if !ctx.cr[6].eq {
	pc = 0x82EFF630; continue 'dispatch;
	}
	// 82EFF620: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFF624: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFF628: 419A0008  beq cr6, 0x82eff630
	if ctx.cr[6].eq {
	pc = 0x82EFF630; continue 'dispatch;
	}
	// 82EFF62C: 4BFF878D  bl 0x82ef7db8
	ctx.lr = 0x82EFF630;
	sub_82EF7DB8(ctx, base);
	// 82EFF630: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFF634: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFF638: 409A0014  bne cr6, 0x82eff64c
	if !ctx.cr[6].eq {
	pc = 0x82EFF64C; continue 'dispatch;
	}
	// 82EFF63C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFF640: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFF644: 419A0008  beq cr6, 0x82eff64c
	if ctx.cr[6].eq {
	pc = 0x82EFF64C; continue 'dispatch;
	}
	// 82EFF648: 4BFF8C51  bl 0x82ef8298
	ctx.lr = 0x82EFF64C;
	sub_82EF8298(ctx, base);
	// 82EFF64C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFF650: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFF654: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFF658: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFF65C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82EFF660: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFF664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFF668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFF66C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFF670: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFF674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFF678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFF678 size=16
    let mut pc: u32 = 0x82EFF678;
    'dispatch: loop {
        match pc {
            0x82EFF678 => {
    //   block [0x82EFF678..0x82EFF688)
	// 82EFF678: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFF67C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFF680: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFF684: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFF688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFF688 size=16
    let mut pc: u32 = 0x82EFF688;
    'dispatch: loop {
        match pc {
            0x82EFF688 => {
    //   block [0x82EFF688..0x82EFF698)
	// 82EFF688: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFF68C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EFF690: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFF694: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFF698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EFF698 size=20
    let mut pc: u32 = 0x82EFF698;
    'dispatch: loop {
        match pc {
            0x82EFF698 => {
    //   block [0x82EFF698..0x82EFF6AC)
	// 82EFF698: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EFF69C: 3D400004  lis r10, 4
	ctx.r[10].s64 = 262144;
	// 82EFF6A0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFF6A4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EFF6A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFF6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EFF6B0 size=128
    let mut pc: u32 = 0x82EFF6B0;
    'dispatch: loop {
        match pc {
            0x82EFF6B0 => {
    //   block [0x82EFF6B0..0x82EFF730)
	// 82EFF6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFF6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFF6B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFF6BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFF6C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFF6C4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFF6C8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EFF6CC: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EFF6D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFF6D4: 409A0010  bne cr6, 0x82eff6e4
	if !ctx.cr[6].eq {
	pc = 0x82EFF6E4; continue 'dispatch;
	}
	// 82EFF6D8: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82EFF6DC: 4BFFE185  bl 0x82efd860
	ctx.lr = 0x82EFF6E0;
	sub_82EFD860(ctx, base);
	// 82EFF6E0: 907E0030  stw r3, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 82EFF6E4: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EFF6E8: C01F0000  lfs f0, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EFF6EC: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82EFF6F0: C01F0004  lfs f0, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EFF6F4: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82EFF6F8: C1BF0000  lfs f13, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EFF6FC: C01F0008  lfs f0, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EFF700: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82EFF704: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82EFF708: C01F000C  lfs f0, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EFF70C: C1BF0004  lfs f13, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EFF710: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82EFF714: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82EFF718: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFF71C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFF720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFF724: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFF728: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFF72C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFF730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EFF730 size=128
    let mut pc: u32 = 0x82EFF730;
    'dispatch: loop {
        match pc {
            0x82EFF730 => {
    //   block [0x82EFF730..0x82EFF7B0)
	// 82EFF730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFF734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFF738: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFF73C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFF740: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFF744: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFF748: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EFF74C: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EFF750: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EFF754: 409A0010  bne cr6, 0x82eff764
	if !ctx.cr[6].eq {
	pc = 0x82EFF764; continue 'dispatch;
	}
	// 82EFF758: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82EFF75C: 4BFFE105  bl 0x82efd860
	ctx.lr = 0x82EFF760;
	sub_82EFD860(ctx, base);
	// 82EFF760: 907E002C  stw r3, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 82EFF764: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EFF768: C01F0000  lfs f0, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EFF76C: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82EFF770: C01F0004  lfs f0, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EFF774: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82EFF778: C1BF0000  lfs f13, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EFF77C: C01F0008  lfs f0, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EFF780: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82EFF784: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82EFF788: C01F000C  lfs f0, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EFF78C: C1BF0004  lfs f13, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EFF790: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82EFF794: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82EFF798: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFF79C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFF7A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFF7A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFF7A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFF7AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFF7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFF7B0 size=96
    let mut pc: u32 = 0x82EFF7B0;
    'dispatch: loop {
        match pc {
            0x82EFF7B0 => {
    //   block [0x82EFF7B0..0x82EFF810)
	// 82EFF7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFF7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFF7B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFF7BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFF7C0: 81630058  lwz r11, 0x58(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EFF7C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EFF7C8: 81230054  lwz r9, 0x54(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EFF7CC: 3BE30030  addi r31, r3, 0x30
	ctx.r[31].s64 = ctx.r[3].s64 + 48;
	// 82EFF7D0: 99430045  stb r10, 0x45(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(69 as u32), ctx.r[10].u8 ) };
	// 82EFF7D4: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82EFF7D8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EFF7DC: 4098000C  bge cr6, 0x82eff7e8
	if !ctx.cr[6].lt {
	pc = 0x82EFF7E8; continue 'dispatch;
	}
	// 82EFF7E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFF7E4: 48030B75  bl 0x82f30358
	ctx.lr = 0x82EFF7E8;
	sub_82F30358(ctx, base);
	// 82EFF7E8: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFF7EC: 7D4BFA14  add r10, r11, r31
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EFF7F0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EFF7F4: 886A0030  lbz r3, 0x30(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EFF7F8: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82EFF7FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFF800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFF804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFF808: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFF80C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFF810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFF810 size=116
    let mut pc: u32 = 0x82EFF810;
    'dispatch: loop {
        match pc {
            0x82EFF810 => {
    //   block [0x82EFF810..0x82EFF884)
	// 82EFF810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFF814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFF818: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFF81C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFF820: 81630058  lwz r11, 0x58(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EFF824: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EFF828: 81230054  lwz r9, 0x54(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EFF82C: 3BE30030  addi r31, r3, 0x30
	ctx.r[31].s64 = ctx.r[3].s64 + 48;
	// 82EFF830: 99430045  stb r10, 0x45(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(69 as u32), ctx.r[10].u8 ) };
	// 82EFF834: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82EFF838: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82EFF83C: 40980010  bge cr6, 0x82eff84c
	if !ctx.cr[6].lt {
	pc = 0x82EFF84C; continue 'dispatch;
	}
	// 82EFF840: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82EFF844: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFF848: 480309F1  bl 0x82f30238
	ctx.lr = 0x82EFF84C;
	sub_82F30238(ctx, base);
	// 82EFF84C: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFF850: 7D6AFA14  add r11, r10, r31
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82EFF854: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82EFF858: 892B0031  lbz r9, 0x31(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(49 as u32) ) } as u64;
	// 82EFF85C: 896B0030  lbz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EFF860: 5529403E  rotlwi r9, r9, 8
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(8)) as u64;
	// 82EFF864: 915F0024  stw r10, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 82EFF868: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 82EFF86C: 5563043E  clrlwi r3, r11, 0x10
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82EFF870: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFF874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFF878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFF87C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFF880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFF888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFF888 size=136
    let mut pc: u32 = 0x82EFF888;
    'dispatch: loop {
        match pc {
            0x82EFF888 => {
    //   block [0x82EFF888..0x82EFF910)
	// 82EFF888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFF88C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFF890: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFF894: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFF898: 81630058  lwz r11, 0x58(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EFF89C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EFF8A0: 81230054  lwz r9, 0x54(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82EFF8A4: 3BE30030  addi r31, r3, 0x30
	ctx.r[31].s64 = ctx.r[3].s64 + 48;
	// 82EFF8A8: 99430045  stb r10, 0x45(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(69 as u32), ctx.r[10].u8 ) };
	// 82EFF8AC: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82EFF8B0: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82EFF8B4: 40980010  bge cr6, 0x82eff8c4
	if !ctx.cr[6].lt {
	pc = 0x82EFF8C4; continue 'dispatch;
	}
	// 82EFF8B8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82EFF8BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFF8C0: 48030979  bl 0x82f30238
	ctx.lr = 0x82EFF8C4;
	sub_82F30238(ctx, base);
	// 82EFF8C4: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFF8C8: 7D6AFA14  add r11, r10, r31
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82EFF8CC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82EFF8D0: 892B0033  lbz r9, 0x33(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(51 as u32) ) } as u64;
	// 82EFF8D4: 890B0032  lbz r8, 0x32(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(50 as u32) ) } as u64;
	// 82EFF8D8: 5529403E  rotlwi r9, r9, 8
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(8)) as u64;
	// 82EFF8DC: 88EB0031  lbz r7, 0x31(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(49 as u32) ) } as u64;
	// 82EFF8E0: 896B0030  lbz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EFF8E4: 7D294378  or r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 82EFF8E8: 915F0024  stw r10, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 82EFF8EC: 552A402E  slwi r10, r9, 8
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EFF8F0: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 82EFF8F4: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EFF8F8: 7D435B78  or r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82EFF8FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFF900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFF904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFF908: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFF90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFF910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFF910 size=84
    let mut pc: u32 = 0x82EFF910;
    'dispatch: loop {
        match pc {
            0x82EFF910 => {
    //   block [0x82EFF910..0x82EFF964)
	// 82EFF910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFF914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFF918: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFF91C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFF920: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFF924: 4BFFE025  bl 0x82efd948
	ctx.lr = 0x82EFF928;
	sub_82EFD948(ctx, base);
	// 82EFF928: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFF92C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82EFF930: 396BD288  addi r11, r11, -0x2d78
	ctx.r[11].s64 = ctx.r[11].s64 + -11640;
	// 82EFF934: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFF938: 4BFFE631  bl 0x82efdf68
	ctx.lr = 0x82EFF93C;
	sub_82EFDF68(ctx, base);
	// 82EFF93C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82EFF940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EFF944: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EFF948: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFF94C: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82EFF950: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFF954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFF958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFF95C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFF960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFF968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFF968 size=104
    let mut pc: u32 = 0x82EFF968;
    'dispatch: loop {
        match pc {
            0x82EFF968 => {
    //   block [0x82EFF968..0x82EFF9D0)
	// 82EFF968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFF96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFF970: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFF974: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFF978: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFF97C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EFF980: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EFF984: 38E30008  addi r7, r3, 8
	ctx.r[7].s64 = ctx.r[3].s64 + 8;
	// 82EFF988: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EFF98C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFF990: 7D403828  lwarx r10, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EFF994: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EFF998: 7D20392D  stwcx. r9, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFF99C: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFF9A0: 4082FFE8  bne 0x82eff988
	if !ctx.cr[0].eq {
	pc = 0x82EFF988; continue 'dispatch;
	}
	// 82EFF9A4: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EFF9A8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFF9AC: 40820008  bne 0x82eff9b4
	if !ctx.cr[0].eq {
	pc = 0x82EFF9B4; continue 'dispatch;
	}
	// 82EFF9B0: 4BFFDEF1  bl 0x82efd8a0
	ctx.lr = 0x82EFF9B4;
	sub_82EFD8A0(ctx, base);
	// 82EFF9B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFF9B8: 4BFFE061  bl 0x82efda18
	ctx.lr = 0x82EFF9BC;
	sub_82EFDA18(ctx, base);
	// 82EFF9BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFF9C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFF9C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFF9C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFF9CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFF9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFF9D0 size=84
    let mut pc: u32 = 0x82EFF9D0;
    'dispatch: loop {
        match pc {
            0x82EFF9D0 => {
    //   block [0x82EFF9D0..0x82EFFA24)
	// 82EFF9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFF9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFF9D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFF9DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFF9E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFF9E4: 4BFFFF2D  bl 0x82eff910
	ctx.lr = 0x82EFF9E8;
	sub_82EFF910(ctx, base);
	// 82EFF9E8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFF9EC: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82EFF9F0: 396BD28C  addi r11, r11, -0x2d74
	ctx.r[11].s64 = ctx.r[11].s64 + -11636;
	// 82EFF9F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFF9F8: 4BFFE571  bl 0x82efdf68
	ctx.lr = 0x82EFF9FC;
	sub_82EFDF68(ctx, base);
	// 82EFF9FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EFFA00: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EFFA04: B17F001E  sth r11, 0x1e(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(30 as u32), ctx.r[11].u16 ) };
	// 82EFFA08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFFA0C: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82EFFA10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFFA14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFFA18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFFA1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFFA20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFFA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFFA28 size=104
    let mut pc: u32 = 0x82EFFA28;
    'dispatch: loop {
        match pc {
            0x82EFFA28 => {
    //   block [0x82EFFA28..0x82EFFA90)
	// 82EFFA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFFA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFFA30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFFA34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFFA38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFFA3C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EFFA40: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFFA44: 38E30008  addi r7, r3, 8
	ctx.r[7].s64 = ctx.r[3].s64 + 8;
	// 82EFFA48: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82EFFA4C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFFA50: 7D403828  lwarx r10, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82EFFA54: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EFFA58: 7D20392D  stwcx. r9, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82EFFA5C: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82EFFA60: 4082FFE8  bne 0x82effa48
	if !ctx.cr[0].eq {
	pc = 0x82EFFA48; continue 'dispatch;
	}
	// 82EFFA64: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82EFFA68: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFFA6C: 40820008  bne 0x82effa74
	if !ctx.cr[0].eq {
	pc = 0x82EFFA74; continue 'dispatch;
	}
	// 82EFFA70: 4BFFDE31  bl 0x82efd8a0
	ctx.lr = 0x82EFFA74;
	sub_82EFD8A0(ctx, base);
	// 82EFFA74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFFA78: 4BFFFEF1  bl 0x82eff968
	ctx.lr = 0x82EFFA7C;
	sub_82EFF968(ctx, base);
	// 82EFFA7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFFA80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFFA84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFFA88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFFA8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFFA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFFA90 size=88
    let mut pc: u32 = 0x82EFFA90;
    'dispatch: loop {
        match pc {
            0x82EFFA90 => {
    //   block [0x82EFFA90..0x82EFFAE8)
	// 82EFFA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFFA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFFA98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFFA9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFFAA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFFAA4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFFAA8: 396BD290  addi r11, r11, -0x2d70
	ctx.r[11].s64 = ctx.r[11].s64 + -11632;
	// 82EFFAAC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFFAB0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EFFAB4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFFAB8: 409A0014  bne cr6, 0x82effacc
	if !ctx.cr[6].eq {
	pc = 0x82EFFACC; continue 'dispatch;
	}
	// 82EFFABC: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFFAC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFFAC4: 419A0008  beq cr6, 0x82effacc
	if ctx.cr[6].eq {
	pc = 0x82EFFACC; continue 'dispatch;
	}
	// 82EFFAC8: 4BFF87D1  bl 0x82ef8298
	ctx.lr = 0x82EFFACC;
	sub_82EF8298(ctx, base);
	// 82EFFACC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFFAD0: 4BFFDF49  bl 0x82efda18
	ctx.lr = 0x82EFFAD4;
	sub_82EFDA18(ctx, base);
	// 82EFFAD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFFAD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFFADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFFAE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFFAE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFFAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFFAE8 size=76
    let mut pc: u32 = 0x82EFFAE8;
    'dispatch: loop {
        match pc {
            0x82EFFAE8 => {
    //   block [0x82EFFAE8..0x82EFFB34)
	// 82EFFAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFFAEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFFAF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFFAF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFFAF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFFAFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFFB00: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EFFB04: 4BFFFF8D  bl 0x82effa90
	ctx.lr = 0x82EFFB08;
	sub_82EFFA90(ctx, base);
	// 82EFFB08: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFFB0C: 4182000C  beq 0x82effb18
	if ctx.cr[0].eq {
	pc = 0x82EFFB18; continue 'dispatch;
	}
	// 82EFFB10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFFB14: 4BFFDD8D  bl 0x82efd8a0
	ctx.lr = 0x82EFFB18;
	sub_82EFD8A0(ctx, base);
	// 82EFFB18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFFB1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFFB20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFFB24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFFB28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFFB2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFFB30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFFB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFFB38 size=116
    let mut pc: u32 = 0x82EFFB38;
    'dispatch: loop {
        match pc {
            0x82EFFB38 => {
    //   block [0x82EFFB38..0x82EFFBAC)
	// 82EFFB38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFFB3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFFB40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFFB44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFFB48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFFB4C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EFFB50: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFFB54: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EFFB58: 419A0018  beq cr6, 0x82effb70
	if ctx.cr[6].eq {
	pc = 0x82EFFB70; continue 'dispatch;
	}
	// 82EFFB5C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFFB60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFFB64: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFFB68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFFB6C: 4E800421  bctrl
	ctx.lr = 0x82EFFB70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFFB70: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EFFB74: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EFFB78: 419A0018  beq cr6, 0x82effb90
	if ctx.cr[6].eq {
	pc = 0x82EFFB90; continue 'dispatch;
	}
	// 82EFFB7C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFFB80: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EFFB84: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFFB88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFFB8C: 4E800421  bctrl
	ctx.lr = 0x82EFFB90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFFB90: 93FE0024  stw r31, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[31].u32 ) };
	// 82EFFB94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFFB98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFFB9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFFBA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFFBA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFFBA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFFBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFFBB0 size=68
    let mut pc: u32 = 0x82EFFBB0;
    'dispatch: loop {
        match pc {
            0x82EFFBB0 => {
    //   block [0x82EFFBB0..0x82EFFBF4)
	// 82EFFBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFFBB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFFBB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFFBBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFFBC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFFBC4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82EFFBC8: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EFFBCC: 396BD21C  addi r11, r11, -0x2de4
	ctx.r[11].s64 = ctx.r[11].s64 + -11748;
	// 82EFFBD0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82EFFBD4: 41820008  beq 0x82effbdc
	if ctx.cr[0].eq {
	pc = 0x82EFFBDC; continue 'dispatch;
	}
	// 82EFFBD8: 4B945BD9  bl 0x828457b0
	ctx.lr = 0x82EFFBDC;
	sub_828457B0(ctx, base);
	// 82EFFBDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EFFBE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFFBE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFFBE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFFBEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFFBF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFFBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFFBF8 size=128
    let mut pc: u32 = 0x82EFFBF8;
    'dispatch: loop {
        match pc {
            0x82EFFBF8 => {
    //   block [0x82EFFBF8..0x82EFFC78)
	// 82EFFBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFFBFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFFC00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EFFC04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFFC08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFFC0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFFC10: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82EFFC14: 480317ED  bl 0x82f31400
	ctx.lr = 0x82EFFC18;
	sub_82F31400(ctx, base);
	// 82EFFC18: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EFFC1C: 815F02D8  lwz r10, 0x2d8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(728 as u32) ) } as u64;
	// 82EFFC20: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EFFC24: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82EFFC28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EFFC2C: 80AB0014  lwz r5, 0x14(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EFFC30: 409A0014  bne cr6, 0x82effc44
	if !ctx.cr[6].eq {
	pc = 0x82EFFC44; continue 'dispatch;
	}
	// 82EFFC34: 807F02DC  lwz r3, 0x2dc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(732 as u32) ) } as u64;
	// 82EFFC38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFFC3C: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82EFFC40: 48000010  b 0x82effc50
	pc = 0x82EFFC50; continue 'dispatch;
	// 82EFFC44: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EFFC48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EFFC4C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EFFC50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82EFFC54: 4E800421  bctrl
	ctx.lr = 0x82EFFC58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EFFC58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EFFC5C: 4BFFDC45  bl 0x82efd8a0
	ctx.lr = 0x82EFFC60;
	sub_82EFD8A0(ctx, base);
	// 82EFFC60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EFFC64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFFC68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFFC6C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EFFC70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFFC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EFFC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EFFC78 size=72
    let mut pc: u32 = 0x82EFFC78;
    'dispatch: loop {
        match pc {
            0x82EFFC78 => {
    //   block [0x82EFFC78..0x82EFFCC0)
	// 82EFFC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EFFC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EFFC80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EFFC84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EFFC88: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EFFC8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EFFC90: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EFFC94: 40990018  ble cr6, 0x82effcac
	if !ctx.cr[6].gt {
	pc = 0x82EFFCAC; continue 'dispatch;
	}
	// 82EFFC98: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82EFFC9C: 480306C5  bl 0x82f30360
	ctx.lr = 0x82EFFCA0;
	sub_82F30360(ctx, base);
	// 82EFFCA0: 807F0040  lwz r3, 0x40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82EFFCA4: 480455ED  bl 0x82f45290
	ctx.lr = 0x82EFFCA8;
	sub_82F45290(ctx, base);
	// 82EFFCA8: 907F02A8  stw r3, 0x2a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(680 as u32), ctx.r[3].u32 ) };
	// 82EFFCAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EFFCB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EFFCB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EFFCB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EFFCBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


