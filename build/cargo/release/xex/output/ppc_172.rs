pub fn sub_82E3A150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3A150 size=76
    let mut pc: u32 = 0x82E3A150;
    'dispatch: loop {
        match pc {
            0x82E3A150 => {
    //   block [0x82E3A150..0x82E3A19C)
	// 82E3A150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3A154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E3A158: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E3A15C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3A160: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E3A164: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E3A168: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3A16C: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82E3A170: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3A174: 4BF23EBD  bl 0x82d5e030
	ctx.lr = 0x82E3A178;
	sub_82D5E030(ctx, base);
	// 82E3A178: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E3A17C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A180: 396B5A94  addi r11, r11, 0x5a94
	ctx.r[11].s64 = ctx.r[11].s64 + 23188;
	// 82E3A184: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E3A188: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E3A18C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3A190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E3A194: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E3A198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3A1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3A1A0 size=84
    let mut pc: u32 = 0x82E3A1A0;
    'dispatch: loop {
        match pc {
            0x82E3A1A0 => {
    //   block [0x82E3A1A0..0x82E3A1F4)
	// 82E3A1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3A1A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E3A1A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E3A1AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E3A1B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3A1B4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E3A1B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3A1BC: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3A1C0: 4BF23FD9  bl 0x82d5e198
	ctx.lr = 0x82E3A1C4;
	sub_82D5E198(ctx, base);
	// 82E3A1C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A1C8: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3A1CC: 4BF23FCD  bl 0x82d5e198
	ctx.lr = 0x82E3A1D0;
	sub_82D5E198(ctx, base);
	// 82E3A1D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A1D4: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3A1D8: 4BF23FC1  bl 0x82d5e198
	ctx.lr = 0x82E3A1DC;
	sub_82D5E198(ctx, base);
	// 82E3A1DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E3A1E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3A1E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E3A1E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E3A1EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E3A1F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3A1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3A1F8 size=176
    let mut pc: u32 = 0x82E3A1F8;
    'dispatch: loop {
        match pc {
            0x82E3A1F8 => {
    //   block [0x82E3A1F8..0x82E3A2A8)
	// 82E3A1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3A1FC: 4BE6F211  bl 0x82ca940c
	ctx.lr = 0x82E3A200;
	sub_82CA93D0(ctx, base);
	// 82E3A200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3A204: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3A208: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E3A20C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E3A210: 38A00019  li r5, 0x19
	ctx.r[5].s64 = 25;
	// 82E3A214: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 82E3A218: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3A21C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3A220: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 82E3A224: 4BF1B025  bl 0x82d55248
	ctx.lr = 0x82E3A228;
	sub_82D55248(ctx, base);
	// 82E3A228: 3960001C  li r11, 0x1c
	ctx.r[11].s64 = 28;
	// 82E3A22C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E3A230: 38A02000  li r5, 0x2000
	ctx.r[5].s64 = 8192;
	// 82E3A234: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E3A238: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E3A23C: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82E3A240: 4BF25C01  bl 0x82d5fe40
	ctx.lr = 0x82E3A244;
	sub_82D5FE40(ctx, base);
	// 82E3A244: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E3A248: 88BD0000  lbz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3A24C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A250: 4BF23CC1  bl 0x82d5df10
	ctx.lr = 0x82E3A254;
	sub_82D5DF10(ctx, base);
	// 82E3A254: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E3A258: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3A25C: 396B5A94  addi r11, r11, 0x5a94
	ctx.r[11].s64 = ctx.r[11].s64 + 23188;
	// 82E3A260: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E3A264: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3A268: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3A26C: 419A0030  beq cr6, 0x82e3a29c
	if ctx.cr[6].eq {
	pc = 0x82E3A29C; continue 'dispatch;
	}
	// 82E3A270: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E3A274: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E3A278: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E3A27C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3A280: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E3A284: 409A0018  bne cr6, 0x82e3a29c
	if !ctx.cr[6].eq {
	pc = 0x82E3A29C; continue 'dispatch;
	}
	// 82E3A288: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3A28C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E3A290: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3A294: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E3A298: 4E800421  bctrl
	ctx.lr = 0x82E3A29C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E3A29C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A2A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E3A2A4: 4BE6F1B8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3A2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3A2A8 size=8
    let mut pc: u32 = 0x82E3A2A8;
    'dispatch: loop {
        match pc {
            0x82E3A2A8 => {
    //   block [0x82E3A2A8..0x82E3A2B0)
	// 82E3A2A8: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82E3A2AC: 4BF241CC  b 0x82d5e478
	sub_82D5E478(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3A2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3A2B0 size=156
    let mut pc: u32 = 0x82E3A2B0;
    'dispatch: loop {
        match pc {
            0x82E3A2B0 => {
    //   block [0x82E3A2B0..0x82E3A34C)
	// 82E3A2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3A2B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E3A2B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E3A2BC: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3A2C0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82E3A2C4: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82E3A2C8: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82E3A2CC: 392B0030  addi r9, r11, 0x30
	ctx.r[9].s64 = ctx.r[11].s64 + 48;
	// 82E3A2D0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3A350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3A350 size=176
    let mut pc: u32 = 0x82E3A350;
    'dispatch: loop {
        match pc {
            0x82E3A350 => {
    //   block [0x82E3A350..0x82E3A400)
	// 82E3A350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3A354: 4BE6F0B1  bl 0x82ca9404
	ctx.lr = 0x82E3A358;
	sub_82CA93D0(ctx, base);
	// 82E3A358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3A35C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82E3A360: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3A364: 83DB0004  lwz r30, 4(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3A368: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E3A36C: 4BF23E2D  bl 0x82d5e198
	ctx.lr = 0x82E3A370;
	sub_82D5E198(ctx, base);
	// 82E3A370: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E3A374: 4099002C  ble cr6, 0x82e3a3a0
	if !ctx.cr[6].gt {
	pc = 0x82E3A3A0; continue 'dispatch;
	}
	// 82E3A378: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E3A37C: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3A380: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82E3A384: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A388: 7C8BEA14  add r4, r11, r29
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82E3A38C: 4BF240ED  bl 0x82d5e478
	ctx.lr = 0x82E3A390;
	sub_82D5E478(ctx, base);
	// 82E3A390: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82E3A394: 3BBD0010  addi r29, r29, 0x10
	ctx.r[29].s64 = ctx.r[29].s64 + 16;
	// 82E3A398: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E3A39C: 409AFFE0  bne cr6, 0x82e3a37c
	if !ctx.cr[6].eq {
	pc = 0x82E3A37C; continue 'dispatch;
	}
	// 82E3A3A0: 83BB0010  lwz r29, 0x10(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E3A3A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A3A8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E3A3AC: 4BF23DED  bl 0x82d5e198
	ctx.lr = 0x82E3A3B0;
	sub_82D5E198(ctx, base);
	// 82E3A3B0: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82E3A3B4: 40990044  ble cr6, 0x82e3a3f8
	if !ctx.cr[6].gt {
	pc = 0x82E3A3F8; continue 'dispatch;
	}
	// 82E3A3B8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E3A3BC: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3A3C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A3C4: 7FCBE214  add r30, r11, r28
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82E3A3C8: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3A3CC: 4BF23DCD  bl 0x82d5e198
	ctx.lr = 0x82E3A3D0;
	sub_82D5E198(ctx, base);
	// 82E3A3D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A3D4: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3A3D8: 4BF23DC1  bl 0x82d5e198
	ctx.lr = 0x82E3A3DC;
	sub_82D5E198(ctx, base);
	// 82E3A3DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A3E0: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3A3E4: 4BF23DB5  bl 0x82d5e198
	ctx.lr = 0x82E3A3E8;
	sub_82D5E198(ctx, base);
	// 82E3A3E8: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 82E3A3EC: 3B9C000C  addi r28, r28, 0xc
	ctx.r[28].s64 = ctx.r[28].s64 + 12;
	// 82E3A3F0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E3A3F4: 409AFFC8  bne cr6, 0x82e3a3bc
	if !ctx.cr[6].eq {
	pc = 0x82E3A3BC; continue 'dispatch;
	}
	// 82E3A3F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E3A3FC: 4BE6F058  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3A400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3A400 size=32
    let mut pc: u32 = 0x82E3A400;
    'dispatch: loop {
        match pc {
            0x82E3A400 => {
    //   block [0x82E3A400..0x82E3A420)
	// 82E3A400: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3A404: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E3A408: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E3A40C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E3A410: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E3A414: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E3A418: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82E3A41C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3A420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3A420 size=180
    let mut pc: u32 = 0x82E3A420;
    'dispatch: loop {
        match pc {
            0x82E3A420 => {
    //   block [0x82E3A420..0x82E3A470)
	// 82E3A420: 81430050  lwz r10, 0x50(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3A424: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E3A428: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E3A42C: 2B0A0008  cmplwi cr6, r10, 8
	ctx.cr[6].compare_u32(ctx.r[10].u32, 8 as u32, &mut ctx.xer);
	// 82E3A430: 4199009C  bgt cr6, 0x82e3a4cc
	if ctx.cr[6].gt {
	pc = 0x82E3A4CC; continue 'dispatch;
	}
	// 82E3A434: 3D8082E4  lis r12, -0x7d1c
	ctx.r[12].s64 = -2098987008;
	// 82E3A438: 398CA44C  addi r12, r12, -0x5bb4
	ctx.r[12].s64 = ctx.r[12].s64 + -23476;
	// 82E3A43C: 5540103A  slwi r0, r10, 2
	ctx.r[0].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82E3A440: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82E3A444: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82E3A448: 4E800420  bctr
	match ctx.r[10].u64 {
		0 => {
	pc = 0x82E3A498; continue 'dispatch;
		},
		1 => {
	pc = 0x82E3A4C8; continue 'dispatch;
		},
		2 => {
	pc = 0x82E3A4B0; continue 'dispatch;
		},
		3 => {
	pc = 0x82E3A4BC; continue 'dispatch;
		},
		4 => {
	pc = 0x82E3A498; continue 'dispatch;
		},
		5 => {
	pc = 0x82E3A470; continue 'dispatch;
		},
		6 => {
	pc = 0x82E3A4C8; continue 'dispatch;
		},
		7 => {
	pc = 0x82E3A4A4; continue 'dispatch;
		},
		8 => {
	pc = 0x82E3A4A4; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82E3A44C: 82E3A498  lwz r23, -0x5b68(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-23400 as u32) ) } as u64;
	// 82E3A450: 82E3A4C8  lwz r23, -0x5b38(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-23352 as u32) ) } as u64;
	// 82E3A454: 82E3A4B0  lwz r23, -0x5b50(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-23376 as u32) ) } as u64;
	// 82E3A458: 82E3A4BC  lwz r23, -0x5b44(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-23364 as u32) ) } as u64;
	// 82E3A45C: 82E3A498  lwz r23, -0x5b68(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-23400 as u32) ) } as u64;
	// 82E3A460: 82E3A470  lwz r23, -0x5b90(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-23440 as u32) ) } as u64;
	// 82E3A464: 82E3A4C8  lwz r23, -0x5b38(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-23352 as u32) ) } as u64;
	// 82E3A468: 82E3A4A4  lwz r23, -0x5b5c(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-23388 as u32) ) } as u64;
	// 82E3A46C: 82E3A4A4  lwz r23, -0x5b5c(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-23388 as u32) ) } as u64;
            }
            0x82E3A470 => {
    //   block [0x82E3A470..0x82E3A498)
	// 82E3A470: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3A474: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E3A478: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3A47C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E3A480: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E3A484: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E3A488: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E3A48C: 396B0025  addi r11, r11, 0x25
	ctx.r[11].s64 = ctx.r[11].s64 + 37;
	// 82E3A490: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E3A494: 4E800020  blr
	return;
            }
            0x82E3A498 => {
    //   block [0x82E3A498..0x82E3A4A4)
	// 82E3A498: 39600035  li r11, 0x35
	ctx.r[11].s64 = 53;
	// 82E3A49C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E3A4A0: 4E800020  blr
	return;
            }
            0x82E3A4A4 => {
    //   block [0x82E3A4A4..0x82E3A4B0)
	// 82E3A4A4: 39600041  li r11, 0x41
	ctx.r[11].s64 = 65;
	// 82E3A4A8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E3A4AC: 4E800020  blr
	return;
            }
            0x82E3A4B0 => {
    //   block [0x82E3A4B0..0x82E3A4BC)
	// 82E3A4B0: 39600019  li r11, 0x19
	ctx.r[11].s64 = 25;
	// 82E3A4B4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E3A4B8: 4E800020  blr
	return;
            }
            0x82E3A4BC => {
    //   block [0x82E3A4BC..0x82E3A4C8)
	// 82E3A4BC: 39600025  li r11, 0x25
	ctx.r[11].s64 = 37;
	// 82E3A4C0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E3A4C4: 4E800020  blr
	return;
            }
            0x82E3A4C8 => {
    //   block [0x82E3A4C8..0x82E3A4D4)
	// 82E3A4C8: 39600029  li r11, 0x29
	ctx.r[11].s64 = 41;
	// 82E3A4CC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E3A4D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3A4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E3A4D8 size=784
    let mut pc: u32 = 0x82E3A4D8;
    'dispatch: loop {
        match pc {
            0x82E3A4D8 => {
    //   block [0x82E3A4D8..0x82E3A540)
	// 82E3A4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3A4DC: 4BE6EF31  bl 0x82ca940c
	ctx.lr = 0x82E3A4E0;
	sub_82CA93D0(ctx, base);
	// 82E3A4E0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3A4E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E3A4E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3A4EC: 83BE0050  lwz r29, 0x50(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3A4F0: 7FA40774  extsb r4, r29
	ctx.r[4].s64 = ctx.r[29].s8 as i64;
	// 82E3A4F4: 4BF236B5  bl 0x82d5dba8
	ctx.lr = 0x82E3A4F8;
	sub_82D5DBA8(ctx, base);
	// 82E3A4F8: 397DFFFF  addi r11, r29, -1
	ctx.r[11].s64 = ctx.r[29].s64 + -1;
	// 82E3A4FC: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 82E3A500: 419902E0  bgt cr6, 0x82e3a7e0
	if ctx.cr[6].gt {
	pc = 0x82E3A7E0; continue 'dispatch;
	}
	// 82E3A504: 3D8082E4  lis r12, -0x7d1c
	ctx.r[12].s64 = -2098987008;
	// 82E3A508: 398CA51C  addi r12, r12, -0x5ae4
	ctx.r[12].s64 = ctx.r[12].s64 + -23268;
	// 82E3A50C: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82E3A510: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82E3A514: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82E3A518: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82E3A560; continue 'dispatch;
		},
		1 => {
	pc = 0x82E3A610; continue 'dispatch;
		},
		2 => {
	pc = 0x82E3A634; continue 'dispatch;
		},
		3 => {
	pc = 0x82E3A66C; continue 'dispatch;
		},
		4 => {
	pc = 0x82E3A6E0; continue 'dispatch;
		},
		5 => {
	pc = 0x82E3A540; continue 'dispatch;
		},
		6 => {
	pc = 0x82E3A780; continue 'dispatch;
		},
		7 => {
	pc = 0x82E3A5B8; continue 'dispatch;
		},
		8 => {
	pc = 0x82E3A5B8; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82E3A51C: 82E3A560  lwz r23, -0x5aa0(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-23200 as u32) ) } as u64;
	// 82E3A520: 82E3A610  lwz r23, -0x59f0(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-23024 as u32) ) } as u64;
	// 82E3A524: 82E3A634  lwz r23, -0x59cc(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-22988 as u32) ) } as u64;
	// 82E3A528: 82E3A66C  lwz r23, -0x5994(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-22932 as u32) ) } as u64;
	// 82E3A52C: 82E3A6E0  lwz r23, -0x5920(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-22816 as u32) ) } as u64;
	// 82E3A530: 82E3A540  lwz r23, -0x5ac0(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-23232 as u32) ) } as u64;
	// 82E3A534: 82E3A780  lwz r23, -0x5880(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-22656 as u32) ) } as u64;
	// 82E3A538: 82E3A5B8  lwz r23, -0x5a48(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-23112 as u32) ) } as u64;
	// 82E3A53C: 82E3A5B8  lwz r23, -0x5a48(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-23112 as u32) ) } as u64;
            }
            0x82E3A540 => {
    //   block [0x82E3A540..0x82E3A560)
	// 82E3A540: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 82E3A544: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A548: 4BFFFD69  bl 0x82e3a2b0
	ctx.lr = 0x82E3A54C;
	sub_82E3A2B0(ctx, base);
	// 82E3A54C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A550: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3A554: 4BFFFDFD  bl 0x82e3a350
	ctx.lr = 0x82E3A558;
	sub_82E3A350(ctx, base);
	// 82E3A558: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82E3A55C: 4BE6EF00  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E3A560 => {
    //   block [0x82E3A560..0x82E3A5B8)
	// 82E3A560: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 82E3A564: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A568: 4BFFFD49  bl 0x82e3a2b0
	ctx.lr = 0x82E3A56C;
	sub_82E3A2B0(ctx, base);
	// 82E3A56C: 39600060  li r11, 0x60
	ctx.r[11].s64 = 96;
	// 82E3A570: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	pc = 0x82E3A5B8; continue 'dispatch;
            }
            0x82E3A5B8 => {
    //   block [0x82E3A5B8..0x82E3A610)
	// 82E3A5B8: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 82E3A5BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A5C0: 4BFFFCF1  bl 0x82e3a2b0
	ctx.lr = 0x82E3A5C4;
	sub_82E3A2B0(ctx, base);
	// 82E3A5C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A5C8: C03E0080  lfs f1, 0x80(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E3A5CC: 4BF23C8D  bl 0x82d5e258
	ctx.lr = 0x82E3A5D0;
	sub_82D5E258(ctx, base);
	// 82E3A5D0: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82E3A5D4: 389E0060  addi r4, r30, 0x60
	ctx.r[4].s64 = ctx.r[30].s64 + 96;
	// 82E3A5D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A5DC: 4BF23E9D  bl 0x82d5e478
	ctx.lr = 0x82E3A5E0;
	sub_82D5E478(ctx, base);
	// 82E3A5E0: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82E3A5E4: 389E0070  addi r4, r30, 0x70
	ctx.r[4].s64 = ctx.r[30].s64 + 112;
	// 82E3A5E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A5EC: 4BF23E8D  bl 0x82d5e478
	ctx.lr = 0x82E3A5F0;
	sub_82D5E478(ctx, base);
	// 82E3A5F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A5F4: 809E0084  lwz r4, 0x84(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E3A5F8: 4BF23BA1  bl 0x82d5e198
	ctx.lr = 0x82E3A5FC;
	sub_82D5E198(ctx, base);
	// 82E3A5FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A600: 809E0088  lwz r4, 0x88(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E3A604: 4BF23B95  bl 0x82d5e198
	ctx.lr = 0x82E3A608;
	sub_82D5E198(ctx, base);
	// 82E3A608: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82E3A60C: 4BE6EE50  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E3A610 => {
    //   block [0x82E3A610..0x82E3A634)
	// 82E3A610: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 82E3A614: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A618: 4BFFFC99  bl 0x82e3a2b0
	ctx.lr = 0x82E3A61C;
	sub_82E3A2B0(ctx, base);
	// 82E3A61C: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82E3A620: 389E0060  addi r4, r30, 0x60
	ctx.r[4].s64 = ctx.r[30].s64 + 96;
	// 82E3A624: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A628: 4BF23E51  bl 0x82d5e478
	ctx.lr = 0x82E3A62C;
	sub_82D5E478(ctx, base);
	// 82E3A62C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82E3A630: 4BE6EE2C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E3A634 => {
    //   block [0x82E3A634..0x82E3A66C)
	// 82E3A634: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E3A638: 48003FF1  bl 0x82e3e628
	ctx.lr = 0x82E3A63C;
	sub_82E3E628(ctx, base);
	// 82E3A63C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E3A640: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82E3A644: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A648: 4BF23E31  bl 0x82d5e478
	ctx.lr = 0x82E3A64C;
	sub_82D5E478(ctx, base);
	// 82E3A64C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E3A650: 48003FE1  bl 0x82e3e630
	ctx.lr = 0x82E3A654;
	sub_82E3E630(ctx, base);
	// 82E3A654: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E3A658: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82E3A65C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A660: 4BF23E19  bl 0x82d5e478
	ctx.lr = 0x82E3A664;
	sub_82D5E478(ctx, base);
	// 82E3A664: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82E3A668: 4BE6EDF4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E3A66C => {
    //   block [0x82E3A66C..0x82E3A6E0)
	// 82E3A66C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E3A670: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E3A674: 4800387D  bl 0x82e3def0
	ctx.lr = 0x82E3A678;
	sub_82E3DEF0(ctx, base);
	// 82E3A678: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E3A67C: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82E3A680: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A684: 4BF23DF5  bl 0x82d5e478
	ctx.lr = 0x82E3A688;
	sub_82D5E478(ctx, base);
	// 82E3A688: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E3A68C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E3A690: 48003879  bl 0x82e3df08
	ctx.lr = 0x82E3A694;
	sub_82E3DF08(ctx, base);
	// 82E3A694: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E3A698: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82E3A69C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A6A0: 4BF23DD9  bl 0x82d5e478
	ctx.lr = 0x82E3A6A4;
	sub_82D5E478(ctx, base);
	// 82E3A6A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E3A6A8: 48003881  bl 0x82e3df28
	ctx.lr = 0x82E3A6AC;
	sub_82E3DF28(ctx, base);
	// 82E3A6AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A6B0: 4BF23BA9  bl 0x82d5e258
	ctx.lr = 0x82E3A6B4;
	sub_82D5E258(ctx, base);
	// 82E3A6B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E3A6B8: 48003869  bl 0x82e3df20
	ctx.lr = 0x82E3A6BC;
	sub_82E3DF20(ctx, base);
	// 82E3A6BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A6C0: 4BF23B99  bl 0x82d5e258
	ctx.lr = 0x82E3A6C4;
	sub_82D5E258(ctx, base);
	// 82E3A6C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E3A6C8: 48003869  bl 0x82e3df30
	ctx.lr = 0x82E3A6CC;
	sub_82E3DF30(ctx, base);
	// 82E3A6CC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E3A6D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A6D4: 4BF23AC5  bl 0x82d5e198
	ctx.lr = 0x82E3A6D8;
	sub_82D5E198(ctx, base);
	// 82E3A6D8: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82E3A6DC: 4BE6ED80  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E3A6E0 => {
    //   block [0x82E3A6E0..0x82E3A780)
	// 82E3A6E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E3A6E4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E3A6E8: 48003221  bl 0x82e3d908
	ctx.lr = 0x82E3A6EC;
	sub_82E3D908(ctx, base);
	// 82E3A6EC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E3A6F0: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82E3A6F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A6F8: 4BF23D81  bl 0x82d5e478
	ctx.lr = 0x82E3A6FC;
	sub_82D5E478(ctx, base);
	// 82E3A6FC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E3A700: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E3A704: 480031B5  bl 0x82e3d8b8
	ctx.lr = 0x82E3A708;
	sub_82E3D8B8(ctx, base);
	// 82E3A708: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E3A70C: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82E3A710: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A714: 4BF23D65  bl 0x82d5e478
	ctx.lr = 0x82E3A718;
	sub_82D5E478(ctx, base);
	// 82E3A718: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E3A71C: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82E3A720: 480031B1  bl 0x82e3d8d0
	ctx.lr = 0x82E3A724;
	sub_82E3D8D0(ctx, base);
	// 82E3A724: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E3A728: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82E3A72C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A730: 4BF23D49  bl 0x82d5e478
	ctx.lr = 0x82E3A734;
	sub_82D5E478(ctx, base);
	// 82E3A734: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E3A738: 480031C1  bl 0x82e3d8f8
	ctx.lr = 0x82E3A73C;
	sub_82E3D8F8(ctx, base);
	// 82E3A73C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A740: 4BF23B19  bl 0x82d5e258
	ctx.lr = 0x82E3A744;
	sub_82D5E258(ctx, base);
	// 82E3A744: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E3A748: 480031A1  bl 0x82e3d8e8
	ctx.lr = 0x82E3A74C;
	sub_82E3D8E8(ctx, base);
	// 82E3A74C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A750: 4BF23B09  bl 0x82d5e258
	ctx.lr = 0x82E3A754;
	sub_82D5E258(ctx, base);
	// 82E3A754: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E3A758: 48003199  bl 0x82e3d8f0
	ctx.lr = 0x82E3A75C;
	sub_82E3D8F0(ctx, base);
	// 82E3A75C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A760: 4BF23AF9  bl 0x82d5e258
	ctx.lr = 0x82E3A764;
	sub_82D5E258(ctx, base);
	// 82E3A764: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E3A768: 48003199  bl 0x82e3d900
	ctx.lr = 0x82E3A76C;
	sub_82E3D900(ctx, base);
	// 82E3A76C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E3A770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A774: 4BF23A25  bl 0x82d5e198
	ctx.lr = 0x82E3A778;
	sub_82D5E198(ctx, base);
	// 82E3A778: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82E3A77C: 4BE6ECE0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E3A780 => {
    //   block [0x82E3A780..0x82E3A7E8)
	// 82E3A780: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E3A784: 48002AB5  bl 0x82e3d238
	ctx.lr = 0x82E3A788;
	sub_82E3D238(ctx, base);
	// 82E3A788: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E3A78C: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82E3A790: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A794: 4BF23CE5  bl 0x82d5e478
	ctx.lr = 0x82E3A798;
	sub_82D5E478(ctx, base);
	// 82E3A798: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E3A79C: 48002A95  bl 0x82e3d230
	ctx.lr = 0x82E3A7A0;
	sub_82E3D230(ctx, base);
	// 82E3A7A0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E3A7A4: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82E3A7A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A7AC: 4BF23CCD  bl 0x82d5e478
	ctx.lr = 0x82E3A7B0;
	sub_82D5E478(ctx, base);
	// 82E3A7B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E3A7B4: 48002A8D  bl 0x82e3d240
	ctx.lr = 0x82E3A7B8;
	sub_82E3D240(ctx, base);
	// 82E3A7B8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E3A7BC: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82E3A7C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A7C4: 4BF23CB5  bl 0x82d5e478
	ctx.lr = 0x82E3A7C8;
	sub_82D5E478(ctx, base);
	// 82E3A7C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E3A7CC: 48002A7D  bl 0x82e3d248
	ctx.lr = 0x82E3A7D0;
	sub_82E3D248(ctx, base);
	// 82E3A7D0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E3A7D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A7D8: C02B0000  lfs f1, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E3A7DC: 4BF23A7D  bl 0x82d5e258
	ctx.lr = 0x82E3A7E0;
	sub_82D5E258(ctx, base);
	// 82E3A7E0: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82E3A7E4: 4BE6EC78  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3A7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3A7E8 size=100
    let mut pc: u32 = 0x82E3A7E8;
    'dispatch: loop {
        match pc {
            0x82E3A7E8 => {
    //   block [0x82E3A7E8..0x82E3A84C)
	// 82E3A7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3A7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E3A7F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E3A7F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E3A7F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3A7FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3A800: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E3A804: 4BF238B5  bl 0x82d5e0b8
	ctx.lr = 0x82E3A808;
	sub_82D5E0B8(ctx, base);
	// 82E3A808: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E3A80C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3A810: 419A0020  beq cr6, 0x82e3a830
	if ctx.cr[6].eq {
	pc = 0x82E3A830; continue 'dispatch;
	}
	// 82E3A814: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3A818: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E3A81C: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 82E3A820: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3A824: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E3A828: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3A82C: 4BF1AA9D  bl 0x82d552c8
	ctx.lr = 0x82E3A830;
	sub_82D552C8(ctx, base);
	// 82E3A830: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A834: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E3A838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3A83C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E3A840: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E3A844: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E3A848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3A850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3A850 size=76
    let mut pc: u32 = 0x82E3A850;
    'dispatch: loop {
        match pc {
            0x82E3A850 => {
    //   block [0x82E3A850..0x82E3A89C)
	// 82E3A850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3A854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E3A858: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E3A85C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3A860: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E3A864: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E3A868: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3A86C: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82E3A870: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3A874: 4829381D  bl 0x830ce090
	ctx.lr = 0x82E3A878;
	sub_830CE090(ctx, base);
	// 82E3A878: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E3A87C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A880: 396B5AA4  addi r11, r11, 0x5aa4
	ctx.r[11].s64 = ctx.r[11].s64 + 23204;
	// 82E3A884: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E3A888: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E3A88C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3A890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E3A894: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E3A898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3A8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3A8A0 size=132
    let mut pc: u32 = 0x82E3A8A0;
    'dispatch: loop {
        match pc {
            0x82E3A8A0 => {
    //   block [0x82E3A8A0..0x82E3A924)
	// 82E3A8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3A8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E3A8A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E3A8AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E3A8B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3A8B4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E3A8B8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3A8BC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3A8C0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E3A8C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3A8C8: 48293629  bl 0x830cdef0
	ctx.lr = 0x82E3A8CC;
	sub_830CDEF0(ctx, base);
	// 82E3A8CC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3A8D0: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3A8D4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3A8D8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E3A8DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A8E0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E3A8E4: 4829360D  bl 0x830cdef0
	ctx.lr = 0x82E3A8E8;
	sub_830CDEF0(ctx, base);
	// 82E3A8E8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3A8EC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3A8F0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3A8F4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E3A8F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A8FC: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E3A900: 482935F1  bl 0x830cdef0
	ctx.lr = 0x82E3A904;
	sub_830CDEF0(ctx, base);
	// 82E3A904: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3A908: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E3A90C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E3A910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3A914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E3A918: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E3A91C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E3A920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3A928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E3A928 size=64
    let mut pc: u32 = 0x82E3A928;
    'dispatch: loop {
        match pc {
            0x82E3A928 => {
    //   block [0x82E3A928..0x82E3A968)
	// 82E3A928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3A92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E3A930: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E3A934: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3A938: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82E3A93C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3A940: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E3A944: 482935AD  bl 0x830cdef0
	ctx.lr = 0x82E3A948;
	sub_830CDEF0(ctx, base);
	// 82E3A948: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E3A94C: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3A950: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E3A954: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E3A958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3A95C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E3A960: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E3A964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3A968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E3A968 size=128
    let mut pc: u32 = 0x82E3A968;
    'dispatch: loop {
        match pc {
            0x82E3A968 => {
    //   block [0x82E3A968..0x82E3A9E8)
	// 82E3A968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3A96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E3A970: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E3A974: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E3A978: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3A97C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E3A980: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82E3A984: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3A988: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E3A98C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3A990: 48293561  bl 0x830cdef0
	ctx.lr = 0x82E3A994;
	sub_830CDEF0(ctx, base);
	// 82E3A994: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E3A998: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82E3A99C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3A9A0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E3A9A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3A9A8: C00B0C14  lfs f0, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3A9AC: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82E3A9B0: 48293541  bl 0x830cdef0
	ctx.lr = 0x82E3A9B4;
	sub_830CDEF0(ctx, base);
	// 82E3A9B4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82E3A9B8: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 82E3A9BC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E3A9C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3A9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E3A9E8 size=392
    let mut pc: u32 = 0x82E3A9E8;
    'dispatch: loop {
        match pc {
            0x82E3A9E8 => {
    //   block [0x82E3A9E8..0x82E3AB70)
	// 82E3A9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3A9EC: 4BE6EA19  bl 0x82ca9404
	ctx.lr = 0x82E3A9F0;
	sub_82CA93D0(ctx, base);
	// 82E3A9F0: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82E3A9F4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3A9F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E3A9FC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3AA00: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3AA04: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E3AA08: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E3AA0C: 482934E5  bl 0x830cdef0
	ctx.lr = 0x82E3AA10;
	sub_830CDEF0(ctx, base);
	// 82E3AA10: 83810050  lwz r28, 0x50(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3AA14: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82E3AA18: 40990078  ble cr6, 0x82e3aa90
	if !ctx.cr[6].gt {
	pc = 0x82E3AA90; continue 'dispatch;
	}
	// 82E3AA1C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E3AA20: C3EB0C18  lfs f31, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E3AA24: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3AA28: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3AA2C: 3BBF0001  addi r29, r31, 1
	ctx.r[29].s64 = ctx.r[31].s64 + 1;
	// 82E3AA30: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3AA34: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82E3AA38: 40980024  bge cr6, 0x82e3aa5c
	if !ctx.cr[6].lt {
	pc = 0x82E3AA5C; continue 'dispatch;
	}
	// 82E3AA3C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E3AA40: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E3AA44: 41980008  blt cr6, 0x82e3aa4c
	if ctx.cr[6].lt {
	pc = 0x82E3AA4C; continue 'dispatch;
	}
	// 82E3AA48: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82E3AA4C: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82E3AA50: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E3AA54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E3AA58: 4BF1C4B9  bl 0x82d56f10
	ctx.lr = 0x82E3AA5C;
	sub_82D56F10(ctx, base);
	// 82E3AA5C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3AA60: 57EB2036  slwi r11, r31, 4
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E3AA64: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82E3AA68: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E3AA6C: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E3AA70: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3AA74: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E3AA78: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E3AA7C: 48293475  bl 0x830cdef0
	ctx.lr = 0x82E3AA80;
	sub_830CDEF0(ctx, base);
	// 82E3AA80: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 82E3AA84: D3FF000C  stfs f31, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E3AA88: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82E3AA8C: 409AFF98  bne cr6, 0x82e3aa24
	if !ctx.cr[6].eq {
	pc = 0x82E3AA24; continue 'dispatch;
	}
	// 82E3AA90: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3AA94: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3AA98: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E3AA9C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E3AAA0: 48293451  bl 0x830cdef0
	ctx.lr = 0x82E3AAA4;
	sub_830CDEF0(ctx, base);
	// 82E3AAA4: 83810050  lwz r28, 0x50(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3AAA8: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82E3AAAC: 409900B8  ble cr6, 0x82e3ab64
	if !ctx.cr[6].gt {
	pc = 0x82E3AB64; continue 'dispatch;
	}
	// 82E3AAB0: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 82E3AAB4: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3AAB8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3AABC: 3BFD0001  addi r31, r29, 1
	ctx.r[31].s64 = ctx.r[29].s64 + 1;
	// 82E3AAC0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3AAC4: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82E3AAC8: 40980024  bge cr6, 0x82e3aaec
	if !ctx.cr[6].lt {
	pc = 0x82E3AAEC; continue 'dispatch;
	}
	// 82E3AACC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E3AAD0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E3AAD4: 41980008  blt cr6, 0x82e3aadc
	if ctx.cr[6].lt {
	pc = 0x82E3AADC; continue 'dispatch;
	}
	// 82E3AAD8: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E3AADC: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 82E3AAE0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E3AAE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E3AAE8: 4BF1C429  bl 0x82d56f10
	ctx.lr = 0x82E3AAEC;
	sub_82D56F10(ctx, base);
	// 82E3AAEC: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E3AAF0: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3AAF4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3AAF8: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82E3AAFC: 7D7D5A14  add r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82E3AB00: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3AB04: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E3AB08: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E3AB0C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E3AB10: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E3AB14: 482933DD  bl 0x830cdef0
	ctx.lr = 0x82E3AB18;
	sub_830CDEF0(ctx, base);
	// 82E3AB18: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3AB1C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3AB20: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3AB24: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82E3AB28: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E3AB2C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E3AB30: 482933C1  bl 0x830cdef0
	ctx.lr = 0x82E3AB34;
	sub_830CDEF0(ctx, base);
	// 82E3AB34: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E3AB38: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3AB3C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3AB40: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E3AB44: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E3AB48: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E3AB4C: 482933A5  bl 0x830cdef0
	ctx.lr = 0x82E3AB50;
	sub_830CDEF0(ctx, base);
	// 82E3AB50: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E3AB54: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 82E3AB58: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82E3AB5C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E3AB60: 409AFF54  bne cr6, 0x82e3aab4
	if !ctx.cr[6].eq {
	pc = 0x82E3AAB4; continue 'dispatch;
	}
	// 82E3AB64: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E3AB68: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82E3AB6C: 4BE6E8E8  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3AB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E3AB70 size=2056
    let mut pc: u32 = 0x82E3AB70;
    'dispatch: loop {
        match pc {
            0x82E3AB70 => {
    //   block [0x82E3AB70..0x82E3ABE8)
	// 82E3AB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3AB74: 4BE6E891  bl 0x82ca9404
	ctx.lr = 0x82E3AB78;
	sub_82CA93D0(ctx, base);
	// 82E3AB78: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82E3AB7C: 9421FDF0  stwu r1, -0x210(r1)
	ea = ctx.r[1].u32.wrapping_add(-528 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3AB80: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E3AB84: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3AB88: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E3AB8C: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 82E3AB90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3AB94: 4829335D  bl 0x830cdef0
	ctx.lr = 0x82E3AB98;
	sub_830CDEF0(ctx, base);
	// 82E3AB98: 8961005C  lbz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E3AB9C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82E3ABA0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E3ABA4: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 82E3ABA8: 419907C4  bgt cr6, 0x82e3b36c
	if ctx.cr[6].gt {
	pc = 0x82E3B36C; continue 'dispatch;
	}
	// 82E3ABAC: 3D8082E4  lis r12, -0x7d1c
	ctx.r[12].s64 = -2098987008;
	// 82E3ABB0: 398CABC4  addi r12, r12, -0x543c
	ctx.r[12].s64 = ctx.r[12].s64 + -21564;
	// 82E3ABB4: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82E3ABB8: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82E3ABBC: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82E3ABC0: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82E3ACE0; continue 'dispatch;
		},
		1 => {
	pc = 0x82E3AFF0; continue 'dispatch;
		},
		2 => {
	pc = 0x82E3B094; continue 'dispatch;
		},
		3 => {
	pc = 0x82E3B108; continue 'dispatch;
		},
		4 => {
	pc = 0x82E3B1C4; continue 'dispatch;
		},
		5 => {
	pc = 0x82E3ABE8; continue 'dispatch;
		},
		6 => {
	pc = 0x82E3B2B4; continue 'dispatch;
		},
		7 => {
	pc = 0x82E3ADE0; continue 'dispatch;
		},
		8 => {
	pc = 0x82E3AEE8; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82E3ABC4: 82E3ACE0  lwz r23, -0x5320(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-21280 as u32) ) } as u64;
	// 82E3ABC8: 82E3AFF0  lwz r23, -0x5010(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-20496 as u32) ) } as u64;
	// 82E3ABCC: 82E3B094  lwz r23, -0x4f6c(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-20332 as u32) ) } as u64;
	// 82E3ABD0: 82E3B108  lwz r23, -0x4ef8(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-20216 as u32) ) } as u64;
	// 82E3ABD4: 82E3B1C4  lwz r23, -0x4e3c(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-20028 as u32) ) } as u64;
	// 82E3ABD8: 82E3ABE8  lwz r23, -0x5418(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-21528 as u32) ) } as u64;
	// 82E3ABDC: 82E3B2B4  lwz r23, -0x4d4c(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-19788 as u32) ) } as u64;
	// 82E3ABE0: 82E3ADE0  lwz r23, -0x5220(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-21024 as u32) ) } as u64;
	// 82E3ABE4: 82E3AEE8  lwz r23, -0x5118(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-20760 as u32) ) } as u64;
            }
            0x82E3ABE8 => {
    //   block [0x82E3ABE8..0x82E3ACE0)
	// 82E3ABE8: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82E3ABEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3ABF0: 4BFFFD79  bl 0x82e3a968
	ctx.lr = 0x82E3ABF4;
	sub_82E3A968(ctx, base);
	// 82E3ABF4: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3ABF8: 3B800004  li r28, 4
	ctx.r[28].s64 = 4;
	// 82E3ABFC: 7C7CD82E  lwzx r3, r28, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82E3AC00: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3AC04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3AC08: 419A001C  beq cr6, 0x82e3ac24
	if ctx.cr[6].eq {
	pc = 0x82E3AC24; continue 'dispatch;
	}
	// 82E3AC0C: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E3AC10: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E3AC14: 91430054  stw r10, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E3AC18: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3AC1C: 91430050  stw r10, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E3AC20: 48000010  b 0x82e3ac30
	pc = 0x82E3AC30; continue 'dispatch;
	// 82E3AC24: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82E3AC28: 4BF1A429  bl 0x82d55050
	ctx.lr = 0x82E3AC2C;
	sub_82D55050(ctx, base);
	// 82E3AC2C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E3AC30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3AC34: 419A002C  beq cr6, 0x82e3ac60
	if ctx.cr[6].eq {
	pc = 0x82E3AC60; continue 'dispatch;
	}
	// 82E3AC38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E3AC3C: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82E3AC40: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82E3AC44: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E3AC48: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E3AC4C: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E3AC50: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E3AC54: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82E3AC58: 912B0014  stw r9, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82E3AC5C: 48000008  b 0x82e3ac64
	pc = 0x82E3AC64; continue 'dispatch;
	// 82E3AC60: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E3AC64: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E3AC68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3AC6C: 4BFFFD7D  bl 0x82e3a9e8
	ctx.lr = 0x82E3AC70;
	sub_82E3A9E8(ctx, base);
	// 82E3AC70: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E3AC74: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 82E3AC78: 7C7CD82E  lwzx r3, r28, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82E3AC7C: 4BF1A5CD  bl 0x82d55248
	ctx.lr = 0x82E3AC80;
	sub_82D55248(ctx, base);
	// 82E3AC80: 39600060  li r11, 0x60
	ctx.r[11].s64 = 96;
	// 82E3AC84: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E3AC88: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E3AC8C: 4800945D  bl 0x82e440e8
	ctx.lr = 0x82E3AC90;
	sub_82E440E8(ctx, base);
	// 82E3AC90: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82E3AC94: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E3AC98: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82E3AC9C: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82E3ACA0: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
            }
            0x82E3ACE0 => {
    //   block [0x82E3ACE0..0x82E3ADE0)
	// 82E3ACE0: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82E3ACE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3ACE8: 4BFFFC81  bl 0x82e3a968
	ctx.lr = 0x82E3ACEC;
	sub_82E3A968(ctx, base);
	// 82E3ACEC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3ACF0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3ACF4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E3ACF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3ACFC: 482931F5  bl 0x830cdef0
	ctx.lr = 0x82E3AD00;
	sub_830CDEF0(ctx, base);
	// 82E3AD00: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82E3AD04: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3AD08: 38810110  addi r4, r1, 0x110
	ctx.r[4].s64 = ctx.r[1].s64 + 272;
	// 82E3AD0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3AD10: 482931E1  bl 0x830cdef0
	ctx.lr = 0x82E3AD14;
	sub_830CDEF0(ctx, base);
	// 82E3AD14: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E3AD18: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3AD1C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3AD20: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82E3AD24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3AD28: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3AD2C: D001011C  stfs f0, 0x11c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(284 as u32), tmp.u32 ) };
	// 82E3AD30: 39610110  addi r11, r1, 0x110
	ctx.r[11].s64 = ctx.r[1].s64 + 272;
	// 82E3AD34: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	pc = 0x82E3ADE0; continue 'dispatch;
            }
            0x82E3ADE0 => {
    //   block [0x82E3ADE0..0x82E3AEE8)
	// 82E3ADE0: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82E3ADE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3ADE8: 4BFFFB81  bl 0x82e3a968
	ctx.lr = 0x82E3ADEC;
	sub_82E3A968(ctx, base);
	// 82E3ADEC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3ADF0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3ADF4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82E3ADF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3ADFC: 482930F5  bl 0x830cdef0
	ctx.lr = 0x82E3AE00;
	sub_830CDEF0(ctx, base);
	// 82E3AE00: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82E3AE04: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3AE08: 388100F0  addi r4, r1, 0xf0
	ctx.r[4].s64 = ctx.r[1].s64 + 240;
	// 82E3AE0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3AE10: 482930E1  bl 0x830cdef0
	ctx.lr = 0x82E3AE14;
	sub_830CDEF0(ctx, base);
	// 82E3AE14: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E3AE18: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82E3AE1C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3AE20: 38810150  addi r4, r1, 0x150
	ctx.r[4].s64 = ctx.r[1].s64 + 336;
	// 82E3AE24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3AE28: C3EB0C18  lfs f31, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E3AE2C: D3E100FC  stfs f31, 0xfc(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 82E3AE30: 482930C1  bl 0x830cdef0
	ctx.lr = 0x82E3AE34;
	sub_830CDEF0(ctx, base);
	// 82E3AE34: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3AE38: D3E1015C  stfs f31, 0x15c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(348 as u32), tmp.u32 ) };
	// 82E3AE3C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3AE40: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E3AE44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3AE48: 482930A9  bl 0x830cdef0
	ctx.lr = 0x82E3AE4C;
	sub_830CDEF0(ctx, base);
	// 82E3AE4C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3AE50: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3AE54: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E3AE58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3AE5C: 48293095  bl 0x830cdef0
	ctx.lr = 0x82E3AE60;
	sub_830CDEF0(ctx, base);
	// 82E3AE60: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3AE64: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E3AE68: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E3AE6C: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 82E3AE70: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3AE74: 4BF1A3D5  bl 0x82d55248
	ctx.lr = 0x82E3AE78;
	sub_82D55248(ctx, base);
	// 82E3AE78: 39600090  li r11, 0x90
	ctx.r[11].s64 = 144;
	// 82E3AE7C: C0210054  lfs f1, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E3AE80: 38A10150  addi r5, r1, 0x150
	ctx.r[5].s64 = ctx.r[1].s64 + 336;
	// 82E3AE84: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3AE88: 388100F0  addi r4, r1, 0xf0
	ctx.r[4].s64 = ctx.r[1].s64 + 240;
	// 82E3AE8C: 80E10058  lwz r7, 0x58(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E3AE90: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E3AE94: 48005B9D  bl 0x82e40a30
	ctx.lr = 0x82E3AE98;
	sub_82E40A30(ctx, base);
	// 82E3AE98: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82E3AE9C: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E3AEA0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82E3AEA4: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82E3AEA8: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	pc = 0x82E3AEE8; continue 'dispatch;
            }
            0x82E3AEE8 => {
    //   block [0x82E3AEE8..0x82E3AFF0)
	// 82E3AEE8: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82E3AEEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3AEF0: 4BFFFA79  bl 0x82e3a968
	ctx.lr = 0x82E3AEF4;
	sub_82E3A968(ctx, base);
	// 82E3AEF4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3AEF8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3AEFC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E3AF00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3AF04: 48292FED  bl 0x830cdef0
	ctx.lr = 0x82E3AF08;
	sub_830CDEF0(ctx, base);
	// 82E3AF08: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82E3AF0C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3AF10: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 82E3AF14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3AF18: 48292FD9  bl 0x830cdef0
	ctx.lr = 0x82E3AF1C;
	sub_830CDEF0(ctx, base);
	// 82E3AF1C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E3AF20: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82E3AF24: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3AF28: 38810190  addi r4, r1, 0x190
	ctx.r[4].s64 = ctx.r[1].s64 + 400;
	// 82E3AF2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3AF30: C3EB0C18  lfs f31, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E3AF34: D3E100CC  stfs f31, 0xcc(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), tmp.u32 ) };
	// 82E3AF38: 48292FB9  bl 0x830cdef0
	ctx.lr = 0x82E3AF3C;
	sub_830CDEF0(ctx, base);
	// 82E3AF3C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3AF40: D3E1019C  stfs f31, 0x19c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(412 as u32), tmp.u32 ) };
	// 82E3AF44: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3AF48: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E3AF4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3AF50: 48292FA1  bl 0x830cdef0
	ctx.lr = 0x82E3AF54;
	sub_830CDEF0(ctx, base);
	// 82E3AF54: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3AF58: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3AF5C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82E3AF60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3AF64: 48292F8D  bl 0x830cdef0
	ctx.lr = 0x82E3AF68;
	sub_830CDEF0(ctx, base);
	// 82E3AF68: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3AF6C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E3AF70: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E3AF74: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 82E3AF78: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3AF7C: 4BF1A2CD  bl 0x82d55248
	ctx.lr = 0x82E3AF80;
	sub_82D55248(ctx, base);
	// 82E3AF80: 39600090  li r11, 0x90
	ctx.r[11].s64 = 144;
	// 82E3AF84: C0210058  lfs f1, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E3AF88: 38A10190  addi r5, r1, 0x190
	ctx.r[5].s64 = ctx.r[1].s64 + 400;
	// 82E3AF8C: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E3AF90: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 82E3AF94: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3AF98: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E3AF9C: 48005235  bl 0x82e401d0
	ctx.lr = 0x82E3AFA0;
	sub_82E401D0(ctx, base);
	// 82E3AFA0: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82E3AFA4: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E3AFA8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82E3AFAC: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82E3AFB0: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	pc = 0x82E3AFF0; continue 'dispatch;
            }
            0x82E3AFF0 => {
    //   block [0x82E3AFF0..0x82E3B094)
	// 82E3AFF0: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82E3AFF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3AFF8: 4BFFF971  bl 0x82e3a968
	ctx.lr = 0x82E3AFFC;
	sub_82E3A968(ctx, base);
	// 82E3AFFC: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82E3B000: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3B004: 38810170  addi r4, r1, 0x170
	ctx.r[4].s64 = ctx.r[1].s64 + 368;
	// 82E3B008: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3B00C: 48292EE5  bl 0x830cdef0
	ctx.lr = 0x82E3B010;
	sub_830CDEF0(ctx, base);
	// 82E3B010: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E3B014: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3B018: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E3B01C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E3B020: 38800070  li r4, 0x70
	ctx.r[4].s64 = 112;
	// 82E3B024: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3B028: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E3B02C: D001017C  stfs f0, 0x17c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(380 as u32), tmp.u32 ) };
	// 82E3B030: 4BF1A219  bl 0x82d55248
	ctx.lr = 0x82E3B034;
	sub_82D55248(ctx, base);
	// 82E3B034: 39600070  li r11, 0x70
	ctx.r[11].s64 = 112;
	// 82E3B038: 38810170  addi r4, r1, 0x170
	ctx.r[4].s64 = ctx.r[1].s64 + 368;
	// 82E3B03C: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E3B040: 48004549  bl 0x82e3f588
	ctx.lr = 0x82E3B044;
	sub_82E3F588(ctx, base);
	// 82E3B044: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82E3B048: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E3B04C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82E3B050: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82E3B054: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	pc = 0x82E3B094; continue 'dispatch;
            }
            0x82E3B094 => {
    //   block [0x82E3B094..0x82E3B108)
	// 82E3B094: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82E3B098: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3B09C: 388101B0  addi r4, r1, 0x1b0
	ctx.r[4].s64 = ctx.r[1].s64 + 432;
	// 82E3B0A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3B0A4: 48292E4D  bl 0x830cdef0
	ctx.lr = 0x82E3B0A8;
	sub_830CDEF0(ctx, base);
	// 82E3B0A8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E3B0AC: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82E3B0B0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3B0B4: 38810130  addi r4, r1, 0x130
	ctx.r[4].s64 = ctx.r[1].s64 + 304;
	// 82E3B0B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3B0BC: C3EB0C18  lfs f31, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E3B0C0: D3E101BC  stfs f31, 0x1bc(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(444 as u32), tmp.u32 ) };
	// 82E3B0C4: 48292E2D  bl 0x830cdef0
	ctx.lr = 0x82E3B0C8;
	sub_830CDEF0(ctx, base);
	// 82E3B0C8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3B0CC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E3B0D0: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E3B0D4: D3E1013C  stfs f31, 0x13c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(316 as u32), tmp.u32 ) };
	// 82E3B0D8: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 82E3B0DC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3B0E0: 4BF1A169  bl 0x82d55248
	ctx.lr = 0x82E3B0E4;
	sub_82D55248(ctx, base);
	// 82E3B0E4: 39600080  li r11, 0x80
	ctx.r[11].s64 = 128;
	// 82E3B0E8: 38A10130  addi r5, r1, 0x130
	ctx.r[5].s64 = ctx.r[1].s64 + 304;
	// 82E3B0EC: 388101B0  addi r4, r1, 0x1b0
	ctx.r[4].s64 = ctx.r[1].s64 + 432;
	// 82E3B0F0: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E3B0F4: 48003565  bl 0x82e3e658
	ctx.lr = 0x82E3B0F8;
	sub_82E3E658(ctx, base);
	// 82E3B0F8: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E3B0FC: 38210210  addi r1, r1, 0x210
	ctx.r[1].s64 = ctx.r[1].s64 + 528;
	// 82E3B100: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82E3B104: 4BE6E350  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E3B108 => {
    //   block [0x82E3B108..0x82E3B1C4)
	// 82E3B108: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82E3B10C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3B110: 388100D0  addi r4, r1, 0xd0
	ctx.r[4].s64 = ctx.r[1].s64 + 208;
	// 82E3B114: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3B118: 48292DD9  bl 0x830cdef0
	ctx.lr = 0x82E3B11C;
	sub_830CDEF0(ctx, base);
	// 82E3B11C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E3B120: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82E3B124: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3B128: 388100E0  addi r4, r1, 0xe0
	ctx.r[4].s64 = ctx.r[1].s64 + 224;
	// 82E3B12C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3B130: C3EB0C18  lfs f31, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E3B134: D3E100DC  stfs f31, 0xdc(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(220 as u32), tmp.u32 ) };
	// 82E3B138: 48292DB9  bl 0x830cdef0
	ctx.lr = 0x82E3B13C;
	sub_830CDEF0(ctx, base);
	// 82E3B13C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3B140: D3E100EC  stfs f31, 0xec(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(236 as u32), tmp.u32 ) };
	// 82E3B144: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3B148: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82E3B14C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3B150: 48292DA1  bl 0x830cdef0
	ctx.lr = 0x82E3B154;
	sub_830CDEF0(ctx, base);
	// 82E3B154: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3B158: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3B15C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E3B160: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3B164: 48292D8D  bl 0x830cdef0
	ctx.lr = 0x82E3B168;
	sub_830CDEF0(ctx, base);
	// 82E3B168: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3B16C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3B170: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E3B174: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3B178: 48292D79  bl 0x830cdef0
	ctx.lr = 0x82E3B17C;
	sub_830CDEF0(ctx, base);
	// 82E3B17C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3B180: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E3B184: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E3B188: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 82E3B18C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3B190: 4BF1A0B9  bl 0x82d55248
	ctx.lr = 0x82E3B194;
	sub_82D55248(ctx, base);
	// 82E3B194: 39600090  li r11, 0x90
	ctx.r[11].s64 = 144;
	// 82E3B198: C0410058  lfs f2, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82E3B19C: 390100D0  addi r8, r1, 0xd0
	ctx.r[8].s64 = ctx.r[1].s64 + 208;
	// 82E3B1A0: C0210054  lfs f1, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E3B1A4: 38E100E0  addi r7, r1, 0xe0
	ctx.r[7].s64 = ctx.r[1].s64 + 224;
	// 82E3B1A8: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3B1AC: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E3B1B0: 48002DB1  bl 0x82e3df60
	ctx.lr = 0x82E3B1B4;
	sub_82E3DF60(ctx, base);
	// 82E3B1B4: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E3B1B8: 38210210  addi r1, r1, 0x210
	ctx.r[1].s64 = ctx.r[1].s64 + 528;
	// 82E3B1BC: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82E3B1C0: 4BE6E294  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E3B1C4 => {
    //   block [0x82E3B1C4..0x82E3B2B4)
	// 82E3B1C4: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82E3B1C8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3B1CC: 38810140  addi r4, r1, 0x140
	ctx.r[4].s64 = ctx.r[1].s64 + 320;
	// 82E3B1D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3B1D4: 48292D1D  bl 0x830cdef0
	ctx.lr = 0x82E3B1D8;
	sub_830CDEF0(ctx, base);
	// 82E3B1D8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E3B1DC: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82E3B1E0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3B1E4: 38810120  addi r4, r1, 0x120
	ctx.r[4].s64 = ctx.r[1].s64 + 288;
	// 82E3B1E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3B1EC: C3EB0C18  lfs f31, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E3B1F0: D3E1014C  stfs f31, 0x14c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(332 as u32), tmp.u32 ) };
	// 82E3B1F4: 48292CFD  bl 0x830cdef0
	ctx.lr = 0x82E3B1F8;
	sub_830CDEF0(ctx, base);
	// 82E3B1F8: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82E3B1FC: D3E1012C  stfs f31, 0x12c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(300 as u32), tmp.u32 ) };
	// 82E3B200: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3B204: 38810100  addi r4, r1, 0x100
	ctx.r[4].s64 = ctx.r[1].s64 + 256;
	// 82E3B208: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3B20C: 48292CE5  bl 0x830cdef0
	ctx.lr = 0x82E3B210;
	sub_830CDEF0(ctx, base);
	// 82E3B210: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3B214: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3B218: D3E1010C  stfs f31, 0x10c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(268 as u32), tmp.u32 ) };
	// 82E3B21C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E3B220: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3B224: 48292CCD  bl 0x830cdef0
	ctx.lr = 0x82E3B228;
	sub_830CDEF0(ctx, base);
	// 82E3B228: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3B22C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3B230: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E3B234: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3B238: 48292CB9  bl 0x830cdef0
	ctx.lr = 0x82E3B23C;
	sub_830CDEF0(ctx, base);
	// 82E3B23C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3B240: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3B244: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E3B248: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3B24C: 48292CA5  bl 0x830cdef0
	ctx.lr = 0x82E3B250;
	sub_830CDEF0(ctx, base);
	// 82E3B250: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3B254: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3B258: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82E3B25C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3B260: 48292C91  bl 0x830cdef0
	ctx.lr = 0x82E3B264;
	sub_830CDEF0(ctx, base);
	// 82E3B264: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3B268: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E3B26C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E3B270: 388000A0  li r4, 0xa0
	ctx.r[4].s64 = 160;
	// 82E3B274: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3B278: 4BF19FD1  bl 0x82d55248
	ctx.lr = 0x82E3B27C;
	sub_82D55248(ctx, base);
	// 82E3B27C: 396000A0  li r11, 0xa0
	ctx.r[11].s64 = 160;
	// 82E3B280: C0610058  lfs f3, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82E3B284: 38C10100  addi r6, r1, 0x100
	ctx.r[6].s64 = ctx.r[1].s64 + 256;
	// 82E3B288: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E3B28C: 38A10120  addi r5, r1, 0x120
	ctx.r[5].s64 = ctx.r[1].s64 + 288;
	// 82E3B290: C0410050  lfs f2, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82E3B294: 38810140  addi r4, r1, 0x140
	ctx.r[4].s64 = ctx.r[1].s64 + 320;
	// 82E3B298: C0210060  lfs f1, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E3B29C: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E3B2A0: 480026B9  bl 0x82e3d958
	ctx.lr = 0x82E3B2A4;
	sub_82E3D958(ctx, base);
	// 82E3B2A4: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E3B2A8: 38210210  addi r1, r1, 0x210
	ctx.r[1].s64 = ctx.r[1].s64 + 528;
	// 82E3B2AC: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82E3B2B0: 4BE6E1A4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E3B2B4 => {
    //   block [0x82E3B2B4..0x82E3B378)
	// 82E3B2B4: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82E3B2B8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3B2BC: 38810160  addi r4, r1, 0x160
	ctx.r[4].s64 = ctx.r[1].s64 + 352;
	// 82E3B2C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3B2C4: 48292C2D  bl 0x830cdef0
	ctx.lr = 0x82E3B2C8;
	sub_830CDEF0(ctx, base);
	// 82E3B2C8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E3B2CC: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82E3B2D0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3B2D4: 388101A0  addi r4, r1, 0x1a0
	ctx.r[4].s64 = ctx.r[1].s64 + 416;
	// 82E3B2D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3B2DC: C3EB0C18  lfs f31, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E3B2E0: D3E1016C  stfs f31, 0x16c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(364 as u32), tmp.u32 ) };
	// 82E3B2E4: 48292C0D  bl 0x830cdef0
	ctx.lr = 0x82E3B2E8;
	sub_830CDEF0(ctx, base);
	// 82E3B2E8: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82E3B2EC: D3E101AC  stfs f31, 0x1ac(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(428 as u32), tmp.u32 ) };
	// 82E3B2F0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3B2F4: 38810180  addi r4, r1, 0x180
	ctx.r[4].s64 = ctx.r[1].s64 + 384;
	// 82E3B2F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3B2FC: 48292BF5  bl 0x830cdef0
	ctx.lr = 0x82E3B300;
	sub_830CDEF0(ctx, base);
	// 82E3B300: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3B304: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3B308: D3E1018C  stfs f31, 0x18c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(396 as u32), tmp.u32 ) };
	// 82E3B30C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E3B310: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3B314: 48292BDD  bl 0x830cdef0
	ctx.lr = 0x82E3B318;
	sub_830CDEF0(ctx, base);
	// 82E3B318: C0010060  lfs f0, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3B31C: D00100B0  stfs f0, 0xb0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), tmp.u32 ) };
	// 82E3B320: 392100B0  addi r9, r1, 0xb0
	ctx.r[9].s64 = ctx.r[1].s64 + 176;
	// 82E3B324: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3B328: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E3B32C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E3B330: 388000A0  li r4, 0xa0
	ctx.r[4].s64 = 160;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3B378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3B378 size=100
    let mut pc: u32 = 0x82E3B378;
    'dispatch: loop {
        match pc {
            0x82E3B378 => {
    //   block [0x82E3B378..0x82E3B3DC)
	// 82E3B378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3B37C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E3B380: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E3B384: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E3B388: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3B38C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3B390: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E3B394: 48292E15  bl 0x830ce1a8
	ctx.lr = 0x82E3B398;
	sub_830CE1A8(ctx, base);
	// 82E3B398: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E3B39C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3B3A0: 419A0020  beq cr6, 0x82e3b3c0
	if ctx.cr[6].eq {
	pc = 0x82E3B3C0; continue 'dispatch;
	}
	// 82E3B3A4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3B3A8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E3B3AC: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 82E3B3B0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3B3B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E3B3B8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3B3BC: 4BF19F0D  bl 0x82d552c8
	ctx.lr = 0x82E3B3C0;
	sub_82D552C8(ctx, base);
	// 82E3B3C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3B3C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E3B3C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3B3CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E3B3D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E3B3D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E3B3D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3B3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3B3E0 size=24
    let mut pc: u32 = 0x82E3B3E0;
    'dispatch: loop {
        match pc {
            0x82E3B3E0 => {
    //   block [0x82E3B3E0..0x82E3B3F8)
	// 82E3B3E0: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82E3B3E4: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82E3B3E8: 396BC840  addi r11, r11, -0x37c0
	ctx.r[11].s64 = ctx.r[11].s64 + -14272;
	// 82E3B3EC: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E3B3F0: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E3B3F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3B3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3B3F8 size=172
    let mut pc: u32 = 0x82E3B3F8;
    'dispatch: loop {
        match pc {
            0x82E3B3F8 => {
    //   block [0x82E3B3F8..0x82E3B4A4)
	// 82E3B3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3B3FC: 4BE6E00D  bl 0x82ca9408
	ctx.lr = 0x82E3B400;
	sub_82CA93D0(ctx, base);
	// 82E3B400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3B404: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3B408: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E3B40C: 38800011  li r4, 0x11
	ctx.r[4].s64 = 17;
	// 82E3B410: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E3B414: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E3B418: 4BF22DB1  bl 0x82d5e1c8
	ctx.lr = 0x82E3B41C;
	sub_82D5E1C8(ctx, base);
	// 82E3B41C: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E3B420: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82E3B424: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E3B428: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3B42C: 419A0048  beq cr6, 0x82e3b474
	if ctx.cr[6].eq {
	pc = 0x82E3B474; continue 'dispatch;
	}
	// 82E3B430: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E3B434: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E3B438: 4BF1A6B9  bl 0x82d55af0
	ctx.lr = 0x82E3B43C;
	sub_82D55AF0(ctx, base);
	// 82E3B43C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3B440: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3B444: 419A0030  beq cr6, 0x82e3b474
	if ctx.cr[6].eq {
	pc = 0x82E3B474; continue 'dispatch;
	}
	// 82E3B448: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E3B44C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E3B450: 806B0048  lwz r3, 0x48(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E3B454: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3B458: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E3B45C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E3B460: 4E800421  bctrl
	ctx.lr = 0x82E3B464;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E3B464: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E3B468: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E3B46C: 409A0008  bne cr6, 0x82e3b474
	if !ctx.cr[6].eq {
	pc = 0x82E3B474; continue 'dispatch;
	}
	// 82E3B470: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82E3B474: 38800021  li r4, 0x21
	ctx.r[4].s64 = 33;
	// 82E3B478: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E3B47C: 4BF2276D  bl 0x82d5dbe8
	ctx.lr = 0x82E3B480;
	sub_82D5DBE8(ctx, base);
	// 82E3B480: 7B840020  clrldi r4, r28, 0x20
	ctx.r[4].u64 = ctx.r[28].u64 & 0x00000000FFFFFFFFu64;
	// 82E3B484: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E3B488: 4BF22DA1  bl 0x82d5e228
	ctx.lr = 0x82E3B48C;
	sub_82D5E228(ctx, base);
	// 82E3B48C: 7BC40020  clrldi r4, r30, 0x20
	ctx.r[4].u64 = ctx.r[30].u64 & 0x00000000FFFFFFFFu64;
	// 82E3B490: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E3B494: 4BF22D95  bl 0x82d5e228
	ctx.lr = 0x82E3B498;
	sub_82D5E228(ctx, base);
	// 82E3B498: 38600011  li r3, 0x11
	ctx.r[3].s64 = 17;
	// 82E3B49C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E3B4A0: 4BE6DFB8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3B4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3B4A8 size=212
    let mut pc: u32 = 0x82E3B4A8;
    'dispatch: loop {
        match pc {
            0x82E3B4A8 => {
    //   block [0x82E3B4A8..0x82E3B57C)
	// 82E3B4A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3B4AC: 4BE6DF5D  bl 0x82ca9408
	ctx.lr = 0x82E3B4B0;
	sub_82CA93D0(ctx, base);
	// 82E3B4B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3B4B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3B4B8: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E3B4BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3B4C0: 419A00B4  beq cr6, 0x82e3b574
	if ctx.cr[6].eq {
	pc = 0x82E3B574; continue 'dispatch;
	}
	// 82E3B4C4: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 82E3B4C8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3B4CC: 4BF22CFD  bl 0x82d5e1c8
	ctx.lr = 0x82E3B4D0;
	sub_82D5E1C8(ctx, base);
	// 82E3B4D0: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82E3B4D4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3B4D8: 4BF22711  bl 0x82d5dbe8
	ctx.lr = 0x82E3B4DC;
	sub_82D5DBE8(ctx, base);
	// 82E3B4DC: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82E3B4E0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3B4E4: 396B7634  addi r11, r11, 0x7634
	ctx.r[11].s64 = ctx.r[11].s64 + 30260;
	// 82E3B4E8: 79640020  clrldi r4, r11, 0x20
	ctx.r[4].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82E3B4EC: 4BF22D3D  bl 0x82d5e228
	ctx.lr = 0x82E3B4F0;
	sub_82D5E228(ctx, base);
	// 82E3B4F0: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 82E3B4F4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3B4F8: 3BCB7D58  addi r30, r11, 0x7d58
	ctx.r[30].s64 = ctx.r[11].s64 + 32088;
	// 82E3B4FC: 889E0000  lbz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3B500: 4BF226E9  bl 0x82d5dbe8
	ctx.lr = 0x82E3B504;
	sub_82D5DBE8(ctx, base);
	// 82E3B504: 889E0001  lbz r4, 1(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(1 as u32) ) } as u64;
	// 82E3B508: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3B50C: 4BF226DD  bl 0x82d5dbe8
	ctx.lr = 0x82E3B510;
	sub_82D5DBE8(ctx, base);
	// 82E3B510: 889E0002  lbz r4, 2(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 82E3B514: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3B518: 4BF226D1  bl 0x82d5dbe8
	ctx.lr = 0x82E3B51C;
	sub_82D5DBE8(ctx, base);
	// 82E3B51C: 889E0003  lbz r4, 3(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(3 as u32) ) } as u64;
	// 82E3B520: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3B524: 4BF226C5  bl 0x82d5dbe8
	ctx.lr = 0x82E3B528;
	sub_82D5DBE8(ctx, base);
	// 82E3B528: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E3B52C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E3B530: 3BCB0024  addi r30, r11, 0x24
	ctx.r[30].s64 = ctx.r[11].s64 + 36;
	// 82E3B534: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3B538: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3B53C: 40990038  ble cr6, 0x82e3b574
	if !ctx.cr[6].gt {
	pc = 0x82E3B574; continue 'dispatch;
	}
	// 82E3B540: 3B9FFFF8  addi r28, r31, -8
	ctx.r[28].s64 = ctx.r[31].s64 + -8;
	// 82E3B544: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E3B548: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3B54C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E3B550: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E3B554: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3B558: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3B55C: 4BFFFE9D  bl 0x82e3b3f8
	ctx.lr = 0x82E3B560;
	sub_82E3B3F8(ctx, base);
	// 82E3B560: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3B564: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E3B568: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82E3B56C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E3B570: 4198FFD8  blt cr6, 0x82e3b548
	if ctx.cr[6].lt {
	pc = 0x82E3B548; continue 'dispatch;
	}
	// 82E3B574: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E3B578: 4BE6DEE0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3B580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3B580 size=360
    let mut pc: u32 = 0x82E3B580;
    'dispatch: loop {
        match pc {
            0x82E3B580 => {
    //   block [0x82E3B580..0x82E3B63C)
	// 82E3B580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3B584: 4BE6DE81  bl 0x82ca9404
	ctx.lr = 0x82E3B588;
	sub_82CA93D0(ctx, base);
	// 82E3B588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3B58C: 3966FFFF  addi r11, r6, -1
	ctx.r[11].s64 = ctx.r[6].s64 + -1;
	// 82E3B590: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E3B594: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E3B598: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 82E3B59C: 2B0B001E  cmplwi cr6, r11, 0x1e
	ctx.cr[6].compare_u32(ctx.r[11].u32, 30 as u32, &mut ctx.xer);
	// 82E3B5A0: 41990130  bgt cr6, 0x82e3b6d0
	if ctx.cr[6].gt {
	pc = 0x82E3B6D0; continue 'dispatch;
	}
	// 82E3B5A4: 54BF083C  slwi r31, r5, 1
	ctx.r[31].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82E3B5A8: 3D8082E4  lis r12, -0x7d1c
	ctx.r[12].s64 = -2098987008;
	// 82E3B5AC: 398CB5C0  addi r12, r12, -0x4a40
	ctx.r[12].s64 = ctx.r[12].s64 + -19008;
	// 82E3B5B0: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82E3B5B4: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82E3B5B8: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82E3B5BC: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82E3B6DC; continue 'dispatch;
		},
		1 => {
	pc = 0x82E3B6DC; continue 'dispatch;
		},
		2 => {
	pc = 0x82E3B6DC; continue 'dispatch;
		},
		3 => {
	pc = 0x82E3B6DC; continue 'dispatch;
		},
		4 => {
	pc = 0x82E3B6DC; continue 'dispatch;
		},
		5 => {
	pc = 0x82E3B6DC; continue 'dispatch;
		},
		6 => {
	pc = 0x82E3B6DC; continue 'dispatch;
		},
		7 => {
	pc = 0x82E3B6DC; continue 'dispatch;
		},
		8 => {
	pc = 0x82E3B6DC; continue 'dispatch;
		},
		9 => {
	pc = 0x82E3B6DC; continue 'dispatch;
		},
		10 => {
	pc = 0x82E3B6DC; continue 'dispatch;
		},
		11 => {
	pc = 0x82E3B6DC; continue 'dispatch;
		},
		12 => {
	pc = 0x82E3B6DC; continue 'dispatch;
		},
		13 => {
	pc = 0x82E3B63C; continue 'dispatch;
		},
		14 => {
	pc = 0x82E3B63C; continue 'dispatch;
		},
		15 => {
	pc = 0x82E3B63C; continue 'dispatch;
		},
		16 => {
	pc = 0x82E3B63C; continue 'dispatch;
		},
		17 => {
	pc = 0x82E3B63C; continue 'dispatch;
		},
		18 => {
	pc = 0x82E3B6D0; continue 'dispatch;
		},
		19 => {
	pc = 0x82E3B6D0; continue 'dispatch;
		},
		20 => {
	pc = 0x82E3B6D0; continue 'dispatch;
		},
		21 => {
	pc = 0x82E3B654; continue 'dispatch;
		},
		22 => {
	pc = 0x82E3B6D0; continue 'dispatch;
		},
		23 => {
	pc = 0x82E3B6DC; continue 'dispatch;
		},
		24 => {
	pc = 0x82E3B67C; continue 'dispatch;
		},
		25 => {
	pc = 0x82E3B654; continue 'dispatch;
		},
		26 => {
	pc = 0x82E3B6D0; continue 'dispatch;
		},
		27 => {
	pc = 0x82E3B6D0; continue 'dispatch;
		},
		28 => {
	pc = 0x82E3B6D0; continue 'dispatch;
		},
		29 => {
	pc = 0x82E3B6DC; continue 'dispatch;
		},
		30 => {
	pc = 0x82E3B6DC; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82E3B5C0: 82E3B6DC  lwz r23, -0x4924(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18724 as u32) ) } as u64;
	// 82E3B5C4: 82E3B6DC  lwz r23, -0x4924(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18724 as u32) ) } as u64;
	// 82E3B5C8: 82E3B6DC  lwz r23, -0x4924(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18724 as u32) ) } as u64;
	// 82E3B5CC: 82E3B6DC  lwz r23, -0x4924(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18724 as u32) ) } as u64;
	// 82E3B5D0: 82E3B6DC  lwz r23, -0x4924(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18724 as u32) ) } as u64;
	// 82E3B5D4: 82E3B6DC  lwz r23, -0x4924(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18724 as u32) ) } as u64;
	// 82E3B5D8: 82E3B6DC  lwz r23, -0x4924(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18724 as u32) ) } as u64;
	// 82E3B5DC: 82E3B6DC  lwz r23, -0x4924(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18724 as u32) ) } as u64;
	// 82E3B5E0: 82E3B6DC  lwz r23, -0x4924(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18724 as u32) ) } as u64;
	// 82E3B5E4: 82E3B6DC  lwz r23, -0x4924(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18724 as u32) ) } as u64;
	// 82E3B5E8: 82E3B6DC  lwz r23, -0x4924(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18724 as u32) ) } as u64;
	// 82E3B5EC: 82E3B6DC  lwz r23, -0x4924(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18724 as u32) ) } as u64;
	// 82E3B5F0: 82E3B6DC  lwz r23, -0x4924(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18724 as u32) ) } as u64;
	// 82E3B5F4: 82E3B63C  lwz r23, -0x49c4(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18884 as u32) ) } as u64;
	// 82E3B5F8: 82E3B63C  lwz r23, -0x49c4(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18884 as u32) ) } as u64;
	// 82E3B5FC: 82E3B63C  lwz r23, -0x49c4(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18884 as u32) ) } as u64;
	// 82E3B600: 82E3B63C  lwz r23, -0x49c4(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18884 as u32) ) } as u64;
	// 82E3B604: 82E3B63C  lwz r23, -0x49c4(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18884 as u32) ) } as u64;
	// 82E3B608: 82E3B6D0  lwz r23, -0x4930(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18736 as u32) ) } as u64;
	// 82E3B60C: 82E3B6D0  lwz r23, -0x4930(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18736 as u32) ) } as u64;
	// 82E3B610: 82E3B6D0  lwz r23, -0x4930(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18736 as u32) ) } as u64;
	// 82E3B614: 82E3B654  lwz r23, -0x49ac(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18860 as u32) ) } as u64;
	// 82E3B618: 82E3B6D0  lwz r23, -0x4930(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18736 as u32) ) } as u64;
	// 82E3B61C: 82E3B6DC  lwz r23, -0x4924(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18724 as u32) ) } as u64;
	// 82E3B620: 82E3B67C  lwz r23, -0x4984(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18820 as u32) ) } as u64;
	// 82E3B624: 82E3B654  lwz r23, -0x49ac(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18860 as u32) ) } as u64;
	// 82E3B628: 82E3B6D0  lwz r23, -0x4930(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18736 as u32) ) } as u64;
	// 82E3B62C: 82E3B6D0  lwz r23, -0x4930(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18736 as u32) ) } as u64;
	// 82E3B630: 82E3B6D0  lwz r23, -0x4930(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18736 as u32) ) } as u64;
	// 82E3B634: 82E3B6DC  lwz r23, -0x4924(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18724 as u32) ) } as u64;
	// 82E3B638: 82E3B6DC  lwz r23, -0x4924(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-18724 as u32) ) } as u64;
            }
            0x82E3B63C => {
    //   block [0x82E3B63C..0x82E3B654)
	// 82E3B63C: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3B640: 3960000C  li r11, 0xc
	ctx.r[11].s64 = 12;
	// 82E3B644: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82E3B648: A14A0002  lhz r10, 2(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 82E3B64C: 554A203E  rotlwi r10, r10, 4
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(4)) as u64;
	// 82E3B650: 4800006C  b 0x82e3b6bc
	pc = 0x82E3B6BC; continue 'dispatch;
            }
            0x82E3B654 => {
    //   block [0x82E3B654..0x82E3B67C)
	// 82E3B654: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3B658: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E3B65C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E3B660: A36B0002  lhz r27, 2(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82E3B664: 4BF219BD  bl 0x82d5d020
	ctx.lr = 0x82E3B668;
	sub_82D5D020(ctx, base);
	// 82E3B668: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3B66C: 7D43D9D6  mullw r10, r3, r27
	ctx.r[10].s64 = (ctx.r[3].s32 as i64) * (ctx.r[27].s32 as i64);
	// 82E3B670: 897D000D  lbz r11, 0xd(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(13 as u32) ) } as u64;
	// 82E3B674: 7FCA4A14  add r30, r10, r9
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82E3B678: 48000048  b 0x82e3b6c0
	pc = 0x82E3B6C0; continue 'dispatch;
            }
            0x82E3B67C => {
    //   block [0x82E3B67C..0x82E3B6D0)
	// 82E3B67C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E3B680: 419A0050  beq cr6, 0x82e3b6d0
	if ctx.cr[6].eq {
	pc = 0x82E3B6D0; continue 'dispatch;
	}
	// 82E3B684: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3B688: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3B68C: 419A0044  beq cr6, 0x82e3b6d0
	if ctx.cr[6].eq {
	pc = 0x82E3B6D0; continue 'dispatch;
	}
	// 82E3B690: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E3B694: 4BF216C5  bl 0x82d5cd58
	ctx.lr = 0x82E3B698;
	sub_82D5CD58(ctx, base);
	// 82E3B698: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E3B69C: 419A0034  beq cr6, 0x82e3b6d0
	if ctx.cr[6].eq {
	pc = 0x82E3B6D0; continue 'dispatch;
	}
	// 82E3B6A0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3B6A4: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E3B6A8: A08B0002  lhz r4, 2(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82E3B6AC: 4BF1A2DD  bl 0x82d55988
	ctx.lr = 0x82E3B6B0;
	sub_82D55988(ctx, base);
	// 82E3B6B0: A1430012  lhz r10, 0x12(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(18 as u32) ) } as u64;
	// 82E3B6B4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E3B6B8: 8963000C  lbz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3B6BC: 7FCAF214  add r30, r10, r30
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82E3B6C0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E3B6C4: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82E3B6C8: 2B0B001E  cmplwi cr6, r11, 0x1e
	ctx.cr[6].compare_u32(ctx.r[11].u32, 30 as u32, &mut ctx.xer);
	// 82E3B6CC: 4099FEDC  ble cr6, 0x82e3b5a8
	if !ctx.cr[6].gt {
	pc = 0x82E3B5A8; continue 'dispatch;
	}
	pc = 0x82E3B6D0; continue 'dispatch;
            }
            0x82E3B6D0 => {
    //   block [0x82E3B6D0..0x82E3B6DC)
	// 82E3B6D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E3B6D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E3B6D8: 4BE6DD7C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            0x82E3B6DC => {
    //   block [0x82E3B6DC..0x82E3B6E8)
	// 82E3B6DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E3B6E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E3B6E4: 4BE6DD70  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3B6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3B6E8 size=192
    let mut pc: u32 = 0x82E3B6E8;
    'dispatch: loop {
        match pc {
            0x82E3B6E8 => {
    //   block [0x82E3B6E8..0x82E3B7A8)
	// 82E3B6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3B6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E3B6F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E3B6F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E3B6F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3B6FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3B700: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E3B704: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 82E3B708: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E3B70C: 4BF22ABD  bl 0x82d5e1c8
	ctx.lr = 0x82E3B710;
	sub_82D5E1C8(ctx, base);
	// 82E3B710: 38800022  li r4, 0x22
	ctx.r[4].s64 = 34;
	// 82E3B714: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E3B718: 4BF224D1  bl 0x82d5dbe8
	ctx.lr = 0x82E3B71C;
	sub_82D5DBE8(ctx, base);
	// 82E3B71C: 7BC40020  clrldi r4, r30, 0x20
	ctx.r[4].u64 = ctx.r[30].u64 & 0x00000000FFFFFFFFu64;
	// 82E3B720: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E3B724: 4BF22B05  bl 0x82d5e228
	ctx.lr = 0x82E3B728;
	sub_82D5E228(ctx, base);
	// 82E3B728: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E3B72C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E3B730: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E3B734: 40990058  ble cr6, 0x82e3b78c
	if !ctx.cr[6].gt {
	pc = 0x82E3B78C; continue 'dispatch;
	}
	// 82E3B738: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E3B73C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3B740: 7F08F040  cmplw cr6, r8, r30
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E3B744: 419A0018  beq cr6, 0x82e3b75c
	if ctx.cr[6].eq {
	pc = 0x82E3B75C; continue 'dispatch;
	}
	// 82E3B748: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E3B74C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82E3B750: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E3B754: 4198FFE8  blt cr6, 0x82e3b73c
	if ctx.cr[6].lt {
	pc = 0x82E3B73C; continue 'dispatch;
	}
	// 82E3B758: 48000034  b 0x82e3b78c
	pc = 0x82E3B78C; continue 'dispatch;
	// 82E3B75C: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E3B760: 55691838  slwi r9, r11, 3
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E3B764: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E3B768: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E3B76C: 7D095A14  add r8, r9, r11
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82E3B770: 55491838  slwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E3B774: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82E3B778: 915F0024  stw r10, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 82E3B77C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3B780: 91480000  stw r10, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E3B784: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3B788: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E3B78C: 38600009  li r3, 9
	ctx.r[3].s64 = 9;
	// 82E3B790: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E3B794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3B798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E3B79C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E3B7A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E3B7A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3B7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3B7A8 size=32
    let mut pc: u32 = 0x82E3B7A8;
    'dispatch: loop {
        match pc {
            0x82E3B7A8 => {
    //   block [0x82E3B7A8..0x82E3B7C8)
	// 82E3B7A8: 7CAA0774  extsb r10, r5
	ctx.r[10].s64 = ctx.r[5].s8 as i64;
	// 82E3B7AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E3B7B0: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 82E3B7B4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E3B7B8: 419A0010  beq cr6, 0x82e3b7c8
	if ctx.cr[6].eq {
		sub_82E3B7C8(ctx, base);
		return;
	}
	// 82E3B7BC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82E3B7C0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E3B7C4: 4BFFFC34  b 0x82e3b3f8
	sub_82E3B3F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3B7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3B7C8 size=8
    let mut pc: u32 = 0x82E3B7C8;
    'dispatch: loop {
        match pc {
            0x82E3B7C8 => {
    //   block [0x82E3B7C8..0x82E3B7D0)
	// 82E3B7C8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E3B7CC: 4BFFFF1C  b 0x82e3b6e8
	sub_82E3B6E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3B7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3B7D0 size=192
    let mut pc: u32 = 0x82E3B7D0;
    'dispatch: loop {
        match pc {
            0x82E3B7D0 => {
    //   block [0x82E3B7D0..0x82E3B890)
	// 82E3B7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3B7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E3B7D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E3B7DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3B7E0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E3B7E4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E3B7E8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E3B7EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3B7F0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E3B7F4: 396B58F4  addi r11, r11, 0x58f4
	ctx.r[11].s64 = ctx.r[11].s64 + 22772;
	// 82E3B7F8: 394A5AE8  addi r10, r10, 0x5ae8
	ctx.r[10].s64 = ctx.r[10].s64 + 23272;
	// 82E3B7FC: 39295AC8  addi r9, r9, 0x5ac8
	ctx.r[9].s64 = ctx.r[9].s64 + 23240;
	// 82E3B800: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82E3B804: 3CC08000  lis r6, -0x8000
	ctx.r[6].s64 = -2147483648;
	// 82E3B808: B0FF0006  sth r7, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82E3B80C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E3B810: 98FF000C  stb r7, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u8 ) };
	// 82E3B814: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E3B818: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E3B81C: 911F0020  stw r8, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[8].u32 ) };
	// 82E3B820: 911F0024  stw r8, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[8].u32 ) };
	// 82E3B824: 90DF0028  stw r6, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[6].u32 ) };
	// 82E3B828: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3B82C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3B830: 40990010  ble cr6, 0x82e3b840
	if !ctx.cr[6].gt {
	pc = 0x82E3B840; continue 'dispatch;
	}
	// 82E3B834: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3B838: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3B83C: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3B840: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82E3B844: 911F002C  stw r8, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[8].u32 ) };
	// 82E3B848: 419A0030  beq cr6, 0x82e3b878
	if ctx.cr[6].eq {
	pc = 0x82E3B878; continue 'dispatch;
	}
	// 82E3B84C: A1680004  lhz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3B850: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3B854: 419A0010  beq cr6, 0x82e3b864
	if ctx.cr[6].eq {
	pc = 0x82E3B864; continue 'dispatch;
	}
	// 82E3B858: A1680006  lhz r11, 6(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E3B85C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E3B860: B1680006  sth r11, 6(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E3B864: 3D6082E4  lis r11, -0x7d1c
	ctx.r[11].s64 = -2098987008;
	// 82E3B868: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E3B86C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E3B870: 388BB7A8  addi r4, r11, -0x4858
	ctx.r[4].s64 = ctx.r[11].s64 + -18520;
	// 82E3B874: 4BFFB465  bl 0x82e36cd8
	ctx.lr = 0x82E3B878;
	sub_82E36CD8(ctx, base);
	// 82E3B878: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3B87C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E3B880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3B884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E3B888: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E3B88C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3B890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3B890 size=264
    let mut pc: u32 = 0x82E3B890;
    'dispatch: loop {
        match pc {
            0x82E3B890 => {
    //   block [0x82E3B890..0x82E3B998)
	// 82E3B890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3B894: 4BE6DB75  bl 0x82ca9408
	ctx.lr = 0x82E3B898;
	sub_82CA93D0(ctx, base);
	// 82E3B898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3B89C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3B8A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E3B8A4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E3B8A8: 394A5AE8  addi r10, r10, 0x5ae8
	ctx.r[10].s64 = ctx.r[10].s64 + 23272;
	// 82E3B8AC: 39295AC8  addi r9, r9, 0x5ac8
	ctx.r[9].s64 = ctx.r[9].s64 + 23240;
	// 82E3B8B0: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E3B8B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3B8B8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E3B8BC: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E3B8C0: 419A008C  beq cr6, 0x82e3b94c
	if ctx.cr[6].eq {
	pc = 0x82E3B94C; continue 'dispatch;
	}
	// 82E3B8C4: 3B8B0024  addi r28, r11, 0x24
	ctx.r[28].s64 = ctx.r[11].s64 + 36;
	// 82E3B8C8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E3B8CC: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3B8D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3B8D4: 4099002C  ble cr6, 0x82e3b900
	if !ctx.cr[6].gt {
	pc = 0x82E3B900; continue 'dispatch;
	}
	// 82E3B8D8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E3B8DC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3B8E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3B8E4: 7C9D582E  lwzx r4, r29, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3B8E8: 4BFFFE01  bl 0x82e3b6e8
	ctx.lr = 0x82E3B8EC;
	sub_82E3B6E8(ctx, base);
	// 82E3B8EC: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3B8F0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E3B8F4: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 82E3B8F8: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E3B8FC: 4198FFE0  blt cr6, 0x82e3b8dc
	if ctx.cr[6].lt {
	pc = 0x82E3B8DC; continue 'dispatch;
	}
	// 82E3B900: 3D6082E4  lis r11, -0x7d1c
	ctx.r[11].s64 = -2098987008;
	// 82E3B904: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E3B908: 388BB7A8  addi r4, r11, -0x4858
	ctx.r[4].s64 = ctx.r[11].s64 + -18520;
	// 82E3B90C: 4BFFB0C5  bl 0x82e369d0
	ctx.lr = 0x82E3B910;
	sub_82E369D0(ctx, base);
	// 82E3B910: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E3B914: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3B918: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3B91C: 419A0030  beq cr6, 0x82e3b94c
	if ctx.cr[6].eq {
	pc = 0x82E3B94C; continue 'dispatch;
	}
	// 82E3B920: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E3B924: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E3B928: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82E3B92C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3B930: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82E3B934: 409A0018  bne cr6, 0x82e3b94c
	if !ctx.cr[6].eq {
	pc = 0x82E3B94C; continue 'dispatch;
	}
	// 82E3B938: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3B93C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E3B940: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3B944: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E3B948: 4E800421  bctrl
	ctx.lr = 0x82E3B94C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E3B94C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E3B950: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3B954: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E3B958: 409A0020  bne cr6, 0x82e3b978
	if !ctx.cr[6].eq {
	pc = 0x82E3B978; continue 'dispatch;
	}
	// 82E3B95C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3B960: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E3B964: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E3B968: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E3B96C: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E3B970: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E3B974: 4BF19955  bl 0x82d552c8
	ctx.lr = 0x82E3B978;
	sub_82D552C8(ctx, base);
	// 82E3B978: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E3B97C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82E3B980: 396B58F4  addi r11, r11, 0x58f4
	ctx.r[11].s64 = ctx.r[11].s64 + 22772;
	// 82E3B984: 394A39E0  addi r10, r10, 0x39e0
	ctx.r[10].s64 = ctx.r[10].s64 + 14816;
	// 82E3B988: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E3B98C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E3B990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E3B994: 4BE6DAC4  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3B998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3B998 size=96
    let mut pc: u32 = 0x82E3B998;
    'dispatch: loop {
        match pc {
            0x82E3B998 => {
    //   block [0x82E3B998..0x82E3B9F8)
	// 82E3B998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3B99C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E3B9A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E3B9A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3B9A8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3B9AC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E3B9B0: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 82E3B9B4: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 82E3B9B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3B9BC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3B9C0: 4BF19889  bl 0x82d55248
	ctx.lr = 0x82E3B9C4;
	sub_82D55248(ctx, base);
	// 82E3B9C4: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 82E3B9C8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E3B9CC: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E3B9D0: 4BFFFE01  bl 0x82e3b7d0
	ctx.lr = 0x82E3B9D4;
	sub_82E3B7D0(ctx, base);
	// 82E3B9D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E3B9D8: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82E3B9DC: 409A0008  bne cr6, 0x82e3b9e4
	if !ctx.cr[6].eq {
	pc = 0x82E3B9E4; continue 'dispatch;
	}
	// 82E3B9E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E3B9E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E3B9E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3B9EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E3B9F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E3B9F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3B9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3B9F8 size=64
    let mut pc: u32 = 0x82E3B9F8;
    'dispatch: loop {
        match pc {
            0x82E3B9F8 => {
    //   block [0x82E3B9F8..0x82E3BA38)
	// 82E3B9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3B9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E3BA00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3BA04: 3D6082E4  lis r11, -0x7d1c
	ctx.r[11].s64 = -2098987008;
	// 82E3BA08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E3BA0C: 38ABB998  addi r5, r11, -0x4668
	ctx.r[5].s64 = ctx.r[11].s64 + -18024;
	// 82E3BA10: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E3BA14: 388A5AB0  addi r4, r10, 0x5ab0
	ctx.r[4].s64 = ctx.r[10].s64 + 23216;
	// 82E3BA18: 806BB068  lwz r3, -0x4f98(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20376 as u32) ) } as u64;
	// 82E3BA1C: 4BFFD27D  bl 0x82e38c98
	ctx.lr = 0x82E3BA20;
	sub_82E38C98(ctx, base);
	// 82E3BA20: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E3BA24: 906BB06C  stw r3, -0x4f94(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-20372 as u32), ctx.r[3].u32 ) };
	// 82E3BA28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E3BA2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3BA30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E3BA34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3BA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3BA38 size=912
    let mut pc: u32 = 0x82E3BA38;
    'dispatch: loop {
        match pc {
            0x82E3BA38 => {
    //   block [0x82E3BA38..0x82E3BDC8)
	// 82E3BA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3BA3C: 4BE6D9B5  bl 0x82ca93f0
	ctx.lr = 0x82E3BA40;
	sub_82CA93D0(ctx, base);
	// 82E3BA40: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3BA44: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E3BA48: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82E3BA4C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82E3BA50: 388B6A98  addi r4, r11, 0x6a98
	ctx.r[4].s64 = ctx.r[11].s64 + 27288;
	// 82E3BA54: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E3BA58: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E3BA5C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E3BA60: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82E3BA64: 4BF35C05  bl 0x82d71668
	ctx.lr = 0x82E3BA68;
	sub_82D71668(ctx, base);
	// 82E3BA68: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E3BA6C: 8178002C  lwz r11, 0x2c(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E3BA70: 3EC08000  lis r22, -0x8000
	ctx.r[22].s64 = -2147483648;
	// 82E3BA74: 7FB9EB78  mr r25, r29
	ctx.r[25].u64 = ctx.r[29].u64;
	// 82E3BA78: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82E3BA7C: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82E3BA80: 92C10060  stw r22, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[22].u32 ) };
	// 82E3BA84: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E3BA88: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3BA8C: 419A0048  beq cr6, 0x82e3bad4
	if ctx.cr[6].eq {
	pc = 0x82E3BAD4; continue 'dispatch;
	}
	// 82E3BA90: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E3BA94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E3BA98: 4BF1A059  bl 0x82d55af0
	ctx.lr = 0x82E3BA9C;
	sub_82D55AF0(ctx, base);
	// 82E3BA9C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3BAA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3BAA4: 419A0030  beq cr6, 0x82e3bad4
	if ctx.cr[6].eq {
	pc = 0x82E3BAD4; continue 'dispatch;
	}
	// 82E3BAA8: 8178002C  lwz r11, 0x2c(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E3BAAC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E3BAB0: 806B0048  lwz r3, 0x48(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E3BAB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3BAB8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E3BABC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E3BAC0: 4E800421  bctrl
	ctx.lr = 0x82E3BAC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E3BAC4: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82E3BAC8: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82E3BACC: 409A0008  bne cr6, 0x82e3bad4
	if !ctx.cr[6].eq {
	pc = 0x82E3BAD4; continue 'dispatch;
	}
	// 82E3BAD0: 7FB9EB78  mr r25, r29
	ctx.r[25].u64 = ctx.r[29].u64;
	// 82E3BAD4: 7F5E0774  extsb r30, r26
	ctx.r[30].s64 = ctx.r[26].s8 as i64;
	// 82E3BAD8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E3BADC: 419A00C0  beq cr6, 0x82e3bb9c
	if ctx.cr[6].eq {
	pc = 0x82E3BB9C; continue 'dispatch;
	}
	// 82E3BAE0: 813C0004  lwz r9, 4(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3BAE4: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E3BAE8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E3BAEC: 40990028  ble cr6, 0x82e3bb14
	if !ctx.cr[6].gt {
	pc = 0x82E3BB14; continue 'dispatch;
	}
	// 82E3BAF0: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3BAF4: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3BAF8: 7F08D840  cmplw cr6, r8, r27
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E3BAFC: 419A0018  beq cr6, 0x82e3bb14
	if ctx.cr[6].eq {
	pc = 0x82E3BB14; continue 'dispatch;
	}
	// 82E3BB00: 811C0004  lwz r8, 4(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3BB04: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E3BB08: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82E3BB0C: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E3BB10: 4198FFE4  blt cr6, 0x82e3baf4
	if ctx.cr[6].lt {
	pc = 0x82E3BAF4; continue 'dispatch;
	}
	// 82E3BB14: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E3BB18: 40980048  bge cr6, 0x82e3bb60
	if !ctx.cr[6].lt {
	pc = 0x82E3BB60; continue 'dispatch;
	}
	// 82E3BB1C: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E3BB20: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3BB24: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E3BB28: 409A002C  bne cr6, 0x82e3bb54
	if !ctx.cr[6].eq {
	pc = 0x82E3BB54; continue 'dispatch;
	}
	// 82E3BB2C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3BB30: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E3BB34: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3BB38: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E3BB3C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E3BB40: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E3BB44: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E3BB48: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E3BB4C: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E3BB50: 4BF19779  bl 0x82d552c8
	ctx.lr = 0x82E3BB54;
	sub_82D552C8(ctx, base);
	// 82E3BB54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E3BB58: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82E3BB5C: 4BE6D8E4  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
	// 82E3BB60: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3BB64: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3BB68: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E3BB6C: 409A0010  bne cr6, 0x82e3bb7c
	if !ctx.cr[6].eq {
	pc = 0x82E3BB7C; continue 'dispatch;
	}
	// 82E3BB70: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82E3BB74: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E3BB78: 4BF1B421  bl 0x82d56f98
	ctx.lr = 0x82E3BB7C;
	sub_82D56F98(ctx, base);
	// 82E3BB7C: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3BB80: 813C0000  lwz r9, 0(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3BB84: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E3BB88: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82E3BB8C: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82E3BB90: 911C0004  stw r8, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82E3BB94: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82E3BB98: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82E3BB9C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E3BBA0: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 82E3BBA4: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82E3BBA8: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 82E3BBAC: 3AEB3BA8  addi r23, r11, 0x3ba8
	ctx.r[23].s64 = ctx.r[11].s64 + 15272;
	// 82E3BBB0: 92C10070  stw r22, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[22].u32 ) };
	// 82E3BBB4: 39610068  addi r11, r1, 0x68
	ctx.r[11].s64 = ctx.r[1].s64 + 104;
	// 82E3BBB8: 93E1008C  stw r31, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[31].u32 ) };
	// 82E3BBBC: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E3BBC0: B3410086  sth r26, 0x86(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(134 as u32), ctx.r[26].u16 ) };
	// 82E3BBC4: 92E10080  stw r23, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[23].u32 ) };
	// 82E3BBC8: 91610088  stw r11, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 82E3BBCC: 93410090  stw r26, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[26].u32 ) };
	// 82E3BBD0: 4BF1C9C9  bl 0x82d58598
	ctx.lr = 0x82E3BBD4;
	sub_82D58598(ctx, base);
	// 82E3BBD4: 39010058  addi r8, r1, 0x58
	ctx.r[8].s64 = ctx.r[1].s64 + 88;
	// 82E3BBD8: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82E3BBDC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82E3BBE0: 7B650020  clrldi r5, r27, 0x20
	ctx.r[5].u64 = ctx.r[27].u64 & 0x00000000FFFFFFFFu64;
	// 82E3BBE4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82E3BBE8: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E3BBEC: 48008E7D  bl 0x82e44a68
	ctx.lr = 0x82E3BBF0;
	sub_82E44A68(ctx, base);
	// 82E3BBF0: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E3BBF4: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E3BBF8: 409A0130  bne cr6, 0x82e3bd28
	if !ctx.cr[6].eq {
	pc = 0x82E3BD28; continue 'dispatch;
	}
	// 82E3BBFC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82E3BC00: 41980128  blt cr6, 0x82e3bd28
	if ctx.cr[6].lt {
	pc = 0x82E3BD28; continue 'dispatch;
	}
	// 82E3BC04: 3BA30001  addi r29, r3, 1
	ctx.r[29].s64 = ctx.r[3].s64 + 1;
	// 82E3BC08: 80780014  lwz r3, 0x14(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E3BC0C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E3BC10: 4BF225B9  bl 0x82d5e1c8
	ctx.lr = 0x82E3BC14;
	sub_82D5E1C8(ctx, base);
	// 82E3BC14: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 82E3BC18: 80780014  lwz r3, 0x14(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E3BC1C: 4BF21FCD  bl 0x82d5dbe8
	ctx.lr = 0x82E3BC20;
	sub_82D5DBE8(ctx, base);
	// 82E3BC20: 80A1006C  lwz r5, 0x6c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E3BC24: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E3BC28: 80780014  lwz r3, 0x14(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E3BC2C: 4BF221DD  bl 0x82d5de08
	ctx.lr = 0x82E3BC30;
	sub_82D5DE08(ctx, base);
	// 82E3BC30: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E3BC34: 419A0054  beq cr6, 0x82e3bc88
	if ctx.cr[6].eq {
	pc = 0x82E3BC88; continue 'dispatch;
	}
	// 82E3BC38: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E3BC3C: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82E3BC40: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3BC44: 40990044  ble cr6, 0x82e3bc88
	if !ctx.cr[6].gt {
	pc = 0x82E3BC88; continue 'dispatch;
	}
	// 82E3BC48: 3B610050  addi r27, r1, 0x50
	ctx.r[27].s64 = ctx.r[1].s64 + 80;
	// 82E3BC4C: 9B410050  stb r26, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u8 ) };
	// 82E3BC50: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E3BC54: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82E3BC58: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82E3BC5C: 88DB0000  lbz r6, 0(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3BC60: 7D7F5A14  add r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82E3BC64: E88B0008  ld r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82E3BC68: E8AB0010  ld r5, 0x10(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82E3BC6C: 4BFFFDCD  bl 0x82e3ba38
	ctx.lr = 0x82E3BC70;
	sub_82E3BA38(ctx, base);
	// 82E3BC70: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E3BC74: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E3BC78: 3BFF0018  addi r31, r31, 0x18
	ctx.r[31].s64 = ctx.r[31].s64 + 24;
	// 82E3BC7C: 7FA3EA14  add r29, r3, r29
	ctx.r[29].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 82E3BC80: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E3BC84: 4198FFCC  blt cr6, 0x82e3bc50
	if ctx.cr[6].lt {
	pc = 0x82E3BC50; continue 'dispatch;
	}
	// 82E3BC88: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E3BC8C: 92E10080  stw r23, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[23].u32 ) };
	// 82E3BC90: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3BC94: 409A0018  bne cr6, 0x82e3bcac
	if !ctx.cr[6].eq {
	pc = 0x82E3BCAC; continue 'dispatch;
	}
	// 82E3BC98: 80610088  lwz r3, 0x88(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E3BC9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E3BCA0: 419A000C  beq cr6, 0x82e3bcac
	if ctx.cr[6].eq {
	pc = 0x82E3BCAC; continue 'dispatch;
	}
	// 82E3BCA4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E3BCA8: 4BFFBF79  bl 0x82e37c20
	ctx.lr = 0x82E3BCAC;
	sub_82E37C20(ctx, base);
	// 82E3BCAC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E3BCB0: 81410070  lwz r10, 0x70(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E3BCB4: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E3BCB8: 55490000  rlwinm r9, r10, 0, 0, 0
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3BCBC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E3BCC0: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82E3BCC4: 409A0020  bne cr6, 0x82e3bce4
	if !ctx.cr[6].eq {
	pc = 0x82E3BCE4; continue 'dispatch;
	}
	// 82E3BCC8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3BCCC: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E3BCD0: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E3BCD4: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E3BCD8: 554500BE  clrlwi r5, r10, 2
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3BCDC: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3BCE0: 4BF195E9  bl 0x82d552c8
	ctx.lr = 0x82E3BCE4;
	sub_82D552C8(ctx, base);
	// 82E3BCE4: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E3BCE8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3BCEC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E3BCF0: 409A002C  bne cr6, 0x82e3bd1c
	if !ctx.cr[6].eq {
	pc = 0x82E3BD1C; continue 'dispatch;
	}
	// 82E3BCF4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3BCF8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E3BCFC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3BD00: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E3BD04: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E3BD08: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E3BD0C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E3BD10: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E3BD14: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E3BD18: 4BF195B1  bl 0x82d552c8
	ctx.lr = 0x82E3BD1C;
	sub_82D552C8(ctx, base);
	// 82E3BD1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E3BD20: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82E3BD24: 4BE6D71C  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
	// 82E3BD28: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E3BD2C: 92E10080  stw r23, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[23].u32 ) };
	// 82E3BD30: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3BD34: 409A0018  bne cr6, 0x82e3bd4c
	if !ctx.cr[6].eq {
	pc = 0x82E3BD4C; continue 'dispatch;
	}
	// 82E3BD38: 80610088  lwz r3, 0x88(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E3BD3C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E3BD40: 419A000C  beq cr6, 0x82e3bd4c
	if ctx.cr[6].eq {
	pc = 0x82E3BD4C; continue 'dispatch;
	}
	// 82E3BD44: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E3BD48: 4BFFBED9  bl 0x82e37c20
	ctx.lr = 0x82E3BD4C;
	sub_82E37C20(ctx, base);
	// 82E3BD4C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E3BD50: 81410070  lwz r10, 0x70(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E3BD54: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E3BD58: 55490000  rlwinm r9, r10, 0, 0, 0
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3BD5C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E3BD60: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82E3BD64: 409A0020  bne cr6, 0x82e3bd84
	if !ctx.cr[6].eq {
	pc = 0x82E3BD84; continue 'dispatch;
	}
	// 82E3BD68: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3BD6C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E3BD70: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E3BD74: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E3BD78: 554500BE  clrlwi r5, r10, 2
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3BD7C: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3BD80: 4BF19549  bl 0x82d552c8
	ctx.lr = 0x82E3BD84;
	sub_82D552C8(ctx, base);
	// 82E3BD84: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E3BD88: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3BD8C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E3BD90: 409A002C  bne cr6, 0x82e3bdbc
	if !ctx.cr[6].eq {
	pc = 0x82E3BDBC; continue 'dispatch;
	}
	// 82E3BD94: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3BD98: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E3BD9C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3BDA0: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E3BDA4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E3BDA8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E3BDAC: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E3BDB0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E3BDB4: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E3BDB8: 4BF19511  bl 0x82d552c8
	ctx.lr = 0x82E3BDBC;
	sub_82D552C8(ctx, base);
	// 82E3BDBC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82E3BDC0: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82E3BDC4: 4BE6D67C  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3BDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3BDC8 size=220
    let mut pc: u32 = 0x82E3BDC8;
    'dispatch: loop {
        match pc {
            0x82E3BDC8 => {
    //   block [0x82E3BDC8..0x82E3BEA4)
	// 82E3BDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3BDCC: 4BE6D635  bl 0x82ca9400
	ctx.lr = 0x82E3BDD0;
	sub_82CA93D0(ctx, base);
	// 82E3BDD0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3BDD4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E3BDD8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E3BDDC: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E3BDE0: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82E3BDE4: 815D001C  lwz r10, 0x1c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E3BDE8: 93810058  stw r28, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 82E3BDEC: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 82E3BDF0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E3BDF4: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E3BDF8: 4099007C  ble cr6, 0x82e3be74
	if !ctx.cr[6].gt {
	pc = 0x82E3BE74; continue 'dispatch;
	}
	// 82E3BDFC: 3B610050  addi r27, r1, 0x50
	ctx.r[27].s64 = ctx.r[1].s64 + 80;
	// 82E3BE00: 9B810050  stb r28, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u8 ) };
	// 82E3BE04: 3B5DFFF8  addi r26, r29, -8
	ctx.r[26].s64 = ctx.r[29].s64 + -8;
	// 82E3BE08: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82E3BE0C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3BE10: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3BE14: 40980024  bge cr6, 0x82e3be38
	if !ctx.cr[6].lt {
	pc = 0x82E3BE38; continue 'dispatch;
	}
	// 82E3BE18: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E3BE1C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3BE20: 41990008  bgt cr6, 0x82e3be28
	if ctx.cr[6].gt {
	pc = 0x82E3BE28; continue 'dispatch;
	}
	// 82E3BE24: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82E3BE28: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E3BE2C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E3BE30: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E3BE34: 4BF1B0DD  bl 0x82d56f10
	ctx.lr = 0x82E3BE38;
	sub_82D56F10(ctx, base);
	// 82E3BE38: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E3BE3C: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 82E3BE40: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E3BE44: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 82E3BE48: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E3BE4C: 88DB0000  lbz r6, 0(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3BE50: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3BE54: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3BE58: 4BFFFBE1  bl 0x82e3ba38
	ctx.lr = 0x82E3BE5C;
	sub_82E3BA38(ctx, base);
	// 82E3BE5C: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E3BE60: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E3BE64: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82E3BE68: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E3BE6C: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E3BE70: 4198FF9C  blt cr6, 0x82e3be0c
	if ctx.cr[6].lt {
	pc = 0x82E3BE0C; continue 'dispatch;
	}
	// 82E3BE74: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3BE78: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E3BE7C: 409A0020  bne cr6, 0x82e3be9c
	if !ctx.cr[6].eq {
	pc = 0x82E3BE9C; continue 'dispatch;
	}
	// 82E3BE80: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3BE84: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E3BE88: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E3BE8C: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E3BE90: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E3BE94: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E3BE98: 4BF19431  bl 0x82d552c8
	ctx.lr = 0x82E3BE9C;
	sub_82D552C8(ctx, base);
	// 82E3BE9C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E3BEA0: 4BE6D5B0  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3BEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3BEA8 size=1012
    let mut pc: u32 = 0x82E3BEA8;
    'dispatch: loop {
        match pc {
            0x82E3BEA8 => {
    //   block [0x82E3BEA8..0x82E3C29C)
	// 82E3BEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3BEAC: 4BE6D559  bl 0x82ca9404
	ctx.lr = 0x82E3BEB0;
	sub_82CA93D0(ctx, base);
	// 82E3BEB0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3BEB4: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82E3BEB8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E3BEBC: 2F0B0023  cmpwi cr6, r11, 0x23
	ctx.cr[6].compare_i32(ctx.r[11].s32, 35, &mut ctx.xer);
	// 82E3BEC0: 419A0214  beq cr6, 0x82e3c0d4
	if ctx.cr[6].eq {
	pc = 0x82E3C0D4; continue 'dispatch;
	}
	// 82E3BEC4: 2F0B0025  cmpwi cr6, r11, 0x25
	ctx.cr[6].compare_i32(ctx.r[11].s32, 37, &mut ctx.xer);
	// 82E3BEC8: 409A03CC  bne cr6, 0x82e3c294
	if !ctx.cr[6].eq {
	pc = 0x82E3C294; continue 'dispatch;
	}
	// 82E3BECC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3BED0: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3BED4: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E3BED8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E3BEDC: 48292015  bl 0x830cdef0
	ctx.lr = 0x82E3BEE0;
	sub_830CDEF0(ctx, base);
	// 82E3BEE0: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3BEE4: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E3BEE8: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3BEEC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E3BEF0: 48292001  bl 0x830cdef0
	ctx.lr = 0x82E3BEF4;
	sub_830CDEF0(ctx, base);
	// 82E3BEF4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3BEF8: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82E3BEFC: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3BF00: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E3BF04: 48291FED  bl 0x830cdef0
	ctx.lr = 0x82E3BF08;
	sub_830CDEF0(ctx, base);
	// 82E3BF08: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3BF0C: 3B800004  li r28, 4
	ctx.r[28].s64 = 4;
	// 82E3BF10: A3E10050  lhz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3BF14: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82E3BF18: 57E4083E  rotlwi r4, r31, 1
	ctx.r[4].u64 = ((ctx.r[31].u32).rotate_left(1)) as u64;
	// 82E3BF1C: 7C7CD82E  lwzx r3, r28, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82E3BF20: 4BF19329  bl 0x82d55248
	ctx.lr = 0x82E3BF24;
	sub_82D55248(ctx, base);
	// 82E3BF24: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E3BF28: 90610068  stw r3, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 82E3BF2C: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 82E3BF30: 93E10070  stw r31, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 82E3BF34: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82E3BF38: 40980024  bge cr6, 0x82e3bf5c
	if !ctx.cr[6].lt {
	pc = 0x82E3BF5C; continue 'dispatch;
	}
	// 82E3BF3C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E3BF40: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E3BF44: 41980008  blt cr6, 0x82e3bf4c
	if ctx.cr[6].lt {
	pc = 0x82E3BF4C; continue 'dispatch;
	}
	// 82E3BF48: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E3BF4C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82E3BF50: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E3BF54: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E3BF58: 4BF1AFB9  bl 0x82d56f10
	ctx.lr = 0x82E3BF5C;
	sub_82D56F10(ctx, base);
	// 82E3BF5C: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 82E3BF60: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E3BF64: 40990038  ble cr6, 0x82e3bf9c
	if !ctx.cr[6].gt {
	pc = 0x82E3BF9C; continue 'dispatch;
	}
	// 82E3BF68: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E3BF6C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3BF70: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3BF74: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82E3BF78: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E3BF7C: 48291F75  bl 0x830cdef0
	ctx.lr = 0x82E3BF80;
	sub_830CDEF0(ctx, base);
	// 82E3BF80: A1610050  lhz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3BF84: 81410068  lwz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E3BF88: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82E3BF8C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E3BF90: 7D7D532E  sthx r11, r29, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u16) };
	// 82E3BF94: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 82E3BF98: 409AFFD4  bne cr6, 0x82e3bf6c
	if !ctx.cr[6].eq {
	pc = 0x82E3BF6C; continue 'dispatch;
	}
	// 82E3BF9C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3BFA0: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3BFA4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E3BFA8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E3BFAC: 48291F45  bl 0x830cdef0
	ctx.lr = 0x82E3BFB0;
	sub_830CDEF0(ctx, base);
	// 82E3BFB0: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3BFB4: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82E3BFB8: 7C7CD82E  lwzx r3, r28, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82E3BFBC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E3BFC0: 4BF19289  bl 0x82d55248
	ctx.lr = 0x82E3BFC4;
	sub_82D55248(ctx, base);
	// 82E3BFC4: 57EB00BE  clrlwi r11, r31, 2
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3BFC8: 90610078  stw r3, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[3].u32 ) };
	// 82E3BFCC: 93E1007C  stw r31, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[31].u32 ) };
	// 82E3BFD0: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82E3BFD4: 93E10080  stw r31, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 82E3BFD8: 40980024  bge cr6, 0x82e3bffc
	if !ctx.cr[6].lt {
	pc = 0x82E3BFFC; continue 'dispatch;
	}
	// 82E3BFDC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E3BFE0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E3BFE4: 41980008  blt cr6, 0x82e3bfec
	if ctx.cr[6].lt {
	pc = 0x82E3BFEC; continue 'dispatch;
	}
	// 82E3BFE8: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E3BFEC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E3BFF0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E3BFF4: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82E3BFF8: 4BF1AF19  bl 0x82d56f10
	ctx.lr = 0x82E3BFFC;
	sub_82D56F10(ctx, base);
	// 82E3BFFC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E3C000: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3C004: 80810078  lwz r4, 0x78(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E3C008: 93E1007C  stw r31, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[31].u32 ) };
	// 82E3C00C: 48292025  bl 0x830ce030
	ctx.lr = 0x82E3C010;
	sub_830CE030(ctx, base);
	// 82E3C010: EBC10058  ld r30, 0x58(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82E3C014: 2B3E0000  cmpldi cr6, r30, 0
	ctx.cr[6].compare_u64(ctx.r[30].u64, 0, &mut ctx.xer);
	// 82E3C018: 4099006C  ble cr6, 0x82e3c084
	if !ctx.cr[6].gt {
	pc = 0x82E3C084; continue 'dispatch;
	}
	// 82E3C01C: E9610060  ld r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E3C020: 2B2B0000  cmpldi cr6, r11, 0
	ctx.cr[6].compare_u64(ctx.r[11].u64, 0, &mut ctx.xer);
	// 82E3C024: 40990060  ble cr6, 0x82e3c084
	if !ctx.cr[6].gt {
	pc = 0x82E3C084; continue 'dispatch;
	}
	// 82E3C028: 8141006C  lwz r10, 0x6c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E3C02C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E3C030: 40990054  ble cr6, 0x82e3c084
	if !ctx.cr[6].gt {
	pc = 0x82E3C084; continue 'dispatch;
	}
	// 82E3C034: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E3C038: 419A004C  beq cr6, 0x82e3c084
	if ctx.cr[6].eq {
	pc = 0x82E3C084; continue 'dispatch;
	}
	// 82E3C03C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82E3C040: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E3C044: A08B0000  lhz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C048: 4BF199A9  bl 0x82d559f0
	ctx.lr = 0x82E3C04C;
	sub_82D559F0(ctx, base);
	// 82E3C04C: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82E3C050: 57CA003E  slwi r10, r30, 0
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E3C054: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E3C058: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 82E3C05C: A1680012  lhz r11, 0x12(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(18 as u32) ) } as u64;
	// 82E3C060: 88E8000D  lbz r7, 0xd(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(13 as u32) ) } as u64;
	// 82E3C064: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E3C068: 88C8000C  lbz r6, 0xc(r8)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3C06C: 4BFFF515  bl 0x82e3b580
	ctx.lr = 0x82E3C070;
	sub_82E3B580(ctx, base);
	// 82E3C070: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E3C074: 419A0010  beq cr6, 0x82e3c084
	if ctx.cr[6].eq {
	pc = 0x82E3C084; continue 'dispatch;
	}
	// 82E3C078: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E3C07C: 80810078  lwz r4, 0x78(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E3C080: 4BF1CCB1  bl 0x82d58d30
	ctx.lr = 0x82E3C084;
	sub_82D58D30(ctx, base);
	// 82E3C084: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E3C088: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3C08C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E3C090: 409A0018  bne cr6, 0x82e3c0a8
	if !ctx.cr[6].eq {
	pc = 0x82E3C0A8; continue 'dispatch;
	}
	// 82E3C094: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E3C098: 7C7CD82E  lwzx r3, r28, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82E3C09C: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3C0A0: 80810078  lwz r4, 0x78(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E3C0A4: 4BF19225  bl 0x82d552c8
	ctx.lr = 0x82E3C0A8;
	sub_82D552C8(ctx, base);
	// 82E3C0A8: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E3C0AC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3C0B0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E3C0B4: 409A01E0  bne cr6, 0x82e3c294
	if !ctx.cr[6].eq {
	pc = 0x82E3C294; continue 'dispatch;
	}
	// 82E3C0B8: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82E3C0BC: 7C7CD82E  lwzx r3, r28, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82E3C0C0: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E3C0C4: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E3C0C8: 4BF19201  bl 0x82d552c8
	ctx.lr = 0x82E3C0CC;
	sub_82D552C8(ctx, base);
	// 82E3C0CC: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82E3C0D0: 4BE6D384  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82E3C0D4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3C0D8: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3C0DC: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E3C0E0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E3C0E4: 48291E0D  bl 0x830cdef0
	ctx.lr = 0x82E3C0E8;
	sub_830CDEF0(ctx, base);
	// 82E3C0E8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3C0EC: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E3C0F0: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3C0F4: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E3C0F8: 48291DF9  bl 0x830cdef0
	ctx.lr = 0x82E3C0FC;
	sub_830CDEF0(ctx, base);
	// 82E3C0FC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3C100: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E3C104: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3C108: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E3C10C: 48291DE5  bl 0x830cdef0
	ctx.lr = 0x82E3C110;
	sub_830CDEF0(ctx, base);
	// 82E3C110: E9410060  ld r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E3C114: 2B2A0000  cmpldi cr6, r10, 0
	ctx.cr[6].compare_u64(ctx.r[10].u64, 0, &mut ctx.xer);
	// 82E3C118: 4099017C  ble cr6, 0x82e3c294
	if !ctx.cr[6].gt {
	pc = 0x82E3C294; continue 'dispatch;
	}
	// 82E3C11C: E9610058  ld r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82E3C120: 2B2B0000  cmpldi cr6, r11, 0
	ctx.cr[6].compare_u64(ctx.r[11].u64, 0, &mut ctx.xer);
	// 82E3C124: 40990170  ble cr6, 0x82e3c294
	if !ctx.cr[6].gt {
	pc = 0x82E3C294; continue 'dispatch;
	}
	// 82E3C128: 557D003E  slwi r29, r11, 0
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82E3C12C: 8B610050  lbz r27, 0x50(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3C130: 555C003E  slwi r28, r10, 0
	ctx.r[28].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82E3C134: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E3C138: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82E3C13C: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E3C140: 396B7634  addi r11, r11, 0x7634
	ctx.r[11].s64 = ctx.r[11].s64 + 30260;
	// 82E3C144: 93E10088  stw r31, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[31].u32 ) };
	// 82E3C148: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E3C14C: 93E1008C  stw r31, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[31].u32 ) };
	// 82E3C150: 91410090  stw r10, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[10].u32 ) };
	// 82E3C154: 419A0014  beq cr6, 0x82e3c168
	if ctx.cr[6].eq {
	pc = 0x82E3C168; continue 'dispatch;
	}
	// 82E3C158: 576B07BC  rlwinm r11, r27, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3C15C: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82E3C160: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3C164: 419A0008  beq cr6, 0x82e3c16c
	if ctx.cr[6].eq {
	pc = 0x82E3C16C; continue 'dispatch;
	}
	// 82E3C168: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E3C16C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E3C170: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82E3C174: 38E10088  addi r7, r1, 0x88
	ctx.r[7].s64 = ctx.r[1].s64 + 136;
	// 82E3C178: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E3C17C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E3C180: 387EFFF8  addi r3, r30, -8
	ctx.r[3].s64 = ctx.r[30].s64 + -8;
	// 82E3C184: 88CB0000  lbz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C188: 4BFFF8B1  bl 0x82e3ba38
	ctx.lr = 0x82E3C18C;
	sub_82E3BA38(ctx, base);
	// 82E3C18C: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E3C190: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E3C194: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E3C198: 40990024  ble cr6, 0x82e3c1bc
	if !ctx.cr[6].gt {
	pc = 0x82E3C1BC; continue 'dispatch;
	}
	// 82E3C19C: 813E0018  lwz r9, 0x18(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E3C1A0: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C1A4: 7F08E040  cmplw cr6, r8, r28
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E3C1A8: 419A0014  beq cr6, 0x82e3c1bc
	if ctx.cr[6].eq {
	pc = 0x82E3C1BC; continue 'dispatch;
	}
	// 82E3C1AC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E3C1B0: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82E3C1B4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E3C1B8: 4198FFE8  blt cr6, 0x82e3c1a0
	if ctx.cr[6].lt {
	pc = 0x82E3C1A0; continue 'dispatch;
	}
	// 82E3C1BC: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E3C1C0: 576907FE  clrlwi r9, r27, 0x1f
	ctx.r[9].u64 = ctx.r[27].u32 as u64 & 0x00000001u64;
	// 82E3C1C4: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82E3C1C8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E3C1CC: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82E3C1D0: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82E3C1D4: 419A0058  beq cr6, 0x82e3c22c
	if ctx.cr[6].eq {
	pc = 0x82E3C22C; continue 'dispatch;
	}
	// 82E3C1D8: 554B063E  clrlwi r11, r10, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82E3C1DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3C1E0: 409A0088  bne cr6, 0x82e3c268
	if !ctx.cr[6].eq {
	pc = 0x82E3C268; continue 'dispatch;
	}
	// 82E3C1E4: 3BFE0018  addi r31, r30, 0x18
	ctx.r[31].s64 = ctx.r[30].s64 + 24;
	// 82E3C1E8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3C1EC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3C1F0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3C1F4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E3C1F8: 409A0010  bne cr6, 0x82e3c208
	if !ctx.cr[6].eq {
	pc = 0x82E3C208; continue 'dispatch;
	}
	// 82E3C1FC: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82E3C200: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3C204: 4BF1AD95  bl 0x82d56f98
	ctx.lr = 0x82E3C208;
	sub_82D56F98(ctx, base);
	// 82E3C208: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3C20C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C210: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E3C214: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82E3C218: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82E3C21C: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82E3C220: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E3C224: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E3C228: 48000040  b 0x82e3c268
	pc = 0x82E3C268; continue 'dispatch;
	// 82E3C22C: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82E3C230: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E3C234: 419A0034  beq cr6, 0x82e3c268
	if ctx.cr[6].eq {
	pc = 0x82E3C268; continue 'dispatch;
	}
	// 82E3C238: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E3C23C: 55691838  slwi r9, r11, 3
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E3C240: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E3C244: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E3C248: 7D095A14  add r8, r9, r11
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82E3C24C: 55491838  slwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E3C250: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82E3C254: 915E001C  stw r10, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82E3C258: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C25C: 91480000  stw r10, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E3C260: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3C264: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E3C268: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E3C26C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3C270: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E3C274: 409A0020  bne cr6, 0x82e3c294
	if !ctx.cr[6].eq {
	pc = 0x82E3C294; continue 'dispatch;
	}
	// 82E3C278: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C27C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E3C280: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E3C284: 80810088  lwz r4, 0x88(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E3C288: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E3C28C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E3C290: 4BF19039  bl 0x82d552c8
	ctx.lr = 0x82E3C294;
	sub_82D552C8(ctx, base);
	// 82E3C294: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82E3C298: 4BE6D1BC  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3C2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3C2A0 size=12
    let mut pc: u32 = 0x82E3C2A0;
    'dispatch: loop {
        match pc {
            0x82E3C2A0 => {
    //   block [0x82E3C2A0..0x82E3C2AC)
	// 82E3C2A0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E3C2A4: 806BB06C  lwz r3, -0x4f94(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20372 as u32) ) } as u64;
	// 82E3C2A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3C2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3C2B0 size=8
    let mut pc: u32 = 0x82E3C2B0;
    'dispatch: loop {
        match pc {
            0x82E3C2B0 => {
    //   block [0x82E3C2B0..0x82E3C2B8)
	// 82E3C2B0: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82E3C2B4: 48000004  b 0x82e3c2b8
	sub_82E3C2B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3C2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3C2B8 size=100
    let mut pc: u32 = 0x82E3C2B8;
    'dispatch: loop {
        match pc {
            0x82E3C2B8 => {
    //   block [0x82E3C2B8..0x82E3C31C)
	// 82E3C2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3C2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E3C2C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E3C2C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E3C2C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3C2CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3C2D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E3C2D4: 4BFFF5BD  bl 0x82e3b890
	ctx.lr = 0x82E3C2D8;
	sub_82E3B890(ctx, base);
	// 82E3C2D8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E3C2DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3C2E0: 419A0020  beq cr6, 0x82e3c300
	if ctx.cr[6].eq {
	pc = 0x82E3C300; continue 'dispatch;
	}
	// 82E3C2E4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C2E8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E3C2EC: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 82E3C2F0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3C2F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E3C2F8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3C2FC: 4BF18FCD  bl 0x82d552c8
	ctx.lr = 0x82E3C300;
	sub_82D552C8(ctx, base);
	// 82E3C300: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3C304: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E3C308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3C30C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E3C310: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E3C314: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E3C318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3C320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3C320 size=220
    let mut pc: u32 = 0x82E3C320;
    'dispatch: loop {
        match pc {
            0x82E3C320 => {
    //   block [0x82E3C320..0x82E3C3FC)
	// 82E3C320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3C324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E3C328: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E3C32C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3C330: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E3C334: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E3C338: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3C33C: 396B5B24  addi r11, r11, 0x5b24
	ctx.r[11].s64 = ctx.r[11].s64 + 23332;
	// 82E3C340: 394A5B04  addi r10, r10, 0x5b04
	ctx.r[10].s64 = ctx.r[10].s64 + 23300;
	// 82E3C344: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E3C348: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E3C34C: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E3C350: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3C354: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E3C358: 409A0020  bne cr6, 0x82e3c378
	if !ctx.cr[6].eq {
	pc = 0x82E3C378; continue 'dispatch;
	}
	// 82E3C35C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C360: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E3C364: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E3C368: 809F0038  lwz r4, 0x38(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E3C36C: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3C370: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E3C374: 4BF18F55  bl 0x82d552c8
	ctx.lr = 0x82E3C378;
	sub_82D552C8(ctx, base);
	// 82E3C378: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E3C37C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3C380: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E3C384: 409A0020  bne cr6, 0x82e3c3a4
	if !ctx.cr[6].eq {
	pc = 0x82E3C3A4; continue 'dispatch;
	}
	// 82E3C388: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C38C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E3C390: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E3C394: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E3C398: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3C39C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E3C3A0: 4BF18F29  bl 0x82d552c8
	ctx.lr = 0x82E3C3A4;
	sub_82D552C8(ctx, base);
	// 82E3C3A4: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E3C3A8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3C3AC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E3C3B0: 409A0020  bne cr6, 0x82e3c3d0
	if !ctx.cr[6].eq {
	pc = 0x82E3C3D0; continue 'dispatch;
	}
	// 82E3C3B4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C3B8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E3C3BC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E3C3C0: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E3C3C4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E3C3C8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E3C3CC: 4BF18EFD  bl 0x82d552c8
	ctx.lr = 0x82E3C3D0;
	sub_82D552C8(ctx, base);
	// 82E3C3D0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E3C3D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82E3C3D8: 396B58F4  addi r11, r11, 0x58f4
	ctx.r[11].s64 = ctx.r[11].s64 + 22772;
	// 82E3C3DC: 394A39E0  addi r10, r10, 0x39e0
	ctx.r[10].s64 = ctx.r[10].s64 + 14816;
	// 82E3C3E0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E3C3E4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E3C3E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E3C3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3C3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E3C3F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E3C3F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3C400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E3C400 size=564
    let mut pc: u32 = 0x82E3C400;
    'dispatch: loop {
        match pc {
            0x82E3C400 => {
    //   block [0x82E3C400..0x82E3C634)
	// 82E3C400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3C404: 4BE6CFF5  bl 0x82ca93f8
	ctx.lr = 0x82E3C408;
	sub_82CA93D0(ctx, base);
	// 82E3C408: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3C40C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E3C410: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E3C414: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E3C418: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E3C41C: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82E3C420: 396B58F4  addi r11, r11, 0x58f4
	ctx.r[11].s64 = ctx.r[11].s64 + 22772;
	// 82E3C424: 394A5B24  addi r10, r10, 0x5b24
	ctx.r[10].s64 = ctx.r[10].s64 + 23332;
	// 82E3C428: 39295B04  addi r9, r9, 0x5b04
	ctx.r[9].s64 = ctx.r[9].s64 + 23300;
	// 82E3C42C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E3C430: 3B7E0020  addi r27, r30, 0x20
	ctx.r[27].s64 = ctx.r[30].s64 + 32;
	// 82E3C434: B39E0006  sth r28, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[28].u16 ) };
	// 82E3C438: 3F408000  lis r26, -0x8000
	ctx.r[26].s64 = -2147483648;
	// 82E3C43C: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E3C440: 3BBE002C  addi r29, r30, 0x2c
	ctx.r[29].s64 = ctx.r[30].s64 + 44;
	// 82E3C444: 9B9E000C  stb r28, 0xc(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[28].u8 ) };
	// 82E3C448: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E3C44C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E3C450: 913E0008  stw r9, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E3C454: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82E3C458: 93FB0000  stw r31, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82E3C45C: 93FB0004  stw r31, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82E3C460: 935B0008  stw r26, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[26].u32 ) };
	// 82E3C464: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82E3C468: 93FD0004  stw r31, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82E3C46C: 935D0008  stw r26, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[26].u32 ) };
	// 82E3C470: 93FE0038  stw r31, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[31].u32 ) };
	// 82E3C474: 93FE003C  stw r31, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[31].u32 ) };
	// 82E3C478: 935E0040  stw r26, 0x40(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[26].u32 ) };
	// 82E3C47C: 4829220D  bl 0x830ce688
	ctx.lr = 0x82E3C480;
	sub_830CE688(ctx, base);
	// 82E3C480: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E3C484: 93E1009C  stw r31, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[31].u32 ) };
	// 82E3C488: 93E10094  stw r31, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[31].u32 ) };
	// 82E3C48C: 93810098  stw r28, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[28].u32 ) };
	// 82E3C490: 48291A31  bl 0x830cdec0
	ctx.lr = 0x82E3C494;
	sub_830CDEC0(ctx, base);
	// 82E3C494: 481CA13D  bl 0x830065d0
	ctx.lr = 0x82E3C498;
	sub_830065D0(ctx, base);
	// 82E3C498: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E3C49C: FDA00818  frsp f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82E3C4A0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E3C4A4: 3B2B3BA8  addi r25, r11, 0x3ba8
	ctx.r[25].s64 = ctx.r[11].s64 + 15272;
	// 82E3C4A8: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82E3C4AC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E3C4B0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82E3C4B4: 93410060  stw r26, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[26].u32 ) };
	// 82E3C4B8: B3810076  sth r28, 0x76(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(118 as u32), ctx.r[28].u16 ) };
	// 82E3C4BC: 93810080  stw r28, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 82E3C4C0: 93210070  stw r25, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[25].u32 ) };
	// 82E3C4C4: C00B0C14  lfs f0, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3C4C8: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 82E3C4CC: D00100A4  stfs f0, 0xa4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 82E3C4D0: 93A10078  stw r29, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[29].u32 ) };
	// 82E3C4D4: C00B9404  lfs f0, -0x6bfc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27644 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3C4D8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3C4DC: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82E3C4E0: D00100A0  stfs f0, 0xa0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 82E3C4E4: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82E3C4E8: 4BF1C0B1  bl 0x82d58598
	ctx.lr = 0x82E3C4EC;
	sub_82D58598(ctx, base);
	// 82E3C4EC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E3C4F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E3C4F4: 388B6A98  addi r4, r11, 0x6a98
	ctx.r[4].s64 = ctx.r[11].s64 + 27288;
	// 82E3C4F8: 4BF35171  bl 0x82d71668
	ctx.lr = 0x82E3C4FC;
	sub_82D71668(ctx, base);
	// 82E3C4FC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82E3C500: 39010058  addi r8, r1, 0x58
	ctx.r[8].s64 = ctx.r[1].s64 + 88;
	// 82E3C504: 38EBB204  addi r7, r11, -0x4dfc
	ctx.r[7].s64 = ctx.r[11].s64 + -19964;
	// 82E3C508: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
	// 82E3C50C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E3C510: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E3C514: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E3C518: 48008551  bl 0x82e44a68
	ctx.lr = 0x82E3C51C;
	sub_82E44A68(ctx, base);
	// 82E3C51C: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E3C520: 93210070  stw r25, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[25].u32 ) };
	// 82E3C524: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3C528: 409A0018  bne cr6, 0x82e3c540
	if !ctx.cr[6].eq {
	pc = 0x82E3C540; continue 'dispatch;
	}
	// 82E3C52C: 80610078  lwz r3, 0x78(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E3C530: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E3C534: 419A000C  beq cr6, 0x82e3c540
	if ctx.cr[6].eq {
	pc = 0x82E3C540; continue 'dispatch;
	}
	// 82E3C538: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E3C53C: 4BFFB6E5  bl 0x82e37c20
	ctx.lr = 0x82E3C540;
	sub_82E37C20(ctx, base);
	// 82E3C540: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E3C544: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E3C548: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E3C54C: 55490000  rlwinm r9, r10, 0, 0, 0
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3C550: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E3C554: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82E3C558: 409A002C  bne cr6, 0x82e3c584
	if !ctx.cr[6].eq {
	pc = 0x82E3C584; continue 'dispatch;
	}
	// 82E3C55C: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3C560: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C564: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E3C568: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E3C56C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E3C570: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E3C574: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E3C578: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E3C57C: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E3C580: 4BF18D49  bl 0x82d552c8
	ctx.lr = 0x82E3C584;
	sub_82D552C8(ctx, base);
	// 82E3C584: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3C588: 81380004  lwz r9, 4(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3C58C: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3C590: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E3C594: 40980060  bge cr6, 0x82e3c5f4
	if !ctx.cr[6].lt {
	pc = 0x82E3C5F4; continue 'dispatch;
	}
	// 82E3C598: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3C59C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3C5A0: 409A0020  bne cr6, 0x82e3c5c0
	if !ctx.cr[6].eq {
	pc = 0x82E3C5C0; continue 'dispatch;
	}
	// 82E3C5A4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C5A8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E3C5AC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E3C5B0: 809B0000  lwz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C5B4: 5545103A  slwi r5, r10, 2
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E3C5B8: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3C5BC: 4BF18D0D  bl 0x82d552c8
	ctx.lr = 0x82E3C5C0;
	sub_82D552C8(ctx, base);
	// 82E3C5C0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C5C4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E3C5C8: 81380004  lwz r9, 4(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3C5CC: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82E3C5D0: 5524103A  slwi r4, r9, 2
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E3C5D4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3C5D8: 4BF18C71  bl 0x82d55248
	ctx.lr = 0x82E3C5DC;
	sub_82D55248(ctx, base);
	// 82E3C5DC: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3C5E0: 907B0000  stw r3, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E3C5E4: 81580004  lwz r10, 4(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3C5E8: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3C5EC: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E3C5F0: 917B0008  stw r11, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E3C5F4: 81580004  lwz r10, 4(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3C5F8: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C5FC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E3C600: 915B0004  stw r10, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E3C604: 81380000  lwz r9, 0(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C608: 40990020  ble cr6, 0x82e3c628
	if !ctx.cr[6].gt {
	pc = 0x82E3C628; continue 'dispatch;
	}
	// 82E3C60C: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82E3C610: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3C614: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E3C618: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E3C61C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E3C620: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E3C624: 409AFFEC  bne cr6, 0x82e3c610
	if !ctx.cr[6].eq {
	pc = 0x82E3C610; continue 'dispatch;
	}
	// 82E3C628: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E3C62C: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 82E3C630: 4BE6CE18  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3C638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3C638 size=1756
    let mut pc: u32 = 0x82E3C638;
    'dispatch: loop {
        match pc {
            0x82E3C638 => {
    //   block [0x82E3C638..0x82E3CD14)
	// 82E3C638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3C63C: 4BE6CDC1  bl 0x82ca93fc
	ctx.lr = 0x82E3C640;
	sub_82CA93D0(ctx, base);
	// 82E3C640: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3C644: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82E3C648: 817A000C  lwz r11, 0xc(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3C64C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3C650: 419A06BC  beq cr6, 0x82e3cd0c
	if ctx.cr[6].eq {
	pc = 0x82E3CD0C; continue 'dispatch;
	}
	// 82E3C654: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82E3C658: 815A001C  lwz r10, 0x1c(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E3C65C: 3FA08000  lis r29, -0x8000
	ctx.r[29].s64 = -2147483648;
	// 82E3C660: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E3C664: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82E3C668: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E3C66C: 93210098  stw r25, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[25].u32 ) };
	// 82E3C670: 9321009C  stw r25, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[25].u32 ) };
	// 82E3C674: 93A100A0  stw r29, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[29].u32 ) };
	// 82E3C678: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 82E3C67C: 9321005C  stw r25, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[25].u32 ) };
	// 82E3C680: 93A10060  stw r29, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 82E3C684: 93210068  stw r25, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[25].u32 ) };
	// 82E3C688: 9321006C  stw r25, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[25].u32 ) };
	// 82E3C68C: 93A10070  stw r29, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[29].u32 ) };
	// 82E3C690: 40990168  ble cr6, 0x82e3c7f8
	if !ctx.cr[6].gt {
	pc = 0x82E3C7F8; continue 'dispatch;
	}
	// 82E3C694: 811A0018  lwz r8, 0x18(r26)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E3C698: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 82E3C69C: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C6A0: 80E7000C  lwz r7, 0xc(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3C6A4: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82E3C6A8: 41990018  bgt cr6, 0x82e3c6c0
	if ctx.cr[6].gt {
	pc = 0x82E3C6C0; continue 'dispatch;
	}
	// 82E3C6AC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E3C6B0: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82E3C6B4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E3C6B8: 4198FFE4  blt cr6, 0x82e3c69c
	if ctx.cr[6].lt {
	pc = 0x82E3C69C; continue 'dispatch;
	}
	// 82E3C6BC: 4800013C  b 0x82e3c7f8
	pc = 0x82E3C7F8; continue 'dispatch;
	// 82E3C6C0: 557E103A  slwi r30, r11, 2
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82E3C6C4: 7D7E402E  lwzx r11, r30, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82E3C6C8: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 82E3C6CC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3C6D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3C6D4: 40990034  ble cr6, 0x82e3c708
	if !ctx.cr[6].gt {
	pc = 0x82E3C708; continue 'dispatch;
	}
	// 82E3C6D8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C6DC: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E3C6E0: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82E3C6E4: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E3C6E8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E3C6EC: 4BF18B5D  bl 0x82d55248
	ctx.lr = 0x82E3C6F0;
	sub_82D55248(ctx, base);
	// 82E3C6F0: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E3C6F4: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 82E3C6F8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3C6FC: 554A0042  rlwinm r10, r10, 0, 1, 1
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3C700: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82E3C704: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E3C708: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3C70C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3C710: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82E3C714: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C718: 40990024  ble cr6, 0x82e3c73c
	if !ctx.cr[6].gt {
	pc = 0x82E3C73C; continue 'dispatch;
	}
	// 82E3C71C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82E3C720: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C724: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E3C728: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E3C72C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3C730: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E3C734: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82E3C738: 409AFFE8  bne cr6, 0x82e3c720
	if !ctx.cr[6].eq {
	pc = 0x82E3C720; continue 'dispatch;
	}
	// 82E3C73C: 815A0018  lwz r10, 0x18(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E3C740: 81210070  lwz r9, 0x70(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E3C744: 552B00BE  clrlwi r11, r9, 2
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3C748: 7D5E502E  lwzx r10, r30, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E3C74C: 3BEA002C  addi r31, r10, 0x2c
	ctx.r[31].s64 = ctx.r[10].s64 + 44;
	// 82E3C750: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3C754: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E3C758: 40980060  bge cr6, 0x82e3c7b8
	if !ctx.cr[6].lt {
	pc = 0x82E3C7B8; continue 'dispatch;
	}
	// 82E3C75C: 552A0000  rlwinm r10, r9, 0, 0, 0
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3C760: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E3C764: 409A0020  bne cr6, 0x82e3c784
	if !ctx.cr[6].eq {
	pc = 0x82E3C784; continue 'dispatch;
	}
	// 82E3C768: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C76C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E3C770: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E3C774: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E3C778: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E3C77C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E3C780: 4BF18B49  bl 0x82d552c8
	ctx.lr = 0x82E3C784;
	sub_82D552C8(ctx, base);
	// 82E3C784: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C788: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E3C78C: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3C790: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82E3C794: 5524103A  slwi r4, r9, 2
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E3C798: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3C79C: 4BF18AAD  bl 0x82d55248
	ctx.lr = 0x82E3C7A0;
	sub_82D55248(ctx, base);
	// 82E3C7A0: 81410070  lwz r10, 0x70(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E3C7A4: 90610068  stw r3, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 82E3C7A8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3C7AC: 554A0042  rlwinm r10, r10, 0, 1, 1
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3C7B0: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82E3C7B4: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82E3C7B8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3C7BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3C7C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82E3C7C4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C7C8: 40990024  ble cr6, 0x82e3c7ec
	if !ctx.cr[6].gt {
	pc = 0x82E3C7EC; continue 'dispatch;
	}
	// 82E3C7CC: 81210068  lwz r9, 0x68(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E3C7D0: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C7D4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E3C7D8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E3C7DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3C7E0: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E3C7E4: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82E3C7E8: 409AFFE8  bne cr6, 0x82e3c7d0
	if !ctx.cr[6].eq {
	pc = 0x82E3C7D0; continue 'dispatch;
	}
	// 82E3C7EC: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E3C7F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3C7F4: 409A008C  bne cr6, 0x82e3c880
	if !ctx.cr[6].eq {
	pc = 0x82E3C880; continue 'dispatch;
	}
	// 82E3C7F8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C7FC: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82E3C800: 81210060  lwz r9, 0x60(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E3C804: 552900BE  clrlwi r9, r9, 2
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3C808: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3C80C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E3C810: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C814: 83CB0004  lwz r30, 4(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3C818: 409A0010  bne cr6, 0x82e3c828
	if !ctx.cr[6].eq {
	pc = 0x82E3C828; continue 'dispatch;
	}
	// 82E3C81C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E3C820: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E3C824: 4BF1A775  bl 0x82d56f98
	ctx.lr = 0x82E3C828;
	sub_82D56F98(ctx, base);
	// 82E3C828: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E3C82C: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E3C830: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E3C834: 7FEB512E  stwx r31, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u32) };
	// 82E3C838: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E3C83C: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E3C840: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E3C844: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3C848: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82E3C84C: 8141006C  lwz r10, 0x6c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E3C850: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E3C854: 409A0010  bne cr6, 0x82e3c864
	if !ctx.cr[6].eq {
	pc = 0x82E3C864; continue 'dispatch;
	}
	// 82E3C858: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E3C85C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E3C860: 4BF1A739  bl 0x82d56f98
	ctx.lr = 0x82E3C864;
	sub_82D56F98(ctx, base);
	// 82E3C864: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E3C868: 81410068  lwz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E3C86C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E3C870: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82E3C874: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E3C878: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E3C87C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82E3C880: 3B7A0030  addi r27, r26, 0x30
	ctx.r[27].s64 = ctx.r[26].s64 + 48;
	// 82E3C884: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3C888: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3C88C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3C890: 40980020  bge cr6, 0x82e3c8b0
	if !ctx.cr[6].lt {
	pc = 0x82E3C8B0; continue 'dispatch;
	}
	// 82E3C894: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E3C898: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82E3C89C: 41990008  bgt cr6, 0x82e3c8a4
	if ctx.cr[6].gt {
	pc = 0x82E3C8A4; continue 'dispatch;
	}
	// 82E3C8A0: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82E3C8A4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E3C8A8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E3C8AC: 4BF1A665  bl 0x82d56f10
	ctx.lr = 0x82E3C8B0;
	sub_82D56F10(ctx, base);
	// 82E3C8B0: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 82E3C8B4: 933B0004  stw r25, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82E3C8B8: 7F3CCB78  mr r28, r25
	ctx.r[28].u64 = ctx.r[25].u64;
	// 82E3C8BC: 4B42A685  bl 0x82266f40
	ctx.lr = 0x82E3C8C0;
	sub_82266F40(ctx, base);
	// 82E3C8C0: 7F29CB78  mr r9, r25
	ctx.r[9].u64 = ctx.r[25].u64;
	// 82E3C8C4: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E3C8C8: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 82E3C8CC: 93210078  stw r25, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[25].u32 ) };
	// 82E3C8D0: 93A10080  stw r29, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[29].u32 ) };
	// 82E3C8D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3C8D8: 9121007C  stw r9, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[9].u32 ) };
	// 82E3C8DC: 40990050  ble cr6, 0x82e3c92c
	if !ctx.cr[6].gt {
	pc = 0x82E3C92C; continue 'dispatch;
	}
	// 82E3C8E0: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E3C8E4: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82E3C8E8: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E3C8EC: 38A10088  addi r5, r1, 0x88
	ctx.r[5].s64 = ctx.r[1].s64 + 136;
	// 82E3C8F0: 7C9F582E  lwzx r4, r31, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3C8F4: 7C7F502E  lwzx r3, r31, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E3C8F8: 48292099  bl 0x830ce990
	ctx.lr = 0x82E3C8FC;
	sub_830CE990(ctx, base);
	// 82E3C8FC: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E3C900: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E3C904: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E3C908: 7D3F582E  lwzx r9, r31, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3C90C: 7D1F502E  lwzx r8, r31, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E3C910: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E3C914: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 82E3C918: 7F89E214  add r28, r9, r28
	ctx.r[28].u64 = ctx.r[9].u64 + ctx.r[28].u64;
	// 82E3C91C: 8121005C  lwz r9, 0x5c(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E3C920: 7F1E4800  cmpw cr6, r30, r9
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E3C924: 4198FFC8  blt cr6, 0x82e3c8ec
	if ctx.cr[6].lt {
	pc = 0x82E3C8EC; continue 'dispatch;
	}
	// 82E3C928: 8121007C  lwz r9, 0x7c(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82E3C92C: 81010090  lwz r8, 0x90(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E3C930: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82E3C934: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E3C938: 41980024  blt cr6, 0x82e3c95c
	if ctx.cr[6].lt {
	pc = 0x82E3C95C; continue 'dispatch;
	}
	// 82E3C93C: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E3C940: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3C944: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82E3C948: 409A0014  bne cr6, 0x82e3c95c
	if !ctx.cr[6].eq {
	pc = 0x82E3C95C; continue 'dispatch;
	}
	// 82E3C94C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E3C950: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E3C954: 7F1F4000  cmpw cr6, r31, r8
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E3C958: 4099FFE8  ble cr6, 0x82e3c940
	if !ctx.cr[6].gt {
	pc = 0x82E3C940; continue 'dispatch;
	}
	// 82E3C95C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82E3C960: 7F1F4000  cmpw cr6, r31, r8
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E3C964: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82E3C968: 40990008  ble cr6, 0x82e3c970
	if !ctx.cr[6].gt {
	pc = 0x82E3C970; continue 'dispatch;
	}
	// 82E3C96C: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82E3C970: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82E3C974: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3C978: 419A00AC  beq cr6, 0x82e3ca24
	if ctx.cr[6].eq {
	pc = 0x82E3CA24; continue 'dispatch;
	}
	// 82E3C97C: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E3C980: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3C984: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E3C988: 409A0014  bne cr6, 0x82e3c99c
	if !ctx.cr[6].eq {
	pc = 0x82E3C99C; continue 'dispatch;
	}
	// 82E3C98C: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82E3C990: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82E3C994: 4BF1A605  bl 0x82d56f98
	ctx.lr = 0x82E3C998;
	sub_82D56F98(ctx, base);
	// 82E3C998: 8121007C  lwz r9, 0x7c(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82E3C99C: 552A2036  slwi r10, r9, 4
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E3C9A0: 81010078  lwz r8, 0x78(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E3C9A4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E3C9A8: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82E3C9AC: 57E8103A  slwi r8, r31, 2
	ctx.r[8].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E3C9B0: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 82E3C9B4: 9121007C  stw r9, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[9].u32 ) };
	// 82E3C9B8: 81210088  lwz r9, 0x88(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E3C9BC: 7D28482E  lwzx r9, r8, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82E3C9C0: 7D2907B4  extsw r9, r9
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 82E3C9C4: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82E3C9C8: 81210090  lwz r9, 0x90(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E3C9CC: 81010088  lwz r8, 0x88(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E3C9D0: 7D29FA14  add r9, r9, r31
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 82E3C9D4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E3C9D8: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E3C9DC: 7D29402E  lwzx r9, r9, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82E3C9E0: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E3C9E4: 81010090  lwz r8, 0x90(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E3C9E8: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E3C9EC: 4199002C  bgt cr6, 0x82e3ca18
	if ctx.cr[6].gt {
	pc = 0x82E3CA18; continue 'dispatch;
	}
	// 82E3C9F0: 81210088  lwz r9, 0x88(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E3C9F4: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E3C9F8: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82E3C9FC: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3CA00: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 82E3CA04: 409A0014  bne cr6, 0x82e3ca18
	if !ctx.cr[6].eq {
	pc = 0x82E3CA18; continue 'dispatch;
	}
	// 82E3CA08: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E3CA0C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E3CA10: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82E3CA14: 4099FFE8  ble cr6, 0x82e3c9fc
	if !ctx.cr[6].gt {
	pc = 0x82E3C9FC; continue 'dispatch;
	}
	// 82E3CA18: 8121007C  lwz r9, 0x7c(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82E3CA1C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82E3CA20: 4BFFFF40  b 0x82e3c960
	pc = 0x82E3C960; continue 'dispatch;
	// 82E3CA24: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E3CA28: B3C100B6  sth r30, 0xb6(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(182 as u32), ctx.r[30].u16 ) };
	// 82E3CA2C: 93C100C0  stw r30, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[30].u32 ) };
	// 82E3CA30: 3BEB3BA8  addi r31, r11, 0x3ba8
	ctx.r[31].s64 = ctx.r[11].s64 + 15272;
	// 82E3CA34: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3CA38: 916100BC  stw r11, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[11].u32 ) };
	// 82E3CA3C: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82E3CA40: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E3CA44: 916100B8  stw r11, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[11].u32 ) };
	// 82E3CA48: 815B0004  lwz r10, 4(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3CA4C: 813B0008  lwz r9, 8(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3CA50: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E3CA54: 552900BE  clrlwi r9, r9, 2
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3CA58: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E3CA5C: 40980028  bge cr6, 0x82e3ca84
	if !ctx.cr[6].lt {
	pc = 0x82E3CA84; continue 'dispatch;
	}
	// 82E3CA60: 552B083C  slwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E3CA64: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E3CA68: 41980008  blt cr6, 0x82e3ca70
	if ctx.cr[6].lt {
	pc = 0x82E3CA70; continue 'dispatch;
	}
	// 82E3CA6C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82E3CA70: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E3CA74: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E3CA78: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E3CA7C: 4BF1A495  bl 0x82d56f10
	ctx.lr = 0x82E3CA80;
	sub_82D56F10(ctx, base);
	// 82E3CA80: 816100B8  lwz r11, 0xb8(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(184 as u32) ) } as u64;
	// 82E3CA84: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3CA88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E3CA8C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3CA90: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E3CA94: 388B6A98  addi r4, r11, 0x6a98
	ctx.r[4].s64 = ctx.r[11].s64 + 27288;
	// 82E3CA98: 7F2A49AE  stbx r25, r10, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[25].u8) };
	// 82E3CA9C: 4BF34BCD  bl 0x82d71668
	ctx.lr = 0x82E3CAA0;
	sub_82D71668(ctx, base);
	// 82E3CAA0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82E3CAA4: 39010098  addi r8, r1, 0x98
	ctx.r[8].s64 = ctx.r[1].s64 + 152;
	// 82E3CAA8: 38EBB1D4  addi r7, r11, -0x4e2c
	ctx.r[7].s64 = ctx.r[11].s64 + -20012;
	// 82E3CAAC: 38C10078  addi r6, r1, 0x78
	ctx.r[6].s64 = ctx.r[1].s64 + 120;
	// 82E3CAB0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E3CAB4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E3CAB8: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E3CABC: 48007FAD  bl 0x82e44a68
	ctx.lr = 0x82E3CAC0;
	sub_82E44A68(ctx, base);
	// 82E3CAC0: 816100C0  lwz r11, 0xc0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(192 as u32) ) } as u64;
	// 82E3CAC4: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E3CAC8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3CACC: 409A0084  bne cr6, 0x82e3cb50
	if !ctx.cr[6].eq {
	pc = 0x82E3CB50; continue 'dispatch;
	}
	// 82E3CAD0: 816100B8  lwz r11, 0xb8(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(184 as u32) ) } as u64;
	// 82E3CAD4: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82E3CAD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3CADC: 419A0074  beq cr6, 0x82e3cb50
	if ctx.cr[6].eq {
	pc = 0x82E3CB50; continue 'dispatch;
	}
	// 82E3CAE0: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3CAE4: 55490000  rlwinm r9, r10, 0, 0, 0
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3CAE8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E3CAEC: 409A0020  bne cr6, 0x82e3cb0c
	if !ctx.cr[6].eq {
	pc = 0x82E3CB0C; continue 'dispatch;
	}
	// 82E3CAF0: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3CAF4: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82E3CAF8: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E3CAFC: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3CB00: 554500BE  clrlwi r5, r10, 2
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3CB04: 7C68482E  lwzx r3, r8, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82E3CB08: 4BF187C1  bl 0x82d552c8
	ctx.lr = 0x82E3CB0C;
	sub_82D552C8(ctx, base);
	// 82E3CB0C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3CB10: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E3CB14: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3CB18: 8163004C  lwz r11, 0x4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82E3CB1C: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E3CB20: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E3CB24: 41980014  blt cr6, 0x82e3cb38
	if ctx.cr[6].lt {
	pc = 0x82E3CB38; continue 'dispatch;
	}
	// 82E3CB28: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E3CB2C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82E3CB30: 4BF185F9  bl 0x82d55128
	ctx.lr = 0x82E3CB34;
	sub_82D55128(ctx, base);
	// 82E3CB34: 4800001C  b 0x82e3cb50
	pc = 0x82E3CB50; continue 'dispatch;
	// 82E3CB38: 8163004C  lwz r11, 0x4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82E3CB3C: 81430048  lwz r10, 0x48(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E3CB40: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E3CB44: 9163004C  stw r11, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82E3CB48: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E3CB4C: 93E30048  stw r31, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	// 82E3CB50: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E3CB54: 81410080  lwz r10, 0x80(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E3CB58: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E3CB5C: 55490000  rlwinm r9, r10, 0, 0, 0
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3CB60: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E3CB64: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 82E3CB68: 409A0020  bne cr6, 0x82e3cb88
	if !ctx.cr[6].eq {
	pc = 0x82E3CB88; continue 'dispatch;
	}
	// 82E3CB6C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3CB70: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E3CB74: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E3CB78: 80810078  lwz r4, 0x78(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E3CB7C: 55452036  slwi r5, r10, 4
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E3CB80: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3CB84: 4BF18745  bl 0x82d552c8
	ctx.lr = 0x82E3CB88;
	sub_82D552C8(ctx, base);
	// 82E3CB88: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 82E3CB8C: 4B42A37D  bl 0x82266f08
	ctx.lr = 0x82E3CB90;
	sub_82266F08(ctx, base);
	// 82E3CB90: 2F1C0001  cmpwi cr6, r28, 1
	ctx.cr[6].compare_i32(ctx.r[28].s32, 1, &mut ctx.xer);
	// 82E3CB94: 419800E8  blt cr6, 0x82e3cc7c
	if ctx.cr[6].lt {
	pc = 0x82E3CC7C; continue 'dispatch;
	}
	// 82E3CB98: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E3CB9C: 83DA0034  lwz r30, 0x34(r26)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E3CBA0: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E3CBA4: 83FA0028  lwz r31, 0x28(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E3CBA8: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82E3CBAC: 807A000C  lwz r3, 0xc(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3CBB0: 7D6AF214  add r11, r10, r30
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82E3CBB4: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E3CBB8: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82E3CBBC: 388B000D  addi r4, r11, 0xd
	ctx.r[4].s64 = ctx.r[11].s64 + 13;
	// 82E3CBC0: 4BF21609  bl 0x82d5e1c8
	ctx.lr = 0x82E3CBC4;
	sub_82D5E1C8(ctx, base);
	// 82E3CBC4: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 82E3CBC8: 807A000C  lwz r3, 0xc(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3CBCC: 4BF2101D  bl 0x82d5dbe8
	ctx.lr = 0x82E3CBD0;
	sub_82D5DBE8(ctx, base);
	// 82E3CBD0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E3CBD4: 807A000C  lwz r3, 0xc(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3CBD8: 4BF215C1  bl 0x82d5e198
	ctx.lr = 0x82E3CBDC;
	sub_82D5E198(ctx, base);
	// 82E3CBDC: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E3CBE0: 40990014  ble cr6, 0x82e3cbf4
	if !ctx.cr[6].gt {
	pc = 0x82E3CBF4; continue 'dispatch;
	}
	// 82E3CBE4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E3CBE8: 809A0024  lwz r4, 0x24(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E3CBEC: 807A000C  lwz r3, 0xc(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3CBF0: 4BF21219  bl 0x82d5de08
	ctx.lr = 0x82E3CBF4;
	sub_82D5DE08(ctx, base);
	// 82E3CBF4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E3CBF8: 807A000C  lwz r3, 0xc(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3CBFC: 4BF2159D  bl 0x82d5e198
	ctx.lr = 0x82E3CC00;
	sub_82D5E198(ctx, base);
	// 82E3CC00: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E3CC04: 40990014  ble cr6, 0x82e3cc18
	if !ctx.cr[6].gt {
	pc = 0x82E3CC18; continue 'dispatch;
	}
	// 82E3CC08: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E3CC0C: 809B0000  lwz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3CC10: 807A000C  lwz r3, 0xc(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3CC14: 4BF211F5  bl 0x82d5de08
	ctx.lr = 0x82E3CC18;
	sub_82D5DE08(ctx, base);
	// 82E3CC18: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E3CC1C: 807A000C  lwz r3, 0xc(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3CC20: 4BF21579  bl 0x82d5e198
	ctx.lr = 0x82E3CC24;
	sub_82D5E198(ctx, base);
	// 82E3CC24: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82E3CC28: 40990054  ble cr6, 0x82e3cc7c
	if !ctx.cr[6].gt {
	pc = 0x82E3CC7C; continue 'dispatch;
	}
	// 82E3CC2C: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82E3CC30: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E3CC34: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E3CC38: 807A000C  lwz r3, 0xc(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3CC3C: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3CC40: 7D5F502E  lwzx r10, r31, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E3CC44: 7FCA5850  subf r30, r10, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E3CC48: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E3CC4C: 4BF2154D  bl 0x82d5e198
	ctx.lr = 0x82E3CC50;
	sub_82D5E198(ctx, base);
	// 82E3CC50: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E3CC54: 40990018  ble cr6, 0x82e3cc6c
	if !ctx.cr[6].gt {
	pc = 0x82E3CC6C; continue 'dispatch;
	}
	// 82E3CC58: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E3CC5C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E3CC60: 807A000C  lwz r3, 0xc(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3CC64: 7C9F582E  lwzx r4, r31, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3CC68: 4BF211A1  bl 0x82d5de08
	ctx.lr = 0x82E3CC6C;
	sub_82D5DE08(ctx, base);
	// 82E3CC6C: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 82E3CC70: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E3CC74: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E3CC78: 409AFFB8  bne cr6, 0x82e3cc30
	if !ctx.cr[6].eq {
	pc = 0x82E3CC30; continue 'dispatch;
	}
	// 82E3CC7C: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E3CC80: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3CC84: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E3CC88: 409A0020  bne cr6, 0x82e3cca8
	if !ctx.cr[6].eq {
	pc = 0x82E3CCA8; continue 'dispatch;
	}
	// 82E3CC8C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3CC90: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E3CC94: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E3CC98: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E3CC9C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E3CCA0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E3CCA4: 4BF18625  bl 0x82d552c8
	ctx.lr = 0x82E3CCA8;
	sub_82D552C8(ctx, base);
	// 82E3CCA8: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E3CCAC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3CCB0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E3CCB4: 409A0020  bne cr6, 0x82e3ccd4
	if !ctx.cr[6].eq {
	pc = 0x82E3CCD4; continue 'dispatch;
	}
	// 82E3CCB8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3CCBC: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E3CCC0: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E3CCC4: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E3CCC8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E3CCCC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E3CCD0: 4BF185F9  bl 0x82d552c8
	ctx.lr = 0x82E3CCD4;
	sub_82D552C8(ctx, base);
	// 82E3CCD4: 816100A0  lwz r11, 0xa0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82E3CCD8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3CCDC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E3CCE0: 409A002C  bne cr6, 0x82e3cd0c
	if !ctx.cr[6].eq {
	pc = 0x82E3CD0C; continue 'dispatch;
	}
	// 82E3CCE4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3CCE8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E3CCEC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3CCF0: 80810098  lwz r4, 0x98(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82E3CCF4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E3CCF8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E3CCFC: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E3CD00: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E3CD04: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E3CD08: 4BF185C1  bl 0x82d552c8
	ctx.lr = 0x82E3CD0C;
	sub_82D552C8(ctx, base);
	// 82E3CD0C: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82E3CD10: 4BE6C73C  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3CD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3CD18 size=96
    let mut pc: u32 = 0x82E3CD18;
    'dispatch: loop {
        match pc {
            0x82E3CD18 => {
    //   block [0x82E3CD18..0x82E3CD78)
	// 82E3CD18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3CD1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E3CD20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E3CD24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3CD28: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3CD2C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E3CD30: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 82E3CD34: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82E3CD38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3CD3C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3CD40: 4BF18509  bl 0x82d55248
	ctx.lr = 0x82E3CD44;
	sub_82D55248(ctx, base);
	// 82E3CD44: 39600044  li r11, 0x44
	ctx.r[11].s64 = 68;
	// 82E3CD48: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E3CD4C: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E3CD50: 4BFFF6B1  bl 0x82e3c400
	ctx.lr = 0x82E3CD54;
	sub_82E3C400(ctx, base);
	// 82E3CD54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E3CD58: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82E3CD5C: 409A0008  bne cr6, 0x82e3cd64
	if !ctx.cr[6].eq {
	pc = 0x82E3CD64; continue 'dispatch;
	}
	// 82E3CD60: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E3CD64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E3CD68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3CD6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E3CD70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E3CD74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3CD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3CD78 size=64
    let mut pc: u32 = 0x82E3CD78;
    'dispatch: loop {
        match pc {
            0x82E3CD78 => {
    //   block [0x82E3CD78..0x82E3CDB8)
	// 82E3CD78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3CD7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E3CD80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3CD84: 3D6082E4  lis r11, -0x7d1c
	ctx.r[11].s64 = -2098987008;
	// 82E3CD88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E3CD8C: 38ABCD18  addi r5, r11, -0x32e8
	ctx.r[5].s64 = ctx.r[11].s64 + -13032;
	// 82E3CD90: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E3CD94: 388A5AF4  addi r4, r10, 0x5af4
	ctx.r[4].s64 = ctx.r[10].s64 + 23284;
	// 82E3CD98: 806BB068  lwz r3, -0x4f98(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20376 as u32) ) } as u64;
	// 82E3CD9C: 4BFFBEFD  bl 0x82e38c98
	ctx.lr = 0x82E3CDA0;
	sub_82E38C98(ctx, base);
	// 82E3CDA0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E3CDA4: 906BB070  stw r3, -0x4f90(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-20368 as u32), ctx.r[3].u32 ) };
	// 82E3CDA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E3CDAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3CDB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E3CDB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3CDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3CDB8 size=12
    let mut pc: u32 = 0x82E3CDB8;
    'dispatch: loop {
        match pc {
            0x82E3CDB8 => {
    //   block [0x82E3CDB8..0x82E3CDC4)
	// 82E3CDB8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E3CDBC: 806BB070  lwz r3, -0x4f90(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20368 as u32) ) } as u64;
	// 82E3CDC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3CDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3CDC8 size=8
    let mut pc: u32 = 0x82E3CDC8;
    'dispatch: loop {
        match pc {
            0x82E3CDC8 => {
    //   block [0x82E3CDC8..0x82E3CDD0)
	// 82E3CDC8: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82E3CDCC: 48000004  b 0x82e3cdd0
	sub_82E3CDD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3CDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3CDD0 size=100
    let mut pc: u32 = 0x82E3CDD0;
    'dispatch: loop {
        match pc {
            0x82E3CDD0 => {
    //   block [0x82E3CDD0..0x82E3CE34)
	// 82E3CDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3CDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E3CDD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E3CDDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E3CDE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3CDE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3CDE8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E3CDEC: 4BFFF535  bl 0x82e3c320
	ctx.lr = 0x82E3CDF0;
	sub_82E3C320(ctx, base);
	// 82E3CDF0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E3CDF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3CDF8: 419A0020  beq cr6, 0x82e3ce18
	if ctx.cr[6].eq {
	pc = 0x82E3CE18; continue 'dispatch;
	}
	// 82E3CDFC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3CE00: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E3CE04: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 82E3CE08: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3CE0C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E3CE10: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3CE14: 4BF184B5  bl 0x82d552c8
	ctx.lr = 0x82E3CE18;
	sub_82D552C8(ctx, base);
	// 82E3CE18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3CE1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E3CE20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3CE24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E3CE28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E3CE2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E3CE30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3CE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3CE38 size=28
    let mut pc: u32 = 0x82E3CE38;
    'dispatch: loop {
        match pc {
            0x82E3CE38 => {
    //   block [0x82E3CE38..0x82E3CE54)
	// 82E3CE38: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E3CE3C: 8063FFF8  lwz r3, -8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3CE40: 80EBB074  lwz r7, -0x4f8c(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20364 as u32) ) } as u64;
	// 82E3CE44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3CE48: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3CE4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E3CE50: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3CE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3CE58 size=28
    let mut pc: u32 = 0x82E3CE58;
    'dispatch: loop {
        match pc {
            0x82E3CE58 => {
    //   block [0x82E3CE58..0x82E3CE74)
	// 82E3CE58: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E3CE5C: 8063FFF8  lwz r3, -8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3CE60: 80EBB074  lwz r7, -0x4f8c(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20364 as u32) ) } as u64;
	// 82E3CE64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3CE68: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3CE6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E3CE70: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3CE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3CE78 size=28
    let mut pc: u32 = 0x82E3CE78;
    'dispatch: loop {
        match pc {
            0x82E3CE78 => {
    //   block [0x82E3CE78..0x82E3CE94)
	// 82E3CE78: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E3CE7C: 8063FFF8  lwz r3, -8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3CE80: 80EBB074  lwz r7, -0x4f8c(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20364 as u32) ) } as u64;
	// 82E3CE84: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3CE88: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E3CE8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E3CE90: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3CE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3CE98 size=28
    let mut pc: u32 = 0x82E3CE98;
    'dispatch: loop {
        match pc {
            0x82E3CE98 => {
    //   block [0x82E3CE98..0x82E3CEB4)
	// 82E3CE98: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E3CE9C: 8063FFF8  lwz r3, -8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3CEA0: 80CBB074  lwz r6, -0x4f8c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20364 as u32) ) } as u64;
	// 82E3CEA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3CEA8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E3CEAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E3CEB0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3CEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3CEB8 size=28
    let mut pc: u32 = 0x82E3CEB8;
    'dispatch: loop {
        match pc {
            0x82E3CEB8 => {
    //   block [0x82E3CEB8..0x82E3CED4)
	// 82E3CEB8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E3CEBC: 8063FFF8  lwz r3, -8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3CEC0: 80CBB074  lwz r6, -0x4f8c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20364 as u32) ) } as u64;
	// 82E3CEC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3CEC8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3CECC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E3CED0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3CED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3CED8 size=28
    let mut pc: u32 = 0x82E3CED8;
    'dispatch: loop {
        match pc {
            0x82E3CED8 => {
    //   block [0x82E3CED8..0x82E3CEF4)
	// 82E3CED8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E3CEDC: 8063FFF8  lwz r3, -8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3CEE0: 80CBB074  lwz r6, -0x4f8c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20364 as u32) ) } as u64;
	// 82E3CEE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3CEE8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E3CEEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E3CEF0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3CEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3CEF8 size=28
    let mut pc: u32 = 0x82E3CEF8;
    'dispatch: loop {
        match pc {
            0x82E3CEF8 => {
    //   block [0x82E3CEF8..0x82E3CF14)
	// 82E3CEF8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E3CEFC: 8063FFF8  lwz r3, -8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3CF00: 80ABB074  lwz r5, -0x4f8c(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20364 as u32) ) } as u64;
	// 82E3CF04: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3CF08: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E3CF0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E3CF10: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3CF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3CF18 size=20
    let mut pc: u32 = 0x82E3CF18;
    'dispatch: loop {
        match pc {
            0x82E3CF18 => {
    //   block [0x82E3CF18..0x82E3CF2C)
	// 82E3CF18: 8063FFF8  lwz r3, -8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3CF1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3CF20: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E3CF24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E3CF28: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3CF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3CF30 size=28
    let mut pc: u32 = 0x82E3CF30;
    'dispatch: loop {
        match pc {
            0x82E3CF30 => {
    //   block [0x82E3CF30..0x82E3CF4C)
	// 82E3CF30: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E3CF34: 8063FFF8  lwz r3, -8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3CF38: 80CBB074  lwz r6, -0x4f8c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20364 as u32) ) } as u64;
	// 82E3CF3C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3CF40: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E3CF44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E3CF48: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3CF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3CF50 size=28
    let mut pc: u32 = 0x82E3CF50;
    'dispatch: loop {
        match pc {
            0x82E3CF50 => {
    //   block [0x82E3CF50..0x82E3CF6C)
	// 82E3CF50: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E3CF54: 8063FFF8  lwz r3, -8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3CF58: 80EBB074  lwz r7, -0x4f8c(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20364 as u32) ) } as u64;
	// 82E3CF5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3CF60: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E3CF64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E3CF68: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3CF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3CF70 size=28
    let mut pc: u32 = 0x82E3CF70;
    'dispatch: loop {
        match pc {
            0x82E3CF70 => {
    //   block [0x82E3CF70..0x82E3CF8C)
	// 82E3CF70: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E3CF74: 8063FFF8  lwz r3, -8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3CF78: 80CBB074  lwz r6, -0x4f8c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20364 as u32) ) } as u64;
	// 82E3CF7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3CF80: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E3CF84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E3CF88: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3CF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3CF90 size=28
    let mut pc: u32 = 0x82E3CF90;
    'dispatch: loop {
        match pc {
            0x82E3CF90 => {
    //   block [0x82E3CF90..0x82E3CFAC)
	// 82E3CF90: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E3CF94: 8063FFF8  lwz r3, -8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3CF98: 80EBB074  lwz r7, -0x4f8c(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20364 as u32) ) } as u64;
	// 82E3CF9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3CFA0: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E3CFA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E3CFA8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3CFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3CFB0 size=20
    let mut pc: u32 = 0x82E3CFB0;
    'dispatch: loop {
        match pc {
            0x82E3CFB0 => {
    //   block [0x82E3CFB0..0x82E3CFC4)
	// 82E3CFB0: 8063FFF8  lwz r3, -8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3CFB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3CFB8: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E3CFBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E3CFC0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3CFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3CFC8 size=20
    let mut pc: u32 = 0x82E3CFC8;
    'dispatch: loop {
        match pc {
            0x82E3CFC8 => {
    //   block [0x82E3CFC8..0x82E3CFDC)
	// 82E3CFC8: 8063FFF8  lwz r3, -8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3CFCC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3CFD0: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E3CFD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E3CFD8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3CFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3CFE0 size=168
    let mut pc: u32 = 0x82E3CFE0;
    'dispatch: loop {
        match pc {
            0x82E3CFE0 => {
    //   block [0x82E3CFE0..0x82E3D088)
	// 82E3CFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3CFE4: 4BE6C429  bl 0x82ca940c
	ctx.lr = 0x82E3CFE8;
	sub_82CA93D0(ctx, base);
	// 82E3CFE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3CFEC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E3CFF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E3CFF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3CFF8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E3CFFC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E3D000: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E3D004: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E3D008: 396B58F4  addi r11, r11, 0x58f4
	ctx.r[11].s64 = ctx.r[11].s64 + 22772;
	// 82E3D00C: 394A59F4  addi r10, r10, 0x59f4
	ctx.r[10].s64 = ctx.r[10].s64 + 23028;
	// 82E3D010: B0DF0006  sth r6, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82E3D014: 39295B9C  addi r9, r9, 0x5b9c
	ctx.r[9].s64 = ctx.r[9].s64 + 23452;
	// 82E3D018: 98DF000C  stb r6, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u8 ) };
	// 82E3D01C: 39085B7C  addi r8, r8, 0x5b7c
	ctx.r[8].s64 = ctx.r[8].s64 + 23420;
	// 82E3D020: 38E75B34  addi r7, r7, 0x5b34
	ctx.r[7].s64 = ctx.r[7].s64 + 23348;
	// 82E3D024: 3FC08334  lis r30, -0x7ccc
	ctx.r[30].s64 = -2093744128;
	// 82E3D028: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E3D02C: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82E3D030: 3BBF0020  addi r29, r31, 0x20
	ctx.r[29].s64 = ctx.r[31].s64 + 32;
	// 82E3D034: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E3D038: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82E3D03C: 90FF0020  stw r7, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[7].u32 ) };
	// 82E3D040: 817EB074  lwz r11, -0x4f8c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-20364 as u32) ) } as u64;
	// 82E3D044: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3D048: 409A0024  bne cr6, 0x82e3d06c
	if !ctx.cr[6].eq {
	pc = 0x82E3D06C; continue 'dispatch;
	}
	// 82E3D04C: 3D6082E4  lis r11, -0x7d1c
	ctx.r[11].s64 = -2098987008;
	// 82E3D050: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E3D054: 38ABD118  addi r5, r11, -0x2ee8
	ctx.r[5].s64 = ctx.r[11].s64 + -12008;
	// 82E3D058: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E3D05C: 388A5570  addi r4, r10, 0x5570
	ctx.r[4].s64 = ctx.r[10].s64 + 21872;
	// 82E3D060: 806BB068  lwz r3, -0x4f98(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20376 as u32) ) } as u64;
	// 82E3D064: 4BFFBC35  bl 0x82e38c98
	ctx.lr = 0x82E3D068;
	sub_82E38C98(ctx, base);
	// 82E3D068: 907EB074  stw r3, -0x4f8c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-20364 as u32), ctx.r[3].u32 ) };
	// 82E3D06C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E3D070: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E3D074: 806B1C44  lwz r3, 0x1c44(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7236 as u32) ) } as u64;
	// 82E3D078: 48008DD1  bl 0x82e45e48
	ctx.lr = 0x82E3D07C;
	sub_82E45E48(ctx, base);
	// 82E3D07C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3D080: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E3D084: 4BE6C3D8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3D088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3D088 size=140
    let mut pc: u32 = 0x82E3D088;
    'dispatch: loop {
        match pc {
            0x82E3D088 => {
    //   block [0x82E3D088..0x82E3D114)
	// 82E3D088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3D08C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E3D090: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E3D094: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E3D098: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3D09C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E3D0A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3D0A4: 396B5B9C  addi r11, r11, 0x5b9c
	ctx.r[11].s64 = ctx.r[11].s64 + 23452;
	// 82E3D0A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E3D0AC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E3D0B0: 394A5B7C  addi r10, r10, 0x5b7c
	ctx.r[10].s64 = ctx.r[10].s64 + 23420;
	// 82E3D0B4: 39295B34  addi r9, r9, 0x5b34
	ctx.r[9].s64 = ctx.r[9].s64 + 23348;
	// 82E3D0B8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E3D0BC: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E3D0C0: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82E3D0C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E3D0C8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E3D0CC: 913F0020  stw r9, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82E3D0D0: 806B1C44  lwz r3, 0x1c44(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7236 as u32) ) } as u64;
	// 82E3D0D4: 48007BDD  bl 0x82e44cb0
	ctx.lr = 0x82E3D0D8;
	sub_82E44CB0(ctx, base);
	// 82E3D0D8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E3D0DC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E3D0E0: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82E3D0E4: 396B59F4  addi r11, r11, 0x59f4
	ctx.r[11].s64 = ctx.r[11].s64 + 23028;
	// 82E3D0E8: 394A58F4  addi r10, r10, 0x58f4
	ctx.r[10].s64 = ctx.r[10].s64 + 22772;
	// 82E3D0EC: 392939E0  addi r9, r9, 0x39e0
	ctx.r[9].s64 = ctx.r[9].s64 + 14816;
	// 82E3D0F0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E3D0F4: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E3D0F8: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E3D0FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E3D100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3D104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E3D108: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E3D10C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E3D110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3D118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3D118 size=80
    let mut pc: u32 = 0x82E3D118;
    'dispatch: loop {
        match pc {
            0x82E3D118 => {
    //   block [0x82E3D118..0x82E3D168)
	// 82E3D118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3D11C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E3D120: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3D124: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3D128: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E3D12C: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 82E3D130: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 82E3D134: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3D138: 4BF18111  bl 0x82d55248
	ctx.lr = 0x82E3D13C;
	sub_82D55248(ctx, base);
	// 82E3D13C: 39600024  li r11, 0x24
	ctx.r[11].s64 = 36;
	// 82E3D140: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E3D144: 4BFFFE9D  bl 0x82e3cfe0
	ctx.lr = 0x82E3D148;
	sub_82E3CFE0(ctx, base);
	// 82E3D148: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E3D14C: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82E3D150: 409A0008  bne cr6, 0x82e3d158
	if !ctx.cr[6].eq {
	pc = 0x82E3D158; continue 'dispatch;
	}
	// 82E3D154: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E3D158: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E3D15C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3D160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E3D164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3D168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3D168 size=64
    let mut pc: u32 = 0x82E3D168;
    'dispatch: loop {
        match pc {
            0x82E3D168 => {
    //   block [0x82E3D168..0x82E3D1A8)
	// 82E3D168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3D16C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E3D170: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3D174: 3D6082E4  lis r11, -0x7d1c
	ctx.r[11].s64 = -2098987008;
	// 82E3D178: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E3D17C: 38ABD118  addi r5, r11, -0x2ee8
	ctx.r[5].s64 = ctx.r[11].s64 + -12008;
	// 82E3D180: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E3D184: 388A5570  addi r4, r10, 0x5570
	ctx.r[4].s64 = ctx.r[10].s64 + 21872;
	// 82E3D188: 806BB068  lwz r3, -0x4f98(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20376 as u32) ) } as u64;
	// 82E3D18C: 4BFFBB0D  bl 0x82e38c98
	ctx.lr = 0x82E3D190;
	sub_82E38C98(ctx, base);
	// 82E3D190: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E3D194: 906BB074  stw r3, -0x4f8c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-20364 as u32), ctx.r[3].u32 ) };
	// 82E3D198: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E3D19C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3D1A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E3D1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3D1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3D1A8 size=12
    let mut pc: u32 = 0x82E3D1A8;
    'dispatch: loop {
        match pc {
            0x82E3D1A8 => {
    //   block [0x82E3D1A8..0x82E3D1B4)
	// 82E3D1A8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82E3D1AC: 806BB074  lwz r3, -0x4f8c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20364 as u32) ) } as u64;
	// 82E3D1B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3D1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3D1B8 size=8
    let mut pc: u32 = 0x82E3D1B8;
    'dispatch: loop {
        match pc {
            0x82E3D1B8 => {
    //   block [0x82E3D1B8..0x82E3D1C0)
	// 82E3D1B8: 3863FFE0  addi r3, r3, -0x20
	ctx.r[3].s64 = ctx.r[3].s64 + -32;
	// 82E3D1BC: 4800000C  b 0x82e3d1c8
	sub_82E3D1C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3D1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3D1C0 size=8
    let mut pc: u32 = 0x82E3D1C0;
    'dispatch: loop {
        match pc {
            0x82E3D1C0 => {
    //   block [0x82E3D1C0..0x82E3D1C8)
	// 82E3D1C0: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82E3D1C4: 48000004  b 0x82e3d1c8
	sub_82E3D1C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3D1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3D1C8 size=100
    let mut pc: u32 = 0x82E3D1C8;
    'dispatch: loop {
        match pc {
            0x82E3D1C8 => {
    //   block [0x82E3D1C8..0x82E3D22C)
	// 82E3D1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3D1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E3D1D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E3D1D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E3D1D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3D1DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3D1E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E3D1E4: 4BFFFEA5  bl 0x82e3d088
	ctx.lr = 0x82E3D1E8;
	sub_82E3D088(ctx, base);
	// 82E3D1E8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E3D1EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3D1F0: 419A0020  beq cr6, 0x82e3d210
	if ctx.cr[6].eq {
	pc = 0x82E3D210; continue 'dispatch;
	}
	// 82E3D1F4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3D1F8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E3D1FC: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 82E3D200: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3D204: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E3D208: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3D20C: 4BF180BD  bl 0x82d552c8
	ctx.lr = 0x82E3D210;
	sub_82D552C8(ctx, base);
	// 82E3D210: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3D214: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E3D218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3D21C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E3D220: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E3D224: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E3D228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3D230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3D230 size=8
    let mut pc: u32 = 0x82E3D230;
    'dispatch: loop {
        match pc {
            0x82E3D230 => {
    //   block [0x82E3D230..0x82E3D238)
	// 82E3D230: 38630060  addi r3, r3, 0x60
	ctx.r[3].s64 = ctx.r[3].s64 + 96;
	// 82E3D234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3D238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3D238 size=8
    let mut pc: u32 = 0x82E3D238;
    'dispatch: loop {
        match pc {
            0x82E3D238 => {
    //   block [0x82E3D238..0x82E3D240)
	// 82E3D238: 38630070  addi r3, r3, 0x70
	ctx.r[3].s64 = ctx.r[3].s64 + 112;
	// 82E3D23C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3D240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3D240 size=8
    let mut pc: u32 = 0x82E3D240;
    'dispatch: loop {
        match pc {
            0x82E3D240 => {
    //   block [0x82E3D240..0x82E3D248)
	// 82E3D240: 38630080  addi r3, r3, 0x80
	ctx.r[3].s64 = ctx.r[3].s64 + 128;
	// 82E3D244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3D248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3D248 size=8
    let mut pc: u32 = 0x82E3D248;
    'dispatch: loop {
        match pc {
            0x82E3D248 => {
    //   block [0x82E3D248..0x82E3D250)
	// 82E3D248: 38630090  addi r3, r3, 0x90
	ctx.r[3].s64 = ctx.r[3].s64 + 144;
	// 82E3D24C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3D250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3D250 size=192
    let mut pc: u32 = 0x82E3D250;
    'dispatch: loop {
        match pc {
            0x82E3D250 => {
    //   block [0x82E3D250..0x82E3D310)
	// 82E3D250: 38E30080  addi r7, r3, 0x80
	ctx.r[7].s64 = ctx.r[3].s64 + 128;
	// 82E3D254: 38C30060  addi r6, r3, 0x60
	ctx.r[6].s64 = ctx.r[3].s64 + 96;
	// 82E3D258: 39030070  addi r8, r3, 0x70
	ctx.r[8].s64 = ctx.r[3].s64 + 112;
	// 82E3D25C: 39640010  addi r11, r4, 0x10
	ctx.r[11].s64 = ctx.r[4].s64 + 16;
	// 82E3D260: 39440020  addi r10, r4, 0x20
	ctx.r[10].s64 = ctx.r[4].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3D310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3D310 size=52
    let mut pc: u32 = 0x82E3D310;
    'dispatch: loop {
        match pc {
            0x82E3D310 => {
    //   block [0x82E3D310..0x82E3D344)
	// 82E3D310: 39600060  li r11, 0x60
	ctx.r[11].s64 = 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3D348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3D348 size=280
    let mut pc: u32 = 0x82E3D348;
    'dispatch: loop {
        match pc {
            0x82E3D348 => {
    //   block [0x82E3D348..0x82E3D460)
	// 82E3D348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3D34C: 4BE6C0B9  bl 0x82ca9404
	ctx.lr = 0x82E3D350;
	sub_82CA93D0(ctx, base);
	// 82E3D350: 3941FFC0  addi r10, r1, -0x40
	ctx.r[10].s64 = ctx.r[1].s64 + -64;
	// 82E3D354: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E3D358: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82E3D35C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82E3D360: 3BE00007  li r31, 7
	ctx.r[31].s64 = 7;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3D460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3D460 size=196
    let mut pc: u32 = 0x82E3D460;
    'dispatch: loop {
        match pc {
            0x82E3D460 => {
    //   block [0x82E3D460..0x82E3D524)
	// 82E3D460: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82E3D464: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 82E3D468: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E3D46C: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82E3D470: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82E3D474: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3D528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3D528 size=264
    let mut pc: u32 = 0x82E3D528;
    'dispatch: loop {
        match pc {
            0x82E3D528 => {
    //   block [0x82E3D528..0x82E3D630)
	// 82E3D528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3D52C: 4BE6BEE1  bl 0x82ca940c
	ctx.lr = 0x82E3D530;
	sub_82CA93D0(ctx, base);
	// 82E3D530: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3D534: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E3D538: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E3D53C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3D540: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3D544: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 82E3D548: 40980020  bge cr6, 0x82e3d568
	if !ctx.cr[6].lt {
	pc = 0x82E3D568; continue 'dispatch;
	}
	// 82E3D54C: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E3D550: 2F04000C  cmpwi cr6, r4, 0xc
	ctx.cr[6].compare_i32(ctx.r[4].s32, 12, &mut ctx.xer);
	// 82E3D554: 41990008  bgt cr6, 0x82e3d55c
	if ctx.cr[6].gt {
	pc = 0x82E3D55C; continue 'dispatch;
	}
	// 82E3D558: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82E3D55C: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82E3D560: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3D564: 4BF199AD  bl 0x82d56f10
	ctx.lr = 0x82E3D568;
	sub_82D56F10(ctx, base);
	// 82E3D568: 3960000C  li r11, 0xc
	ctx.r[11].s64 = 12;
	// 82E3D56C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E3D570: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E3D574: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E3D578: 4BFFFCD9  bl 0x82e3d250
	ctx.lr = 0x82E3D57C;
	sub_82E3D250(ctx, base);
	// 82E3D57C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82E3D580: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3D584: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82E3D588: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 82E3D58C: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 82E3D590: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3D630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3D630 size=648
    let mut pc: u32 = 0x82E3D630;
    'dispatch: loop {
        match pc {
            0x82E3D630 => {
    //   block [0x82E3D630..0x82E3D8B8)
	// 82E3D630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3D634: 4BE6BDC5  bl 0x82ca93f8
	ctx.lr = 0x82E3D638;
	sub_82CA93D0(ctx, base);
	// 82E3D638: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3D63C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3D640: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E3D644: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E3D648: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3D64C: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3D650: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3D654: 419A001C  beq cr6, 0x82e3d670
	if ctx.cr[6].eq {
	pc = 0x82E3D670; continue 'dispatch;
	}
	// 82E3D658: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E3D65C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E3D660: 91430054  stw r10, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E3D664: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3D668: 91430050  stw r10, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E3D66C: 48000010  b 0x82e3d67c
	pc = 0x82E3D67C; continue 'dispatch;
	// 82E3D670: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82E3D674: 4BF179DD  bl 0x82d55050
	ctx.lr = 0x82E3D678;
	sub_82D55050(ctx, base);
	// 82E3D678: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E3D67C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3D680: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82E3D684: 419A0028  beq cr6, 0x82e3d6ac
	if ctx.cr[6].eq {
	pc = 0x82E3D6AC; continue 'dispatch;
	}
	// 82E3D688: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E3D68C: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82E3D690: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82E3D694: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82E3D698: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E3D69C: 932B000C  stw r25, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[25].u32 ) };
	// 82E3D6A0: 932B0010  stw r25, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[25].u32 ) };
	// 82E3D6A4: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82E3D6A8: 48000008  b 0x82e3d6b0
	pc = 0x82E3D6B0; continue 'dispatch;
	// 82E3D6AC: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82E3D6B0: 93FC0008  stw r31, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82E3D6B4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3D6B8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3D6BC: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 82E3D6C0: 40980020  bge cr6, 0x82e3d6e0
	if !ctx.cr[6].lt {
	pc = 0x82E3D6E0; continue 'dispatch;
	}
	// 82E3D6C4: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E3D6C8: 2F040005  cmpwi cr6, r4, 5
	ctx.cr[6].compare_i32(ctx.r[4].s32, 5, &mut ctx.xer);
	// 82E3D6CC: 41990008  bgt cr6, 0x82e3d6d4
	if ctx.cr[6].gt {
	pc = 0x82E3D6D4; continue 'dispatch;
	}
	// 82E3D6D0: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82E3D6D4: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82E3D6D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3D6DC: 4BF19835  bl 0x82d56f10
	ctx.lr = 0x82E3D6E0;
	sub_82D56F10(ctx, base);
	// 82E3D6E0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82E3D6E4: 39600070  li r11, 0x70
	ctx.r[11].s64 = 112;
	// 82E3D6E8: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 82E3D6EC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E3D6F0: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E3D6F4: 813C0008  lwz r9, 8(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3D8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3D8B8 size=24
    let mut pc: u32 = 0x82E3D8B8;
    'dispatch: loop {
        match pc {
            0x82E3D8B8 => {
    //   block [0x82E3D8B8..0x82E3D8D0)
	// 82E3D8B8: 39640060  addi r11, r4, 0x60
	ctx.r[11].s64 = ctx.r[4].s64 + 96;
	// 82E3D8BC: E94B0000  ld r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82E3D8C0: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82E3D8C4: F9430000  std r10, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82E3D8C8: F9630008  std r11, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82E3D8CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3D8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3D8D0 size=24
    let mut pc: u32 = 0x82E3D8D0;
    'dispatch: loop {
        match pc {
            0x82E3D8D0 => {
    //   block [0x82E3D8D0..0x82E3D8E8)
	// 82E3D8D0: 39640070  addi r11, r4, 0x70
	ctx.r[11].s64 = ctx.r[4].s64 + 112;
	// 82E3D8D4: E94B0000  ld r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82E3D8D8: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82E3D8DC: F9430000  std r10, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82E3D8E0: F9630008  std r11, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82E3D8E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3D8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E3D8E8 size=8
    let mut pc: u32 = 0x82E3D8E8;
    'dispatch: loop {
        match pc {
            0x82E3D8E8 => {
    //   block [0x82E3D8E8..0x82E3D8F0)
	// 82E3D8E8: C0230090  lfs f1, 0x90(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E3D8EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3D8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E3D8F0 size=8
    let mut pc: u32 = 0x82E3D8F0;
    'dispatch: loop {
        match pc {
            0x82E3D8F0 => {
    //   block [0x82E3D8F0..0x82E3D8F8)
	// 82E3D8F0: C0230094  lfs f1, 0x94(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E3D8F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3D8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E3D8F8 size=8
    let mut pc: u32 = 0x82E3D8F8;
    'dispatch: loop {
        match pc {
            0x82E3D8F8 => {
    //   block [0x82E3D8F8..0x82E3D900)
	// 82E3D8F8: C0230098  lfs f1, 0x98(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E3D8FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3D900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3D900 size=8
    let mut pc: u32 = 0x82E3D900;
    'dispatch: loop {
        match pc {
            0x82E3D900 => {
    //   block [0x82E3D900..0x82E3D908)
	// 82E3D900: 8063009C  lwz r3, 0x9c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 82E3D904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3D908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3D908 size=24
    let mut pc: u32 = 0x82E3D908;
    'dispatch: loop {
        match pc {
            0x82E3D908 => {
    //   block [0x82E3D908..0x82E3D920)
	// 82E3D908: 39640080  addi r11, r4, 0x80
	ctx.r[11].s64 = ctx.r[4].s64 + 128;
	// 82E3D90C: E94B0000  ld r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82E3D910: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82E3D914: F9430000  std r10, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82E3D918: F9630008  std r11, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82E3D91C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3D920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3D920 size=56
    let mut pc: u32 = 0x82E3D920;
    'dispatch: loop {
        match pc {
            0x82E3D920 => {
    //   block [0x82E3D920..0x82E3D958)
	// 82E3D920: 39600080  li r11, 0x80
	ctx.r[11].s64 = 128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3D958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3D958 size=260
    let mut pc: u32 = 0x82E3D958;
    'dispatch: loop {
        match pc {
            0x82E3D958 => {
    //   block [0x82E3D958..0x82E3DA5C)
	// 82E3D958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3D95C: 4BE6BAAD  bl 0x82ca9408
	ctx.lr = 0x82E3D960;
	sub_82CA93D0(ctx, base);
	// 82E3D960: 3921FFC0  addi r9, r1, -0x40
	ctx.r[9].s64 = ctx.r[1].s64 + -64;
	// 82E3D964: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82E3D968: 3BE00005  li r31, 5
	ctx.r[31].s64 = 5;
	// 82E3D96C: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82E3D970: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3DA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3DA60 size=204
    let mut pc: u32 = 0x82E3DA60;
    'dispatch: loop {
        match pc {
            0x82E3DA60 => {
    //   block [0x82E3DA60..0x82E3DB2C)
	// 82E3DA60: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82E3DA64: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 82E3DA68: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E3DA6C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82E3DA70: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82E3DA74: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3DB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E3DB30 size=396
    let mut pc: u32 = 0x82E3DB30;
    'dispatch: loop {
        match pc {
            0x82E3DB30 => {
    //   block [0x82E3DB30..0x82E3DCBC)
	// 82E3DB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3DB34: 4BE6B8CD  bl 0x82ca9400
	ctx.lr = 0x82E3DB38;
	sub_82CA93D0(ctx, base);
	// 82E3DB38: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82E3DB3C: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3DB40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3DB44: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E3DB48: 3BDF0060  addi r30, r31, 0x60
	ctx.r[30].s64 = ctx.r[31].s64 + 96;
	// 82E3DB4C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E3DB50: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E3DB54: 817F009C  lwz r11, 0x9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82E3DB58: C03F0090  lfs f1, 0x90(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E3DB5C: C01F0094  lfs f0, 0x94(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3DB60: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82E3DB64: EC000828  fsubs f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 82E3DB68: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82E3DB6C: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E3DB70: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82E3DB74: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82E3DB78: EFE06824  fdivs f31, f0, f13
	ctx.f[31].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82E3DB7C: 4BF19CED  bl 0x82d57868
	ctx.lr = 0x82E3DB80;
	sub_82D57868(ctx, base);
	// 82E3DB80: 39600098  li r11, 0x98
	ctx.r[11].s64 = 152;
	// 82E3DB84: 39400070  li r10, 0x70
	ctx.r[10].s64 = 112;
	// 82E3DB88: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E3DB8C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82E3DB90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3DCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3DCC0 size=252
    let mut pc: u32 = 0x82E3DCC0;
    'dispatch: loop {
        match pc {
            0x82E3DCC0 => {
    //   block [0x82E3DCC0..0x82E3DDBC)
	// 82E3DCC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3DCC4: 4BE6B745  bl 0x82ca9408
	ctx.lr = 0x82E3DCC8;
	sub_82CA93D0(ctx, base);
	// 82E3DCC8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3DCCC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E3DCD0: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E3DCD4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E3DCD8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E3DCDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E3DCE0: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82E3DCE4: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82E3DCE8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82E3DCEC: 4BFFFE45  bl 0x82e3db30
	ctx.lr = 0x82E3DCF0;
	sub_82E3DB30(ctx, base);
	// 82E3DCF0: 817E009C  lwz r11, 0x9c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(156 as u32) ) } as u64;
	// 82E3DCF4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3DCF8: 557D083C  slwi r29, r11, 1
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82E3DCFC: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3DD00: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82E3DD04: 40980024  bge cr6, 0x82e3dd28
	if !ctx.cr[6].lt {
	pc = 0x82E3DD28; continue 'dispatch;
	}
	// 82E3DD08: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E3DD0C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E3DD10: 41980008  blt cr6, 0x82e3dd18
	if ctx.cr[6].lt {
	pc = 0x82E3DD18; continue 'dispatch;
	}
	// 82E3DD14: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82E3DD18: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82E3DD1C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E3DD20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3DD24: 4BF191ED  bl 0x82d56f10
	ctx.lr = 0x82E3DD28;
	sub_82D56F10(ctx, base);
	// 82E3DD28: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E3DD2C: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 82E3DD30: 817E009C  lwz r11, 0x9c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(156 as u32) ) } as u64;
	// 82E3DD34: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3DD38: 40990050  ble cr6, 0x82e3dd88
	if !ctx.cr[6].gt {
	pc = 0x82E3DD88; continue 'dispatch;
	}
	// 82E3DD3C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82E3DD40: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82E3DD44: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82E3DD48: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3DD4C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E3DD50: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3DDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3DDC0 size=300
    let mut pc: u32 = 0x82E3DDC0;
    'dispatch: loop {
        match pc {
            0x82E3DDC0 => {
    //   block [0x82E3DDC0..0x82E3DEEC)
	// 82E3DDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3DDC4: 4BE6B649  bl 0x82ca940c
	ctx.lr = 0x82E3DDC8;
	sub_82CA93D0(ctx, base);
	// 82E3DDC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3DDCC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3DDD0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E3DDD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3DDD8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3DDDC: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3DDE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3DDE4: 419A001C  beq cr6, 0x82e3de00
	if ctx.cr[6].eq {
	pc = 0x82E3DE00; continue 'dispatch;
	}
	// 82E3DDE8: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E3DDEC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E3DDF0: 91430054  stw r10, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E3DDF4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3DDF8: 91430050  stw r10, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E3DDFC: 48000010  b 0x82e3de0c
	pc = 0x82E3DE0C; continue 'dispatch;
	// 82E3DE00: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82E3DE04: 4BF1724D  bl 0x82d55050
	ctx.lr = 0x82E3DE08;
	sub_82D55050(ctx, base);
	// 82E3DE08: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E3DE0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3DE10: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E3DE14: 419A0028  beq cr6, 0x82e3de3c
	if ctx.cr[6].eq {
	pc = 0x82E3DE3C; continue 'dispatch;
	}
	// 82E3DE18: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E3DE1C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82E3DE20: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E3DE24: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E3DE28: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E3DE2C: 93AB000C  stw r29, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82E3DE30: 93AB0010  stw r29, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82E3DE34: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82E3DE38: 48000008  b 0x82e3de40
	pc = 0x82E3DE40; continue 'dispatch;
	// 82E3DE3C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E3DE40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3DE44: 909F0008  stw r4, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82E3DE48: 4BFFFCE9  bl 0x82e3db30
	ctx.lr = 0x82E3DE4C;
	sub_82E3DB30(ctx, base);
	// 82E3DE4C: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3DE50: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3DE54: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3DE58: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3DE5C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E3DE60: 409A0010  bne cr6, 0x82e3de70
	if !ctx.cr[6].eq {
	pc = 0x82E3DE70; continue 'dispatch;
	}
	// 82E3DE64: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82E3DE68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E3DE6C: 4BF1912D  bl 0x82d56f98
	ctx.lr = 0x82E3DE70;
	sub_82D56F98(ctx, base);
	// 82E3DE70: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3DE74: 38E00080  li r7, 0x80
	ctx.r[7].s64 = 128;
	// 82E3DE78: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3DE7C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82E3DE80: 55492036  slwi r9, r10, 4
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E3DE84: 38CA0001  addi r6, r10, 1
	ctx.r[6].s64 = ctx.r[10].s64 + 1;
	// 82E3DE88: 7D494214  add r10, r9, r8
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82E3DE8C: 90DE0004  stw r6, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3DEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3DEF0 size=24
    let mut pc: u32 = 0x82E3DEF0;
    'dispatch: loop {
        match pc {
            0x82E3DEF0 => {
    //   block [0x82E3DEF0..0x82E3DF08)
	// 82E3DEF0: 39640060  addi r11, r4, 0x60
	ctx.r[11].s64 = ctx.r[4].s64 + 96;
	// 82E3DEF4: E94B0000  ld r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82E3DEF8: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82E3DEFC: F9430000  std r10, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82E3DF00: F9630008  std r11, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82E3DF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3DF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3DF08 size=24
    let mut pc: u32 = 0x82E3DF08;
    'dispatch: loop {
        match pc {
            0x82E3DF08 => {
    //   block [0x82E3DF08..0x82E3DF20)
	// 82E3DF08: 39640070  addi r11, r4, 0x70
	ctx.r[11].s64 = ctx.r[4].s64 + 112;
	// 82E3DF0C: E94B0000  ld r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82E3DF10: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82E3DF14: F9430000  std r10, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82E3DF18: F9630008  std r11, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82E3DF1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3DF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E3DF20 size=8
    let mut pc: u32 = 0x82E3DF20;
    'dispatch: loop {
        match pc {
            0x82E3DF20 => {
    //   block [0x82E3DF20..0x82E3DF28)
	// 82E3DF20: C0230088  lfs f1, 0x88(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(136 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E3DF24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3DF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E3DF28 size=8
    let mut pc: u32 = 0x82E3DF28;
    'dispatch: loop {
        match pc {
            0x82E3DF28 => {
    //   block [0x82E3DF28..0x82E3DF30)
	// 82E3DF28: C0230084  lfs f1, 0x84(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E3DF2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3DF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3DF30 size=8
    let mut pc: u32 = 0x82E3DF30;
    'dispatch: loop {
        match pc {
            0x82E3DF30 => {
    //   block [0x82E3DF30..0x82E3DF38)
	// 82E3DF30: 80630080  lwz r3, 0x80(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E3DF34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3DF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E3DF38 size=40
    let mut pc: u32 = 0x82E3DF38;
    'dispatch: loop {
        match pc {
            0x82E3DF38 => {
    //   block [0x82E3DF38..0x82E3DF60)
	// 82E3DF38: 39600060  li r11, 0x60
	ctx.r[11].s64 = 96;
	// 82E3DF3C: D0230084  stfs f1, 0x84(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82E3DF40: 39400070  li r10, 0x70
	ctx.r[10].s64 = 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3DF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E3DF60 size=228
    let mut pc: u32 = 0x82E3DF60;
    'dispatch: loop {
        match pc {
            0x82E3DF60 => {
    //   block [0x82E3DF60..0x82E3E044)
	// 82E3DF60: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82E3DF64: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82E3DF68: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E3DF6C: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 82E3DF70: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E3DF74: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E3DF78: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82E3DF7C: C0090C14  lfs f0, 0xc14(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3DF80: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3E048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3E048 size=176
    let mut pc: u32 = 0x82E3E048;
    'dispatch: loop {
        match pc {
            0x82E3E048 => {
    //   block [0x82E3E048..0x82E3E0F8)
	// 82E3E048: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 82E3E04C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E3E050: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82E3E054: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82E3E058: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3E0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E3E0F8 size=588
    let mut pc: u32 = 0x82E3E0F8;
    'dispatch: loop {
        match pc {
            0x82E3E0F8 => {
    //   block [0x82E3E0F8..0x82E3E344)
	// 82E3E0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3E0FC: 4BE6B30D  bl 0x82ca9408
	ctx.lr = 0x82E3E100;
	sub_82CA93D0(ctx, base);
	// 82E3E100: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3E104: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E3E108: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E3E10C: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3E110: 839E0080  lwz r28, 0x80(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E3E114: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3E118: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82E3E11C: 40980024  bge cr6, 0x82e3e140
	if !ctx.cr[6].lt {
	pc = 0x82E3E140; continue 'dispatch;
	}
	// 82E3E120: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E3E124: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E3E128: 41980008  blt cr6, 0x82e3e130
	if ctx.cr[6].lt {
	pc = 0x82E3E130; continue 'dispatch;
	}
	// 82E3E12C: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82E3E130: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82E3E134: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E3E138: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E3E13C: 4BF18DD5  bl 0x82d56f10
	ctx.lr = 0x82E3E140;
	sub_82D56F10(ctx, base);
	// 82E3E140: 3BFE0070  addi r31, r30, 0x70
	ctx.r[31].s64 = ctx.r[30].s64 + 112;
	// 82E3E144: 939D0004  stw r28, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82E3E148: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E3E14C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E3E150: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82E3E154: C01F0000  lfs f0, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3E158: C1BF0004  lfs f13, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E3E15C: FC000210  fabs f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 82E3E160: FDA06A10  fabs f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 82E3E164: C19F0008  lfs f12, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E3E168: FD806210  fabs f12, f12
	ctx.f[12].u64 = ctx.f[12].u64 & !0x8000_0000_0000_0000u64;
	// 82E3E16C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82E3E170: 40980010  bge cr6, 0x82e3e180
	if !ctx.cr[6].lt {
	pc = 0x82E3E180; continue 'dispatch;
	}
	// 82E3E174: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E3E178: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82E3E17C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82E3E180: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 82E3E184: 40980008  bge cr6, 0x82e3e18c
	if !ctx.cr[6].lt {
	pc = 0x82E3E18C; continue 'dispatch;
	}
	// 82E3E188: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82E3E18C: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82E3E190: C03E0084  lfs f1, 0x84(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E3E194: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E3E198: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82E3E19C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E3E1A0: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3E348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3E348 size=336
    let mut pc: u32 = 0x82E3E348;
    'dispatch: loop {
        match pc {
            0x82E3E348 => {
    //   block [0x82E3E348..0x82E3E498)
	// 82E3E348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3E34C: 4BE6B0C1  bl 0x82ca940c
	ctx.lr = 0x82E3E350;
	sub_82CA93D0(ctx, base);
	// 82E3E350: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82E3E354: 9421EF80  stwu r1, -0x1080(r1)
	ea = ctx.r[1].u32.wrapping_add(-4224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3E358: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82E3E35C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E3E360: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E3E364: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E3E368: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E3E36C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E3E370: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E3E374: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E3E378: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 82E3E37C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82E3E380: 4BFFFD79  bl 0x82e3e0f8
	ctx.lr = 0x82E3E384;
	sub_82E3E0F8(ctx, base);
	// 82E3E384: 817E0080  lwz r11, 0x80(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E3E388: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3E38C: 557D103A  slwi r29, r11, 2
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82E3E390: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3E394: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82E3E398: 40980024  bge cr6, 0x82e3e3bc
	if !ctx.cr[6].lt {
	pc = 0x82E3E3BC; continue 'dispatch;
	}
	// 82E3E39C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E3E3A0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E3E3A4: 41980008  blt cr6, 0x82e3e3ac
	if ctx.cr[6].lt {
	pc = 0x82E3E3AC; continue 'dispatch;
	}
	// 82E3E3A8: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82E3E3AC: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82E3E3B0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E3E3B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3E3B8: 4BF18B59  bl 0x82d56f10
	ctx.lr = 0x82E3E3BC;
	sub_82D56F10(ctx, base);
	// 82E3E3BC: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E3E3C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E3E3C4: 815E0080  lwz r10, 0x80(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E3E3C8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E3E3CC: 40990098  ble cr6, 0x82e3e464
	if !ctx.cr[6].gt {
	pc = 0x82E3E464; continue 'dispatch;
	}
	// 82E3E3D0: 38FE0060  addi r7, r30, 0x60
	ctx.r[7].s64 = ctx.r[30].s64 + 96;
	// 82E3E3D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E3E3D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E3E3DC: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3E498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E3E498 size=400
    let mut pc: u32 = 0x82E3E498;
    'dispatch: loop {
        match pc {
            0x82E3E498 => {
    //   block [0x82E3E498..0x82E3E628)
	// 82E3E498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3E49C: 4BE6AF6D  bl 0x82ca9408
	ctx.lr = 0x82E3E4A0;
	sub_82CA93D0(ctx, base);
	// 82E3E4A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3E4A4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3E4A8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E3E4AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3E4B0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3E4B4: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3E4B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3E4BC: 419A001C  beq cr6, 0x82e3e4d8
	if ctx.cr[6].eq {
	pc = 0x82E3E4D8; continue 'dispatch;
	}
	// 82E3E4C0: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E3E4C4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E3E4C8: 91430054  stw r10, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E3E4CC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3E4D0: 91430050  stw r10, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E3E4D4: 48000010  b 0x82e3e4e4
	pc = 0x82E3E4E4; continue 'dispatch;
	// 82E3E4D8: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82E3E4DC: 4BF16B75  bl 0x82d55050
	ctx.lr = 0x82E3E4E0;
	sub_82D55050(ctx, base);
	// 82E3E4E0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E3E4E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3E4E8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E3E4EC: 419A0028  beq cr6, 0x82e3e514
	if ctx.cr[6].eq {
	pc = 0x82E3E514; continue 'dispatch;
	}
	// 82E3E4F0: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E3E4F4: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E3E4F8: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82E3E4FC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E3E500: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E3E504: 938B000C  stw r28, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82E3E508: 938B0010  stw r28, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 82E3E50C: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82E3E510: 48000008  b 0x82e3e518
	pc = 0x82E3E518; continue 'dispatch;
	// 82E3E514: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E3E518: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3E51C: 909F0008  stw r4, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82E3E520: 4BFFFBD9  bl 0x82e3e0f8
	ctx.lr = 0x82E3E524;
	sub_82E3E0F8(ctx, base);
	// 82E3E524: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3E528: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3E52C: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3E530: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3E534: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E3E538: 409A0010  bne cr6, 0x82e3e548
	if !ctx.cr[6].eq {
	pc = 0x82E3E548; continue 'dispatch;
	}
	// 82E3E53C: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82E3E540: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E3E544: 4BF18A55  bl 0x82d56f98
	ctx.lr = 0x82E3E548;
	sub_82D56F98(ctx, base);
	// 82E3E548: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3E54C: 39000060  li r8, 0x60
	ctx.r[8].s64 = 96;
	// 82E3E550: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3E554: 556A2036  slwi r10, r11, 4
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E3E558: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 82E3E55C: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82E3E560: 90FE0004  stw r7, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3E628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3E628 size=8
    let mut pc: u32 = 0x82E3E628;
    'dispatch: loop {
        match pc {
            0x82E3E628 => {
    //   block [0x82E3E628..0x82E3E630)
	// 82E3E628: 38630060  addi r3, r3, 0x60
	ctx.r[3].s64 = ctx.r[3].s64 + 96;
	// 82E3E62C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3E630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3E630 size=8
    let mut pc: u32 = 0x82E3E630;
    'dispatch: loop {
        match pc {
            0x82E3E630 => {
    //   block [0x82E3E630..0x82E3E638)
	// 82E3E630: 38630070  addi r3, r3, 0x70
	ctx.r[3].s64 = ctx.r[3].s64 + 112;
	// 82E3E634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3E638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3E638 size=28
    let mut pc: u32 = 0x82E3E638;
    'dispatch: loop {
        match pc {
            0x82E3E638 => {
    //   block [0x82E3E638..0x82E3E654)
	// 82E3E638: 39600060  li r11, 0x60
	ctx.r[11].s64 = 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3E658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E3E658 size=208
    let mut pc: u32 = 0x82E3E658;
    'dispatch: loop {
        match pc {
            0x82E3E658 => {
    //   block [0x82E3E658..0x82E3E728)
	// 82E3E658: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82E3E65C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E3E660: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 82E3E664: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82E3E668: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82E3E66C: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82E3E670: C0090C14  lfs f0, 0xc14(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3E674: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3E728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3E728 size=156
    let mut pc: u32 = 0x82E3E728;
    'dispatch: loop {
        match pc {
            0x82E3E728 => {
    //   block [0x82E3E728..0x82E3E7C4)
	// 82E3E728: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 82E3E72C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E3E730: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82E3E734: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82E3E738: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3E7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E3E7C8 size=1236
    let mut pc: u32 = 0x82E3E7C8;
    'dispatch: loop {
        match pc {
            0x82E3E7C8 => {
    //   block [0x82E3E7C8..0x82E3EC9C)
	// 82E3E7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3E7CC: 4BE6AC39  bl 0x82ca9404
	ctx.lr = 0x82E3E7D0;
	sub_82CA93D0(ctx, base);
	// 82E3E7D0: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3E7D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E3E7D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3E7DC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3E7E0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3E7E4: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 82E3E7E8: 40980020  bge cr6, 0x82e3e808
	if !ctx.cr[6].lt {
	pc = 0x82E3E808; continue 'dispatch;
	}
	// 82E3E7EC: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E3E7F0: 2F040018  cmpwi cr6, r4, 0x18
	ctx.cr[6].compare_i32(ctx.r[4].s32, 24, &mut ctx.xer);
	// 82E3E7F4: 41990008  bgt cr6, 0x82e3e7fc
	if ctx.cr[6].gt {
	pc = 0x82E3E7FC; continue 'dispatch;
	}
	// 82E3E7F8: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	// 82E3E7FC: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82E3E800: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E3E804: 4BF1870D  bl 0x82d56f10
	ctx.lr = 0x82E3E808;
	sub_82D56F10(ctx, base);
	// 82E3E808: 38E00018  li r7, 0x18
	ctx.r[7].s64 = 24;
	// 82E3E80C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E3E810: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82E3E814: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82E3E818: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 82E3E81C: 90FE0004  stw r7, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82E3E820: C1BF0060  lfs f13, 0x60(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E3E824: D1A100C0  stfs f13, 0xc0(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), tmp.u32 ) };
	// 82E3E828: 38E00040  li r7, 0x40
	ctx.r[7].s64 = 64;
	// 82E3E82C: C1BF0064  lfs f13, 0x64(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E3E830: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82E3E834: D1A100C4  stfs f13, 0xc4(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 82E3E838: 38A00060  li r5, 0x60
	ctx.r[5].s64 = 96;
	// 82E3E83C: C00B0C18  lfs f0, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3E840: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3E844: C1BF0068  lfs f13, 0x68(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E3E848: 38800070  li r4, 0x70
	ctx.r[4].s64 = 112;
	// 82E3E84C: D1A100C8  stfs f13, 0xc8(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), tmp.u32 ) };
	// 82E3E850: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 82E3E854: D00100CC  stfs f0, 0xcc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), tmp.u32 ) };
	// 82E3E858: 3BA100C0  addi r29, r1, 0xc0
	ctx.r[29].s64 = ctx.r[1].s64 + 192;
	// 82E3E85C: D00100BC  stfs f0, 0xbc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 82E3E860: D00100DC  stfs f0, 0xdc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(220 as u32), tmp.u32 ) };
	// 82E3E864: D00100AC  stfs f0, 0xac(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 82E3E868: D001009C  stfs f0, 0x9c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3ECA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E3ECA0 size=2276
    let mut pc: u32 = 0x82E3ECA0;
    'dispatch: loop {
        match pc {
            0x82E3ECA0 => {
    //   block [0x82E3ECA0..0x82E3F584)
	// 82E3ECA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3ECA4: 4BE6A745  bl 0x82ca93e8
	ctx.lr = 0x82E3ECA8;
	sub_82CA93D0(ctx, base);
	// 82E3ECA8: DBE1FF90  stfd f31, -0x70(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-112 as u32), ctx.f[31].u64 ) };
	// 82E3ECAC: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3ECB0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3ECB4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E3ECB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3ECBC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3ECC0: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3ECC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3ECC8: 419A001C  beq cr6, 0x82e3ece4
	if ctx.cr[6].eq {
	pc = 0x82E3ECE4; continue 'dispatch;
	}
	// 82E3ECCC: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E3ECD0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E3ECD4: 91430054  stw r10, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E3ECD8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3ECDC: 91430050  stw r10, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E3ECE0: 48000010  b 0x82e3ecf0
	pc = 0x82E3ECF0; continue 'dispatch;
	// 82E3ECE4: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82E3ECE8: 4BF16369  bl 0x82d55050
	ctx.lr = 0x82E3ECEC;
	sub_82D55050(ctx, base);
	// 82E3ECEC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E3ECF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3ECF4: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 82E3ECF8: 419A0028  beq cr6, 0x82e3ed20
	if ctx.cr[6].eq {
	pc = 0x82E3ED20; continue 'dispatch;
	}
	// 82E3ECFC: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E3ED00: 928B0000  stw r20, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[20].u32 ) };
	// 82E3ED04: 928B0004  stw r20, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[20].u32 ) };
	// 82E3ED08: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82E3ED0C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E3ED10: 928B000C  stw r20, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[20].u32 ) };
	// 82E3ED14: 928B0010  stw r20, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[20].u32 ) };
	// 82E3ED18: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82E3ED1C: 48000008  b 0x82e3ed24
	pc = 0x82E3ED24; continue 'dispatch;
	// 82E3ED20: 7E9EA378  mr r30, r20
	ctx.r[30].u64 = ctx.r[20].u64;
	// 82E3ED24: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82E3ED28: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3ED2C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3ED30: 3BBC0001  addi r29, r28, 1
	ctx.r[29].s64 = ctx.r[28].s64 + 1;
	// 82E3ED34: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3ED38: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82E3ED3C: 40980024  bge cr6, 0x82e3ed60
	if !ctx.cr[6].lt {
	pc = 0x82E3ED60; continue 'dispatch;
	}
	// 82E3ED40: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E3ED44: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E3ED48: 41980008  blt cr6, 0x82e3ed50
	if ctx.cr[6].lt {
	pc = 0x82E3ED50; continue 'dispatch;
	}
	// 82E3ED4C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82E3ED50: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82E3ED54: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E3ED58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E3ED5C: 4BF181B5  bl 0x82d56f10
	ctx.lr = 0x82E3ED60;
	sub_82D56F10(ctx, base);
	// 82E3ED60: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E3ED64: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E3ED68: C01F0060  lfs f0, 0x60(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3ED6C: 578A2036  slwi r10, r28, 4
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E3ED70: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E3ED74: C01F0064  lfs f0, 0x64(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3ED78: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82E3ED7C: C3EB0C18  lfs f31, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E3ED80: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3ED84: C01F0068  lfs f0, 0x68(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3ED88: D3E1005C  stfs f31, 0x5c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82E3ED8C: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82E3ED90: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3F588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E3F588 size=176
    let mut pc: u32 = 0x82E3F588;
    'dispatch: loop {
        match pc {
            0x82E3F588 => {
    //   block [0x82E3F588..0x82E3F638)
	// 82E3F588: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82E3F58C: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82E3F590: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E3F594: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 82E3F598: 39295BAC  addi r9, r9, 0x5bac
	ctx.r[9].s64 = ctx.r[9].s64 + 23468;
	// 82E3F59C: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82E3F5A0: C0080C14  lfs f0, 0xc14(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3F5A4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82E3F5A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3F638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E3F638 size=140
    let mut pc: u32 = 0x82E3F638;
    'dispatch: loop {
        match pc {
            0x82E3F638 => {
    //   block [0x82E3F638..0x82E3F6C4)
	// 82E3F638: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E3F63C: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 82E3F640: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82E3F644: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82E3F648: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82E3F64C: C0090C14  lfs f0, 0xc14(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3F650: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3F6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E3F6C8 size=64
    let mut pc: u32 = 0x82E3F6C8;
    'dispatch: loop {
        match pc {
            0x82E3F6C8 => {
    //   block [0x82E3F6C8..0x82E3F708)
	// 82E3F6C8: 39400060  li r10, 0x60
	ctx.r[10].s64 = 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3F708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E3F708 size=348
    let mut pc: u32 = 0x82E3F708;
    'dispatch: loop {
        match pc {
            0x82E3F708 => {
    //   block [0x82E3F708..0x82E3F864)
	// 82E3F708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3F70C: 4BE69CF9  bl 0x82ca9404
	ctx.lr = 0x82E3F710;
	sub_82CA93D0(ctx, base);
	// 82E3F710: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82E3F714: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3F718: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E3F71C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E3F720: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3F724: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3F728: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 82E3F72C: 40980020  bge cr6, 0x82e3f74c
	if !ctx.cr[6].lt {
	pc = 0x82E3F74C; continue 'dispatch;
	}
	// 82E3F730: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E3F734: 2F040018  cmpwi cr6, r4, 0x18
	ctx.cr[6].compare_i32(ctx.r[4].s32, 24, &mut ctx.xer);
	// 82E3F738: 41990008  bgt cr6, 0x82e3f740
	if ctx.cr[6].gt {
	pc = 0x82E3F740; continue 'dispatch;
	}
	// 82E3F73C: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	// 82E3F740: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82E3F744: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3F748: 4BF177C9  bl 0x82d56f10
	ctx.lr = 0x82E3F74C;
	sub_82D56F10(ctx, base);
	// 82E3F74C: 39600018  li r11, 0x18
	ctx.r[11].s64 = 24;
	// 82E3F750: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E3F754: 3B7C0010  addi r27, r28, 0x10
	ctx.r[27].s64 = ctx.r[28].s64 + 16;
	// 82E3F758: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
	// 82E3F75C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E3F760: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E3F764: C3EB0EE0  lfs f31, 0xee0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3808 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E3F768: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E3F76C: E93C0068  ld r9, 0x68(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(104 as u32) ) };
	// 82E3F770: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E3F774: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E3F778: E95C0060  ld r10, 0x60(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(96 as u32) ) };
	// 82E3F77C: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82E3F780: F92B0008  std r9, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u64 ) };
	// 82E3F784: 419A0010  beq cr6, 0x82e3f794
	if ctx.cr[6].eq {
	pc = 0x82E3F794; continue 'dispatch;
	}
	// 82E3F788: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3F78C: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82E3F790: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E3F794: 57CB07BC  rlwinm r11, r30, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3F798: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3F79C: 419A0010  beq cr6, 0x82e3f7ac
	if ctx.cr[6].eq {
	pc = 0x82E3F7AC; continue 'dispatch;
	}
	// 82E3F7A0: C0010054  lfs f0, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3F7A4: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82E3F7A8: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82E3F7AC: 57CB077A  rlwinm r11, r30, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	// 82E3F7B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3F7B4: 419A0010  beq cr6, 0x82e3f7c4
	if ctx.cr[6].eq {
	pc = 0x82E3F7C4; continue 'dispatch;
	}
	// 82E3F7B8: C0010058  lfs f0, 0x58(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3F7BC: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82E3F7C0: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82E3F7C4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E3F7C8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E3F7CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E3F7D0: 4BF16BD9  bl 0x82d563a8
	ctx.lr = 0x82E3F7D4;
	sub_82D563A8(ctx, base);
	// 82E3F7D4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E3F7D8: 3BBD0010  addi r29, r29, 0x10
	ctx.r[29].s64 = ctx.r[29].s64 + 16;
	// 82E3F7DC: 2F1E0008  cmpwi cr6, r30, 8
	ctx.cr[6].compare_i32(ctx.r[30].s32, 8, &mut ctx.xer);
	// 82E3F7E0: 4198FF88  blt cr6, 0x82e3f768
	if ctx.cr[6].lt {
	pc = 0x82E3F768; continue 'dispatch;
	}
	// 82E3F7E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82E3F7E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E3F7EC: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82E3F7F0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E3F7F4: 550B2036  slwi r11, r8, 4
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E3F7F8: 7CA72278  xor r7, r5, r4
	ctx.r[7].u64 = ctx.r[5].u64 ^ ctx.r[4].u64;
	// 82E3F7FC: 7F043800  cmpw cr6, r4, r7
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82E3F800: 4098003C  bge cr6, 0x82e3f83c
	if !ctx.cr[6].lt {
	pc = 0x82E3F83C; continue 'dispatch;
	}
	// 82E3F804: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3F808: 39480001  addi r10, r8, 1
	ctx.r[10].s64 = ctx.r[8].s64 + 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E3F868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E3F868 size=2400
    let mut pc: u32 = 0x82E3F868;
    'dispatch: loop {
        match pc {
            0x82E3F868 => {
    //   block [0x82E3F868..0x82E401C8)
	// 82E3F868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3F86C: 4BE69B7D  bl 0x82ca93e8
	ctx.lr = 0x82E3F870;
	sub_82CA93D0(ctx, base);
	// 82E3F870: DBE1FF90  stfd f31, -0x70(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-112 as u32), ctx.f[31].u64 ) };
	// 82E3F874: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3F878: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3F87C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E3F880: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E3F884: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E3F888: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3F88C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3F890: 419A001C  beq cr6, 0x82e3f8ac
	if ctx.cr[6].eq {
	pc = 0x82E3F8AC; continue 'dispatch;
	}
	// 82E3F894: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E3F898: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E3F89C: 91430054  stw r10, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E3F8A0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3F8A4: 91430050  stw r10, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E3F8A8: 48000010  b 0x82e3f8b8
	pc = 0x82E3F8B8; continue 'dispatch;
	// 82E3F8AC: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82E3F8B0: 4BF157A1  bl 0x82d55050
	ctx.lr = 0x82E3F8B4;
	sub_82D55050(ctx, base);
	// 82E3F8B4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E3F8B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E3F8BC: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 82E3F8C0: 419A0028  beq cr6, 0x82e3f8e8
	if ctx.cr[6].eq {
	pc = 0x82E3F8E8; continue 'dispatch;
	}
	// 82E3F8C4: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E3F8C8: 928B0000  stw r20, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[20].u32 ) };
	// 82E3F8CC: 928B0004  stw r20, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[20].u32 ) };
	// 82E3F8D0: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82E3F8D4: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E3F8D8: 928B000C  stw r20, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[20].u32 ) };
	// 82E3F8DC: 928B0010  stw r20, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[20].u32 ) };
	// 82E3F8E0: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82E3F8E4: 48000008  b 0x82e3f8ec
	pc = 0x82E3F8EC; continue 'dispatch;
	// 82E3F8E8: 7E9EA378  mr r30, r20
	ctx.r[30].u64 = ctx.r[20].u64;
	// 82E3F8EC: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82E3F8F0: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3F8F4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3F8F8: 3BBC0001  addi r29, r28, 1
	ctx.r[29].s64 = ctx.r[28].s64 + 1;
	// 82E3F8FC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E3F900: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82E3F904: 40980024  bge cr6, 0x82e3f928
	if !ctx.cr[6].lt {
	pc = 0x82E3F928; continue 'dispatch;
	}
	// 82E3F908: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E3F90C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E3F910: 41980008  blt cr6, 0x82e3f918
	if ctx.cr[6].lt {
	pc = 0x82E3F918; continue 'dispatch;
	}
	// 82E3F914: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82E3F918: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82E3F91C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E3F920: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E3F924: 4BF175ED  bl 0x82d56f10
	ctx.lr = 0x82E3F928;
	sub_82D56F10(ctx, base);
	// 82E3F928: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E3F92C: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E3F930: C01F0060  lfs f0, 0x60(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3F934: 578A2036  slwi r10, r28, 4
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E3F938: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82E3F93C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E3F940: C1BF0064  lfs f13, 0x64(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E3F944: C01F0068  lfs f0, 0x68(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E3F948: C3EB0C18  lfs f31, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E3F94C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3F950: D3E1005C  stfs f31, 0x5c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82E3F954: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82E3F958: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82E3F95C: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E401C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E401C8 size=4
    let mut pc: u32 = 0x82E401C8;
    'dispatch: loop {
        match pc {
            0x82E401C8 => {
    //   block [0x82E401C8..0x82E401CC)
	// 82E401C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E401D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E401D0 size=168
    let mut pc: u32 = 0x82E401D0;
    'dispatch: loop {
        match pc {
            0x82E401D0 => {
    //   block [0x82E401D0..0x82E40278)
	// 82E401D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E401D4: 4BE69235  bl 0x82ca9408
	ctx.lr = 0x82E401D8;
	sub_82CA93D0(ctx, base);
	// 82E401D8: 3941FFC0  addi r10, r1, -0x40
	ctx.r[10].s64 = ctx.r[1].s64 + -64;
	// 82E401DC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E401E0: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82E401E4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E401E8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E40278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E40278 size=1864
    let mut pc: u32 = 0x82E40278;
    'dispatch: loop {
        match pc {
            0x82E40278 => {
    //   block [0x82E40278..0x82E409C0)
	// 82E40278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4027C: 4BE69161  bl 0x82ca93dc
	ctx.lr = 0x82E40280;
	sub_82CA93D0(ctx, base);
	// 82E40280: 3981FF80  addi r12, r1, -0x80
	ctx.r[12].s64 = ctx.r[1].s64 + -128;
	// 82E40284: 481C6749  bl 0x830069cc
	ctx.lr = 0x82E40288;
	sub_83006760(ctx, base);
	// 82E40288: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4028C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E40290: 39400060  li r10, 0x60
	ctx.r[10].s64 = 96;
	// 82E40294: 397E0070  addi r11, r30, 0x70
	ctx.r[11].s64 = ctx.r[30].s64 + 112;
	// 82E40298: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82E4029C: 7D795B78  mr r25, r11
	ctx.r[25].u64 = ctx.r[11].u64;
	// 82E402A0: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E409C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E409C0 size=100
    let mut pc: u32 = 0x82E409C0;
    'dispatch: loop {
        match pc {
            0x82E409C0 => {
    //   block [0x82E409C0..0x82E40A24)
	// 82E409C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E409C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E409C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E409CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E409D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E409D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E409D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E409DC: 48005915  bl 0x82e462f0
	ctx.lr = 0x82E409E0;
	sub_82E462F0(ctx, base);
	// 82E409E0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E409E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E409E8: 419A0020  beq cr6, 0x82e40a08
	if ctx.cr[6].eq {
	pc = 0x82E40A08; continue 'dispatch;
	}
	// 82E409EC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E409F0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E409F4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82E409F8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E409FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E40A00: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E40A04: 4BF148C5  bl 0x82d552c8
	ctx.lr = 0x82E40A08;
	sub_82D552C8(ctx, base);
	// 82E40A08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E40A0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E40A10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E40A14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E40A18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E40A1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E40A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E40A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E40A28 size=4
    let mut pc: u32 = 0x82E40A28;
    'dispatch: loop {
        match pc {
            0x82E40A28 => {
    //   block [0x82E40A28..0x82E40A2C)
	// 82E40A28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E40A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E40A30 size=168
    let mut pc: u32 = 0x82E40A30;
    'dispatch: loop {
        match pc {
            0x82E40A30 => {
    //   block [0x82E40A30..0x82E40AD8)
	// 82E40A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E40A34: 4BE689D5  bl 0x82ca9408
	ctx.lr = 0x82E40A38;
	sub_82CA93D0(ctx, base);
	// 82E40A38: 3941FFC0  addi r10, r1, -0x40
	ctx.r[10].s64 = ctx.r[1].s64 + -64;
	// 82E40A3C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E40A40: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82E40A44: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E40A48: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E40AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E40AD8 size=11644
    let mut pc: u32 = 0x82E40AD8;
    'dispatch: loop {
        match pc {
            0x82E40AD8 => {
    //   block [0x82E40AD8..0x82E43854)
	// 82E40AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E40ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E40AE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E40AE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E40AE8: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82E40AEC: 9421EB40  stwu r1, -0x14c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-5312 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E40AF0: 906114D4  stw r3, 0x14d4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(5332 as u32), ctx.r[3].u32 ) };
	// 82E40AF4: 816114D4  lwz r11, 0x14d4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(5332 as u32) ) } as u64;
	// 82E40AF8: 396B0060  addi r11, r11, 0x60
	ctx.r[11].s64 = ctx.r[11].s64 + 96;
	// 82E40AFC: 91610188  stw r11, 0x188(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(392 as u32), ctx.r[11].u32 ) };
	// 82E40B00: 816114D4  lwz r11, 0x14d4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(5332 as u32) ) } as u64;
	// 82E40B04: 396B0070  addi r11, r11, 0x70
	ctx.r[11].s64 = ctx.r[11].s64 + 112;
	// 82E40B08: 91610184  stw r11, 0x184(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(388 as u32), ctx.r[11].u32 ) };
	// 82E40B0C: 816114D4  lwz r11, 0x14d4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(5332 as u32) ) } as u64;
	// 82E40B10: C00B0080  lfs f0, 0x80(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E40B14: D0010180  stfs f0, 0x180(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(384 as u32), tmp.u32 ) };
	// 82E40B18: 816114D4  lwz r11, 0x14d4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(5332 as u32) ) } as u64;
	// 82E40B1C: 816B0088  lwz r11, 0x88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E40B20: 916100F0  stw r11, 0xf0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 82E40B24: 816114D4  lwz r11, 0x14d4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(5332 as u32) ) } as u64;
	// 82E40B28: 816B0084  lwz r11, 0x84(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E40B2C: 91610134  stw r11, 0x134(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(308 as u32), ctx.r[11].u32 ) };
	// 82E40B30: 81610134  lwz r11, 0x134(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82E40B34: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82E40B38: 91610144  stw r11, 0x144(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(324 as u32), ctx.r[11].u32 ) };
	// 82E40B3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E40B40: 91610138  stw r11, 0x138(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(312 as u32), ctx.r[11].u32 ) };
	// 82E40B44: 39610138  addi r11, r1, 0x138
	ctx.r[11].s64 = ctx.r[1].s64 + 312;
	// 82E40B48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E40B4C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E40B50: 39610138  addi r11, r1, 0x138
	ctx.r[11].s64 = ctx.r[1].s64 + 312;
	// 82E40B54: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E40B58: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E40B5C: 386101A0  addi r3, r1, 0x1a0
	ctx.r[3].s64 = ctx.r[1].s64 + 416;
	// 82E40B60: 4BD56319  bl 0x82b96e78
	ctx.lr = 0x82E40B64;
	sub_82B96E78(ctx, base);
	// 82E40B64: 81610188  lwz r11, 0x188(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(392 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E43858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E43858 size=4
    let mut pc: u32 = 0x82E43858;
    'dispatch: loop {
        match pc {
            0x82E43858 => {
    //   block [0x82E43858..0x82E4385C)
	// 82E43858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E43860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E43860 size=176
    let mut pc: u32 = 0x82E43860;
    'dispatch: loop {
        match pc {
            0x82E43860 => {
    //   block [0x82E43860..0x82E43910)
	// 82E43860: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82E43864: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 82E43868: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E4386C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82E43870: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82E43874: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E43910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E43910 size=728
    let mut pc: u32 = 0x82E43910;
    'dispatch: loop {
        match pc {
            0x82E43910 => {
    //   block [0x82E43910..0x82E43BE8)
	// 82E43910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E43914: 4BE65AED  bl 0x82ca9400
	ctx.lr = 0x82E43918;
	sub_82CA93D0(ctx, base);
	// 82E43918: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 82E4391C: 4BE6A3BD  bl 0x82cadcd8
	ctx.lr = 0x82E43920;
	sub_82CADCA0(ctx, base);
	// 82E43920: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E43924: FFE01090  fmr f31, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[2].f64;
	// 82E43928: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4392C: FFA00890  fmr f29, f1
	ctx.f[29].f64 = ctx.f[1].f64;
	// 82E43930: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E43934: D3A10134  stfs f29, 0x134(r1)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(308 as u32), tmp.u32 ) };
	// 82E43938: C38B0C18  lfs f28, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 82E4393C: FF1FE000  fcmpu cr6, f31, f28
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[28].f64);
	// 82E43940: 419A0298  beq cr6, 0x82e43bd8
	if ctx.cr[6].eq {
	pc = 0x82E43BD8; continue 'dispatch;
	}
	// 82E43944: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E43948: 3B7D000C  addi r27, r29, 0xc
	ctx.r[27].s64 = ctx.r[29].s64 + 12;
	// 82E4394C: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82E43950: 557A103A  slwi r26, r11, 2
	ctx.r[26].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 82E43954: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E43958: C3CB0C14  lfs f30, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82E4395C: 83FD0004  lwz r31, 4(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E43960: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E43964: 3B9F0003  addi r28, r31, 3
	ctx.r[28].s64 = ctx.r[31].s64 + 3;
	// 82E43968: 83DD0010  lwz r30, 0x10(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4396C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E43970: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82E43974: 40980024  bge cr6, 0x82e43998
	if !ctx.cr[6].lt {
	pc = 0x82E43998; continue 'dispatch;
	}
	// 82E43978: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E4397C: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E43980: 41980008  blt cr6, 0x82e43988
	if ctx.cr[6].lt {
	pc = 0x82E43988; continue 'dispatch;
	}
	// 82E43984: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82E43988: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82E4398C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E43990: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E43994: 4BF1357D  bl 0x82d56f10
	ctx.lr = 0x82E43998;
	sub_82D56F10(ctx, base);
	// 82E43998: 939D0004  stw r28, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82E4399C: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E439A0: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E439A4: 3B8B0003  addi r28, r11, 3
	ctx.r[28].s64 = ctx.r[11].s64 + 3;
	// 82E439A8: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82E439AC: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82E439B0: 40980024  bge cr6, 0x82e439d4
	if !ctx.cr[6].lt {
	pc = 0x82E439D4; continue 'dispatch;
	}
	// 82E439B4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E439B8: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E439BC: 41980008  blt cr6, 0x82e439c4
	if ctx.cr[6].lt {
	pc = 0x82E439C4; continue 'dispatch;
	}
	// 82E439C0: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82E439C4: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 82E439C8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E439CC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E439D0: 4BF13541  bl 0x82d56f10
	ctx.lr = 0x82E439D4;
	sub_82D56F10(ctx, base);
	// 82E439D4: 939B0004  stw r28, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82E439D8: 38E10134  addi r7, r1, 0x134
	ctx.r[7].s64 = ctx.r[1].s64 + 308;
	// 82E439DC: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E439E0: 38C10134  addi r6, r1, 0x134
	ctx.r[6].s64 = ctx.r[1].s64 + 308;
	// 82E439E4: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E439E8: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E439EC: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82E439F0: 57E82036  slwi r8, r31, 4
	ctx.r[8].u32 = ctx.r[31].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E43BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E43BE8 size=1272
    let mut pc: u32 = 0x82E43BE8;
    'dispatch: loop {
        match pc {
            0x82E43BE8 => {
    //   block [0x82E43BE8..0x82E440E0)
	// 82E43BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E43BEC: 4BE65821  bl 0x82ca940c
	ctx.lr = 0x82E43BF0;
	sub_82CA93D0(ctx, base);
	// 82E43BF0: DBA1FFC8  stfd f29, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[29].u64 ) };
	// 82E43BF4: DBC1FFD0  stfd f30, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82E43BF8: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E43BFC: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E43C00: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E43C04: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E43C08: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E43C0C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E43C10: 93C10174  stw r30, 0x174(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(372 as u32), ctx.r[30].u32 ) };
	// 82E43C14: 806B0050  lwz r3, 0x50(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E43C18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E43C1C: 419A001C  beq cr6, 0x82e43c38
	if ctx.cr[6].eq {
	pc = 0x82E43C38; continue 'dispatch;
	}
	// 82E43C20: 814B0054  lwz r10, 0x54(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E43C24: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E43C28: 914B0054  stw r10, 0x54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E43C2C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E43C30: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E43C34: 48000010  b 0x82e43c44
	pc = 0x82E43C44; continue 'dispatch;
	// 82E43C38: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82E43C3C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E43C40: 4BF11411  bl 0x82d55050
	ctx.lr = 0x82E43C44;
	sub_82D55050(ctx, base);
	// 82E43C44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E43C48: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E43C4C: 419A0028  beq cr6, 0x82e43c74
	if ctx.cr[6].eq {
	pc = 0x82E43C74; continue 'dispatch;
	}
	// 82E43C50: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E43C54: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82E43C58: 93A30004  stw r29, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E43C5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E43C60: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E43C64: 93A3000C  stw r29, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82E43C68: 93A30010  stw r29, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82E43C6C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82E43C70: 48000008  b 0x82e43c78
	pc = 0x82E43C78; continue 'dispatch;
	// 82E43C74: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82E43C78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E43C7C: 93FE0008  stw r31, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82E43C80: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E43C84: C3BE006C  lfs f29, 0x6c(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82E43C88: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E43C8C: C00A5C94  lfs f0, 0x5c94(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(23700 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E43C90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E43C94: EFFD0032  fmuls f31, f29, f0
	ctx.f[31].f64 = (((ctx.f[29].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E43C98: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 82E43C9C: C00A5C90  lfs f0, 0x5c90(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(23696 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E43CA0: EFDD0032  fmuls f30, f29, f0
	ctx.f[30].f64 = (((ctx.f[29].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E43CA4: 40980020  bge cr6, 0x82e43cc4
	if !ctx.cr[6].lt {
	pc = 0x82E43CC4; continue 'dispatch;
	}
	// 82E43CA8: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E43CAC: 2F04000C  cmpwi cr6, r4, 0xc
	ctx.cr[6].compare_i32(ctx.r[4].s32, 12, &mut ctx.xer);
	// 82E43CB0: 41990008  bgt cr6, 0x82e43cb8
	if ctx.cr[6].gt {
	pc = 0x82E43CB8; continue 'dispatch;
	}
	// 82E43CB4: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82E43CB8: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82E43CBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E43CC0: 4BF13251  bl 0x82d56f10
	ctx.lr = 0x82E43CC4;
	sub_82D56F10(ctx, base);
	// 82E43CC4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E43CC8: D3E100F0  stfs f31, 0xf0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 82E43CCC: D3C100F8  stfs f30, 0xf8(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(248 as u32), tmp.u32 ) };
	// 82E43CD0: FDA0F850  fneg f13, f31
	ctx.f[13].u64 = ctx.f[31].u64 ^ 0x8000_0000_0000_0000u64;
	// 82E43CD4: D3C100A4  stfs f30, 0xa4(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 82E43CD8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E43CDC: D3E100A8  stfs f31, 0xa8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 82E43CE0: FD80F050  fneg f12, f30
	ctx.f[12].u64 = ctx.f[30].u64 ^ 0x8000_0000_0000_0000u64;
	// 82E43CE4: D3C10110  stfs f30, 0x110(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(272 as u32), tmp.u32 ) };
	// 82E43CE8: C00A0C18  lfs f0, 0xc18(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E43CEC: 3940000C  li r10, 0xc
	ctx.r[10].s64 = 12;
	// 82E43CF0: D00100F4  stfs f0, 0xf4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(244 as u32), tmp.u32 ) };
	// 82E43CF4: D00100FC  stfs f0, 0xfc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 82E43CF8: D00100A0  stfs f0, 0xa0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 82E43CFC: D00100AC  stfs f0, 0xac(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 82E43D00: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E43D04: 394100F0  addi r10, r1, 0xf0
	ctx.r[10].s64 = ctx.r[1].s64 + 240;
	// 82E43D08: D3E10114  stfs f31, 0x114(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(276 as u32), tmp.u32 ) };
	// 82E43D0C: D0010118  stfs f0, 0x118(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(280 as u32), tmp.u32 ) };
	// 82E43D10: D001011C  stfs f0, 0x11c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(284 as u32), tmp.u32 ) };
	// 82E43D14: D00100C4  stfs f0, 0xc4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E440E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E440E0 size=4
    let mut pc: u32 = 0x82E440E0;
    'dispatch: loop {
        match pc {
            0x82E440E0 => {
    //   block [0x82E440E0..0x82E440E4)
	// 82E440E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E440E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E440E8 size=136
    let mut pc: u32 = 0x82E440E8;
    'dispatch: loop {
        match pc {
            0x82E440E8 => {
    //   block [0x82E440E8..0x82E44170)
	// 82E440E8: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82E440EC: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 82E440F0: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E440F4: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82E440F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82E440FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E44170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E44170 size=572
    let mut pc: u32 = 0x82E44170;
    'dispatch: loop {
        match pc {
            0x82E44170 => {
    //   block [0x82E44170..0x82E443AC)
	// 82E44170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E44174: 4BE65281  bl 0x82ca93f4
	ctx.lr = 0x82E44178;
	sub_82CA93D0(ctx, base);
	// 82E44178: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4417C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E44180: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E44184: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E44188: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4418C: 419A0218  beq cr6, 0x82e443a4
	if ctx.cr[6].eq {
	pc = 0x82E443A4; continue 'dispatch;
	}
	// 82E44190: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E44194: 3AEBFFFF  addi r23, r11, -1
	ctx.r[23].s64 = ctx.r[11].s64 + -1;
	// 82E44198: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 82E4419C: 41980208  blt cr6, 0x82e443a4
	if ctx.cr[6].lt {
	pc = 0x82E443A4; continue 'dispatch;
	}
	// 82E441A0: 56EB083C  slwi r11, r23, 1
	ctx.r[11].u32 = ctx.r[23].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E441A4: 7D775A14  add r11, r23, r11
	ctx.r[11].u64 = ctx.r[23].u64 + ctx.r[11].u64;
	// 82E441A8: 5578103A  slwi r24, r11, 2
	ctx.r[24].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 82E441AC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E441B0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E441B4: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E441B8: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82E441BC: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E441C0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E441C4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E441C8: 7D6AC214  add r11, r10, r24
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[24].u64;
	// 82E441CC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E441D0: 83AB0004  lwz r29, 4(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E441D4: 555B2036  slwi r27, r10, 4
	ctx.r[27].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82E441D8: 832B0008  lwz r25, 8(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E441DC: 7F89DA14  add r28, r9, r27
	ctx.r[28].u64 = ctx.r[9].u64 + ctx.r[27].u64;
	// 82E441E0: 409A0010  bne cr6, 0x82e441f0
	if !ctx.cr[6].eq {
	pc = 0x82E441F0; continue 'dispatch;
	}
	// 82E441E4: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82E441E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E441EC: 4BF12DAD  bl 0x82d56f98
	ctx.lr = 0x82E441F0;
	sub_82D56F98(ctx, base);
	// 82E441F0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E441F4: 57BA2036  slwi r26, r29, 4
	ctx.r[26].u32 = ctx.r[29].u32.wrapping_shl(4);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 82E441F8: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E441FC: 556A2036  slwi r10, r11, 4
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E44200: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82E44204: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82E44208: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E443B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E443B0 size=580
    let mut pc: u32 = 0x82E443B0;
    'dispatch: loop {
        match pc {
            0x82E443B0 => {
    //   block [0x82E443B0..0x82E445F4)
	// 82E443B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E443B4: 4BE65045  bl 0x82ca93f8
	ctx.lr = 0x82E443B8;
	sub_82CA93D0(ctx, base);
	// 82E443B8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E443BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E443C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E443C4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E443C8: 7D254B78  mr r5, r9
	ctx.r[5].u64 = ctx.r[9].u64;
	// 82E443CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E443D0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E443D4: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82E443D8: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82E443DC: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 82E443E0: 4BF19B31  bl 0x82d5df10
	ctx.lr = 0x82E443E4;
	sub_82D5DF10(ctx, base);
	// 82E443E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E443E8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E443EC: 4BF19E3D  bl 0x82d5e228
	ctx.lr = 0x82E443F0;
	sub_82D5E228(ctx, base);
	// 82E443F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E443F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E443F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E443FC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44400: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44404: 4E800421  bctrl
	ctx.lr = 0x82E44408;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44408: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4440C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E44410: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E44414: 419A01D0  beq cr6, 0x82e445e4
	if ctx.cr[6].eq {
	pc = 0x82E445E4; continue 'dispatch;
	}
	// 82E44418: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E4441C: 4BF19E0D  bl 0x82d5e228
	ctx.lr = 0x82E44420;
	sub_82D5E228(ctx, base);
	// 82E44420: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44424: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E44428: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4442C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44430: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44434: 4E800421  bctrl
	ctx.lr = 0x82E44438;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44438: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4443C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E44440: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E44444: 419A01A0  beq cr6, 0x82e445e4
	if ctx.cr[6].eq {
	pc = 0x82E445E4; continue 'dispatch;
	}
	// 82E44448: 835D0004  lwz r26, 4(r29)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4444C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E44450: 4BF19D79  bl 0x82d5e1c8
	ctx.lr = 0x82E44454;
	sub_82D5E1C8(ctx, base);
	// 82E44454: 3B600014  li r27, 0x14
	ctx.r[27].s64 = 20;
	// 82E44458: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E4445C: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82E44460: 419A0064  beq cr6, 0x82e444c4
	if ctx.cr[6].eq {
	pc = 0x82E444C4; continue 'dispatch;
	}
	// 82E44464: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E44468: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4446C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E44470: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E44474: 4BF19D25  bl 0x82d5e198
	ctx.lr = 0x82E44478;
	sub_82D5E198(ctx, base);
	// 82E44478: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4447C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E44480: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82E44484: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E44488: 4BF19D11  bl 0x82d5e198
	ctx.lr = 0x82E4448C;
	sub_82D5E198(ctx, base);
	// 82E4448C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44490: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E44494: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E44498: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4449C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E444A0: 4E800421  bctrl
	ctx.lr = 0x82E444A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E444A4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E444A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E444AC: 419A0134  beq cr6, 0x82e445e0
	if ctx.cr[6].eq {
	pc = 0x82E445E0; continue 'dispatch;
	}
	// 82E444B0: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82E444B4: 3B7B0008  addi r27, r27, 8
	ctx.r[27].s64 = ctx.r[27].s64 + 8;
	// 82E444B8: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82E444BC: 7F1CD040  cmplw cr6, r28, r26
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E444C0: 4198FFA8  blt cr6, 0x82e44468
	if ctx.cr[6].lt {
	pc = 0x82E44468; continue 'dispatch;
	}
	// 82E444C4: 835D0010  lwz r26, 0x10(r29)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E444C8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E444CC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E444D0: 4BF19CF9  bl 0x82d5e1c8
	ctx.lr = 0x82E444D4;
	sub_82D5E1C8(ctx, base);
	// 82E444D4: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82E444D8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E444DC: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82E444E0: 419A0078  beq cr6, 0x82e44558
	if ctx.cr[6].eq {
	pc = 0x82E44558; continue 'dispatch;
	}
	// 82E444E4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E444E8: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E444EC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E444F0: 7C8BF02E  lwzx r4, r11, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E444F4: 4BF19CA5  bl 0x82d5e198
	ctx.lr = 0x82E444F8;
	sub_82D5E198(ctx, base);
	// 82E444F8: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E444FC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E44500: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E44504: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E44508: 4BF19D21  bl 0x82d5e228
	ctx.lr = 0x82E4450C;
	sub_82D5E228(ctx, base);
	// 82E4450C: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44510: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E44514: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E44518: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4451C: 4BF19D0D  bl 0x82d5e228
	ctx.lr = 0x82E44520;
	sub_82D5E228(ctx, base);
	// 82E44520: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44524: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E44528: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4452C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44530: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44534: 4E800421  bctrl
	ctx.lr = 0x82E44538;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44538: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4453C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E44540: 419A00A0  beq cr6, 0x82e445e0
	if ctx.cr[6].eq {
	pc = 0x82E445E0; continue 'dispatch;
	}
	// 82E44544: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82E44548: 3B7B0014  addi r27, r27, 0x14
	ctx.r[27].s64 = ctx.r[27].s64 + 20;
	// 82E4454C: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 82E44550: 7F1CD040  cmplw cr6, r28, r26
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E44554: 4198FF94  blt cr6, 0x82e444e8
	if ctx.cr[6].lt {
	pc = 0x82E444E8; continue 'dispatch;
	}
	// 82E44558: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82E4455C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E44560: 4BF19C69  bl 0x82d5e1c8
	ctx.lr = 0x82E44564;
	sub_82D5E1C8(ctx, base);
	// 82E44564: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44568: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4456C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E44570: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44574: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44578: 4E800421  bctrl
	ctx.lr = 0x82E4457C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4457C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44580: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E44584: 419A005C  beq cr6, 0x82e445e0
	if ctx.cr[6].eq {
	pc = 0x82E445E0; continue 'dispatch;
	}
	// 82E44588: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4458C: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82E44590: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82E44594: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E44598: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4459C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E445A0: 4E800421  bctrl
	ctx.lr = 0x82E445A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E445A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E445A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E445AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E445B0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E445B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E445B8: 4E800421  bctrl
	ctx.lr = 0x82E445BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E445BC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E445C0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E445C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E445C8: 419A001C  beq cr6, 0x82e445e4
	if ctx.cr[6].eq {
	pc = 0x82E445E4; continue 'dispatch;
	}
	// 82E445CC: 4BF19AED  bl 0x82d5e0b8
	ctx.lr = 0x82E445D0;
	sub_82D5E0B8(ctx, base);
	// 82E445D0: 7D7BCA14  add r11, r27, r25
	ctx.r[11].u64 = ctx.r[27].u64 + ctx.r[25].u64;
	// 82E445D4: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E445D8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82E445DC: 4BE64E6C  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
	// 82E445E0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E445E4: 4BF19AD5  bl 0x82d5e0b8
	ctx.lr = 0x82E445E8;
	sub_82D5E0B8(ctx, base);
	// 82E445E8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82E445EC: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82E445F0: 4BE64E58  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E445F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E445F8 size=1132
    let mut pc: u32 = 0x82E445F8;
    'dispatch: loop {
        match pc {
            0x82E445F8 => {
    //   block [0x82E445F8..0x82E44A64)
	// 82E445F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E445FC: 4BE64DFD  bl 0x82ca93f8
	ctx.lr = 0x82E44600;
	sub_82CA93D0(ctx, base);
	// 82E44600: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E44604: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E44608: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82E4460C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82E44610: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82E44614: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E44618: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4461C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E44620: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E44624: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E44628: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4462C: 4E800421  bctrl
	ctx.lr = 0x82E44630;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44630: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44634: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E44638: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4463C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44640: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44644: 4E800421  bctrl
	ctx.lr = 0x82E44648;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44648: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4464C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E44650: 419A0388  beq cr6, 0x82e449d8
	if ctx.cr[6].eq {
	pc = 0x82E449D8; continue 'dispatch;
	}
	// 82E44654: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44658: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E4465C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E44660: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E44664: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E44668: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4466C: 4E800421  bctrl
	ctx.lr = 0x82E44670;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44670: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44674: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E44678: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4467C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44680: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44684: 4E800421  bctrl
	ctx.lr = 0x82E44688;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44688: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4468C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E44690: 419A0348  beq cr6, 0x82e449d8
	if ctx.cr[6].eq {
	pc = 0x82E449D8; continue 'dispatch;
	}
	// 82E44694: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44698: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82E4469C: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E446A0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E446A4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82E446A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E446AC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E446B0: 93210068  stw r25, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[25].u32 ) };
	// 82E446B4: 9321006C  stw r25, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[25].u32 ) };
	// 82E446B8: 91410070  stw r10, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 82E446BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E446C0: 4E800421  bctrl
	ctx.lr = 0x82E446C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E446C4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E446C8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E446CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E446D0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E446D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E446D8: 4E800421  bctrl
	ctx.lr = 0x82E446DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E446DC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E446E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E446E4: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E446E8: 419A02C8  beq cr6, 0x82e449b0
	if ctx.cr[6].eq {
	pc = 0x82E449B0; continue 'dispatch;
	}
	// 82E446EC: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E446F0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E446F4: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82E446F8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E446FC: 40980028  bge cr6, 0x82e44724
	if !ctx.cr[6].lt {
	pc = 0x82E44724; continue 'dispatch;
	}
	// 82E44700: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E44704: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E44708: 40980008  bge cr6, 0x82e44710
	if !ctx.cr[6].lt {
	pc = 0x82E44710; continue 'dispatch;
	}
	// 82E4470C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82E44710: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E44714: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E44718: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E4471C: 4BF127F5  bl 0x82d56f10
	ctx.lr = 0x82E44720;
	sub_82D56F10(ctx, base);
	// 82E44720: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E44724: 93C1006C  stw r30, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 82E44728: 7F3CCB78  mr r28, r25
	ctx.r[28].u64 = ctx.r[25].u64;
	// 82E4472C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E44730: 419A0080  beq cr6, 0x82e447b0
	if ctx.cr[6].eq {
	pc = 0x82E447B0; continue 'dispatch;
	}
	// 82E44734: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 82E44738: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E4473C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E44740: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E44744: 7FDD5A14  add r30, r29, r11
	ctx.r[30].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82E44748: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4474C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E44750: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E44754: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44758: 4E800421  bctrl
	ctx.lr = 0x82E4475C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4475C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44760: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E44764: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 82E44768: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4476C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E44770: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44774: 4E800421  bctrl
	ctx.lr = 0x82E44778;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44778: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4477C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E44780: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E44784: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44788: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4478C: 4E800421  bctrl
	ctx.lr = 0x82E44790;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44790: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44794: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E44798: 419A0214  beq cr6, 0x82e449ac
	if ctx.cr[6].eq {
	pc = 0x82E449AC; continue 'dispatch;
	}
	// 82E4479C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E447A0: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82E447A4: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 82E447A8: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E447AC: 4198FF8C  blt cr6, 0x82e44738
	if ctx.cr[6].lt {
	pc = 0x82E44738; continue 'dispatch;
	}
	// 82E447B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E447B4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E447B8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E447BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E447C0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E447C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E447C8: 4E800421  bctrl
	ctx.lr = 0x82E447CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E447CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E447D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E447D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E447D8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E447DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E447E0: 4E800421  bctrl
	ctx.lr = 0x82E447E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E447E4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E447E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E447EC: 419A01C0  beq cr6, 0x82e449ac
	if ctx.cr[6].eq {
	pc = 0x82E449AC; continue 'dispatch;
	}
	// 82E447F0: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E447F4: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E447F8: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82E447FC: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82E44800: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E44804: 40980028  bge cr6, 0x82e4482c
	if !ctx.cr[6].lt {
	pc = 0x82E4482C; continue 'dispatch;
	}
	// 82E44808: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E4480C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E44810: 40980008  bge cr6, 0x82e44818
	if !ctx.cr[6].lt {
	pc = 0x82E44818; continue 'dispatch;
	}
	// 82E44814: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82E44818: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 82E4481C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E44820: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E44824: 4BF126ED  bl 0x82d56f10
	ctx.lr = 0x82E44828;
	sub_82D56F10(ctx, base);
	// 82E44828: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E4482C: 7F3CCB78  mr r28, r25
	ctx.r[28].u64 = ctx.r[25].u64;
	// 82E44830: 93DB0004  stw r30, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82E44834: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E44838: 419A009C  beq cr6, 0x82e448d4
	if ctx.cr[6].eq {
	pc = 0x82E448D4; continue 'dispatch;
	}
	// 82E4483C: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 82E44840: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44844: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E44848: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4484C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E44850: 7FDD5A14  add r30, r29, r11
	ctx.r[30].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82E44854: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E44858: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4485C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44860: 4E800421  bctrl
	ctx.lr = 0x82E44864;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44864: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44868: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E4486C: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 82E44870: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E44874: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E44878: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4487C: 4E800421  bctrl
	ctx.lr = 0x82E44880;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44880: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44884: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E44888: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 82E4488C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E44890: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E44894: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44898: 4E800421  bctrl
	ctx.lr = 0x82E4489C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4489C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E448A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E448A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E448A8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E448AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E448B0: 4E800421  bctrl
	ctx.lr = 0x82E448B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E448B4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E448B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E448BC: 419A00F0  beq cr6, 0x82e449ac
	if ctx.cr[6].eq {
	pc = 0x82E449AC; continue 'dispatch;
	}
	// 82E448C0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E448C4: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82E448C8: 3BBD0018  addi r29, r29, 0x18
	ctx.r[29].s64 = ctx.r[29].s64 + 24;
	// 82E448CC: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E448D0: 4198FF70  blt cr6, 0x82e44840
	if ctx.cr[6].lt {
	pc = 0x82E44840; continue 'dispatch;
	}
	// 82E448D4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E448D8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82E448DC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E448E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E448E4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E448E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E448EC: 4E800421  bctrl
	ctx.lr = 0x82E448F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E448F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E448F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E448F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E448FC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44900: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44904: 4E800421  bctrl
	ctx.lr = 0x82E44908;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44908: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4490C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E44910: 419A009C  beq cr6, 0x82e449ac
	if ctx.cr[6].eq {
	pc = 0x82E449AC; continue 'dispatch;
	}
	// 82E44914: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44918: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 82E4491C: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 82E44920: 809A0000  lwz r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44924: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82E44928: 4BF10B51  bl 0x82d55478
	ctx.lr = 0x82E4492C;
	sub_82D55478(ctx, base);
	// 82E4492C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44930: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E44934: 80BA0000  lwz r5, 0(r26)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44938: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4493C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E44940: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E44944: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44948: 4E800421  bctrl
	ctx.lr = 0x82E4494C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4494C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44950: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E44954: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E44958: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4495C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44960: 4E800421  bctrl
	ctx.lr = 0x82E44964;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44964: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44968: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4496C: 409A0078  bne cr6, 0x82e449e4
	if !ctx.cr[6].eq {
	pc = 0x82E449E4; continue 'dispatch;
	}
	// 82E44970: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E44974: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82E44978: 4BF10B59  bl 0x82d554d0
	ctx.lr = 0x82E4497C;
	sub_82D554D0(ctx, base);
	// 82E4497C: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E44980: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E44984: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E44988: 409A0018  bne cr6, 0x82e449a0
	if !ctx.cr[6].eq {
	pc = 0x82E449A0; continue 'dispatch;
	}
	// 82E4498C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E44990: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82E44994: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E44998: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E4499C: 4BF1092D  bl 0x82d552c8
	ctx.lr = 0x82E449A0;
	sub_82D552C8(ctx, base);
	// 82E449A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E449A4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82E449A8: 4BE64AA0  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
	// 82E449AC: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E449B0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E449B4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E449B8: 409A0020  bne cr6, 0x82e449d8
	if !ctx.cr[6].eq {
	pc = 0x82E449D8; continue 'dispatch;
	}
	// 82E449BC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E449C0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E449C4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E449C8: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E449CC: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E449D0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E449D4: 4BF108F5  bl 0x82d552c8
	ctx.lr = 0x82E449D8;
	sub_82D552C8(ctx, base);
	// 82E449D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E449DC: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82E449E0: 4BE64A68  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
	// 82E449E4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E449E8: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82E449EC: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E449F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E449F4: 419A003C  beq cr6, 0x82e44a30
	if ctx.cr[6].eq {
	pc = 0x82E44A30; continue 'dispatch;
	}
	// 82E449F8: 7F29CB78  mr r9, r25
	ctx.r[9].u64 = ctx.r[25].u64;
	// 82E449FC: 7D69202E  lwzx r11, r9, r4
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82E44A00: 7D492214  add r10, r9, r4
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 82E44A04: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E44A08: 41980014  blt cr6, 0x82e44a1c
	if ctx.cr[6].lt {
	pc = 0x82E44A1C; continue 'dispatch;
	}
	// 82E44A0C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E44A10: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82E44A14: 7D4BF12E  stwx r10, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u32) };
	// 82E44A18: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E44A1C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E44A20: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82E44A24: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82E44A28: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E44A2C: 4198FFD0  blt cr6, 0x82e449fc
	if ctx.cr[6].lt {
	pc = 0x82E449FC; continue 'dispatch;
	}
	// 82E44A30: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E44A34: EBE10060  ld r31, 0x60(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E44A38: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E44A3C: 93D80000  stw r30, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82E44A40: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E44A44: 409A0014  bne cr6, 0x82e44a58
	if !ctx.cr[6].eq {
	pc = 0x82E44A58; continue 'dispatch;
	}
	// 82E44A48: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E44A4C: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82E44A50: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E44A54: 4BF10875  bl 0x82d552c8
	ctx.lr = 0x82E44A58;
	sub_82D552C8(ctx, base);
	// 82E44A58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E44A5C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82E44A60: 4BE649E8  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E44A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E44A68 size=584
    let mut pc: u32 = 0x82E44A68;
    'dispatch: loop {
        match pc {
            0x82E44A68 => {
    //   block [0x82E44A68..0x82E44CB0)
	// 82E44A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E44A6C: 4BE64989  bl 0x82ca93f4
	ctx.lr = 0x82E44A70;
	sub_82CA93D0(ctx, base);
	// 82E44A70: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E44A74: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82E44A78: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E44A7C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82E44A80: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 82E44A84: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E44A88: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82E44A8C: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 82E44A90: 4828F9F9  bl 0x830d4488
	ctx.lr = 0x82E44A94;
	sub_830D4488(ctx, base);
	// 82E44A94: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E44A98: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E44A9C: 3B2B3BA8  addi r25, r11, 0x3ba8
	ctx.r[25].s64 = ctx.r[11].s64 + 15272;
	// 82E44AA0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E44AA4: 39410058  addi r10, r1, 0x58
	ctx.r[10].s64 = ctx.r[1].s64 + 88;
	// 82E44AA8: 3FA08000  lis r29, -0x8000
	ctx.r[29].s64 = -2147483648;
	// 82E44AAC: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E44AB0: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82E44AB4: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82E44AB8: B1610086  sth r11, 0x86(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(134 as u32), ctx.r[11].u16 ) };
	// 82E44ABC: 93210080  stw r25, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[25].u32 ) };
	// 82E44AC0: 93A10060  stw r29, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 82E44AC4: 91410088  stw r10, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82E44AC8: 93E1008C  stw r31, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[31].u32 ) };
	// 82E44ACC: 91610090  stw r11, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82E44AD0: 4BF13AC9  bl 0x82d58598
	ctx.lr = 0x82E44AD4;
	sub_82D58598(ctx, base);
	// 82E44AD4: 38E100A0  addi r7, r1, 0xa0
	ctx.r[7].s64 = ctx.r[1].s64 + 160;
	// 82E44AD8: 93E100A0  stw r31, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[31].u32 ) };
	// 82E44ADC: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E44AE0: 93E100A4  stw r31, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[31].u32 ) };
	// 82E44AE4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E44AE8: 93A100A8  stw r29, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[29].u32 ) };
	// 82E44AEC: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82E44AF0: 93E100AC  stw r31, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[31].u32 ) };
	// 82E44AF4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E44AF8: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E44AFC: 93A100B4  stw r29, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[29].u32 ) };
	// 82E44B00: 93E100B8  stw r31, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[31].u32 ) };
	// 82E44B04: 93E100BC  stw r31, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[31].u32 ) };
	// 82E44B08: 93A100C0  stw r29, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[29].u32 ) };
	// 82E44B0C: 93E100C4  stw r31, 0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), ctx.r[31].u32 ) };
	// 82E44B10: 93E100C8  stw r31, 0xc8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), ctx.r[31].u32 ) };
	// 82E44B14: 93A100CC  stw r29, 0xcc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), ctx.r[29].u32 ) };
	// 82E44B18: 93E100D0  stw r31, 0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), ctx.r[31].u32 ) };
	// 82E44B1C: 4828FB5D  bl 0x830d4678
	ctx.lr = 0x82E44B20;
	sub_830D4678(ctx, base);
	// 82E44B20: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E44B24: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E44B28: 812100B0  lwz r9, 0xb0(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82E44B2C: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82E44B30: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82E44B34: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E44B38: 40980024  bge cr6, 0x82e44b5c
	if !ctx.cr[6].lt {
	pc = 0x82E44B5C; continue 'dispatch;
	}
	// 82E44B3C: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E44B40: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E44B44: 40980008  bge cr6, 0x82e44b4c
	if !ctx.cr[6].lt {
	pc = 0x82E44B4C; continue 'dispatch;
	}
	// 82E44B48: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82E44B4C: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 82E44B50: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E44B54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E44B58: 4BF123B9  bl 0x82d56f10
	ctx.lr = 0x82E44B5C;
	sub_82D56F10(ctx, base);
	// 82E44B5C: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82E44B60: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 82E44B64: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E44B68: 40990084  ble cr6, 0x82e44bec
	if !ctx.cr[6].gt {
	pc = 0x82E44BEC; continue 'dispatch;
	}
	// 82E44B6C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E44B70: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E44B74: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E44B78: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E44B7C: 409A0010  bne cr6, 0x82e44b8c
	if !ctx.cr[6].eq {
	pc = 0x82E44B8C; continue 'dispatch;
	}
	// 82E44B80: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	// 82E44B84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E44B88: 4BF12411  bl 0x82d56f98
	ctx.lr = 0x82E44B8C;
	sub_82D56F98(ctx, base);
	// 82E44B8C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E44B90: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E44B94: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44B98: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E44B9C: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82E44BA0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82E44BA4: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E44BA8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E44BAC: 814100AC  lwz r10, 0xac(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82E44BB0: 911E0004  stw r8, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82E44BB4: 7D5F502E  lwzx r10, r31, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E44BB8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E44BBC: 814100AC  lwz r10, 0xac(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82E44BC0: 7D5F5214  add r10, r31, r10
	ctx.r[10].u64 = ctx.r[31].u64 + ctx.r[10].u64;
	// 82E44BC4: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E44BC8: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 82E44BCC: 814100AC  lwz r10, 0xac(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82E44BD0: 7D5F5214  add r10, r31, r10
	ctx.r[10].u64 = ctx.r[31].u64 + ctx.r[10].u64;
	// 82E44BD4: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 82E44BD8: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E44BDC: F94B0010  std r10, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82E44BE0: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82E44BE4: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E44BE8: 4198FF84  blt cr6, 0x82e44b6c
	if ctx.cr[6].lt {
	pc = 0x82E44B6C; continue 'dispatch;
	}
	// 82E44BEC: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 82E44BF0: 893B0001  lbz r9, 1(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(1 as u32) ) } as u64;
	// 82E44BF4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E44BF8: 80E1005C  lwz r7, 0x5c(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E44BFC: 394A7D58  addi r10, r10, 0x7d58
	ctx.r[10].s64 = ctx.r[10].s64 + 32088;
	// 82E44C00: 80C10058  lwz r6, 0x58(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E44C04: 390100A0  addi r8, r1, 0xa0
	ctx.r[8].s64 = ctx.r[1].s64 + 160;
	// 82E44C08: 7B450020  clrldi r5, r26, 0x20
	ctx.r[5].u64 = ctx.r[26].u64 & 0x00000000FFFFFFFFu64;
	// 82E44C0C: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82E44C10: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82E44C14: 894A0001  lbz r10, 1(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 82E44C18: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82E44C1C: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82E44C20: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82E44C24: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82E44C28: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82E44C2C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44C30: 4BFFF781  bl 0x82e443b0
	ctx.lr = 0x82E44C34;
	sub_82E443B0(ctx, base);
	// 82E44C34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E44C38: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82E44C3C: 4BF309ED  bl 0x82d75628
	ctx.lr = 0x82E44C40;
	sub_82D75628(ctx, base);
	// 82E44C40: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E44C44: 93210080  stw r25, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[25].u32 ) };
	// 82E44C48: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E44C4C: 409A0018  bne cr6, 0x82e44c64
	if !ctx.cr[6].eq {
	pc = 0x82E44C64; continue 'dispatch;
	}
	// 82E44C50: 80610088  lwz r3, 0x88(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E44C54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E44C58: 419A000C  beq cr6, 0x82e44c64
	if ctx.cr[6].eq {
	pc = 0x82E44C64; continue 'dispatch;
	}
	// 82E44C5C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E44C60: 4BFF2FC1  bl 0x82e37c20
	ctx.lr = 0x82E44C64;
	sub_82E37C20(ctx, base);
	// 82E44C64: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E44C68: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E44C6C: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E44C70: 55490000  rlwinm r9, r10, 0, 0, 0
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E44C74: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E44C78: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82E44C7C: 409A0020  bne cr6, 0x82e44c9c
	if !ctx.cr[6].eq {
	pc = 0x82E44C9C; continue 'dispatch;
	}
	// 82E44C80: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44C84: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E44C88: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E44C8C: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E44C90: 554500BE  clrlwi r5, r10, 2
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82E44C94: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E44C98: 4BF10631  bl 0x82d552c8
	ctx.lr = 0x82E44C9C;
	sub_82D552C8(ctx, base);
	// 82E44C9C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E44CA0: 4828F949  bl 0x830d45e8
	ctx.lr = 0x82E44CA4;
	sub_830D45E8(ctx, base);
	// 82E44CA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E44CA8: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 82E44CAC: 4BE64798  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E44CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E44CB0 size=168
    let mut pc: u32 = 0x82E44CB0;
    'dispatch: loop {
        match pc {
            0x82E44CB0 => {
    //   block [0x82E44CB0..0x82E44D58)
	// 82E44CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E44CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E44CB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E44CBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E44CC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E44CC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E44CC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E44CCC: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E44CD0: 4B33FBC1  bl 0x82184890
	ctx.lr = 0x82E44CD4;
	sub_82184890(ctx, base);
	// 82E44CD4: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44CD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E44CDC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E44CE0: 40990050  ble cr6, 0x82e44d30
	if !ctx.cr[6].gt {
	pc = 0x82E44D30; continue 'dispatch;
	}
	// 82E44CE4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E44CE8: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44CEC: 7F08F040  cmplw cr6, r8, r30
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E44CF0: 419A0018  beq cr6, 0x82e44d08
	if ctx.cr[6].eq {
	pc = 0x82E44D08; continue 'dispatch;
	}
	// 82E44CF4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E44CF8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E44CFC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E44D00: 4198FFE8  blt cr6, 0x82e44ce8
	if ctx.cr[6].lt {
	pc = 0x82E44CE8; continue 'dispatch;
	}
	// 82E44D04: 4800002C  b 0x82e44d30
	pc = 0x82E44D30; continue 'dispatch;
	// 82E44D08: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E44D0C: 41980024  blt cr6, 0x82e44d30
	if ctx.cr[6].lt {
	pc = 0x82E44D30; continue 'dispatch;
	}
	// 82E44D10: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44D14: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E44D18: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E44D1C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E44D20: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E44D24: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E44D28: 7D48582E  lwzx r10, r8, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E44D2C: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82E44D30: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E44D34: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E44D38: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E44D3C: 48474C19  bl 0x832b9954
	ctx.lr = 0x82E44D40;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E44D40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E44D44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E44D48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E44D4C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E44D50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E44D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E44D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E44D58 size=72
    let mut pc: u32 = 0x82E44D58;
    'dispatch: loop {
        match pc {
            0x82E44D58 => {
    //   block [0x82E44D58..0x82E44DA0)
	// 82E44D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E44D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E44D60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E44D64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E44D68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E44D6C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E44D70: 4B33FB21  bl 0x82184890
	ctx.lr = 0x82E44D74;
	sub_82184890(ctx, base);
	// 82E44D74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E44D78: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E44D7C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82E44D80: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E44D84: F9430020  std r10, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u64 ) };
	// 82E44D88: 48474BCD  bl 0x832b9954
	ctx.lr = 0x82E44D8C;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E44D8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E44D90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E44D94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E44D98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E44D9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E44DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E44DA0 size=152
    let mut pc: u32 = 0x82E44DA0;
    'dispatch: loop {
        match pc {
            0x82E44DA0 => {
    //   block [0x82E44DA0..0x82E44E38)
	// 82E44DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E44DA4: 4BE64655  bl 0x82ca93f8
	ctx.lr = 0x82E44DA8;
	sub_82CA93D0(ctx, base);
	// 82E44DA8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E44DAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E44DB0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E44DB4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E44DB8: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E44DBC: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82E44DC0: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E44DC4: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 82E44DC8: 4B33FAC9  bl 0x82184890
	ctx.lr = 0x82E44DCC;
	sub_82184890(ctx, base);
	// 82E44DCC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44DD0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E44DD4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E44DD8: 40990048  ble cr6, 0x82e44e20
	if !ctx.cr[6].gt {
	pc = 0x82E44E20; continue 'dispatch;
	}
	// 82E44DDC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E44DE0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E44DE4: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 82E44DE8: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82E44DEC: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E44DF0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E44DF4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E44DF8: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E44DFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44E00: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E44E04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44E08: 4E800421  bctrl
	ctx.lr = 0x82E44E0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44E0C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44E10: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E44E14: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E44E18: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E44E1C: 4198FFC4  blt cr6, 0x82e44de0
	if ctx.cr[6].lt {
	pc = 0x82E44DE0; continue 'dispatch;
	}
	// 82E44E20: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E44E24: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E44E28: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E44E2C: 48474B29  bl 0x832b9954
	ctx.lr = 0x82E44E30;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E44E30: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E44E34: 4BE64614  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E44E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E44E38 size=152
    let mut pc: u32 = 0x82E44E38;
    'dispatch: loop {
        match pc {
            0x82E44E38 => {
    //   block [0x82E44E38..0x82E44ED0)
	// 82E44E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E44E3C: 4BE645BD  bl 0x82ca93f8
	ctx.lr = 0x82E44E40;
	sub_82CA93D0(ctx, base);
	// 82E44E40: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E44E44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E44E48: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E44E4C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E44E50: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E44E54: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82E44E58: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E44E5C: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 82E44E60: 4B33FA31  bl 0x82184890
	ctx.lr = 0x82E44E64;
	sub_82184890(ctx, base);
	// 82E44E64: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44E68: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E44E6C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E44E70: 40990048  ble cr6, 0x82e44eb8
	if !ctx.cr[6].gt {
	pc = 0x82E44EB8; continue 'dispatch;
	}
	// 82E44E74: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E44E78: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E44E7C: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 82E44E80: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82E44E84: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E44E88: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E44E8C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E44E90: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E44E94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44E98: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E44E9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44EA0: 4E800421  bctrl
	ctx.lr = 0x82E44EA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44EA4: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44EA8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E44EAC: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E44EB0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E44EB4: 4198FFC4  blt cr6, 0x82e44e78
	if ctx.cr[6].lt {
	pc = 0x82E44E78; continue 'dispatch;
	}
	// 82E44EB8: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E44EBC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E44EC0: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E44EC4: 48474A91  bl 0x832b9954
	ctx.lr = 0x82E44EC8;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E44EC8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E44ECC: 4BE6457C  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E44ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E44ED0 size=136
    let mut pc: u32 = 0x82E44ED0;
    'dispatch: loop {
        match pc {
            0x82E44ED0 => {
    //   block [0x82E44ED0..0x82E44F58)
	// 82E44ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E44ED4: 4BE6452D  bl 0x82ca9400
	ctx.lr = 0x82E44ED8;
	sub_82CA93D0(ctx, base);
	// 82E44ED8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E44EDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E44EE0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E44EE4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E44EE8: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E44EEC: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E44EF0: 4B33F9A1  bl 0x82184890
	ctx.lr = 0x82E44EF4;
	sub_82184890(ctx, base);
	// 82E44EF4: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44EF8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E44EFC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E44F00: 40990040  ble cr6, 0x82e44f40
	if !ctx.cr[6].gt {
	pc = 0x82E44F40; continue 'dispatch;
	}
	// 82E44F04: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E44F08: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E44F0C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E44F10: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E44F14: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E44F18: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E44F1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44F20: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44F24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44F28: 4E800421  bctrl
	ctx.lr = 0x82E44F2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44F2C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44F30: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E44F34: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E44F38: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E44F3C: 4198FFCC  blt cr6, 0x82e44f08
	if ctx.cr[6].lt {
	pc = 0x82E44F08; continue 'dispatch;
	}
	// 82E44F40: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E44F44: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E44F48: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E44F4C: 48474A09  bl 0x832b9954
	ctx.lr = 0x82E44F50;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E44F50: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E44F54: 4BE644FC  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E44F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E44F58 size=136
    let mut pc: u32 = 0x82E44F58;
    'dispatch: loop {
        match pc {
            0x82E44F58 => {
    //   block [0x82E44F58..0x82E44FE0)
	// 82E44F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E44F5C: 4BE644A5  bl 0x82ca9400
	ctx.lr = 0x82E44F60;
	sub_82CA93D0(ctx, base);
	// 82E44F60: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E44F64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E44F68: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E44F6C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E44F70: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E44F74: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E44F78: 4B33F919  bl 0x82184890
	ctx.lr = 0x82E44F7C;
	sub_82184890(ctx, base);
	// 82E44F7C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44F80: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E44F84: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E44F88: 40990040  ble cr6, 0x82e44fc8
	if !ctx.cr[6].gt {
	pc = 0x82E44FC8; continue 'dispatch;
	}
	// 82E44F8C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E44F90: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E44F94: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E44F98: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E44F9C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E44FA0: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E44FA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E44FA8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E44FAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E44FB0: 4E800421  bctrl
	ctx.lr = 0x82E44FB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E44FB4: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E44FB8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E44FBC: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E44FC0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E44FC4: 4198FFCC  blt cr6, 0x82e44f90
	if ctx.cr[6].lt {
	pc = 0x82E44F90; continue 'dispatch;
	}
	// 82E44FC8: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E44FCC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E44FD0: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E44FD4: 48474981  bl 0x832b9954
	ctx.lr = 0x82E44FD8;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E44FD8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E44FDC: 4BE64474  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E44FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E44FE0 size=136
    let mut pc: u32 = 0x82E44FE0;
    'dispatch: loop {
        match pc {
            0x82E44FE0 => {
    //   block [0x82E44FE0..0x82E45068)
	// 82E44FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E44FE4: 4BE6441D  bl 0x82ca9400
	ctx.lr = 0x82E44FE8;
	sub_82CA93D0(ctx, base);
	// 82E44FE8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E44FEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E44FF0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E44FF4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E44FF8: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E44FFC: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45000: 4B33F891  bl 0x82184890
	ctx.lr = 0x82E45004;
	sub_82184890(ctx, base);
	// 82E45004: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E45008: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E4500C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E45010: 40990040  ble cr6, 0x82e45050
	if !ctx.cr[6].gt {
	pc = 0x82E45050; continue 'dispatch;
	}
	// 82E45014: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E45018: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4501C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E45020: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E45024: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E45028: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4502C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E45030: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45034: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E45038: 4E800421  bctrl
	ctx.lr = 0x82E4503C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4503C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E45040: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E45044: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E45048: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4504C: 4198FFCC  blt cr6, 0x82e45018
	if ctx.cr[6].lt {
	pc = 0x82E45018; continue 'dispatch;
	}
	// 82E45050: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45054: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E45058: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E4505C: 484748F9  bl 0x832b9954
	ctx.lr = 0x82E45060;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E45060: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E45064: 4BE643EC  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45068 size=192
    let mut pc: u32 = 0x82E45068;
    'dispatch: loop {
        match pc {
            0x82E45068 => {
    //   block [0x82E45068..0x82E45128)
	// 82E45068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4506C: 4BE64391  bl 0x82ca93fc
	ctx.lr = 0x82E45070;
	sub_82CA93D0(ctx, base);
	// 82E45070: DBA1FFA8  stfd f29, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[29].u64 ) };
	// 82E45074: DBC1FFB0  stfd f30, -0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[30].u64 ) };
	// 82E45078: DBE1FFB8  stfd f31, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 82E4507C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E45080: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E45084: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E45088: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E4508C: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82E45090: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E45094: FFA01890  fmr f29, f3
	ctx.f[29].f64 = ctx.f[3].f64;
	// 82E45098: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E4509C: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 82E450A0: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E450A4: 4B33F7ED  bl 0x82184890
	ctx.lr = 0x82E450A8;
	sub_82184890(ctx, base);
	// 82E450A8: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E450AC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E450B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E450B4: 40990050  ble cr6, 0x82e45104
	if !ctx.cr[6].gt {
	pc = 0x82E45104; continue 'dispatch;
	}
	// 82E450B8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E450BC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E450C0: 7F2ACB78  mr r10, r25
	ctx.r[10].u64 = ctx.r[25].u64;
	// 82E450C4: FC60E890  fmr f3, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[29].f64;
	// 82E450C8: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E450CC: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82E450D0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E450D4: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E450D8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E450DC: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E450E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E450E4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E450E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E450EC: 4E800421  bctrl
	ctx.lr = 0x82E450F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E450F0: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E450F4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E450F8: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E450FC: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E45100: 4198FFBC  blt cr6, 0x82e450bc
	if ctx.cr[6].lt {
	pc = 0x82E450BC; continue 'dispatch;
	}
	// 82E45104: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45108: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E4510C: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E45110: 48474845  bl 0x832b9954
	ctx.lr = 0x82E45114;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E45114: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E45118: CBA1FFA8  lfd f29, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 82E4511C: CBC1FFB0  lfd f30, -0x50(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 82E45120: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 82E45124: 4BE64328  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45128 size=136
    let mut pc: u32 = 0x82E45128;
    'dispatch: loop {
        match pc {
            0x82E45128 => {
    //   block [0x82E45128..0x82E451B0)
	// 82E45128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4512C: 4BE642D5  bl 0x82ca9400
	ctx.lr = 0x82E45130;
	sub_82CA93D0(ctx, base);
	// 82E45130: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E45134: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E45138: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E4513C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E45140: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E45144: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45148: 4B33F749  bl 0x82184890
	ctx.lr = 0x82E4514C;
	sub_82184890(ctx, base);
	// 82E4514C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E45150: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E45154: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E45158: 40990040  ble cr6, 0x82e45198
	if !ctx.cr[6].gt {
	pc = 0x82E45198; continue 'dispatch;
	}
	// 82E4515C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E45160: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E45164: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E45168: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E4516C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E45170: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E45174: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E45178: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E4517C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E45180: 4E800421  bctrl
	ctx.lr = 0x82E45184;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E45184: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E45188: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E4518C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E45190: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E45194: 4198FFCC  blt cr6, 0x82e45160
	if ctx.cr[6].lt {
	pc = 0x82E45160; continue 'dispatch;
	}
	// 82E45198: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E4519C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E451A0: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E451A4: 484747B1  bl 0x832b9954
	ctx.lr = 0x82E451A8;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E451A8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E451AC: 4BE642A4  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E451B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E451B0 size=144
    let mut pc: u32 = 0x82E451B0;
    'dispatch: loop {
        match pc {
            0x82E451B0 => {
    //   block [0x82E451B0..0x82E45240)
	// 82E451B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E451B4: 4BE64249  bl 0x82ca93fc
	ctx.lr = 0x82E451B8;
	sub_82CA93D0(ctx, base);
	// 82E451B8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E451BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E451C0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E451C4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E451C8: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E451CC: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82E451D0: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E451D4: 4B33F6BD  bl 0x82184890
	ctx.lr = 0x82E451D8;
	sub_82184890(ctx, base);
	// 82E451D8: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E451DC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E451E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E451E4: 40990044  ble cr6, 0x82e45228
	if !ctx.cr[6].gt {
	pc = 0x82E45228; continue 'dispatch;
	}
	// 82E451E8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E451EC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E451F0: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82E451F4: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E451F8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E451FC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E45200: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E45204: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E45208: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E4520C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E45210: 4E800421  bctrl
	ctx.lr = 0x82E45214;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E45214: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E45218: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E4521C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E45220: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E45224: 4198FFC8  blt cr6, 0x82e451ec
	if ctx.cr[6].lt {
	pc = 0x82E451EC; continue 'dispatch;
	}
	// 82E45228: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E4522C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E45230: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E45234: 48474721  bl 0x832b9954
	ctx.lr = 0x82E45238;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E45238: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E4523C: 4BE64210  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45240 size=88
    let mut pc: u32 = 0x82E45240;
    'dispatch: loop {
        match pc {
            0x82E45240 => {
    //   block [0x82E45240..0x82E45298)
	// 82E45240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E45244: 4BE641C1  bl 0x82ca9404
	ctx.lr = 0x82E45248;
	sub_82CA93D0(ctx, base);
	// 82E45248: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4524C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E45250: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E45254: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E45258: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82E4525C: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82E45260: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82E45264: 4BF11295  bl 0x82d564f8
	ctx.lr = 0x82E45268;
	sub_82D564F8(ctx, base);
	// 82E45268: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E4526C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E45270: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E45274: 4BF11285  bl 0x82d564f8
	ctx.lr = 0x82E45278;
	sub_82D564F8(ctx, base);
	// 82E45278: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82E4527C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E45280: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E45284: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E45288: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E4528C: 4BFFFF25  bl 0x82e451b0
	ctx.lr = 0x82E45290;
	sub_82E451B0(ctx, base);
	// 82E45290: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E45294: 4BE641C0  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45298 size=104
    let mut pc: u32 = 0x82E45298;
    'dispatch: loop {
        match pc {
            0x82E45298 => {
    //   block [0x82E45298..0x82E45300)
	// 82E45298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4529C: 4BE64169  bl 0x82ca9404
	ctx.lr = 0x82E452A0;
	sub_82CA93D0(ctx, base);
	// 82E452A0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E452A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E452A8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E452AC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E452B0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82E452B4: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82E452B8: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82E452BC: 4BF1604D  bl 0x82d5b308
	ctx.lr = 0x82E452C0;
	sub_82D5B308(ctx, base);
	// 82E452C0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E452C4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82E452C8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E452CC: 4BF1122D  bl 0x82d564f8
	ctx.lr = 0x82E452D0;
	sub_82D564F8(ctx, base);
	// 82E452D0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E452D4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82E452D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E452DC: 4BF1121D  bl 0x82d564f8
	ctx.lr = 0x82E452E0;
	sub_82D564F8(ctx, base);
	// 82E452E0: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82E452E4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E452E8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E452EC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E452F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E452F4: 4BFFFEBD  bl 0x82e451b0
	ctx.lr = 0x82E452F8;
	sub_82E451B0(ctx, base);
	// 82E452F8: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82E452FC: 4BE64158  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45300 size=136
    let mut pc: u32 = 0x82E45300;
    'dispatch: loop {
        match pc {
            0x82E45300 => {
    //   block [0x82E45300..0x82E45388)
	// 82E45300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E45304: 4BE640FD  bl 0x82ca9400
	ctx.lr = 0x82E45308;
	sub_82CA93D0(ctx, base);
	// 82E45308: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4530C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45388 size=164
    let mut pc: u32 = 0x82E45388;
    'dispatch: loop {
        match pc {
            0x82E45388 => {
    //   block [0x82E45388..0x82E4542C)
	// 82E45388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4538C: 4BE64079  bl 0x82ca9404
	ctx.lr = 0x82E45390;
	sub_82CA93D0(ctx, base);
	// 82E45390: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E45394: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E45398: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4539C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E453A0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82E453A4: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82E453A8: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82E453AC: 4BF1114D  bl 0x82d564f8
	ctx.lr = 0x82E453B0;
	sub_82D564F8(ctx, base);
	// 82E453B0: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45430 size=76
    let mut pc: u32 = 0x82E45430;
    'dispatch: loop {
        match pc {
            0x82E45430 => {
    //   block [0x82E45430..0x82E4547C)
	// 82E45430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E45434: 4BE63FD1  bl 0x82ca9404
	ctx.lr = 0x82E45438;
	sub_82CA93D0(ctx, base);
	// 82E45438: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4543C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E45440: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E45444: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E45448: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82E4544C: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82E45450: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82E45454: 4BF15EB5  bl 0x82d5b308
	ctx.lr = 0x82E45458;
	sub_82D5B308(ctx, base);
	// 82E45458: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 82E4545C: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82E45460: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E45464: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E45468: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E4546C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E45470: 4BFFFF19  bl 0x82e45388
	ctx.lr = 0x82E45474;
	sub_82E45388(ctx, base);
	// 82E45474: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E45478: 4BE63FDC  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45480 size=628
    let mut pc: u32 = 0x82E45480;
    'dispatch: loop {
        match pc {
            0x82E45480 => {
    //   block [0x82E45480..0x82E456F4)
	// 82E45480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E45484: 4BE63F81  bl 0x82ca9404
	ctx.lr = 0x82E45488;
	sub_82CA93D0(ctx, base);
	// 82E45488: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E456F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E456F8 size=172
    let mut pc: u32 = 0x82E456F8;
    'dispatch: loop {
        match pc {
            0x82E456F8 => {
    //   block [0x82E456F8..0x82E457A4)
	// 82E456F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E456FC: 4BE63D05  bl 0x82ca9400
	ctx.lr = 0x82E45700;
	sub_82CA93D0(ctx, base);
	// 82E45700: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82E45704: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E457A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E457A8 size=76
    let mut pc: u32 = 0x82E457A8;
    'dispatch: loop {
        match pc {
            0x82E457A8 => {
    //   block [0x82E457A8..0x82E457F4)
	// 82E457A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E457AC: 4BE63C61  bl 0x82ca940c
	ctx.lr = 0x82E457B0;
	sub_82CA93D0(ctx, base);
	// 82E457B0: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E457B4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E457B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E457BC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E457C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E457C4: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E457C8: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 82E457CC: 4BF10D2D  bl 0x82d564f8
	ctx.lr = 0x82E457D0;
	sub_82D564F8(ctx, base);
	// 82E457D0: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82E457D4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E457D8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E457DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E457E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E457E4: 4BFFFF15  bl 0x82e456f8
	ctx.lr = 0x82E457E8;
	sub_82E456F8(ctx, base);
	// 82E457E8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E457EC: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82E457F0: 4BE63C6C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E457F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E457F8 size=76
    let mut pc: u32 = 0x82E457F8;
    'dispatch: loop {
        match pc {
            0x82E457F8 => {
    //   block [0x82E457F8..0x82E45844)
	// 82E457F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E457FC: 4BE63C11  bl 0x82ca940c
	ctx.lr = 0x82E45800;
	sub_82CA93D0(ctx, base);
	// 82E45800: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E45804: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E45808: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4580C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E45810: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E45814: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E45818: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 82E4581C: 4BF10B8D  bl 0x82d563a8
	ctx.lr = 0x82E45820;
	sub_82D563A8(ctx, base);
	// 82E45820: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82E45824: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E45828: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E4582C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E45830: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E45834: 4BFFFEC5  bl 0x82e456f8
	ctx.lr = 0x82E45838;
	sub_82E456F8(ctx, base);
	// 82E45838: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E4583C: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82E45840: 4BE63C1C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E45848 size=724
    let mut pc: u32 = 0x82E45848;
    'dispatch: loop {
        match pc {
            0x82E45848 => {
    //   block [0x82E45848..0x82E45B1C)
	// 82E45848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4584C: 4BE63BB5  bl 0x82ca9400
	ctx.lr = 0x82E45850;
	sub_82CA93D0(ctx, base);
	// 82E45850: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45B20 size=136
    let mut pc: u32 = 0x82E45B20;
    'dispatch: loop {
        match pc {
            0x82E45B20 => {
    //   block [0x82E45B20..0x82E45BA8)
	// 82E45B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E45B24: 4BE638DD  bl 0x82ca9400
	ctx.lr = 0x82E45B28;
	sub_82CA93D0(ctx, base);
	// 82E45B28: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E45B2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E45B30: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E45B34: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E45B38: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E45B3C: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45B40: 4B33ED51  bl 0x82184890
	ctx.lr = 0x82E45B44;
	sub_82184890(ctx, base);
	// 82E45B44: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E45B48: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E45B4C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E45B50: 40990040  ble cr6, 0x82e45b90
	if !ctx.cr[6].gt {
	pc = 0x82E45B90; continue 'dispatch;
	}
	// 82E45B54: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E45B58: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E45B5C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E45B60: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E45B64: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E45B68: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E45B6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E45B70: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E45B74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E45B78: 4E800421  bctrl
	ctx.lr = 0x82E45B7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E45B7C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E45B80: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E45B84: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E45B88: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E45B8C: 4198FFCC  blt cr6, 0x82e45b58
	if ctx.cr[6].lt {
	pc = 0x82E45B58; continue 'dispatch;
	}
	// 82E45B90: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45B94: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E45B98: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E45B9C: 48473DB9  bl 0x832b9954
	ctx.lr = 0x82E45BA0;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E45BA0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E45BA4: 4BE638AC  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45BA8 size=144
    let mut pc: u32 = 0x82E45BA8;
    'dispatch: loop {
        match pc {
            0x82E45BA8 => {
    //   block [0x82E45BA8..0x82E45C38)
	// 82E45BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E45BAC: 4BE63851  bl 0x82ca93fc
	ctx.lr = 0x82E45BB0;
	sub_82CA93D0(ctx, base);
	// 82E45BB0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E45BB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E45BB8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E45BBC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E45BC0: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E45BC4: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82E45BC8: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45BCC: 4B33ECC5  bl 0x82184890
	ctx.lr = 0x82E45BD0;
	sub_82184890(ctx, base);
	// 82E45BD0: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E45BD4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E45BD8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E45BDC: 40990044  ble cr6, 0x82e45c20
	if !ctx.cr[6].gt {
	pc = 0x82E45C20; continue 'dispatch;
	}
	// 82E45BE0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E45BE4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E45BE8: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82E45BEC: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E45BF0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E45BF4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E45BF8: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E45BFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E45C00: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E45C04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E45C08: 4E800421  bctrl
	ctx.lr = 0x82E45C0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E45C0C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E45C10: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E45C14: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E45C18: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E45C1C: 4198FFC8  blt cr6, 0x82e45be4
	if ctx.cr[6].lt {
	pc = 0x82E45BE4; continue 'dispatch;
	}
	// 82E45C20: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45C24: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E45C28: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E45C2C: 48473D29  bl 0x832b9954
	ctx.lr = 0x82E45C30;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E45C30: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E45C34: 4BE63818  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45C38 size=144
    let mut pc: u32 = 0x82E45C38;
    'dispatch: loop {
        match pc {
            0x82E45C38 => {
    //   block [0x82E45C38..0x82E45CC8)
	// 82E45C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E45C3C: 4BE637C1  bl 0x82ca93fc
	ctx.lr = 0x82E45C40;
	sub_82CA93D0(ctx, base);
	// 82E45C40: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E45C44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E45C48: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E45C4C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E45C50: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E45C54: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82E45C58: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45C5C: 4B33EC35  bl 0x82184890
	ctx.lr = 0x82E45C60;
	sub_82184890(ctx, base);
	// 82E45C60: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E45C64: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E45C68: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E45C6C: 40990044  ble cr6, 0x82e45cb0
	if !ctx.cr[6].gt {
	pc = 0x82E45CB0; continue 'dispatch;
	}
	// 82E45C70: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E45C74: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E45C78: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82E45C7C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E45C80: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E45C84: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E45C88: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E45C8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E45C90: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E45C94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E45C98: 4E800421  bctrl
	ctx.lr = 0x82E45C9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E45C9C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E45CA0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E45CA4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E45CA8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E45CAC: 4198FFC8  blt cr6, 0x82e45c74
	if ctx.cr[6].lt {
	pc = 0x82E45C74; continue 'dispatch;
	}
	// 82E45CB0: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45CB4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E45CB8: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E45CBC: 48473C99  bl 0x832b9954
	ctx.lr = 0x82E45CC0;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E45CC0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E45CC4: 4BE63788  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45CC8 size=136
    let mut pc: u32 = 0x82E45CC8;
    'dispatch: loop {
        match pc {
            0x82E45CC8 => {
    //   block [0x82E45CC8..0x82E45D50)
	// 82E45CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E45CCC: 4BE63735  bl 0x82ca9400
	ctx.lr = 0x82E45CD0;
	sub_82CA93D0(ctx, base);
	// 82E45CD0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E45CD4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E45CD8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E45CDC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E45CE0: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E45CE4: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45CE8: 4B33EBA9  bl 0x82184890
	ctx.lr = 0x82E45CEC;
	sub_82184890(ctx, base);
	// 82E45CEC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E45CF0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E45CF4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E45CF8: 40990040  ble cr6, 0x82e45d38
	if !ctx.cr[6].gt {
	pc = 0x82E45D38; continue 'dispatch;
	}
	// 82E45CFC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E45D00: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E45D04: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E45D08: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E45D0C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E45D10: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E45D14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E45D18: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E45D1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E45D20: 4E800421  bctrl
	ctx.lr = 0x82E45D24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E45D24: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E45D28: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E45D2C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E45D30: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E45D34: 4198FFCC  blt cr6, 0x82e45d00
	if ctx.cr[6].lt {
	pc = 0x82E45D00; continue 'dispatch;
	}
	// 82E45D38: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45D3C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E45D40: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E45D44: 48473C11  bl 0x832b9954
	ctx.lr = 0x82E45D48;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E45D48: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E45D4C: 4BE63704  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45D50 size=248
    let mut pc: u32 = 0x82E45D50;
    'dispatch: loop {
        match pc {
            0x82E45D50 => {
    //   block [0x82E45D50..0x82E45E48)
	// 82E45D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E45D54: 4BE636B1  bl 0x82ca9404
	ctx.lr = 0x82E45D58;
	sub_82CA93D0(ctx, base);
	// 82E45D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E45D5C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E45D60: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E45D64: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E45D68: 396B5CCC  addi r11, r11, 0x5ccc
	ctx.r[11].s64 = ctx.r[11].s64 + 23756;
	// 82E45D6C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82E45D70: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E45D74: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82E45D78: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82E45D7C: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E45D80: B13B0006  sth r9, 6(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82E45D84: 939B0008  stw r28, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82E45D88: 939B000C  stw r28, 0xc(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82E45D8C: 911B0010  stw r8, 0x10(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82E45D90: 7C67502E  lwzx r3, r7, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E45D94: 83A30060  lwz r29, 0x60(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E45D98: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E45D9C: 419A001C  beq cr6, 0x82e45db8
	if ctx.cr[6].eq {
	pc = 0x82E45DB8; continue 'dispatch;
	}
	// 82E45DA0: 81630064  lwz r11, 0x64(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E45DA4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E45DA8: 91630064  stw r11, 0x64(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82E45DAC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E45DB0: 91630060  stw r11, 0x60(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E45DB4: 48000010  b 0x82e45dc4
	pc = 0x82E45DC4; continue 'dispatch;
	// 82E45DB8: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82E45DBC: 4BF0F295  bl 0x82d55050
	ctx.lr = 0x82E45DC0;
	sub_82D55050(ctx, base);
	// 82E45DC0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E45DC4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E45DC8: 419A0070  beq cr6, 0x82e45e38
	if ctx.cr[6].eq {
	pc = 0x82E45E38; continue 'dispatch;
	}
	// 82E45DCC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82E45DD0: 3BFD0028  addi r31, r29, 0x28
	ctx.r[31].s64 = ctx.r[29].s64 + 40;
	// 82E45DD4: 394003E8  li r10, 0x3e8
	ctx.r[10].s64 = 1000;
	// 82E45DD8: 3BCBB0A0  addi r30, r11, -0x4f60
	ctx.r[30].s64 = ctx.r[11].s64 + -20320;
	// 82E45DDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E45DE0: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82E45DE4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E45DE8: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82E45DEC: 4B33EAA5  bl 0x82184890
	ctx.lr = 0x82E45DF0;
	sub_82184890(ctx, base);
	// 82E45DF0: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E45DF4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E45DF8: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E45DFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E45E00: 419A0008  beq cr6, 0x82e45e08
	if ctx.cr[6].eq {
	pc = 0x82E45E08; continue 'dispatch;
	}
	// 82E45E04: 93AB002C  stw r29, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 82E45E08: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E45E0C: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82E45E10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E45E14: 93BE0030  stw r29, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 82E45E18: F97E0020  std r11, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E45E1C: 48473B39  bl 0x832b9954
	ctx.lr = 0x82E45E20;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E45E20: 388003E8  li r4, 0x3e8
	ctx.r[4].s64 = 1000;
	// 82E45E24: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E45E28: 48473E6D  bl 0x832b9c94
	ctx.lr = 0x82E45E2C;
	// extern call 0x832B9C94 → crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount
	crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount(ctx, base);
	// 82E45E2C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E45E30: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 82E45E34: F97D0020  std r11, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82E45E38: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E45E3C: 939B0014  stw r28, 0x14(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 82E45E40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E45E44: 4BE63610  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45E48 size=116
    let mut pc: u32 = 0x82E45E48;
    'dispatch: loop {
        match pc {
            0x82E45E48 => {
    //   block [0x82E45E48..0x82E45EBC)
	// 82E45E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E45E4C: 4BE635C1  bl 0x82ca940c
	ctx.lr = 0x82E45E50;
	sub_82CA93D0(ctx, base);
	// 82E45E50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E45E54: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E45E58: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E45E5C: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45E60: 4B33EA31  bl 0x82184890
	ctx.lr = 0x82E45E64;
	sub_82184890(ctx, base);
	// 82E45E64: 3BFE0008  addi r31, r30, 8
	ctx.r[31].s64 = ctx.r[30].s64 + 8;
	// 82E45E68: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E45E6C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E45E70: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E45E74: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E45E78: 409A0010  bne cr6, 0x82e45e88
	if !ctx.cr[6].eq {
	pc = 0x82E45E88; continue 'dispatch;
	}
	// 82E45E7C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E45E80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E45E84: 4BF11115  bl 0x82d56f98
	ctx.lr = 0x82E45E88;
	sub_82D56F98(ctx, base);
	// 82E45E88: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E45E8C: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82E45E90: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E45E94: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E45E98: 7FAB512E  stwx r29, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u32) };
	// 82E45E9C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E45EA0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E45EA4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E45EA8: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E45EAC: F9230020  std r9, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[9].u64 ) };
	// 82E45EB0: 48473AA5  bl 0x832b9954
	ctx.lr = 0x82E45EB4;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82E45EB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E45EB8: 4BE635A4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E45EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E45EC0 size=384
    let mut pc: u32 = 0x82E45EC0;
    'dispatch: loop {
        match pc {
            0x82E45EC0 => {
    //   block [0x82E45EC0..0x82E46040)
	// 82E45EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E45EC4: 4BE63549  bl 0x82ca940c
	ctx.lr = 0x82E45EC8;
	sub_82CA93D0(ctx, base);
	// 82E45EC8: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82E45ECC: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E45ED0: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E45ED4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E45ED8: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82E45EDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E45EE0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E45EE4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E45EE8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E46040 size=92
    let mut pc: u32 = 0x82E46040;
    'dispatch: loop {
        match pc {
            0x82E46040 => {
    //   block [0x82E46040..0x82E4609C)
	// 82E46040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E46044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E46048: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4604C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E46050: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82E46054: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E46058: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4605C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E46060: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E46064: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E46068: 4BF152A1  bl 0x82d5b308
	ctx.lr = 0x82E4606C;
	sub_82D5B308(ctx, base);
	// 82E4606C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E46070: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E46074: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E46078: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4607C: 4BFFFE45  bl 0x82e45ec0
	ctx.lr = 0x82E46080;
	sub_82E45EC0(ctx, base);
	// 82E46080: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E46084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E46088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4608C: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82E46090: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E46094: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E46098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E460A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E460A0 size=64
    let mut pc: u32 = 0x82E460A0;
    'dispatch: loop {
        match pc {
            0x82E460A0 => {
    //   block [0x82E460A0..0x82E460E0)
	// 82E460A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E460A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E460A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E460AC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E460B0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E460B4: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 82E460B8: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	// 82E460BC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E460C0: 4BF0F189  bl 0x82d55248
	ctx.lr = 0x82E460C4;
	sub_82D55248(ctx, base);
	// 82E460C4: 39600018  li r11, 0x18
	ctx.r[11].s64 = 24;
	// 82E460C8: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82E460CC: 4BFFFC85  bl 0x82e45d50
	ctx.lr = 0x82E460D0;
	sub_82E45D50(ctx, base);
	// 82E460D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E460D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E460D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E460DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E460E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E460E0 size=128
    let mut pc: u32 = 0x82E460E0;
    'dispatch: loop {
        match pc {
            0x82E460E0 => {
    //   block [0x82E460E0..0x82E46160)
	// 82E460E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E460E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E460E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E460EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E460F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E460F4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E460F8: 396B5CCC  addi r11, r11, 0x5ccc
	ctx.r[11].s64 = ctx.r[11].s64 + 23756;
	// 82E460FC: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E46100: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E46104: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E46108: 419A000C  beq cr6, 0x82e46114
	if ctx.cr[6].eq {
	pc = 0x82E46114; continue 'dispatch;
	}
	// 82E4610C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E46110: 4BF39709  bl 0x82d7f818
	ctx.lr = 0x82E46114;
	sub_82D7F818(ctx, base);
	// 82E46114: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E46118: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4611C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E46120: 409A0020  bne cr6, 0x82e46140
	if !ctx.cr[6].eq {
	pc = 0x82E46140; continue 'dispatch;
	}
	// 82E46124: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E46128: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4612C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E46130: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E46134: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E46138: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4613C: 4BF0F18D  bl 0x82d552c8
	ctx.lr = 0x82E46140;
	sub_82D552C8(ctx, base);
	// 82E46140: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E46144: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E46148: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4614C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E46150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E46154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E46158: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4615C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E46160 size=100
    let mut pc: u32 = 0x82E46160;
    'dispatch: loop {
        match pc {
            0x82E46160 => {
    //   block [0x82E46160..0x82E461C4)
	// 82E46160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E46164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E46168: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4616C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E46170: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E46174: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E46178: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4617C: 4BFFFF65  bl 0x82e460e0
	ctx.lr = 0x82E46180;
	sub_82E460E0(ctx, base);
	// 82E46180: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E46184: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E46188: 419A0020  beq cr6, 0x82e461a8
	if ctx.cr[6].eq {
	pc = 0x82E461A8; continue 'dispatch;
	}
	// 82E4618C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E46190: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E46194: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 82E46198: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4619C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E461A0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E461A4: 4BF0F125  bl 0x82d552c8
	ctx.lr = 0x82E461A8;
	sub_82D552C8(ctx, base);
	// 82E461A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E461AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E461B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E461B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E461B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E461BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E461C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E461C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E461C8 size=140
    let mut pc: u32 = 0x82E461C8;
    'dispatch: loop {
        match pc {
            0x82E461C8 => {
    //   block [0x82E461C8..0x82E46254)
	// 82E461C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E461CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E461D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E461D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E461D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E461DC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E461E0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E461E4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E461E8: 409A002C  bne cr6, 0x82e46214
	if !ctx.cr[6].eq {
	pc = 0x82E46214; continue 'dispatch;
	}
	// 82E461EC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E461F0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E461F4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E461F8: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E461FC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E46200: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E46204: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E46208: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E4620C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E46210: 4BF0F0B9  bl 0x82d552c8
	ctx.lr = 0x82E46214;
	sub_82D552C8(ctx, base);
	// 82E46214: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E46218: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4621C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E46220: 409A0020  bne cr6, 0x82e46240
	if !ctx.cr[6].eq {
	pc = 0x82E46240; continue 'dispatch;
	}
	// 82E46224: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E46228: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4622C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E46230: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E46234: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E46238: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4623C: 4BF0F08D  bl 0x82d552c8
	ctx.lr = 0x82E46240;
	sub_82D552C8(ctx, base);
	// 82E46240: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E46244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E46248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4624C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E46250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E46258 size=152
    let mut pc: u32 = 0x82E46258;
    'dispatch: loop {
        match pc {
            0x82E46258 => {
    //   block [0x82E46258..0x82E462F0)
	// 82E46258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4625C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E46260: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E46264: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E46268: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4626C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E46270: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E46274: 4BFFFF55  bl 0x82e461c8
	ctx.lr = 0x82E46278;
	sub_82E461C8(ctx, base);
	// 82E46278: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4627C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E46280: 419A0054  beq cr6, 0x82e462d4
	if ctx.cr[6].eq {
	pc = 0x82E462D4; continue 'dispatch;
	}
	// 82E46284: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E46288: 419A004C  beq cr6, 0x82e462d4
	if ctx.cr[6].eq {
	pc = 0x82E462D4; continue 'dispatch;
	}
	// 82E4628C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E46290: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E46294: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E46298: 814B0054  lwz r10, 0x54(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E4629C: 812B0034  lwz r9, 0x34(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E462A0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E462A4: 41980018  blt cr6, 0x82e462bc
	if ctx.cr[6].lt {
	pc = 0x82E462BC; continue 'dispatch;
	}
	// 82E462A8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E462AC: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82E462B0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E462B4: 4BF0EE75  bl 0x82d55128
	ctx.lr = 0x82E462B8;
	sub_82D55128(ctx, base);
	// 82E462B8: 4800001C  b 0x82e462d4
	pc = 0x82E462D4; continue 'dispatch;
	// 82E462BC: 814B0054  lwz r10, 0x54(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E462C0: 812B0050  lwz r9, 0x50(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E462C4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E462C8: 914B0054  stw r10, 0x54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E462CC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E462D0: 93EB0050  stw r31, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E462D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E462D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E462DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E462E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E462E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E462E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E462EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E462F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E462F0 size=92
    let mut pc: u32 = 0x82E462F0;
    'dispatch: loop {
        match pc {
            0x82E462F0 => {
    //   block [0x82E462F0..0x82E4634C)
	// 82E462F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E462F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E462F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E462FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E46300: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E46304: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E46308: 396B5BAC  addi r11, r11, 0x5bac
	ctx.r[11].s64 = ctx.r[11].s64 + 23468;
	// 82E4630C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E46310: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E46314: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E46318: 419A0014  beq cr6, 0x82e4632c
	if ctx.cr[6].eq {
	pc = 0x82E4632C; continue 'dispatch;
	}
	// 82E4631C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E46320: 4BFFFF39  bl 0x82e46258
	ctx.lr = 0x82E46324;
	sub_82E46258(ctx, base);
	// 82E46324: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E46328: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E4632C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E46330: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82E46334: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E46338: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E4633C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E46340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E46344: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E46348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E46350 size=20
    let mut pc: u32 = 0x82E46350;
    'dispatch: loop {
        match pc {
            0x82E46350 => {
    //   block [0x82E46350..0x82E46364)
	// 82E46350: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E46354: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E46358: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4635C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E46360: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E46368 size=8
    let mut pc: u32 = 0x82E46368;
    'dispatch: loop {
        match pc {
            0x82E46368 => {
    //   block [0x82E46368..0x82E46370)
	// 82E46368: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4636C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E46370 size=24
    let mut pc: u32 = 0x82E46370;
    'dispatch: loop {
        match pc {
            0x82E46370 => {
    //   block [0x82E46370..0x82E46388)
	// 82E46370: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E46374: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E46378: 396B5D20  addi r11, r11, 0x5d20
	ctx.r[11].s64 = ctx.r[11].s64 + 23840;
	// 82E4637C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82E46380: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E46384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E46388 size=12
    let mut pc: u32 = 0x82E46388;
    'dispatch: loop {
        match pc {
            0x82E46388 => {
    //   block [0x82E46388..0x82E46394)
	// 82E46388: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4638C: 386B5D20  addi r3, r11, 0x5d20
	ctx.r[3].s64 = ctx.r[11].s64 + 23840;
	// 82E46390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E46398 size=100
    let mut pc: u32 = 0x82E46398;
    'dispatch: loop {
        match pc {
            0x82E46398 => {
    //   block [0x82E46398..0x82E463FC)
	// 82E46398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4639C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E463A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E463A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E463A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E463AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E463B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E463B4: 48005085  bl 0x82e4b438
	ctx.lr = 0x82E463B8;
	sub_82E4B438(ctx, base);
	// 82E463B8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E463BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E463C0: 419A0020  beq cr6, 0x82e463e0
	if ctx.cr[6].eq {
	pc = 0x82E463E0; continue 'dispatch;
	}
	// 82E463C4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E463C8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E463CC: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 82E463D0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E463D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E463D8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E463DC: 4BF0EEED  bl 0x82d552c8
	ctx.lr = 0x82E463E0;
	sub_82D552C8(ctx, base);
	// 82E463E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E463E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E463E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E463EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E463F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E463F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E463F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E46400 size=104
    let mut pc: u32 = 0x82E46400;
    'dispatch: loop {
        match pc {
            0x82E46400 => {
    //   block [0x82E46400..0x82E46468)
	// 82E46400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E46404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E46408: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4640C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E46410: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E46414: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E46418: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4641C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E46420: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E46424: 419A002C  beq cr6, 0x82e46450
	if ctx.cr[6].eq {
	pc = 0x82E46450; continue 'dispatch;
	}
	// 82E46428: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4642C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E46430: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E46434: 4E800421  bctrl
	ctx.lr = 0x82E46438;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E46438: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4643C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E46440: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E46444: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E46448: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4644C: 4E800421  bctrl
	ctx.lr = 0x82E46450;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E46450: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E46454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E46458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4645C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E46460: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E46464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E46468 size=72
    let mut pc: u32 = 0x82E46468;
    'dispatch: loop {
        match pc {
            0x82E46468 => {
    //   block [0x82E46468..0x82E464B0)
	// 82E46468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4646C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E46470: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E46474: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E46478: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4647C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E46480: 396B5D40  addi r11, r11, 0x5d40
	ctx.r[11].s64 = ctx.r[11].s64 + 23872;
	// 82E46484: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82E46488: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E4648C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E46490: 419A000C  beq cr6, 0x82e4649c
	if ctx.cr[6].eq {
	pc = 0x82E4649C; continue 'dispatch;
	}
	// 82E46494: 4B9FF31D  bl 0x828457b0
	ctx.lr = 0x82E46498;
	sub_828457B0(ctx, base);
	// 82E46498: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4649C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E464A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E464A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E464A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E464AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E464B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E464B0 size=72
    let mut pc: u32 = 0x82E464B0;
    'dispatch: loop {
        match pc {
            0x82E464B0 => {
    //   block [0x82E464B0..0x82E464F8)
	// 82E464B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E464B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E464B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E464BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E464C0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E464C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E464C8: 396B5D58  addi r11, r11, 0x5d58
	ctx.r[11].s64 = ctx.r[11].s64 + 23896;
	// 82E464CC: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82E464D0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E464D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E464D8: 419A000C  beq cr6, 0x82e464e4
	if ctx.cr[6].eq {
	pc = 0x82E464E4; continue 'dispatch;
	}
	// 82E464DC: 4B9FF2D5  bl 0x828457b0
	ctx.lr = 0x82E464E0;
	sub_828457B0(ctx, base);
	// 82E464E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E464E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E464E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E464EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E464F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E464F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E464F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E464F8 size=8
    let mut pc: u32 = 0x82E464F8;
    'dispatch: loop {
        match pc {
            0x82E464F8 => {
    //   block [0x82E464F8..0x82E46500)
	// 82E464F8: 38630074  addi r3, r3, 0x74
	ctx.r[3].s64 = ctx.r[3].s64 + 116;
	// 82E464FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E46500 size=16
    let mut pc: u32 = 0x82E46500;
    'dispatch: loop {
        match pc {
            0x82E46500 => {
    //   block [0x82E46500..0x82E46510)
	// 82E46500: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E46504: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E46508: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 82E4650C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E46510 size=20
    let mut pc: u32 = 0x82E46510;
    'dispatch: loop {
        match pc {
            0x82E46510 => {
    //   block [0x82E46510..0x82E46524)
	// 82E46510: C003005C  lfs f0, 0x5c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E46514: C1A30058  lfs f13, 0x58(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E46518: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E4651C: EC20682A  fadds f1, f0, f13
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82E46520: 4BF3A790  b 0x82d80cb0
	sub_82D80CB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E46528 size=8
    let mut pc: u32 = 0x82E46528;
    'dispatch: loop {
        match pc {
            0x82E46528 => {
    //   block [0x82E46528..0x82E46530)
	// 82E46528: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82E4652C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E46530 size=16
    let mut pc: u32 = 0x82E46530;
    'dispatch: loop {
        match pc {
            0x82E46530 => {
    //   block [0x82E46530..0x82E46540)
	// 82E46530: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E46540 size=8
    let mut pc: u32 = 0x82E46540;
    'dispatch: loop {
        match pc {
            0x82E46540 => {
    //   block [0x82E46540..0x82E46548)
	// 82E46540: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E46544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E46548 size=8
    let mut pc: u32 = 0x82E46548;
    'dispatch: loop {
        match pc {
            0x82E46548 => {
    //   block [0x82E46548..0x82E46550)
	// 82E46548: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E4654C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E46550 size=292
    let mut pc: u32 = 0x82E46550;
    'dispatch: loop {
        match pc {
            0x82E46550 => {
    //   block [0x82E46550..0x82E46674)
	// 82E46550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E46554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E46558: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4655C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E46560: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E46564: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E46568: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82E4656C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E46570: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82E46574: 39000040  li r8, 0x40
	ctx.r[8].s64 = 64;
	// 82E46578: 813F0030  lwz r9, 0x30(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E4657C: 81290018  lwz r9, 0x18(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E46678 size=560
    let mut pc: u32 = 0x82E46678;
    'dispatch: loop {
        match pc {
            0x82E46678 => {
    //   block [0x82E46678..0x82E468A8)
	// 82E46678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4667C: 4BE62D89  bl 0x82ca9404
	ctx.lr = 0x82E46680;
	sub_82CA93D0(ctx, base);
	// 82E46680: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E46684: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E46688: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4668C: 807E0048  lwz r3, 0x48(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E46690: 4BF39D31  bl 0x82d803c0
	ctx.lr = 0x82E46694;
	sub_82D803C0(ctx, base);
	// 82E46694: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E46698: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4669C: 419A0008  beq cr6, 0x82e466a4
	if ctx.cr[6].eq {
	pc = 0x82E466A4; continue 'dispatch;
	}
	// 82E466A0: 4BF39D41  bl 0x82d803e0
	ctx.lr = 0x82E466A4;
	sub_82D803E0(ctx, base);
	// 82E466A4: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E468A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E468A8 size=320
    let mut pc: u32 = 0x82E468A8;
    'dispatch: loop {
        match pc {
            0x82E468A8 => {
    //   block [0x82E468A8..0x82E469E8)
	// 82E468A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E468AC: 4BE62B61  bl 0x82ca940c
	ctx.lr = 0x82E468B0;
	sub_82CA93D0(ctx, base);
	// 82E468B0: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E468B4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E468B8: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82E468BC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E468C0: 54BD3032  slwi r29, r5, 6
	ctx.r[29].u32 = ctx.r[5].u32.wrapping_shl(6);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82E468C4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82E468C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E469E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E469E8 size=232
    let mut pc: u32 = 0x82E469E8;
    'dispatch: loop {
        match pc {
            0x82E469E8 => {
    //   block [0x82E469E8..0x82E46AD0)
	// 82E469E8: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82E469EC: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E46AD0 size=536
    let mut pc: u32 = 0x82E46AD0;
    'dispatch: loop {
        match pc {
            0x82E46AD0 => {
    //   block [0x82E46AD0..0x82E46CE8)
	// 82E46AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E46AD4: 4BE6292D  bl 0x82ca9400
	ctx.lr = 0x82E46AD8;
	sub_82CA93D0(ctx, base);
	// 82E46AD8: 38C40010  addi r6, r4, 0x10
	ctx.r[6].s64 = ctx.r[4].s64 + 16;
	// 82E46ADC: 39650010  addi r11, r5, 0x10
	ctx.r[11].s64 = ctx.r[5].s64 + 16;
	// 82E46AE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E46AE4: 3905000C  addi r8, r5, 0xc
	ctx.r[8].s64 = ctx.r[5].s64 + 12;
	// 82E46AE8: 3BE50030  addi r31, r5, 0x30
	ctx.r[31].s64 = ctx.r[5].s64 + 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E46CE8 size=128
    let mut pc: u32 = 0x82E46CE8;
    'dispatch: loop {
        match pc {
            0x82E46CE8 => {
    //   block [0x82E46CE8..0x82E46D68)
	// 82E46CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E46CEC: 4BE62719  bl 0x82ca9404
	ctx.lr = 0x82E46CF0;
	sub_82CA93D0(ctx, base);
	// 82E46CF0: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82E46CF4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E46CF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E46CFC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E46D00: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E46D04: 3B60FFFF  li r27, -1
	ctx.r[27].s64 = -1;
	// 82E46D08: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E46D0C: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E46D10: C3EB0BF8  lfs f31, 0xbf8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3064 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E46D14: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E46D18: 40990040  ble cr6, 0x82e46d58
	if !ctx.cr[6].gt {
	pc = 0x82E46D58; continue 'dispatch;
	}
	// 82E46D1C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E46D20: 817F0074  lwz r11, 0x74(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E46D24: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E46D28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E46D2C: 7CBE5A14  add r5, r30, r11
	ctx.r[5].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82E46D30: 4B433969  bl 0x8227a698
	ctx.lr = 0x82E46D34;
	sub_8227A698(ctx, base);
	// 82E46D34: FF01F800  fcmpu cr6, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 82E46D38: 4098000C  bge cr6, 0x82e46d44
	if !ctx.cr[6].lt {
	pc = 0x82E46D44; continue 'dispatch;
	}
	// 82E46D3C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E46D40: 7FBBEB78  mr r27, r29
	ctx.r[27].u64 = ctx.r[29].u64;
	// 82E46D44: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E46D48: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E46D4C: 3BDE0030  addi r30, r30, 0x30
	ctx.r[30].s64 = ctx.r[30].s64 + 48;
	// 82E46D50: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E46D54: 4198FFCC  blt cr6, 0x82e46d20
	if ctx.cr[6].lt {
	pc = 0x82E46D20; continue 'dispatch;
	}
	// 82E46D58: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E46D5C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E46D60: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82E46D64: 4BE626F0  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E46D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E46D68 size=1184
    let mut pc: u32 = 0x82E46D68;
    'dispatch: loop {
        match pc {
            0x82E46D68 => {
    //   block [0x82E46D68..0x82E47208)
	// 82E46D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E46D6C: 4BE62669  bl 0x82ca93d4
	ctx.lr = 0x82E46D70;
	sub_82CA93D0(ctx, base);
	// 82E46D70: 3981FF70  addi r12, r1, -0x90
	ctx.r[12].s64 = ctx.r[1].s64 + -144;
	// 82E46D74: 4BE66F61  bl 0x82cadcd4
	ctx.lr = 0x82E46D78;
	sub_82CADCA0(ctx, base);
	// 82E46D78: 9421FE20  stwu r1, -0x1e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-480 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E46D7C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E46D80: 7C912378  mr r17, r4
	ctx.r[17].u64 = ctx.r[4].u64;
	// 82E46D84: 7CAF2B78  mr r15, r5
	ctx.r[15].u64 = ctx.r[5].u64;
	// 82E46D88: 3A910008  addi r20, r17, 8
	ctx.r[20].s64 = ctx.r[17].s64 + 8;
	// 82E46D8C: 3A400000  li r18, 0
	ctx.r[18].s64 = 0;
	// 82E46D90: 817B0078  lwz r11, 0x78(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E46D94: 3A1B0078  addi r16, r27, 0x78
	ctx.r[16].s64 = ctx.r[27].s64 + 120;
	// 82E46D98: C0110008  lfs f0, 8(r17)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E46D9C: D0010094  stfs f0, 0x94(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 82E46DA0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E46DA4: 40990454  ble cr6, 0x82e471f8
	if !ctx.cr[6].gt {
	pc = 0x82E471F8; continue 'dispatch;
	}
	// 82E46DA8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82E46DAC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E46DB0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E46DB4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E46DB8: 3B3B0074  addi r25, r27, 0x74
	ctx.r[25].s64 = ctx.r[27].s64 + 116;
	// 82E46DBC: C3680A5C  lfs f27, 0xa5c(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2652 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 82E46DC0: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82E46DC4: C3890A78  lfs f28, 0xa78(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2680 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 82E46DC8: 3B000010  li r24, 0x10
	ctx.r[24].s64 = 16;
	// 82E46DCC: C3CA0C18  lfs f30, 0xc18(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82E46DD0: 3AC00060  li r22, 0x60
	ctx.r[22].s64 = 96;
	// 82E46DD4: C3AB0A74  lfs f29, 0xa74(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2676 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82E46DD8: 3AE000D0  li r23, 0xd0
	ctx.r[23].s64 = 208;
	// 82E46DDC: 3A600130  li r19, 0x130
	ctx.r[19].s64 = 304;
	// 82E46DE0: 3AA00190  li r21, 0x190
	ctx.r[21].s64 = 400;
	// 82E46DE4: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E46DE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E46DEC: 7D7A5A14  add r11, r26, r11
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 82E46DF0: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E46DF4: 892B0010  lbz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E46DF8: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 82E46DFC: 7D095A14  add r8, r9, r11
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82E46E00: 81280078  lwz r9, 0x78(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E46E04: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E46E08: 40990024  ble cr6, 0x82e46e2c
	if !ctx.cr[6].gt {
	pc = 0x82E46E2C; continue 'dispatch;
	}
	// 82E46E0C: 81680074  lwz r11, 0x74(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E46E10: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E46E14: 2B071300  cmplwi cr6, r7, 0x1300
	ctx.cr[6].compare_u32(ctx.r[7].u32, 4864 as u32, &mut ctx.xer);
	// 82E46E18: 419A02BC  beq cr6, 0x82e470d4
	if ctx.cr[6].eq {
	pc = 0x82E470D4; continue 'dispatch;
	}
	// 82E46E1C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E46E20: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E46E24: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E46E28: 4198FFE8  blt cr6, 0x82e46e10
	if ctx.cr[6].lt {
	pc = 0x82E46E10; continue 'dispatch;
	}
	// 82E46E2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E46E30: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82E46E34: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E46E38: 419A0094  beq cr6, 0x82e46ecc
	if ctx.cr[6].eq {
	pc = 0x82E46ECC; continue 'dispatch;
	}
	// 82E46E3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E46E40: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E46E44: 40990028  ble cr6, 0x82e46e6c
	if !ctx.cr[6].gt {
	pc = 0x82E46E6C; continue 'dispatch;
	}
	// 82E46E48: 81080074  lwz r8, 0x74(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E46E4C: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82E46E50: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E46E54: 2B071300  cmplwi cr6, r7, 0x1300
	ctx.cr[6].compare_u32(ctx.r[7].u32, 4864 as u32, &mut ctx.xer);
	// 82E46E58: 419A0284  beq cr6, 0x82e470dc
	if ctx.cr[6].eq {
	pc = 0x82E470DC; continue 'dispatch;
	}
	// 82E46E5C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E46E60: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82E46E64: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E46E68: 4198FFE8  blt cr6, 0x82e46e50
	if ctx.cr[6].lt {
	pc = 0x82E46E50; continue 'dispatch;
	}
	// 82E46E6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E46E70: 815B0084  lwz r10, 0x84(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E46E74: 557E003E  slwi r30, r11, 0
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82E46E78: 3BAAFFFF  addi r29, r10, -1
	ctx.r[29].s64 = ctx.r[10].s64 + -1;
	// 82E46E7C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82E46E80: 4198004C  blt cr6, 0x82e46ecc
	if ctx.cr[6].lt {
	pc = 0x82E46ECC; continue 'dispatch;
	}
	// 82E46E84: 57BF103A  slwi r31, r29, 2
	ctx.r[31].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82E46E88: 817B0080  lwz r11, 0x80(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E46E8C: 7D5F582E  lwzx r10, r31, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E46E90: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E46E94: 419A0028  beq cr6, 0x82e46ebc
	if ctx.cr[6].eq {
	pc = 0x82E46EBC; continue 'dispatch;
	}
	// 82E46E98: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82E46E9C: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E46EA0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E46EA4: 7CDA5A14  add r6, r26, r11
	ctx.r[6].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 82E46EA8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E46EAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E46EB0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E46EB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E46EB8: 4E800421  bctrl
	ctx.lr = 0x82E46EBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E46EBC: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 82E46EC0: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 82E46EC4: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82E46EC8: 4098FFC0  bge cr6, 0x82e46e88
	if !ctx.cr[6].lt {
	pc = 0x82E46E88; continue 'dispatch;
	}
	// 82E46ECC: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E46ED0: 7D7A5A14  add r11, r26, r11
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 82E46ED4: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E46ED8: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E46EDC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E46EE0: 409A0304  bne cr6, 0x82e471e4
	if !ctx.cr[6].eq {
	pc = 0x82E471E4; continue 'dispatch;
	}
	// 82E46EE4: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E46EE8: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82E46EEC: 7FAA5A14  add r29, r10, r11
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E46EF0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E46EF4: 419A02F0  beq cr6, 0x82e471e4
	if ctx.cr[6].eq {
	pc = 0x82E471E4; continue 'dispatch;
	}
	// 82E46EF8: 897D00D8  lbz r11, 0xd8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(216 as u32) ) } as u64;
	// 82E46EFC: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82E46F00: 419A0010  beq cr6, 0x82e46f10
	if ctx.cr[6].eq {
	pc = 0x82E46F10; continue 'dispatch;
	}
	// 82E46F04: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82E46F08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E46F0C: 409A0008  bne cr6, 0x82e46f14
	if !ctx.cr[6].eq {
	pc = 0x82E46F14; continue 'dispatch;
	}
	// 82E46F10: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E46F14: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82E46F18: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E46F1C: 409A02C8  bne cr6, 0x82e471e4
	if !ctx.cr[6].eq {
	pc = 0x82E471E4; continue 'dispatch;
	}
	// 82E46F20: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E46F24: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82E46F28: 3BFD00D0  addi r31, r29, 0xd0
	ctx.r[31].s64 = ctx.r[29].s64 + 208;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E47208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E47208 size=88
    let mut pc: u32 = 0x82E47208;
    'dispatch: loop {
        match pc {
            0x82E47208 => {
    //   block [0x82E47208..0x82E47260)
	// 82E47208: 81230084  lwz r9, 0x84(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E4720C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E47210: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E47214: 40990024  ble cr6, 0x82e47238
	if !ctx.cr[6].gt {
	pc = 0x82E47238; continue 'dispatch;
	}
	// 82E47218: 81430080  lwz r10, 0x80(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E4721C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E47220: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82E47224: 419A0018  beq cr6, 0x82e4723c
	if ctx.cr[6].eq {
	pc = 0x82E4723C; continue 'dispatch;
	}
	// 82E47228: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E4722C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E47230: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E47234: 4198FFE8  blt cr6, 0x82e4721c
	if ctx.cr[6].lt {
	pc = 0x82E4721C; continue 'dispatch;
	}
	// 82E47238: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E4723C: 81430084  lwz r10, 0x84(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E47240: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E47244: 81630080  lwz r11, 0x80(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E47248: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E4724C: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E47250: 91430084  stw r10, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[10].u32 ) };
	// 82E47254: 7D48582E  lwzx r10, r8, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E47258: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82E4725C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E47260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E47260 size=100
    let mut pc: u32 = 0x82E47260;
    'dispatch: loop {
        match pc {
            0x82E47260 => {
    //   block [0x82E47260..0x82E472C4)
	// 82E47260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E47264: 4BE621A1  bl 0x82ca9404
	ctx.lr = 0x82E47268;
	sub_82CA93D0(ctx, base);
	// 82E47268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4726C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E47270: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E47274: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E47278: 817D0084  lwz r11, 0x84(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E4727C: 3BEBFFFF  addi r31, r11, -1
	ctx.r[31].s64 = ctx.r[11].s64 + -1;
	// 82E47280: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E47284: 41980038  blt cr6, 0x82e472bc
	if ctx.cr[6].lt {
	pc = 0x82E472BC; continue 'dispatch;
	}
	// 82E47288: 57FE103A  slwi r30, r31, 2
	ctx.r[30].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82E4728C: 817D0080  lwz r11, 0x80(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E47290: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E47294: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E47298: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E4729C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E472A0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E472A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E472A8: 4E800421  bctrl
	ctx.lr = 0x82E472AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E472AC: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82E472B0: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 82E472B4: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E472B8: 4098FFD4  bge cr6, 0x82e4728c
	if !ctx.cr[6].lt {
	pc = 0x82E4728C; continue 'dispatch;
	}
	// 82E472BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E472C0: 4BE62194  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E472C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E472C8 size=92
    let mut pc: u32 = 0x82E472C8;
    'dispatch: loop {
        match pc {
            0x82E472C8 => {
    //   block [0x82E472C8..0x82E47324)
	// 82E472C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E472CC: 4BE6213D  bl 0x82ca9408
	ctx.lr = 0x82E472D0;
	sub_82CA93D0(ctx, base);
	// 82E472D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E472D4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E472D8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E472DC: 817D0084  lwz r11, 0x84(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E472E0: 3BEBFFFF  addi r31, r11, -1
	ctx.r[31].s64 = ctx.r[11].s64 + -1;
	// 82E472E4: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E472E8: 41980034  blt cr6, 0x82e4731c
	if ctx.cr[6].lt {
	pc = 0x82E4731C; continue 'dispatch;
	}
	// 82E472EC: 57FE103A  slwi r30, r31, 2
	ctx.r[30].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82E472F0: 817D0080  lwz r11, 0x80(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E472F4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E472F8: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E472FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E47300: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E47304: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E47308: 4E800421  bctrl
	ctx.lr = 0x82E4730C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4730C: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82E47310: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 82E47314: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E47318: 4098FFD8  bge cr6, 0x82e472f0
	if !ctx.cr[6].lt {
	pc = 0x82E472F0; continue 'dispatch;
	}
	// 82E4731C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E47320: 4BE62138  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E47328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E47328 size=92
    let mut pc: u32 = 0x82E47328;
    'dispatch: loop {
        match pc {
            0x82E47328 => {
    //   block [0x82E47328..0x82E47384)
	// 82E47328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4732C: 4BE620DD  bl 0x82ca9408
	ctx.lr = 0x82E47330;
	sub_82CA93D0(ctx, base);
	// 82E47330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E47334: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E47338: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E4733C: 817D0084  lwz r11, 0x84(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E47340: 3BEBFFFF  addi r31, r11, -1
	ctx.r[31].s64 = ctx.r[11].s64 + -1;
	// 82E47344: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E47348: 41980034  blt cr6, 0x82e4737c
	if ctx.cr[6].lt {
	pc = 0x82E4737C; continue 'dispatch;
	}
	// 82E4734C: 57FE103A  slwi r30, r31, 2
	ctx.r[30].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82E47350: 817D0080  lwz r11, 0x80(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E47354: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E47358: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E4735C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E47360: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E47364: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E47368: 4E800421  bctrl
	ctx.lr = 0x82E4736C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4736C: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82E47370: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 82E47374: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E47378: 4098FFD8  bge cr6, 0x82e47350
	if !ctx.cr[6].lt {
	pc = 0x82E47350; continue 'dispatch;
	}
	// 82E4737C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E47380: 4BE620D8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E47388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E47388 size=176
    let mut pc: u32 = 0x82E47388;
    'dispatch: loop {
        match pc {
            0x82E47388 => {
    //   block [0x82E47388..0x82E47438)
	// 82E47388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4738C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E47390: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E47394: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E47398: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4739C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E473A0: 394B5D40  addi r10, r11, 0x5d40
	ctx.r[10].s64 = ctx.r[11].s64 + 23872;
	// 82E473A4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E473A8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E473AC: 392B5D58  addi r9, r11, 0x5d58
	ctx.r[9].s64 = ctx.r[11].s64 + 23896;
	// 82E473B0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E473B4: 390B17E0  addi r8, r11, 0x17e0
	ctx.r[8].s64 = ctx.r[11].s64 + 6112;
	// 82E473B8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E473BC: B0BF0006  sth r5, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[5].u16 ) };
	// 82E473C0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E473C4: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E473C8: 38EB5D88  addi r7, r11, 0x5d88
	ctx.r[7].s64 = ctx.r[11].s64 + 23944;
	// 82E473CC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82E473D0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E473D4: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E473D8: 38CB5D70  addi r6, r11, 0x5d70
	ctx.r[6].s64 = ctx.r[11].s64 + 23920;
	// 82E473DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E473E0: 90FF0008  stw r7, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82E473E4: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82E473E8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82E473EC: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82E473F0: 917F0078  stw r11, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82E473F4: 915F007C  stw r10, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 82E473F8: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82E473FC: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82E47400: 915F0088  stw r10, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82E47404: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82E47408: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82E4740C: 915F0094  stw r10, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 82E47410: 917F0098  stw r11, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 82E47414: 917F009C  stw r11, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 82E47418: 915F00A0  stw r10, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[10].u32 ) };
	// 82E4741C: 4BFFF25D  bl 0x82e46678
	ctx.lr = 0x82E47420;
	sub_82E46678(ctx, base);
	// 82E47420: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E47424: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E47428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4742C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E47430: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E47434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E47438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E47438 size=440
    let mut pc: u32 = 0x82E47438;
    'dispatch: loop {
        match pc {
            0x82E47438 => {
    //   block [0x82E47438..0x82E475F0)
	// 82E47438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4743C: 4BE61FC5  bl 0x82ca9400
	ctx.lr = 0x82E47440;
	sub_82CA93D0(ctx, base);
	// 82E47440: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E47444: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E47448: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4744C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E47450: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E47454: 396B17E0  addi r11, r11, 0x17e0
	ctx.r[11].s64 = ctx.r[11].s64 + 6112;
	// 82E47458: 394A5D88  addi r10, r10, 0x5d88
	ctx.r[10].s64 = ctx.r[10].s64 + 23944;
	// 82E4745C: 811F0090  lwz r8, 0x90(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E47460: 39295D70  addi r9, r9, 0x5d70
	ctx.r[9].s64 = ctx.r[9].s64 + 23920;
	// 82E47464: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82E47468: 3B9F0008  addi r28, r31, 8
	ctx.r[28].s64 = ctx.r[31].s64 + 8;
	// 82E4746C: 3B7F000C  addi r27, r31, 0xc
	ctx.r[27].s64 = ctx.r[31].s64 + 12;
	// 82E47470: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E47474: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 82E47478: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E4747C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82E47480: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E47484: 4099002C  ble cr6, 0x82e474b0
	if !ctx.cr[6].gt {
	pc = 0x82E474B0; continue 'dispatch;
	}
	// 82E47488: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82E4748C: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82E47490: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E47494: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82E47498: 4BF3E981  bl 0x82d85e18
	ctx.lr = 0x82E4749C;
	sub_82D85E18(ctx, base);
	// 82E4749C: 817F0090  lwz r11, 0x90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E474A0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E474A4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E474A8: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E474AC: 4198FFE0  blt cr6, 0x82e4748c
	if ctx.cr[6].lt {
	pc = 0x82E4748C; continue 'dispatch;
	}
	// 82E474B0: 817F009C  lwz r11, 0x9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82E474B4: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82E474B8: 935F0090  stw r26, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[26].u32 ) };
	// 82E474BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E474C0: 4099002C  ble cr6, 0x82e474ec
	if !ctx.cr[6].gt {
	pc = 0x82E474EC; continue 'dispatch;
	}
	// 82E474C4: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 82E474C8: 817F0098  lwz r11, 0x98(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82E474CC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E474D0: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E474D4: 4BF523DD  bl 0x82d998b0
	ctx.lr = 0x82E474D8;
	sub_82D998B0(ctx, base);
	// 82E474D8: 817F009C  lwz r11, 0x9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82E474DC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E474E0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E474E4: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E474E8: 4198FFE0  blt cr6, 0x82e474c8
	if ctx.cr[6].lt {
	pc = 0x82E474C8; continue 'dispatch;
	}
	// 82E474EC: 38A01300  li r5, 0x1300
	ctx.r[5].s64 = 4864;
	// 82E474F0: 809F0030  lwz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E474F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E474F8: 935F009C  stw r26, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[26].u32 ) };
	// 82E474FC: 4BF38FCD  bl 0x82d804c8
	ctx.lr = 0x82E47500;
	sub_82D804C8(ctx, base);
	// 82E47500: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E47504: 4BF38EDD  bl 0x82d803e0
	ctx.lr = 0x82E47508;
	sub_82D803E0(ctx, base);
	// 82E47508: 817F00A0  lwz r11, 0xa0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 82E4750C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E47510: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E47514: 409A0020  bne cr6, 0x82e47534
	if !ctx.cr[6].eq {
	pc = 0x82E47534; continue 'dispatch;
	}
	// 82E47518: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4751C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E47520: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E47524: 809F0098  lwz r4, 0x98(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82E47528: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4752C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E47530: 4BF0DD99  bl 0x82d552c8
	ctx.lr = 0x82E47534;
	sub_82D552C8(ctx, base);
	// 82E47534: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82E47538: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4753C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E47540: 409A0020  bne cr6, 0x82e47560
	if !ctx.cr[6].eq {
	pc = 0x82E47560; continue 'dispatch;
	}
	// 82E47544: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E47548: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4754C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E47550: 809F008C  lwz r4, 0x8c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82E47554: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E47558: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4755C: 4BF0DD6D  bl 0x82d552c8
	ctx.lr = 0x82E47560;
	sub_82D552C8(ctx, base);
	// 82E47560: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E47564: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E47568: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4756C: 409A0020  bne cr6, 0x82e4758c
	if !ctx.cr[6].eq {
	pc = 0x82E4758C; continue 'dispatch;
	}
	// 82E47570: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E47574: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E47578: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4757C: 809F0080  lwz r4, 0x80(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E47580: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E47584: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E47588: 4BF0DD41  bl 0x82d552c8
	ctx.lr = 0x82E4758C;
	sub_82D552C8(ctx, base);
	// 82E4758C: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82E47590: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E47594: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E47598: 409A002C  bne cr6, 0x82e475c4
	if !ctx.cr[6].eq {
	pc = 0x82E475C4; continue 'dispatch;
	}
	// 82E4759C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E475A0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E475A4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E475A8: 809F0074  lwz r4, 0x74(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E475AC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E475B0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E475B4: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E475B8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E475BC: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E475C0: 4BF0DD09  bl 0x82d552c8
	ctx.lr = 0x82E475C4;
	sub_82D552C8(ctx, base);
	// 82E475C4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E475C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E475CC: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82E475D0: 396B5D58  addi r11, r11, 0x5d58
	ctx.r[11].s64 = ctx.r[11].s64 + 23896;
	// 82E475D4: 394A5D40  addi r10, r10, 0x5d40
	ctx.r[10].s64 = ctx.r[10].s64 + 23872;
	// 82E475D8: 392939E0  addi r9, r9, 0x39e0
	ctx.r[9].s64 = ctx.r[9].s64 + 14816;
	// 82E475DC: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E475E0: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E475E4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E475E8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E475EC: 4BE61E64  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E475F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E475F0 size=112
    let mut pc: u32 = 0x82E475F0;
    'dispatch: loop {
        match pc {
            0x82E475F0 => {
    //   block [0x82E475F0..0x82E47660)
	// 82E475F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E475F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E475F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E475FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E47600: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E47604: 3BE30080  addi r31, r3, 0x80
	ctx.r[31].s64 = ctx.r[3].s64 + 128;
	// 82E47608: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4760C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E47610: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E47614: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E47618: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4761C: 409A0010  bne cr6, 0x82e4762c
	if !ctx.cr[6].eq {
	pc = 0x82E4762C; continue 'dispatch;
	}
	// 82E47620: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E47624: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E47628: 4BF0F971  bl 0x82d56f98
	ctx.lr = 0x82E4762C;
	sub_82D56F98(ctx, base);
	// 82E4762C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E47630: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E47634: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E47638: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82E4763C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E47640: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E47644: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E47648: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4764C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E47650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E47654: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E47658: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4765C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E47660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E47660 size=344
    let mut pc: u32 = 0x82E47660;
    'dispatch: loop {
        match pc {
            0x82E47660 => {
    //   block [0x82E47660..0x82E477B8)
	// 82E47660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E47664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E47668: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4766C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E47670: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E47674: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E47678: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4767C: 357FFFF8  addic. r11, r31, -8
	ctx.xer.ca = (ctx.r[31].u32 > (!(-8 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + -8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E47680: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E47684: 40820008  bne 0x82e4768c
	if !ctx.cr[0].eq {
	pc = 0x82E4768C; continue 'dispatch;
	}
	// 82E47688: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4768C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E47690: 4BF3E789  bl 0x82d85e18
	ctx.lr = 0x82E47694;
	sub_82D85E18(ctx, base);
	// 82E47694: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E47698: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 82E4769C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82E476A0: 419800AC  blt cr6, 0x82e4774c
	if ctx.cr[6].lt {
	pc = 0x82E4774C; continue 'dispatch;
	}
	// 82E476A4: 54AB083C  slwi r11, r5, 1
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E476A8: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82E476AC: 7D655A14  add r11, r5, r11
	ctx.r[11].u64 = ctx.r[5].u64 + ctx.r[11].u64;
	// 82E476B0: 55672036  slwi r7, r11, 4
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82E476B4: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E476B8: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E476BC: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E476C0: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E476C4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E476C8: 409A0014  bne cr6, 0x82e476dc
	if !ctx.cr[6].eq {
	pc = 0x82E476DC; continue 'dispatch;
	}
	// 82E476CC: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E476D0: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82E476D4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E476D8: 48000008  b 0x82e476e0
	pc = 0x82E476E0; continue 'dispatch;
	// 82E476DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E476E0: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E476E4: 409A0058  bne cr6, 0x82e4773c
	if !ctx.cr[6].eq {
	pc = 0x82E4773C; continue 'dispatch;
	}
	// 82E476E8: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E476EC: 815F006C  lwz r10, 0x6c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E476F0: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 82E476F4: 7D675214  add r11, r7, r10
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 82E476F8: 5528083C  slwi r8, r9, 1
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E476FC: 7D094214  add r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82E47700: 913F0070  stw r9, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 82E47704: 55082036  slwi r8, r8, 4
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E47708: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E477B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E477B8 size=344
    let mut pc: u32 = 0x82E477B8;
    'dispatch: loop {
        match pc {
            0x82E477B8 => {
    //   block [0x82E477B8..0x82E47910)
	// 82E477B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E477BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E477C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E477C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E477C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E477CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E477D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E477D4: 357FFFF4  addic. r11, r31, -0xc
	ctx.xer.ca = (ctx.r[31].u32 > (!(-12 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + -12;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E477D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E477DC: 40820008  bne 0x82e477e4
	if !ctx.cr[0].eq {
	pc = 0x82E477E4; continue 'dispatch;
	}
	// 82E477E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E477E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E477E8: 4BF520C9  bl 0x82d998b0
	ctx.lr = 0x82E477EC;
	sub_82D998B0(ctx, base);
	// 82E477EC: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E477F0: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 82E477F4: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82E477F8: 419800AC  blt cr6, 0x82e478a4
	if ctx.cr[6].lt {
	pc = 0x82E478A4; continue 'dispatch;
	}
	// 82E477FC: 54AB083C  slwi r11, r5, 1
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E47800: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82E47804: 7D655A14  add r11, r5, r11
	ctx.r[11].u64 = ctx.r[5].u64 + ctx.r[11].u64;
	// 82E47808: 55672036  slwi r7, r11, 4
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82E4780C: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E47810: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E47814: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E47818: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E4781C: 2B0A0002  cmplwi cr6, r10, 2
	ctx.cr[6].compare_u32(ctx.r[10].u32, 2 as u32, &mut ctx.xer);
	// 82E47820: 409A0014  bne cr6, 0x82e47834
	if !ctx.cr[6].eq {
	pc = 0x82E47834; continue 'dispatch;
	}
	// 82E47824: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E47828: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82E4782C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E47830: 48000008  b 0x82e47838
	pc = 0x82E47838; continue 'dispatch;
	// 82E47834: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E47838: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E4783C: 409A0058  bne cr6, 0x82e47894
	if !ctx.cr[6].eq {
	pc = 0x82E47894; continue 'dispatch;
	}
	// 82E47840: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E47844: 815F0068  lwz r10, 0x68(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E47848: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 82E4784C: 7D675214  add r11, r7, r10
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 82E47850: 5528083C  slwi r8, r9, 1
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E47854: 7D094214  add r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82E47858: 913F006C  stw r9, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82E4785C: 55082036  slwi r8, r8, 4
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E47860: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E47910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E47910 size=304
    let mut pc: u32 = 0x82E47910;
    'dispatch: loop {
        match pc {
            0x82E47910 => {
    //   block [0x82E47910..0x82E47A40)
	// 82E47910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E47914: 4BE61AF5  bl 0x82ca9408
	ctx.lr = 0x82E47918;
	sub_82CA93D0(ctx, base);
	// 82E47918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4791C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E47920: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E47924: 815F007C  lwz r10, 0x7c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82E47928: 554B0000  rlwinm r11, r10, 0, 0, 0
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4792C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E47930: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E47934: 3BAB5DB8  addi r29, r11, 0x5db8
	ctx.r[29].s64 = ctx.r[11].s64 + 23992;
	// 82E47938: 409A0044  bne cr6, 0x82e4797c
	if !ctx.cr[6].eq {
	pc = 0x82E4797C; continue 'dispatch;
	}
	// 82E4793C: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E47940: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82E47944: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E47948: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E4794C: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E47950: 80DF0074  lwz r6, 0x74(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E47954: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E47958: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82E4795C: 83890008  lwz r28, 8(r9)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E47960: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E47964: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E47968: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82E4796C: 55482036  slwi r8, r10, 4
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E47970: 55672036  slwi r7, r11, 4
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82E47974: 7F8903A6  mtctr r28
	ctx.ctr.u64 = ctx.r[28].u64;
	// 82E47978: 4E800421  bctrl
	ctx.lr = 0x82E4797C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4797C: 815F0094  lwz r10, 0x94(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82E47980: 554B0000  rlwinm r11, r10, 0, 0, 0
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E47984: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E47988: 409A0034  bne cr6, 0x82e479bc
	if !ctx.cr[6].eq {
	pc = 0x82E479BC; continue 'dispatch;
	}
	// 82E4798C: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E47990: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E47994: 80FF0090  lwz r7, 0x90(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E47998: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E4799C: 388B5DA8  addi r4, r11, 0x5da8
	ctx.r[4].s64 = ctx.r[11].s64 + 23976;
	// 82E479A0: 80DF008C  lwz r6, 0x8c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82E479A4: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82E479A8: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E479AC: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E479B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E479B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E479B8: 4E800421  bctrl
	ctx.lr = 0x82E479BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E479BC: 817F00A0  lwz r11, 0xa0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 82E479C0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E479C4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E479C8: 409A0030  bne cr6, 0x82e479f8
	if !ctx.cr[6].eq {
	pc = 0x82E479F8; continue 'dispatch;
	}
	// 82E479CC: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E479D0: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E479D4: 813F009C  lwz r9, 0x9c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82E479D8: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E479DC: 80DF0098  lwz r6, 0x98(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82E479E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E479E4: 5527103A  slwi r7, r9, 2
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82E479E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E479EC: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E479F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E479F4: 4E800421  bctrl
	ctx.lr = 0x82E479F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E479F8: 815F0088  lwz r10, 0x88(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E479FC: 554B0000  rlwinm r11, r10, 0, 0, 0
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E47A00: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E47A04: 409A0034  bne cr6, 0x82e47a38
	if !ctx.cr[6].eq {
	pc = 0x82E47A38; continue 'dispatch;
	}
	// 82E47A08: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E47A0C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E47A10: 80FF0084  lwz r7, 0x84(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E47A14: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E47A18: 388B5D9C  addi r4, r11, 0x5d9c
	ctx.r[4].s64 = ctx.r[11].s64 + 23964;
	// 82E47A1C: 80DF0080  lwz r6, 0x80(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E47A20: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82E47A24: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E47A28: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E47A2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E47A30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E47A34: 4E800421  bctrl
	ctx.lr = 0x82E47A38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E47A38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E47A3C: 4BE61A1C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E47A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E47A40 size=280
    let mut pc: u32 = 0x82E47A40;
    'dispatch: loop {
        match pc {
            0x82E47A40 => {
    //   block [0x82E47A40..0x82E47B58)
	// 82E47A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E47A44: 4BE619C9  bl 0x82ca940c
	ctx.lr = 0x82E47A48;
	sub_82CA93D0(ctx, base);
	// 82E47A48: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E47A4C: 9421FD90  stwu r1, -0x270(r1)
	ea = ctx.r[1].u32.wrapping_add(-624 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E47A50: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E47A54: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E47A58: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E47A5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E47A60: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82E47A64: C3EB0C64  lfs f31, 0xc64(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3172 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E47A68: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82E47A6C: D3FE0004  stfs f31, 4(r30)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E47A70: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E47A74: 93BE0014  stw r29, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 82E47A78: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E47A7C: C1BF0058  lfs f13, 0x58(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E47A80: C01F005C  lfs f0, 0x5c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E47A84: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E47A88: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82E47A8C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E47A90: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E47A94: 394B0020  addi r10, r11, 0x20
	ctx.r[10].s64 = ctx.r[11].s64 + 32;
	// 82E47A98: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E47A9C: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82E47AA0: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E47AA4: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82E47AA8: 80EB0008  lwz r7, 8(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E47AAC: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82E47AB0: 80EB000C  lwz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E47AB4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82E47AB8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E47ABC: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E47AC0: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E47B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E47B58 size=2872
    let mut pc: u32 = 0x82E47B58;
    'dispatch: loop {
        match pc {
            0x82E47B58 => {
    //   block [0x82E47B58..0x82E48690)
	// 82E47B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E47B5C: 4BE61879  bl 0x82ca93d4
	ctx.lr = 0x82E47B60;
	sub_82CA93D0(ctx, base);
	// 82E47B60: DBA1FF58  stfd f29, -0xa8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[29].u64 ) };
	// 82E47B64: DBC1FF60  stfd f30, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[30].u64 ) };
	// 82E47B68: DBE1FF68  stfd f31, -0x98(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-152 as u32), ctx.f[31].u64 ) };
	// 82E47B6C: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E47B70: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E47B74: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 82E47B78: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E47B7C: 7CB52B78  mr r21, r5
	ctx.r[21].u64 = ctx.r[5].u64;
	// 82E47B80: 7E7C9B78  mr r28, r19
	ctx.r[28].u64 = ctx.r[19].u64;
	// 82E47B84: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E47B88: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E47B8C: 40990030  ble cr6, 0x82e47bbc
	if !ctx.cr[6].gt {
	pc = 0x82E47BBC; continue 'dispatch;
	}
	// 82E47B90: 3B7E0008  addi r27, r30, 8
	ctx.r[27].s64 = ctx.r[30].s64 + 8;
	// 82E47B94: 7E7D9B78  mr r29, r19
	ctx.r[29].u64 = ctx.r[19].u64;
	// 82E47B98: 817E008C  lwz r11, 0x8c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 82E47B9C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E47BA0: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82E47BA4: 4BF3E275  bl 0x82d85e18
	ctx.lr = 0x82E47BA8;
	sub_82D85E18(ctx, base);
	// 82E47BA8: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E47BAC: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82E47BB0: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E47BB4: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E47BB8: 4198FFE0  blt cr6, 0x82e47b98
	if ctx.cr[6].lt {
	pc = 0x82E47B98; continue 'dispatch;
	}
	// 82E47BBC: 817E009C  lwz r11, 0x9c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(156 as u32) ) } as u64;
	// 82E47BC0: 7E7C9B78  mr r28, r19
	ctx.r[28].u64 = ctx.r[19].u64;
	// 82E47BC4: 927E0090  stw r19, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[19].u32 ) };
	// 82E47BC8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E47BCC: 40990030  ble cr6, 0x82e47bfc
	if !ctx.cr[6].gt {
	pc = 0x82E47BFC; continue 'dispatch;
	}
	// 82E47BD0: 3B7E000C  addi r27, r30, 0xc
	ctx.r[27].s64 = ctx.r[30].s64 + 12;
	// 82E47BD4: 7E7D9B78  mr r29, r19
	ctx.r[29].u64 = ctx.r[19].u64;
	// 82E47BD8: 817E0098  lwz r11, 0x98(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(152 as u32) ) } as u64;
	// 82E47BDC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E47BE0: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82E47BE4: 4BF51CCD  bl 0x82d998b0
	ctx.lr = 0x82E47BE8;
	sub_82D998B0(ctx, base);
	// 82E47BE8: 817E009C  lwz r11, 0x9c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(156 as u32) ) } as u64;
	// 82E47BEC: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82E47BF0: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E47BF4: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E47BF8: 4198FFE0  blt cr6, 0x82e47bd8
	if ctx.cr[6].lt {
	pc = 0x82E47BD8; continue 'dispatch;
	}
	// 82E47BFC: 927E009C  stw r19, 0x9c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(156 as u32), ctx.r[19].u32 ) };
	// 82E47C00: 3A400004  li r18, 4
	ctx.r[18].s64 = 4;
	// 82E47C04: 83BF0014  lwz r29, 0x14(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E47C08: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E47C0C: 822D0000  lwz r17, 0(r13)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E47C10: 57AA083C  slwi r10, r29, 1
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E47C14: 7D5D5214  add r10, r29, r10
	ctx.r[10].u64 = ctx.r[29].u64 + ctx.r[10].u64;
	// 82E47C18: C3EB0C64  lfs f31, 0xc64(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3172 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E47C1C: 7C72882E  lwzx r3, r18, r17
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[18].u32.wrapping_add(ctx.r[17].u32)) } as u64;
	// 82E47C20: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E47C24: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82E47C28: 55440036  rlwinm r4, r10, 0, 0, 0x1b
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E47C2C: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E47C30: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E47C34: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82E47C38: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E47C3C: 41990010  bgt cr6, 0x82e47c4c
	if ctx.cr[6].gt {
	pc = 0x82E47C4C; continue 'dispatch;
	}
	// 82E47C40: 7D745B78  mr r20, r11
	ctx.r[20].u64 = ctx.r[11].u64;
	// 82E47C44: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82E47C48: 48000018  b 0x82e47c60
	pc = 0x82E47C60; continue 'dispatch;
	// 82E47C4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E47C50: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E47C54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E47C58: 4E800421  bctrl
	ctx.lr = 0x82E47C5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E47C5C: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
	// 82E47C60: 82FF0014  lwz r23, 0x14(r31)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E47C64: 7E669B78  mr r6, r19
	ctx.r[6].u64 = ctx.r[19].u64;
	// 82E47C68: 67B08000  oris r16, r29, 0x8000
	ctx.r[16].u64 = ctx.r[29].u64 | 2147483648;
	// 82E47C6C: 3AC00010  li r22, 0x10
	ctx.r[22].s64 = 16;
	// 82E47C70: 3B00FFE0  li r24, -0x20
	ctx.r[24].s64 = -32;
	// 82E47C74: 3B20FFF0  li r25, -0x10
	ctx.r[25].s64 = -16;
	// 82E47C78: 2F170004  cmpwi cr6, r23, 4
	ctx.cr[6].compare_i32(ctx.r[23].s32, 4, &mut ctx.xer);
	// 82E47C7C: 41980178  blt cr6, 0x82e47df4
	if ctx.cr[6].lt {
	pc = 0x82E47DF4; continue 'dispatch;
	}
	// 82E47C80: 3977FFFC  addi r11, r23, -4
	ctx.r[11].s64 = ctx.r[23].s64 + -4;
	// 82E47C84: 20B40010  subfic r5, r20, 0x10
	ctx.xer.ca = ctx.r[20].u32 <= 16 as u32;
	ctx.r[5].s64 = (16 as i64) - ctx.r[20].s64;
	// 82E47C88: 556BF0BE  srwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E47C8C: 21140040  subfic r8, r20, 0x40
	ctx.xer.ca = ctx.r[20].u32 <= 64 as u32;
	ctx.r[8].s64 = (64 as i64) - ctx.r[20].s64;
	// 82E47C90: 3B4B0001  addi r26, r11, 1
	ctx.r[26].s64 = ctx.r[11].s64 + 1;
	// 82E47C94: 39740050  addi r11, r20, 0x50
	ctx.r[11].s64 = ctx.r[20].s64 + 80;
	// 82E47C98: 20F4FFB0  subfic r7, r20, -0x50
	ctx.xer.ca = ctx.r[20].u32 <= -80 as u32;
	ctx.r[7].s64 = (-80 as i64) - ctx.r[20].s64;
	// 82E47C9C: 5746103A  slwi r6, r26, 2
	ctx.r[6].u32 = ctx.r[26].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82E47CA0: 3B80FFB0  li r28, -0x50
	ctx.r[28].s64 = -80;
	// 82E47CA4: 3BA0FFC0  li r29, -0x40
	ctx.r[29].s64 = -64;
	// 82E47CA8: 3B600020  li r27, 0x20
	ctx.r[27].s64 = 32;
	// 82E47CAC: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 82E47CB0: 38800050  li r4, 0x50
	ctx.r[4].s64 = 80;
	// 82E47CB4: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E47CB8: 7D275A14  add r9, r7, r11
	ctx.r[9].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82E47CBC: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E48690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E48690 size=2868
    let mut pc: u32 = 0x82E48690;
    'dispatch: loop {
        match pc {
            0x82E48690 => {
    //   block [0x82E48690..0x82E491C4)
	// 82E48690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E48694: 4BE60D3D  bl 0x82ca93d0
	ctx.lr = 0x82E48698;
	sub_82CA93D0(ctx, base);
	// 82E48698: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 82E4869C: 4BE65631  bl 0x82cadccc
	ctx.lr = 0x82E486A0;
	sub_82CADCA0(ctx, base);
	// 82E486A0: 9421FDB0  stwu r1, -0x250(r1)
	ea = ctx.r[1].u32.wrapping_add(-592 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E486A4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E486A8: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82E486AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E486B0: 9081026C  stw r4, 0x26c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(620 as u32), ctx.r[4].u32 ) };
	// 82E486B4: 7DEA5A14  add r15, r10, r11
	ctx.r[15].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E486B8: 90A10274  stw r5, 0x274(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(628 as u32), ctx.r[5].u32 ) };
	// 82E486BC: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 82E486C0: 7CF03B78  mr r16, r7
	ctx.r[16].u64 = ctx.r[7].u64;
	// 82E486C4: 91410088  stw r10, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82E486C8: 814F0000  lwz r10, 0(r15)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E486CC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E486D0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E486D4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E486D8: 4098002C  bge cr6, 0x82e48704
	if !ctx.cr[6].lt {
	pc = 0x82E48704; continue 'dispatch;
	}
	// 82E486DC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E486E0: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E486E4: 39295E14  addi r9, r9, 0x5e14
	ctx.r[9].s64 = ctx.r[9].s64 + 24084;
	// 82E486E8: 39085E0C  addi r8, r8, 0x5e0c
	ctx.r[8].s64 = ctx.r[8].s64 + 24076;
	// 82E486EC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E486F0: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82E486F4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82E486F8: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82E486FC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E48700: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82E48704: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E48708: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82E4870C: C01F005C  lfs f0, 0x5c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E48710: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82E48714: C1BF0058  lfs f13, 0x58(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E48718: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 82E4871C: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82E48720: C3C40008  lfs f30, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82E48724: 39DF0030  addi r14, r31, 0x30
	ctx.r[14].s64 = ctx.r[31].s64 + 48;
	// 82E48728: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E4872C: 3A600010  li r19, 0x10
	ctx.r[19].s64 = 16;
	// 82E48730: 92810060  stw r20, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[20].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E491C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E491C8 size=1916
    let mut pc: u32 = 0x82E491C8;
    'dispatch: loop {
        match pc {
            0x82E491C8 => {
    //   block [0x82E491C8..0x82E49944)
	// 82E491C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E491CC: 4BE60221  bl 0x82ca93ec
	ctx.lr = 0x82E491D0;
	sub_82CA93D0(ctx, base);
	// 82E491D0: DBE1FF98  stfd f31, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[31].u64 ) };
	// 82E491D4: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E491D8: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E491DC: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82E491E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E491E4: 7EBE5A14  add r21, r30, r11
	ctx.r[21].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82E491E8: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82E491EC: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 82E491F0: 81750000  lwz r11, 0(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E491F4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E491F8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E491FC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E49200: 40980020  bge cr6, 0x82e49220
	if !ctx.cr[6].lt {
	pc = 0x82E49220; continue 'dispatch;
	}
	// 82E49204: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E49208: 39295E28  addi r9, r9, 0x5e28
	ctx.r[9].s64 = ctx.r[9].s64 + 24104;
	// 82E4920C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E49210: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82E49214: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82E49218: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E4921C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82E49220: 897F00B0  lbz r11, 0xb0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 82E49224: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E49228: 419A0010  beq cr6, 0x82e49238
	if ctx.cr[6].eq {
	pc = 0x82E49238; continue 'dispatch;
	}
	// 82E4922C: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82E49230: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E49234: 4BFFE80D  bl 0x82e47a40
	ctx.lr = 0x82E49238;
	sub_82E47A40(ctx, base);
	// 82E49238: 815F0064  lwz r10, 0x64(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E4923C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E49240: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E49244: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82E49248: 7F1E4A14  add r24, r30, r9
	ctx.r[24].u64 = ctx.r[30].u64 + ctx.r[9].u64;
	// 82E4924C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E49250: 3F408000  lis r26, -0x8000
	ctx.r[26].s64 = -2147483648;
	// 82E49254: 3BCB000A  addi r30, r11, 0xa
	ctx.r[30].s64 = ctx.r[11].s64 + 10;
	// 82E49258: 93210050  stw r25, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[25].u32 ) };
	// 82E4925C: 57CA3032  slwi r10, r30, 6
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E49260: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E49264: 93210054  stw r25, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[25].u32 ) };
	// 82E49268: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82E4926C: 93410058  stw r26, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[26].u32 ) };
	// 82E49270: 55440036  rlwinm r4, r10, 0, 0, 0x1b
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E49274: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E49278: 810B002C  lwz r8, 0x2c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E4927C: 7D2A2214  add r9, r10, r4
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82E49280: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E49284: 4199000C  bgt cr6, 0x82e49290
	if ctx.cr[6].gt {
	pc = 0x82E49290; continue 'dispatch;
	}
	// 82E49288: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82E4928C: 4800001C  b 0x82e492a8
	pc = 0x82E492A8; continue 'dispatch;
	// 82E49290: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E49294: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E49298: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E4929C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E492A0: 4E800421  bctrl
	ctx.lr = 0x82E492A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E492A4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82E492A8: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E492AC: 7FC9D378  or r9, r30, r26
	ctx.r[9].u64 = ctx.r[30].u64 | ctx.r[26].u64;
	// 82E492B0: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E492B4: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 82E492B8: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82E492BC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E492C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E492C4: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 82E492C8: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E492CC: 40990070  ble cr6, 0x82e4933c
	if !ctx.cr[6].gt {
	pc = 0x82E4933C; continue 'dispatch;
	}
	// 82E492D0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E492D4: 3B7F0040  addi r27, r31, 0x40
	ctx.r[27].s64 = ctx.r[31].s64 + 64;
	// 82E492D8: 7F3CCB78  mr r28, r25
	ctx.r[28].u64 = ctx.r[25].u64;
	// 82E492DC: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 82E492E0: C3EB0C18  lfs f31, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E492E4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E492E8: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E492EC: 817F0074  lwz r11, 0x74(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E492F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E492F4: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E492F8: 7C9C5A14  add r4, r28, r11
	ctx.r[4].u64 = ctx.r[28].u64 + ctx.r[11].u64;
	// 82E492FC: 7CBD4A14  add r5, r29, r9
	ctx.r[5].u64 = ctx.r[29].u64 + ctx.r[9].u64;
	// 82E49300: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E49304: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E49308: 4E800421  bctrl
	ctx.lr = 0x82E4930C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4930C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82E49310: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E49314: C03F00A4  lfs f1, 0xa4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E49318: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E4931C: 4BFFD58D  bl 0x82e468a8
	ctx.lr = 0x82E49320;
	sub_82E468A8(ctx, base);
	// 82E49320: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E49324: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E49328: 3BBD0040  addi r29, r29, 0x40
	ctx.r[29].s64 = ctx.r[29].s64 + 64;
	// 82E4932C: 3B9C0030  addi r28, r28, 0x30
	ctx.r[28].s64 = ctx.r[28].s64 + 48;
	// 82E49330: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E49334: 4198FFB0  blt cr6, 0x82e492e4
	if ctx.cr[6].lt {
	pc = 0x82E492E4; continue 'dispatch;
	}
	// 82E49338: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E4933C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E49340: 813F0064  lwz r9, 0x64(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E49344: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E49348: 7D0A5850  subf r8, r10, r11
	ctx.r[8].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E4934C: 7F084800  cmpw cr6, r8, r9
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E49350: 40980034  bge cr6, 0x82e49384
	if !ctx.cr[6].lt {
	pc = 0x82E49384; continue 'dispatch;
	}
	// 82E49354: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82E49358: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E4935C: 40980028  bge cr6, 0x82e49384
	if !ctx.cr[6].lt {
	pc = 0x82E49384; continue 'dispatch;
	}
	// 82E49360: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E49364: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E49368: 41980008  blt cr6, 0x82e49370
	if ctx.cr[6].lt {
	pc = 0x82E49370; continue 'dispatch;
	}
	// 82E4936C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82E49370: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82E49374: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E49378: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4937C: 4BF0DB95  bl 0x82d56f10
	ctx.lr = 0x82E49380;
	sub_82D56F10(ctx, base);
	// 82E49380: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E49384: 813F0064  lwz r9, 0x64(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E49388: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4938C: 7FAA4A14  add r29, r10, r9
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82E49390: 93210070  stw r25, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[25].u32 ) };
	// 82E49394: 93210074  stw r25, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[25].u32 ) };
	// 82E49398: 395D0001  addi r10, r29, 1
	ctx.r[10].s64 = ctx.r[29].s64 + 1;
	// 82E4939C: 93410078  stw r26, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[26].u32 ) };
	// 82E493A0: 806B0020  lwz r3, 0x20(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E493A4: 55442036  slwi r4, r10, 4
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E493A8: 812B002C  lwz r9, 0x2c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E493AC: 7D432214  add r10, r3, r4
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82E493B0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E493B4: 4199000C  bgt cr6, 0x82e493c0
	if ctx.cr[6].gt {
	pc = 0x82E493C0; continue 'dispatch;
	}
	// 82E493B8: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82E493BC: 48000018  b 0x82e493d4
	pc = 0x82E493D4; continue 'dispatch;
	// 82E493C0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E493C4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E493C8: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E493CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E493D0: 4E800421  bctrl
	ctx.lr = 0x82E493D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E493D4: 813F0064  lwz r9, 0x64(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E493D8: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E493DC: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E493E0: 7FCA4A14  add r30, r10, r9
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82E493E4: 90610070  stw r3, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 82E493E8: 7FAAD378  or r10, r29, r26
	ctx.r[10].u64 = ctx.r[29].u64 | ctx.r[26].u64;
	// 82E493EC: 9061007C  stw r3, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[3].u32 ) };
	// 82E493F0: 393E0001  addi r9, r30, 1
	ctx.r[9].s64 = ctx.r[30].s64 + 1;
	// 82E493F4: 93210080  stw r25, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[25].u32 ) };
	// 82E493F8: 93210084  stw r25, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[25].u32 ) };
	// 82E493FC: 93410088  stw r26, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[26].u32 ) };
	// 82E49400: 55242036  slwi r4, r9, 4
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E49404: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82E49408: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E4940C: 810B002C  lwz r8, 0x2c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E49410: 7D2A2214  add r9, r10, r4
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82E49414: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E49418: 4199000C  bgt cr6, 0x82e49424
	if ctx.cr[6].gt {
	pc = 0x82E49424; continue 'dispatch;
	}
	// 82E4941C: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82E49420: 4800001C  b 0x82e4943c
	pc = 0x82E4943C; continue 'dispatch;
	// 82E49424: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E49428: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E4942C: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E49430: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E49434: 4E800421  bctrl
	ctx.lr = 0x82E49438;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E49438: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82E4943C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E49948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E49948 size=1424
    let mut pc: u32 = 0x82E49948;
    'dispatch: loop {
        match pc {
            0x82E49948 => {
    //   block [0x82E49948..0x82E49ED8)
	// 82E49948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4994C: 4BE5FAA1  bl 0x82ca93ec
	ctx.lr = 0x82E49950;
	sub_82CA93D0(ctx, base);
	// 82E49950: DBE1FF98  stfd f31, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[31].u64 ) };
	// 82E49954: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E49958: 810D0000  lwz r8, 0(r13)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4995C: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82E49960: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E49964: 7EA85A14  add r21, r8, r11
	ctx.r[21].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82E49968: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82E4996C: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 82E49970: 81750000  lwz r11, 0(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E49974: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E49978: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4997C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E49980: 40980020  bge cr6, 0x82e499a0
	if !ctx.cr[6].lt {
	pc = 0x82E499A0; continue 'dispatch;
	}
	// 82E49984: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E49988: 39295E28  addi r9, r9, 0x5e28
	ctx.r[9].s64 = ctx.r[9].s64 + 24104;
	// 82E4998C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E49990: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82E49994: 38EA000C  addi r7, r10, 0xc
	ctx.r[7].s64 = ctx.r[10].s64 + 12;
	// 82E49998: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E4999C: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82E499A0: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E499A4: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E499A8: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E499AC: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82E499B0: 7F084A14  add r24, r8, r9
	ctx.r[24].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82E499B4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E499B8: 3F408000  lis r26, -0x8000
	ctx.r[26].s64 = -2147483648;
	// 82E499BC: 3BCB000A  addi r30, r11, 0xa
	ctx.r[30].s64 = ctx.r[11].s64 + 10;
	// 82E499C0: 93210060  stw r25, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[25].u32 ) };
	// 82E499C4: 57CA3032  slwi r10, r30, 6
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E499C8: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E499CC: 93210064  stw r25, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[25].u32 ) };
	// 82E499D0: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82E499D4: 93410068  stw r26, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[26].u32 ) };
	// 82E499D8: 55440036  rlwinm r4, r10, 0, 0, 0x1b
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E499DC: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E499E0: 810B002C  lwz r8, 0x2c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E499E4: 7D2A2214  add r9, r10, r4
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82E499E8: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E499EC: 4199000C  bgt cr6, 0x82e499f8
	if ctx.cr[6].gt {
	pc = 0x82E499F8; continue 'dispatch;
	}
	// 82E499F0: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82E499F4: 4800001C  b 0x82e49a10
	pc = 0x82E49A10; continue 'dispatch;
	// 82E499F8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E499FC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E49A00: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E49A04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E49A08: 4E800421  bctrl
	ctx.lr = 0x82E49A0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E49A0C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82E49A10: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E49A14: 7FC9D378  or r9, r30, r26
	ctx.r[9].u64 = ctx.r[30].u64 | ctx.r[26].u64;
	// 82E49A18: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82E49A1C: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 82E49A20: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 82E49A24: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E49A28: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E49A2C: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 82E49A30: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82E49A34: 40990070  ble cr6, 0x82e49aa4
	if !ctx.cr[6].gt {
	pc = 0x82E49AA4; continue 'dispatch;
	}
	// 82E49A38: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E49A3C: 3B7F0040  addi r27, r31, 0x40
	ctx.r[27].s64 = ctx.r[31].s64 + 64;
	// 82E49A40: 7F3CCB78  mr r28, r25
	ctx.r[28].u64 = ctx.r[25].u64;
	// 82E49A44: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 82E49A48: C3EB0C18  lfs f31, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E49A4C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E49A50: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E49A54: 817F0074  lwz r11, 0x74(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E49A58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E49A5C: 81210060  lwz r9, 0x60(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E49A60: 7C9C5A14  add r4, r28, r11
	ctx.r[4].u64 = ctx.r[28].u64 + ctx.r[11].u64;
	// 82E49A64: 7CBD4A14  add r5, r29, r9
	ctx.r[5].u64 = ctx.r[29].u64 + ctx.r[9].u64;
	// 82E49A68: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E49A6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E49A70: 4E800421  bctrl
	ctx.lr = 0x82E49A74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E49A74: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82E49A78: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E49A7C: C03F00A4  lfs f1, 0xa4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E49A80: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E49A84: 4BFFCE25  bl 0x82e468a8
	ctx.lr = 0x82E49A88;
	sub_82E468A8(ctx, base);
	// 82E49A88: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E49A8C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E49A90: 3BBD0040  addi r29, r29, 0x40
	ctx.r[29].s64 = ctx.r[29].s64 + 64;
	// 82E49A94: 3B9C0030  addi r28, r28, 0x30
	ctx.r[28].s64 = ctx.r[28].s64 + 48;
	// 82E49A98: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E49A9C: 4198FFB0  blt cr6, 0x82e49a4c
	if ctx.cr[6].lt {
	pc = 0x82E49A4C; continue 'dispatch;
	}
	// 82E49AA0: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E49AA4: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E49AA8: 813F0064  lwz r9, 0x64(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E49AAC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E49AB0: 7D0A5850  subf r8, r10, r11
	ctx.r[8].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E49AB4: 7F084800  cmpw cr6, r8, r9
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E49AB8: 40980034  bge cr6, 0x82e49aec
	if !ctx.cr[6].lt {
	pc = 0x82E49AEC; continue 'dispatch;
	}
	// 82E49ABC: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82E49AC0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E49AC4: 40980028  bge cr6, 0x82e49aec
	if !ctx.cr[6].lt {
	pc = 0x82E49AEC; continue 'dispatch;
	}
	// 82E49AC8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E49ACC: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E49AD0: 41980008  blt cr6, 0x82e49ad8
	if ctx.cr[6].lt {
	pc = 0x82E49AD8; continue 'dispatch;
	}
	// 82E49AD4: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82E49AD8: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82E49ADC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E49AE0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E49AE4: 4BF0D42D  bl 0x82d56f10
	ctx.lr = 0x82E49AE8;
	sub_82D56F10(ctx, base);
	// 82E49AE8: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E49AEC: 813F0064  lwz r9, 0x64(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E49AF0: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E49AF4: 7FC95214  add r30, r9, r10
	ctx.r[30].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82E49AF8: 93210070  stw r25, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[25].u32 ) };
	// 82E49AFC: 93210074  stw r25, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[25].u32 ) };
	// 82E49B00: 395E0001  addi r10, r30, 1
	ctx.r[10].s64 = ctx.r[30].s64 + 1;
	// 82E49B04: 93410078  stw r26, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[26].u32 ) };
	// 82E49B08: 806B0020  lwz r3, 0x20(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E49B0C: 55442036  slwi r4, r10, 4
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E49B10: 812B002C  lwz r9, 0x2c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E49B14: 7D432214  add r10, r3, r4
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82E49B18: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E49B1C: 4199000C  bgt cr6, 0x82e49b28
	if ctx.cr[6].gt {
	pc = 0x82E49B28; continue 'dispatch;
	}
	// 82E49B20: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82E49B24: 48000018  b 0x82e49b3c
	pc = 0x82E49B3C; continue 'dispatch;
	// 82E49B28: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E49B2C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E49B30: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E49B34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E49B38: 4E800421  bctrl
	ctx.lr = 0x82E49B3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E49B3C: 7FCBD378  or r11, r30, r26
	ctx.r[11].u64 = ctx.r[30].u64 | ctx.r[26].u64;
	// 82E49B40: 815F0064  lwz r10, 0x64(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E49B44: 81210064  lwz r9, 0x64(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E49B48: 90610070  stw r3, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 82E49B4C: 7FCA4A14  add r30, r10, r9
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82E49B50: 9061007C  stw r3, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[3].u32 ) };
	// 82E49B54: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82E49B58: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82E49B5C: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82E49B60: 40980024  bge cr6, 0x82e49b84
	if !ctx.cr[6].lt {
	pc = 0x82E49B84; continue 'dispatch;
	}
	// 82E49B64: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E49B68: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E49B6C: 41980008  blt cr6, 0x82e49b74
	if ctx.cr[6].lt {
	pc = 0x82E49B74; continue 'dispatch;
	}
	// 82E49B70: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82E49B74: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82E49B78: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E49B7C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E49B80: 4BF0D391  bl 0x82d56f10
	ctx.lr = 0x82E49B84;
	sub_82D56F10(ctx, base);
	// 82E49B84: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E49ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E49ED8 size=132
    let mut pc: u32 = 0x82E49ED8;
    'dispatch: loop {
        match pc {
            0x82E49ED8 => {
    //   block [0x82E49ED8..0x82E49F5C)
	// 82E49ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E49EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E49EE0: 9421FC60  stwu r1, -0x3a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-928 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E49EE4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E49EE8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E49EEC: 394B0E2C  addi r10, r11, 0xe2c
	ctx.r[10].s64 = ctx.r[11].s64 + 3628;
	// 82E49EF0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E49EF4: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82E49EF8: 38C101F0  addi r6, r1, 0x1f0
	ctx.r[6].s64 = ctx.r[1].s64 + 496;
	// 82E49EFC: 91210204  stw r9, 0x204(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(516 as u32), ctx.r[9].u32 ) };
	// 82E49F00: 914101F0  stw r10, 0x1f0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(496 as u32), ctx.r[10].u32 ) };
	// 82E49F04: C00B0C64  lfs f0, 0xc64(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E49F08: 39610210  addi r11, r1, 0x210
	ctx.r[11].s64 = ctx.r[1].s64 + 528;
	// 82E49F0C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E49F10: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82E49F14: D00101F4  stfs f0, 0x1f4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(500 as u32), tmp.u32 ) };
	// 82E49F18: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82E49F1C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82E49F20: 91610200  stw r11, 0x200(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(512 as u32), ctx.r[11].u32 ) };
	// 82E49F24: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E49F28: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82E49F2C: 616B0008  ori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 8;
	// 82E49F30: 91610208  stw r11, 0x208(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(520 as u32), ctx.r[11].u32 ) };
	// 82E49F34: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82E49F38: 4BFFE759  bl 0x82e48690
	ctx.lr = 0x82E49F3C;
	sub_82E48690(ctx, base);
	// 82E49F3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E49F40: 4B4159C9  bl 0x8225f908
	ctx.lr = 0x82E49F44;
	sub_8225F908(ctx, base);
	// 82E49F44: 386101F0  addi r3, r1, 0x1f0
	ctx.r[3].s64 = ctx.r[1].s64 + 496;
	// 82E49F48: 4B4159C1  bl 0x8225f908
	ctx.lr = 0x82E49F4C;
	sub_8225F908(ctx, base);
	// 82E49F4C: 382103A0  addi r1, r1, 0x3a0
	ctx.r[1].s64 = ctx.r[1].s64 + 928;
	// 82E49F50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E49F54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E49F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E49F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E49F60 size=96
    let mut pc: u32 = 0x82E49F60;
    'dispatch: loop {
        match pc {
            0x82E49F60 => {
    //   block [0x82E49F60..0x82E49FC0)
	// 82E49F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E49F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E49F68: 9421FE00  stwu r1, -0x200(r1)
	ea = ctx.r[1].u32.wrapping_add(-512 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E49F6C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E49F70: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E49F74: 396B0E2C  addi r11, r11, 0xe2c
	ctx.r[11].s64 = ctx.r[11].s64 + 3628;
	// 82E49F78: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82E49F7C: C00A0C64  lfs f0, 0xc64(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E49F80: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E49F84: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82E49F88: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82E49F8C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E49F90: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82E49F94: 616B0008  ori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 8;
	// 82E49F98: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82E49F9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E49FA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82E49FA4: 4BFFF225  bl 0x82e491c8
	ctx.lr = 0x82E49FA8;
	sub_82E491C8(ctx, base);
	// 82E49FA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E49FAC: 4B41595D  bl 0x8225f908
	ctx.lr = 0x82E49FB0;
	sub_8225F908(ctx, base);
	// 82E49FB0: 38210200  addi r1, r1, 0x200
	ctx.r[1].s64 = ctx.r[1].s64 + 512;
	// 82E49FB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E49FB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E49FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E49FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E49FC0 size=64
    let mut pc: u32 = 0x82E49FC0;
    'dispatch: loop {
        match pc {
            0x82E49FC0 => {
    //   block [0x82E49FC0..0x82E4A000)
	// 82E49FC0: 81240078  lwz r9, 0x78(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E49FC4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E49FC8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E49FCC: 40990028  ble cr6, 0x82e49ff4
	if !ctx.cr[6].gt {
	pc = 0x82E49FF4; continue 'dispatch;
	}
	// 82E49FD0: 81040074  lwz r8, 0x74(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E49FD4: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82E49FD8: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E49FDC: 7F072840  cmplw cr6, r7, r5
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82E49FE0: 419A0020  beq cr6, 0x82e4a000
	if ctx.cr[6].eq {
		sub_82E4A000(ctx, base);
		return;
	}
	// 82E49FE4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E49FE8: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82E49FEC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E49FF0: 4198FFE8  blt cr6, 0x82e49fd8
	if ctx.cr[6].lt {
	pc = 0x82E49FD8; continue 'dispatch;
	}
	// 82E49FF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E49FF8: F9630000  std r11, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82E49FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A000 size=20
    let mut pc: u32 = 0x82E4A000;
    'dispatch: loop {
        match pc {
            0x82E4A000 => {
    //   block [0x82E4A000..0x82E4A014)
	// 82E4A000: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E4A004: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82E4A008: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82E4A00C: F9630000  std r11, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82E4A010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A018 size=8
    let mut pc: u32 = 0x82E4A018;
    'dispatch: loop {
        match pc {
            0x82E4A018 => {
    //   block [0x82E4A018..0x82E4A020)
	// 82E4A018: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82E4A01C: 4800000C  b 0x82e4a028
	sub_82E4A028(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A020 size=8
    let mut pc: u32 = 0x82E4A020;
    'dispatch: loop {
        match pc {
            0x82E4A020 => {
    //   block [0x82E4A020..0x82E4A028)
	// 82E4A020: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 82E4A024: 48000004  b 0x82e4a028
	sub_82E4A028(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A028 size=100
    let mut pc: u32 = 0x82E4A028;
    'dispatch: loop {
        match pc {
            0x82E4A028 => {
    //   block [0x82E4A028..0x82E4A08C)
	// 82E4A028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A02C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A030: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4A034: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4A038: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A03C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4A040: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4A044: 4BFFD3F5  bl 0x82e47438
	ctx.lr = 0x82E4A048;
	sub_82E47438(ctx, base);
	// 82E4A048: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4A04C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4A050: 419A0020  beq cr6, 0x82e4a070
	if ctx.cr[6].eq {
	pc = 0x82E4A070; continue 'dispatch;
	}
	// 82E4A054: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A058: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4A05C: 38C00033  li r6, 0x33
	ctx.r[6].s64 = 51;
	// 82E4A060: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4A064: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4A068: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4A06C: 4BF0B25D  bl 0x82d552c8
	ctx.lr = 0x82E4A070;
	sub_82D552C8(ctx, base);
	// 82E4A070: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4A074: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4A078: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4A07C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4A080: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4A084: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4A088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A090 size=20
    let mut pc: u32 = 0x82E4A090;
    'dispatch: loop {
        match pc {
            0x82E4A090 => {
    //   block [0x82E4A090..0x82E4A0A4)
	// 82E4A090: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A094: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A098: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A09C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4A0A0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A0A8 size=12
    let mut pc: u32 = 0x82E4A0A8;
    'dispatch: loop {
        match pc {
            0x82E4A0A8 => {
    //   block [0x82E4A0A8..0x82E4A0B4)
	// 82E4A0A8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4A0AC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4A0B0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A0B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A0B4 size=8
    let mut pc: u32 = 0x82E4A0B4;
    'dispatch: loop {
        match pc {
            0x82E4A0B4 => {
    //   block [0x82E4A0B4..0x82E4A0BC)
	// 82E4A0B4: 4800003C  b 0x82e4a0f0
	sub_82E4A0F0(ctx, base);
	return;
	// 82E4A0B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A0C0 size=44
    let mut pc: u32 = 0x82E4A0C0;
    'dispatch: loop {
        match pc {
            0x82E4A0C0 => {
    //   block [0x82E4A0C0..0x82E4A0EC)
	// 82E4A0C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A0C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A0C8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A0CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A0D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4A0D4: 4800001D  bl 0x82e4a0f0
	ctx.lr = 0x82E4A0D8;
	sub_82E4A0F0(ctx, base);
	// 82E4A0D8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4A0DC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4A0E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4A0E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4A0E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A0F0 size=168
    let mut pc: u32 = 0x82E4A0F0;
    'dispatch: loop {
        match pc {
            0x82E4A0F0 => {
    //   block [0x82E4A0F0..0x82E4A198)
	// 82E4A0F0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A0F4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E4A0F8: 396B85E8  addi r11, r11, -0x7a18
	ctx.r[11].s64 = ctx.r[11].s64 + -31256;
	// 82E4A0FC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E4A100: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E4A104: 394A8610  addi r10, r10, -0x79f0
	ctx.r[10].s64 = ctx.r[10].s64 + -31216;
	// 82E4A108: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82E4A10C: 39298624  addi r9, r9, -0x79dc
	ctx.r[9].s64 = ctx.r[9].s64 + -31196;
	// 82E4A110: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E4A114: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A118: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E4A11C: 38CB5EB8  addi r6, r11, 0x5eb8
	ctx.r[6].s64 = ctx.r[11].s64 + 24248;
	// 82E4A120: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A124: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E4A128: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82E4A12C: 39088604  addi r8, r8, -0x79fc
	ctx.r[8].s64 = ctx.r[8].s64 + -31228;
	// 82E4A130: 38AB5EAC  addi r5, r11, 0x5eac
	ctx.r[5].s64 = ctx.r[11].s64 + 24236;
	// 82E4A134: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A138: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E4A13C: 394B5E98  addi r10, r11, 0x5e98
	ctx.r[10].s64 = ctx.r[11].s64 + 24216;
	// 82E4A140: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A144: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82E4A148: 38E75D40  addi r7, r7, 0x5d40
	ctx.r[7].s64 = ctx.r[7].s64 + 23872;
	// 82E4A14C: 392B5E8C  addi r9, r11, 0x5e8c
	ctx.r[9].s64 = ctx.r[11].s64 + 24204;
	// 82E4A150: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A154: 388B5E80  addi r4, r11, 0x5e80
	ctx.r[4].s64 = ctx.r[11].s64 + 24192;
	// 82E4A158: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A15C: 390B5E68  addi r8, r11, 0x5e68
	ctx.r[8].s64 = ctx.r[11].s64 + 24168;
	// 82E4A160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4A164: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82E4A168: 90E30030  stw r7, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[7].u32 ) };
	// 82E4A16C: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E4A170: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82E4A174: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82E4A178: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82E4A17C: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82E4A180: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82E4A184: 91030030  stw r8, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[8].u32 ) };
	// 82E4A188: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82E4A18C: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82E4A190: 9143003C  stw r10, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 82E4A194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A198 size=100
    let mut pc: u32 = 0x82E4A198;
    'dispatch: loop {
        match pc {
            0x82E4A198 => {
    //   block [0x82E4A198..0x82E4A1FC)
	// 82E4A198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A1A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4A1A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4A1A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A1AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4A1B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4A1B4: 48008DE5  bl 0x82e52f98
	ctx.lr = 0x82E4A1B8;
	sub_82E52F98(ctx, base);
	// 82E4A1B8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4A1BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4A1C0: 419A0020  beq cr6, 0x82e4a1e0
	if ctx.cr[6].eq {
	pc = 0x82E4A1E0; continue 'dispatch;
	}
	// 82E4A1C4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A1C8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4A1CC: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82E4A1D0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4A1D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4A1D8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4A1DC: 4BF0B0ED  bl 0x82d552c8
	ctx.lr = 0x82E4A1E0;
	sub_82D552C8(ctx, base);
	// 82E4A1E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4A1E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4A1E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4A1EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4A1F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4A1F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4A1F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A200 size=8
    let mut pc: u32 = 0x82E4A200;
    'dispatch: loop {
        match pc {
            0x82E4A200 => {
    //   block [0x82E4A200..0x82E4A208)
	// 82E4A200: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82E4A204: 4BFFFF94  b 0x82e4a198
	sub_82E4A198(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A208 size=8
    let mut pc: u32 = 0x82E4A208;
    'dispatch: loop {
        match pc {
            0x82E4A208 => {
    //   block [0x82E4A208..0x82E4A210)
	// 82E4A208: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 82E4A20C: 4BFFFF8C  b 0x82e4a198
	sub_82E4A198(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A210 size=8
    let mut pc: u32 = 0x82E4A210;
    'dispatch: loop {
        match pc {
            0x82E4A210 => {
    //   block [0x82E4A210..0x82E4A218)
	// 82E4A210: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82E4A214: 4BFFFF84  b 0x82e4a198
	sub_82E4A198(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A218 size=8
    let mut pc: u32 = 0x82E4A218;
    'dispatch: loop {
        match pc {
            0x82E4A218 => {
    //   block [0x82E4A218..0x82E4A220)
	// 82E4A218: 3863FFD0  addi r3, r3, -0x30
	ctx.r[3].s64 = ctx.r[3].s64 + -48;
	// 82E4A21C: 4BFFFF7C  b 0x82e4a198
	sub_82E4A198(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A220 size=8
    let mut pc: u32 = 0x82E4A220;
    'dispatch: loop {
        match pc {
            0x82E4A220 => {
    //   block [0x82E4A220..0x82E4A228)
	// 82E4A220: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82E4A224: 4BFFFF74  b 0x82e4a198
	sub_82E4A198(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A228 size=4
    let mut pc: u32 = 0x82E4A228;
    'dispatch: loop {
        match pc {
            0x82E4A228 => {
    //   block [0x82E4A228..0x82E4A22C)
	// 82E4A228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A230 size=4
    let mut pc: u32 = 0x82E4A230;
    'dispatch: loop {
        match pc {
            0x82E4A230 => {
    //   block [0x82E4A230..0x82E4A234)
	// 82E4A230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A238 size=20
    let mut pc: u32 = 0x82E4A238;
    'dispatch: loop {
        match pc {
            0x82E4A238 => {
    //   block [0x82E4A238..0x82E4A24C)
	// 82E4A238: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A23C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A240: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A244: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4A248: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A250 size=8
    let mut pc: u32 = 0x82E4A250;
    'dispatch: loop {
        match pc {
            0x82E4A250 => {
    //   block [0x82E4A250..0x82E4A258)
	// 82E4A250: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4A254: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A258 size=24
    let mut pc: u32 = 0x82E4A258;
    'dispatch: loop {
        match pc {
            0x82E4A258 => {
    //   block [0x82E4A258..0x82E4A270)
	// 82E4A258: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A25C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4A260: 396B6004  addi r11, r11, 0x6004
	ctx.r[11].s64 = ctx.r[11].s64 + 24580;
	// 82E4A264: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82E4A268: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4A26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A270 size=12
    let mut pc: u32 = 0x82E4A270;
    'dispatch: loop {
        match pc {
            0x82E4A270 => {
    //   block [0x82E4A270..0x82E4A27C)
	// 82E4A270: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A274: 386B6004  addi r3, r11, 0x6004
	ctx.r[3].s64 = ctx.r[11].s64 + 24580;
	// 82E4A278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A280 size=100
    let mut pc: u32 = 0x82E4A280;
    'dispatch: loop {
        match pc {
            0x82E4A280 => {
    //   block [0x82E4A280..0x82E4A2E4)
	// 82E4A280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A288: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4A28C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4A290: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A294: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4A298: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4A29C: 48009E25  bl 0x82e540c0
	ctx.lr = 0x82E4A2A0;
	sub_82E540C0(ctx, base);
	// 82E4A2A0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4A2A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4A2A8: 419A0020  beq cr6, 0x82e4a2c8
	if ctx.cr[6].eq {
	pc = 0x82E4A2C8; continue 'dispatch;
	}
	// 82E4A2AC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A2B0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4A2B4: 38C0002B  li r6, 0x2b
	ctx.r[6].s64 = 43;
	// 82E4A2B8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4A2BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4A2C0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4A2C4: 4BF0B005  bl 0x82d552c8
	ctx.lr = 0x82E4A2C8;
	sub_82D552C8(ctx, base);
	// 82E4A2C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4A2CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4A2D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4A2D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4A2D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4A2DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4A2E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A2E8 size=20
    let mut pc: u32 = 0x82E4A2E8;
    'dispatch: loop {
        match pc {
            0x82E4A2E8 => {
    //   block [0x82E4A2E8..0x82E4A2FC)
	// 82E4A2E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A2EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A2F0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A2F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4A2F8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A300 size=8
    let mut pc: u32 = 0x82E4A300;
    'dispatch: loop {
        match pc {
            0x82E4A300 => {
    //   block [0x82E4A300..0x82E4A308)
	// 82E4A300: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4A304: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A308 size=128
    let mut pc: u32 = 0x82E4A308;
    'dispatch: loop {
        match pc {
            0x82E4A308 => {
    //   block [0x82E4A308..0x82E4A388)
	// 82E4A308: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A30C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E4A310: 396B85E8  addi r11, r11, -0x7a18
	ctx.r[11].s64 = ctx.r[11].s64 + -31256;
	// 82E4A314: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E4A318: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82E4A31C: 394A8610  addi r10, r10, -0x79f0
	ctx.r[10].s64 = ctx.r[10].s64 + -31216;
	// 82E4A320: B0C30006  sth r6, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82E4A324: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82E4A328: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E4A32C: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82E4A330: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82E4A334: 3CA08203  lis r5, -0x7dfd
	ctx.r[5].s64 = -2113732608;
	// 82E4A338: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A33C: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E4A340: 3C808203  lis r4, -0x7dfd
	ctx.r[4].s64 = -2113732608;
	// 82E4A344: 39298624  addi r9, r9, -0x79dc
	ctx.r[9].s64 = ctx.r[9].s64 + -31196;
	// 82E4A348: 39088604  addi r8, r8, -0x79fc
	ctx.r[8].s64 = ctx.r[8].s64 + -31228;
	// 82E4A34C: 38E760F8  addi r7, r7, 0x60f8
	ctx.r[7].s64 = ctx.r[7].s64 + 24824;
	// 82E4A350: 38C660EC  addi r6, r6, 0x60ec
	ctx.r[6].s64 = ctx.r[6].s64 + 24812;
	// 82E4A354: 38A560D8  addi r5, r5, 0x60d8
	ctx.r[5].s64 = ctx.r[5].s64 + 24792;
	// 82E4A358: 396B60CC  addi r11, r11, 0x60cc
	ctx.r[11].s64 = ctx.r[11].s64 + 24780;
	// 82E4A35C: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82E4A360: 388460C0  addi r4, r4, 0x60c0
	ctx.r[4].s64 = ctx.r[4].s64 + 24768;
	// 82E4A364: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82E4A368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E4A36C: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82E4A370: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82E4A374: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82E4A378: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82E4A37C: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82E4A380: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82E4A384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A388 size=12
    let mut pc: u32 = 0x82E4A388;
    'dispatch: loop {
        match pc {
            0x82E4A388 => {
    //   block [0x82E4A388..0x82E4A394)
	// 82E4A388: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A38C: 386B60F8  addi r3, r11, 0x60f8
	ctx.r[3].s64 = ctx.r[11].s64 + 24824;
	// 82E4A390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A398 size=8
    let mut pc: u32 = 0x82E4A398;
    'dispatch: loop {
        match pc {
            0x82E4A398 => {
    //   block [0x82E4A398..0x82E4A3A0)
	// 82E4A398: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82E4A39C: 4800001C  b 0x82e4a3b8
	sub_82E4A3B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A3A0 size=8
    let mut pc: u32 = 0x82E4A3A0;
    'dispatch: loop {
        match pc {
            0x82E4A3A0 => {
    //   block [0x82E4A3A0..0x82E4A3A8)
	// 82E4A3A0: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82E4A3A4: 48000014  b 0x82e4a3b8
	sub_82E4A3B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A3A8 size=8
    let mut pc: u32 = 0x82E4A3A8;
    'dispatch: loop {
        match pc {
            0x82E4A3A8 => {
    //   block [0x82E4A3A8..0x82E4A3B0)
	// 82E4A3A8: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 82E4A3AC: 4800000C  b 0x82e4a3b8
	sub_82E4A3B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A3B0 size=8
    let mut pc: u32 = 0x82E4A3B0;
    'dispatch: loop {
        match pc {
            0x82E4A3B0 => {
    //   block [0x82E4A3B0..0x82E4A3B8)
	// 82E4A3B0: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82E4A3B4: 48000004  b 0x82e4a3b8
	sub_82E4A3B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A3B8 size=100
    let mut pc: u32 = 0x82E4A3B8;
    'dispatch: loop {
        match pc {
            0x82E4A3B8 => {
    //   block [0x82E4A3B8..0x82E4A41C)
	// 82E4A3B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A3BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A3C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4A3C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4A3C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A3CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4A3D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4A3D4: 4B815105  bl 0x8265f4d8
	ctx.lr = 0x82E4A3D8;
	sub_8265F4D8(ctx, base);
	// 82E4A3D8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4A3DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4A3E0: 419A0020  beq cr6, 0x82e4a400
	if ctx.cr[6].eq {
	pc = 0x82E4A400; continue 'dispatch;
	}
	// 82E4A3E4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A3E8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4A3EC: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82E4A3F0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4A3F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4A3F8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4A3FC: 4BF0AECD  bl 0x82d552c8
	ctx.lr = 0x82E4A400;
	sub_82D552C8(ctx, base);
	// 82E4A400: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4A404: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4A408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4A40C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4A410: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4A414: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4A418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A420 size=8
    let mut pc: u32 = 0x82E4A420;
    'dispatch: loop {
        match pc {
            0x82E4A420 => {
    //   block [0x82E4A420..0x82E4A428)
	// 82E4A420: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4A424: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A428 size=24
    let mut pc: u32 = 0x82E4A428;
    'dispatch: loop {
        match pc {
            0x82E4A428 => {
    //   block [0x82E4A428..0x82E4A440)
	// 82E4A428: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A42C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4A430: 396B6194  addi r11, r11, 0x6194
	ctx.r[11].s64 = ctx.r[11].s64 + 24980;
	// 82E4A434: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82E4A438: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4A43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A440 size=20
    let mut pc: u32 = 0x82E4A440;
    'dispatch: loop {
        match pc {
            0x82E4A440 => {
    //   block [0x82E4A440..0x82E4A454)
	// 82E4A440: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A444: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A448: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A44C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4A450: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A458 size=12
    let mut pc: u32 = 0x82E4A458;
    'dispatch: loop {
        match pc {
            0x82E4A458 => {
    //   block [0x82E4A458..0x82E4A464)
	// 82E4A458: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A45C: 386B6194  addi r3, r11, 0x6194
	ctx.r[3].s64 = ctx.r[11].s64 + 24980;
	// 82E4A460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A468 size=4
    let mut pc: u32 = 0x82E4A468;
    'dispatch: loop {
        match pc {
            0x82E4A468 => {
    //   block [0x82E4A468..0x82E4A46C)
	// 82E4A468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A470 size=4
    let mut pc: u32 = 0x82E4A470;
    'dispatch: loop {
        match pc {
            0x82E4A470 => {
    //   block [0x82E4A470..0x82E4A474)
	// 82E4A470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A478 size=4
    let mut pc: u32 = 0x82E4A478;
    'dispatch: loop {
        match pc {
            0x82E4A478 => {
    //   block [0x82E4A478..0x82E4A47C)
	// 82E4A478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A480 size=8
    let mut pc: u32 = 0x82E4A480;
    'dispatch: loop {
        match pc {
            0x82E4A480 => {
    //   block [0x82E4A480..0x82E4A488)
	// 82E4A480: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A484: 4800008C  b 0x82e4a510
	sub_82E4A510(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A488 size=4
    let mut pc: u32 = 0x82E4A488;
    'dispatch: loop {
        match pc {
            0x82E4A488 => {
    //   block [0x82E4A488..0x82E4A48C)
	// 82E4A488: 48000008  b 0x82e4a490
	sub_82E4A490(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A490 size=128
    let mut pc: u32 = 0x82E4A490;
    'dispatch: loop {
        match pc {
            0x82E4A490 => {
    //   block [0x82E4A490..0x82E4A510)
	// 82E4A490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A498: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4A49C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A4A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4A4A4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E4A4A8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4A4AC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4A4B0: 409A0020  bne cr6, 0x82e4a4d0
	if !ctx.cr[6].eq {
	pc = 0x82E4A4D0; continue 'dispatch;
	}
	// 82E4A4B4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A4B8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4A4BC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4A4C0: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4A4C4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4A4C8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4A4CC: 4BF0ADFD  bl 0x82d552c8
	ctx.lr = 0x82E4A4D0;
	sub_82D552C8(ctx, base);
	// 82E4A4D0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4A4D4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4A4D8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4A4DC: 409A0020  bne cr6, 0x82e4a4fc
	if !ctx.cr[6].eq {
	pc = 0x82E4A4FC; continue 'dispatch;
	}
	// 82E4A4E0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A4E4: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4A4E8: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4A4EC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A4F0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4A4F4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4A4F8: 4BF0ADD1  bl 0x82d552c8
	ctx.lr = 0x82E4A4FC;
	sub_82D552C8(ctx, base);
	// 82E4A4FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E4A500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4A504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4A508: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4A50C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A510 size=184
    let mut pc: u32 = 0x82E4A510;
    'dispatch: loop {
        match pc {
            0x82E4A510 => {
    //   block [0x82E4A510..0x82E4A5C8)
	// 82E4A510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A518: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4A51C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4A520: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A524: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4A528: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4A52C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4A530: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4A534: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4A538: 409A0020  bne cr6, 0x82e4a558
	if !ctx.cr[6].eq {
	pc = 0x82E4A558; continue 'dispatch;
	}
	// 82E4A53C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A540: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4A544: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4A548: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A54C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4A550: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4A554: 4BF0AD75  bl 0x82d552c8
	ctx.lr = 0x82E4A558;
	sub_82D552C8(ctx, base);
	// 82E4A558: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4A55C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4A560: 419A004C  beq cr6, 0x82e4a5ac
	if ctx.cr[6].eq {
	pc = 0x82E4A5AC; continue 'dispatch;
	}
	// 82E4A564: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A568: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4A56C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4A570: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82E4A574: 812B0034  lwz r9, 0x34(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E4A578: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E4A57C: 41980018  blt cr6, 0x82e4a594
	if ctx.cr[6].lt {
	pc = 0x82E4A594; continue 'dispatch;
	}
	// 82E4A580: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E4A584: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82E4A588: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E4A58C: 4BF0AB9D  bl 0x82d55128
	ctx.lr = 0x82E4A590;
	sub_82D55128(ctx, base);
	// 82E4A590: 4800001C  b 0x82e4a5ac
	pc = 0x82E4A5AC; continue 'dispatch;
	// 82E4A594: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82E4A598: 812B0048  lwz r9, 0x48(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E4A59C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E4A5A0: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 82E4A5A4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E4A5A8: 93EB0048  stw r31, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	// 82E4A5AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4A5B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4A5B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4A5B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4A5BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4A5C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4A5C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A5C8 size=20
    let mut pc: u32 = 0x82E4A5C8;
    'dispatch: loop {
        match pc {
            0x82E4A5C8 => {
    //   block [0x82E4A5C8..0x82E4A5DC)
	// 82E4A5C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A5CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A5D0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A5D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4A5D8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A5E0 size=92
    let mut pc: u32 = 0x82E4A5E0;
    'dispatch: loop {
        match pc {
            0x82E4A5E0 => {
    //   block [0x82E4A5E0..0x82E4A63C)
	// 82E4A5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A5E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4A5EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A5F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4A5F4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E4A5F8: 419A0030  beq cr6, 0x82e4a628
	if ctx.cr[6].eq {
	pc = 0x82E4A628; continue 'dispatch;
	}
	// 82E4A5FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E4A600: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A604: 4BF40785  bl 0x82d8ad88
	ctx.lr = 0x82E4A608;
	sub_82D8AD88(ctx, base);
	// 82E4A608: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A60C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E4A610: 396B646C  addi r11, r11, 0x646c
	ctx.r[11].s64 = ctx.r[11].s64 + 25708;
	// 82E4A614: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82E4A618: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4A61C: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E4A620: 915F0058  stw r10, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82E4A624: 913F005C  stw r9, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82E4A628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E4A62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4A630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4A634: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4A638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A640 size=52
    let mut pc: u32 = 0x82E4A640;
    'dispatch: loop {
        match pc {
            0x82E4A640 => {
    //   block [0x82E4A640..0x82E4A674)
	// 82E4A640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A648: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A64C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E4A650: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A654: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4A658: 4BF40731  bl 0x82d8ad88
	ctx.lr = 0x82E4A65C;
	sub_82D8AD88(ctx, base);
	// 82E4A65C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4A660: 386B646C  addi r3, r11, 0x646c
	ctx.r[3].s64 = ctx.r[11].s64 + 25708;
	// 82E4A664: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82E4A668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4A66C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4A670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4A678 size=148
    let mut pc: u32 = 0x82E4A678;
    'dispatch: loop {
        match pc {
            0x82E4A678 => {
    //   block [0x82E4A678..0x82E4A70C)
	// 82E4A678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4A67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4A680: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4A684: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4A688: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4A68C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4A690: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4A694: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E4A698: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E4A69C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4A6A0: 409A0020  bne cr6, 0x82e4a6c0
	if !ctx.cr[6].eq {
	pc = 0x82E4A6C0; continue 'dispatch;
	}
	// 82E4A6A4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A6A8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82E4A6AC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82E4A6B0: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E4A6B4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E4A6B8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E4A6BC: 4BF0AC0D  bl 0x82d552c8
	ctx.lr = 0x82E4A6C0;
	sub_82D552C8(ctx, base);
	// 82E4A6C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4A6C4: 4BF4072D  bl 0x82d8adf0
	ctx.lr = 0x82E4A6C8;
	sub_82D8ADF0(ctx, base);
	// 82E4A6C8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E4A6CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4A6D0: 419A0020  beq cr6, 0x82e4a6f0
	if ctx.cr[6].eq {
	pc = 0x82E4A6F0; continue 'dispatch;
	}
	// 82E4A6D4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A6D8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E4A6DC: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 82E4A6E0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4A6E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4A6E8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E4A6EC: 4BF0ABDD  bl 0x82d552c8
	ctx.lr = 0x82E4A6F0;
	sub_82D552C8(ctx, base);
	// 82E4A6F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4A6F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4A6F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4A6FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4A700: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4A704: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4A708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A710 size=4
    let mut pc: u32 = 0x82E4A710;
    'dispatch: loop {
        match pc {
            0x82E4A710 => {
    //   block [0x82E4A710..0x82E4A714)
	// 82E4A710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A718 size=20
    let mut pc: u32 = 0x82E4A718;
    'dispatch: loop {
        match pc {
            0x82E4A718 => {
    //   block [0x82E4A718..0x82E4A72C)
	// 82E4A718: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A71C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4A720: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4A724: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4A728: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4A730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4A730 size=8
    let mut pc: u32 = 0x82E4A730;
    'dispatch: loop {
        match pc {
            0x82E4A730 => {
    //   block [0x82E4A730..0x82E4A738)
	// 82E4A730: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4A734: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


