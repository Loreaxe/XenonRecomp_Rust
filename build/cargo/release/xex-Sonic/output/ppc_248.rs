pub fn sub_83060B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83060B60 size=12
    let mut pc: u32 = 0x83060B60;
    'dispatch: loop {
        match pc {
            0x83060B60 => {
    //   block [0x83060B60..0x83060B6C)
	// 83060B60: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83060B64: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83060B68: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83060B6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83060B6C size=12
    let mut pc: u32 = 0x83060B6C;
    'dispatch: loop {
        match pc {
            0x83060B6C => {
    //   block [0x83060B6C..0x83060B78)
	// 83060B6C: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83060B70: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 83060B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83060B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83060B78 size=8
    let mut pc: u32 = 0x83060B78;
    'dispatch: loop {
        match pc {
            0x83060B78 => {
    //   block [0x83060B78..0x83060B80)
	// 83060B78: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83060B7C: 82165C70  lwz r16, 0x5c70(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(23664 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83060B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83060B80 size=164
    let mut pc: u32 = 0x83060B80;
    'dispatch: loop {
        match pc {
            0x83060B80 => {
    //   block [0x83060B80..0x83060C24)
	// 83060B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83060B84: 481475E9  bl 0x831a816c
	ctx.lr = 0x83060B88;
	sub_831A8130(ctx, base);
	// 83060B88: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83060B8C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83060B90: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83060B94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83060B98: 396B5C60  addi r11, r11, 0x5c60
	ctx.r[11].s64 = ctx.r[11].s64 + 23648;
	// 83060B9C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83060BA0: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83060BA4: 909E0024  stw r4, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[4].u32 ) };
	// 83060BA8: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 83060BAC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83060BB0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83060BB4: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83060BB8: 90DE0008  stw r6, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 83060BBC: 90BE000C  stw r5, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 83060BC0: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 83060BC4: 93BE0014  stw r29, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 83060BC8: 93BE0018  stw r29, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 83060BCC: 9BBE001C  stb r29, 0x1c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 83060BD0: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83060BD4: 93BE0028  stw r29, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 83060BD8: 93BE002C  stw r29, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 83060BDC: 93BE0030  stw r29, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 83060BE0: 93BE0038  stw r29, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 83060BE4: 93BE003C  stw r29, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[29].u32 ) };
	// 83060BE8: 90FE0004  stw r7, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 83060BEC: 93BE0034  stw r29, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 83060BF0: 4BF776A9  bl 0x82fd8298
	ctx.lr = 0x83060BF4;
	sub_82FD8298(ctx, base);
	// 83060BF4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83060BF8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83060BFC: 41820018  beq 0x83060c14
	if ctx.cr[0].eq {
	pc = 0x83060C14; continue 'dispatch;
	}
	// 83060C00: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 83060C04: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83060C08: 3880006D  li r4, 0x6d
	ctx.r[4].s64 = 109;
	// 83060C0C: 4BFEBA25  bl 0x8304c630
	ctx.lr = 0x83060C10;
	sub_8304C630(ctx, base);
	// 83060C10: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83060C14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83060C18: 93BE0034  stw r29, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 83060C1C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83060C20: 4814759C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83060C24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83060C24 size=48
    let mut pc: u32 = 0x83060C24;
    'dispatch: loop {
        match pc {
            0x83060C24 => {
    //   block [0x83060C24..0x83060C54)
	// 83060C24: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83060C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83060C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83060C30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83060C34: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83060C38: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83060C3C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83060C40: 4BF776A1  bl 0x82fd82e0
	ctx.lr = 0x83060C44;
	sub_82FD82E0(ctx, base);
	// 83060C44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83060C48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83060C4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83060C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83060C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83060C58 size=8
    let mut pc: u32 = 0x83060C58;
    'dispatch: loop {
        match pc {
            0x83060C58 => {
    //   block [0x83060C58..0x83060C60)
	// 83060C58: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83060C5C: 82165CA8  lwz r16, 0x5ca8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(23720 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83060C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83060C60 size=492
    let mut pc: u32 = 0x83060C60;
    'dispatch: loop {
        match pc {
            0x83060C60 => {
    //   block [0x83060C60..0x83060E4C)
	// 83060C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83060C64: 481474F5  bl 0x831a8158
	ctx.lr = 0x83060C68;
	sub_831A8130(ctx, base);
	// 83060C68: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 83060C6C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83060C70: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83060C74: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83060C78: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83060C7C: 388B7950  addi r4, r11, 0x7950
	ctx.r[4].s64 = ctx.r[11].s64 + 31056;
	// 83060C80: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83060C84: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83060C88: 4BF92849  bl 0x82ff34d0
	ctx.lr = 0x83060C8C;
	sub_82FF34D0(ctx, base);
	// 83060C8C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83060C90: 41820014  beq 0x83060ca4
	if ctx.cr[0].eq {
	pc = 0x83060CA4; continue 'dispatch;
	}
	// 83060C94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83060C98: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83060C9C: 917D0024  stw r11, 0x24(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83060CA0: 480001A4  b 0x83060e44
	pc = 0x83060E44; continue 'dispatch;
	// 83060CA4: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83060CA8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83060CAC: 388A78D8  addi r4, r10, 0x78d8
	ctx.r[4].s64 = ctx.r[10].s64 + 30936;
	// 83060CB0: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83060CB4: 4BF9281D  bl 0x82ff34d0
	ctx.lr = 0x83060CB8;
	sub_82FF34D0(ctx, base);
	// 83060CB8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83060CBC: 4182000C  beq 0x83060cc8
	if ctx.cr[0].eq {
	pc = 0x83060CC8; continue 'dispatch;
	}
	// 83060CC0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83060CC4: 4BFFFFD4  b 0x83060c98
	pc = 0x83060C98; continue 'dispatch;
	// 83060CC8: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 83060CCC: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83060CD0: 4BF8D5E1  bl 0x82fee2b0
	ctx.lr = 0x83060CD4;
	sub_82FEE2B0(ctx, base);
	// 83060CD4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83060CD8: 40820030  bne 0x83060d08
	if !ctx.cr[0].eq {
	pc = 0x83060D08; continue 'dispatch;
	}
	// 83060CDC: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83060CE0: 4BF7D689  bl 0x82fde368
	ctx.lr = 0x83060CE4;
	sub_82FDE368(ctx, base);
	// 83060CE4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83060CE8: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83060CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83060CF0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83060CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83060CF8: 388000E9  li r4, 0xe9
	ctx.r[4].s64 = 233;
	// 83060CFC: 4BF87CDD  bl 0x82fe89d8
	ctx.lr = 0x83060D00;
	sub_82FE89D8(ctx, base);
	// 83060D00: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83060D04: 48000140  b 0x83060e44
	pc = 0x83060E44; continue 'dispatch;
	// 83060D08: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83060D0C: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 83060D10: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83060D14: 61598028  ori r25, r10, 0x8028
	ctx.r[25].u64 = ctx.r[10].u64 | 32808;
	// 83060D18: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83060D1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83060D20: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83060D24: 7F0BC82E  lwzx r24, r11, r25
	ctx.r[24].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 83060D28: 48003521  bl 0x83064248
	ctx.lr = 0x83060D2C;
	sub_83064248(ctx, base);
	// 83060D2C: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83060D30: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83060D34: 388A7C54  addi r4, r10, 0x7c54
	ctx.r[4].s64 = ctx.r[10].s64 + 31828;
	// 83060D38: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83060D3C: 4BF92795  bl 0x82ff34d0
	ctx.lr = 0x83060D40;
	sub_82FF34D0(ctx, base);
	// 83060D40: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83060D44: 41820060  beq 0x83060da4
	if ctx.cr[0].eq {
	pc = 0x83060DA4; continue 'dispatch;
	}
	// 83060D48: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83060D4C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83060D50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83060D54: 917D0024  stw r11, 0x24(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83060D58: 4BFFEDE9  bl 0x8305fb40
	ctx.lr = 0x83060D5C;
	sub_8305FB40(ctx, base);
	// 83060D5C: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83060D60: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83060D64: 896B0010  lbz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83060D68: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83060D6C: 418200A4  beq 0x83060e10
	if ctx.cr[0].eq {
	pc = 0x83060E10; continue 'dispatch;
	}
	// 83060D70: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83060D74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83060D78: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 83060D7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83060D80: 4E800421  bctrl
	ctx.lr = 0x83060D84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83060D84: 48026075  bl 0x83086df8
	ctx.lr = 0x83060D88;
	sub_83086DF8(ctx, base);
	// 83060D88: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83060D8C: 41820084  beq 0x83060e10
	if ctx.cr[0].eq {
	pc = 0x83060E10; continue 'dispatch;
	}
	// 83060D90: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83060D94: 3880001B  li r4, 0x1b
	ctx.r[4].s64 = 27;
	// 83060D98: 806B00A8  lwz r3, 0xa8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 83060D9C: 4BFB664D  bl 0x830173e8
	ctx.lr = 0x83060DA0;
	sub_830173E8(ctx, base);
	// 83060DA0: 48000070  b 0x83060e10
	pc = 0x83060E10; continue 'dispatch;
	// 83060DA4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83060DA8: 917D0024  stw r11, 0x24(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83060DAC: 839E0028  lwz r28, 0x28(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83060DB0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83060DB4: 4BF7E245  bl 0x82fdeff8
	ctx.lr = 0x83060DB8;
	sub_82FDEFF8(ctx, base);
	// 83060DB8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83060DBC: 939F0054  stw r28, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 83060DC0: 937F0050  stw r27, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 83060DC4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83060DC8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83060DCC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83060DD0: 4BFFDB91  bl 0x8305e960
	ctx.lr = 0x83060DD4;
	sub_8305E960(ctx, base);
	// 83060DD4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83060DD8: 7C8B0034  cntlzw r11, r4
	ctx.r[11].u64 = if ctx.r[4].u32 == 0 { 32 } else { ctx.r[4].u32.leading_zeros() as u64 };
	// 83060DDC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83060DE0: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83060DE4: 697A0001  xori r26, r11, 1
	ctx.r[26].u64 = ctx.r[11].u64 ^ 1;
	// 83060DE8: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83060DEC: 41820018  beq 0x83060e04
	if ctx.cr[0].eq {
	pc = 0x83060E04; continue 'dispatch;
	}
	// 83060DF0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83060DF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83060DF8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83060DFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83060E00: 4E800421  bctrl
	ctx.lr = 0x83060E04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83060E04: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83060E08: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83060E0C: 4BF7E31D  bl 0x82fdf128
	ctx.lr = 0x83060E10;
	sub_82FDF128(ctx, base);
	// 83060E10: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83060E14: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83060E18: 7D6BC82E  lwzx r11, r11, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 83060E1C: 7F185840  cmplw cr6, r24, r11
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83060E20: 419A0020  beq cr6, 0x83060e40
	if ctx.cr[6].eq {
	pc = 0x83060E40; continue 'dispatch;
	}
	// 83060E24: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83060E28: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83060E2C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83060E30: 41820010  beq 0x83060e40
	if ctx.cr[0].eq {
	pc = 0x83060E40; continue 'dispatch;
	}
	// 83060E34: 38800056  li r4, 0x56
	ctx.r[4].s64 = 86;
	// 83060E38: 806B00A8  lwz r3, 0xa8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 83060E3C: 4BFB65AD  bl 0x830173e8
	ctx.lr = 0x83060E40;
	sub_830173E8(ctx, base);
	// 83060E40: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83060E44: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 83060E48: 48147360  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83060E4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83060E4C size=40
    let mut pc: u32 = 0x83060E4C;
    'dispatch: loop {
        match pc {
            0x83060E4C => {
    //   block [0x83060E4C..0x83060E74)
	// 83060E4C: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83060E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83060E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83060E58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83060E5C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83060E60: 4BF73129  bl 0x82fd3f88
	ctx.lr = 0x83060E64;
	sub_82FD3F88(ctx, base);
	// 83060E64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83060E68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83060E6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83060E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83060E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83060E78 size=8
    let mut pc: u32 = 0x83060E78;
    'dispatch: loop {
        match pc {
            0x83060E78 => {
    //   block [0x83060E78..0x83060E80)
	// 83060E78: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83060E7C: 82165D00  lwz r16, 0x5d00(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(23808 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83060E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83060E80 size=808
    let mut pc: u32 = 0x83060E80;
    'dispatch: loop {
        match pc {
            0x83060E80 => {
    //   block [0x83060E80..0x830611A8)
	// 83060E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83060E84: 481472D9  bl 0x831a815c
	ctx.lr = 0x83060E88;
	sub_831A8130(ctx, base);
	// 83060E88: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 83060E8C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83060E90: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83060E94: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83060E98: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83060E9C: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 83060EA0: 480033A9  bl 0x83064248
	ctx.lr = 0x83060EA4;
	sub_83064248(ctx, base);
	// 83060EA4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83060EA8: 40820010  bne 0x83060eb8
	if !ctx.cr[0].eq {
	pc = 0x83060EB8; continue 'dispatch;
	}
	// 83060EAC: 388000CF  li r4, 0xcf
	ctx.r[4].s64 = 207;
	// 83060EB0: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83060EB4: 4BF879D5  bl 0x82fe8888
	ctx.lr = 0x83060EB8;
	sub_82FE8888(ctx, base);
	// 83060EB8: 833E0028  lwz r25, 0x28(r30)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83060EBC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83060EC0: 4BF7E139  bl 0x82fdeff8
	ctx.lr = 0x83060EC4;
	sub_82FDEFF8(ctx, base);
	// 83060EC4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83060EC8: 933F005C  stw r25, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[25].u32 ) };
	// 83060ECC: 93BF0058  stw r29, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 83060ED0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83060ED4: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83060ED8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83060EDC: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83060EE0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83060EE4: 939D0004  stw r28, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 83060EE8: B38A0000  sth r28, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[28].u16 ) };
	// 83060EEC: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83060EF0: 4BF92219  bl 0x82ff3108
	ctx.lr = 0x83060EF4;
	sub_82FF3108(ctx, base);
	// 83060EF4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83060EF8: 40820020  bne 0x83060f18
	if !ctx.cr[0].eq {
	pc = 0x83060F18; continue 'dispatch;
	}
	// 83060EFC: 388000B7  li r4, 0xb7
	ctx.r[4].s64 = 183;
	// 83060F00: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83060F04: 4BF87985  bl 0x82fe8888
	ctx.lr = 0x83060F08;
	sub_82FE8888(ctx, base);
	// 83060F08: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 83060F0C: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83060F10: 4BFB6901  bl 0x83017810
	ctx.lr = 0x83060F14;
	sub_83017810(ctx, base);
	// 83060F14: 48000280  b 0x83061194
	pc = 0x83061194; continue 'dispatch;
	// 83060F18: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83060F1C: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 83060F20: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83060F24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83060F28: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83060F2C: 7F8B532E  sthx r28, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u16) };
	// 83060F30: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83060F34: 80DD0018  lwz r6, 0x18(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83060F38: 809E0038  lwz r4, 0x38(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 83060F3C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83060F40: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83060F44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83060F48: 4E800421  bctrl
	ctx.lr = 0x83060F4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83060F4C: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 83060F50: 418200D4  beq 0x83061024
	if ctx.cr[0].eq {
	pc = 0x83061024; continue 'dispatch;
	}
	// 83060F54: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 83060F58: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83060F5C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83060F60: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83060F64: 41820124  beq 0x83061088
	if ctx.cr[0].eq {
	pc = 0x83061088; continue 'dispatch;
	}
	// 83060F68: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83060F6C: 896B0010  lbz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83060F70: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83060F74: 41820034  beq 0x83060fa8
	if ctx.cr[0].eq {
	pc = 0x83060FA8; continue 'dispatch;
	}
	// 83060F78: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83060F7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83060F80: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83060F84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83060F88: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83060F8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83060F90: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 83060F94: 7F8B532E  sthx r28, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u16) };
	// 83060F98: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83060F9C: 80BD0018  lwz r5, 0x18(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83060FA0: 806B00A8  lwz r3, 0xa8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 83060FA4: 4BFB65B5  bl 0x83017558
	ctx.lr = 0x83060FA8;
	sub_83017558(ctx, base);
	// 83060FA8: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83060FAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83060FB0: 409A0050  bne cr6, 0x83061000
	if !ctx.cr[6].eq {
	pc = 0x83061000; continue 'dispatch;
	}
	// 83060FB4: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 83060FB8: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83060FBC: 4BF772DD  bl 0x82fd8298
	ctx.lr = 0x83060FC0;
	sub_82FD8298(ctx, base);
	// 83060FC0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83060FC4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83060FC8: 4182002C  beq 0x83060ff4
	if ctx.cr[0].eq {
	pc = 0x83060FF4; continue 'dispatch;
	}
	// 83060FCC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83060FD0: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83060FD4: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83060FD8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83060FDC: 7F8B532E  sthx r28, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u16) };
	// 83060FE0: 809D0018  lwz r4, 0x18(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83060FE4: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83060FE8: 80BE0038  lwz r5, 0x38(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 83060FEC: 48003805  bl 0x830647f0
	ctx.lr = 0x83060FF0;
	sub_830647F0(ctx, base);
	// 83060FF0: 48000008  b 0x83060ff8
	pc = 0x83060FF8; continue 'dispatch;
	// 83060FF4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83060FF8: 907E0014  stw r3, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 83060FFC: 4800008C  b 0x83061088
	pc = 0x83061088; continue 'dispatch;
	// 83061000: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83061004: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83061008: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8306100C: 7F8B532E  sthx r28, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u16) };
	// 83061010: 809D0018  lwz r4, 0x18(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83061014: 80BE0038  lwz r5, 0x38(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 83061018: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8306101C: 4801D505  bl 0x8307e520
	ctx.lr = 0x83061020;
	sub_8307E520(ctx, base);
	// 83061020: 48000068  b 0x83061088
	pc = 0x83061088; continue 'dispatch;
	// 83061024: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 83061028: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8306102C: 4BF7726D  bl 0x82fd8298
	ctx.lr = 0x83061030;
	sub_82FD8298(ctx, base);
	// 83061030: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83061034: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83061038: 41820030  beq 0x83061068
	if ctx.cr[0].eq {
	pc = 0x83061068; continue 'dispatch;
	}
	// 8306103C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83061040: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83061044: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83061048: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8306104C: 7F8B532E  sthx r28, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u16) };
	// 83061050: 809D0018  lwz r4, 0x18(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83061054: 80FE0008  lwz r7, 8(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83061058: 80BE0038  lwz r5, 0x38(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 8306105C: 48003795  bl 0x830647f0
	ctx.lr = 0x83061060;
	sub_830647F0(ctx, base);
	// 83061060: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83061064: 48000008  b 0x8306106c
	pc = 0x8306106C; continue 'dispatch;
	// 83061068: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8306106C: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83061070: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83061074: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83061078: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8306107C: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 83061080: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83061084: 4E800421  bctrl
	ctx.lr = 0x83061088;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83061088: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 8306108C: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 83061090: 813E003C  lwz r9, 0x3c(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 83061094: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 83061098: 614A8028  ori r10, r10, 0x8028
	ctx.r[10].u64 = ctx.r[10].u64 | 32808;
	// 8306109C: 80FE0014  lwz r7, 0x14(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830610A0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830610A4: 7CFB3850  subf r7, r27, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[27].s64;
	// 830610A8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830610AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830610B0: 7CE70034  cntlzw r7, r7
	ctx.r[7].u64 = if ctx.r[7].u32 == 0 { 32 } else { ctx.r[7].u32.leading_zeros() as u64 };
	// 830610B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830610B8: 54FADFFE  rlwinm r26, r7, 0x1b, 0x1f, 0x1f
	ctx.r[26].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 830610BC: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830610C0: 911B000C  stw r8, 0xc(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 830610C4: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 830610C8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830610CC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830610D0: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 830610D4: 997B0014  stb r11, 0x14(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 830610D8: 48003171  bl 0x83064248
	ctx.lr = 0x830610DC;
	sub_83064248(ctx, base);
	// 830610DC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830610E0: 40820010  bne 0x830610f0
	if !ctx.cr[0].eq {
	pc = 0x830610F0; continue 'dispatch;
	}
	// 830610E4: 388000CF  li r4, 0xcf
	ctx.r[4].s64 = 207;
	// 830610E8: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830610EC: 4BF8779D  bl 0x82fe8888
	ctx.lr = 0x830610F0;
	sub_82FE8888(ctx, base);
	// 830610F0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830610F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830610F8: 4BFFFB69  bl 0x83060c60
	ctx.lr = 0x830610FC;
	sub_83060C60(ctx, base);
	// 830610FC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83061100: 40820014  bne 0x83061114
	if !ctx.cr[0].eq {
	pc = 0x83061114; continue 'dispatch;
	}
	// 83061104: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 83061108: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 8306110C: 4BFB6705  bl 0x83017810
	ctx.lr = 0x83061110;
	sub_83017810(ctx, base);
	// 83061110: 48000084  b 0x83061194
	pc = 0x83061194; continue 'dispatch;
	// 83061114: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83061118: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8306111C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83061120: 48003129  bl 0x83064248
	ctx.lr = 0x83061124;
	sub_83064248(ctx, base);
	// 83061124: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 83061128: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 8306112C: 4BF8D185  bl 0x82fee2b0
	ctx.lr = 0x83061130;
	sub_82FEE2B0(ctx, base);
	// 83061130: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83061134: 4082003C  bne 0x83061170
	if !ctx.cr[0].eq {
	pc = 0x83061170; continue 'dispatch;
	}
	// 83061138: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8306113C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83061140: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83061144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83061148: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8306114C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83061150: 388000E8  li r4, 0xe8
	ctx.r[4].s64 = 232;
	// 83061154: 7F8B532E  sthx r28, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u16) };
	// 83061158: 80BD0018  lwz r5, 0x18(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8306115C: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83061160: 4BF87879  bl 0x82fe89d8
	ctx.lr = 0x83061164;
	sub_82FE89D8(ctx, base);
	// 83061164: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 83061168: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 8306116C: 4BFB66A5  bl 0x83017810
	ctx.lr = 0x83061170;
	sub_83017810(ctx, base);
	// 83061170: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83061174: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83061178: 4182001C  beq 0x83061194
	if ctx.cr[0].eq {
	pc = 0x83061194; continue 'dispatch;
	}
	// 8306117C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83061180: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83061184: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83061188: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8306118C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83061190: 4E800421  bctrl
	ctx.lr = 0x83061194;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83061194: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83061198: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8306119C: 4BF7DF8D  bl 0x82fdf128
	ctx.lr = 0x830611A0;
	sub_82FDF128(ctx, base);
	// 830611A0: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 830611A4: 48147008  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830611A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830611A8 size=40
    let mut pc: u32 = 0x830611A8;
    'dispatch: loop {
        match pc {
            0x830611A8 => {
    //   block [0x830611A8..0x830611D0)
	// 830611A8: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 830611AC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830611B0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830611B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830611B8: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 830611BC: 4BF72DCD  bl 0x82fd3f88
	ctx.lr = 0x830611C0;
	sub_82FD3F88(ctx, base);
	// 830611C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830611C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830611C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830611CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830611D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830611D0 size=48
    let mut pc: u32 = 0x830611D0;
    'dispatch: loop {
        match pc {
            0x830611D0 => {
    //   block [0x830611D0..0x83061200)
	// 830611D0: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 830611D4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830611D8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830611DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830611E0: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 830611E4: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830611E8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830611EC: 4BF770F5  bl 0x82fd82e0
	ctx.lr = 0x830611F0;
	sub_82FD82E0(ctx, base);
	// 830611F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830611F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830611F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830611FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83061200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83061200 size=48
    let mut pc: u32 = 0x83061200;
    'dispatch: loop {
        match pc {
            0x83061200 => {
    //   block [0x83061200..0x83061230)
	// 83061200: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83061204: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83061208: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8306120C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83061210: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 83061214: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83061218: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8306121C: 4BF770C5  bl 0x82fd82e0
	ctx.lr = 0x83061220;
	sub_82FD82E0(ctx, base);
	// 83061220: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83061224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83061228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8306122C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83061230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83061230 size=452
    let mut pc: u32 = 0x83061230;
    'dispatch: loop {
        match pc {
            0x83061230 => {
    //   block [0x83061230..0x830613F4)
	// 83061230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83061234: 48146F31  bl 0x831a8164
	ctx.lr = 0x83061238;
	sub_831A8130(ctx, base);
	// 83061238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8306123C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83061240: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83061244: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83061248: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8306124C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83061250: 815E0018  lwz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83061254: 388B7CC0  addi r4, r11, 0x7cc0
	ctx.r[4].s64 = ctx.r[11].s64 + 31936;
	// 83061258: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8306125C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 83061260: B3AA0000  sth r29, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[29].u16 ) };
	// 83061264: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83061268: 93BC0004  stw r29, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8306126C: B3AB0000  sth r29, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u16 ) };
	// 83061270: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83061274: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83061278: 4BF92259  bl 0x82ff34d0
	ctx.lr = 0x8306127C;
	sub_82FF34D0(ctx, base);
	// 8306127C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83061280: 4182004C  beq 0x830612cc
	if ctx.cr[0].eq {
	pc = 0x830612CC; continue 'dispatch;
	}
	// 83061284: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 83061288: 409A0018  bne cr6, 0x830612a0
	if !ctx.cr[6].eq {
	pc = 0x830612A0; continue 'dispatch;
	}
	// 8306128C: 388000D2  li r4, 0xd2
	ctx.r[4].s64 = 210;
	// 83061290: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83061294: 4BF875F5  bl 0x82fe8888
	ctx.lr = 0x83061298;
	sub_82FE8888(ctx, base);
	// 83061298: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8306129C: 48000150  b 0x830613ec
	pc = 0x830613EC; continue 'dispatch;
	// 830612A0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830612A4: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 830612A8: 4BF8D179  bl 0x82fee420
	ctx.lr = 0x830612AC;
	sub_82FEE420(ctx, base);
	// 830612AC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830612B0: 4082000C  bne 0x830612bc
	if !ctx.cr[0].eq {
	pc = 0x830612BC; continue 'dispatch;
	}
	// 830612B4: 388000CF  li r4, 0xcf
	ctx.r[4].s64 = 207;
	// 830612B8: 4BFFFFD8  b 0x83061290
	pc = 0x83061290; continue 'dispatch;
	// 830612BC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830612C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830612C4: 4BFFF3CD  bl 0x83060690
	ctx.lr = 0x830612C8;
	sub_83060690(ctx, base);
	// 830612C8: 48000124  b 0x830613ec
	pc = 0x830613EC; continue 'dispatch;
	// 830612CC: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 830612D0: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 830612D4: 388A7C6C  addi r4, r10, 0x7c6c
	ctx.r[4].s64 = ctx.r[10].s64 + 31852;
	// 830612D8: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830612DC: 4BF921F5  bl 0x82ff34d0
	ctx.lr = 0x830612E0;
	sub_82FF34D0(ctx, base);
	// 830612E0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830612E4: 4082000C  bne 0x830612f0
	if !ctx.cr[0].eq {
	pc = 0x830612F0; continue 'dispatch;
	}
	// 830612E8: 388000ED  li r4, 0xed
	ctx.r[4].s64 = 237;
	// 830612EC: 4BFFFFA4  b 0x83061290
	pc = 0x83061290; continue 'dispatch;
	// 830612F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830612F4: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 830612F8: 4BF8D129  bl 0x82fee420
	ctx.lr = 0x830612FC;
	sub_82FEE420(ctx, base);
	// 830612FC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83061300: 4082002C  bne 0x8306132c
	if !ctx.cr[0].eq {
	pc = 0x8306132C; continue 'dispatch;
	}
	// 83061304: 388000CF  li r4, 0xcf
	ctx.r[4].s64 = 207;
	// 83061308: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8306130C: 4BF8757D  bl 0x82fe8888
	ctx.lr = 0x83061310;
	sub_82FE8888(ctx, base);
	// 83061310: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83061314: 4BF8CF35  bl 0x82fee248
	ctx.lr = 0x83061318;
	sub_82FEE248(ctx, base);
	// 83061318: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 8306131C: 2B0B0022  cmplwi cr6, r11, 0x22
	ctx.cr[6].compare_u32(ctx.r[11].u32, 34 as u32, &mut ctx.xer);
	// 83061320: 419A000C  beq cr6, 0x8306132c
	if ctx.cr[6].eq {
	pc = 0x8306132C; continue 'dispatch;
	}
	// 83061324: 2B0B0027  cmplwi cr6, r11, 0x27
	ctx.cr[6].compare_u32(ctx.r[11].u32, 39 as u32, &mut ctx.xer);
	// 83061328: 409AFF70  bne cr6, 0x83061298
	if !ctx.cr[6].eq {
	pc = 0x83061298; continue 'dispatch;
	}
	// 8306132C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83061330: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83061334: 4BFFF25D  bl 0x83060590
	ctx.lr = 0x83061338;
	sub_83060590(ctx, base);
	// 83061338: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8306133C: 4182FF5C  beq 0x83061298
	if ctx.cr[0].eq {
	pc = 0x83061298; continue 'dispatch;
	}
	// 83061340: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 83061344: 419A00A4  beq cr6, 0x830613e8
	if ctx.cr[6].eq {
	pc = 0x830613E8; continue 'dispatch;
	}
	// 83061348: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8306134C: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83061350: 4BF8D0D1  bl 0x82fee420
	ctx.lr = 0x83061354;
	sub_82FEE420(ctx, base);
	// 83061354: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83061358: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8306135C: 4BF8CEED  bl 0x82fee248
	ctx.lr = 0x83061360;
	sub_82FEE248(ctx, base);
	// 83061360: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 83061364: 2B0B0022  cmplwi cr6, r11, 0x22
	ctx.cr[6].compare_u32(ctx.r[11].u32, 34 as u32, &mut ctx.xer);
	// 83061368: 419A000C  beq cr6, 0x83061374
	if ctx.cr[6].eq {
	pc = 0x83061374; continue 'dispatch;
	}
	// 8306136C: 2B0B0027  cmplwi cr6, r11, 0x27
	ctx.cr[6].compare_u32(ctx.r[11].u32, 39 as u32, &mut ctx.xer);
	// 83061370: 409A0008  bne cr6, 0x83061378
	if !ctx.cr[6].eq {
	pc = 0x83061378; continue 'dispatch;
	}
	// 83061374: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 83061378: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8306137C: 40820038  bne 0x830613b4
	if !ctx.cr[0].eq {
	pc = 0x830613B4; continue 'dispatch;
	}
	// 83061380: 2F1B0001  cmpwi cr6, r27, 1
	ctx.cr[6].compare_i32(ctx.r[27].s32, 1, &mut ctx.xer);
	// 83061384: 409A001C  bne cr6, 0x830613a0
	if !ctx.cr[6].eq {
	pc = 0x830613A0; continue 'dispatch;
	}
	// 83061388: 388000CF  li r4, 0xcf
	ctx.r[4].s64 = 207;
	// 8306138C: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83061390: 4BF874F9  bl 0x82fe8888
	ctx.lr = 0x83061394;
	sub_82FE8888(ctx, base);
	// 83061394: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83061398: 4082001C  bne 0x830613b4
	if !ctx.cr[0].eq {
	pc = 0x830613B4; continue 'dispatch;
	}
	// 8306139C: 4BFFFEFC  b 0x83061298
	pc = 0x83061298; continue 'dispatch;
	// 830613A0: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830613A4: 41820044  beq 0x830613e8
	if ctx.cr[0].eq {
	pc = 0x830613E8; continue 'dispatch;
	}
	// 830613A8: 388000CF  li r4, 0xcf
	ctx.r[4].s64 = 207;
	// 830613AC: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830613B0: 4BF874D9  bl 0x82fe8888
	ctx.lr = 0x830613B4;
	sub_82FE8888(ctx, base);
	// 830613B4: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830613B8: 4182001C  beq 0x830613d4
	if ctx.cr[0].eq {
	pc = 0x830613D4; continue 'dispatch;
	}
	// 830613BC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830613C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830613C4: 4BFFF2CD  bl 0x83060690
	ctx.lr = 0x830613C8;
	sub_83060690(ctx, base);
	// 830613C8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830613CC: 4082001C  bne 0x830613e8
	if !ctx.cr[0].eq {
	pc = 0x830613E8; continue 'dispatch;
	}
	// 830613D0: 4BFFFEC8  b 0x83061298
	pc = 0x83061298; continue 'dispatch;
	// 830613D4: 2F1B0001  cmpwi cr6, r27, 1
	ctx.cr[6].compare_i32(ctx.r[27].s32, 1, &mut ctx.xer);
	// 830613D8: 409A0010  bne cr6, 0x830613e8
	if !ctx.cr[6].eq {
	pc = 0x830613E8; continue 'dispatch;
	}
	// 830613DC: 388000D1  li r4, 0xd1
	ctx.r[4].s64 = 209;
	// 830613E0: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830613E4: 4BF874A5  bl 0x82fe8888
	ctx.lr = 0x830613E8;
	sub_82FE8888(ctx, base);
	// 830613E8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830613EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830613F0: 48146DC4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830613F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830613F8 size=8
    let mut pc: u32 = 0x830613F8;
    'dispatch: loop {
        match pc {
            0x830613F8 => {
    //   block [0x830613F8..0x83061400)
	// 830613F8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830613FC: 82165D90  lwz r16, 0x5d90(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(23952 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83061400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83061400 size=1176
    let mut pc: u32 = 0x83061400;
    'dispatch: loop {
        match pc {
            0x83061400 => {
    //   block [0x83061400..0x83061898)
	// 83061400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83061404: 48146D4D  bl 0x831a8150
	ctx.lr = 0x83061408;
	sub_831A8130(ctx, base);
	// 83061408: 3BE1FF20  addi r31, r1, -0xe0
	ctx.r[31].s64 = ctx.r[1].s64 + -224;
	// 8306140C: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83061410: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 83061414: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83061418: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8306141C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83061420: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 83061424: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 83061428: 9B770000  stb r27, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[27].u8 ) };
	// 8306142C: 617A8028  ori r26, r11, 0x8028
	ctx.r[26].u64 = ctx.r[11].u64 | 32808;
	// 83061430: B37D0000  sth r27, 0(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[27].u16 ) };
	// 83061434: 38800023  li r4, 0x23
	ctx.r[4].s64 = 35;
	// 83061438: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 8306143C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83061440: 7F2BD02E  lwzx r25, r11, r26
	ctx.r[25].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 83061444: 4BF8CE6D  bl 0x82fee2b0
	ctx.lr = 0x83061448;
	sub_82FEE2B0(ctx, base);
	// 83061448: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8306144C: 41820054  beq 0x830614a0
	if ctx.cr[0].eq {
	pc = 0x830614A0; continue 'dispatch;
	}
	// 83061450: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83061454: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83061458: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8306145C: 4BFFD31D  bl 0x8305e778
	ctx.lr = 0x83061460;
	sub_8305E778(ctx, base);
	// 83061460: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83061464: 4082000C  bne 0x83061470
	if !ctx.cr[0].eq {
	pc = 0x83061470; continue 'dispatch;
	}
	// 83061468: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8306146C: 48000424  b 0x83061890
	pc = 0x83061890; continue 'dispatch;
	// 83061470: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83061474: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83061478: 7D6BD02E  lwzx r11, r11, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 8306147C: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83061480: 419A0010  beq cr6, 0x83061490
	if ctx.cr[6].eq {
	pc = 0x83061490; continue 'dispatch;
	}
	// 83061484: 388000E7  li r4, 0xe7
	ctx.r[4].s64 = 231;
	// 83061488: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8306148C: 4BF873FD  bl 0x82fe8888
	ctx.lr = 0x83061490;
	sub_82FE8888(ctx, base);
	// 83061490: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83061494: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83061498: 99770000  stb r11, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8306149C: 480003F4  b 0x83061890
	pc = 0x83061890; continue 'dispatch;
	// 830614A0: 82DE0028  lwz r22, 0x28(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830614A4: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 830614A8: 4BF7DB51  bl 0x82fdeff8
	ctx.lr = 0x830614AC;
	sub_82FDEFF8(ctx, base);
	// 830614AC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830614B0: 92DF006C  stw r22, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[22].u32 ) };
	// 830614B4: 939F0068  stw r28, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[28].u32 ) };
	// 830614B8: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 830614BC: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 830614C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830614C4: 937C0004  stw r27, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 830614C8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830614CC: B36A0000  sth r27, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[27].u16 ) };
	// 830614D0: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830614D4: 4BF91C35  bl 0x82ff3108
	ctx.lr = 0x830614D8;
	sub_82FF3108(ctx, base);
	// 830614D8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830614DC: 40820020  bne 0x830614fc
	if !ctx.cr[0].eq {
	pc = 0x830614FC; continue 'dispatch;
	}
	// 830614E0: 388000E2  li r4, 0xe2
	ctx.r[4].s64 = 226;
	// 830614E4: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830614E8: 4BF873A1  bl 0x82fe8888
	ctx.lr = 0x830614EC;
	sub_82FE8888(ctx, base);
	// 830614EC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830614F0: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 830614F4: 4BF7DC35  bl 0x82fdf128
	ctx.lr = 0x830614F8;
	sub_82FDF128(ctx, base);
	// 830614F8: 4BFFFF70  b 0x83061468
	pc = 0x83061468; continue 'dispatch;
	// 830614FC: 3880003B  li r4, 0x3b
	ctx.r[4].s64 = 59;
	// 83061500: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83061504: 4BF8CDAD  bl 0x82fee2b0
	ctx.lr = 0x83061508;
	sub_82FEE2B0(ctx, base);
	// 83061508: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8306150C: 40820030  bne 0x8306153c
	if !ctx.cr[0].eq {
	pc = 0x8306153C; continue 'dispatch;
	}
	// 83061510: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83061514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83061518: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 8306151C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83061520: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83061524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83061528: 388000E5  li r4, 0xe5
	ctx.r[4].s64 = 229;
	// 8306152C: 7F6B532E  sthx r27, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u16) };
	// 83061530: 80BC0018  lwz r5, 0x18(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83061534: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83061538: 4BF874A1  bl 0x82fe89d8
	ctx.lr = 0x8306153C;
	sub_82FE89D8(ctx, base);
	// 8306153C: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83061540: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83061544: 7D6BD02E  lwzx r11, r11, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 83061548: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8306154C: 419A0010  beq cr6, 0x8306155c
	if ctx.cr[6].eq {
	pc = 0x8306155C; continue 'dispatch;
	}
	// 83061550: 388000E7  li r4, 0xe7
	ctx.r[4].s64 = 231;
	// 83061554: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83061558: 4BF87331  bl 0x82fe8888
	ctx.lr = 0x8306155C;
	sub_82FE8888(ctx, base);
	// 8306155C: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83061560: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83061564: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83061568: 7F6B532E  sthx r27, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u16) };
	// 8306156C: 809C0018  lwz r4, 0x18(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83061570: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83061574: 4BFBE9BD  bl 0x8301ff30
	ctx.lr = 0x83061578;
	sub_8301FF30(ctx, base);
	// 83061578: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8306157C: 408200A0  bne 0x8306161c
	if !ctx.cr[0].eq {
	pc = 0x8306161C; continue 'dispatch;
	}
	// 83061580: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83061584: 894B000E  lbz r10, 0xe(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(14 as u32) ) } as u64;
	// 83061588: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8306158C: 40820050  bne 0x830615dc
	if !ctx.cr[0].eq {
	pc = 0x830615DC; continue 'dispatch;
	}
	// 83061590: 894B000F  lbz r10, 0xf(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(15 as u32) ) } as u64;
	// 83061594: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83061598: 40820044  bne 0x830615dc
	if !ctx.cr[0].eq {
	pc = 0x830615DC; continue 'dispatch;
	}
	// 8306159C: 896B0010  lbz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830615A0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830615A4: 41820064  beq 0x83061608
	if ctx.cr[0].eq {
	pc = 0x83061608; continue 'dispatch;
	}
	// 830615A8: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 830615AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830615B0: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 830615B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830615B8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830615BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830615C0: 38800055  li r4, 0x55
	ctx.r[4].s64 = 85;
	// 830615C4: 7F6B532E  sthx r27, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u16) };
	// 830615C8: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830615CC: 80BC0018  lwz r5, 0x18(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 830615D0: 806B00A8  lwz r3, 0xa8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 830615D4: 4BFB5F85  bl 0x83017558
	ctx.lr = 0x830615D8;
	sub_83017558(ctx, base);
	// 830615D8: 48000030  b 0x83061608
	pc = 0x83061608; continue 'dispatch;
	// 830615DC: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 830615E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830615E4: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 830615E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830615EC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830615F0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830615F4: 388000E3  li r4, 0xe3
	ctx.r[4].s64 = 227;
	// 830615F8: 7F6B532E  sthx r27, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u16) };
	// 830615FC: 80BC0018  lwz r5, 0x18(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83061600: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83061604: 4BF873D5  bl 0x82fe89d8
	ctx.lr = 0x83061608;
	sub_82FE89D8(ctx, base);
	// 83061608: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8306160C: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 83061610: 4BF7DB19  bl 0x82fdf128
	ctx.lr = 0x83061614;
	sub_82FDF128(ctx, base);
	// 83061614: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83061618: 48000278  b 0x83061890
	pc = 0x83061890; continue 'dispatch;
	// 8306161C: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83061620: 896B000E  lbz r11, 0xe(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(14 as u32) ) } as u64;
	// 83061624: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83061628: 4182004C  beq 0x83061674
	if ctx.cr[0].eq {
	pc = 0x83061674; continue 'dispatch;
	}
	// 8306162C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83061630: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83061634: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83061638: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8306163C: 4E800421  bctrl
	ctx.lr = 0x83061640;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83061640: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83061644: 40820030  bne 0x83061674
	if !ctx.cr[0].eq {
	pc = 0x83061674; continue 'dispatch;
	}
	// 83061648: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8306164C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83061650: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83061654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83061658: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8306165C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83061660: 3880012A  li r4, 0x12a
	ctx.r[4].s64 = 298;
	// 83061664: 7F6B532E  sthx r27, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u16) };
	// 83061668: 80BC0018  lwz r5, 0x18(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 8306166C: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83061670: 4BF87369  bl 0x82fe89d8
	ctx.lr = 0x83061674;
	sub_82FE89D8(ctx, base);
	// 83061674: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83061678: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8306167C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83061680: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83061684: 4E800421  bctrl
	ctx.lr = 0x83061688;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83061688: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8306168C: 41820020  beq 0x830616ac
	if ctx.cr[0].eq {
	pc = 0x830616AC; continue 'dispatch;
	}
	// 83061690: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 83061694: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83061698: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8306169C: B1780000  sth r11, 0(r24)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 830616A0: 99570000  stb r10, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 830616A4: 3B600002  li r27, 2
	ctx.r[27].s64 = 2;
	// 830616A8: 4BFFFF60  b 0x83061608
	pc = 0x83061608; continue 'dispatch;
	// 830616AC: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 830616B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830616B4: 409A0014  bne cr6, 0x830616c8
	if !ctx.cr[6].eq {
	pc = 0x830616C8; continue 'dispatch;
	}
	// 830616B8: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 830616BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830616C0: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 830616C4: 419A0008  beq cr6, 0x830616cc
	if ctx.cr[6].eq {
	pc = 0x830616CC; continue 'dispatch;
	}
	// 830616C8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830616CC: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830616D0: 41820158  beq 0x83061828
	if ctx.cr[0].eq {
	pc = 0x83061828; continue 'dispatch;
	}
	// 830616D4: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 830616D8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830616DC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830616E0: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 830616E4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830616E8: 41820034  beq 0x8306171c
	if ctx.cr[0].eq {
	pc = 0x8306171C; continue 'dispatch;
	}
	// 830616EC: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 830616F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830616F4: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 830616F8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830616FC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83061700: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83061704: 388000E4  li r4, 0xe4
	ctx.r[4].s64 = 228;
	// 83061708: 7F6B532E  sthx r27, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u16) };
	// 8306170C: 80BC0018  lwz r5, 0x18(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83061710: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83061714: 4BF872C5  bl 0x82fe89d8
	ctx.lr = 0x83061718;
	sub_82FE89D8(ctx, base);
	// 83061718: 4BFFFEF0  b 0x83061608
	pc = 0x83061608; continue 'dispatch;
	// 8306171C: 3880010E  li r4, 0x10e
	ctx.r[4].s64 = 270;
	// 83061720: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83061724: 4BF87165  bl 0x82fe8888
	ctx.lr = 0x83061728;
	sub_82FE8888(ctx, base);
	// 83061728: 397F0064  addi r11, r31, 0x64
	ctx.r[11].s64 = ctx.r[31].s64 + 100;
	// 8306172C: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 83061730: 80DD0018  lwz r6, 0x18(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83061734: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83061738: 80BD001C  lwz r5, 0x1c(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 8306173C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83061740: 809D0020  lwz r4, 0x20(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 83061744: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 83061748: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 8306174C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83061750: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83061754: 9B41005F  stb r26, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[26].u8 ) };
	// 83061758: 4BF8D821  bl 0x82feef78
	ctx.lr = 0x8306175C;
	sub_82FEEF78(ctx, base);
	// 8306175C: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83061760: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 83061764: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83061768: 40820054  bne 0x830617bc
	if !ctx.cr[0].eq {
	pc = 0x830617BC; continue 'dispatch;
	}
	// 8306176C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83061770: 83DE0004  lwz r30, 4(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83061774: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83061778: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8306177C: 4E800421  bctrl
	ctx.lr = 0x83061780;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83061780: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83061784: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 83061788: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8306178C: 388B5790  addi r4, r11, 0x5790
	ctx.r[4].s64 = ctx.r[11].s64 + 22416;
	// 83061790: 38C0002E  li r6, 0x2e
	ctx.r[6].s64 = 46;
	// 83061794: 38A00897  li r5, 0x897
	ctx.r[5].s64 = 2199;
	// 83061798: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8306179C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830617A0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830617A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830617A8: 4BFB5FD9  bl 0x83017780
	ctx.lr = 0x830617AC;
	sub_83017780(ctx, base);
	// 830617AC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830617B0: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 830617B4: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830617B8: 4814F471  bl 0x831b0c28
	ctx.lr = 0x830617BC;
	sub_831B0C28(ctx, base);
	// 830617BC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830617C0: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 830617C4: 4BF8CEE5  bl 0x82fee6a8
	ctx.lr = 0x830617C8;
	sub_82FEE6A8(ctx, base);
	// 830617C8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830617CC: 40820030  bne 0x830617fc
	if !ctx.cr[0].eq {
	pc = 0x830617FC; continue 'dispatch;
	}
	// 830617D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830617D4: 80BD0010  lwz r5, 0x10(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 830617D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830617DC: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830617E0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830617E4: 388000E6  li r4, 0xe6
	ctx.r[4].s64 = 230;
	// 830617E8: 4BF871F1  bl 0x82fe89d8
	ctx.lr = 0x830617EC;
	sub_82FE89D8(ctx, base);
	// 830617EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830617F0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830617F4: 480074E5  bl 0x83068cd8
	ctx.lr = 0x830617F8;
	sub_83068CD8(ctx, base);
	// 830617F8: 4BFFFE10  b 0x83061608
	pc = 0x83061608; continue 'dispatch;
	// 830617FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83061800: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83061804: 4BF88015  bl 0x82fe9818
	ctx.lr = 0x83061808;
	sub_82FE9818(ctx, base);
	// 83061808: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8306180C: 4182000C  beq 0x83061818
	if ctx.cr[0].eq {
	pc = 0x83061818; continue 'dispatch;
	}
	// 83061810: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83061814: 4BFFEF4D  bl 0x83060760
	ctx.lr = 0x83061818;
	sub_83060760(ctx, base);
	// 83061818: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8306181C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83061820: 480074B9  bl 0x83068cd8
	ctx.lr = 0x83061824;
	sub_83068CD8(ctx, base);
	// 83061824: 4800005C  b 0x83061880
	pc = 0x83061880; continue 'dispatch;
	// 83061828: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8306182C: 811D0008  lwz r8, 8(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83061830: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83061834: 80FD000C  lwz r7, 0xc(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 83061838: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8306183C: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83061840: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83061844: 809D0010  lwz r4, 0x10(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 83061848: 4BF8BC99  bl 0x82fed4e0
	ctx.lr = 0x8306184C;
	sub_82FED4E0(ctx, base);
	// 8306184C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83061850: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83061854: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83061858: 4BF8CE51  bl 0x82fee6a8
	ctx.lr = 0x8306185C;
	sub_82FEE6A8(ctx, base);
	// 8306185C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83061860: 40820020  bne 0x83061880
	if !ctx.cr[0].eq {
	pc = 0x83061880; continue 'dispatch;
	}
	// 83061864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83061868: 80BD0010  lwz r5, 0x10(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8306186C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83061870: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83061874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83061878: 388000E6  li r4, 0xe6
	ctx.r[4].s64 = 230;
	// 8306187C: 4BF8715D  bl 0x82fe89d8
	ctx.lr = 0x83061880;
	sub_82FE89D8(ctx, base);
	// 83061880: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83061884: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 83061888: 4BF7D8A1  bl 0x82fdf128
	ctx.lr = 0x8306188C;
	sub_82FDF128(ctx, base);
	// 8306188C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83061890: 383F00E0  addi r1, r31, 0xe0
	ctx.r[1].s64 = ctx.r[31].s64 + 224;
	// 83061894: 4814690C  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83061898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83061898 size=40
    let mut pc: u32 = 0x83061898;
    'dispatch: loop {
        match pc {
            0x83061898 => {
    //   block [0x83061898..0x830618C0)
	// 83061898: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 8306189C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830618A0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830618A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830618A8: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 830618AC: 4BF726DD  bl 0x82fd3f88
	ctx.lr = 0x830618B0;
	sub_82FD3F88(ctx, base);
	// 830618B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830618B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830618B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830618BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830618C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830618C0 size=40
    let mut pc: u32 = 0x830618C0;
    'dispatch: loop {
        match pc {
            0x830618C0 => {
    //   block [0x830618C0..0x830618E8)
	// 830618C0: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 830618C4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830618C8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830618CC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830618D0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830618D4: 4BFB6EB5  bl 0x83018788
	ctx.lr = 0x830618D8;
	sub_83018788(ctx, base);
	// 830618D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830618DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830618E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830618E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830618E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830618E8 size=8
    let mut pc: u32 = 0x830618E8;
    'dispatch: loop {
        match pc {
            0x830618E8 => {
    //   block [0x830618E8..0x830618F0)
	// 830618E8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830618EC: 82165E30  lwz r16, 0x5e30(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(24112 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830618F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830618F0 size=888
    let mut pc: u32 = 0x830618F0;
    'dispatch: loop {
        match pc {
            0x830618F0 => {
    //   block [0x830618F0..0x83061C68)
	// 830618F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830618F4: 4814685D  bl 0x831a8150
	ctx.lr = 0x830618F8;
	sub_831A8130(ctx, base);
	// 830618F8: 3BE1FF20  addi r31, r1, -0xe0
	ctx.r[31].s64 = ctx.r[1].s64 + -224;
	// 830618FC: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83061900: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83061904: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 83061908: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8306190C: 807D002C  lwz r3, 0x2c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 83061910: 4BF8C939  bl 0x82fee248
	ctx.lr = 0x83061914;
	sub_82FEE248(ctx, base);
	// 83061914: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 83061918: 396BFFD9  addi r11, r11, -0x27
	ctx.r[11].s64 = ctx.r[11].s64 + -39;
	// 8306191C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83061920: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83061924: 408202CC  bne 0x83061bf0
	if !ctx.cr[0].eq {
	pc = 0x83061BF0; continue 'dispatch;
	}
	// 83061928: 807D002C  lwz r3, 0x2c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8306192C: 4BF8C91D  bl 0x82fee248
	ctx.lr = 0x83061930;
	sub_82FEE248(ctx, base);
	// 83061930: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 83061934: 396BFFDE  addi r11, r11, -0x22
	ctx.r[11].s64 = ctx.r[11].s64 + -34;
	// 83061938: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8306193C: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83061940: 408202B0  bne 0x83061bf0
	if !ctx.cr[0].eq {
	pc = 0x83061BF0; continue 'dispatch;
	}
	// 83061944: 82DD0028  lwz r22, 0x28(r29)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 83061948: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 8306194C: 4BF7D6AD  bl 0x82fdeff8
	ctx.lr = 0x83061950;
	sub_82FDEFF8(ctx, base);
	// 83061950: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 83061954: 92DF0054  stw r22, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[22].u32 ) };
	// 83061958: 92FF0050  stw r23, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[23].u32 ) };
	// 8306195C: 833D0028  lwz r25, 0x28(r29)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 83061960: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83061964: 4BF7D695  bl 0x82fdeff8
	ctx.lr = 0x83061968;
	sub_82FDEFF8(ctx, base);
	// 83061968: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8306196C: 933F005C  stw r25, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[25].u32 ) };
	// 83061970: 937F0058  stw r27, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 83061974: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83061978: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8306197C: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 83061980: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83061984: 4BFFF8AD  bl 0x83061230
	ctx.lr = 0x83061988;
	sub_83061230(ctx, base);
	// 83061988: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8306198C: 40820028  bne 0x830619b4
	if !ctx.cr[0].eq {
	pc = 0x830619B4; continue 'dispatch;
	}
	// 83061990: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83061994: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83061998: 4BF7D791  bl 0x82fdf128
	ctx.lr = 0x8306199C;
	sub_82FDF128(ctx, base);
	// 8306199C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830619A0: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 830619A4: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 830619A8: 4BF7D781  bl 0x82fdf128
	ctx.lr = 0x830619AC;
	sub_82FDF128(ctx, base);
	// 830619AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830619B0: 480002B0  b 0x83061c60
	pc = 0x83061C60; continue 'dispatch;
	// 830619B4: 389F0070  addi r4, r31, 0x70
	ctx.r[4].s64 = ctx.r[31].s64 + 112;
	// 830619B8: 807D002C  lwz r3, 0x2c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 830619BC: 4BF8CC3D  bl 0x82fee5f8
	ctx.lr = 0x830619C0;
	sub_82FEE5F8(ctx, base);
	// 830619C0: 81770004  lwz r11, 4(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 830619C4: 81570018  lwz r10, 0x18(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24 as u32) ) } as u64;
	// 830619C8: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 830619CC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830619D0: 7F4B532E  sthx r26, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[26].u16) };
	// 830619D4: 815B0004  lwz r10, 4(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 830619D8: 813B0018  lwz r9, 0x18(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 830619DC: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830619E0: 81770018  lwz r11, 0x18(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24 as u32) ) } as u64;
	// 830619E4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830619E8: 7F4A4B2E  sthx r26, r10, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[26].u16) };
	// 830619EC: 83DB0018  lwz r30, 0x18(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 830619F0: 41820010  beq 0x83061a00
	if ctx.cr[0].eq {
	pc = 0x83061A00; continue 'dispatch;
	}
	// 830619F4: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830619F8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830619FC: 40820008  bne 0x83061a04
	if !ctx.cr[0].eq {
	pc = 0x83061A04; continue 'dispatch;
	}
	// 83061A00: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 83061A04: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 83061A08: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83061A0C: 4BFFC915  bl 0x8305e320
	ctx.lr = 0x83061A10;
	sub_8305E320(ctx, base);
	// 83061A10: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83061A14: 419A0014  beq cr6, 0x83061a28
	if ctx.cr[6].eq {
	pc = 0x83061A28; continue 'dispatch;
	}
	// 83061A18: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83061A1C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83061A20: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83061A24: 40820008  bne 0x83061a2c
	if !ctx.cr[0].eq {
	pc = 0x83061A2C; continue 'dispatch;
	}
	// 83061A28: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83061A2C: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83061A30: 4BFB5F29  bl 0x83017958
	ctx.lr = 0x83061A34;
	sub_83017958(ctx, base);
	// 83061A34: 809F0070  lwz r4, 0x70(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 83061A38: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83061A3C: 419A0010  beq cr6, 0x83061a4c
	if ctx.cr[6].eq {
	pc = 0x83061A4C; continue 'dispatch;
	}
	// 83061A40: A1640000  lhz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83061A44: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83061A48: 40820008  bne 0x83061a50
	if !ctx.cr[0].eq {
	pc = 0x83061A50; continue 'dispatch;
	}
	// 83061A4C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83061A50: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83061A54: 4BFFC935  bl 0x8305e388
	ctx.lr = 0x83061A58;
	sub_8305E388(ctx, base);
	// 83061A58: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83061A5C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83061A60: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83061A64: 480027E5  bl 0x83064248
	ctx.lr = 0x83061A68;
	sub_83064248(ctx, base);
	// 83061A68: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83061A6C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83061A70: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83061A74: 3BCB7C24  addi r30, r11, 0x7c24
	ctx.r[30].s64 = ctx.r[11].s64 + 31780;
	// 83061A78: 41820030  beq 0x83061aa8
	if ctx.cr[0].eq {
	pc = 0x83061AA8; continue 'dispatch;
	}
	// 83061A7C: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83061A80: 41820054  beq 0x83061ad4
	if ctx.cr[0].eq {
	pc = 0x83061AD4; continue 'dispatch;
	}
	// 83061A84: 817D002C  lwz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 83061A88: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83061A8C: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83061A90: 4BF91A41  bl 0x82ff34d0
	ctx.lr = 0x83061A94;
	sub_82FF34D0(ctx, base);
	// 83061A94: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83061A98: 41820010  beq 0x83061aa8
	if ctx.cr[0].eq {
	pc = 0x83061AA8; continue 'dispatch;
	}
	// 83061A9C: 3880010A  li r4, 0x10a
	ctx.r[4].s64 = 266;
	// 83061AA0: 807D0030  lwz r3, 0x30(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 83061AA4: 4BF86DE5  bl 0x82fe8888
	ctx.lr = 0x83061AA8;
	sub_82FE8888(ctx, base);
	// 83061AA8: 807D002C  lwz r3, 0x2c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 83061AAC: 4BF8C79D  bl 0x82fee248
	ctx.lr = 0x83061AB0;
	sub_82FEE248(ctx, base);
	// 83061AB0: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 83061AB4: 396BFFC2  addi r11, r11, -0x3e
	ctx.r[11].s64 = ctx.r[11].s64 + -62;
	// 83061AB8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83061ABC: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83061AC0: 41820028  beq 0x83061ae8
	if ctx.cr[0].eq {
	pc = 0x83061AE8; continue 'dispatch;
	}
	// 83061AC4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83061AC8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83061ACC: 4BF7D65D  bl 0x82fdf128
	ctx.lr = 0x83061AD0;
	sub_82FDF128(ctx, base);
	// 83061AD0: 48000010  b 0x83061ae0
	pc = 0x83061AE0; continue 'dispatch;
	// 83061AD4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83061AD8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83061ADC: 4BF7D64D  bl 0x82fdf128
	ctx.lr = 0x83061AE0;
	sub_82FDF128(ctx, base);
	// 83061AE0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 83061AE4: 4BFFFEBC  b 0x830619a0
	pc = 0x830619A0; continue 'dispatch;
	// 83061AE8: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83061AEC: 40820010  bne 0x83061afc
	if !ctx.cr[0].eq {
	pc = 0x83061AFC; continue 'dispatch;
	}
	// 83061AF0: 388000CF  li r4, 0xcf
	ctx.r[4].s64 = 207;
	// 83061AF4: 807D0030  lwz r3, 0x30(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 83061AF8: 4BF86D91  bl 0x82fe8888
	ctx.lr = 0x83061AFC;
	sub_82FE8888(ctx, base);
	// 83061AFC: 817D002C  lwz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 83061B00: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83061B04: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83061B08: 4BF919C9  bl 0x82ff34d0
	ctx.lr = 0x83061B0C;
	sub_82FF34D0(ctx, base);
	// 83061B0C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83061B10: 40820010  bne 0x83061b20
	if !ctx.cr[0].eq {
	pc = 0x83061B20; continue 'dispatch;
	}
	// 83061B14: 38800109  li r4, 0x109
	ctx.r[4].s64 = 265;
	// 83061B18: 807D0030  lwz r3, 0x30(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 83061B1C: 4BF86D6D  bl 0x82fe8888
	ctx.lr = 0x83061B20;
	sub_82FE8888(ctx, base);
	// 83061B20: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83061B24: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83061B28: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83061B2C: 4800271D  bl 0x83064248
	ctx.lr = 0x83061B30;
	sub_83064248(ctx, base);
	// 83061B30: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83061B34: 40820010  bne 0x83061b44
	if !ctx.cr[0].eq {
	pc = 0x83061B44; continue 'dispatch;
	}
	// 83061B38: 388000CF  li r4, 0xcf
	ctx.r[4].s64 = 207;
	// 83061B3C: 807D0030  lwz r3, 0x30(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 83061B40: 4BF86D49  bl 0x82fe8888
	ctx.lr = 0x83061B44;
	sub_82FE8888(ctx, base);
	// 83061B44: 839D0028  lwz r28, 0x28(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 83061B48: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83061B4C: 4BF7D4AD  bl 0x82fdeff8
	ctx.lr = 0x83061B50;
	sub_82FDEFF8(ctx, base);
	// 83061B50: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83061B54: 939F0064  stw r28, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 83061B58: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 83061B5C: 815E0018  lwz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83061B60: 817D002C  lwz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 83061B64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83061B68: 935E0004  stw r26, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 83061B6C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83061B70: B34A0000  sth r26, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[26].u16 ) };
	// 83061B74: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83061B78: 4BF91591  bl 0x82ff3108
	ctx.lr = 0x83061B7C;
	sub_82FF3108(ctx, base);
	// 83061B7C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83061B80: 40820030  bne 0x83061bb0
	if !ctx.cr[0].eq {
	pc = 0x83061BB0; continue 'dispatch;
	}
	// 83061B84: 388000AF  li r4, 0xaf
	ctx.r[4].s64 = 175;
	// 83061B88: 807D0030  lwz r3, 0x30(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 83061B8C: 4BF86CFD  bl 0x82fe8888
	ctx.lr = 0x83061B90;
	sub_82FE8888(ctx, base);
	// 83061B90: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83061B94: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83061B98: 4BF7D591  bl 0x82fdf128
	ctx.lr = 0x83061B9C;
	sub_82FDF128(ctx, base);
	// 83061B9C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83061BA0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83061BA4: 4BF7D585  bl 0x82fdf128
	ctx.lr = 0x83061BA8;
	sub_82FDF128(ctx, base);
	// 83061BA8: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 83061BAC: 4BFFFDF4  b 0x830619a0
	pc = 0x830619A0; continue 'dispatch;
	// 83061BB0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83061BB4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83061BB8: 815E0018  lwz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83061BBC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83061BC0: 7F4B532E  sthx r26, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[26].u16) };
	// 83061BC4: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83061BC8: 4BFFC6F1  bl 0x8305e2b8
	ctx.lr = 0x83061BCC;
	sub_8305E2B8(ctx, base);
	// 83061BCC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83061BD0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83061BD4: 4BF7D555  bl 0x82fdf128
	ctx.lr = 0x83061BD8;
	sub_82FDF128(ctx, base);
	// 83061BD8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83061BDC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83061BE0: 4BF7D549  bl 0x82fdf128
	ctx.lr = 0x83061BE4;
	sub_82FDF128(ctx, base);
	// 83061BE4: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 83061BE8: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 83061BEC: 4800006C  b 0x83061c58
	pc = 0x83061C58; continue 'dispatch;
	// 83061BF0: 839D0028  lwz r28, 0x28(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 83061BF4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83061BF8: 4BF7D401  bl 0x82fdeff8
	ctx.lr = 0x83061BFC;
	sub_82FDEFF8(ctx, base);
	// 83061BFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83061C00: 939F0064  stw r28, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 83061C04: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 83061C08: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83061C0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83061C10: 4BFFD819  bl 0x8305f428
	ctx.lr = 0x83061C14;
	sub_8305F428(ctx, base);
	// 83061C14: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83061C18: 40820018  bne 0x83061c30
	if !ctx.cr[0].eq {
	pc = 0x83061C30; continue 'dispatch;
	}
	// 83061C1C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83061C20: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83061C24: 4BF7D505  bl 0x82fdf128
	ctx.lr = 0x83061C28;
	sub_82FDF128(ctx, base);
	// 83061C28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83061C2C: 48000034  b 0x83061c60
	pc = 0x83061C60; continue 'dispatch;
	// 83061C30: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83061C34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83061C38: 813E0018  lwz r9, 0x18(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83061C3C: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83061C40: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83061C44: 7D4B4B2E  sthx r10, r11, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u16) };
	// 83061C48: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83061C4C: 4BFFC7A5  bl 0x8305e3f0
	ctx.lr = 0x83061C50;
	sub_8305E3F0(ctx, base);
	// 83061C50: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83061C54: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83061C58: 4BF7D4D1  bl 0x82fdf128
	ctx.lr = 0x83061C5C;
	sub_82FDF128(ctx, base);
	// 83061C5C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83061C60: 383F00E0  addi r1, r31, 0xe0
	ctx.r[1].s64 = ctx.r[31].s64 + 224;
	// 83061C64: 4814653C  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83061C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83061C68 size=40
    let mut pc: u32 = 0x83061C68;
    'dispatch: loop {
        match pc {
            0x83061C68 => {
    //   block [0x83061C68..0x83061C90)
	// 83061C68: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 83061C6C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83061C70: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83061C74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83061C78: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83061C7C: 4BF7230D  bl 0x82fd3f88
	ctx.lr = 0x83061C80;
	sub_82FD3F88(ctx, base);
	// 83061C80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83061C84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83061C88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83061C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83061C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83061C90 size=40
    let mut pc: u32 = 0x83061C90;
    'dispatch: loop {
        match pc {
            0x83061C90 => {
    //   block [0x83061C90..0x83061CB8)
	// 83061C90: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 83061C94: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83061C98: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83061C9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83061CA0: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 83061CA4: 4BF722E5  bl 0x82fd3f88
	ctx.lr = 0x83061CA8;
	sub_82FD3F88(ctx, base);
	// 83061CA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83061CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83061CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83061CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83061CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83061CB8 size=40
    let mut pc: u32 = 0x83061CB8;
    'dispatch: loop {
        match pc {
            0x83061CB8 => {
    //   block [0x83061CB8..0x83061CE0)
	// 83061CB8: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 83061CBC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83061CC0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83061CC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83061CC8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83061CCC: 4BF722BD  bl 0x82fd3f88
	ctx.lr = 0x83061CD0;
	sub_82FD3F88(ctx, base);
	// 83061CD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83061CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83061CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83061CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83061CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83061CE0 size=40
    let mut pc: u32 = 0x83061CE0;
    'dispatch: loop {
        match pc {
            0x83061CE0 => {
    //   block [0x83061CE0..0x83061D08)
	// 83061CE0: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 83061CE4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83061CE8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83061CEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83061CF0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83061CF4: 4BF72295  bl 0x82fd3f88
	ctx.lr = 0x83061CF8;
	sub_82FD3F88(ctx, base);
	// 83061CF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83061CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83061D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83061D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83061D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83061D08 size=8
    let mut pc: u32 = 0x83061D08;
    'dispatch: loop {
        match pc {
            0x83061D08 => {
    //   block [0x83061D08..0x83061D10)
	// 83061D08: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83061D0C: 82165F44  lwz r16, 0x5f44(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(24388 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83061D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83061D10 size=864
    let mut pc: u32 = 0x83061D10;
    'dispatch: loop {
        match pc {
            0x83061D10 => {
    //   block [0x83061D10..0x83062070)
	// 83061D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83061D14: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83061D18: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83061D1C: 4814642D  bl 0x831a8148
	ctx.lr = 0x83061D20;
	sub_831A8130(ctx, base);
	// 83061D20: 3BE1FF00  addi r31, r1, -0x100
	ctx.r[31].s64 = ctx.r[1].s64 + -256;
	// 83061D24: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83061D28: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 83061D2C: 90DF012C  stw r6, 0x12c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(300 as u32), ctx.r[6].u32 ) };
	// 83061D30: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 83061D34: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83061D38: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 83061D3C: 389F0056  addi r4, r31, 0x56
	ctx.r[4].s64 = ctx.r[31].s64 + 86;
	// 83061D40: 81770018  lwz r11, 0x18(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24 as u32) ) } as u64;
	// 83061D44: 92970004  stw r20, 4(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(4 as u32), ctx.r[20].u32 ) };
	// 83061D48: 93BF0114  stw r29, 0x114(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), ctx.r[29].u32 ) };
	// 83061D4C: 92DF011C  stw r22, 0x11c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(284 as u32), ctx.r[22].u32 ) };
	// 83061D50: 92FF0124  stw r23, 0x124(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(292 as u32), ctx.r[23].u32 ) };
	// 83061D54: B28B0000  sth r20, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[20].u16 ) };
	// 83061D58: 807D002C  lwz r3, 0x2c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 83061D5C: 4BF8C645  bl 0x82fee3a0
	ctx.lr = 0x83061D60;
	sub_82FEE3A0(ctx, base);
	// 83061D60: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83061D64: 40820010  bne 0x83061d74
	if !ctx.cr[0].eq {
	pc = 0x83061D74; continue 'dispatch;
	}
	// 83061D68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83061D6C: 383F0100  addi r1, r31, 0x100
	ctx.r[1].s64 = ctx.r[31].s64 + 256;
	// 83061D70: 48146428  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
	// 83061D74: 817D002C  lwz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 83061D78: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 83061D7C: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 83061D80: B29F0052  sth r20, 0x52(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(82 as u32), ctx.r[20].u16 ) };
	// 83061D84: 614A8028  ori r10, r10, 0x8028
	ctx.r[10].u64 = ctx.r[10].u64 | 32808;
	// 83061D88: 9A9F0050  stb r20, 0x50(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[20].u8 ) };
	// 83061D8C: 7E9BA378  mr r27, r20
	ctx.r[27].u64 = ctx.r[20].u64;
	// 83061D90: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83061D94: 933F0058  stw r25, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[25].u32 ) };
	// 83061D98: 7F0B502E  lwzx r24, r11, r10
	ctx.r[24].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83061D9C: 931F005C  stw r24, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[24].u32 ) };
	// 83061DA0: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 83061DA4: 3AA00001  li r21, 1
	ctx.r[21].s64 = 1;
	// 83061DA8: 617A8054  ori r26, r11, 0x8054
	ctx.r[26].u64 = ctx.r[11].u64 | 32852;
	// 83061DAC: 807D002C  lwz r3, 0x2c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 83061DB0: 4BF8C361  bl 0x82fee110
	ctx.lr = 0x83061DB4;
	sub_82FEE110(ctx, base);
	// 83061DB4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83061DB8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83061DBC: B39F0054  sth r28, 0x54(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[28].u16 ) };
	// 83061DC0: 5783043F  clrlwi. r3, r28, 0x10
	ctx.r[3].u64 = ctx.r[28].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83061DC4: 40820038  bne 0x83061dfc
	if !ctx.cr[0].eq {
	pc = 0x83061DFC; continue 'dispatch;
	}
	// 83061DC8: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83061DCC: 80FD0004  lwz r7, 4(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83061DD0: 38C0002F  li r6, 0x2f
	ctx.r[6].s64 = 47;
	// 83061DD4: 388B5790  addi r4, r11, 0x5790
	ctx.r[4].s64 = ctx.r[11].s64 + 22416;
	// 83061DD8: 38A003CF  li r5, 0x3cf
	ctx.r[5].s64 = 975;
	// 83061DDC: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83061DE0: 4BF85F01  bl 0x82fe7ce0
	ctx.lr = 0x83061DE4;
	sub_82FE7CE0(ctx, base);
	// 83061DE4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83061DE8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83061DEC: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83061DF0: 388BC700  addi r4, r11, -0x3900
	ctx.r[4].s64 = ctx.r[11].s64 + -14592;
	// 83061DF4: 4814EE35  bl 0x831b0c28
	ctx.lr = 0x83061DF8;
	sub_831B0C28(ctx, base);
	// 83061DF8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83061DFC: A17F0056  lhz r11, 0x56(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(86 as u32) ) } as u64;
	// 83061E00: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83061E04: 409A0044  bne cr6, 0x83061e48
	if !ctx.cr[6].eq {
	pc = 0x83061E48; continue 'dispatch;
	}
	// 83061E08: 817D002C  lwz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 83061E0C: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 83061E10: 614A8028  ori r10, r10, 0x8028
	ctx.r[10].u64 = ctx.r[10].u64 | 32808;
	// 83061E14: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83061E18: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83061E1C: 7F185840  cmplw cr6, r24, r11
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83061E20: 409A000C  bne cr6, 0x83061e2c
	if !ctx.cr[6].eq {
	pc = 0x83061E2C; continue 'dispatch;
	}
	// 83061E24: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83061E28: 4BFFFF44  b 0x83061d6c
	pc = 0x83061D6C; continue 'dispatch;
	// 83061E2C: 4099001C  ble cr6, 0x83061e48
	if !ctx.cr[6].gt {
	pc = 0x83061E48; continue 'dispatch;
	}
	// 83061E30: 388000E7  li r4, 0xe7
	ctx.r[4].s64 = 231;
	// 83061E34: 807D0030  lwz r3, 0x30(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 83061E38: 4BF86A51  bl 0x82fe8888
	ctx.lr = 0x83061E3C;
	sub_82FE8888(ctx, base);
	// 83061E3C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83061E40: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83061E44: 4BFFFF28  b 0x83061d6c
	pc = 0x83061D6C; continue 'dispatch;
	// 83061E48: 9A9F0051  stb r20, 0x51(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(81 as u32), ctx.r[20].u8 ) };
	// 83061E4C: 2B030026  cmplwi cr6, r3, 0x26
	ctx.cr[6].compare_u32(ctx.r[3].u32, 38 as u32, &mut ctx.xer);
	// 83061E50: 409A002C  bne cr6, 0x83061e7c
	if !ctx.cr[6].eq {
	pc = 0x83061E7C; continue 'dispatch;
	}
	// 83061E54: 38DF0051  addi r6, r31, 0x51
	ctx.r[6].s64 = ctx.r[31].s64 + 81;
	// 83061E58: 38BF0052  addi r5, r31, 0x52
	ctx.r[5].s64 = ctx.r[31].s64 + 82;
	// 83061E5C: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 83061E60: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83061E64: 4BFFF59D  bl 0x83061400
	ctx.lr = 0x83061E68;
	sub_83061400(ctx, base);
	// 83061E68: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83061E6C: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83061E70: 419A00C0  beq cr6, 0x83061f30
	if ctx.cr[6].eq {
	pc = 0x83061F30; continue 'dispatch;
	}
	// 83061E74: 7E9BA378  mr r27, r20
	ctx.r[27].u64 = ctx.r[20].u64;
	// 83061E78: 4BFFFF34  b 0x83061dac
	pc = 0x83061DAC; continue 'dispatch;
	// 83061E7C: 2B03D800  cmplwi cr6, r3, 0xd800
	ctx.cr[6].compare_u32(ctx.r[3].u32, 55296 as u32, &mut ctx.xer);
	// 83061E80: 41980030  blt cr6, 0x83061eb0
	if ctx.cr[6].lt {
	pc = 0x83061EB0; continue 'dispatch;
	}
	// 83061E84: 2B03DBFF  cmplwi cr6, r3, 0xdbff
	ctx.cr[6].compare_u32(ctx.r[3].u32, 56319 as u32, &mut ctx.xer);
	// 83061E88: 41990028  bgt cr6, 0x83061eb0
	if ctx.cr[6].gt {
	pc = 0x83061EB0; continue 'dispatch;
	}
	// 83061E8C: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83061E90: 41820018  beq 0x83061ea8
	if ctx.cr[0].eq {
	pc = 0x83061EA8; continue 'dispatch;
	}
	// 83061E94: 388000F9  li r4, 0xf9
	ctx.r[4].s64 = 249;
	// 83061E98: 807D0030  lwz r3, 0x30(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 83061E9C: 4BF869ED  bl 0x82fe8888
	ctx.lr = 0x83061EA0;
	sub_82FE8888(ctx, base);
	// 83061EA0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83061EA4: 48000090  b 0x83061f34
	pc = 0x83061F34; continue 'dispatch;
	// 83061EA8: 7EBBAB78  mr r27, r21
	ctx.r[27].u64 = ctx.r[21].u64;
	// 83061EAC: 48000088  b 0x83061f34
	pc = 0x83061F34; continue 'dispatch;
	// 83061EB0: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83061EB4: 41820024  beq 0x83061ed8
	if ctx.cr[0].eq {
	pc = 0x83061ED8; continue 'dispatch;
	}
	// 83061EB8: 2B03DC00  cmplwi cr6, r3, 0xdc00
	ctx.cr[6].compare_u32(ctx.r[3].u32, 56320 as u32, &mut ctx.xer);
	// 83061EBC: 4198000C  blt cr6, 0x83061ec8
	if ctx.cr[6].lt {
	pc = 0x83061EC8; continue 'dispatch;
	}
	// 83061EC0: 2B03DFFF  cmplwi cr6, r3, 0xdfff
	ctx.cr[6].compare_u32(ctx.r[3].u32, 57343 as u32, &mut ctx.xer);
	// 83061EC4: 40990064  ble cr6, 0x83061f28
	if !ctx.cr[6].gt {
	pc = 0x83061F28; continue 'dispatch;
	}
	// 83061EC8: 388000F9  li r4, 0xf9
	ctx.r[4].s64 = 249;
	// 83061ECC: 807D0030  lwz r3, 0x30(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 83061ED0: 4BF869B9  bl 0x82fe8888
	ctx.lr = 0x83061ED4;
	sub_82FE8888(ctx, base);
	// 83061ED4: 48000050  b 0x83061f24
	pc = 0x83061F24; continue 'dispatch;
	// 83061ED8: 817D002C  lwz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 83061EDC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83061EE0: 7D6BD02E  lwzx r11, r11, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 83061EE4: 7D6B18AE  lbzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 83061EE8: 556B0673  rlwinm. r11, r11, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83061EEC: 4082003C  bne 0x83061f28
	if !ctx.cr[0].eq {
	pc = 0x83061F28; continue 'dispatch;
	}
	// 83061EF0: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83061EF4: 80FD0004  lwz r7, 4(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83061EF8: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 83061EFC: 389F0060  addi r4, r31, 0x60
	ctx.r[4].s64 = ctx.r[31].s64 + 96;
	// 83061F00: 4BF6F971  bl 0x82fd1870
	ctx.lr = 0x83061F04;
	sub_82FD1870(ctx, base);
	// 83061F04: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83061F08: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83061F0C: 807D0030  lwz r3, 0x30(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 83061F10: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83061F14: 38DF0060  addi r6, r31, 0x60
	ctx.r[6].s64 = ctx.r[31].s64 + 96;
	// 83061F18: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 83061F1C: 388000DA  li r4, 0xda
	ctx.r[4].s64 = 218;
	// 83061F20: 4BF86AB9  bl 0x82fe89d8
	ctx.lr = 0x83061F24;
	sub_82FE89D8(ctx, base);
	// 83061F24: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83061F28: 7E9BA378  mr r27, r20
	ctx.r[27].u64 = ctx.r[20].u64;
	// 83061F2C: 48000008  b 0x83061f34
	pc = 0x83061F34; continue 'dispatch;
	// 83061F30: A39F0054  lhz r28, 0x54(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83061F34: 8BDF0051  lbz r30, 0x51(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(81 as u32) ) } as u64;
	// 83061F38: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83061F3C: 40820030  bne 0x83061f6c
	if !ctx.cr[0].eq {
	pc = 0x83061F6C; continue 'dispatch;
	}
	// 83061F40: 578B043E  clrlwi r11, r28, 0x10
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x0000FFFFu64;
	// 83061F44: 2B0B003C  cmplwi cr6, r11, 0x3c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 60 as u32, &mut ctx.xer);
	// 83061F48: 409A0024  bne cr6, 0x83061f6c
	if !ctx.cr[6].eq {
	pc = 0x83061F6C; continue 'dispatch;
	}
	// 83061F4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83061F50: 807D0030  lwz r3, 0x30(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 83061F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83061F58: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83061F5C: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 83061F60: 388000F8  li r4, 0xf8
	ctx.r[4].s64 = 248;
	// 83061F64: 4BF86A75  bl 0x82fe89d8
	ctx.lr = 0x83061F68;
	sub_82FE89D8(ctx, base);
	// 83061F68: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83061F6C: 817F012C  lwz r11, 0x12c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(300 as u32) ) } as u64;
	// 83061F70: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83061F74: 409A0030  bne cr6, 0x83061fa4
	if !ctx.cr[6].eq {
	pc = 0x83061FA4; continue 'dispatch;
	}
	// 83061F78: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83061F7C: 409A0074  bne cr6, 0x83061ff0
	if !ctx.cr[6].eq {
	pc = 0x83061FF0; continue 'dispatch;
	}
	// 83061F80: 578B043E  clrlwi r11, r28, 0x10
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x0000FFFFu64;
	// 83061F84: 2B0B0009  cmplwi cr6, r11, 9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 9 as u32, &mut ctx.xer);
	// 83061F88: 419A0014  beq cr6, 0x83061f9c
	if ctx.cr[6].eq {
	pc = 0x83061F9C; continue 'dispatch;
	}
	// 83061F8C: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 83061F90: 419A000C  beq cr6, 0x83061f9c
	if ctx.cr[6].eq {
	pc = 0x83061F9C; continue 'dispatch;
	}
	// 83061F94: 2B0B000D  cmplwi cr6, r11, 0xd
	ctx.cr[6].compare_u32(ctx.r[11].u32, 13 as u32, &mut ctx.xer);
	// 83061F98: 409A0058  bne cr6, 0x83061ff0
	if !ctx.cr[6].eq {
	pc = 0x83061FF0; continue 'dispatch;
	}
	// 83061F9C: 3B800020  li r28, 0x20
	ctx.r[28].s64 = 32;
	// 83061FA0: 48000050  b 0x83061ff0
	pc = 0x83061FF0; continue 'dispatch;
	// 83061FA4: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 83061FA8: 409A0078  bne cr6, 0x83062020
	if !ctx.cr[6].eq {
	pc = 0x83062020; continue 'dispatch;
	}
	// 83061FAC: 817D002C  lwz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 83061FB0: 578A043E  clrlwi r10, r28, 0x10
	ctx.r[10].u64 = ctx.r[28].u32 as u64 & 0x0000FFFFu64;
	// 83061FB4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83061FB8: 7D6BD02E  lwzx r11, r11, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 83061FBC: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83061FC0: 556B0031  rlwinm. r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83061FC4: 4082FDE8  bne 0x83061dac
	if !ctx.cr[0].eq {
	pc = 0x83061DAC; continue 'dispatch;
	}
	// 83061FC8: 897F0050  lbz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83061FCC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83061FD0: 41820014  beq 0x83061fe4
	if ctx.cr[0].eq {
	pc = 0x83061FE4; continue 'dispatch;
	}
	// 83061FD4: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 83061FD8: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 83061FDC: 4BF6EB3D  bl 0x82fd0b18
	ctx.lr = 0x83061FE0;
	sub_82FD0B18(ctx, base);
	// 83061FE0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83061FE4: 7EB9AB78  mr r25, r21
	ctx.r[25].u64 = ctx.r[21].u64;
	// 83061FE8: 933F0058  stw r25, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[25].u32 ) };
	// 83061FEC: 9ABF0050  stb r21, 0x50(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[21].u8 ) };
	// 83061FF0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83061FF4: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 83061FF8: 4BF6EB21  bl 0x82fd0b18
	ctx.lr = 0x83061FFC;
	sub_82FD0B18(ctx, base);
	// 83061FFC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83062000: A09F0052  lhz r4, 0x52(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(82 as u32) ) } as u64;
	// 83062004: 7C8B2379  or. r11, r4, r4
	ctx.r[11].u64 = ctx.r[4].u64 | ctx.r[4].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83062008: 4182FDA4  beq 0x83061dac
	if ctx.cr[0].eq {
	pc = 0x83061DAC; continue 'dispatch;
	}
	// 8306200C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 83062010: 4BF6EB09  bl 0x82fd0b18
	ctx.lr = 0x83062014;
	sub_82FD0B18(ctx, base);
	// 83062014: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83062018: B29F0052  sth r20, 0x52(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(82 as u32), ctx.r[20].u16 ) };
	// 8306201C: 4BFFFD90  b 0x83061dac
	pc = 0x83061DAC; continue 'dispatch;
	// 83062020: 2F190001  cmpwi cr6, r25, 1
	ctx.cr[6].compare_i32(ctx.r[25].s32, 1, &mut ctx.xer);
	// 83062024: 409AFFCC  bne cr6, 0x83061ff0
	if !ctx.cr[6].eq {
	pc = 0x83061FF0; continue 'dispatch;
	}
	// 83062028: 817D002C  lwz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8306202C: 578A043E  clrlwi r10, r28, 0x10
	ctx.r[10].u64 = ctx.r[28].u32 as u64 & 0x0000FFFFu64;
	// 83062030: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83062034: 7D6BD02E  lwzx r11, r11, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 83062038: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8306203C: 556B0031  rlwinm. r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83062040: 4182FFAC  beq 0x83061fec
	if ctx.cr[0].eq {
	pc = 0x83061FEC; continue 'dispatch;
	}
	// 83062044: 7E99A378  mr r25, r20
	ctx.r[25].u64 = ctx.r[20].u64;
	// 83062048: 933F0058  stw r25, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[25].u32 ) };
	// 8306204C: 4BFFFD60  b 0x83061dac
	pc = 0x83061DAC; continue 'dispatch;
	// 83062050: 83BF0114  lwz r29, 0x114(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 83062054: 82FF0124  lwz r23, 0x124(r31)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(292 as u32) ) } as u64;
	// 83062058: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 8306205C: 82DF011C  lwz r22, 0x11c(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(284 as u32) ) } as u64;
	// 83062060: 831F005C  lwz r24, 0x5c(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83062064: 833F0058  lwz r25, 0x58(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83062068: 8B7F0051  lbz r27, 0x51(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(81 as u32) ) } as u64;
	// 8306206C: 4BFFFD34  b 0x83061da0
	pc = 0x83061DA0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83062070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83062070 size=8
    let mut pc: u32 = 0x83062070;
    'dispatch: loop {
        match pc {
            0x83062070 => {
    //   block [0x83062070..0x83062078)
	// 83062070: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83062074: 82165F44  lwz r16, 0x5f44(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(24388 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83062078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83062078 size=32
    let mut pc: u32 = 0x83062078;
    'dispatch: loop {
        match pc {
            0x83062078 => {
    //   block [0x83062078..0x83062098)
	// 83062078: 3BECFF00  addi r31, r12, -0x100
	ctx.r[31].s64 = ctx.r[12].s64 + -256;
	// 8306207C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83062080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83062084: 3C608306  lis r3, -0x7cfa
	ctx.r[3].s64 = -2096758784;
	// 83062088: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8306208C: 38632050  addi r3, r3, 0x2050
	ctx.r[3].s64 = ctx.r[3].s64 + 8272;
	// 83062090: 997F0051  stb r11, 0x51(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(81 as u32), ctx.r[11].u8 ) };
	// 83062094: 48146104  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83062098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83062098 size=8
    let mut pc: u32 = 0x83062098;
    'dispatch: loop {
        match pc {
            0x83062098 => {
    //   block [0x83062098..0x830620A0)
	// 83062098: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8306209C: 82165F88  lwz r16, 0x5f88(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(24456 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830620A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830620A0 size=344
    let mut pc: u32 = 0x830620A0;
    'dispatch: loop {
        match pc {
            0x830620A0 => {
    //   block [0x830620A0..0x830621F8)
	// 830620A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830620A4: 481460B9  bl 0x831a815c
	ctx.lr = 0x830620A8;
	sub_831A8130(ctx, base);
	// 830620A8: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 830620AC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830620B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830620B4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 830620B8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 830620BC: 388B7C84  addi r4, r11, 0x7c84
	ctx.r[4].s64 = ctx.r[11].s64 + 31876;
	// 830620C0: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 830620C4: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830620C8: 4BF91409  bl 0x82ff34d0
	ctx.lr = 0x830620CC;
	sub_82FF34D0(ctx, base);
	// 830620CC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830620D0: 41820010  beq 0x830620e0
	if ctx.cr[0].eq {
	pc = 0x830620E0; continue 'dispatch;
	}
	// 830620D4: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 830620D8: 917B0004  stw r11, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830620DC: 48000114  b 0x830621f0
	pc = 0x830621F0; continue 'dispatch;
	// 830620E0: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 830620E4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 830620E8: 388A7AE8  addi r4, r10, 0x7ae8
	ctx.r[4].s64 = ctx.r[10].s64 + 31464;
	// 830620EC: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830620F0: 4BF913E1  bl 0x82ff34d0
	ctx.lr = 0x830620F4;
	sub_82FF34D0(ctx, base);
	// 830620F4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830620F8: 4182000C  beq 0x83062104
	if ctx.cr[0].eq {
	pc = 0x83062104; continue 'dispatch;
	}
	// 830620FC: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83062100: 4BFFFFD8  b 0x830620d8
	pc = 0x830620D8; continue 'dispatch;
	// 83062104: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83062108: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8306210C: 388A7A10  addi r4, r10, 0x7a10
	ctx.r[4].s64 = ctx.r[10].s64 + 31248;
	// 83062110: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83062114: 4BF913BD  bl 0x82ff34d0
	ctx.lr = 0x83062118;
	sub_82FF34D0(ctx, base);
	// 83062118: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8306211C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83062120: 4182003C  beq 0x8306215c
	if ctx.cr[0].eq {
	pc = 0x8306215C; continue 'dispatch;
	}
	// 83062124: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83062128: 4BF8C209  bl 0x82fee330
	ctx.lr = 0x8306212C;
	sub_82FEE330(ctx, base);
	// 8306212C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83062130: 40820014  bne 0x83062144
	if !ctx.cr[0].eq {
	pc = 0x83062144; continue 'dispatch;
	}
	// 83062134: 388000CF  li r4, 0xcf
	ctx.r[4].s64 = 207;
	// 83062138: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8306213C: 4BF8674D  bl 0x82fe8888
	ctx.lr = 0x83062140;
	sub_82FE8888(ctx, base);
	// 83062140: 48000010  b 0x83062150
	pc = 0x83062150; continue 'dispatch;
	// 83062144: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83062148: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 8306214C: 4BF8C2D5  bl 0x82fee420
	ctx.lr = 0x83062150;
	sub_82FEE420(ctx, base);
	// 83062150: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83062154: 917B0004  stw r11, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83062158: 48000008  b 0x83062160
	pc = 0x83062160; continue 'dispatch;
	// 8306215C: 933B0004  stw r25, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 83062160: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83062164: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83062168: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8306216C: 480020DD  bl 0x83064248
	ctx.lr = 0x83062170;
	sub_83064248(ctx, base);
	// 83062170: 839E0028  lwz r28, 0x28(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83062174: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83062178: 4BF7CE81  bl 0x82fdeff8
	ctx.lr = 0x8306217C;
	sub_82FDEFF8(ctx, base);
	// 8306217C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83062180: 939F0054  stw r28, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 83062184: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 83062188: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8306218C: 835B0008  lwz r26, 8(r27)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 83062190: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83062194: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83062198: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8306219C: 4E800421  bctrl
	ctx.lr = 0x830621A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830621A0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830621A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830621A8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830621AC: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 830621B0: 4BFFFB61  bl 0x83061d10
	ctx.lr = 0x830621B4;
	sub_83061D10(ctx, base);
	// 830621B4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830621B8: 40820010  bne 0x830621c8
	if !ctx.cr[0].eq {
	pc = 0x830621C8; continue 'dispatch;
	}
	// 830621BC: 388000B2  li r4, 0xb2
	ctx.r[4].s64 = 178;
	// 830621C0: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830621C4: 4BF866C5  bl 0x82fe8888
	ctx.lr = 0x830621C8;
	sub_82FE8888(ctx, base);
	// 830621C8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830621CC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830621D0: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 830621D4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830621D8: 7F2B532E  sthx r25, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[25].u16) };
	// 830621DC: 809D0018  lwz r4, 0x18(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 830621E0: 4BFFC071  bl 0x8305e250
	ctx.lr = 0x830621E4;
	sub_8305E250(ctx, base);
	// 830621E4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830621E8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830621EC: 4BF7CF3D  bl 0x82fdf128
	ctx.lr = 0x830621F0;
	sub_82FDF128(ctx, base);
	// 830621F0: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 830621F4: 48145FB8  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830621F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830621F8 size=40
    let mut pc: u32 = 0x830621F8;
    'dispatch: loop {
        match pc {
            0x830621F8 => {
    //   block [0x830621F8..0x83062220)
	// 830621F8: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 830621FC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83062200: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83062204: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83062208: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8306220C: 4BF71D7D  bl 0x82fd3f88
	ctx.lr = 0x83062210;
	sub_82FD3F88(ctx, base);
	// 83062210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83062214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83062218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8306221C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83062220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83062220 size=8
    let mut pc: u32 = 0x83062220;
    'dispatch: loop {
        match pc {
            0x83062220 => {
    //   block [0x83062220..0x83062228)
	// 83062220: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83062224: 82165FE0  lwz r16, 0x5fe0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(24544 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83062228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83062228 size=924
    let mut pc: u32 = 0x83062228;
    'dispatch: loop {
        match pc {
            0x83062228 => {
    //   block [0x83062228..0x830625C4)
	// 83062228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306222C: 48145F25  bl 0x831a8150
	ctx.lr = 0x83062230;
	sub_831A8130(ctx, base);
	// 83062230: 3BE1FF40  addi r31, r1, -0xc0
	ctx.r[31].s64 = ctx.r[1].s64 + -192;
	// 83062234: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83062238: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8306223C: 83BE002C  lwz r29, 0x2c(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83062240: 93DF00D4  stw r30, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[30].u32 ) };
	// 83062244: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83062248: 4BF8C001  bl 0x82fee248
	ctx.lr = 0x8306224C;
	sub_82FEE248(ctx, base);
	// 8306224C: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 83062250: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83062254: 614A8054  ori r10, r10, 0x8054
	ctx.r[10].u64 = ctx.r[10].u64 | 32852;
	// 83062258: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8306225C: 546A043E  clrlwi r10, r3, 0x10
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 83062260: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83062264: 556B0031  rlwinm. r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83062268: 40820014  bne 0x8306227c
	if !ctx.cr[0].eq {
	pc = 0x8306227C; continue 'dispatch;
	}
	// 8306226C: 388000CF  li r4, 0xcf
	ctx.r[4].s64 = 207;
	// 83062270: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83062274: 4BF86615  bl 0x82fe8888
	ctx.lr = 0x83062278;
	sub_82FE8888(ctx, base);
	// 83062278: 48000010  b 0x83062288
	pc = 0x83062288; continue 'dispatch;
	// 8306227C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83062280: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83062284: 4BF8C19D  bl 0x82fee420
	ctx.lr = 0x83062288;
	sub_82FEE420(ctx, base);
	// 83062288: 38800025  li r4, 0x25
	ctx.r[4].s64 = 37;
	// 8306228C: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83062290: 4BF8C021  bl 0x82fee2b0
	ctx.lr = 0x83062294;
	sub_82FEE2B0(ctx, base);
	// 83062294: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 83062298: 56D8063F  clrlwi. r24, r22, 0x18
	ctx.r[24].u64 = ctx.r[22].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 8306229C: 41820028  beq 0x830622c4
	if ctx.cr[0].eq {
	pc = 0x830622C4; continue 'dispatch;
	}
	// 830622A0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830622A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830622A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830622AC: 48001F9D  bl 0x83064248
	ctx.lr = 0x830622B0;
	sub_83064248(ctx, base);
	// 830622B0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830622B4: 40820010  bne 0x830622c4
	if !ctx.cr[0].eq {
	pc = 0x830622C4; continue 'dispatch;
	}
	// 830622B8: 388000CF  li r4, 0xcf
	ctx.r[4].s64 = 207;
	// 830622BC: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830622C0: 4BF865C9  bl 0x82fe8888
	ctx.lr = 0x830622C4;
	sub_82FE8888(ctx, base);
	// 830622C4: 82FE0028  lwz r23, 0x28(r30)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830622C8: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 830622CC: 4BF7CD2D  bl 0x82fdeff8
	ctx.lr = 0x830622D0;
	sub_82FDEFF8(ctx, base);
	// 830622D0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830622D4: 92FF005C  stw r23, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[23].u32 ) };
	// 830622D8: 939F0058  stw r28, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 830622DC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 830622E0: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 830622E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830622E8: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 830622EC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830622F0: 937C0004  stw r27, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 830622F4: B36A0000  sth r27, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[27].u16 ) };
	// 830622F8: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830622FC: 4BF90E0D  bl 0x82ff3108
	ctx.lr = 0x83062300;
	sub_82FF3108(ctx, base);
	// 83062300: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83062304: 40820020  bne 0x83062324
	if !ctx.cr[0].eq {
	pc = 0x83062324; continue 'dispatch;
	}
	// 83062308: 388000DE  li r4, 0xde
	ctx.r[4].s64 = 222;
	// 8306230C: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83062310: 4BF86579  bl 0x82fe8888
	ctx.lr = 0x83062314;
	sub_82FE8888(ctx, base);
	// 83062314: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 83062318: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 8306231C: 4BFB54F5  bl 0x83017810
	ctx.lr = 0x83062320;
	sub_83017810(ctx, base);
	// 83062320: 48000290  b 0x830625b0
	pc = 0x830625B0; continue 'dispatch;
	// 83062324: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83062328: 896B000A  lbz r11, 0xa(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 8306232C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83062330: 41820034  beq 0x83062364
	if ctx.cr[0].eq {
	pc = 0x83062364; continue 'dispatch;
	}
	// 83062334: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83062338: 3880003A  li r4, 0x3a
	ctx.r[4].s64 = 58;
	// 8306233C: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83062340: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83062344: 7F6B532E  sthx r27, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u16) };
	// 83062348: 807C0018  lwz r3, 0x18(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 8306234C: 4BF6FA65  bl 0x82fd1db0
	ctx.lr = 0x83062350;
	sub_82FD1DB0(ctx, base);
	// 83062350: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 83062354: 419A0010  beq cr6, 0x83062364
	if ctx.cr[6].eq {
	pc = 0x83062364; continue 'dispatch;
	}
	// 83062358: 38800122  li r4, 0x122
	ctx.r[4].s64 = 290;
	// 8306235C: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83062360: 4BF86529  bl 0x82fe8888
	ctx.lr = 0x83062364;
	sub_82FE8888(ctx, base);
	// 83062364: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83062368: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 8306236C: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83062370: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83062374: 7F6B532E  sthx r27, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u16) };
	// 83062378: 809C0018  lwz r4, 0x18(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 8306237C: 419A0020  beq cr6, 0x8306239c
	if ctx.cr[6].eq {
	pc = 0x8306239C; continue 'dispatch;
	}
	// 83062380: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 83062384: 807E0034  lwz r3, 0x34(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83062388: 4BFBCE61  bl 0x8301f1e8
	ctx.lr = 0x8306238C;
	sub_8301F1E8(ctx, base);
	// 8306238C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83062390: 41820098  beq 0x83062428
	if ctx.cr[0].eq {
	pc = 0x83062428; continue 'dispatch;
	}
	// 83062394: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83062398: 4800000C  b 0x830623a4
	pc = 0x830623A4; continue 'dispatch;
	// 8306239C: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830623A0: 4BFBDB91  bl 0x8301ff30
	ctx.lr = 0x830623A4;
	sub_8301FF30(ctx, base);
	// 830623A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830623A8: 419A0080  beq cr6, 0x83062428
	if ctx.cr[6].eq {
	pc = 0x83062428; continue 'dispatch;
	}
	// 830623AC: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 830623B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830623B4: 409A0050  bne cr6, 0x83062404
	if !ctx.cr[6].eq {
	pc = 0x83062404; continue 'dispatch;
	}
	// 830623B8: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 830623BC: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830623C0: 4BF75ED9  bl 0x82fd8298
	ctx.lr = 0x830623C4;
	sub_82FD8298(ctx, base);
	// 830623C4: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830623C8: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 830623CC: 41820030  beq 0x830623fc
	if ctx.cr[0].eq {
	pc = 0x830623FC; continue 'dispatch;
	}
	// 830623D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830623D4: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830623D8: 4BFF1439  bl 0x83053810
	ctx.lr = 0x830623DC;
	sub_83053810(ctx, base);
	// 830623DC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830623E0: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 830623E4: 9B7D0028  stb r27, 0x28(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(40 as u32), ctx.r[27].u8 ) };
	// 830623E8: 396B3FF0  addi r11, r11, 0x3ff0
	ctx.r[11].s64 = ctx.r[11].s64 + 16368;
	// 830623EC: 9B7D0029  stb r27, 0x29(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(41 as u32), ctx.r[27].u8 ) };
	// 830623F0: 9B7D002A  stb r27, 0x2a(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(42 as u32), ctx.r[27].u8 ) };
	// 830623F4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830623F8: 48000008  b 0x83062400
	pc = 0x83062400; continue 'dispatch;
	// 830623FC: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 83062400: 915E0018  stw r10, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 83062404: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83062408: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 8306240C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83062410: 7F6B532E  sthx r27, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u16) };
	// 83062414: 809C0018  lwz r4, 0x18(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83062418: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8306241C: 4BFF14DD  bl 0x830538f8
	ctx.lr = 0x83062420;
	sub_830538F8(ctx, base);
	// 83062420: 83BE0018  lwz r29, 0x18(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83062424: 48000070  b 0x83062494
	pc = 0x83062494; continue 'dispatch;
	// 83062428: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 8306242C: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83062430: 4BF75E69  bl 0x82fd8298
	ctx.lr = 0x83062434;
	sub_82FD8298(ctx, base);
	// 83062434: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83062438: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8306243C: 4182002C  beq 0x83062468
	if ctx.cr[0].eq {
	pc = 0x83062468; continue 'dispatch;
	}
	// 83062440: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83062444: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83062448: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 8306244C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83062450: 7F6B532E  sthx r27, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u16) };
	// 83062454: 809C0018  lwz r4, 0x18(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83062458: 80DE0008  lwz r6, 8(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8306245C: 4BFB5565  bl 0x830179c0
	ctx.lr = 0x83062460;
	sub_830179C0(ctx, base);
	// 83062460: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83062464: 48000008  b 0x8306246c
	pc = 0x8306246C; continue 'dispatch;
	// 83062468: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8306246C: 897E001C  lbz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83062470: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83062474: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 83062478: 99640028  stb r11, 0x28(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 8306247C: 419A000C  beq cr6, 0x83062488
	if ctx.cr[6].eq {
	pc = 0x83062488; continue 'dispatch;
	}
	// 83062480: 807E0034  lwz r3, 0x34(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83062484: 4800000C  b 0x83062490
	pc = 0x83062490; continue 'dispatch;
	// 83062488: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 8306248C: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83062490: 4BFEA7F9  bl 0x8304cc88
	ctx.lr = 0x83062494;
	sub_8304CC88(ctx, base);
	// 83062494: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83062498: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8306249C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830624A0: 9ADD0029  stb r22, 0x29(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(41 as u32), ctx.r[22].u8 ) };
	// 830624A4: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 830624A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830624AC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830624B0: 5579DFFE  rlwinm r25, r11, 0x1b, 0x1f, 0x1f
	ctx.r[25].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830624B4: 48001D95  bl 0x83064248
	ctx.lr = 0x830624B8;
	sub_83064248(ctx, base);
	// 830624B8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830624BC: 40820010  bne 0x830624cc
	if !ctx.cr[0].eq {
	pc = 0x830624CC; continue 'dispatch;
	}
	// 830624C0: 388000CF  li r4, 0xcf
	ctx.r[4].s64 = 207;
	// 830624C4: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830624C8: 4BF863C1  bl 0x82fe8888
	ctx.lr = 0x830624CC;
	sub_82FE8888(ctx, base);
	// 830624CC: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830624D0: 8B4B000F  lbz r26, 0xf(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(15 as u32) ) } as u64;
	// 830624D4: 281A0000  cmplwi r26, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830624D8: 41820010  beq 0x830624e8
	if ctx.cr[0].eq {
	pc = 0x830624E8; continue 'dispatch;
	}
	// 830624DC: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 830624E0: 419A0008  beq cr6, 0x830624e8
	if ctx.cr[6].eq {
	pc = 0x830624E8; continue 'dispatch;
	}
	// 830624E4: 9B6B000F  stb r27, 0xf(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(15 as u32), ctx.r[27].u8 ) };
	// 830624E8: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 830624EC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830624F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830624F4: 4BFFF3FD  bl 0x830618f0
	ctx.lr = 0x830624F8;
	sub_830618F0(ctx, base);
	// 830624F8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830624FC: 4082002C  bne 0x83062528
	if !ctx.cr[0].eq {
	pc = 0x83062528; continue 'dispatch;
	}
	// 83062500: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 83062504: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83062508: 4BFB5309  bl 0x83017810
	ctx.lr = 0x8306250C;
	sub_83017810(ctx, base);
	// 8306250C: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83062510: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83062514: 38800110  li r4, 0x110
	ctx.r[4].s64 = 272;
	// 83062518: 994B000F  stb r10, 0xf(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(15 as u32), ctx.r[10].u8 ) };
	// 8306251C: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83062520: 4BF86369  bl 0x82fe8888
	ctx.lr = 0x83062524;
	sub_82FE8888(ctx, base);
	// 83062524: 4800008C  b 0x830625b0
	pc = 0x830625B0; continue 'dispatch;
	// 83062528: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8306252C: 419A0010  beq cr6, 0x8306253c
	if ctx.cr[6].eq {
	pc = 0x8306253C; continue 'dispatch;
	}
	// 83062530: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83062534: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83062538: 994B000F  stb r10, 0xf(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(15 as u32), ctx.r[10].u8 ) };
	// 8306253C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83062540: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83062544: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83062548: 48001D01  bl 0x83064248
	ctx.lr = 0x8306254C;
	sub_83064248(ctx, base);
	// 8306254C: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 83062550: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83062554: 4BF8BD5D  bl 0x82fee2b0
	ctx.lr = 0x83062558;
	sub_82FEE2B0(ctx, base);
	// 83062558: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8306255C: 4082002C  bne 0x83062588
	if !ctx.cr[0].eq {
	pc = 0x83062588; continue 'dispatch;
	}
	// 83062560: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83062564: 80BD0010  lwz r5, 0x10(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 83062568: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8306256C: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83062570: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83062574: 388000DF  li r4, 0xdf
	ctx.r[4].s64 = 223;
	// 83062578: 4BF86461  bl 0x82fe89d8
	ctx.lr = 0x8306257C;
	sub_82FE89D8(ctx, base);
	// 8306257C: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 83062580: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83062584: 4BFB528D  bl 0x83017810
	ctx.lr = 0x83062588;
	sub_83017810(ctx, base);
	// 83062588: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8306258C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83062590: 41820020  beq 0x830625b0
	if ctx.cr[0].eq {
	pc = 0x830625B0; continue 'dispatch;
	}
	// 83062594: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83062598: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 8306259C: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 830625A0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830625A4: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 830625A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830625AC: 4E800421  bctrl
	ctx.lr = 0x830625B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830625B0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830625B4: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 830625B8: 4BF7CB71  bl 0x82fdf128
	ctx.lr = 0x830625BC;
	sub_82FDF128(ctx, base);
	// 830625BC: 383F00C0  addi r1, r31, 0xc0
	ctx.r[1].s64 = ctx.r[31].s64 + 192;
	// 830625C0: 48145BE0  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830625C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830625C4 size=40
    let mut pc: u32 = 0x830625C4;
    'dispatch: loop {
        match pc {
            0x830625C4 => {
    //   block [0x830625C4..0x830625EC)
	// 830625C4: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 830625C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830625CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830625D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830625D4: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 830625D8: 4BF719B1  bl 0x82fd3f88
	ctx.lr = 0x830625DC;
	sub_82FD3F88(ctx, base);
	// 830625DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830625E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830625E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830625E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830625EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830625EC size=48
    let mut pc: u32 = 0x830625EC;
    'dispatch: loop {
        match pc {
            0x830625EC => {
    //   block [0x830625EC..0x8306261C)
	// 830625EC: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 830625F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830625F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830625F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830625FC: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 83062600: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83062604: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83062608: 4BF75CD9  bl 0x82fd82e0
	ctx.lr = 0x8306260C;
	sub_82FD82E0(ctx, base);
	// 8306260C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83062610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83062614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83062618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8306261C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8306261C size=48
    let mut pc: u32 = 0x8306261C;
    'dispatch: loop {
        match pc {
            0x8306261C => {
    //   block [0x8306261C..0x8306264C)
	// 8306261C: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 83062620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83062624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83062628: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8306262C: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 83062630: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83062634: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83062638: 4BF75CA9  bl 0x82fd82e0
	ctx.lr = 0x8306263C;
	sub_82FD82E0(ctx, base);
	// 8306263C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83062640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83062644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83062648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83062650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83062650 size=8
    let mut pc: u32 = 0x83062650;
    'dispatch: loop {
        match pc {
            0x83062650 => {
    //   block [0x83062650..0x83062658)
	// 83062650: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83062654: 82166070  lwz r16, 0x6070(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(24688 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83062658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83062658 size=1720
    let mut pc: u32 = 0x83062658;
    'dispatch: loop {
        match pc {
            0x83062658 => {
    //   block [0x83062658..0x83062D10)
	// 83062658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306265C: 48145AED  bl 0x831a8148
	ctx.lr = 0x83062660;
	sub_831A8130(ctx, base);
	// 83062660: 3BE1FF10  addi r31, r1, -0xf0
	ctx.r[31].s64 = ctx.r[1].s64 + -240;
	// 83062664: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83062668: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8306266C: 7C952378  mr r21, r4
	ctx.r[21].u64 = ctx.r[4].u64;
	// 83062670: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83062674: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83062678: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8306267C: 93DF0104  stw r30, 0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[30].u32 ) };
	// 83062680: 48001BC9  bl 0x83064248
	ctx.lr = 0x83062684;
	sub_83064248(ctx, base);
	// 83062684: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 83062688: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8306268C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83062690: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83062694: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83062698: 92FD0004  stw r23, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[23].u32 ) };
	// 8306269C: B2EA0000  sth r23, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[23].u16 ) };
	// 830626A0: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830626A4: 4BF90A65  bl 0x82ff3108
	ctx.lr = 0x830626A8;
	sub_82FF3108(ctx, base);
	// 830626A8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830626AC: 40820014  bne 0x830626c0
	if !ctx.cr[0].eq {
	pc = 0x830626C0; continue 'dispatch;
	}
	// 830626B0: 388000AE  li r4, 0xae
	ctx.r[4].s64 = 174;
	// 830626B4: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830626B8: 4BF861D1  bl 0x82fe8888
	ctx.lr = 0x830626BC;
	sub_82FE8888(ctx, base);
	// 830626BC: 48000648  b 0x83062d04
	pc = 0x83062D04; continue 'dispatch;
	// 830626C0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830626C4: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 830626C8: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 830626CC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830626D0: 7EEB532E  sthx r23, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[23].u16) };
	// 830626D4: 809D0018  lwz r4, 0x18(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 830626D8: 480029B9  bl 0x83065090
	ctx.lr = 0x830626DC;
	sub_83065090(ctx, base);
	// 830626DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830626E0: 418200A4  beq 0x83062784
	if ctx.cr[0].eq {
	pc = 0x83062784; continue 'dispatch;
	}
	// 830626E4: 80750008  lwz r3, 8(r21)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(8 as u32) ) } as u64;
	// 830626E8: 4BF7BC81  bl 0x82fde368
	ctx.lr = 0x830626EC;
	sub_82FDE368(ctx, base);
	// 830626EC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830626F0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 830626F4: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 830626F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830626FC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83062700: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83062704: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83062708: 7EEB532E  sthx r23, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[23].u16) };
	// 8306270C: 80BD0018  lwz r5, 0x18(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83062710: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83062714: 4BF862C5  bl 0x82fe89d8
	ctx.lr = 0x83062718;
	sub_82FE89D8(ctx, base);
	// 83062718: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8306271C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83062720: 409A0040  bne cr6, 0x83062760
	if !ctx.cr[6].eq {
	pc = 0x83062760; continue 'dispatch;
	}
	// 83062724: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 83062728: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8306272C: 4BF75B6D  bl 0x82fd8298
	ctx.lr = 0x83062730;
	sub_82FD8298(ctx, base);
	// 83062730: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83062734: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83062738: 41820010  beq 0x83062748
	if ctx.cr[0].eq {
	pc = 0x83062748; continue 'dispatch;
	}
	// 8306273C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83062740: 480326F9  bl 0x83094e38
	ctx.lr = 0x83062744;
	sub_83094E38(ctx, base);
	// 83062744: 48000008  b 0x8306274c
	pc = 0x8306274C; continue 'dispatch;
	// 83062748: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 8306274C: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83062750: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83062754: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 83062758: 915E0020  stw r10, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8306275C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83062760: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83062764: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83062768: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8306276C: 7EEB532E  sthx r23, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[23].u16) };
	// 83062770: 809D0018  lwz r4, 0x18(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83062774: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83062778: 48032861  bl 0x83094fd8
	ctx.lr = 0x8306277C;
	sub_83094FD8(ctx, base);
	// 8306277C: 839E0010  lwz r28, 0x10(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83062780: 48000094  b 0x83062814
	pc = 0x83062814; continue 'dispatch;
	// 83062784: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 83062788: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8306278C: 4BF75B0D  bl 0x82fd8298
	ctx.lr = 0x83062790;
	sub_82FD8298(ctx, base);
	// 83062790: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83062794: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83062798: 41820030  beq 0x830627c8
	if ctx.cr[0].eq {
	pc = 0x830627C8; continue 'dispatch;
	}
	// 8306279C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830627A0: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 830627A4: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 830627A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830627AC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830627B0: 7EEB532E  sthx r23, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[23].u16) };
	// 830627B4: 809D0018  lwz r4, 0x18(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 830627B8: 80FE0008  lwz r7, 8(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830627BC: 480326F5  bl 0x83094eb0
	ctx.lr = 0x830627C0;
	sub_83094EB0(ctx, base);
	// 830627C0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830627C4: 48000008  b 0x830627cc
	pc = 0x830627CC; continue 'dispatch;
	// 830627C8: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 830627CC: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 830627D0: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 830627D4: 616A8028  ori r10, r11, 0x8028
	ctx.r[10].u64 = ctx.r[11].u64 | 32808;
	// 830627D8: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830627DC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830627E0: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 830627E4: 913E0020  stw r9, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 830627E8: 91640014  stw r11, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830627EC: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 830627F0: 813E003C  lwz r9, 0x3c(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830627F4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830627F8: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830627FC: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 83062800: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83062804: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83062808: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8306280C: 99640011  stb r11, 0x11(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 83062810: 48002DA9  bl 0x830655b8
	ctx.lr = 0x83062814;
	sub_830655B8(ctx, base);
	// 83062814: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83062818: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8306281C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83062820: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 83062824: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83062828: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8306282C: 5576DFFE  rlwinm r22, r11, 0x1b, 0x1f, 0x1f
	ctx.r[22].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83062830: 48001A19  bl 0x83064248
	ctx.lr = 0x83062834;
	sub_83064248(ctx, base);
	// 83062834: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83062838: 40820010  bne 0x83062848
	if !ctx.cr[0].eq {
	pc = 0x83062848; continue 'dispatch;
	}
	// 8306283C: 388000CF  li r4, 0xcf
	ctx.r[4].s64 = 207;
	// 83062840: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83062844: 4BF86045  bl 0x82fe8888
	ctx.lr = 0x83062848;
	sub_82FE8888(ctx, base);
	// 83062848: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 8306284C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83062850: 388A78FC  addi r4, r10, 0x78fc
	ctx.r[4].s64 = ctx.r[10].s64 + 30972;
	// 83062854: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83062858: 4BF90C79  bl 0x82ff34d0
	ctx.lr = 0x8306285C;
	sub_82FF34D0(ctx, base);
	// 8306285C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83062860: 4182000C  beq 0x8306286c
	if ctx.cr[0].eq {
	pc = 0x8306286C; continue 'dispatch;
	}
	// 83062864: 92FC0008  stw r23, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[23].u32 ) };
	// 83062868: 4800019C  b 0x83062a04
	pc = 0x83062A04; continue 'dispatch;
	// 8306286C: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83062870: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83062874: 388A7AC4  addi r4, r10, 0x7ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 31428;
	// 83062878: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8306287C: 4BF90C55  bl 0x82ff34d0
	ctx.lr = 0x83062880;
	sub_82FF34D0(ctx, base);
	// 83062880: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83062884: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83062888: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8306288C: 41820044  beq 0x830628d0
	if ctx.cr[0].eq {
	pc = 0x830628D0; continue 'dispatch;
	}
	// 83062890: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83062894: 388A7C7C  addi r4, r10, 0x7c7c
	ctx.r[4].s64 = ctx.r[10].s64 + 31868;
	// 83062898: 4BF90C39  bl 0x82ff34d0
	ctx.lr = 0x8306289C;
	sub_82FF34D0(ctx, base);
	// 8306289C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830628A0: 4082000C  bne 0x830628ac
	if !ctx.cr[0].eq {
	pc = 0x830628AC; continue 'dispatch;
	}
	// 830628A4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830628A8: 480000B4  b 0x8306295c
	pc = 0x8306295C; continue 'dispatch;
	// 830628AC: 38800053  li r4, 0x53
	ctx.r[4].s64 = 83;
	// 830628B0: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 830628B4: 4BF8B9FD  bl 0x82fee2b0
	ctx.lr = 0x830628B8;
	sub_82FEE2B0(ctx, base);
	// 830628B8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830628BC: 4082000C  bne 0x830628c8
	if !ctx.cr[0].eq {
	pc = 0x830628C8; continue 'dispatch;
	}
	// 830628C0: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 830628C4: 48000098  b 0x8306295c
	pc = 0x8306295C; continue 'dispatch;
	// 830628C8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 830628CC: 48000090  b 0x8306295c
	pc = 0x8306295C; continue 'dispatch;
	// 830628D0: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 830628D4: 388A7970  addi r4, r10, 0x7970
	ctx.r[4].s64 = ctx.r[10].s64 + 31088;
	// 830628D8: 4BF90BF9  bl 0x82ff34d0
	ctx.lr = 0x830628DC;
	sub_82FF34D0(ctx, base);
	// 830628DC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830628E0: 41820044  beq 0x83062924
	if ctx.cr[0].eq {
	pc = 0x83062924; continue 'dispatch;
	}
	// 830628E4: 38800059  li r4, 0x59
	ctx.r[4].s64 = 89;
	// 830628E8: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 830628EC: 4BF8B9C5  bl 0x82fee2b0
	ctx.lr = 0x830628F0;
	sub_82FEE2B0(ctx, base);
	// 830628F0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830628F4: 4182000C  beq 0x83062900
	if ctx.cr[0].eq {
	pc = 0x83062900; continue 'dispatch;
	}
	// 830628F8: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 830628FC: 48000060  b 0x8306295c
	pc = 0x8306295C; continue 'dispatch;
	// 83062900: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83062904: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83062908: 388A7ABC  addi r4, r10, 0x7abc
	ctx.r[4].s64 = ctx.r[10].s64 + 31420;
	// 8306290C: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83062910: 4BF90BC1  bl 0x82ff34d0
	ctx.lr = 0x83062914;
	sub_82FF34D0(ctx, base);
	// 83062914: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83062918: 418203B0  beq 0x83062cc8
	if ctx.cr[0].eq {
	pc = 0x83062CC8; continue 'dispatch;
	}
	// 8306291C: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 83062920: 4800003C  b 0x8306295c
	pc = 0x8306295C; continue 'dispatch;
	// 83062924: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83062928: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8306292C: 388A7C30  addi r4, r10, 0x7c30
	ctx.r[4].s64 = ctx.r[10].s64 + 31792;
	// 83062930: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83062934: 4BF90B9D  bl 0x82ff34d0
	ctx.lr = 0x83062938;
	sub_82FF34D0(ctx, base);
	// 83062938: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8306293C: 41820028  beq 0x83062964
	if ctx.cr[0].eq {
	pc = 0x83062964; continue 'dispatch;
	}
	// 83062940: 38800053  li r4, 0x53
	ctx.r[4].s64 = 83;
	// 83062944: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83062948: 4BF8B969  bl 0x82fee2b0
	ctx.lr = 0x8306294C;
	sub_82FEE2B0(ctx, base);
	// 8306294C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83062950: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 83062954: 40820008  bne 0x8306295c
	if !ctx.cr[0].eq {
	pc = 0x8306295C; continue 'dispatch;
	}
	// 83062958: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 8306295C: 917C0008  stw r11, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83062960: 480000A4  b 0x83062a04
	pc = 0x83062A04; continue 'dispatch;
	// 83062964: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83062968: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8306296C: 388A7C10  addi r4, r10, 0x7c10
	ctx.r[4].s64 = ctx.r[10].s64 + 31760;
	// 83062970: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83062974: 4BF90B5D  bl 0x82ff34d0
	ctx.lr = 0x83062978;
	sub_82FF34D0(ctx, base);
	// 83062978: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8306297C: 41820034  beq 0x830629b0
	if ctx.cr[0].eq {
	pc = 0x830629B0; continue 'dispatch;
	}
	// 83062980: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83062984: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83062988: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8306298C: 480018BD  bl 0x83064248
	ctx.lr = 0x83062990;
	sub_83064248(ctx, base);
	// 83062990: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83062994: 40820010  bne 0x830629a4
	if !ctx.cr[0].eq {
	pc = 0x830629A4; continue 'dispatch;
	}
	// 83062998: 388000CF  li r4, 0xcf
	ctx.r[4].s64 = 207;
	// 8306299C: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830629A0: 4BF85EE9  bl 0x82fe8888
	ctx.lr = 0x830629A4;
	sub_82FE8888(ctx, base);
	// 830629A4: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 830629A8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 830629AC: 48000020  b 0x830629cc
	pc = 0x830629CC; continue 'dispatch;
	// 830629B0: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 830629B4: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 830629B8: 4BF8B8F9  bl 0x82fee2b0
	ctx.lr = 0x830629BC;
	sub_82FEE2B0(ctx, base);
	// 830629BC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830629C0: 41820308  beq 0x83062cc8
	if ctx.cr[0].eq {
	pc = 0x83062CC8; continue 'dispatch;
	}
	// 830629C4: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	// 830629C8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830629CC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830629D0: 917C0008  stw r11, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830629D4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830629D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830629DC: 4BFFCDA5  bl 0x8305f780
	ctx.lr = 0x830629E0;
	sub_8305F780(ctx, base);
	// 830629E0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830629E4: 41820320  beq 0x83062d04
	if ctx.cr[0].eq {
	pc = 0x83062D04; continue 'dispatch;
	}
	// 830629E8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830629EC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830629F0: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 830629F4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830629F8: 7EEB532E  sthx r23, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[23].u16) };
	// 830629FC: 809D0018  lwz r4, 0x18(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83062A00: 4BFFB7E9  bl 0x8305e1e8
	ctx.lr = 0x83062A04;
	sub_8305E1E8(ctx, base);
	// 83062A04: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83062A08: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83062A0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83062A10: 48001839  bl 0x83064248
	ctx.lr = 0x83062A14;
	sub_83064248(ctx, base);
	// 83062A14: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83062A18: 40820010  bne 0x83062a28
	if !ctx.cr[0].eq {
	pc = 0x83062A28; continue 'dispatch;
	}
	// 83062A1C: 388000CF  li r4, 0xcf
	ctx.r[4].s64 = 207;
	// 83062A20: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83062A24: 4BF85E65  bl 0x82fe8888
	ctx.lr = 0x83062A28;
	sub_82FE8888(ctx, base);
	// 83062A28: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83062A2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83062A30: 4BFFF671  bl 0x830620a0
	ctx.lr = 0x83062A34;
	sub_830620A0(ctx, base);
	// 83062A34: 815E0030  lwz r10, 0x30(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83062A38: 896A0010  lbz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83062A3C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83062A40: 41820258  beq 0x83062c98
	if ctx.cr[0].eq {
	pc = 0x83062C98; continue 'dispatch;
	}
	// 83062A44: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 83062A48: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83062A4C: 409A004C  bne cr6, 0x83062a98
	if !ctx.cr[6].eq {
	pc = 0x83062A98; continue 'dispatch;
	}
	// 83062A50: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83062A54: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83062A58: 419A0040  beq cr6, 0x83062a98
	if ctx.cr[6].eq {
	pc = 0x83062A98; continue 'dispatch;
	}
	// 83062A5C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83062A60: 419A0038  beq cr6, 0x83062a98
	if ctx.cr[6].eq {
	pc = 0x83062A98; continue 'dispatch;
	}
	// 83062A64: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83062A68: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83062A6C: 83AA00A8  lwz r29, 0xa8(r10)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(168 as u32) ) } as u64;
	// 83062A70: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83062A74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83062A78: 4E800421  bctrl
	ctx.lr = 0x83062A7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83062A7C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83062A80: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 83062A84: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83062A88: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83062A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83062A90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83062A94: 4BFB4AC5  bl 0x83017558
	ctx.lr = 0x83062A98;
	sub_83017558(ctx, base);
	// 83062A98: 39400078  li r10, 0x78
	ctx.r[10].s64 = 120;
	// 83062A9C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83062AA0: 3B00006C  li r24, 0x6c
	ctx.r[24].s64 = 108;
	// 83062AA4: B2FF0082  sth r23, 0x82(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(130 as u32), ctx.r[23].u16 ) };
	// 83062AA8: 3B200073  li r25, 0x73
	ctx.r[25].s64 = 115;
	// 83062AAC: 3B400070  li r26, 0x70
	ctx.r[26].s64 = 112;
	// 83062AB0: 3B600061  li r27, 0x61
	ctx.r[27].s64 = 97;
	// 83062AB4: B15F0070  sth r10, 0x70(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[10].u16 ) };
	// 83062AB8: 3940006D  li r10, 0x6d
	ctx.r[10].s64 = 109;
	// 83062ABC: 3BA00065  li r29, 0x65
	ctx.r[29].s64 = 101;
	// 83062AC0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83062AC4: B31F0074  sth r24, 0x74(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[24].u16 ) };
	// 83062AC8: 3A9F0070  addi r20, r31, 0x70
	ctx.r[20].s64 = ctx.r[31].s64 + 112;
	// 83062ACC: B33F0078  sth r25, 0x78(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[25].u16 ) };
	// 83062AD0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83062AD4: B35F007A  sth r26, 0x7a(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(122 as u32), ctx.r[26].u16 ) };
	// 83062AD8: B15F0072  sth r10, 0x72(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(114 as u32), ctx.r[10].u16 ) };
	// 83062ADC: 3940003A  li r10, 0x3a
	ctx.r[10].s64 = 58;
	// 83062AE0: B37F007C  sth r27, 0x7c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[27].u16 ) };
	// 83062AE4: B3BF0080  sth r29, 0x80(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[29].u16 ) };
	// 83062AE8: B15F0076  sth r10, 0x76(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(118 as u32), ctx.r[10].u16 ) };
	// 83062AEC: 39400063  li r10, 0x63
	ctx.r[10].s64 = 99;
	// 83062AF0: B15F007E  sth r10, 0x7e(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(126 as u32), ctx.r[10].u16 ) };
	// 83062AF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83062AF8: 4E800421  bctrl
	ctx.lr = 0x83062AFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83062AFC: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 83062B00: 4BF71141  bl 0x82fd3c40
	ctx.lr = 0x83062B04;
	sub_82FD3C40(ctx, base);
	// 83062B04: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83062B08: 41820190  beq 0x83062c98
	if ctx.cr[0].eq {
	pc = 0x83062C98; continue 'dispatch;
	}
	// 83062B0C: 39600072  li r11, 0x72
	ctx.r[11].s64 = 114;
	// 83062B10: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 83062B14: B37F0066  sth r27, 0x66(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(102 as u32), ctx.r[27].u16 ) };
	// 83062B18: 7EFBBB78  mr r27, r23
	ctx.r[27].u64 = ctx.r[23].u64;
	// 83062B1C: B35F0070  sth r26, 0x70(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[26].u16 ) };
	// 83062B20: 2F0A0009  cmpwi cr6, r10, 9
	ctx.cr[6].compare_i32(ctx.r[10].s32, 9, &mut ctx.xer);
	// 83062B24: B3BF0074  sth r29, 0x74(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[29].u16 ) };
	// 83062B28: B33F0076  sth r25, 0x76(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(118 as u32), ctx.r[25].u16 ) };
	// 83062B2C: B17F0072  sth r11, 0x72(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(114 as u32), ctx.r[11].u16 ) };
	// 83062B30: B17F007A  sth r11, 0x7a(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(122 as u32), ctx.r[11].u16 ) };
	// 83062B34: 39600076  li r11, 0x76
	ctx.r[11].s64 = 118;
	// 83062B38: B3BF0078  sth r29, 0x78(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[29].u16 ) };
	// 83062B3C: B3BF007E  sth r29, 0x7e(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(126 as u32), ctx.r[29].u16 ) };
	// 83062B40: B2FF0080  sth r23, 0x80(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[23].u16 ) };
	// 83062B44: B3BF0062  sth r29, 0x62(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(98 as u32), ctx.r[29].u16 ) };
	// 83062B48: B17F007C  sth r11, 0x7c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u16 ) };
	// 83062B4C: 39600064  li r11, 0x64
	ctx.r[11].s64 = 100;
	// 83062B50: B31F006A  sth r24, 0x6a(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(106 as u32), ctx.r[24].u16 ) };
	// 83062B54: B2FF006E  sth r23, 0x6e(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(110 as u32), ctx.r[23].u16 ) };
	// 83062B58: B17F0060  sth r11, 0x60(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u16 ) };
	// 83062B5C: 39600066  li r11, 0x66
	ctx.r[11].s64 = 102;
	// 83062B60: B17F0064  sth r11, 0x64(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u16 ) };
	// 83062B64: 39600075  li r11, 0x75
	ctx.r[11].s64 = 117;
	// 83062B68: B17F0068  sth r11, 0x68(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u16 ) };
	// 83062B6C: 39600074  li r11, 0x74
	ctx.r[11].s64 = 116;
	// 83062B70: B17F006C  sth r11, 0x6c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u16 ) };
	// 83062B74: 409A010C  bne cr6, 0x83062c80
	if !ctx.cr[6].eq {
	pc = 0x83062C80; continue 'dispatch;
	}
	// 83062B78: 807C001C  lwz r3, 0x1c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 83062B7C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83062B80: 4BF70C61  bl 0x82fd37e0
	ctx.lr = 0x83062B84;
	sub_82FD37E0(ctx, base);
	// 83062B84: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83062B88: 837D0008  lwz r27, 8(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83062B8C: 2F1B0001  cmpwi cr6, r27, 1
	ctx.cr[6].compare_i32(ctx.r[27].s32, 1, &mut ctx.xer);
	// 83062B90: 409A0040  bne cr6, 0x83062bd0
	if !ctx.cr[6].eq {
	pc = 0x83062BD0; continue 'dispatch;
	}
	// 83062B94: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83062B98: 3B5F0060  addi r26, r31, 0x60
	ctx.r[26].s64 = ctx.r[31].s64 + 96;
	// 83062B9C: 4BFC9CD5  bl 0x8302c870
	ctx.lr = 0x83062BA0;
	sub_8302C870(ctx, base);
	// 83062BA0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83062BA4: 4BF7109D  bl 0x82fd3c40
	ctx.lr = 0x83062BA8;
	sub_82FD3C40(ctx, base);
	// 83062BA8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83062BAC: 408200AC  bne 0x83062c58
	if !ctx.cr[0].eq {
	pc = 0x83062C58; continue 'dispatch;
	}
	// 83062BB0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83062BB4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83062BB8: 3B5F0070  addi r26, r31, 0x70
	ctx.r[26].s64 = ctx.r[31].s64 + 112;
	// 83062BBC: 4BFC9CB5  bl 0x8302c870
	ctx.lr = 0x83062BC0;
	sub_8302C870(ctx, base);
	// 83062BC0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83062BC4: 4BF7107D  bl 0x82fd3c40
	ctx.lr = 0x83062BC8;
	sub_82FD3C40(ctx, base);
	// 83062BC8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83062BCC: 4082008C  bne 0x83062c58
	if !ctx.cr[0].eq {
	pc = 0x83062C58; continue 'dispatch;
	}
	// 83062BD0: 2F1B0002  cmpwi cr6, r27, 2
	ctx.cr[6].compare_i32(ctx.r[27].s32, 2, &mut ctx.xer);
	// 83062BD4: 409A008C  bne cr6, 0x83062c60
	if !ctx.cr[6].eq {
	pc = 0x83062C60; continue 'dispatch;
	}
	// 83062BD8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83062BDC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83062BE0: 3B7F0060  addi r27, r31, 0x60
	ctx.r[27].s64 = ctx.r[31].s64 + 96;
	// 83062BE4: 4BFC9C8D  bl 0x8302c870
	ctx.lr = 0x83062BE8;
	sub_8302C870(ctx, base);
	// 83062BE8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83062BEC: 4BF71055  bl 0x82fd3c40
	ctx.lr = 0x83062BF0;
	sub_82FD3C40(ctx, base);
	// 83062BF0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83062BF4: 41820024  beq 0x83062c18
	if ctx.cr[0].eq {
	pc = 0x83062C18; continue 'dispatch;
	}
	// 83062BF8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83062BFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83062C00: 3B7F0070  addi r27, r31, 0x70
	ctx.r[27].s64 = ctx.r[31].s64 + 112;
	// 83062C04: 4BFC9C6D  bl 0x8302c870
	ctx.lr = 0x83062C08;
	sub_8302C870(ctx, base);
	// 83062C08: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83062C0C: 4BF71035  bl 0x82fd3c40
	ctx.lr = 0x83062C10;
	sub_82FD3C40(ctx, base);
	// 83062C10: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83062C14: 40820044  bne 0x83062c58
	if !ctx.cr[0].eq {
	pc = 0x83062C58; continue 'dispatch;
	}
	// 83062C18: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83062C1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83062C20: 3B7F0060  addi r27, r31, 0x60
	ctx.r[27].s64 = ctx.r[31].s64 + 96;
	// 83062C24: 4BFC9C4D  bl 0x8302c870
	ctx.lr = 0x83062C28;
	sub_8302C870(ctx, base);
	// 83062C28: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83062C2C: 4BF71015  bl 0x82fd3c40
	ctx.lr = 0x83062C30;
	sub_82FD3C40(ctx, base);
	// 83062C30: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83062C34: 4182002C  beq 0x83062c60
	if ctx.cr[0].eq {
	pc = 0x83062C60; continue 'dispatch;
	}
	// 83062C38: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83062C3C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83062C40: 3B7F0070  addi r27, r31, 0x70
	ctx.r[27].s64 = ctx.r[31].s64 + 112;
	// 83062C44: 4BFC9C2D  bl 0x8302c870
	ctx.lr = 0x83062C48;
	sub_8302C870(ctx, base);
	// 83062C48: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83062C4C: 4BF70FF5  bl 0x82fd3c40
	ctx.lr = 0x83062C50;
	sub_82FD3C40(ctx, base);
	// 83062C50: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83062C54: 4182000C  beq 0x83062c60
	if ctx.cr[0].eq {
	pc = 0x83062C60; continue 'dispatch;
	}
	// 83062C58: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83062C5C: 48000008  b 0x83062c64
	pc = 0x83062C64; continue 'dispatch;
	// 83062C60: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 83062C64: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83062C68: 557B063E  clrlwi r27, r11, 0x18
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83062C6C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83062C70: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83062C74: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83062C78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83062C7C: 4E800421  bctrl
	ctx.lr = 0x83062C80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83062C80: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83062C84: 40820014  bne 0x83062c98
	if !ctx.cr[0].eq {
	pc = 0x83062C98; continue 'dispatch;
	}
	// 83062C88: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83062C8C: 38800032  li r4, 0x32
	ctx.r[4].s64 = 50;
	// 83062C90: 806B00A8  lwz r3, 0xa8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 83062C94: 4BFB4755  bl 0x830173e8
	ctx.lr = 0x83062C98;
	sub_830173E8(ctx, base);
	// 83062C98: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83062C9C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83062CA0: 41820020  beq 0x83062cc0
	if ctx.cr[0].eq {
	pc = 0x83062CC0; continue 'dispatch;
	}
	// 83062CA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83062CA8: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 83062CAC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83062CB0: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 83062CB4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83062CB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83062CBC: 4E800421  bctrl
	ctx.lr = 0x83062CC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83062CC0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83062CC4: 48000044  b 0x83062d08
	pc = 0x83062D08; continue 'dispatch;
	// 83062CC8: 80750008  lwz r3, 8(r21)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(8 as u32) ) } as u64;
	// 83062CCC: 4BF7B69D  bl 0x82fde368
	ctx.lr = 0x83062CD0;
	sub_82FDE368(ctx, base);
	// 83062CD0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83062CD4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83062CD8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83062CDC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83062CE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83062CE4: 4E800421  bctrl
	ctx.lr = 0x83062CE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83062CE8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83062CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83062CF0: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83062CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83062CF8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83062CFC: 388000C9  li r4, 0xc9
	ctx.r[4].s64 = 201;
	// 83062D00: 4BF85CD9  bl 0x82fe89d8
	ctx.lr = 0x83062D04;
	sub_82FE89D8(ctx, base);
	// 83062D04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83062D08: 383F00F0  addi r1, r31, 0xf0
	ctx.r[1].s64 = ctx.r[31].s64 + 240;
	// 83062D0C: 4814548C  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83062D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83062D10 size=48
    let mut pc: u32 = 0x83062D10;
    'dispatch: loop {
        match pc {
            0x83062D10 => {
    //   block [0x83062D10..0x83062D40)
	// 83062D10: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83062D14: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83062D18: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83062D1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83062D20: 817F0104  lwz r11, 0x104(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 83062D24: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83062D28: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83062D2C: 4BF755B5  bl 0x82fd82e0
	ctx.lr = 0x83062D30;
	sub_82FD82E0(ctx, base);
	// 83062D30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83062D34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83062D38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83062D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83062D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83062D40 size=48
    let mut pc: u32 = 0x83062D40;
    'dispatch: loop {
        match pc {
            0x83062D40 => {
    //   block [0x83062D40..0x83062D70)
	// 83062D40: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83062D44: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83062D48: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83062D4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83062D50: 817F0104  lwz r11, 0x104(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 83062D54: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83062D58: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83062D5C: 4BF75585  bl 0x82fd82e0
	ctx.lr = 0x83062D60;
	sub_82FD82E0(ctx, base);
	// 83062D60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83062D64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83062D68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83062D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83062D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83062D70 size=8
    let mut pc: u32 = 0x83062D70;
    'dispatch: loop {
        match pc {
            0x83062D70 => {
    //   block [0x83062D70..0x83062D78)
	// 83062D70: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83062D74: 821660D8  lwz r16, 0x60d8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(24792 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83062D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83062D78 size=860
    let mut pc: u32 = 0x83062D78;
    'dispatch: loop {
        match pc {
            0x83062D78 => {
    //   block [0x83062D78..0x830630D4)
	// 83062D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83062D7C: 481453D9  bl 0x831a8154
	ctx.lr = 0x83062D80;
	sub_831A8130(ctx, base);
	// 83062D80: 3BE1FF30  addi r31, r1, -0xd0
	ctx.r[31].s64 = ctx.r[1].s64 + -208;
	// 83062D84: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83062D88: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83062D8C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83062D90: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83062D94: 93DF00E4  stw r30, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[30].u32 ) };
	// 83062D98: 480014B1  bl 0x83064248
	ctx.lr = 0x83062D9C;
	sub_83064248(ctx, base);
	// 83062D9C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83062DA0: 40820020  bne 0x83062dc0
	if !ctx.cr[0].eq {
	pc = 0x83062DC0; continue 'dispatch;
	}
	// 83062DA4: 388000CF  li r4, 0xcf
	ctx.r[4].s64 = 207;
	// 83062DA8: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83062DAC: 4BF85ADD  bl 0x82fe8888
	ctx.lr = 0x83062DB0;
	sub_82FE8888(ctx, base);
	// 83062DB0: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 83062DB4: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83062DB8: 4BFB4A59  bl 0x83017810
	ctx.lr = 0x83062DBC;
	sub_83017810(ctx, base);
	// 83062DBC: 48000310  b 0x830630cc
	pc = 0x830630CC; continue 'dispatch;
	// 83062DC0: 82FE0028  lwz r23, 0x28(r30)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83062DC4: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 83062DC8: 4BF7C231  bl 0x82fdeff8
	ctx.lr = 0x83062DCC;
	sub_82FDEFF8(ctx, base);
	// 83062DCC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83062DD0: 92FF005C  stw r23, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[23].u32 ) };
	// 83062DD4: 939F0058  stw r28, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 83062DD8: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 83062DDC: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83062DE0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83062DE4: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83062DE8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83062DEC: 935C0004  stw r26, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 83062DF0: B34A0000  sth r26, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[26].u16 ) };
	// 83062DF4: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83062DF8: 4BF90311  bl 0x82ff3108
	ctx.lr = 0x83062DFC;
	sub_82FF3108(ctx, base);
	// 83062DFC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83062E00: 40820020  bne 0x83062e20
	if !ctx.cr[0].eq {
	pc = 0x83062E20; continue 'dispatch;
	}
	// 83062E04: 388000B7  li r4, 0xb7
	ctx.r[4].s64 = 183;
	// 83062E08: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83062E0C: 4BF85A7D  bl 0x82fe8888
	ctx.lr = 0x83062E10;
	sub_82FE8888(ctx, base);
	// 83062E10: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 83062E14: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83062E18: 4BFB49F9  bl 0x83017810
	ctx.lr = 0x83062E1C;
	sub_83017810(ctx, base);
	// 83062E1C: 480002A4  b 0x830630c0
	pc = 0x830630C0; continue 'dispatch;
	// 83062E20: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83062E24: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 83062E28: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83062E2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83062E30: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83062E34: 7F4B532E  sthx r26, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[26].u16) };
	// 83062E38: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83062E3C: 80DC0018  lwz r6, 0x18(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83062E40: 809E0038  lwz r4, 0x38(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 83062E44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83062E48: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83062E4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83062E50: 4E800421  bctrl
	ctx.lr = 0x83062E54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83062E54: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 83062E58: 4082009C  bne 0x83062ef4
	if !ctx.cr[0].eq {
	pc = 0x83062EF4; continue 'dispatch;
	}
	// 83062E5C: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 83062E60: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83062E64: 4BF75435  bl 0x82fd8298
	ctx.lr = 0x83062E68;
	sub_82FD8298(ctx, base);
	// 83062E68: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83062E6C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83062E70: 41820030  beq 0x83062ea0
	if ctx.cr[0].eq {
	pc = 0x83062EA0; continue 'dispatch;
	}
	// 83062E74: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83062E78: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83062E7C: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83062E80: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83062E84: 7F4B532E  sthx r26, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[26].u16) };
	// 83062E88: 809C0018  lwz r4, 0x18(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83062E8C: 80FE0008  lwz r7, 8(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83062E90: 80BE0038  lwz r5, 0x38(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 83062E94: 4800195D  bl 0x830647f0
	ctx.lr = 0x83062E98;
	sub_830647F0(ctx, base);
	// 83062E98: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83062E9C: 48000008  b 0x83062ea4
	pc = 0x83062EA4; continue 'dispatch;
	// 83062EA0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83062EA4: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83062EA8: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 83062EAC: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83062EB0: 614A8028  ori r10, r10, 0x8028
	ctx.r[10].u64 = ctx.r[10].u64 | 32808;
	// 83062EB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83062EB8: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83062EBC: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83062EC0: 813E003C  lwz r9, 0x3c(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 83062EC4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83062EC8: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83062ECC: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 83062ED0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83062ED4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83062ED8: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83062EDC: 99640014  stb r11, 0x14(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83062EE0: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83062EE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83062EE8: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 83062EEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83062EF0: 4E800421  bctrl
	ctx.lr = 0x83062EF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83062EF4: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83062EF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83062EFC: 419A001C  beq cr6, 0x83062f18
	if ctx.cr[6].eq {
	pc = 0x83062F18; continue 'dispatch;
	}
	// 83062F00: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83062F04: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83062F08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83062F0C: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83062F10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83062F14: 4E800421  bctrl
	ctx.lr = 0x83062F18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83062F18: 833E0028  lwz r25, 0x28(r30)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83062F1C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83062F20: 4BF7C0D9  bl 0x82fdeff8
	ctx.lr = 0x83062F24;
	sub_82FDEFF8(ctx, base);
	// 83062F24: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83062F28: 933F0054  stw r25, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[25].u32 ) };
	// 83062F2C: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 83062F30: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83062F34: 7F58D378  mr r24, r26
	ctx.r[24].u64 = ctx.r[26].u64;
	// 83062F38: 4800010C  b 0x83063044
	pc = 0x83063044; continue 'dispatch;
	// 83062F3C: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83062F40: 2B0B003E  cmplwi cr6, r11, 0x3e
	ctx.cr[6].compare_u32(ctx.r[11].u32, 62 as u32, &mut ctx.xer);
	// 83062F44: 419A0138  beq cr6, 0x8306307c
	if ctx.cr[6].eq {
	pc = 0x8306307C; continue 'dispatch;
	}
	// 83062F48: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 83062F4C: 61498054  ori r9, r10, 0x8054
	ctx.r[9].u64 = ctx.r[10].u64 | 32852;
	// 83062F50: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83062F54: 7D4A482E  lwzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 83062F58: 7D4A58AE  lbzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83062F5C: 554A0031  rlwinm. r10, r10, 0, 0, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83062F60: 41820050  beq 0x83062fb0
	if ctx.cr[0].eq {
	pc = 0x83062FB0; continue 'dispatch;
	}
	// 83062F64: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83062F68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83062F6C: 419A0038  beq cr6, 0x83062fa4
	if ctx.cr[6].eq {
	pc = 0x83062FA4; continue 'dispatch;
	}
	// 83062F70: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83062F74: 4BF8B205  bl 0x82fee178
	ctx.lr = 0x83062F78;
	sub_82FEE178(ctx, base);
	// 83062F78: 80BD0004  lwz r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83062F7C: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83062F80: 54AB083C  slwi r11, r5, 1
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83062F84: 7F4B532E  sthx r26, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[26].u16) };
	// 83062F88: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83062F8C: 809D0018  lwz r4, 0x18(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83062F90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83062F94: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83062F98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83062F9C: 4E800421  bctrl
	ctx.lr = 0x83062FA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83062FA0: 480000A0  b 0x83063040
	pc = 0x83063040; continue 'dispatch;
	// 83062FA4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83062FA8: 4BF8B479  bl 0x82fee420
	ctx.lr = 0x83062FAC;
	sub_82FEE420(ctx, base);
	// 83062FAC: 48000094  b 0x83063040
	pc = 0x83063040; continue 'dispatch;
	// 83062FB0: 2B0B0025  cmplwi cr6, r11, 0x25
	ctx.cr[6].compare_u32(ctx.r[11].u32, 37 as u32, &mut ctx.xer);
	// 83062FB4: 409A0024  bne cr6, 0x83062fd8
	if !ctx.cr[6].eq {
	pc = 0x83062FD8; continue 'dispatch;
	}
	// 83062FB8: 4BF8B159  bl 0x82fee110
	ctx.lr = 0x83062FBC;
	sub_82FEE110(ctx, base);
	// 83062FBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83062FC0: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83062FC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83062FC8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83062FCC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83062FD0: 48000A81  bl 0x83063a50
	ctx.lr = 0x83062FD4;
	sub_83063A50(ctx, base);
	// 83062FD4: 4800006C  b 0x83063040
	pc = 0x83063040; continue 'dispatch;
	// 83062FD8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83062FDC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83062FE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83062FE4: 4BFFF675  bl 0x83062658
	ctx.lr = 0x83062FE8;
	sub_83062658(ctx, base);
	// 83062FE8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83062FEC: 41820098  beq 0x83063084
	if ctx.cr[0].eq {
	pc = 0x83063084; continue 'dispatch;
	}
	// 83062FF0: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83062FF4: 896B0010  lbz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83062FF8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83062FFC: 41820044  beq 0x83063040
	if ctx.cr[0].eq {
	pc = 0x83063040; continue 'dispatch;
	}
	// 83063000: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83063004: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83063008: 409A0038  bne cr6, 0x83063040
	if !ctx.cr[6].eq {
	pc = 0x83063040; continue 'dispatch;
	}
	// 8306300C: 570B063F  clrlwi. r11, r24, 0x18
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83063010: 4182002C  beq 0x8306303c
	if ctx.cr[0].eq {
	pc = 0x8306303C; continue 'dispatch;
	}
	// 83063014: 807B0008  lwz r3, 8(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 83063018: 4BF7B351  bl 0x82fde368
	ctx.lr = 0x8306301C;
	sub_82FDE368(ctx, base);
	// 8306301C: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83063020: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83063024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83063028: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8306302C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83063030: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 83063034: 806B00A8  lwz r3, 0xa8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 83063038: 4BFB4521  bl 0x83017558
	ctx.lr = 0x8306303C;
	sub_83017558(ctx, base);
	// 8306303C: 3B000001  li r24, 1
	ctx.r[24].s64 = 1;
	// 83063040: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83063044: 4BF8B205  bl 0x82fee248
	ctx.lr = 0x83063048;
	sub_82FEE248(ctx, base);
	// 83063048: 546B043F  clrlwi. r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8306304C: 4082FEF0  bne 0x83062f3c
	if !ctx.cr[0].eq {
	pc = 0x83062F3C; continue 'dispatch;
	}
	// 83063050: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83063054: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83063058: 38C0002F  li r6, 0x2f
	ctx.r[6].s64 = 47;
	// 8306305C: 388B5790  addi r4, r11, 0x5790
	ctx.r[4].s64 = ctx.r[11].s64 + 22416;
	// 83063060: 38A0034E  li r5, 0x34e
	ctx.r[5].s64 = 846;
	// 83063064: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83063068: 4BF84C79  bl 0x82fe7ce0
	ctx.lr = 0x8306306C;
	sub_82FE7CE0(ctx, base);
	// 8306306C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83063070: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83063074: 388BC700  addi r4, r11, -0x3900
	ctx.r[4].s64 = ctx.r[11].s64 + -14592;
	// 83063078: 4814DBB1  bl 0x831b0c28
	ctx.lr = 0x8306307C;
	sub_831B0C28(ctx, base);
	// 8306307C: 4BF8B095  bl 0x82fee110
	ctx.lr = 0x83063080;
	sub_82FEE110(ctx, base);
	// 83063080: 48000010  b 0x83063090
	pc = 0x83063090; continue 'dispatch;
	// 83063084: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 83063088: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 8306308C: 4BFB4785  bl 0x83017810
	ctx.lr = 0x83063090;
	sub_83017810(ctx, base);
	// 83063090: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83063094: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83063098: 419A001C  beq cr6, 0x830630b4
	if ctx.cr[6].eq {
	pc = 0x830630B4; continue 'dispatch;
	}
	// 8306309C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 830630A0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830630A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830630A8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 830630AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830630B0: 4E800421  bctrl
	ctx.lr = 0x830630B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830630B4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830630B8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 830630BC: 4BF7C06D  bl 0x82fdf128
	ctx.lr = 0x830630C0;
	sub_82FDF128(ctx, base);
	// 830630C0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830630C4: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 830630C8: 4BF7C061  bl 0x82fdf128
	ctx.lr = 0x830630CC;
	sub_82FDF128(ctx, base);
	// 830630CC: 383F00D0  addi r1, r31, 0xd0
	ctx.r[1].s64 = ctx.r[31].s64 + 208;
	// 830630D0: 481450D4  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830630D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830630D4 size=40
    let mut pc: u32 = 0x830630D4;
    'dispatch: loop {
        match pc {
            0x830630D4 => {
    //   block [0x830630D4..0x830630FC)
	// 830630D4: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 830630D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830630DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830630E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830630E4: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 830630E8: 4BF70EA1  bl 0x82fd3f88
	ctx.lr = 0x830630EC;
	sub_82FD3F88(ctx, base);
	// 830630EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830630F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830630F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830630F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830630FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830630FC size=48
    let mut pc: u32 = 0x830630FC;
    'dispatch: loop {
        match pc {
            0x830630FC => {
    //   block [0x830630FC..0x8306312C)
	// 830630FC: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 83063100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83063104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83063108: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8306310C: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 83063110: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83063114: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83063118: 4BF751C9  bl 0x82fd82e0
	ctx.lr = 0x8306311C;
	sub_82FD82E0(ctx, base);
	// 8306311C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83063120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83063124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83063128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8306312C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8306312C size=40
    let mut pc: u32 = 0x8306312C;
    'dispatch: loop {
        match pc {
            0x8306312C => {
    //   block [0x8306312C..0x83063154)
	// 8306312C: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 83063130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83063134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83063138: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8306313C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83063140: 4BF70E49  bl 0x82fd3f88
	ctx.lr = 0x83063144;
	sub_82FD3F88(ctx, base);
	// 83063144: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83063148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8306314C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83063150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83063158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83063158 size=708
    let mut pc: u32 = 0x83063158;
    'dispatch: loop {
        match pc {
            0x83063158 => {
    //   block [0x83063158..0x8306341C)
	// 83063158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306315C: 48145011  bl 0x831a816c
	ctx.lr = 0x83063160;
	sub_831A8130(ctx, base);
	// 83063160: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83063164: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83063168: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8306316C: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83063170: 4BF8AFA1  bl 0x82fee110
	ctx.lr = 0x83063174;
	sub_82FEE110(ctx, base);
	// 83063174: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 83063178: 2B0B0021  cmplwi cr6, r11, 0x21
	ctx.cr[6].compare_u32(ctx.r[11].u32, 33 as u32, &mut ctx.xer);
	// 8306317C: 409A023C  bne cr6, 0x830633b8
	if !ctx.cr[6].eq {
	pc = 0x830633B8; continue 'dispatch;
	}
	// 83063180: 3880002D  li r4, 0x2d
	ctx.r[4].s64 = 45;
	// 83063184: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83063188: 4BF8B129  bl 0x82fee2b0
	ctx.lr = 0x8306318C;
	sub_82FEE2B0(ctx, base);
	// 8306318C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83063190: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83063194: 41820028  beq 0x830631bc
	if ctx.cr[0].eq {
	pc = 0x830631BC; continue 'dispatch;
	}
	// 83063198: 3880002D  li r4, 0x2d
	ctx.r[4].s64 = 45;
	// 8306319C: 4BF8B115  bl 0x82fee2b0
	ctx.lr = 0x830631A0;
	sub_82FEE2B0(ctx, base);
	// 830631A0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830631A4: 41820010  beq 0x830631b4
	if ctx.cr[0].eq {
	pc = 0x830631B4; continue 'dispatch;
	}
	// 830631A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830631AC: 4BFFC035  bl 0x8305f1e0
	ctx.lr = 0x830631B0;
	sub_8305F1E0(ctx, base);
	// 830631B0: 48000264  b 0x83063414
	pc = 0x83063414; continue 'dispatch;
	// 830631B4: 388000B9  li r4, 0xb9
	ctx.r[4].s64 = 185;
	// 830631B8: 48000248  b 0x83063400
	pc = 0x83063400; continue 'dispatch;
	// 830631BC: 3880005B  li r4, 0x5b
	ctx.r[4].s64 = 91;
	// 830631C0: 4BF8B0F1  bl 0x82fee2b0
	ctx.lr = 0x830631C4;
	sub_82FEE2B0(ctx, base);
	// 830631C4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830631C8: 41820150  beq 0x83063318
	if ctx.cr[0].eq {
	pc = 0x83063318; continue 'dispatch;
	}
	// 830631CC: 897F001C  lbz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830631D0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830631D4: 4182000C  beq 0x830631e0
	if ctx.cr[0].eq {
	pc = 0x830631E0; continue 'dispatch;
	}
	// 830631D8: 388000DD  li r4, 0xdd
	ctx.r[4].s64 = 221;
	// 830631DC: 48000224  b 0x83063400
	pc = 0x83063400; continue 'dispatch;
	// 830631E0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830631E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830631E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830631EC: 4800105D  bl 0x83064248
	ctx.lr = 0x830631F0;
	sub_83064248(ctx, base);
	// 830631F0: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 830631F4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 830631F8: 388A7B0C  addi r4, r10, 0x7b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 31500;
	// 830631FC: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83063200: 4BF902D1  bl 0x82ff34d0
	ctx.lr = 0x83063204;
	sub_82FF34D0(ctx, base);
	// 83063204: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83063208: 4182006C  beq 0x83063274
	if ctx.cr[0].eq {
	pc = 0x83063274; continue 'dispatch;
	}
	// 8306320C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83063210: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83063214: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83063218: 48001031  bl 0x83064248
	ctx.lr = 0x8306321C;
	sub_83064248(ctx, base);
	// 8306321C: 3880005B  li r4, 0x5b
	ctx.r[4].s64 = 91;
	// 83063220: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83063224: 4BF8B08D  bl 0x82fee2b0
	ctx.lr = 0x83063228;
	sub_82FEE2B0(ctx, base);
	// 83063228: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8306322C: 40820010  bne 0x8306323c
	if !ctx.cr[0].eq {
	pc = 0x8306323C; continue 'dispatch;
	}
	// 83063230: 388000FC  li r4, 0xfc
	ctx.r[4].s64 = 252;
	// 83063234: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83063238: 4BF85651  bl 0x82fe8888
	ctx.lr = 0x8306323C;
	sub_82FE8888(ctx, base);
	// 8306323C: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83063240: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 83063244: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83063248: 615E8028  ori r30, r10, 0x8028
	ctx.r[30].u64 = ctx.r[10].u64 | 32808;
	// 8306324C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83063250: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83063254: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83063258: 7FABF02E  lwzx r29, r11, r30
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8306325C: 48000FED  bl 0x83064248
	ctx.lr = 0x83063260;
	sub_83064248(ctx, base);
	// 83063260: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83063264: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83063268: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8306326C: 4800026D  bl 0x830634d8
	ctx.lr = 0x83063270;
	sub_830634D8(ctx, base);
	// 83063270: 4800006C  b 0x830632dc
	pc = 0x830632DC; continue 'dispatch;
	// 83063274: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83063278: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8306327C: 388A7AFC  addi r4, r10, 0x7afc
	ctx.r[4].s64 = ctx.r[10].s64 + 31484;
	// 83063280: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83063284: 4BF9024D  bl 0x82ff34d0
	ctx.lr = 0x83063288;
	sub_82FF34D0(ctx, base);
	// 83063288: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8306328C: 41820084  beq 0x83063310
	if ctx.cr[0].eq {
	pc = 0x83063310; continue 'dispatch;
	}
	// 83063290: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83063294: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83063298: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8306329C: 48000FAD  bl 0x83064248
	ctx.lr = 0x830632A0;
	sub_83064248(ctx, base);
	// 830632A0: 3880005B  li r4, 0x5b
	ctx.r[4].s64 = 91;
	// 830632A4: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 830632A8: 4BF8B009  bl 0x82fee2b0
	ctx.lr = 0x830632AC;
	sub_82FEE2B0(ctx, base);
	// 830632AC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830632B0: 40820010  bne 0x830632c0
	if !ctx.cr[0].eq {
	pc = 0x830632C0; continue 'dispatch;
	}
	// 830632B4: 388000FC  li r4, 0xfc
	ctx.r[4].s64 = 252;
	// 830632B8: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830632BC: 4BF855CD  bl 0x82fe8888
	ctx.lr = 0x830632C0;
	sub_82FE8888(ctx, base);
	// 830632C0: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 830632C4: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 830632C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830632CC: 615E8028  ori r30, r10, 0x8028
	ctx.r[30].u64 = ctx.r[10].u64 | 32808;
	// 830632D0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830632D4: 7FABF02E  lwzx r29, r11, r30
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 830632D8: 4BFFC6C1  bl 0x8305f998
	ctx.lr = 0x830632DC;
	sub_8305F998(ctx, base);
	// 830632DC: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 830632E0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830632E4: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 830632E8: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 830632EC: 419A0128  beq cr6, 0x83063414
	if ctx.cr[6].eq {
	pc = 0x83063414; continue 'dispatch;
	}
	// 830632F0: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830632F4: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830632F8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830632FC: 41820118  beq 0x83063414
	if ctx.cr[0].eq {
	pc = 0x83063414; continue 'dispatch;
	}
	// 83063300: 38800056  li r4, 0x56
	ctx.r[4].s64 = 86;
	// 83063304: 806B00A8  lwz r3, 0xa8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 83063308: 4BFB40E1  bl 0x830173e8
	ctx.lr = 0x8306330C;
	sub_830173E8(ctx, base);
	// 8306330C: 48000108  b 0x83063414
	pc = 0x83063414; continue 'dispatch;
	// 83063310: 388000FB  li r4, 0xfb
	ctx.r[4].s64 = 251;
	// 83063314: 480000EC  b 0x83063400
	pc = 0x83063400; continue 'dispatch;
	// 83063318: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8306331C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83063320: 388A78E0  addi r4, r10, 0x78e0
	ctx.r[4].s64 = ctx.r[10].s64 + 30944;
	// 83063324: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83063328: 4BF901A9  bl 0x82ff34d0
	ctx.lr = 0x8306332C;
	sub_82FF34D0(ctx, base);
	// 8306332C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83063330: 41820010  beq 0x83063340
	if ctx.cr[0].eq {
	pc = 0x83063340; continue 'dispatch;
	}
	// 83063334: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83063338: 4BFFFA41  bl 0x83062d78
	ctx.lr = 0x8306333C;
	sub_83062D78(ctx, base);
	// 8306333C: 480000D8  b 0x83063414
	pc = 0x83063414; continue 'dispatch;
	// 83063340: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83063344: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83063348: 388A7940  addi r4, r10, 0x7940
	ctx.r[4].s64 = ctx.r[10].s64 + 31040;
	// 8306334C: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83063350: 4BF90181  bl 0x82ff34d0
	ctx.lr = 0x83063354;
	sub_82FF34D0(ctx, base);
	// 83063354: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83063358: 41820010  beq 0x83063368
	if ctx.cr[0].eq {
	pc = 0x83063368; continue 'dispatch;
	}
	// 8306335C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83063360: 4BFFDB21  bl 0x83060e80
	ctx.lr = 0x83063364;
	sub_83060E80(ctx, base);
	// 83063364: 480000B0  b 0x83063414
	pc = 0x83063414; continue 'dispatch;
	// 83063368: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8306336C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83063370: 388A797C  addi r4, r10, 0x797c
	ctx.r[4].s64 = ctx.r[10].s64 + 31100;
	// 83063374: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83063378: 4BF90159  bl 0x82ff34d0
	ctx.lr = 0x8306337C;
	sub_82FF34D0(ctx, base);
	// 8306337C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83063380: 41820010  beq 0x83063390
	if ctx.cr[0].eq {
	pc = 0x83063390; continue 'dispatch;
	}
	// 83063384: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83063388: 4BFFEEA1  bl 0x83062228
	ctx.lr = 0x8306338C;
	sub_83062228(ctx, base);
	// 8306338C: 48000088  b 0x83063414
	pc = 0x83063414; continue 'dispatch;
	// 83063390: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83063394: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 83063398: 388A7C10  addi r4, r10, 0x7c10
	ctx.r[4].s64 = ctx.r[10].s64 + 31760;
	// 8306339C: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830633A0: 4BF90131  bl 0x82ff34d0
	ctx.lr = 0x830633A4;
	sub_82FF34D0(ctx, base);
	// 830633A4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830633A8: 41820054  beq 0x830633fc
	if ctx.cr[0].eq {
	pc = 0x830633FC; continue 'dispatch;
	}
	// 830633AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830633B0: 48000F31  bl 0x830642e0
	ctx.lr = 0x830633B4;
	sub_830642E0(ctx, base);
	// 830633B4: 48000060  b 0x83063414
	pc = 0x83063414; continue 'dispatch;
	// 830633B8: 2B0B003F  cmplwi cr6, r11, 0x3f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 63 as u32, &mut ctx.xer);
	// 830633BC: 409A0040  bne cr6, 0x830633fc
	if !ctx.cr[6].eq {
	pc = 0x830633FC; continue 'dispatch;
	}
	// 830633C0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830633C4: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830633C8: 4BF86451  bl 0x82fe9818
	ctx.lr = 0x830633CC;
	sub_82FE9818(ctx, base);
	// 830633CC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830633D0: 41820020  beq 0x830633f0
	if ctx.cr[0].eq {
	pc = 0x830633F0; continue 'dispatch;
	}
	// 830633D4: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830633D8: 41820010  beq 0x830633e8
	if ctx.cr[0].eq {
	pc = 0x830633E8; continue 'dispatch;
	}
	// 830633DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830633E0: 4BFFD381  bl 0x83060760
	ctx.lr = 0x830633E4;
	sub_83060760(ctx, base);
	// 830633E4: 48000030  b 0x83063414
	pc = 0x83063414; continue 'dispatch;
	// 830633E8: 388000DC  li r4, 0xdc
	ctx.r[4].s64 = 220;
	// 830633EC: 48000014  b 0x83063400
	pc = 0x83063400; continue 'dispatch;
	// 830633F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830633F4: 4BFFCE05  bl 0x830601f8
	ctx.lr = 0x830633F8;
	sub_830601F8(ctx, base);
	// 830633F8: 4800001C  b 0x83063414
	pc = 0x83063414; continue 'dispatch;
	// 830633FC: 388000DB  li r4, 0xdb
	ctx.r[4].s64 = 219;
	// 83063400: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83063404: 4BF85485  bl 0x82fe8888
	ctx.lr = 0x83063408;
	sub_82FE8888(ctx, base);
	// 83063408: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 8306340C: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83063410: 4BFB4401  bl 0x83017810
	ctx.lr = 0x83063414;
	sub_83017810(ctx, base);
	// 83063414: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83063418: 48144DA4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83063420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83063420 size=176
    let mut pc: u32 = 0x83063420;
    'dispatch: loop {
        match pc {
            0x83063420 => {
    //   block [0x83063420..0x830634D0)
	// 83063420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83063424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83063428: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8306342C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83063430: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83063434: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83063438: 396B5C60  addi r11, r11, 0x5c60
	ctx.r[11].s64 = ctx.r[11].s64 + 23648;
	// 8306343C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83063440: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83063444: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83063448: 41820018  beq 0x83063460
	if ctx.cr[0].eq {
	pc = 0x83063460; continue 'dispatch;
	}
	// 8306344C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83063450: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83063454: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83063458: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8306345C: 4E800421  bctrl
	ctx.lr = 0x83063460;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83063460: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83063464: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83063468: 41820018  beq 0x83063480
	if ctx.cr[0].eq {
	pc = 0x83063480; continue 'dispatch;
	}
	// 8306346C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83063470: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83063474: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83063478: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8306347C: 4E800421  bctrl
	ctx.lr = 0x83063480;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83063480: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83063484: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83063488: 41820018  beq 0x830634a0
	if ctx.cr[0].eq {
	pc = 0x830634A0; continue 'dispatch;
	}
	// 8306348C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83063490: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83063494: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83063498: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8306349C: 4E800421  bctrl
	ctx.lr = 0x830634A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830634A0: 83FF0034  lwz r31, 0x34(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830634A4: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830634A8: 41820014  beq 0x830634bc
	if ctx.cr[0].eq {
	pc = 0x830634BC; continue 'dispatch;
	}
	// 830634AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830634B0: 4BFB9929  bl 0x8301cdd8
	ctx.lr = 0x830634B4;
	sub_8301CDD8(ctx, base);
	// 830634B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830634B8: 4BF74E29  bl 0x82fd82e0
	ctx.lr = 0x830634BC;
	sub_82FD82E0(ctx, base);
	// 830634BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830634C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830634C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830634C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830634CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830634D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830634D0 size=8
    let mut pc: u32 = 0x830634D0;
    'dispatch: loop {
        match pc {
            0x830634D0 => {
    //   block [0x830634D0..0x830634D8)
	// 830634D0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830634D4: 82166194  lwz r16, 0x6194(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(24980 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830634D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830634D8 size=1032
    let mut pc: u32 = 0x830634D8;
    'dispatch: loop {
        match pc {
            0x830634D8 => {
    //   block [0x830634D8..0x830638E0)
	// 830634D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830634DC: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 830634E0: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 830634E4: 48144C61  bl 0x831a8144
	ctx.lr = 0x830634E8;
	sub_831A8130(ctx, base);
	// 830634E8: 3BE1FF00  addi r31, r1, -0x100
	ctx.r[31].s64 = ctx.r[1].s64 + -256;
	// 830634EC: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830634F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830634F4: 98BF0127  stb r5, 0x127(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(295 as u32), ctx.r[5].u8 ) };
	// 830634F8: 7C942378  mr r20, r4
	ctx.r[20].u64 = ctx.r[4].u64;
	// 830634FC: 367E001C  addic. r19, r30, 0x1c
	ctx.xer.ca = (ctx.r[30].u32 > (!(28 as u32)));
	ctx.r[19].s64 = ctx.r[30].s64 + 28;
	ctx.cr[0].compare_i32(ctx.r[19].s32, 0, &mut ctx.xer);
	// 83063500: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 83063504: 93DF0114  stw r30, 0x114(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), ctx.r[30].u32 ) };
	// 83063508: 9A9F011F  stb r20, 0x11f(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(287 as u32), ctx.r[20].u8 ) };
	// 8306350C: 927F006C  stw r19, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[19].u32 ) };
	// 83063510: 41820010  beq 0x83063520
	if ctx.cr[0].eq {
	pc = 0x83063520; continue 'dispatch;
	}
	// 83063514: 89730000  lbz r11, 0(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 83063518: 9B330000  stb r25, 0(r19)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[19].u32.wrapping_add(0 as u32), ctx.r[25].u8 ) };
	// 8306351C: 997F0068  stb r11, 0x68(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u8 ) };
	// 83063520: 569C063E  clrlwi r28, r20, 0x18
	ctx.r[28].u64 = ctx.r[20].u32 as u64 & 0x000000FFu64;
	// 83063524: 83BE0028  lwz r29, 0x28(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83063528: 7F8B0034  cntlzw r11, r28
	ctx.r[11].u64 = if ctx.r[28].u32 == 0 { 32 } else { ctx.r[28].u32.leading_zeros() as u64 };
	// 8306352C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83063530: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83063534: 997F0052  stb r11, 0x52(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(82 as u32), ctx.r[11].u8 ) };
	// 83063538: 4BF7BAC1  bl 0x82fdeff8
	ctx.lr = 0x8306353C;
	sub_82FDEFF8(ctx, base);
	// 8306353C: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 83063540: 93BF0064  stw r29, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 83063544: 92FF0060  stw r23, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[23].u32 ) };
	// 83063548: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8306354C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83063550: 419A0020  beq cr6, 0x83063570
	if ctx.cr[6].eq {
	pc = 0x83063570; continue 'dispatch;
	}
	// 83063554: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83063558: 409A0018  bne cr6, 0x83063570
	if !ctx.cr[6].eq {
	pc = 0x83063570; continue 'dispatch;
	}
	// 8306355C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83063560: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83063564: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 83063568: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8306356C: 4E800421  bctrl
	ctx.lr = 0x83063570;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83063570: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83063574: 4BF8A0FD  bl 0x82fed670
	ctx.lr = 0x83063578;
	sub_82FED670(ctx, base);
	// 83063578: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8306357C: 41820034  beq 0x830635b0
	if ctx.cr[0].eq {
	pc = 0x830635B0; continue 'dispatch;
	}
	// 83063580: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83063584: 4BF8ADAD  bl 0x82fee330
	ctx.lr = 0x83063588;
	sub_82FEE330(ctx, base);
	// 83063588: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8306358C: 41820024  beq 0x830635b0
	if ctx.cr[0].eq {
	pc = 0x830635B0; continue 'dispatch;
	}
	// 83063590: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83063594: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83063598: 4BF86281  bl 0x82fe9818
	ctx.lr = 0x8306359C;
	sub_82FE9818(ctx, base);
	// 8306359C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830635A0: 41820010  beq 0x830635b0
	if ctx.cr[0].eq {
	pc = 0x830635B0; continue 'dispatch;
	}
	// 830635A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830635A8: 4BFFD1B9  bl 0x83060760
	ctx.lr = 0x830635AC;
	sub_83060760(ctx, base);
	// 830635AC: 9B3F0052  stb r25, 0x52(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(82 as u32), ctx.r[25].u8 ) };
	// 830635B0: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 830635B4: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 830635B8: 9B3F0050  stb r25, 0x50(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[25].u8 ) };
	// 830635BC: 615A8028  ori r26, r10, 0x8028
	ctx.r[26].u64 = ctx.r[10].u64 | 32808;
	// 830635C0: 9B3F0051  stb r25, 0x51(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(81 as u32), ctx.r[25].u8 ) };
	// 830635C4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830635C8: 7D6BD02E  lwzx r11, r11, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 830635CC: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 830635D0: 48000024  b 0x830635f4
	pc = 0x830635F4; continue 'dispatch;
	// 830635D4: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 830635D8: 83DF0114  lwz r30, 0x114(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 830635DC: 8A9F011F  lbz r20, 0x11f(r31)
	ctx.r[20].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(287 as u32) ) } as u64;
	// 830635E0: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 830635E4: 827F006C  lwz r19, 0x6c(r31)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 830635E8: 617A8028  ori r26, r11, 0x8028
	ctx.r[26].u64 = ctx.r[11].u64 | 32808;
	// 830635EC: 83BF0064  lwz r29, 0x64(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 830635F0: 82FF0060  lwz r23, 0x60(r31)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 830635F4: 9B3F0053  stb r25, 0x53(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(83 as u32), ctx.r[25].u8 ) };
	// 830635F8: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830635FC: 3AC00001  li r22, 1
	ctx.r[22].s64 = 1;
	// 83063600: 3AAB6148  addi r21, r11, 0x6148
	ctx.r[21].s64 = ctx.r[11].s64 + 24904;
	// 83063604: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 83063608: 61788054  ori r24, r11, 0x8054
	ctx.r[24].u64 = ctx.r[11].u64 | 32852;
	// 8306360C: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83063610: 4BF8AC39  bl 0x82fee248
	ctx.lr = 0x83063614;
	sub_82FEE248(ctx, base);
	// 83063614: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83063618: 547C043E  clrlwi r28, r3, 0x10
	ctx.r[28].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 8306361C: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83063620: 2B1C003C  cmplwi cr6, r28, 0x3c
	ctx.cr[6].compare_u32(ctx.r[28].u32, 60 as u32, &mut ctx.xer);
	// 83063624: 409A0094  bne cr6, 0x830636b8
	if !ctx.cr[6].eq {
	pc = 0x830636B8; continue 'dispatch;
	}
	// 83063628: 546B003E  slwi r11, r3, 0
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8306362C: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 83063630: 614A8050  ori r10, r10, 0x8050
	ctx.r[10].u64 = ctx.r[10].u64 | 32848;
	// 83063634: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83063638: 7F8BD02E  lwzx r28, r11, r26
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 8306363C: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83063640: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83063644: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83063648: 557B063E  clrlwi r27, r11, 0x18
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8306364C: 4BF8AAC5  bl 0x82fee110
	ctx.lr = 0x83063650;
	sub_82FEE110(ctx, base);
	// 83063650: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83063654: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83063658: 889F0052  lbz r4, 0x52(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(82 as u32) ) } as u64;
	// 8306365C: 9ADF0050  stb r22, 0x50(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[22].u8 ) };
	// 83063660: 4BFFFAF9  bl 0x83063158
	ctx.lr = 0x83063664;
	sub_83063158(ctx, base);
	// 83063664: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83063668: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 8306366C: 9B3F0050  stb r25, 0x50(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[25].u8 ) };
	// 83063670: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83063674: 7D6BD02E  lwzx r11, r11, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 83063678: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8306367C: 419A01F8  beq cr6, 0x83063874
	if ctx.cr[6].eq {
	pc = 0x83063874; continue 'dispatch;
	}
	// 83063680: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83063684: 41820014  beq 0x83063698
	if ctx.cr[0].eq {
	pc = 0x83063698; continue 'dispatch;
	}
	// 83063688: 3880012B  li r4, 0x12b
	ctx.r[4].s64 = 299;
	// 8306368C: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83063690: 4BF851F9  bl 0x82fe8888
	ctx.lr = 0x83063694;
	sub_82FE8888(ctx, base);
	// 83063694: 480001DC  b 0x83063870
	pc = 0x83063870; continue 'dispatch;
	// 83063698: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8306369C: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830636A0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830636A4: 418201D0  beq 0x83063874
	if ctx.cr[0].eq {
	pc = 0x83063874; continue 'dispatch;
	}
	// 830636A8: 38800056  li r4, 0x56
	ctx.r[4].s64 = 86;
	// 830636AC: 806B00A8  lwz r3, 0xa8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 830636B0: 4BFB3D39  bl 0x830173e8
	ctx.lr = 0x830636B4;
	sub_830173E8(ctx, base);
	// 830636B4: 480001BC  b 0x83063870
	pc = 0x83063870; continue 'dispatch;
	// 830636B8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830636BC: 7D6BC02E  lwzx r11, r11, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 830636C0: 7D6BE0AE  lbzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 830636C4: 556B0031  rlwinm. r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830636C8: 4182005C  beq 0x83063724
	if ctx.cr[0].eq {
	pc = 0x83063724; continue 'dispatch;
	}
	// 830636CC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830636D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830636D4: 419A0044  beq cr6, 0x83063718
	if ctx.cr[6].eq {
	pc = 0x83063718; continue 'dispatch;
	}
	// 830636D8: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 830636DC: 9ADF0051  stb r22, 0x51(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(81 as u32), ctx.r[22].u8 ) };
	// 830636E0: 4BF8AA99  bl 0x82fee178
	ctx.lr = 0x830636E4;
	sub_82FEE178(ctx, base);
	// 830636E4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830636E8: 80B70004  lwz r5, 4(r23)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 830636EC: 81570018  lwz r10, 0x18(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24 as u32) ) } as u64;
	// 830636F0: 54AB083C  slwi r11, r5, 1
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830636F4: 9B3F0051  stb r25, 0x51(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(81 as u32), ctx.r[25].u8 ) };
	// 830636F8: 7F2B532E  sthx r25, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[25].u16) };
	// 830636FC: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83063700: 80970018  lwz r4, 0x18(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24 as u32) ) } as u64;
	// 83063704: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83063708: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8306370C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83063710: 4E800421  bctrl
	ctx.lr = 0x83063714;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83063714: 4800015C  b 0x83063870
	pc = 0x83063870; continue 'dispatch;
	// 83063718: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8306371C: 4BF8AD05  bl 0x82fee420
	ctx.lr = 0x83063720;
	sub_82FEE420(ctx, base);
	// 83063720: 48000150  b 0x83063870
	pc = 0x83063870; continue 'dispatch;
	// 83063724: 2B1C0025  cmplwi cr6, r28, 0x25
	ctx.cr[6].compare_u32(ctx.r[28].u32, 37 as u32, &mut ctx.xer);
	// 83063728: 409A0028  bne cr6, 0x83063750
	if !ctx.cr[6].eq {
	pc = 0x83063750; continue 'dispatch;
	}
	// 8306372C: 4BF8A9E5  bl 0x82fee110
	ctx.lr = 0x83063730;
	sub_82FEE110(ctx, base);
	// 83063730: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83063734: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 83063738: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8306373C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83063740: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83063744: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83063748: 48000309  bl 0x83063a50
	ctx.lr = 0x8306374C;
	sub_83063A50(ctx, base);
	// 8306374C: 48000124  b 0x83063870
	pc = 0x83063870; continue 'dispatch;
	// 83063750: 568B063F  clrlwi. r11, r20, 0x18
	ctx.r[11].u64 = ctx.r[20].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83063754: 41820088  beq 0x830637dc
	if ctx.cr[0].eq {
	pc = 0x830637DC; continue 'dispatch;
	}
	// 83063758: 2B1C005D  cmplwi cr6, r28, 0x5d
	ctx.cr[6].compare_u32(ctx.r[28].u32, 93 as u32, &mut ctx.xer);
	// 8306375C: 409A0080  bne cr6, 0x830637dc
	if !ctx.cr[6].eq {
	pc = 0x830637DC; continue 'dispatch;
	}
	// 83063760: 4BF8A9B1  bl 0x82fee110
	ctx.lr = 0x83063764;
	sub_82FEE110(ctx, base);
	// 83063764: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83063768: 3880005D  li r4, 0x5d
	ctx.r[4].s64 = 93;
	// 8306376C: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83063770: 4BF8AB41  bl 0x82fee2b0
	ctx.lr = 0x83063774;
	sub_82FEE2B0(ctx, base);
	// 83063774: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83063778: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8306377C: 4182001C  beq 0x83063798
	if ctx.cr[0].eq {
	pc = 0x83063798; continue 'dispatch;
	}
	// 83063780: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 83063784: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83063788: 4BF8AB29  bl 0x82fee2b0
	ctx.lr = 0x8306378C;
	sub_82FEE2B0(ctx, base);
	// 8306378C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83063790: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83063794: 40820024  bne 0x830637b8
	if !ctx.cr[0].eq {
	pc = 0x830637B8; continue 'dispatch;
	}
	// 83063798: 388000FA  li r4, 0xfa
	ctx.r[4].s64 = 250;
	// 8306379C: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830637A0: 4BF850E9  bl 0x82fe8888
	ctx.lr = 0x830637A4;
	sub_82FE8888(ctx, base);
	// 830637A4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830637A8: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 830637AC: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 830637B0: 4BFB4061  bl 0x83017810
	ctx.lr = 0x830637B4;
	sub_83017810(ctx, base);
	// 830637B4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830637B8: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 830637BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830637C0: 4BF7B969  bl 0x82fdf128
	ctx.lr = 0x830637C4;
	sub_82FDF128(ctx, base);
	// 830637C4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830637C8: 2B130000  cmplwi cr6, r19, 0
	ctx.cr[6].compare_u32(ctx.r[19].u32, 0 as u32, &mut ctx.xer);
	// 830637CC: 419A010C  beq cr6, 0x830638d8
	if ctx.cr[6].eq {
	pc = 0x830638D8; continue 'dispatch;
	}
	// 830637D0: 897F0068  lbz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 830637D4: 99730000  stb r11, 0(r19)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[19].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 830637D8: 48000100  b 0x830638d8
	pc = 0x830638D8; continue 'dispatch;
	// 830637DC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 830637E0: 409A0018  bne cr6, 0x830637f8
	if !ctx.cr[6].eq {
	pc = 0x830637F8; continue 'dispatch;
	}
	// 830637E4: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 830637E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830637EC: 4BF7B93D  bl 0x82fdf128
	ctx.lr = 0x830637F0;
	sub_82FDF128(ctx, base);
	// 830637F0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830637F4: 4BFFFFD4  b 0x830637c8
	pc = 0x830637C8; continue 'dispatch;
	// 830637F8: 4BF8A919  bl 0x82fee110
	ctx.lr = 0x830637FC;
	sub_82FEE110(ctx, base);
	// 830637FC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83063800: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83063804: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83063808: 7D6BC02E  lwzx r11, r11, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 8306380C: 7D6BE0AE  lbzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 83063810: 556B0673  rlwinm. r11, r11, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83063814: 40820040  bne 0x83063854
	if !ctx.cr[0].eq {
	pc = 0x83063854; continue 'dispatch;
	}
	// 83063818: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8306381C: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83063820: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 83063824: 389F0070  addi r4, r31, 0x70
	ctx.r[4].s64 = ctx.r[31].s64 + 112;
	// 83063828: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8306382C: 4BF6E045  bl 0x82fd1870
	ctx.lr = 0x83063830;
	sub_82FD1870(ctx, base);
	// 83063830: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83063834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83063838: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8306383C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83063840: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83063844: 38BF0070  addi r5, r31, 0x70
	ctx.r[5].s64 = ctx.r[31].s64 + 112;
	// 83063848: 388000C4  li r4, 0xc4
	ctx.r[4].s64 = 196;
	// 8306384C: 4BF8518D  bl 0x82fe89d8
	ctx.lr = 0x83063850;
	sub_82FE89D8(ctx, base);
	// 83063850: 48000010  b 0x83063860
	pc = 0x83063860; continue 'dispatch;
	// 83063854: 388000BA  li r4, 0xba
	ctx.r[4].s64 = 186;
	// 83063858: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8306385C: 4BF8502D  bl 0x82fe8888
	ctx.lr = 0x83063860;
	sub_82FE8888(ctx, base);
	// 83063860: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83063864: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 83063868: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 8306386C: 4BF8ACED  bl 0x82fee558
	ctx.lr = 0x83063870;
	sub_82FEE558(ctx, base);
	// 83063870: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83063874: 9B3F0052  stb r25, 0x52(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(82 as u32), ctx.r[25].u8 ) };
	// 83063878: 4BFFFD94  b 0x8306360c
	pc = 0x8306360C; continue 'dispatch;
	// 8306387C: 897F0053  lbz r11, 0x53(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(83 as u32) ) } as u64;
	// 83063880: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83063884: 4182FD50  beq 0x830635d4
	if ctx.cr[0].eq {
	pc = 0x830635D4; continue 'dispatch;
	}
	// 83063888: 817F0114  lwz r11, 0x114(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 8306388C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83063890: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83063894: 419A0024  beq cr6, 0x830638b8
	if ctx.cr[6].eq {
	pc = 0x830638B8; continue 'dispatch;
	}
	// 83063898: 895F0127  lbz r10, 0x127(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(295 as u32) ) } as u64;
	// 8306389C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830638A0: 41820018  beq 0x830638b8
	if ctx.cr[0].eq {
	pc = 0x830638B8; continue 'dispatch;
	}
	// 830638A4: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830638A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830638AC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 830638B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830638B4: 4E800421  bctrl
	ctx.lr = 0x830638B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830638B8: 809F0060  lwz r4, 0x60(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 830638BC: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 830638C0: 4BF7B869  bl 0x82fdf128
	ctx.lr = 0x830638C4;
	sub_82FDF128(ctx, base);
	// 830638C4: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 830638C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830638CC: 419A000C  beq cr6, 0x830638d8
	if ctx.cr[6].eq {
	pc = 0x830638D8; continue 'dispatch;
	}
	// 830638D0: 895F0068  lbz r10, 0x68(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 830638D4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 830638D8: 383F0100  addi r1, r31, 0x100
	ctx.r[1].s64 = ctx.r[31].s64 + 256;
	// 830638DC: 481448B8  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830638E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830638E0 size=8
    let mut pc: u32 = 0x830638E0;
    'dispatch: loop {
        match pc {
            0x830638E0 => {
    //   block [0x830638E0..0x830638E8)
	// 830638E0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830638E4: 82166194  lwz r16, 0x6194(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(24980 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830638E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830638E8 size=188
    let mut pc: u32 = 0x830638E8;
    'dispatch: loop {
        match pc {
            0x830638E8 => {
    //   block [0x830638E8..0x830639A4)
	// 830638E8: 3BECFF00  addi r31, r12, -0x100
	ctx.r[31].s64 = ctx.r[12].s64 + -256;
	// 830638EC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830638F0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830638F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830638F8: 897F0050  lbz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830638FC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83063900: 41820020  beq 0x83063920
	if ctx.cr[0].eq {
	pc = 0x83063920; continue 'dispatch;
	}
	// 83063904: 83DF0114  lwz r30, 0x114(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 83063908: 388000E7  li r4, 0xe7
	ctx.r[4].s64 = 231;
	// 8306390C: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83063910: 4BF84F79  bl 0x82fe8888
	ctx.lr = 0x83063914;
	sub_82FE8888(ctx, base);
	// 83063914: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 83063918: 9B3F0050  stb r25, 0x50(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[25].u8 ) };
	// 8306391C: 4800000C  b 0x83063928
	pc = 0x83063928; continue 'dispatch;
	// 83063920: 83DF0114  lwz r30, 0x114(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 83063924: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 83063928: 897F0051  lbz r11, 0x51(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(81 as u32) ) } as u64;
	// 8306392C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83063930: 41820040  beq 0x83063970
	if ctx.cr[0].eq {
	pc = 0x83063970; continue 'dispatch;
	}
	// 83063934: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83063938: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8306393C: 419A0030  beq cr6, 0x8306396c
	if ctx.cr[6].eq {
	pc = 0x8306396C; continue 'dispatch;
	}
	// 83063940: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83063944: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83063948: 812B0018  lwz r9, 0x18(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8306394C: 54AA083C  slwi r10, r5, 1
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83063950: 7F2A4B2E  sthx r25, r10, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[25].u16) };
	// 83063954: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83063958: 808B0018  lwz r4, 0x18(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8306395C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83063960: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83063964: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83063968: 4E800421  bctrl
	ctx.lr = 0x8306396C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8306396C: 9B3F0051  stb r25, 0x51(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(81 as u32), ctx.r[25].u8 ) };
	// 83063970: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83063974: 815F0054  lwz r10, 0x54(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83063978: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8306397C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83063980: 409A000C  bne cr6, 0x8306398c
	if !ctx.cr[6].eq {
	pc = 0x8306398C; continue 'dispatch;
	}
	// 83063984: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83063988: 997F0053  stb r11, 0x53(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(83 as u32), ctx.r[11].u8 ) };
	// 8306398C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83063990: 3C608306  lis r3, -0x7cfa
	ctx.r[3].s64 = -2096758784;
	// 83063994: 3863387C  addi r3, r3, 0x387c
	ctx.r[3].s64 = ctx.r[3].s64 + 14460;
	// 83063998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8306399C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830639A0: 481447F4  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830639A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830639A4 size=40
    let mut pc: u32 = 0x830639A4;
    'dispatch: loop {
        match pc {
            0x830639A4 => {
    //   block [0x830639A4..0x830639CC)
	// 830639A4: 3BECFF00  addi r31, r12, -0x100
	ctx.r[31].s64 = ctx.r[12].s64 + -256;
	// 830639A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830639AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830639B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830639B4: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 830639B8: 4BFFD1A9  bl 0x83060b60
	ctx.lr = 0x830639BC;
	sub_83060B60(ctx, base);
	// 830639BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830639C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830639C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830639C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830639CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830639CC size=40
    let mut pc: u32 = 0x830639CC;
    'dispatch: loop {
        match pc {
            0x830639CC => {
    //   block [0x830639CC..0x830639F4)
	// 830639CC: 3BECFF00  addi r31, r12, -0x100
	ctx.r[31].s64 = ctx.r[12].s64 + -256;
	// 830639D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830639D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830639D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830639DC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830639E0: 4BF705A9  bl 0x82fd3f88
	ctx.lr = 0x830639E4;
	sub_82FD3F88(ctx, base);
	// 830639E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830639E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830639EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830639F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830639F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830639F8 size=76
    let mut pc: u32 = 0x830639F8;
    'dispatch: loop {
        match pc {
            0x830639F8 => {
    //   block [0x830639F8..0x83063A44)
	// 830639F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830639FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83063A00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83063A04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83063A08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83063A0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83063A10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83063A14: 4BFFFA0D  bl 0x83063420
	ctx.lr = 0x83063A18;
	sub_83063420(ctx, base);
	// 83063A18: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83063A1C: 4182000C  beq 0x83063a28
	if ctx.cr[0].eq {
	pc = 0x83063A28; continue 'dispatch;
	}
	// 83063A20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83063A24: 4BF748BD  bl 0x82fd82e0
	ctx.lr = 0x83063A28;
	sub_82FD82E0(ctx, base);
	// 83063A28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83063A2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83063A30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83063A34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83063A38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83063A3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83063A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83063A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83063A48 size=8
    let mut pc: u32 = 0x83063A48;
    'dispatch: loop {
        match pc {
            0x83063A48 => {
    //   block [0x83063A48..0x83063A50)
	// 83063A48: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83063A4C: 82166334  lwz r16, 0x6334(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(25396 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83063A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83063A50 size=1100
    let mut pc: u32 = 0x83063A50;
    'dispatch: loop {
        match pc {
            0x83063A50 => {
    //   block [0x83063A50..0x83063E9C)
	// 83063A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83063A54: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83063A58: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83063A5C: 481446F9  bl 0x831a8154
	ctx.lr = 0x83063A60;
	sub_831A8130(ctx, base);
	// 83063A60: 3BE1FF10  addi r31, r1, -0xf0
	ctx.r[31].s64 = ctx.r[1].s64 + -240;
	// 83063A64: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83063A68: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83063A6C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83063A70: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 83063A74: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 83063A78: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83063A7C: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83063A80: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 83063A84: 93DF0104  stw r30, 0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[30].u32 ) };
	// 83063A88: 9B6B000F  stb r27, 0xf(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(15 as u32), ctx.r[27].u8 ) };
	// 83063A8C: 82FE0028  lwz r23, 0x28(r30)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83063A90: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 83063A94: 4BF7B565  bl 0x82fdeff8
	ctx.lr = 0x83063A98;
	sub_82FDEFF8(ctx, base);
	// 83063A98: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83063A9C: 92FF0074  stw r23, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[23].u32 ) };
	// 83063AA0: 939F0070  stw r28, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[28].u32 ) };
	// 83063AA4: 897E001C  lbz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83063AA8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83063AAC: 41820018  beq 0x83063ac4
	if ctx.cr[0].eq {
	pc = 0x83063AC4; continue 'dispatch;
	}
	// 83063AB0: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83063AB4: 41820010  beq 0x83063ac4
	if ctx.cr[0].eq {
	pc = 0x83063AC4; continue 'dispatch;
	}
	// 83063AB8: 38800102  li r4, 0x102
	ctx.r[4].s64 = 258;
	// 83063ABC: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83063AC0: 4BF84DC9  bl 0x82fe8888
	ctx.lr = 0x83063AC4;
	sub_82FE8888(ctx, base);
	// 83063AC4: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83063AC8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83063ACC: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83063AD0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83063AD4: 937C0004  stw r27, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 83063AD8: B36A0000  sth r27, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[27].u16 ) };
	// 83063ADC: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83063AE0: 4BF8F629  bl 0x82ff3108
	ctx.lr = 0x83063AE4;
	sub_82FF3108(ctx, base);
	// 83063AE4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83063AE8: 40820030  bne 0x83063b18
	if !ctx.cr[0].eq {
	pc = 0x83063B18; continue 'dispatch;
	}
	// 83063AEC: 388000DE  li r4, 0xde
	ctx.r[4].s64 = 222;
	// 83063AF0: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83063AF4: 4BF84D95  bl 0x82fe8888
	ctx.lr = 0x83063AF8;
	sub_82FE8888(ctx, base);
	// 83063AF8: 3880003B  li r4, 0x3b
	ctx.r[4].s64 = 59;
	// 83063AFC: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83063B00: 4BF8A7B1  bl 0x82fee2b0
	ctx.lr = 0x83063B04;
	sub_82FEE2B0(ctx, base);
	// 83063B04: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83063B08: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 83063B0C: 4BF7B61D  bl 0x82fdf128
	ctx.lr = 0x83063B10;
	sub_82FDF128(ctx, base);
	// 83063B10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83063B14: 48000380  b 0x83063e94
	pc = 0x83063E94; continue 'dispatch;
	// 83063B18: 3880003B  li r4, 0x3b
	ctx.r[4].s64 = 59;
	// 83063B1C: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83063B20: 4BF8A791  bl 0x82fee2b0
	ctx.lr = 0x83063B24;
	sub_82FEE2B0(ctx, base);
	// 83063B24: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83063B28: 40820030  bne 0x83063b58
	if !ctx.cr[0].eq {
	pc = 0x83063B58; continue 'dispatch;
	}
	// 83063B2C: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83063B30: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83063B34: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83063B38: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83063B3C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83063B40: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83063B44: 388000E5  li r4, 0xe5
	ctx.r[4].s64 = 229;
	// 83063B48: 7F6B532E  sthx r27, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u16) };
	// 83063B4C: 80BC0018  lwz r5, 0x18(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83063B50: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83063B54: 4BF84E85  bl 0x82fe89d8
	ctx.lr = 0x83063B58;
	sub_82FE89D8(ctx, base);
	// 83063B58: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83063B5C: 38BF006C  addi r5, r31, 0x6c
	ctx.r[5].s64 = ctx.r[31].s64 + 108;
	// 83063B60: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83063B64: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83063B68: 7F6B532E  sthx r27, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u16) };
	// 83063B6C: 809C0018  lwz r4, 0x18(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83063B70: 807E0034  lwz r3, 0x34(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83063B74: 4BFBB675  bl 0x8301f1e8
	ctx.lr = 0x83063B78;
	sub_8301F1E8(ctx, base);
	// 83063B78: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83063B7C: 41820010  beq 0x83063b8c
	if ctx.cr[0].eq {
	pc = 0x83063B8C; continue 'dispatch;
	}
	// 83063B80: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83063B84: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83063B88: 40820084  bne 0x83063c0c
	if !ctx.cr[0].eq {
	pc = 0x83063C0C; continue 'dispatch;
	}
	// 83063B8C: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83063B90: 894B000E  lbz r10, 0xe(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(14 as u32) ) } as u64;
	// 83063B94: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83063B98: 41820034  beq 0x83063bcc
	if ctx.cr[0].eq {
	pc = 0x83063BCC; continue 'dispatch;
	}
	// 83063B9C: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83063BA0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83063BA4: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83063BA8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83063BAC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83063BB0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83063BB4: 388000E3  li r4, 0xe3
	ctx.r[4].s64 = 227;
	// 83063BB8: 7F6B532E  sthx r27, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u16) };
	// 83063BBC: 80BC0018  lwz r5, 0x18(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83063BC0: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83063BC4: 4BF84E15  bl 0x82fe89d8
	ctx.lr = 0x83063BC8;
	sub_82FE89D8(ctx, base);
	// 83063BC8: 48000040  b 0x83063c08
	pc = 0x83063C08; continue 'dispatch;
	// 83063BCC: 896B0010  lbz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83063BD0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83063BD4: 41820034  beq 0x83063c08
	if ctx.cr[0].eq {
	pc = 0x83063C08; continue 'dispatch;
	}
	// 83063BD8: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83063BDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83063BE0: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83063BE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83063BE8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83063BEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83063BF0: 38800055  li r4, 0x55
	ctx.r[4].s64 = 85;
	// 83063BF4: 7F6B532E  sthx r27, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u16) };
	// 83063BF8: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83063BFC: 80BC0018  lwz r5, 0x18(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83063C00: 806B00A8  lwz r3, 0xa8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 83063C04: 4BFB3955  bl 0x83017558
	ctx.lr = 0x83063C08;
	sub_83017558(ctx, base);
	// 83063C08: 4BFFFEFC  b 0x83063b04
	pc = 0x83063B04; continue 'dispatch;
	// 83063C0C: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83063C10: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83063C14: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83063C18: 4182005C  beq 0x83063c74
	if ctx.cr[0].eq {
	pc = 0x83063C74; continue 'dispatch;
	}
	// 83063C1C: 896B000E  lbz r11, 0xe(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(14 as u32) ) } as u64;
	// 83063C20: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83063C24: 41820050  beq 0x83063c74
	if ctx.cr[0].eq {
	pc = 0x83063C74; continue 'dispatch;
	}
	// 83063C28: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83063C2C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83063C30: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83063C34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83063C38: 4E800421  bctrl
	ctx.lr = 0x83063C3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83063C3C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83063C40: 40820034  bne 0x83063c74
	if !ctx.cr[0].eq {
	pc = 0x83063C74; continue 'dispatch;
	}
	// 83063C44: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83063C48: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83063C4C: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83063C50: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83063C54: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83063C58: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83063C5C: 38800051  li r4, 0x51
	ctx.r[4].s64 = 81;
	// 83063C60: 7F6B532E  sthx r27, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u16) };
	// 83063C64: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83063C68: 80BC0018  lwz r5, 0x18(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83063C6C: 806B00A8  lwz r3, 0xa8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 83063C70: 4BFB38E9  bl 0x83017558
	ctx.lr = 0x83063C74;
	sub_83017558(ctx, base);
	// 83063C74: 80DD0018  lwz r6, 0x18(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83063C78: 28060000  cmplwi r6, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83063C7C: 40820010  bne 0x83063c8c
	if !ctx.cr[0].eq {
	pc = 0x83063C8C; continue 'dispatch;
	}
	// 83063C80: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 83063C84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83063C88: 419A0008  beq cr6, 0x83063c90
	if ctx.cr[6].eq {
	pc = 0x83063C90; continue 'dispatch;
	}
	// 83063C8C: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 83063C90: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83063C94: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83063C98: 574B063E  clrlwi r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	// 83063C9C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83063CA0: 41820190  beq 0x83063e30
	if ctx.cr[0].eq {
	pc = 0x83063E30; continue 'dispatch;
	}
	// 83063CA4: 5568DFFE  rlwinm r8, r11, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83063CA8: 80BD001C  lwz r5, 0x1c(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 83063CAC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83063CB0: 809D0020  lwz r4, 0x20(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 83063CB4: 3B7F0060  addi r27, r31, 0x60
	ctx.r[27].s64 = ctx.r[31].s64 + 96;
	// 83063CB8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83063CBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83063CC0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83063CC4: 9961005F  stb r11, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[11].u8 ) };
	// 83063CC8: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 83063CCC: 4BF8B2AD  bl 0x82feef78
	ctx.lr = 0x83063CD0;
	sub_82FEEF78(ctx, base);
	// 83063CD0: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83063CD4: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83063CD8: 907F0064  stw r3, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 83063CDC: 40820054  bne 0x83063d30
	if !ctx.cr[0].eq {
	pc = 0x83063D30; continue 'dispatch;
	}
	// 83063CE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83063CE4: 83DE0004  lwz r30, 4(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83063CE8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83063CEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83063CF0: 4E800421  bctrl
	ctx.lr = 0x83063CF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83063CF4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83063CF8: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 83063CFC: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 83063D00: 388B5790  addi r4, r11, 0x5790
	ctx.r[4].s64 = ctx.r[11].s64 + 22416;
	// 83063D04: 38C0002E  li r6, 0x2e
	ctx.r[6].s64 = 46;
	// 83063D08: 38A001D1  li r5, 0x1d1
	ctx.r[5].s64 = 465;
	// 83063D0C: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83063D10: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83063D14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83063D18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83063D1C: 4BFB3A65  bl 0x83017780
	ctx.lr = 0x83063D20;
	sub_83017780(ctx, base);
	// 83063D20: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83063D24: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83063D28: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 83063D2C: 4814CEFD  bl 0x831b0c28
	ctx.lr = 0x83063D30;
	sub_831B0C28(ctx, base);
	// 83063D30: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 83063D34: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83063D38: 616B8049  ori r11, r11, 0x8049
	ctx.r[11].u64 = ctx.r[11].u64 | 32841;
	// 83063D3C: 7F2459AE  stbx r25, r4, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[4].u32.wrapping_add(ctx.r[11].u32), ctx.r[25].u8) };
	// 83063D40: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83063D44: 4BF8A965  bl 0x82fee6a8
	ctx.lr = 0x83063D48;
	sub_82FEE6A8(ctx, base);
	// 83063D48: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83063D4C: 40820030  bne 0x83063d7c
	if !ctx.cr[0].eq {
	pc = 0x83063D7C; continue 'dispatch;
	}
	// 83063D50: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83063D54: 80BD0010  lwz r5, 0x10(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 83063D58: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83063D5C: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83063D60: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83063D64: 388000E6  li r4, 0xe6
	ctx.r[4].s64 = 230;
	// 83063D68: 4BF84C71  bl 0x82fe89d8
	ctx.lr = 0x83063D6C;
	sub_82FE89D8(ctx, base);
	// 83063D6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83063D70: 387F0064  addi r3, r31, 0x64
	ctx.r[3].s64 = ctx.r[31].s64 + 100;
	// 83063D74: 48004F65  bl 0x83068cd8
	ctx.lr = 0x83063D78;
	sub_83068CD8(ctx, base);
	// 83063D78: 4BFFFD8C  b 0x83063b04
	pc = 0x83063B04; continue 'dispatch;
	// 83063D7C: 570B063F  clrlwi. r11, r24, 0x18
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83063D80: 41820084  beq 0x83063e04
	if ctx.cr[0].eq {
	pc = 0x83063E04; continue 'dispatch;
	}
	// 83063D84: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83063D88: 83AB0064  lwz r29, 0x64(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 83063D8C: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83063D90: 93BF006C  stw r29, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[29].u32 ) };
	// 83063D94: 4182001C  beq 0x83063db0
	if ctx.cr[0].eq {
	pc = 0x83063DB0; continue 'dispatch;
	}
	// 83063D98: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83063D9C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83063DA0: 809F0060  lwz r4, 0x60(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83063DA4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83063DA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83063DAC: 4E800421  bctrl
	ctx.lr = 0x83063DB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83063DB0: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83063DB4: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 83063DB8: 614A8028  ori r10, r10, 0x8028
	ctx.r[10].u64 = ctx.r[10].u64 | 32808;
	// 83063DBC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83063DC0: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83063DC4: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 83063DC8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83063DCC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83063DD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83063DD4: 4BFFF705  bl 0x830634d8
	ctx.lr = 0x83063DD8;
	sub_830634D8(ctx, base);
	// 83063DD8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83063DDC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83063DE0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83063DE4: 419A003C  beq cr6, 0x83063e20
	if ctx.cr[6].eq {
	pc = 0x83063E20; continue 'dispatch;
	}
	// 83063DE8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83063DEC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83063DF0: 809F0060  lwz r4, 0x60(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83063DF4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83063DF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83063DFC: 4E800421  bctrl
	ctx.lr = 0x83063E00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83063E00: 48000020  b 0x83063e20
	pc = 0x83063E20; continue 'dispatch;
	// 83063E04: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83063E08: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83063E0C: 4BF85A0D  bl 0x82fe9818
	ctx.lr = 0x83063E10;
	sub_82FE9818(ctx, base);
	// 83063E10: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83063E14: 4182000C  beq 0x83063e20
	if ctx.cr[0].eq {
	pc = 0x83063E20; continue 'dispatch;
	}
	// 83063E18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83063E1C: 4BFFC945  bl 0x83060760
	ctx.lr = 0x83063E20;
	sub_83060760(ctx, base);
	// 83063E20: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83063E24: 387F0064  addi r3, r31, 0x64
	ctx.r[3].s64 = ctx.r[31].s64 + 100;
	// 83063E28: 48004EB1  bl 0x83068cd8
	ctx.lr = 0x83063E2C;
	sub_83068CD8(ctx, base);
	// 83063E2C: 48000058  b 0x83063e84
	pc = 0x83063E84; continue 'dispatch;
	// 83063E30: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83063E34: 811D0008  lwz r8, 8(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83063E38: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83063E3C: 80FD000C  lwz r7, 0xc(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 83063E40: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83063E44: 809D0010  lwz r4, 0x10(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 83063E48: 5565DFFE  rlwinm r5, r11, 0x1b, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83063E4C: 4BF89695  bl 0x82fed4e0
	ctx.lr = 0x83063E50;
	sub_82FED4E0(ctx, base);
	// 83063E50: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83063E54: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83063E58: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83063E5C: 4BF8A84D  bl 0x82fee6a8
	ctx.lr = 0x83063E60;
	sub_82FEE6A8(ctx, base);
	// 83063E60: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83063E64: 40820020  bne 0x83063e84
	if !ctx.cr[0].eq {
	pc = 0x83063E84; continue 'dispatch;
	}
	// 83063E68: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83063E6C: 80BD0010  lwz r5, 0x10(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 83063E70: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83063E74: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83063E78: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83063E7C: 388000E6  li r4, 0xe6
	ctx.r[4].s64 = 230;
	// 83063E80: 4BF84B59  bl 0x82fe89d8
	ctx.lr = 0x83063E84;
	sub_82FE89D8(ctx, base);
	// 83063E84: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83063E88: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 83063E8C: 4BF7B29D  bl 0x82fdf128
	ctx.lr = 0x83063E90;
	sub_82FDF128(ctx, base);
	// 83063E90: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83063E94: 383F00F0  addi r1, r31, 0xf0
	ctx.r[1].s64 = ctx.r[31].s64 + 240;
	// 83063E98: 4814430C  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83063E9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83063E9C size=8
    let mut pc: u32 = 0x83063E9C;
    'dispatch: loop {
        match pc {
            0x83063E9C => {
    //   block [0x83063E9C..0x83063EA4)
	// 83063E9C: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83063EA0: 82166334  lwz r16, 0x6334(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(25396 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83063EA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83063EA4 size=24
    let mut pc: u32 = 0x83063EA4;
    'dispatch: loop {
        match pc {
            0x83063EA4 => {
    //   block [0x83063EA4..0x83063EBC)
	// 83063EA4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83063EA8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83063EAC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83063EB0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83063EB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83063EB8: 4814CD71  bl 0x831b0c28
	ctx.lr = 0x83063EBC;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83063EC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83063EC4 size=76
    let mut pc: u32 = 0x83063EC4;
    'dispatch: loop {
        match pc {
            0x83063EC4 => {
    //   block [0x83063EC4..0x83063F10)
	// 83063EC4: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83063EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83063ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83063ED0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83063ED4: 817F0104  lwz r11, 0x104(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 83063ED8: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 83063EDC: 806B002C  lwz r3, 0x2c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83063EE0: 4BF89F29  bl 0x82fede08
	ctx.lr = 0x83063EE4;
	sub_82FEDE08(ctx, base);
	// 83063EE4: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 83063EE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83063EEC: 419A0018  beq cr6, 0x83063f04
	if ctx.cr[6].eq {
	pc = 0x83063F04; continue 'dispatch;
	}
	// 83063EF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83063EF4: 809F0060  lwz r4, 0x60(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83063EF8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83063EFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83063F00: 4E800421  bctrl
	ctx.lr = 0x83063F04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83063F04: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83063F08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83063F0C: 4814CD1D  bl 0x831b0c28
	ctx.lr = 0x83063F10;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83063F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83063F10 size=40
    let mut pc: u32 = 0x83063F10;
    'dispatch: loop {
        match pc {
            0x83063F10 => {
    //   block [0x83063F10..0x83063F38)
	// 83063F10: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83063F14: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83063F18: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83063F1C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83063F20: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83063F24: 4BF70065  bl 0x82fd3f88
	ctx.lr = 0x83063F28;
	sub_82FD3F88(ctx, base);
	// 83063F28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83063F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83063F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83063F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83063F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83063F38 size=40
    let mut pc: u32 = 0x83063F38;
    'dispatch: loop {
        match pc {
            0x83063F38 => {
    //   block [0x83063F38..0x83063F60)
	// 83063F38: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83063F3C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83063F40: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83063F44: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83063F48: 387F0064  addi r3, r31, 0x64
	ctx.r[3].s64 = ctx.r[31].s64 + 100;
	// 83063F4C: 4BFB483D  bl 0x83018788
	ctx.lr = 0x83063F50;
	sub_83018788(ctx, base);
	// 83063F50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83063F54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83063F58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83063F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83063F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83063F60 size=8
    let mut pc: u32 = 0x83063F60;
    'dispatch: loop {
        match pc {
            0x83063F60 => {
    //   block [0x83063F60..0x83063F68)
	// 83063F60: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83063F64: 821663F8  lwz r16, 0x63f8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(25592 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83063F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83063F68 size=656
    let mut pc: u32 = 0x83063F68;
    'dispatch: loop {
        match pc {
            0x83063F68 => {
    //   block [0x83063F68..0x830641F8)
	// 83063F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83063F6C: 481441DD  bl 0x831a8148
	ctx.lr = 0x83063F70;
	sub_831A8130(ctx, base);
	// 83063F70: 3BE1FF20  addi r31, r1, -0xe0
	ctx.r[31].s64 = ctx.r[1].s64 + -224;
	// 83063F74: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83063F78: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83063F7C: 3AC00001  li r22, 1
	ctx.r[22].s64 = 1;
	// 83063F80: 36FE001C  addic. r23, r30, 0x1c
	ctx.xer.ca = (ctx.r[30].u32 > (!(28 as u32)));
	ctx.r[23].s64 = ctx.r[30].s64 + 28;
	ctx.cr[0].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 83063F84: 92FF0054  stw r23, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[23].u32 ) };
	// 83063F88: 41820014  beq 0x83063f9c
	if ctx.cr[0].eq {
	pc = 0x83063F9C; continue 'dispatch;
	}
	// 83063F8C: 8AB70000  lbz r21, 0(r23)
	ctx.r[21].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 83063F90: 9AD70000  stb r22, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[22].u8 ) };
	// 83063F94: 9ABF0050  stb r21, 0x50(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[21].u8 ) };
	// 83063F98: 48000008  b 0x83063fa0
	pc = 0x83063FA0; continue 'dispatch;
	// 83063F9C: 8ABF0050  lbz r21, 0x50(r31)
	ctx.r[21].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83063FA0: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83063FA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83063FA8: 419A0018  beq cr6, 0x83063fc0
	if ctx.cr[6].eq {
	pc = 0x83063FC0; continue 'dispatch;
	}
	// 83063FAC: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83063FB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83063FB4: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 83063FB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83063FBC: 4E800421  bctrl
	ctx.lr = 0x83063FC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83063FC0: 833E0028  lwz r25, 0x28(r30)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83063FC4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83063FC8: 4BF7B031  bl 0x82fdeff8
	ctx.lr = 0x83063FCC;
	sub_82FDEFF8(ctx, base);
	// 83063FCC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83063FD0: 933F005C  stw r25, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[25].u32 ) };
	// 83063FD4: 937F0058  stw r27, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 83063FD8: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83063FDC: 4BF8A26D  bl 0x82fee248
	ctx.lr = 0x83063FE0;
	sub_82FEE248(ctx, base);
	// 83063FE0: 547D043F  clrlwi. r29, r3, 0x10
	ctx.r[29].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83063FE4: 418201A0  beq 0x83064184
	if ctx.cr[0].eq {
	pc = 0x83064184; continue 'dispatch;
	}
	// 83063FE8: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83063FEC: 3B0B6148  addi r24, r11, 0x6148
	ctx.r[24].s64 = ctx.r[11].s64 + 24904;
	// 83063FF0: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 83063FF4: 617A8028  ori r26, r11, 0x8028
	ctx.r[26].u64 = ctx.r[11].u64 | 32808;
	// 83063FF8: 2B1D005D  cmplwi cr6, r29, 0x5d
	ctx.cr[6].compare_u32(ctx.r[29].u32, 93 as u32, &mut ctx.xer);
	// 83063FFC: 419A01A8  beq cr6, 0x830641a4
	if ctx.cr[6].eq {
	pc = 0x830641A4; continue 'dispatch;
	}
	// 83064000: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83064004: 2B1D0025  cmplwi cr6, r29, 0x25
	ctx.cr[6].compare_u32(ctx.r[29].u32, 37 as u32, &mut ctx.xer);
	// 83064008: 409A0024  bne cr6, 0x8306402c
	if !ctx.cr[6].eq {
	pc = 0x8306402C; continue 'dispatch;
	}
	// 8306400C: 4BF8A105  bl 0x82fee110
	ctx.lr = 0x83064010;
	sub_82FEE110(ctx, base);
	// 83064010: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 83064014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83064018: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8306401C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83064020: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83064024: 4BFFFA2D  bl 0x83063a50
	ctx.lr = 0x83064028;
	sub_83063A50(ctx, base);
	// 83064028: 4800014C  b 0x83064174
	pc = 0x83064174; continue 'dispatch;
	// 8306402C: 2B1D003C  cmplwi cr6, r29, 0x3c
	ctx.cr[6].compare_u32(ctx.r[29].u32, 60 as u32, &mut ctx.xer);
	// 83064030: 409A0084  bne cr6, 0x830640b4
	if !ctx.cr[6].eq {
	pc = 0x830640B4; continue 'dispatch;
	}
	// 83064034: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83064038: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 8306403C: 614A8050  ori r10, r10, 0x8050
	ctx.r[10].u64 = ctx.r[10].u64 | 32848;
	// 83064040: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83064044: 7FABD02E  lwzx r29, r11, r26
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 83064048: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8306404C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83064050: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83064054: 557C063E  clrlwi r28, r11, 0x18
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83064058: 4BF8A0B9  bl 0x82fee110
	ctx.lr = 0x8306405C;
	sub_82FEE110(ctx, base);
	// 8306405C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83064060: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83064064: 4BFFF0F5  bl 0x83063158
	ctx.lr = 0x83064068;
	sub_83063158(ctx, base);
	// 83064068: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 8306406C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83064070: 7D6BD02E  lwzx r11, r11, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 83064074: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83064078: 419A00FC  beq cr6, 0x83064174
	if ctx.cr[6].eq {
	pc = 0x83064174; continue 'dispatch;
	}
	// 8306407C: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83064080: 41820014  beq 0x83064094
	if ctx.cr[0].eq {
	pc = 0x83064094; continue 'dispatch;
	}
	// 83064084: 3880012B  li r4, 0x12b
	ctx.r[4].s64 = 299;
	// 83064088: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8306408C: 4BF847FD  bl 0x82fe8888
	ctx.lr = 0x83064090;
	sub_82FE8888(ctx, base);
	// 83064090: 480000E4  b 0x83064174
	pc = 0x83064174; continue 'dispatch;
	// 83064094: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83064098: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8306409C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830640A0: 418200D4  beq 0x83064174
	if ctx.cr[0].eq {
	pc = 0x83064174; continue 'dispatch;
	}
	// 830640A4: 38800056  li r4, 0x56
	ctx.r[4].s64 = 86;
	// 830640A8: 806B00A8  lwz r3, 0xa8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 830640AC: 4BFB333D  bl 0x830173e8
	ctx.lr = 0x830640B0;
	sub_830173E8(ctx, base);
	// 830640B0: 480000C4  b 0x83064174
	pc = 0x83064174; continue 'dispatch;
	// 830640B4: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 830640B8: 616A8054  ori r10, r11, 0x8054
	ctx.r[10].u64 = ctx.r[11].u64 | 32852;
	// 830640BC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830640C0: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830640C4: 7D6BE8AE  lbzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 830640C8: 556B0031  rlwinm. r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830640CC: 41820054  beq 0x83064120
	if ctx.cr[0].eq {
	pc = 0x83064120; continue 'dispatch;
	}
	// 830640D0: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830640D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830640D8: 419A003C  beq cr6, 0x83064114
	if ctx.cr[6].eq {
	pc = 0x83064114; continue 'dispatch;
	}
	// 830640DC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830640E0: 4BF8A099  bl 0x82fee178
	ctx.lr = 0x830640E4;
	sub_82FEE178(ctx, base);
	// 830640E4: 80BB0004  lwz r5, 4(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 830640E8: 815B0018  lwz r10, 0x18(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 830640EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830640F0: 54AB083C  slwi r11, r5, 1
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830640F4: 7D2B532E  sthx r9, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u16) };
	// 830640F8: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830640FC: 809B0018  lwz r4, 0x18(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 83064100: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83064104: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83064108: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8306410C: 4E800421  bctrl
	ctx.lr = 0x83064110;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83064110: 48000064  b 0x83064174
	pc = 0x83064174; continue 'dispatch;
	// 83064114: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83064118: 4BF8A309  bl 0x82fee420
	ctx.lr = 0x8306411C;
	sub_82FEE420(ctx, base);
	// 8306411C: 48000058  b 0x83064174
	pc = 0x83064174; continue 'dispatch;
	// 83064120: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83064124: 3A9F0060  addi r20, r31, 0x60
	ctx.r[20].s64 = ctx.r[31].s64 + 96;
	// 83064128: 4BF89FE9  bl 0x82fee110
	ctx.lr = 0x8306412C;
	sub_82FEE110(ctx, base);
	// 8306412C: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 83064130: 5463043E  clrlwi r3, r3, 0x10
	ctx.r[3].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 83064134: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 83064138: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8306413C: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 83064140: 4BF6D731  bl 0x82fd1870
	ctx.lr = 0x83064144;
	sub_82FD1870(ctx, base);
	// 83064144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83064148: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8306414C: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83064150: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83064154: 38BF0060  addi r5, r31, 0x60
	ctx.r[5].s64 = ctx.r[31].s64 + 96;
	// 83064158: 388000D5  li r4, 0xd5
	ctx.r[4].s64 = 213;
	// 8306415C: 4BF8487D  bl 0x82fe89d8
	ctx.lr = 0x83064160;
	sub_82FE89D8(ctx, base);
	// 83064160: 2B1D003E  cmplwi cr6, r29, 0x3e
	ctx.cr[6].compare_u32(ctx.r[29].u32, 62 as u32, &mut ctx.xer);
	// 83064164: 419A004C  beq cr6, 0x830641b0
	if ctx.cr[6].eq {
	pc = 0x830641B0; continue 'dispatch;
	}
	// 83064168: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8306416C: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83064170: 4BF8A3E9  bl 0x82fee558
	ctx.lr = 0x83064174;
	sub_82FEE558(ctx, base);
	// 83064174: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83064178: 4BF8A0D1  bl 0x82fee248
	ctx.lr = 0x8306417C;
	sub_82FEE248(ctx, base);
	// 8306417C: 547D043F  clrlwi. r29, r3, 0x10
	ctx.r[29].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83064180: 4082FE78  bne 0x83063ff8
	if !ctx.cr[0].eq {
	pc = 0x83063FF8; continue 'dispatch;
	}
	// 83064184: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83064188: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8306418C: 4BF7AF9D  bl 0x82fdf128
	ctx.lr = 0x83064190;
	sub_82FDF128(ctx, base);
	// 83064190: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 83064194: 419A0008  beq cr6, 0x8306419c
	if ctx.cr[6].eq {
	pc = 0x8306419C; continue 'dispatch;
	}
	// 83064198: 9AB70000  stb r21, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[21].u8 ) };
	// 8306419C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830641A0: 48000050  b 0x830641f0
	pc = 0x830641F0; continue 'dispatch;
	// 830641A4: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 830641A8: 4BF89F69  bl 0x82fee110
	ctx.lr = 0x830641AC;
	sub_82FEE110(ctx, base);
	// 830641AC: 48000008  b 0x830641b4
	pc = 0x830641B4; continue 'dispatch;
	// 830641B0: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 830641B4: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830641B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830641BC: 419A0018  beq cr6, 0x830641d4
	if ctx.cr[6].eq {
	pc = 0x830641D4; continue 'dispatch;
	}
	// 830641C0: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 830641C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830641C8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830641CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830641D0: 4E800421  bctrl
	ctx.lr = 0x830641D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830641D4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830641D8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 830641DC: 4BF7AF4D  bl 0x82fdf128
	ctx.lr = 0x830641E0;
	sub_82FDF128(ctx, base);
	// 830641E0: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 830641E4: 419A0008  beq cr6, 0x830641ec
	if ctx.cr[6].eq {
	pc = 0x830641EC; continue 'dispatch;
	}
	// 830641E8: 9AB70000  stb r21, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[21].u8 ) };
	// 830641EC: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 830641F0: 383F00E0  addi r1, r31, 0xe0
	ctx.r[1].s64 = ctx.r[31].s64 + 224;
	// 830641F4: 48143FA4  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830641F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830641F8 size=40
    let mut pc: u32 = 0x830641F8;
    'dispatch: loop {
        match pc {
            0x830641F8 => {
    //   block [0x830641F8..0x83064220)
	// 830641F8: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 830641FC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83064200: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83064204: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83064208: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8306420C: 4BFFC955  bl 0x83060b60
	ctx.lr = 0x83064210;
	sub_83060B60(ctx, base);
	// 83064210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83064214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83064218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8306421C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83064220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83064220 size=40
    let mut pc: u32 = 0x83064220;
    'dispatch: loop {
        match pc {
            0x83064220 => {
    //   block [0x83064220..0x83064248)
	// 83064220: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 83064224: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83064228: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8306422C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83064230: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 83064234: 4BF6FD55  bl 0x82fd3f88
	ctx.lr = 0x83064238;
	sub_82FD3F88(ctx, base);
	// 83064238: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8306423C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83064240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83064244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83064248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83064248 size=140
    let mut pc: u32 = 0x83064248;
    'dispatch: loop {
        match pc {
            0x83064248 => {
    //   block [0x83064248..0x830642D4)
	// 83064248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306424C: 48143F1D  bl 0x831a8168
	ctx.lr = 0x83064250;
	sub_831A8130(ctx, base);
	// 83064250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83064254: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83064258: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8306425C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83064260: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83064264: 48000030  b 0x83064294
	pc = 0x83064294; continue 'dispatch;
	// 83064268: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8306426C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83064270: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83064274: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83064278: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8306427C: 4BFFF7D5  bl 0x83063a50
	ctx.lr = 0x83064280;
	sub_83063A50(ctx, base);
	// 83064280: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83064284: 40820010  bne 0x83064294
	if !ctx.cr[0].eq {
	pc = 0x83064294; continue 'dispatch;
	}
	// 83064288: 388000E2  li r4, 0xe2
	ctx.r[4].s64 = 226;
	// 8306428C: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83064290: 4BF845F9  bl 0x82fe8888
	ctx.lr = 0x83064294;
	sub_82FE8888(ctx, base);
	// 83064294: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83064298: 4BF8A099  bl 0x82fee330
	ctx.lr = 0x8306429C;
	sub_82FEE330(ctx, base);
	// 8306429C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830642A0: 41820014  beq 0x830642b4
	if ctx.cr[0].eq {
	pc = 0x830642B4; continue 'dispatch;
	}
	// 830642A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830642A8: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 830642AC: 4BF8A175  bl 0x82fee420
	ctx.lr = 0x830642B0;
	sub_82FEE420(ctx, base);
	// 830642B0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 830642B4: 38800025  li r4, 0x25
	ctx.r[4].s64 = 37;
	// 830642B8: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 830642BC: 4BF89FF5  bl 0x82fee2b0
	ctx.lr = 0x830642C0;
	sub_82FEE2B0(ctx, base);
	// 830642C0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830642C4: 4082FFA4  bne 0x83064268
	if !ctx.cr[0].eq {
	pc = 0x83064268; continue 'dispatch;
	}
	// 830642C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830642CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830642D0: 48143EE8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830642D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830642D8 size=8
    let mut pc: u32 = 0x830642D8;
    'dispatch: loop {
        match pc {
            0x830642D8 => {
    //   block [0x830642D8..0x830642E0)
	// 830642D8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830642DC: 82166468  lwz r16, 0x6468(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(25704 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830642E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830642E0 size=892
    let mut pc: u32 = 0x830642E0;
    'dispatch: loop {
        match pc {
            0x830642E0 => {
    //   block [0x830642E0..0x8306465C)
	// 830642E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830642E4: 48143E65  bl 0x831a8148
	ctx.lr = 0x830642E8;
	sub_831A8130(ctx, base);
	// 830642E8: 3BE1FF10  addi r31, r1, -0xf0
	ctx.r[31].s64 = ctx.r[1].s64 + -240;
	// 830642EC: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830642F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830642F4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830642F8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830642FC: 93DF0104  stw r30, 0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[30].u32 ) };
	// 83064300: 4BFFFF49  bl 0x83064248
	ctx.lr = 0x83064304;
	sub_83064248(ctx, base);
	// 83064304: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83064308: 40820020  bne 0x83064328
	if !ctx.cr[0].eq {
	pc = 0x83064328; continue 'dispatch;
	}
	// 8306430C: 388000CF  li r4, 0xcf
	ctx.r[4].s64 = 207;
	// 83064310: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83064314: 4BF84575  bl 0x82fe8888
	ctx.lr = 0x83064318;
	sub_82FE8888(ctx, base);
	// 83064318: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 8306431C: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83064320: 4BFB34F1  bl 0x83017810
	ctx.lr = 0x83064324;
	sub_83017810(ctx, base);
	// 83064324: 48000330  b 0x83064654
	pc = 0x83064654; continue 'dispatch;
	// 83064328: 829E0028  lwz r20, 0x28(r30)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 8306432C: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 83064330: 4BF7ACC9  bl 0x82fdeff8
	ctx.lr = 0x83064334;
	sub_82FDEFF8(ctx, base);
	// 83064334: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83064338: 929F005C  stw r20, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[20].u32 ) };
	// 8306433C: 93BF0058  stw r29, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 83064340: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83064344: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83064348: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8306434C: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83064350: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83064354: 939D0004  stw r28, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 83064358: B38A0000  sth r28, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[28].u16 ) };
	// 8306435C: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83064360: 4BF8EDA9  bl 0x82ff3108
	ctx.lr = 0x83064364;
	sub_82FF3108(ctx, base);
	// 83064364: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83064368: 40820020  bne 0x83064388
	if !ctx.cr[0].eq {
	pc = 0x83064388; continue 'dispatch;
	}
	// 8306436C: 388000AF  li r4, 0xaf
	ctx.r[4].s64 = 175;
	// 83064370: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83064374: 4BF84515  bl 0x82fe8888
	ctx.lr = 0x83064378;
	sub_82FE8888(ctx, base);
	// 83064378: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 8306437C: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83064380: 4BFB3491  bl 0x83017810
	ctx.lr = 0x83064384;
	sub_83017810(ctx, base);
	// 83064384: 480002C4  b 0x83064648
	pc = 0x83064648; continue 'dispatch;
	// 83064388: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8306438C: 896B000A  lbz r11, 0xa(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 83064390: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83064394: 41820034  beq 0x830643c8
	if ctx.cr[0].eq {
	pc = 0x830643C8; continue 'dispatch;
	}
	// 83064398: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8306439C: 3880003A  li r4, 0x3a
	ctx.r[4].s64 = 58;
	// 830643A0: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 830643A4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830643A8: 7F8B532E  sthx r28, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u16) };
	// 830643AC: 807D0018  lwz r3, 0x18(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 830643B0: 4BF6DA01  bl 0x82fd1db0
	ctx.lr = 0x830643B4;
	sub_82FD1DB0(ctx, base);
	// 830643B4: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 830643B8: 419A0010  beq cr6, 0x830643c8
	if ctx.cr[6].eq {
	pc = 0x830643C8; continue 'dispatch;
	}
	// 830643BC: 38800122  li r4, 0x122
	ctx.r[4].s64 = 290;
	// 830643C0: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830643C4: 4BF844C5  bl 0x82fe8888
	ctx.lr = 0x830643C8;
	sub_82FE8888(ctx, base);
	// 830643C8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830643CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830643D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830643D4: 4BFFFE75  bl 0x83064248
	ctx.lr = 0x830643D8;
	sub_83064248(ctx, base);
	// 830643D8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830643DC: 40820020  bne 0x830643fc
	if !ctx.cr[0].eq {
	pc = 0x830643FC; continue 'dispatch;
	}
	// 830643E0: 388000CF  li r4, 0xcf
	ctx.r[4].s64 = 207;
	// 830643E4: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830643E8: 4BF844A1  bl 0x82fe8888
	ctx.lr = 0x830643EC;
	sub_82FE8888(ctx, base);
	// 830643EC: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 830643F0: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 830643F4: 4BFB341D  bl 0x83017810
	ctx.lr = 0x830643F8;
	sub_83017810(ctx, base);
	// 830643F8: 48000250  b 0x83064648
	pc = 0x83064648; continue 'dispatch;
	// 830643FC: 82DE0028  lwz r22, 0x28(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83064400: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 83064404: 4BF7ABF5  bl 0x82fdeff8
	ctx.lr = 0x83064408;
	sub_82FDEFF8(ctx, base);
	// 83064408: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8306440C: 92DF0064  stw r22, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[22].u32 ) };
	// 83064410: 933F0060  stw r25, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[25].u32 ) };
	// 83064414: 82BE0028  lwz r21, 0x28(r30)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83064418: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8306441C: 4BF7ABDD  bl 0x82fdeff8
	ctx.lr = 0x83064420;
	sub_82FDEFF8(ctx, base);
	// 83064420: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 83064424: 92BF006C  stw r21, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[21].u32 ) };
	// 83064428: 931F0068  stw r24, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[24].u32 ) };
	// 8306442C: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83064430: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 83064434: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83064438: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8306443C: 4BFFCDF5  bl 0x83061230
	ctx.lr = 0x83064440;
	sub_83061230(ctx, base);
	// 83064440: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83064444: 4082002C  bne 0x83064470
	if !ctx.cr[0].eq {
	pc = 0x83064470; continue 'dispatch;
	}
	// 83064448: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 8306444C: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83064450: 4BFB33C1  bl 0x83017810
	ctx.lr = 0x83064454;
	sub_83017810(ctx, base);
	// 83064454: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83064458: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8306445C: 4BF7ACCD  bl 0x82fdf128
	ctx.lr = 0x83064460;
	sub_82FDF128(ctx, base);
	// 83064460: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83064464: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 83064468: 4BF7ACC1  bl 0x82fdf128
	ctx.lr = 0x8306446C;
	sub_82FDF128(ctx, base);
	// 8306446C: 480001DC  b 0x83064648
	pc = 0x83064648; continue 'dispatch;
	// 83064470: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83064474: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83064478: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8306447C: 4BFFFDCD  bl 0x83064248
	ctx.lr = 0x83064480;
	sub_83064248(ctx, base);
	// 83064480: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83064484: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83064488: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8306448C: 7F8B532E  sthx r28, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u16) };
	// 83064490: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83064494: 809D0018  lwz r4, 0x18(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83064498: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8306449C: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830644A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830644A4: 4E800421  bctrl
	ctx.lr = 0x830644A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830644A8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 830644AC: 7F6B0034  cntlzw r11, r27
	ctx.r[11].u64 = if ctx.r[27].u32 == 0 { 32 } else { ctx.r[27].u32.leading_zeros() as u64 };
	// 830644B0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830644B4: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830644B8: 69770001  xori r23, r11, 1
	ctx.r[23].u64 = ctx.r[11].u64 ^ 1;
	// 830644BC: 56EB063F  clrlwi. r11, r23, 0x18
	ctx.r[11].u64 = ctx.r[23].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830644C0: 41820034  beq 0x830644f4
	if ctx.cr[0].eq {
	pc = 0x830644F4; continue 'dispatch;
	}
	// 830644C4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830644C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830644CC: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 830644D0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830644D4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830644D8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830644DC: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 830644E0: 7F8B532E  sthx r28, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u16) };
	// 830644E4: 80BD0018  lwz r5, 0x18(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 830644E8: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830644EC: 4BF844ED  bl 0x82fe89d8
	ctx.lr = 0x830644F0;
	sub_82FE89D8(ctx, base);
	// 830644F0: 480000E8  b 0x830645d8
	pc = 0x830645D8; continue 'dispatch;
	// 830644F4: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 830644F8: 389F0070  addi r4, r31, 0x70
	ctx.r[4].s64 = ctx.r[31].s64 + 112;
	// 830644FC: 81590018  lwz r10, 0x18(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(24 as u32) ) } as u64;
	// 83064500: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83064504: 7F8B532E  sthx r28, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u16) };
	// 83064508: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 8306450C: 81580018  lwz r10, 0x18(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(24 as u32) ) } as u64;
	// 83064510: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83064514: 83590018  lwz r26, 0x18(r25)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(24 as u32) ) } as u64;
	// 83064518: 7F8B532E  sthx r28, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u16) };
	// 8306451C: 83780018  lwz r27, 0x18(r24)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(24 as u32) ) } as u64;
	// 83064520: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83064524: 4BF8A0D5  bl 0x82fee5f8
	ctx.lr = 0x83064528;
	sub_82FEE5F8(ctx, base);
	// 83064528: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8306452C: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83064530: 4BF73D69  bl 0x82fd8298
	ctx.lr = 0x83064534;
	sub_82FD8298(ctx, base);
	// 83064534: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83064538: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8306453C: 41820080  beq 0x830645bc
	if ctx.cr[0].eq {
	pc = 0x830645BC; continue 'dispatch;
	}
	// 83064540: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 83064544: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83064548: 419A0010  beq cr6, 0x83064558
	if ctx.cr[6].eq {
	pc = 0x83064558; continue 'dispatch;
	}
	// 8306454C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83064550: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83064554: 40820008  bne 0x8306455c
	if !ctx.cr[0].eq {
	pc = 0x8306455C; continue 'dispatch;
	}
	// 83064558: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 8306455C: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 83064560: 419A0014  beq cr6, 0x83064574
	if ctx.cr[6].eq {
	pc = 0x83064574; continue 'dispatch;
	}
	// 83064564: A15B0000  lhz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83064568: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8306456C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83064570: 40820008  bne 0x83064578
	if !ctx.cr[0].eq {
	pc = 0x83064578; continue 'dispatch;
	}
	// 83064574: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83064578: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8306457C: 419A0014  beq cr6, 0x83064590
	if ctx.cr[6].eq {
	pc = 0x83064590; continue 'dispatch;
	}
	// 83064580: A15A0000  lhz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83064584: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83064588: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8306458C: 40820008  bne 0x83064594
	if !ctx.cr[0].eq {
	pc = 0x83064594; continue 'dispatch;
	}
	// 83064590: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83064594: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83064598: 7D675B78  mr r7, r11
	ctx.r[7].u64 = ctx.r[11].u64;
	// 8306459C: 813D0018  lwz r9, 0x18(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 830645A0: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830645A4: 7F8B4B2E  sthx r28, r11, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[28].u16) };
	// 830645A8: 809D0018  lwz r4, 0x18(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 830645AC: 811E0008  lwz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830645B0: 480352B1  bl 0x83099860
	ctx.lr = 0x830645B4;
	sub_83099860(ctx, base);
	// 830645B4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830645B8: 48000008  b 0x830645c0
	pc = 0x830645C0; continue 'dispatch;
	// 830645BC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830645C0: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830645C4: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 830645C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830645CC: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 830645D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830645D4: 4E800421  bctrl
	ctx.lr = 0x830645D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830645D8: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830645DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830645E0: 419A0020  beq cr6, 0x83064600
	if ctx.cr[6].eq {
	pc = 0x83064600; continue 'dispatch;
	}
	// 830645E4: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 830645E8: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 830645EC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830645F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830645F4: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830645F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830645FC: 4E800421  bctrl
	ctx.lr = 0x83064600;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83064600: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83064604: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83064608: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8306460C: 4BFFFC3D  bl 0x83064248
	ctx.lr = 0x83064610;
	sub_83064248(ctx, base);
	// 83064610: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 83064614: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83064618: 4BF89C99  bl 0x82fee2b0
	ctx.lr = 0x8306461C;
	sub_82FEE2B0(ctx, base);
	// 8306461C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83064620: 40820010  bne 0x83064630
	if !ctx.cr[0].eq {
	pc = 0x83064630; continue 'dispatch;
	}
	// 83064624: 388000EE  li r4, 0xee
	ctx.r[4].s64 = 238;
	// 83064628: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8306462C: 4BF8425D  bl 0x82fe8888
	ctx.lr = 0x83064630;
	sub_82FE8888(ctx, base);
	// 83064630: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83064634: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83064638: 4BF7AAF1  bl 0x82fdf128
	ctx.lr = 0x8306463C;
	sub_82FDF128(ctx, base);
	// 8306463C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83064640: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 83064644: 4BF7AAE5  bl 0x82fdf128
	ctx.lr = 0x83064648;
	sub_82FDF128(ctx, base);
	// 83064648: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8306464C: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 83064650: 4BF7AAD9  bl 0x82fdf128
	ctx.lr = 0x83064654;
	sub_82FDF128(ctx, base);
	// 83064654: 383F00F0  addi r1, r31, 0xf0
	ctx.r[1].s64 = ctx.r[31].s64 + 240;
	// 83064658: 48143B40  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8306465C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8306465C size=40
    let mut pc: u32 = 0x8306465C;
    'dispatch: loop {
        match pc {
            0x8306465C => {
    //   block [0x8306465C..0x83064684)
	// 8306465C: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83064660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83064664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83064668: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8306466C: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 83064670: 4BF6F919  bl 0x82fd3f88
	ctx.lr = 0x83064674;
	sub_82FD3F88(ctx, base);
	// 83064674: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83064678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8306467C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83064680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83064684(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83064684 size=40
    let mut pc: u32 = 0x83064684;
    'dispatch: loop {
        match pc {
            0x83064684 => {
    //   block [0x83064684..0x830646AC)
	// 83064684: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83064688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306468C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83064690: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83064694: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83064698: 4BF6F8F1  bl 0x82fd3f88
	ctx.lr = 0x8306469C;
	sub_82FD3F88(ctx, base);
	// 8306469C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830646A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830646A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830646A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830646AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830646AC size=40
    let mut pc: u32 = 0x830646AC;
    'dispatch: loop {
        match pc {
            0x830646AC => {
    //   block [0x830646AC..0x830646D4)
	// 830646AC: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 830646B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830646B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830646B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830646BC: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 830646C0: 4BF6F8C9  bl 0x82fd3f88
	ctx.lr = 0x830646C4;
	sub_82FD3F88(ctx, base);
	// 830646C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830646C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830646CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830646D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830646D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830646D4 size=48
    let mut pc: u32 = 0x830646D4;
    'dispatch: loop {
        match pc {
            0x830646D4 => {
    //   block [0x830646D4..0x83064704)
	// 830646D4: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 830646D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830646DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830646E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830646E4: 817F0104  lwz r11, 0x104(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 830646E8: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830646EC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830646F0: 4BF73BF1  bl 0x82fd82e0
	ctx.lr = 0x830646F4;
	sub_82FD82E0(ctx, base);
	// 830646F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830646F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830646FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83064700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83064708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83064708 size=92
    let mut pc: u32 = 0x83064708;
    'dispatch: loop {
        match pc {
            0x83064708 => {
    //   block [0x83064708..0x83064764)
	// 83064708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306470C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83064710: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83064714: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83064718: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8306471C: 48019EA5  bl 0x8307e5c0
	ctx.lr = 0x83064720;
	sub_8307E5C0(ctx, base);
	// 83064720: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83064724: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83064728: 394B6528  addi r10, r11, 0x6528
	ctx.r[10].s64 = ctx.r[11].s64 + 25896;
	// 8306472C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83064730: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83064734: 913F0024  stw r9, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[9].u32 ) };
	// 83064738: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8306473C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83064740: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83064744: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83064748: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8306474C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83064750: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83064754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83064758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8306475C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83064760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83064768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83064768 size=128
    let mut pc: u32 = 0x83064768;
    'dispatch: loop {
        match pc {
            0x83064768 => {
    //   block [0x83064768..0x830647E8)
	// 83064768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306476C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83064770: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83064774: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83064778: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8306477C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83064780: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83064784: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83064788: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8306478C: 41820018  beq 0x830647a4
	if ctx.cr[0].eq {
	pc = 0x830647A4; continue 'dispatch;
	}
	// 83064790: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83064794: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83064798: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8306479C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830647A0: 4E800421  bctrl
	ctx.lr = 0x830647A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830647A4: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 830647A8: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 830647AC: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830647B0: 41820020  beq 0x830647d0
	if ctx.cr[0].eq {
	pc = 0x830647D0; continue 'dispatch;
	}
	// 830647B4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830647B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830647BC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830647C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830647C4: 4E800421  bctrl
	ctx.lr = 0x830647C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830647C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830647CC: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830647D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830647D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830647D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830647DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830647E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830647E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830647E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830647E8 size=8
    let mut pc: u32 = 0x830647E8;
    'dispatch: loop {
        match pc {
            0x830647E8 => {
    //   block [0x830647E8..0x830647F0)
	// 830647E8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830647EC: 82166578  lwz r16, 0x6578(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(25976 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830647F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830647F0 size=112
    let mut pc: u32 = 0x830647F0;
    'dispatch: loop {
        match pc {
            0x830647F0 => {
    //   block [0x830647F0..0x83064860)
	// 830647F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830647F4: 48143971  bl 0x831a8164
	ctx.lr = 0x830647F8;
	sub_831A8130(ctx, base);
	// 830647F8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830647FC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83064800: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83064804: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83064808: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 8306480C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83064810: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 83064814: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83064818: 48019DA9  bl 0x8307e5c0
	ctx.lr = 0x8306481C;
	sub_8307E5C0(ctx, base);
	// 8306481C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83064820: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83064824: 937E0024  stw r27, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[27].u32 ) };
	// 83064828: 394B6528  addi r10, r11, 0x6528
	ctx.r[10].s64 = ctx.r[11].s64 + 25896;
	// 8306482C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83064830: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83064834: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83064838: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8306483C: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83064840: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83064844: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83064848: 917E0028  stw r11, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8306484C: 917E002C  stw r11, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83064850: 48019CD1  bl 0x8307e520
	ctx.lr = 0x83064854;
	sub_8307E520(ctx, base);
	// 83064854: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83064858: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8306485C: 48143958  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83064860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83064860 size=40
    let mut pc: u32 = 0x83064860;
    'dispatch: loop {
        match pc {
            0x83064860 => {
    //   block [0x83064860..0x83064888)
	// 83064860: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83064864: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83064868: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8306486C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83064870: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83064874: 48019B0D  bl 0x8307e380
	ctx.lr = 0x83064878;
	sub_8307E380(ctx, base);
	// 83064878: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8306487C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83064880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83064884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83064888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83064888 size=36
    let mut pc: u32 = 0x83064888;
    'dispatch: loop {
        match pc {
            0x83064888 => {
    //   block [0x83064888..0x830648AC)
	// 83064888: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 8306488C: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83064890: 4182001C  beq 0x830648ac
	if ctx.cr[0].eq {
		sub_830648AC(ctx, base);
		return;
	}
	// 83064894: 396BFFFD  addi r11, r11, -3
	ctx.r[11].s64 = ctx.r[11].s64 + -3;
	// 83064898: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8306489C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830648A0: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 830648A4: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 830648A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830648AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830648AC size=8
    let mut pc: u32 = 0x830648AC;
    'dispatch: loop {
        match pc {
            0x830648AC => {
    //   block [0x830648AC..0x830648B4)
	// 830648AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830648B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830648B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830648B8 size=112
    let mut pc: u32 = 0x830648B8;
    'dispatch: loop {
        match pc {
            0x830648B8 => {
    //   block [0x830648B8..0x83064928)
	// 830648B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830648BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830648C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830648C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830648C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830648CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830648D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830648D4: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830648D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830648DC: 41820018  beq 0x830648f4
	if ctx.cr[0].eq {
	pc = 0x830648F4; continue 'dispatch;
	}
	// 830648E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830648E4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830648E8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830648EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830648F0: 4E800421  bctrl
	ctx.lr = 0x830648F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830648F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830648F8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830648FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83064900: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 83064904: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83064908: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8306490C: 4E800421  bctrl
	ctx.lr = 0x83064910;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83064910: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83064914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83064918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8306491C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83064920: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83064924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83064928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83064928 size=8
    let mut pc: u32 = 0x83064928;
    'dispatch: loop {
        match pc {
            0x83064928 => {
    //   block [0x83064928..0x83064930)
	// 83064928: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8306492C: 821665B0  lwz r16, 0x65b0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(26032 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83064930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83064930 size=196
    let mut pc: u32 = 0x83064930;
    'dispatch: loop {
        match pc {
            0x83064930 => {
    //   block [0x83064930..0x830649F4)
	// 83064930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83064934: 48143839  bl 0x831a816c
	ctx.lr = 0x83064938;
	sub_831A8130(ctx, base);
	// 83064938: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8306493C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83064940: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83064944: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83064948: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8306494C: 409A0010  bne cr6, 0x8306495c
	if !ctx.cr[6].eq {
	pc = 0x8306495C; continue 'dispatch;
	}
	// 83064950: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83064954: 386B78D8  addi r3, r11, 0x78d8
	ctx.r[3].s64 = ctx.r[11].s64 + 30936;
	// 83064958: 48000014  b 0x8306496c
	pc = 0x8306496C; continue 'dispatch;
	// 8306495C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83064960: 409A001C  bne cr6, 0x8306497c
	if !ctx.cr[6].eq {
	pc = 0x8306497C; continue 'dispatch;
	}
	// 83064964: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83064968: 386B7950  addi r3, r11, 0x7950
	ctx.r[3].s64 = ctx.r[11].s64 + 31056;
	// 8306496C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83064970: 4BF6C211  bl 0x82fd0b80
	ctx.lr = 0x83064974;
	sub_82FD0B80(ctx, base);
	// 83064974: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83064978: 48000070  b 0x830649e8
	pc = 0x830649E8; continue 'dispatch;
	// 8306497C: 388003FF  li r4, 0x3ff
	ctx.r[4].s64 = 1023;
	// 83064980: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83064984: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83064988: 4BF7A4D1  bl 0x82fdee58
	ctx.lr = 0x8306498C;
	sub_82FDEE58(ctx, base);
	// 8306498C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83064990: 3BBF0050  addi r29, r31, 0x50
	ctx.r[29].s64 = ctx.r[31].s64 + 80;
	// 83064994: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83064998: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8306499C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830649A0: 4E800421  bctrl
	ctx.lr = 0x830649A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830649A4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830649A8: 4801E181  bl 0x83082b28
	ctx.lr = 0x830649AC;
	sub_83082B28(ctx, base);
	// 830649AC: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830649B0: 813F0068  lwz r9, 0x68(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 830649B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830649B8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830649BC: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830649C0: 7D4B4B2E  sthx r10, r11, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u16) };
	// 830649C4: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 830649C8: 4BF6C1B9  bl 0x82fd0b80
	ctx.lr = 0x830649CC;
	sub_82FD0B80(ctx, base);
	// 830649CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830649D0: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 830649D4: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 830649D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830649DC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830649E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830649E4: 4E800421  bctrl
	ctx.lr = 0x830649E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830649E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830649EC: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 830649F0: 481437CC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830649F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830649F4 size=40
    let mut pc: u32 = 0x830649F4;
    'dispatch: loop {
        match pc {
            0x830649F4 => {
    //   block [0x830649F4..0x83064A1C)
	// 830649F4: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830649F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830649FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83064A00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83064A04: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83064A08: 4BF7A4D1  bl 0x82fdeed8
	ctx.lr = 0x83064A0C;
	sub_82FDEED8(ctx, base);
	// 83064A0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83064A10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83064A14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83064A18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83064A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83064A20 size=8
    let mut pc: u32 = 0x83064A20;
    'dispatch: loop {
        match pc {
            0x83064A20 => {
    //   block [0x83064A20..0x83064A28)
	// 83064A20: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83064A24: 82166650  lwz r16, 0x6650(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(26192 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83064A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83064A28 size=600
    let mut pc: u32 = 0x83064A28;
    'dispatch: loop {
        match pc {
            0x83064A28 => {
    //   block [0x83064A28..0x83064C80)
	// 83064A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83064A2C: 4814373D  bl 0x831a8168
	ctx.lr = 0x83064A30;
	sub_831A8130(ctx, base);
	// 83064A30: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 83064A34: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83064A38: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83064A3C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83064A40: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83064A44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83064A48: 4E800421  bctrl
	ctx.lr = 0x83064A4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83064A4C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83064A50: 40820030  bne 0x83064a80
	if !ctx.cr[0].eq {
	pc = 0x83064A80; continue 'dispatch;
	}
	// 83064A54: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83064A58: 80FD0004  lwz r7, 4(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83064A5C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 83064A60: 388B65F0  addi r4, r11, 0x65f0
	ctx.r[4].s64 = ctx.r[11].s64 + 26096;
	// 83064A64: 38A00158  li r5, 0x158
	ctx.r[5].s64 = 344;
	// 83064A68: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83064A6C: 4BF6C5ED  bl 0x82fd1058
	ctx.lr = 0x83064A70;
	sub_82FD1058(ctx, base);
	// 83064A70: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83064A74: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83064A78: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 83064A7C: 4814C1AD  bl 0x831b0c28
	ctx.lr = 0x83064A80;
	sub_831B0C28(ctx, base);
	// 83064A80: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83064A84: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83064A88: 41820044  beq 0x83064acc
	if ctx.cr[0].eq {
	pc = 0x83064ACC; continue 'dispatch;
	}
	// 83064A8C: 3D408217  lis r10, -0x7de9
	ctx.r[10].s64 = -2112421888;
	// 83064A90: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83064A94: 814A9774  lwz r10, -0x688c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26764 as u32) ) } as u64;
	// 83064A98: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83064A9C: 409A0030  bne cr6, 0x83064acc
	if !ctx.cr[6].eq {
	pc = 0x83064ACC; continue 'dispatch;
	}
	// 83064AA0: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83064AA4: 80FD0004  lwz r7, 4(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83064AA8: 38C00012  li r6, 0x12
	ctx.r[6].s64 = 18;
	// 83064AAC: 388B65F0  addi r4, r11, 0x65f0
	ctx.r[4].s64 = ctx.r[11].s64 + 26096;
	// 83064AB0: 38A00160  li r5, 0x160
	ctx.r[5].s64 = 352;
	// 83064AB4: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83064AB8: 4BF6C5A1  bl 0x82fd1058
	ctx.lr = 0x83064ABC;
	sub_82FD1058(ctx, base);
	// 83064ABC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83064AC0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83064AC4: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 83064AC8: 4814C161  bl 0x831b0c28
	ctx.lr = 0x83064ACC;
	sub_831B0C28(ctx, base);
	// 83064ACC: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83064AD0: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83064AD4: 40820044  bne 0x83064b18
	if !ctx.cr[0].eq {
	pc = 0x83064B18; continue 'dispatch;
	}
	// 83064AD8: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83064ADC: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83064AE0: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 83064AE4: 4BF737B5  bl 0x82fd8298
	ctx.lr = 0x83064AE8;
	sub_82FD8298(ctx, base);
	// 83064AE8: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83064AEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83064AF0: 41820020  beq 0x83064b10
	if ctx.cr[0].eq {
	pc = 0x83064B10; continue 'dispatch;
	}
	// 83064AF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83064AF8: 811D0004  lwz r8, 4(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83064AFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83064B00: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83064B04: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83064B08: 4BFD9749  bl 0x8303e250
	ctx.lr = 0x83064B0C;
	sub_8303E250(ctx, base);
	// 83064B0C: 48000008  b 0x83064b14
	pc = 0x83064B14; continue 'dispatch;
	// 83064B10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83064B14: 48000164  b 0x83064c78
	pc = 0x83064C78; continue 'dispatch;
	// 83064B18: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83064B1C: 419A00A4  beq cr6, 0x83064bc0
	if ctx.cr[6].eq {
	pc = 0x83064BC0; continue 'dispatch;
	}
	// 83064B20: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 83064B24: 419A009C  beq cr6, 0x83064bc0
	if ctx.cr[6].eq {
	pc = 0x83064BC0; continue 'dispatch;
	}
	// 83064B28: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83064B2C: 419A0040  beq cr6, 0x83064b6c
	if ctx.cr[6].eq {
	pc = 0x83064B6C; continue 'dispatch;
	}
	// 83064B30: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83064B34: 419A0038  beq cr6, 0x83064b6c
	if ctx.cr[6].eq {
	pc = 0x83064B6C; continue 'dispatch;
	}
	// 83064B38: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83064B3C: 419A0030  beq cr6, 0x83064b6c
	if ctx.cr[6].eq {
	pc = 0x83064B6C; continue 'dispatch;
	}
	// 83064B40: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83064B44: 80FD0004  lwz r7, 4(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83064B48: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 83064B4C: 388B65F0  addi r4, r11, 0x65f0
	ctx.r[4].s64 = ctx.r[11].s64 + 26096;
	// 83064B50: 38A0019E  li r5, 0x19e
	ctx.r[5].s64 = 414;
	// 83064B54: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83064B58: 4BF6C501  bl 0x82fd1058
	ctx.lr = 0x83064B5C;
	sub_82FD1058(ctx, base);
	// 83064B5C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83064B60: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83064B64: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 83064B68: 4814C0C1  bl 0x831b0c28
	ctx.lr = 0x83064B6C;
	sub_831B0C28(ctx, base);
	// 83064B6C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83064B70: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83064B74: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83064B78: 408200B0  bne 0x83064c28
	if !ctx.cr[0].eq {
	pc = 0x83064C28; continue 'dispatch;
	}
	// 83064B7C: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83064B80: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83064B84: 909F0054  stw r4, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 83064B88: 4BF73711  bl 0x82fd8298
	ctx.lr = 0x83064B8C;
	sub_82FD8298(ctx, base);
	// 83064B8C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83064B90: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83064B94: 41820024  beq 0x83064bb8
	if ctx.cr[0].eq {
	pc = 0x83064BB8; continue 'dispatch;
	}
	// 83064B98: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83064B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83064BA0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83064BA4: 811D0004  lwz r8, 4(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83064BA8: 80FE0018  lwz r7, 0x18(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83064BAC: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83064BB0: 4BFD96A1  bl 0x8303e250
	ctx.lr = 0x83064BB4;
	sub_8303E250(ctx, base);
	// 83064BB4: 48000008  b 0x83064bbc
	pc = 0x83064BBC; continue 'dispatch;
	// 83064BB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83064BBC: 480000BC  b 0x83064c78
	pc = 0x83064C78; continue 'dispatch;
	// 83064BC0: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83064BC4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83064BC8: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83064BCC: 4082005C  bne 0x83064c28
	if !ctx.cr[0].eq {
	pc = 0x83064C28; continue 'dispatch;
	}
	// 83064BD0: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83064BD4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83064BD8: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83064BDC: 4082004C  bne 0x83064c28
	if !ctx.cr[0].eq {
	pc = 0x83064C28; continue 'dispatch;
	}
	// 83064BE0: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83064BE4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83064BE8: 909F0054  stw r4, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 83064BEC: 4BF736AD  bl 0x82fd8298
	ctx.lr = 0x83064BF0;
	sub_82FD8298(ctx, base);
	// 83064BF0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83064BF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83064BF8: 41820028  beq 0x83064c20
	if ctx.cr[0].eq {
	pc = 0x83064C20; continue 'dispatch;
	}
	// 83064BFC: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83064C00: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83064C04: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83064C08: 811D0004  lwz r8, 4(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83064C0C: 80FE0018  lwz r7, 0x18(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83064C10: 80CB0008  lwz r6, 8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83064C14: 80AA0008  lwz r5, 8(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 83064C18: 4BFD9639  bl 0x8303e250
	ctx.lr = 0x83064C1C;
	sub_8303E250(ctx, base);
	// 83064C1C: 48000008  b 0x83064c24
	pc = 0x83064C24; continue 'dispatch;
	// 83064C20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83064C24: 48000054  b 0x83064c78
	pc = 0x83064C78; continue 'dispatch;
	// 83064C28: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83064C2C: 38600044  li r3, 0x44
	ctx.r[3].s64 = 68;
	// 83064C30: 909F0054  stw r4, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 83064C34: 4BF73665  bl 0x82fd8298
	ctx.lr = 0x83064C38;
	sub_82FD8298(ctx, base);
	// 83064C38: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83064C3C: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83064C40: 41820034  beq 0x83064c74
	if ctx.cr[0].eq {
	pc = 0x83064C74; continue 'dispatch;
	}
	// 83064C44: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83064C48: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83064C4C: 839D0004  lwz r28, 4(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83064C50: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83064C54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83064C58: 4E800421  bctrl
	ctx.lr = 0x83064C5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83064C5C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83064C60: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83064C64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83064C68: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83064C6C: 4802152D  bl 0x83086198
	ctx.lr = 0x83064C70;
	sub_83086198(ctx, base);
	// 83064C70: 48000008  b 0x83064c78
	pc = 0x83064C78; continue 'dispatch;
	// 83064C74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83064C78: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 83064C7C: 4814353C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83064C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83064C80 size=44
    let mut pc: u32 = 0x83064C80;
    'dispatch: loop {
        match pc {
            0x83064C80 => {
    //   block [0x83064C80..0x83064CAC)
	// 83064C80: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83064C84: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83064C88: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83064C8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83064C90: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83064C94: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83064C98: 4BF73649  bl 0x82fd82e0
	ctx.lr = 0x83064C9C;
	sub_82FD82E0(ctx, base);
	// 83064C9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83064CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83064CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83064CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83064CAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83064CAC size=44
    let mut pc: u32 = 0x83064CAC;
    'dispatch: loop {
        match pc {
            0x83064CAC => {
    //   block [0x83064CAC..0x83064CD8)
	// 83064CAC: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83064CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83064CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83064CB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83064CBC: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83064CC0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83064CC4: 4BF7361D  bl 0x82fd82e0
	ctx.lr = 0x83064CC8;
	sub_82FD82E0(ctx, base);
	// 83064CC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83064CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83064CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83064CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83064CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83064CD8 size=44
    let mut pc: u32 = 0x83064CD8;
    'dispatch: loop {
        match pc {
            0x83064CD8 => {
    //   block [0x83064CD8..0x83064D04)
	// 83064CD8: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83064CDC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83064CE0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83064CE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83064CE8: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83064CEC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83064CF0: 4BF735F1  bl 0x82fd82e0
	ctx.lr = 0x83064CF4;
	sub_82FD82E0(ctx, base);
	// 83064CF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83064CF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83064CFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83064D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83064D04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83064D04 size=44
    let mut pc: u32 = 0x83064D04;
    'dispatch: loop {
        match pc {
            0x83064D04 => {
    //   block [0x83064D04..0x83064D30)
	// 83064D04: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83064D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83064D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83064D10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83064D14: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83064D18: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83064D1C: 4BF735C5  bl 0x82fd82e0
	ctx.lr = 0x83064D20;
	sub_82FD82E0(ctx, base);
	// 83064D20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83064D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83064D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83064D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83064D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83064D30 size=8
    let mut pc: u32 = 0x83064D30;
    'dispatch: loop {
        match pc {
            0x83064D30 => {
    //   block [0x83064D30..0x83064D38)
	// 83064D30: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83064D34: 821666C0  lwz r16, 0x66c0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(26304 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83064D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83064D38 size=96
    let mut pc: u32 = 0x83064D38;
    'dispatch: loop {
        match pc {
            0x83064D38 => {
    //   block [0x83064D38..0x83064D98)
	// 83064D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83064D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83064D40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83064D44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83064D48: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83064D4C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83064D50: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83064D54: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 83064D58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83064D5C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83064D60: 4BF73539  bl 0x82fd8298
	ctx.lr = 0x83064D64;
	sub_82FD8298(ctx, base);
	// 83064D64: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83064D68: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83064D6C: 41820010  beq 0x83064d7c
	if ctx.cr[0].eq {
	pc = 0x83064D7C; continue 'dispatch;
	}
	// 83064D70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83064D74: 4BFFF995  bl 0x83064708
	ctx.lr = 0x83064D78;
	sub_83064708(ctx, base);
	// 83064D78: 48000008  b 0x83064d80
	pc = 0x83064D80; continue 'dispatch;
	// 83064D7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83064D80: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83064D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83064D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83064D8C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83064D90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83064D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83064D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83064D98 size=44
    let mut pc: u32 = 0x83064D98;
    'dispatch: loop {
        match pc {
            0x83064D98 => {
    //   block [0x83064D98..0x83064DC4)
	// 83064D98: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83064D9C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83064DA0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83064DA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83064DA8: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83064DAC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83064DB0: 4BF73531  bl 0x82fd82e0
	ctx.lr = 0x83064DB4;
	sub_82FD82E0(ctx, base);
	// 83064DB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83064DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83064DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83064DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83064DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83064DC8 size=12
    let mut pc: u32 = 0x83064DC8;
    'dispatch: loop {
        match pc {
            0x83064DC8 => {
    //   block [0x83064DC8..0x83064DD4)
	// 83064DC8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83064DCC: 386B3434  addi r3, r11, 0x3434
	ctx.r[3].s64 = ctx.r[11].s64 + 13364;
	// 83064DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83064DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83064DD8 size=224
    let mut pc: u32 = 0x83064DD8;
    'dispatch: loop {
        match pc {
            0x83064DD8 => {
    //   block [0x83064DD8..0x83064EB8)
	// 83064DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83064DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83064DE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83064DE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83064DE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83064DEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83064DF0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83064DF4: 4801980D  bl 0x8307e600
	ctx.lr = 0x83064DF8;
	sub_8307E600(ctx, base);
	// 83064DF8: A97E0000  lha r11, 0(r30)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 83064DFC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83064E00: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83064E04: 41820038  beq 0x83064e3c
	if ctx.cr[0].eq {
	pc = 0x83064E3C; continue 'dispatch;
	}
	// 83064E08: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83064E0C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83064E10: 4BFEADD1  bl 0x8304fbe0
	ctx.lr = 0x83064E14;
	sub_8304FBE0(ctx, base);
	// 83064E14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83064E18: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83064E1C: 4BF94DE5  bl 0x82ff9c00
	ctx.lr = 0x83064E20;
	sub_82FF9C00(ctx, base);
	// 83064E20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83064E24: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83064E28: 4BF94DD9  bl 0x82ff9c00
	ctx.lr = 0x83064E2C;
	sub_82FF9C00(ctx, base);
	// 83064E2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83064E30: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83064E34: 4BF944C5  bl 0x82ff92f8
	ctx.lr = 0x83064E38;
	sub_82FF92F8(ctx, base);
	// 83064E38: 48000068  b 0x83064ea0
	pc = 0x83064EA0; continue 'dispatch;
	// 83064E3C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 83064E40: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83064E44: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 83064E48: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83064E4C: 4BFE94FD  bl 0x8304e348
	ctx.lr = 0x83064E50;
	sub_8304E348(ctx, base);
	// 83064E50: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83064E54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83064E58: 388B3608  addi r4, r11, 0x3608
	ctx.r[4].s64 = ctx.r[11].s64 + 13832;
	// 83064E5C: 4BF94E65  bl 0x82ff9cc0
	ctx.lr = 0x83064E60;
	sub_82FF9CC0(ctx, base);
	// 83064E60: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83064E64: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83064E68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83064E6C: 388B34D0  addi r4, r11, 0x34d0
	ctx.r[4].s64 = ctx.r[11].s64 + 13520;
	// 83064E70: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 83064E74: 4BF94E4D  bl 0x82ff9cc0
	ctx.lr = 0x83064E78;
	sub_82FF9CC0(ctx, base);
	// 83064E78: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83064E7C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83064E80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83064E84: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83064E88: 4BF946F1  bl 0x82ff9578
	ctx.lr = 0x83064E8C;
	sub_82FF9578(ctx, base);
	// 83064E8C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83064E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83064E94: 915F0024  stw r10, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 83064E98: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83064E9C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83064EA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83064EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83064EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83064EAC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83064EB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83064EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83064EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83064EB8 size=20
    let mut pc: u32 = 0x83064EB8;
    'dispatch: loop {
        match pc {
            0x83064EB8 => {
    //   block [0x83064EB8..0x83064ECC)
	// 83064EB8: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 83064EBC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83064EC0: 4082000C  bne 0x83064ecc
	if !ctx.cr[0].eq {
		sub_83064ECC(ctx, base);
		return;
	}
	// 83064EC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83064EC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83064ECC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83064ECC size=72
    let mut pc: u32 = 0x83064ECC;
    'dispatch: loop {
        match pc {
            0x83064ECC => {
    //   block [0x83064ECC..0x83064F14)
	// 83064ECC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 83064ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83064ED4: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83064ED8: 41820024  beq 0x83064efc
	if ctx.cr[0].eq {
	pc = 0x83064EFC; continue 'dispatch;
	}
	// 83064EDC: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 83064EE0: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83064EE4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 83064EE8: 409A002C  bne cr6, 0x83064f14
	if !ctx.cr[6].eq {
		sub_83064F14(ctx, base);
		return;
	}
	// 83064EEC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83064EF0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83064EF4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83064EF8: 4198FFE8  blt cr6, 0x83064ee0
	if ctx.cr[6].lt {
	pc = 0x83064EE0; continue 'dispatch;
	}
	// 83064EFC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83064F00: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83064F04: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83064F08: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83064F0C: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83064F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83064F14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83064F14 size=8
    let mut pc: u32 = 0x83064F14;
    'dispatch: loop {
        match pc {
            0x83064F14 => {
    //   block [0x83064F14..0x83064F1C)
	// 83064F14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83064F18: 4BFFFFE8  b 0x83064f00
	sub_83064ECC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83064F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83064F20 size=64
    let mut pc: u32 = 0x83064F20;
    'dispatch: loop {
        match pc {
            0x83064F20 => {
    //   block [0x83064F20..0x83064F60)
	// 83064F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83064F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83064F28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83064F2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83064F30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83064F34: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83064F38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83064F3C: 409A000C  bne cr6, 0x83064f48
	if !ctx.cr[6].eq {
	pc = 0x83064F48; continue 'dispatch;
	}
	// 83064F40: 4BFFF9F1  bl 0x83064930
	ctx.lr = 0x83064F44;
	sub_83064930(ctx, base);
	// 83064F44: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 83064F48: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83064F4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83064F50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83064F54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83064F58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83064F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83064F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83064F60 size=8
    let mut pc: u32 = 0x83064F60;
    'dispatch: loop {
        match pc {
            0x83064F60 => {
    //   block [0x83064F60..0x83064F68)
	// 83064F60: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83064F64: 821666F8  lwz r16, 0x66f8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(26360 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83064F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83064F68 size=188
    let mut pc: u32 = 0x83064F68;
    'dispatch: loop {
        match pc {
            0x83064F68 => {
    //   block [0x83064F68..0x83065024)
	// 83064F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83064F6C: 481431FD  bl 0x831a8168
	ctx.lr = 0x83064F70;
	sub_831A8130(ctx, base);
	// 83064F70: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 83064F74: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83064F78: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83064F7C: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83064F80: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83064F84: 409A0060  bne cr6, 0x83064fe4
	if !ctx.cr[6].eq {
	pc = 0x83064FE4; continue 'dispatch;
	}
	// 83064F88: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83064F8C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83064F90: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 83064F94: 4BF73305  bl 0x82fd8298
	ctx.lr = 0x83064F98;
	sub_82FD8298(ctx, base);
	// 83064F98: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83064F9C: 93BF0054  stw r29, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 83064FA0: 41820038  beq 0x83064fd8
	if ctx.cr[0].eq {
	pc = 0x83064FD8; continue 'dispatch;
	}
	// 83064FA4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83064FA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83064FAC: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83064FB0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83064FB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83064FB8: 4E800421  bctrl
	ctx.lr = 0x83064FBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83064FBC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83064FC0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83064FC4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83064FC8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83064FCC: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 83064FD0: 48022401  bl 0x830873d0
	ctx.lr = 0x83064FD4;
	sub_830873D0(ctx, base);
	// 83064FD4: 48000008  b 0x83064fdc
	pc = 0x83064FDC; continue 'dispatch;
	// 83064FD8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83064FDC: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 83064FE0: 481431D8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83064FE4: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83064FE8: 409A0010  bne cr6, 0x83064ff8
	if !ctx.cr[6].eq {
	pc = 0x83064FF8; continue 'dispatch;
	}
	// 83064FEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83064FF0: 4BFFFA39  bl 0x83064a28
	ctx.lr = 0x83064FF4;
	sub_83064A28(ctx, base);
	// 83064FF4: 4BFFFFE8  b 0x83064fdc
	pc = 0x83064FDC; continue 'dispatch;
	// 83064FF8: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83064FFC: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83065000: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 83065004: 388B65F0  addi r4, r11, 0x65f0
	ctx.r[4].s64 = ctx.r[11].s64 + 26096;
	// 83065008: 38A0014C  li r5, 0x14c
	ctx.r[5].s64 = 332;
	// 8306500C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83065010: 4BF6C049  bl 0x82fd1058
	ctx.lr = 0x83065014;
	sub_82FD1058(ctx, base);
	// 83065014: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83065018: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8306501C: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 83065020: 4814BC09  bl 0x831b0c28
	ctx.lr = 0x83065024;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065024(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83065024 size=44
    let mut pc: u32 = 0x83065024;
    'dispatch: loop {
        match pc {
            0x83065024 => {
    //   block [0x83065024..0x83065050)
	// 83065024: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83065028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306502C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83065030: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83065034: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83065038: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8306503C: 4BF732A5  bl 0x82fd82e0
	ctx.lr = 0x83065040;
	sub_82FD82E0(ctx, base);
	// 83065040: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83065044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83065048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8306504C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83065050 size=64
    let mut pc: u32 = 0x83065050;
    'dispatch: loop {
        match pc {
            0x83065050 => {
    //   block [0x83065050..0x83065090)
	// 83065050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83065054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83065058: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8306505C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83065060: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83065064: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83065068: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8306506C: 409A000C  bne cr6, 0x83065078
	if !ctx.cr[6].eq {
	pc = 0x83065078; continue 'dispatch;
	}
	// 83065070: 4BFFFEF9  bl 0x83064f68
	ctx.lr = 0x83065074;
	sub_83064F68(ctx, base);
	// 83065074: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 83065078: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8306507C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83065080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83065084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83065088: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8306508C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83065090 size=68
    let mut pc: u32 = 0x83065090;
    'dispatch: loop {
        match pc {
            0x83065090 => {
    //   block [0x83065090..0x830650D4)
	// 83065090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83065094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83065098: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8306509C: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 830650A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830650A4: 4082000C  bne 0x830650b0
	if !ctx.cr[0].eq {
	pc = 0x830650B0; continue 'dispatch;
	}
	// 830650A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830650AC: 48000018  b 0x830650c4
	pc = 0x830650C4; continue 'dispatch;
	// 830650B0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 830650B4: 4BF953AD  bl 0x82ffa460
	ctx.lr = 0x830650B8;
	sub_82FFA460(ctx, base);
	// 830650B8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830650BC: 4182FFEC  beq 0x830650a8
	if ctx.cr[0].eq {
	pc = 0x830650A8; continue 'dispatch;
	}
	// 830650C0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830650C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830650C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830650CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830650D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830650D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830650D8 size=8
    let mut pc: u32 = 0x830650D8;
    'dispatch: loop {
        match pc {
            0x830650D8 => {
    //   block [0x830650D8..0x830650E0)
	// 830650D8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830650DC: 82166748  lwz r16, 0x6748(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(26440 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830650E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830650E0 size=108
    let mut pc: u32 = 0x830650E0;
    'dispatch: loop {
        match pc {
            0x830650E0 => {
    //   block [0x830650E0..0x8306514C)
	// 830650E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830650E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830650E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830650EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830650F0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830650F4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830650F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830650FC: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 83065100: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83065104: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 83065108: 4BF73191  bl 0x82fd8298
	ctx.lr = 0x8306510C;
	sub_82FD8298(ctx, base);
	// 8306510C: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83065110: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065114: 41820018  beq 0x8306512c
	if ctx.cr[0].eq {
	pc = 0x8306512C; continue 'dispatch;
	}
	// 83065118: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8306511C: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83065120: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 83065124: 4BFE7E3D  bl 0x8304cf60
	ctx.lr = 0x83065128;
	sub_8304CF60(ctx, base);
	// 83065128: 48000008  b 0x83065130
	pc = 0x83065130; continue 'dispatch;
	// 8306512C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83065130: 907E0018  stw r3, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 83065134: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83065138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8306513C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83065140: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83065144: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83065148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8306514C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8306514C size=44
    let mut pc: u32 = 0x8306514C;
    'dispatch: loop {
        match pc {
            0x8306514C => {
    //   block [0x8306514C..0x83065178)
	// 8306514C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83065150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83065154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83065158: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8306515C: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83065160: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83065164: 4BF7317D  bl 0x82fd82e0
	ctx.lr = 0x83065168;
	sub_82FD82E0(ctx, base);
	// 83065168: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8306516C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83065170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83065174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83065178 size=8
    let mut pc: u32 = 0x83065178;
    'dispatch: loop {
        match pc {
            0x83065178 => {
    //   block [0x83065178..0x83065180)
	// 83065178: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8306517C: 82166780  lwz r16, 0x6780(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(26496 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83065180 size=200
    let mut pc: u32 = 0x83065180;
    'dispatch: loop {
        match pc {
            0x83065180 => {
    //   block [0x83065180..0x83065248)
	// 83065180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83065184: 48142FE9  bl 0x831a816c
	ctx.lr = 0x83065188;
	sub_831A8130(ctx, base);
	// 83065188: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8306518C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83065190: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83065194: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83065198: 396B6528  addi r11, r11, 0x6528
	ctx.r[11].s64 = ctx.r[11].s64 + 25896;
	// 8306519C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830651A0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830651A4: 83BE0018  lwz r29, 0x18(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 830651A8: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830651AC: 41820014  beq 0x830651c0
	if ctx.cr[0].eq {
	pc = 0x830651C0; continue 'dispatch;
	}
	// 830651B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830651B4: 4BF81AB5  bl 0x82fe6c68
	ctx.lr = 0x830651B8;
	sub_82FE6C68(ctx, base);
	// 830651B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830651BC: 4BF73125  bl 0x82fd82e0
	ctx.lr = 0x830651C0;
	sub_82FD82E0(ctx, base);
	// 830651C0: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 830651C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830651C8: 41820018  beq 0x830651e0
	if ctx.cr[0].eq {
	pc = 0x830651E0; continue 'dispatch;
	}
	// 830651CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830651D0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830651D4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830651D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830651DC: 4E800421  bctrl
	ctx.lr = 0x830651E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830651E0: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830651E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830651E8: 41820018  beq 0x83065200
	if ctx.cr[0].eq {
	pc = 0x83065200; continue 'dispatch;
	}
	// 830651EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830651F0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830651F4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830651F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830651FC: 4E800421  bctrl
	ctx.lr = 0x83065200;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83065200: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83065204: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065208: 41820018  beq 0x83065220
	if ctx.cr[0].eq {
	pc = 0x83065220; continue 'dispatch;
	}
	// 8306520C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83065210: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83065214: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83065218: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8306521C: 4E800421  bctrl
	ctx.lr = 0x83065220;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83065220: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83065224: 809E002C  lwz r4, 0x2c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83065228: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8306522C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83065230: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83065234: 4E800421  bctrl
	ctx.lr = 0x83065238;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83065238: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8306523C: 48019145  bl 0x8307e380
	ctx.lr = 0x83065240;
	sub_8307E380(ctx, base);
	// 83065240: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83065244: 48142F78  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83065248 size=40
    let mut pc: u32 = 0x83065248;
    'dispatch: loop {
        match pc {
            0x83065248 => {
    //   block [0x83065248..0x83065270)
	// 83065248: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8306524C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83065250: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83065254: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83065258: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8306525C: 48019125  bl 0x8307e380
	ctx.lr = 0x83065260;
	sub_8307E380(ctx, base);
	// 83065260: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83065264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83065268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8306526C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83065270 size=8
    let mut pc: u32 = 0x83065270;
    'dispatch: loop {
        match pc {
            0x83065270 => {
    //   block [0x83065270..0x83065278)
	// 83065270: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83065274: 821667D0  lwz r16, 0x67d0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(26576 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83065278 size=316
    let mut pc: u32 = 0x83065278;
    'dispatch: loop {
        match pc {
            0x83065278 => {
    //   block [0x83065278..0x830653B4)
	// 83065278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306527C: 48142EE5  bl 0x831a8160
	ctx.lr = 0x83065280;
	sub_831A8130(ctx, base);
	// 83065280: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83065284: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83065288: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8306528C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83065290: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 83065294: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 83065298: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8306529C: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 830652A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830652A4: 41820020  beq 0x830652c4
	if ctx.cr[0].eq {
	pc = 0x830652C4; continue 'dispatch;
	}
	// 830652A8: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 830652AC: 4BF951B5  bl 0x82ffa460
	ctx.lr = 0x830652B0;
	sub_82FFA460(ctx, base);
	// 830652B0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830652B4: 41820010  beq 0x830652c4
	if ctx.cr[0].eq {
	pc = 0x830652C4; continue 'dispatch;
	}
	// 830652B8: 83830000  lwz r28, 0(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830652BC: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830652C0: 408200E0  bne 0x830653a0
	if !ctx.cr[0].eq {
	pc = 0x830653A0; continue 'dispatch;
	}
	// 830652C4: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 830652C8: 409A00D8  bne cr6, 0x830653a0
	if !ctx.cr[6].eq {
	pc = 0x830653A0; continue 'dispatch;
	}
	// 830652CC: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 830652D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830652D4: 409A000C  bne cr6, 0x830652e0
	if !ctx.cr[6].eq {
	pc = 0x830652E0; continue 'dispatch;
	}
	// 830652D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830652DC: 4BFFFE05  bl 0x830650e0
	ctx.lr = 0x830652E0;
	sub_830650E0(ctx, base);
	// 830652E0: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830652E4: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 830652E8: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 830652EC: 4BF72FAD  bl 0x82fd8298
	ctx.lr = 0x830652F0;
	sub_82FD8298(ctx, base);
	// 830652F0: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830652F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830652F8: 41820020  beq 0x83065318
	if ctx.cr[0].eq {
	pc = 0x83065318; continue 'dispatch;
	}
	// 830652FC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83065300: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83065304: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83065308: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8306530C: 4802FBA5  bl 0x83094eb0
	ctx.lr = 0x83065310;
	sub_83094EB0(ctx, base);
	// 83065310: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83065314: 48000008  b 0x8306531c
	pc = 0x8306531C; continue 'dispatch;
	// 83065318: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8306531C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83065320: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83065324: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 83065328: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8306532C: 917D0024  stw r11, 0x24(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83065330: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83065334: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83065338: 4E800421  bctrl
	ctx.lr = 0x8306533C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8306533C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83065340: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83065344: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83065348: 4BFC7B01  bl 0x8302ce48
	ctx.lr = 0x8306534C;
	sub_8302CE48(ctx, base);
	// 8306534C: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83065350: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83065354: 409A0038  bne cr6, 0x8306538c
	if !ctx.cr[6].eq {
	pc = 0x8306538C; continue 'dispatch;
	}
	// 83065358: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8306535C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 83065360: 909F0054  stw r4, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 83065364: 4BF72F35  bl 0x82fd8298
	ctx.lr = 0x83065368;
	sub_82FD8298(ctx, base);
	// 83065368: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8306536C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065370: 41820014  beq 0x83065384
	if ctx.cr[0].eq {
	pc = 0x83065384; continue 'dispatch;
	}
	// 83065374: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83065378: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8306537C: 4803929D  bl 0x8309e618
	ctx.lr = 0x83065380;
	sub_8309E618(ctx, base);
	// 83065380: 48000008  b 0x83065388
	pc = 0x83065388; continue 'dispatch;
	// 83065384: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83065388: 907E001C  stw r3, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 8306538C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83065390: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83065394: 4BFD8B4D  bl 0x8303dee0
	ctx.lr = 0x83065398;
	sub_8303DEE0(ctx, base);
	// 83065398: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8306539C: 48000008  b 0x830653a4
	pc = 0x830653A4; continue 'dispatch;
	// 830653A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830653A4: 997A0000  stb r11, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 830653A8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830653AC: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 830653B0: 48142E00  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830653B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830653B4 size=44
    let mut pc: u32 = 0x830653B4;
    'dispatch: loop {
        match pc {
            0x830653B4 => {
    //   block [0x830653B4..0x830653E0)
	// 830653B4: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830653B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830653BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830653C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830653C4: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830653C8: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830653CC: 4BF72F15  bl 0x82fd82e0
	ctx.lr = 0x830653D0;
	sub_82FD82E0(ctx, base);
	// 830653D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830653D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830653D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830653DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830653E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830653E0 size=44
    let mut pc: u32 = 0x830653E0;
    'dispatch: loop {
        match pc {
            0x830653E0 => {
    //   block [0x830653E0..0x8306540C)
	// 830653E0: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830653E4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830653E8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830653EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830653F0: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830653F4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830653F8: 4BF72EE9  bl 0x82fd82e0
	ctx.lr = 0x830653FC;
	sub_82FD82E0(ctx, base);
	// 830653FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83065400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83065404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83065408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83065410 size=8
    let mut pc: u32 = 0x83065410;
    'dispatch: loop {
        match pc {
            0x83065410 => {
    //   block [0x83065410..0x83065418)
	// 83065410: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83065414: 82166828  lwz r16, 0x6828(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(26664 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83065418 size=156
    let mut pc: u32 = 0x83065418;
    'dispatch: loop {
        match pc {
            0x83065418 => {
    //   block [0x83065418..0x830654B4)
	// 83065418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306541C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83065420: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83065424: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83065428: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8306542C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83065430: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83065434: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83065438: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8306543C: 409A0048  bne cr6, 0x83065484
	if !ctx.cr[6].eq {
	pc = 0x83065484; continue 'dispatch;
	}
	// 83065440: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83065444: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83065448: 409A0008  bne cr6, 0x83065450
	if !ctx.cr[6].eq {
	pc = 0x83065450; continue 'dispatch;
	}
	// 8306544C: 4BFFFC95  bl 0x830650e0
	ctx.lr = 0x83065450;
	sub_830650E0(ctx, base);
	// 83065450: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83065454: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 83065458: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 8306545C: 4BF72E3D  bl 0x82fd8298
	ctx.lr = 0x83065460;
	sub_82FD8298(ctx, base);
	// 83065460: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83065464: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065468: 41820014  beq 0x8306547c
	if ctx.cr[0].eq {
	pc = 0x8306547C; continue 'dispatch;
	}
	// 8306546C: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83065470: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83065474: 480391A5  bl 0x8309e618
	ctx.lr = 0x83065478;
	sub_8309E618(ctx, base);
	// 83065478: 48000008  b 0x83065480
	pc = 0x83065480; continue 'dispatch;
	// 8306547C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83065480: 907E001C  stw r3, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 83065484: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83065488: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8306548C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83065490: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83065494: 4E800421  bctrl
	ctx.lr = 0x83065498;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83065498: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8306549C: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830654A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830654A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830654A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830654AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830654B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830654B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830654B4 size=44
    let mut pc: u32 = 0x830654B4;
    'dispatch: loop {
        match pc {
            0x830654B4 => {
    //   block [0x830654B4..0x830654E0)
	// 830654B4: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830654B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830654BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830654C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830654C4: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830654C8: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830654CC: 4BF72E15  bl 0x82fd82e0
	ctx.lr = 0x830654D0;
	sub_82FD82E0(ctx, base);
	// 830654D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830654D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830654D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830654DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830654E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830654E0 size=8
    let mut pc: u32 = 0x830654E0;
    'dispatch: loop {
        match pc {
            0x830654E0 => {
    //   block [0x830654E0..0x830654E8)
	// 830654E0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830654E4: 82166868  lwz r16, 0x6868(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(26728 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830654E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830654E8 size=156
    let mut pc: u32 = 0x830654E8;
    'dispatch: loop {
        match pc {
            0x830654E8 => {
    //   block [0x830654E8..0x83065584)
	// 830654E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830654EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830654F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830654F4: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830654F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830654FC: 80830018  lwz r4, 0x18(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 83065500: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065504: 4082000C  bne 0x83065510
	if !ctx.cr[0].eq {
	pc = 0x83065510; continue 'dispatch;
	}
	// 83065508: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8306550C: 48000064  b 0x83065570
	pc = 0x83065570; continue 'dispatch;
	// 83065510: 80C30004  lwz r6, 4(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83065514: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83065518: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8306551C: 4BF81625  bl 0x82fe6b40
	ctx.lr = 0x83065520;
	sub_82FE6B40(ctx, base);
	// 83065520: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83065524: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83065528: 409A001C  bne cr6, 0x83065544
	if !ctx.cr[6].eq {
	pc = 0x83065544; continue 'dispatch;
	}
	// 8306552C: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83065530: 815F005C  lwz r10, 0x5c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83065534: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83065538: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8306553C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83065540: 419A0008  beq cr6, 0x83065548
	if ctx.cr[6].eq {
	pc = 0x83065548; continue 'dispatch;
	}
	// 83065544: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83065548: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8306554C: 41820018  beq 0x83065564
	if ctx.cr[0].eq {
	pc = 0x83065564; continue 'dispatch;
	}
	// 83065550: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83065554: 4BFA8425  bl 0x8300d978
	ctx.lr = 0x83065558;
	sub_8300D978(ctx, base);
	// 83065558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8306555C: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 83065560: 4BFFFFC0  b 0x83065520
	pc = 0x83065520; continue 'dispatch;
	// 83065564: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83065568: 4BFA8C61  bl 0x8300e1c8
	ctx.lr = 0x8306556C;
	sub_8300E1C8(ctx, base);
	// 8306556C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83065570: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83065574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83065578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8306557C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83065580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065584(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83065584 size=40
    let mut pc: u32 = 0x83065584;
    'dispatch: loop {
        match pc {
            0x83065584 => {
    //   block [0x83065584..0x830655AC)
	// 83065584: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83065588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306558C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83065590: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83065594: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83065598: 4BFA8C31  bl 0x8300e1c8
	ctx.lr = 0x8306559C;
	sub_8300E1C8(ctx, base);
	// 8306559C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830655A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830655A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830655A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830655B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830655B0 size=8
    let mut pc: u32 = 0x830655B0;
    'dispatch: loop {
        match pc {
            0x830655B0 => {
    //   block [0x830655B0..0x830655B8)
	// 830655B0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830655B4: 821668A8  lwz r16, 0x68a8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(26792 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830655B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830655B8 size=168
    let mut pc: u32 = 0x830655B8;
    'dispatch: loop {
        match pc {
            0x830655B8 => {
    //   block [0x830655B8..0x83065660)
	// 830655B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830655BC: 48142BB1  bl 0x831a816c
	ctx.lr = 0x830655C0;
	sub_831A8130(ctx, base);
	// 830655C0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830655C4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830655C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830655CC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830655D0: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 830655D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830655D8: 409A0008  bne cr6, 0x830655e0
	if !ctx.cr[6].eq {
	pc = 0x830655E0; continue 'dispatch;
	}
	// 830655DC: 4BFFFB05  bl 0x830650e0
	ctx.lr = 0x830655E0;
	sub_830650E0(ctx, base);
	// 830655E0: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830655E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830655E8: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830655EC: 917D0024  stw r11, 0x24(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 830655F0: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 830655F4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 830655F8: 4E800421  bctrl
	ctx.lr = 0x830655FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830655FC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83065600: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83065604: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83065608: 4BFC7841  bl 0x8302ce48
	ctx.lr = 0x8306560C;
	sub_8302CE48(ctx, base);
	// 8306560C: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83065610: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83065614: 409A0038  bne cr6, 0x8306564c
	if !ctx.cr[6].eq {
	pc = 0x8306564C; continue 'dispatch;
	}
	// 83065618: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8306561C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 83065620: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 83065624: 4BF72C75  bl 0x82fd8298
	ctx.lr = 0x83065628;
	sub_82FD8298(ctx, base);
	// 83065628: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8306562C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065630: 41820014  beq 0x83065644
	if ctx.cr[0].eq {
	pc = 0x83065644; continue 'dispatch;
	}
	// 83065634: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83065638: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8306563C: 48038FDD  bl 0x8309e618
	ctx.lr = 0x83065640;
	sub_8309E618(ctx, base);
	// 83065640: 48000008  b 0x83065648
	pc = 0x83065648; continue 'dispatch;
	// 83065644: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83065648: 907E001C  stw r3, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 8306564C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83065650: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83065654: 4BFD888D  bl 0x8303dee0
	ctx.lr = 0x83065658;
	sub_8303DEE0(ctx, base);
	// 83065658: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8306565C: 48142B60  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83065660 size=44
    let mut pc: u32 = 0x83065660;
    'dispatch: loop {
        match pc {
            0x83065660 => {
    //   block [0x83065660..0x8306568C)
	// 83065660: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83065664: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83065668: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8306566C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83065670: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83065674: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83065678: 4BF72C69  bl 0x82fd82e0
	ctx.lr = 0x8306567C;
	sub_82FD82E0(ctx, base);
	// 8306567C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83065680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83065684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83065688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83065690 size=76
    let mut pc: u32 = 0x83065690;
    'dispatch: loop {
        match pc {
            0x83065690 => {
    //   block [0x83065690..0x830656DC)
	// 83065690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83065694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83065698: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8306569C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830656A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830656A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830656A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830656AC: 4BFFFAD5  bl 0x83065180
	ctx.lr = 0x830656B0;
	sub_83065180(ctx, base);
	// 830656B0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830656B4: 4182000C  beq 0x830656c0
	if ctx.cr[0].eq {
	pc = 0x830656C0; continue 'dispatch;
	}
	// 830656B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830656BC: 4BF72C25  bl 0x82fd82e0
	ctx.lr = 0x830656C0;
	sub_82FD82E0(ctx, base);
	// 830656C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830656C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830656C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830656CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830656D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830656D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830656D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830656E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830656E0 size=152
    let mut pc: u32 = 0x830656E0;
    'dispatch: loop {
        match pc {
            0x830656E0 => {
    //   block [0x830656E0..0x83065778)
	// 830656E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830656E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830656E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830656EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830656F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830656F4: 48018ECD  bl 0x8307e5c0
	ctx.lr = 0x830656F8;
	sub_8307E5C0(ctx, base);
	// 830656F8: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830656FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83065700: 392B6900  addi r9, r11, 0x6900
	ctx.r[9].s64 = ctx.r[11].s64 + 26880;
	// 83065704: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83065708: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 8306570C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065710: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 83065714: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83065718: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8306571C: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83065720: 911F0024  stw r8, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[8].u32 ) };
	// 83065724: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83065728: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8306572C: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 83065730: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 83065734: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 83065738: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8306573C: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 83065740: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 83065744: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 83065748: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 8306574C: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83065750: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 83065754: 915F0058  stw r10, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 83065758: 997F005C  stb r11, 0x5c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u8 ) };
	// 8306575C: 997F005D  stb r11, 0x5d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(93 as u32), ctx.r[11].u8 ) };
	// 83065760: 997F005E  stb r11, 0x5e(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(94 as u32), ctx.r[11].u8 ) };
	// 83065764: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83065768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8306576C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83065770: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83065774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83065778 size=20
    let mut pc: u32 = 0x83065778;
    'dispatch: loop {
        match pc {
            0x83065778 => {
    //   block [0x83065778..0x8306578C)
	// 83065778: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 8306577C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065780: 4182000C  beq 0x8306578c
	if ctx.cr[0].eq {
		sub_8306578C(ctx, base);
		return;
	}
	// 83065784: 806B003C  lwz r3, 0x3c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 83065788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8306578C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8306578C size=8
    let mut pc: u32 = 0x8306578C;
    'dispatch: loop {
        match pc {
            0x8306578C => {
    //   block [0x8306578C..0x83065794)
	// 8306578C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83065790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83065798 size=100
    let mut pc: u32 = 0x83065798;
    'dispatch: loop {
        match pc {
            0x83065798 => {
    //   block [0x83065798..0x830657FC)
	// 83065798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306579C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830657A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830657A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830657A8: 83E30038  lwz r31, 0x38(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 830657AC: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830657B0: 41820034  beq 0x830657e4
	if ctx.cr[0].eq {
	pc = 0x830657E4; continue 'dispatch;
	}
	// 830657B4: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830657B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830657BC: 409A0020  bne cr6, 0x830657dc
	if !ctx.cr[6].eq {
	pc = 0x830657DC; continue 'dispatch;
	}
	// 830657C0: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 830657C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830657C8: 419A0014  beq cr6, 0x830657dc
	if ctx.cr[6].eq {
	pc = 0x830657DC; continue 'dispatch;
	}
	// 830657CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830657D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830657D4: 4BFDA7ED  bl 0x8303ffc0
	ctx.lr = 0x830657D8;
	sub_8303FFC0(ctx, base);
	// 830657D8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830657DC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830657E0: 48000008  b 0x830657e8
	pc = 0x830657E8; continue 'dispatch;
	// 830657E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830657E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830657EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830657F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830657F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830657F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83065800 size=24
    let mut pc: u32 = 0x83065800;
    'dispatch: loop {
        match pc {
            0x83065800 => {
    //   block [0x83065800..0x83065818)
	// 83065800: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 83065804: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83065808: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8306580C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83065810: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83065814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83065818 size=20
    let mut pc: u32 = 0x83065818;
    'dispatch: loop {
        match pc {
            0x83065818 => {
    //   block [0x83065818..0x8306582C)
	// 83065818: 81630040  lwz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 8306581C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065820: 4182000C  beq 0x8306582c
	if ctx.cr[0].eq {
		sub_8306582C(ctx, base);
		return;
	}
	// 83065824: 806B0028  lwz r3, 0x28(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83065828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8306582C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8306582C size=32
    let mut pc: u32 = 0x8306582C;
    'dispatch: loop {
        match pc {
            0x8306582C => {
    //   block [0x8306582C..0x8306584C)
	// 8306582C: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 83065830: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065834: 4082FFF0  bne 0x83065824
	if !ctx.cr[0].eq {
		sub_83065818(ctx, base);
		return;
	}
	// 83065838: 81630044  lwz r11, 0x44(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 8306583C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065840: 4182000C  beq 0x8306584c
	if ctx.cr[0].eq {
		sub_8306584C(ctx, base);
		return;
	}
	// 83065844: 806B0030  lwz r3, 0x30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 83065848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8306584C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8306584C size=24
    let mut pc: u32 = 0x8306584C;
    'dispatch: loop {
        match pc {
            0x8306584C => {
    //   block [0x8306584C..0x83065864)
	// 8306584C: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 83065850: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065854: 4082FFF0  bne 0x83065844
	if !ctx.cr[0].eq {
		sub_8306582C(ctx, base);
		return;
	}
	// 83065858: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8306585C: 386BD5D4  addi r3, r11, -0x2a2c
	ctx.r[3].s64 = ctx.r[11].s64 + -10796;
	// 83065860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83065868 size=20
    let mut pc: u32 = 0x83065868;
    'dispatch: loop {
        match pc {
            0x83065868 => {
    //   block [0x83065868..0x8306587C)
	// 83065868: 81630040  lwz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 8306586C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065870: 4182000C  beq 0x8306587c
	if ctx.cr[0].eq {
		sub_8306587C(ctx, base);
		return;
	}
	// 83065874: 806B002C  lwz r3, 0x2c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83065878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8306587C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8306587C size=32
    let mut pc: u32 = 0x8306587C;
    'dispatch: loop {
        match pc {
            0x8306587C => {
    //   block [0x8306587C..0x8306589C)
	// 8306587C: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 83065880: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065884: 4082FFF0  bne 0x83065874
	if !ctx.cr[0].eq {
		sub_83065868(ctx, base);
		return;
	}
	// 83065888: 81630044  lwz r11, 0x44(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 8306588C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065890: 4182000C  beq 0x8306589c
	if ctx.cr[0].eq {
		sub_8306589C(ctx, base);
		return;
	}
	// 83065894: 806B0034  lwz r3, 0x34(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83065898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8306589C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8306589C size=24
    let mut pc: u32 = 0x8306589C;
    'dispatch: loop {
        match pc {
            0x8306589C => {
    //   block [0x8306589C..0x830658B4)
	// 8306589C: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 830658A0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830658A4: 4082FFF0  bne 0x83065894
	if !ctx.cr[0].eq {
		sub_8306587C(ctx, base);
		return;
	}
	// 830658A8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830658AC: 386BCC98  addi r3, r11, -0x3368
	ctx.r[3].s64 = ctx.r[11].s64 + -13160;
	// 830658B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830658B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830658B8 size=68
    let mut pc: u32 = 0x830658B8;
    'dispatch: loop {
        match pc {
            0x830658B8 => {
    //   block [0x830658B8..0x830658FC)
	// 830658B8: 81630044  lwz r11, 0x44(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 830658BC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830658C0: 41820010  beq 0x830658d0
	if ctx.cr[0].eq {
	pc = 0x830658D0; continue 'dispatch;
	}
	// 830658C4: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830658C8: 2F0A0019  cmpwi cr6, r10, 0x19
	ctx.cr[6].compare_i32(ctx.r[10].s32, 25, &mut ctx.xer);
	// 830658CC: 419A001C  beq cr6, 0x830658e8
	if ctx.cr[6].eq {
	pc = 0x830658E8; continue 'dispatch;
	}
	// 830658D0: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 830658D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830658D8: 41820024  beq 0x830658fc
	if ctx.cr[0].eq {
		sub_830658FC(ctx, base);
		return;
	}
	// 830658DC: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830658E0: 2F0A0019  cmpwi cr6, r10, 0x19
	ctx.cr[6].compare_i32(ctx.r[10].s32, 25, &mut ctx.xer);
	// 830658E4: 409A0018  bne cr6, 0x830658fc
	if !ctx.cr[6].eq {
		sub_830658FC(ctx, base);
		return;
	}
	// 830658E8: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 830658EC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830658F0: 4182000C  beq 0x830658fc
	if ctx.cr[0].eq {
		sub_830658FC(ctx, base);
		return;
	}
	// 830658F4: 886B0008  lbz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830658F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830658FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830658FC size=8
    let mut pc: u32 = 0x830658FC;
    'dispatch: loop {
        match pc {
            0x830658FC => {
    //   block [0x830658FC..0x83065904)
	// 830658FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83065900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83065908 size=280
    let mut pc: u32 = 0x83065908;
    'dispatch: loop {
        match pc {
            0x83065908 => {
    //   block [0x83065908..0x83065A20)
	// 83065908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306590C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83065910: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83065914: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83065918: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8306591C: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83065920: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83065924: 419A003C  beq cr6, 0x83065960
	if ctx.cr[6].eq {
	pc = 0x83065960; continue 'dispatch;
	}
	// 83065928: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8306592C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065930: 4182000C  beq 0x8306593c
	if ctx.cr[0].eq {
	pc = 0x8306593C; continue 'dispatch;
	}
	// 83065934: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83065938: 48000008  b 0x83065940
	pc = 0x83065940; continue 'dispatch;
	// 8306593C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83065940: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 83065944: 409A0010  bne cr6, 0x83065954
	if !ctx.cr[6].eq {
	pc = 0x83065954; continue 'dispatch;
	}
	// 83065948: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8306594C: 386BD8FC  addi r3, r11, -0x2704
	ctx.r[3].s64 = ctx.r[11].s64 + -9988;
	// 83065950: 480000BC  b 0x83065a0c
	pc = 0x83065A0C; continue 'dispatch;
	// 83065954: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83065958: 386BD5D4  addi r3, r11, -0x2a2c
	ctx.r[3].s64 = ctx.r[11].s64 + -10796;
	// 8306595C: 480000B0  b 0x83065a0c
	pc = 0x83065A0C; continue 'dispatch;
	// 83065960: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 83065964: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065968: 4182000C  beq 0x83065974
	if ctx.cr[0].eq {
	pc = 0x83065974; continue 'dispatch;
	}
	// 8306596C: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83065970: 48000030  b 0x830659a0
	pc = 0x830659A0; continue 'dispatch;
	// 83065974: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83065978: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8306597C: 4082FFF0  bne 0x8306596c
	if !ctx.cr[0].eq {
	pc = 0x8306596C; continue 'dispatch;
	}
	// 83065980: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83065984: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065988: 4182000C  beq 0x83065994
	if ctx.cr[0].eq {
	pc = 0x83065994; continue 'dispatch;
	}
	// 8306598C: 896B0008  lbz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83065990: 48000010  b 0x830659a0
	pc = 0x830659A0; continue 'dispatch;
	// 83065994: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83065998: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8306599C: 4082FFF0  bne 0x8306598c
	if !ctx.cr[0].eq {
	pc = 0x8306598C; continue 'dispatch;
	}
	// 830659A0: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830659A4: 40820064  bne 0x83065a08
	if !ctx.cr[0].eq {
	pc = 0x83065A08; continue 'dispatch;
	}
	// 830659A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830659AC: 4BFFFF0D  bl 0x830658b8
	ctx.lr = 0x830659B0;
	sub_830658B8(ctx, base);
	// 830659B0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830659B4: 40820054  bne 0x83065a08
	if !ctx.cr[0].eq {
	pc = 0x83065A08; continue 'dispatch;
	}
	// 830659B8: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830659BC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830659C0: 41820024  beq 0x830659e4
	if ctx.cr[0].eq {
	pc = 0x830659E4; continue 'dispatch;
	}
	// 830659C4: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830659C8: 2F0A0019  cmpwi cr6, r10, 0x19
	ctx.cr[6].compare_i32(ctx.r[10].s32, 25, &mut ctx.xer);
	// 830659CC: 409A0018  bne cr6, 0x830659e4
	if !ctx.cr[6].eq {
	pc = 0x830659E4; continue 'dispatch;
	}
	// 830659D0: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 830659D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830659D8: 41820030  beq 0x83065a08
	if ctx.cr[0].eq {
	pc = 0x83065A08; continue 'dispatch;
	}
	// 830659DC: 806B0030  lwz r3, 0x30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830659E0: 4800002C  b 0x83065a0c
	pc = 0x83065A0C; continue 'dispatch;
	// 830659E4: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 830659E8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830659EC: 41820010  beq 0x830659fc
	if ctx.cr[0].eq {
	pc = 0x830659FC; continue 'dispatch;
	}
	// 830659F0: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830659F4: 2F0A0019  cmpwi cr6, r10, 0x19
	ctx.cr[6].compare_i32(ctx.r[10].s32, 25, &mut ctx.xer);
	// 830659F8: 419AFFD8  beq cr6, 0x830659d0
	if ctx.cr[6].eq {
	pc = 0x830659D0; continue 'dispatch;
	}
	// 830659FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065A00: 4BFFFE19  bl 0x83065818
	ctx.lr = 0x83065A04;
	sub_83065818(ctx, base);
	// 83065A04: 48000008  b 0x83065a0c
	pc = 0x83065A0C; continue 'dispatch;
	// 83065A08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83065A0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83065A10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83065A14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83065A18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83065A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83065A20 size=236
    let mut pc: u32 = 0x83065A20;
    'dispatch: loop {
        match pc {
            0x83065A20 => {
    //   block [0x83065A20..0x83065B0C)
	// 83065A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83065A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83065A28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83065A2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83065A30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83065A34: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83065A38: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83065A3C: 419A0010  beq cr6, 0x83065a4c
	if ctx.cr[6].eq {
	pc = 0x83065A4C; continue 'dispatch;
	}
	// 83065A40: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83065A44: 386BCC98  addi r3, r11, -0x3368
	ctx.r[3].s64 = ctx.r[11].s64 + -13160;
	// 83065A48: 480000B0  b 0x83065af8
	pc = 0x83065AF8; continue 'dispatch;
	// 83065A4C: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 83065A50: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065A54: 4182000C  beq 0x83065a60
	if ctx.cr[0].eq {
	pc = 0x83065A60; continue 'dispatch;
	}
	// 83065A58: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83065A5C: 48000030  b 0x83065a8c
	pc = 0x83065A8C; continue 'dispatch;
	// 83065A60: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83065A64: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065A68: 4082FFF0  bne 0x83065a58
	if !ctx.cr[0].eq {
	pc = 0x83065A58; continue 'dispatch;
	}
	// 83065A6C: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83065A70: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065A74: 4182000C  beq 0x83065a80
	if ctx.cr[0].eq {
	pc = 0x83065A80; continue 'dispatch;
	}
	// 83065A78: 896B0008  lbz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83065A7C: 48000010  b 0x83065a8c
	pc = 0x83065A8C; continue 'dispatch;
	// 83065A80: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83065A84: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065A88: 4082FFF0  bne 0x83065a78
	if !ctx.cr[0].eq {
	pc = 0x83065A78; continue 'dispatch;
	}
	// 83065A8C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83065A90: 40820064  bne 0x83065af4
	if !ctx.cr[0].eq {
	pc = 0x83065AF4; continue 'dispatch;
	}
	// 83065A94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065A98: 4BFFFE21  bl 0x830658b8
	ctx.lr = 0x83065A9C;
	sub_830658B8(ctx, base);
	// 83065A9C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83065AA0: 40820054  bne 0x83065af4
	if !ctx.cr[0].eq {
	pc = 0x83065AF4; continue 'dispatch;
	}
	// 83065AA4: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83065AA8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065AAC: 41820024  beq 0x83065ad0
	if ctx.cr[0].eq {
	pc = 0x83065AD0; continue 'dispatch;
	}
	// 83065AB0: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83065AB4: 2F0A0019  cmpwi cr6, r10, 0x19
	ctx.cr[6].compare_i32(ctx.r[10].s32, 25, &mut ctx.xer);
	// 83065AB8: 409A0018  bne cr6, 0x83065ad0
	if !ctx.cr[6].eq {
	pc = 0x83065AD0; continue 'dispatch;
	}
	// 83065ABC: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 83065AC0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065AC4: 41820030  beq 0x83065af4
	if ctx.cr[0].eq {
	pc = 0x83065AF4; continue 'dispatch;
	}
	// 83065AC8: 806B0034  lwz r3, 0x34(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83065ACC: 4800002C  b 0x83065af8
	pc = 0x83065AF8; continue 'dispatch;
	// 83065AD0: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83065AD4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065AD8: 41820010  beq 0x83065ae8
	if ctx.cr[0].eq {
	pc = 0x83065AE8; continue 'dispatch;
	}
	// 83065ADC: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83065AE0: 2F0A0019  cmpwi cr6, r10, 0x19
	ctx.cr[6].compare_i32(ctx.r[10].s32, 25, &mut ctx.xer);
	// 83065AE4: 419AFFD8  beq cr6, 0x83065abc
	if ctx.cr[6].eq {
	pc = 0x83065ABC; continue 'dispatch;
	}
	// 83065AE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065AEC: 4BFFFD7D  bl 0x83065868
	ctx.lr = 0x83065AF0;
	sub_83065868(ctx, base);
	// 83065AF0: 48000008  b 0x83065af8
	pc = 0x83065AF8; continue 'dispatch;
	// 83065AF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83065AF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83065AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83065B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83065B04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83065B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83065B10 size=8
    let mut pc: u32 = 0x83065B10;
    'dispatch: loop {
        match pc {
            0x83065B10 => {
    //   block [0x83065B10..0x83065B18)
	// 83065B10: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83065B14: 82166958  lwz r16, 0x6958(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(26968 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83065B18 size=184
    let mut pc: u32 = 0x83065B18;
    'dispatch: loop {
        match pc {
            0x83065B18 => {
    //   block [0x83065B18..0x83065BD0)
	// 83065B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83065B1C: 48142641  bl 0x831a815c
	ctx.lr = 0x83065B20;
	sub_831A8130(ctx, base);
	// 83065B20: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83065B24: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83065B28: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83065B2C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83065B30: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 83065B34: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83065B38: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 83065B3C: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 83065B40: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 83065B44: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 83065B48: 48018A79  bl 0x8307e5c0
	ctx.lr = 0x83065B4C;
	sub_8307E5C0(ctx, base);
	// 83065B4C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83065B50: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83065B54: 935E0018  stw r26, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[26].u32 ) };
	// 83065B58: 394B6900  addi r10, r11, 0x6900
	ctx.r[10].s64 = ctx.r[11].s64 + 26880;
	// 83065B5C: 933E0024  stw r25, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[25].u32 ) };
	// 83065B60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83065B64: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83065B68: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83065B6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83065B70: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83065B74: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83065B78: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83065B7C: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83065B80: 917E0028  stw r11, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83065B84: 917E002C  stw r11, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83065B88: 917E0030  stw r11, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 83065B8C: 917E0034  stw r11, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 83065B90: 917E0038  stw r11, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 83065B94: 917E003C  stw r11, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 83065B98: 917E0040  stw r11, 0x40(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 83065B9C: 917E0044  stw r11, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 83065BA0: 917E0048  stw r11, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 83065BA4: 917E004C  stw r11, 0x4c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 83065BA8: 917E0050  stw r11, 0x50(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83065BAC: 915E0054  stw r10, 0x54(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 83065BB0: 915E0058  stw r10, 0x58(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 83065BB4: 997E005C  stb r11, 0x5c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(92 as u32), ctx.r[11].u8 ) };
	// 83065BB8: 997E005D  stb r11, 0x5d(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(93 as u32), ctx.r[11].u8 ) };
	// 83065BBC: 997E005E  stb r11, 0x5e(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(94 as u32), ctx.r[11].u8 ) };
	// 83065BC0: 480188B1  bl 0x8307e470
	ctx.lr = 0x83065BC4;
	sub_8307E470(ctx, base);
	// 83065BC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83065BC8: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83065BCC: 481425E0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83065BD0 size=40
    let mut pc: u32 = 0x83065BD0;
    'dispatch: loop {
        match pc {
            0x83065BD0 => {
    //   block [0x83065BD0..0x83065BF8)
	// 83065BD0: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83065BD4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83065BD8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83065BDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83065BE0: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83065BE4: 4801879D  bl 0x8307e380
	ctx.lr = 0x83065BE8;
	sub_8307E380(ctx, base);
	// 83065BE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83065BEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83065BF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83065BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83065BF8 size=92
    let mut pc: u32 = 0x83065BF8;
    'dispatch: loop {
        match pc {
            0x83065BF8 => {
    //   block [0x83065BF8..0x83065C54)
	// 83065BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83065BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83065C00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83065C04: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83065C08: 806A0038  lwz r3, 0x38(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(56 as u32) ) } as u64;
	// 83065C0C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065C10: 40820030  bne 0x83065c40
	if !ctx.cr[0].eq {
	pc = 0x83065C40; continue 'dispatch;
	}
	// 83065C14: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83065C18: 80EA0004  lwz r7, 4(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83065C1C: 38C0012D  li r6, 0x12d
	ctx.r[6].s64 = 301;
	// 83065C20: 388B6988  addi r4, r11, 0x6988
	ctx.r[4].s64 = ctx.r[11].s64 + 27016;
	// 83065C24: 38A00134  li r5, 0x134
	ctx.r[5].s64 = 308;
	// 83065C28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83065C2C: 4BF6B42D  bl 0x82fd1058
	ctx.lr = 0x83065C30;
	sub_82FD1058(ctx, base);
	// 83065C30: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83065C34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83065C38: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 83065C3C: 4814AFED  bl 0x831b0c28
	ctx.lr = 0x83065C40;
	sub_831B0C28(ctx, base);
	// 83065C40: 4BFDA5F1  bl 0x83040230
	ctx.lr = 0x83065C44;
	sub_83040230(ctx, base);
	// 83065C44: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83065C48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83065C4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83065C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83065C58 size=52
    let mut pc: u32 = 0x83065C58;
    'dispatch: loop {
        match pc {
            0x83065C58 => {
    //   block [0x83065C58..0x83065C8C)
	// 83065C58: 81430038  lwz r10, 0x38(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 83065C5C: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 83065C60: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065C64: 41820008  beq 0x83065c6c
	if ctx.cr[0].eq {
	pc = 0x83065C6C; continue 'dispatch;
	}
	// 83065C68: 816A0020  lwz r11, 0x20(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 83065C6C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83065C70: 419A001C  beq cr6, 0x83065c8c
	if ctx.cr[6].eq {
		sub_83065C8C(ctx, base);
		return;
	}
	// 83065C74: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 83065C78: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83065C7C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83065C80: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83065C84: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 83065C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065C8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83065C8C size=8
    let mut pc: u32 = 0x83065C8C;
    'dispatch: loop {
        match pc {
            0x83065C8C => {
    //   block [0x83065C8C..0x83065C94)
	// 83065C8C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83065C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83065C98 size=16
    let mut pc: u32 = 0x83065C98;
    'dispatch: loop {
        match pc {
            0x83065C98 => {
    //   block [0x83065C98..0x83065CA8)
	// 83065C98: 80630038  lwz r3, 0x38(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 83065C9C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065CA0: 41820008  beq 0x83065ca8
	if ctx.cr[0].eq {
		sub_83065CA8(ctx, base);
		return;
	}
	// 83065CA4: 4BFD9D7C  b 0x8303fa20
	sub_8303FA20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83065CA8 size=8
    let mut pc: u32 = 0x83065CA8;
    'dispatch: loop {
        match pc {
            0x83065CA8 => {
    //   block [0x83065CA8..0x83065CB0)
	// 83065CA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83065CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83065CB0 size=8
    let mut pc: u32 = 0x83065CB0;
    'dispatch: loop {
        match pc {
            0x83065CB0 => {
    //   block [0x83065CB0..0x83065CB8)
	// 83065CB0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83065CB4: 821669D8  lwz r16, 0x69d8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(27096 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83065CB8 size=96
    let mut pc: u32 = 0x83065CB8;
    'dispatch: loop {
        match pc {
            0x83065CB8 => {
    //   block [0x83065CB8..0x83065D18)
	// 83065CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83065CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83065CC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83065CC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83065CC8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83065CCC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83065CD0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83065CD4: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 83065CD8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83065CDC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83065CE0: 4BF725B9  bl 0x82fd8298
	ctx.lr = 0x83065CE4;
	sub_82FD8298(ctx, base);
	// 83065CE4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83065CE8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065CEC: 41820010  beq 0x83065cfc
	if ctx.cr[0].eq {
	pc = 0x83065CFC; continue 'dispatch;
	}
	// 83065CF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83065CF4: 4BFFF9ED  bl 0x830656e0
	ctx.lr = 0x83065CF8;
	sub_830656E0(ctx, base);
	// 83065CF8: 48000008  b 0x83065d00
	pc = 0x83065D00; continue 'dispatch;
	// 83065CFC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83065D00: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83065D04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83065D08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83065D0C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83065D10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83065D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83065D18 size=44
    let mut pc: u32 = 0x83065D18;
    'dispatch: loop {
        match pc {
            0x83065D18 => {
    //   block [0x83065D18..0x83065D44)
	// 83065D18: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83065D1C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83065D20: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83065D24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83065D28: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83065D2C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83065D30: 4BF725B1  bl 0x82fd82e0
	ctx.lr = 0x83065D34;
	sub_82FD82E0(ctx, base);
	// 83065D34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83065D38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83065D3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83065D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83065D48 size=12
    let mut pc: u32 = 0x83065D48;
    'dispatch: loop {
        match pc {
            0x83065D48 => {
    //   block [0x83065D48..0x83065D54)
	// 83065D48: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83065D4C: 386B343C  addi r3, r11, 0x343c
	ctx.r[3].s64 = ctx.r[11].s64 + 13372;
	// 83065D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83065D58 size=620
    let mut pc: u32 = 0x83065D58;
    'dispatch: loop {
        match pc {
            0x83065D58 => {
    //   block [0x83065D58..0x83065FC4)
	// 83065D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83065D5C: 48142411  bl 0x831a816c
	ctx.lr = 0x83065D60;
	sub_831A8130(ctx, base);
	// 83065D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83065D64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83065D68: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83065D6C: 48018895  bl 0x8307e600
	ctx.lr = 0x83065D70;
	sub_8307E600(ctx, base);
	// 83065D70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065D74: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 83065D78: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83065D7C: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83065D80: 418200F0  beq 0x83065e70
	if ctx.cr[0].eq {
	pc = 0x83065E70; continue 'dispatch;
	}
	// 83065D84: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83065D88: 4BF93571  bl 0x82ff92f8
	ctx.lr = 0x83065D8C;
	sub_82FF92F8(ctx, base);
	// 83065D8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065D90: 809E0020  lwz r4, 0x20(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83065D94: 4BFDB4D5  bl 0x83041268
	ctx.lr = 0x83065D98;
	sub_83041268(ctx, base);
	// 83065D98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065D9C: 809E0024  lwz r4, 0x24(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83065DA0: 4BF93559  bl 0x82ff92f8
	ctx.lr = 0x83065DA4;
	sub_82FF92F8(ctx, base);
	// 83065DA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065DA8: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83065DAC: 4BF9354D  bl 0x82ff92f8
	ctx.lr = 0x83065DB0;
	sub_82FF92F8(ctx, base);
	// 83065DB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065DB4: 809E002C  lwz r4, 0x2c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83065DB8: 4BF93541  bl 0x82ff92f8
	ctx.lr = 0x83065DBC;
	sub_82FF92F8(ctx, base);
	// 83065DBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065DC0: 809E0030  lwz r4, 0x30(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83065DC4: 4BF93535  bl 0x82ff92f8
	ctx.lr = 0x83065DC8;
	sub_82FF92F8(ctx, base);
	// 83065DC8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83065DCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83065DD0: 809E0034  lwz r4, 0x34(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83065DD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065DD8: 4BF93B29  bl 0x82ff9900
	ctx.lr = 0x83065DDC;
	sub_82FF9900(ctx, base);
	// 83065DDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065DE0: 809E0038  lwz r4, 0x38(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 83065DE4: 4BF93E1D  bl 0x82ff9c00
	ctx.lr = 0x83065DE8;
	sub_82FF9C00(ctx, base);
	// 83065DE8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83065DEC: 807E003C  lwz r3, 0x3c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 83065DF0: 4BFEA6B9  bl 0x830504a8
	ctx.lr = 0x83065DF4;
	sub_830504A8(ctx, base);
	// 83065DF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065DF8: 809E0040  lwz r4, 0x40(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83065DFC: 4BF93E05  bl 0x82ff9c00
	ctx.lr = 0x83065E00;
	sub_82FF9C00(ctx, base);
	// 83065E00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065E04: 809E0044  lwz r4, 0x44(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 83065E08: 4BFDB461  bl 0x83041268
	ctx.lr = 0x83065E0C;
	sub_83041268(ctx, base);
	// 83065E0C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83065E10: 807E0048  lwz r3, 0x48(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 83065E14: 4BFE6E05  bl 0x8304cc18
	ctx.lr = 0x83065E18;
	sub_8304CC18(ctx, base);
	// 83065E18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065E1C: 809E004C  lwz r4, 0x4c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 83065E20: 4BF93DE1  bl 0x82ff9c00
	ctx.lr = 0x83065E24;
	sub_82FF9C00(ctx, base);
	// 83065E24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065E28: 809E0050  lwz r4, 0x50(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 83065E2C: 4BF93DD5  bl 0x82ff9c00
	ctx.lr = 0x83065E30;
	sub_82FF9C00(ctx, base);
	// 83065E30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065E34: 809E0054  lwz r4, 0x54(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) } as u64;
	// 83065E38: 4BF934C1  bl 0x82ff92f8
	ctx.lr = 0x83065E3C;
	sub_82FF92F8(ctx, base);
	// 83065E3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065E40: 809E0058  lwz r4, 0x58(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 83065E44: 4BF934B5  bl 0x82ff92f8
	ctx.lr = 0x83065E48;
	sub_82FF92F8(ctx, base);
	// 83065E48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065E4C: 889E005C  lbz r4, 0x5c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 83065E50: 4BF933B1  bl 0x82ff9200
	ctx.lr = 0x83065E54;
	sub_82FF9200(ctx, base);
	// 83065E54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065E58: 889E005D  lbz r4, 0x5d(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(93 as u32) ) } as u64;
	// 83065E5C: 4BF933A5  bl 0x82ff9200
	ctx.lr = 0x83065E60;
	sub_82FF9200(ctx, base);
	// 83065E60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065E64: 889E005E  lbz r4, 0x5e(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(94 as u32) ) } as u64;
	// 83065E68: 4BF93399  bl 0x82ff9200
	ctx.lr = 0x83065E6C;
	sub_82FF9200(ctx, base);
	// 83065E6C: 48000150  b 0x83065fbc
	pc = 0x83065FBC; continue 'dispatch;
	// 83065E70: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83065E74: 4BF93705  bl 0x82ff9578
	ctx.lr = 0x83065E78;
	sub_82FF9578(ctx, base);
	// 83065E78: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83065E7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065E80: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83065E84: 4BFDB4AD  bl 0x83041330
	ctx.lr = 0x83065E88;
	sub_83041330(ctx, base);
	// 83065E88: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83065E8C: 389E0024  addi r4, r30, 0x24
	ctx.r[4].s64 = ctx.r[30].s64 + 36;
	// 83065E90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065E94: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83065E98: 4BF936E1  bl 0x82ff9578
	ctx.lr = 0x83065E9C;
	sub_82FF9578(ctx, base);
	// 83065E9C: 389E0028  addi r4, r30, 0x28
	ctx.r[4].s64 = ctx.r[30].s64 + 40;
	// 83065EA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065EA4: 4BF936D5  bl 0x82ff9578
	ctx.lr = 0x83065EA8;
	sub_82FF9578(ctx, base);
	// 83065EA8: 389E002C  addi r4, r30, 0x2c
	ctx.r[4].s64 = ctx.r[30].s64 + 44;
	// 83065EAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065EB0: 4BF936C9  bl 0x82ff9578
	ctx.lr = 0x83065EB4;
	sub_82FF9578(ctx, base);
	// 83065EB4: 389E0030  addi r4, r30, 0x30
	ctx.r[4].s64 = ctx.r[30].s64 + 48;
	// 83065EB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065EBC: 4BF936BD  bl 0x82ff9578
	ctx.lr = 0x83065EC0;
	sub_82FF9578(ctx, base);
	// 83065EC0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83065EC4: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 83065EC8: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 83065ECC: 389E0034  addi r4, r30, 0x34
	ctx.r[4].s64 = ctx.r[30].s64 + 52;
	// 83065ED0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065ED4: 4BF93C55  bl 0x82ff9b28
	ctx.lr = 0x83065ED8;
	sub_82FF9B28(ctx, base);
	// 83065ED8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83065EDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065EE0: 3BAB32CC  addi r29, r11, 0x32cc
	ctx.r[29].s64 = ctx.r[11].s64 + 13004;
	// 83065EE4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83065EE8: 4BF93DD9  bl 0x82ff9cc0
	ctx.lr = 0x83065EEC;
	sub_82FF9CC0(ctx, base);
	// 83065EEC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83065EF0: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83065EF4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83065EF8: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 83065EFC: 387E003C  addi r3, r30, 0x3c
	ctx.r[3].s64 = ctx.r[30].s64 + 60;
	// 83065F00: 917E0038  stw r11, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 83065F04: 4BFE8F15  bl 0x8304ee18
	ctx.lr = 0x83065F08;
	sub_8304EE18(ctx, base);
	// 83065F08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065F0C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83065F10: 4BF93DB1  bl 0x82ff9cc0
	ctx.lr = 0x83065F14;
	sub_82FF9CC0(ctx, base);
	// 83065F14: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83065F18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065F1C: 917E0040  stw r11, 0x40(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 83065F20: 4BFDB411  bl 0x83041330
	ctx.lr = 0x83065F24;
	sub_83041330(ctx, base);
	// 83065F24: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83065F28: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83065F2C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83065F30: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 83065F34: 387E0048  addi r3, r30, 0x48
	ctx.r[3].s64 = ctx.r[30].s64 + 72;
	// 83065F38: 917E0044  stw r11, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 83065F3C: 4BFE7C95  bl 0x8304dbd0
	ctx.lr = 0x83065F40;
	sub_8304DBD0(ctx, base);
	// 83065F40: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83065F44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065F48: 388B34C0  addi r4, r11, 0x34c0
	ctx.r[4].s64 = ctx.r[11].s64 + 13504;
	// 83065F4C: 4BF93D75  bl 0x82ff9cc0
	ctx.lr = 0x83065F50;
	sub_82FF9CC0(ctx, base);
	// 83065F50: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83065F54: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83065F58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065F5C: 388B343C  addi r4, r11, 0x343c
	ctx.r[4].s64 = ctx.r[11].s64 + 13372;
	// 83065F60: 915E004C  stw r10, 0x4c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 83065F64: 4BF93D5D  bl 0x82ff9cc0
	ctx.lr = 0x83065F68;
	sub_82FF9CC0(ctx, base);
	// 83065F68: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83065F6C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83065F70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065F74: 917E0050  stw r11, 0x50(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83065F78: 4BF93601  bl 0x82ff9578
	ctx.lr = 0x83065F7C;
	sub_82FF9578(ctx, base);
	// 83065F7C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83065F80: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83065F84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065F88: 917E0054  stw r11, 0x54(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83065F8C: 4BF935ED  bl 0x82ff9578
	ctx.lr = 0x83065F90;
	sub_82FF9578(ctx, base);
	// 83065F90: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83065F94: 389E005C  addi r4, r30, 0x5c
	ctx.r[4].s64 = ctx.r[30].s64 + 92;
	// 83065F98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065F9C: 917E0058  stw r11, 0x58(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 83065FA0: 4BF934E1  bl 0x82ff9480
	ctx.lr = 0x83065FA4;
	sub_82FF9480(ctx, base);
	// 83065FA4: 389E005D  addi r4, r30, 0x5d
	ctx.r[4].s64 = ctx.r[30].s64 + 93;
	// 83065FA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065FAC: 4BF934D5  bl 0x82ff9480
	ctx.lr = 0x83065FB0;
	sub_82FF9480(ctx, base);
	// 83065FB0: 389E005E  addi r4, r30, 0x5e
	ctx.r[4].s64 = ctx.r[30].s64 + 94;
	// 83065FB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83065FB8: 4BF934C9  bl 0x82ff9480
	ctx.lr = 0x83065FBC;
	sub_82FF9480(ctx, base);
	// 83065FBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83065FC0: 481421FC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83065FC8 size=16
    let mut pc: u32 = 0x83065FC8;
    'dispatch: loop {
        match pc {
            0x83065FC8 => {
    //   block [0x83065FC8..0x83065FD8)
	// 83065FC8: 80630038  lwz r3, 0x38(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 83065FCC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83065FD0: 41820008  beq 0x83065fd8
	if ctx.cr[0].eq {
		sub_83065FD8(ctx, base);
		return;
	}
	// 83065FD4: 4BFD50B4  b 0x8303b088
	sub_8303B088(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83065FD8 size=8
    let mut pc: u32 = 0x83065FD8;
    'dispatch: loop {
        match pc {
            0x83065FD8 => {
    //   block [0x83065FD8..0x83065FE0)
	// 83065FD8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83065FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83065FE0 size=8
    let mut pc: u32 = 0x83065FE0;
    'dispatch: loop {
        match pc {
            0x83065FE0 => {
    //   block [0x83065FE0..0x83065FE8)
	// 83065FE0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83065FE4: 82166A18  lwz r16, 0x6a18(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(27160 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83065FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83065FE8 size=312
    let mut pc: u32 = 0x83065FE8;
    'dispatch: loop {
        match pc {
            0x83065FE8 => {
    //   block [0x83065FE8..0x83066120)
	// 83065FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83065FEC: 48142171  bl 0x831a815c
	ctx.lr = 0x83065FF0;
	sub_831A8130(ctx, base);
	// 83065FF0: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 83065FF4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83065FF8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83065FFC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83066000: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 83066004: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 83066008: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 8306600C: 807E0038  lwz r3, 0x38(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 83066010: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83066014: 4182000C  beq 0x83066020
	if ctx.cr[0].eq {
	pc = 0x83066020; continue 'dispatch;
	}
	// 83066018: 4BFDA2F1  bl 0x83040308
	ctx.lr = 0x8306601C;
	sub_83040308(ctx, base);
	// 8306601C: 480000FC  b 0x83066118
	pc = 0x83066118; continue 'dispatch;
	// 83066020: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83066024: 409A00E8  bne cr6, 0x8306610c
	if !ctx.cr[6].eq {
	pc = 0x8306610C; continue 'dispatch;
	}
	// 83066028: 817E003C  lwz r11, 0x3c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 8306602C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83066030: 409A003C  bne cr6, 0x8306606c
	if !ctx.cr[6].eq {
	pc = 0x8306606C; continue 'dispatch;
	}
	// 83066034: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83066038: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 8306603C: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 83066040: 4BF72259  bl 0x82fd8298
	ctx.lr = 0x83066044;
	sub_82FD8298(ctx, base);
	// 83066044: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83066048: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8306604C: 41820018  beq 0x83066064
	if ctx.cr[0].eq {
	pc = 0x83066064; continue 'dispatch;
	}
	// 83066050: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83066054: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83066058: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 8306605C: 4BFD5155  bl 0x8303b1b0
	ctx.lr = 0x83066060;
	sub_8303B1B0(ctx, base);
	// 83066060: 48000008  b 0x83066068
	pc = 0x83066068; continue 'dispatch;
	// 83066064: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83066068: 907E003C  stw r3, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[3].u32 ) };
	// 8306606C: 38DF0054  addi r6, r31, 0x54
	ctx.r[6].s64 = ctx.r[31].s64 + 84;
	// 83066070: 807E003C  lwz r3, 0x3c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 83066074: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83066078: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8306607C: 48002B85  bl 0x83068c00
	ctx.lr = 0x83066080;
	sub_83068C00(ctx, base);
	// 83066080: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83066084: 41820010  beq 0x83066094
	if ctx.cr[0].eq {
	pc = 0x83066094; continue 'dispatch;
	}
	// 83066088: 83830000  lwz r28, 0(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8306608C: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83066090: 40820070  bne 0x83066100
	if !ctx.cr[0].eq {
	pc = 0x83066100; continue 'dispatch;
	}
	// 83066094: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83066098: 3860004C  li r3, 0x4c
	ctx.r[3].s64 = 76;
	// 8306609C: 909F0054  stw r4, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 830660A0: 4BF721F9  bl 0x82fd8298
	ctx.lr = 0x830660A4;
	sub_82FD8298(ctx, base);
	// 830660A4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830660A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830660AC: 41820028  beq 0x830660d4
	if ctx.cr[0].eq {
	pc = 0x830660D4; continue 'dispatch;
	}
	// 830660B0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 830660B4: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830660B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830660BC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 830660C0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 830660C4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830660C8: 4801B7A1  bl 0x83081868
	ctx.lr = 0x830660CC;
	sub_83081868(ctx, base);
	// 830660CC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 830660D0: 48000008  b 0x830660d8
	pc = 0x830660D8; continue 'dispatch;
	// 830660D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830660D8: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830660DC: 81460028  lwz r10, 0x28(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(40 as u32) ) } as u64;
	// 830660E0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830660E4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 830660E8: 91660024  stw r11, 0x24(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 830660EC: 808A0010  lwz r4, 0x10(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 830660F0: 807E003C  lwz r3, 0x3c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830660F4: 4BFD9C0D  bl 0x8303fd00
	ctx.lr = 0x830660F8;
	sub_8303FD00(ctx, base);
	// 830660F8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830660FC: 48000008  b 0x83066104
	pc = 0x83066104; continue 'dispatch;
	// 83066100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83066104: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83066108: 4800000C  b 0x83066114
	pc = 0x83066114; continue 'dispatch;
	// 8306610C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83066110: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83066114: 99790000  stb r11, 0(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83066118: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8306611C: 48142090  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83066120 size=44
    let mut pc: u32 = 0x83066120;
    'dispatch: loop {
        match pc {
            0x83066120 => {
    //   block [0x83066120..0x8306614C)
	// 83066120: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83066124: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83066128: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8306612C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83066130: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83066134: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83066138: 4BF721A9  bl 0x82fd82e0
	ctx.lr = 0x8306613C;
	sub_82FD82E0(ctx, base);
	// 8306613C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83066140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83066144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83066148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8306614C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8306614C size=44
    let mut pc: u32 = 0x8306614C;
    'dispatch: loop {
        match pc {
            0x8306614C => {
    //   block [0x8306614C..0x83066178)
	// 8306614C: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83066150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83066154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83066158: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8306615C: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83066160: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83066164: 4BF7217D  bl 0x82fd82e0
	ctx.lr = 0x83066168;
	sub_82FD82E0(ctx, base);
	// 83066168: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8306616C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83066170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83066174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83066178 size=96
    let mut pc: u32 = 0x83066178;
    'dispatch: loop {
        match pc {
            0x83066178 => {
    //   block [0x83066178..0x830661D8)
	// 83066178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306617C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83066180: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83066184: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 83066188: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8306618C: 41820038  beq 0x830661c4
	if ctx.cr[0].eq {
	pc = 0x830661C4; continue 'dispatch;
	}
	// 83066190: 806B004C  lwz r3, 0x4c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 83066194: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83066198: 4082000C  bne 0x830661a4
	if !ctx.cr[0].eq {
	pc = 0x830661A4; continue 'dispatch;
	}
	// 8306619C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830661A0: 4800001C  b 0x830661bc
	pc = 0x830661BC; continue 'dispatch;
	// 830661A4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 830661A8: 48002A59  bl 0x83068c00
	ctx.lr = 0x830661AC;
	sub_83068C00(ctx, base);
	// 830661AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830661B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830661B4: 41820008  beq 0x830661bc
	if ctx.cr[0].eq {
	pc = 0x830661BC; continue 'dispatch;
	}
	// 830661B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830661BC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830661C0: 48000008  b 0x830661c8
	pc = 0x830661C8; continue 'dispatch;
	// 830661C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830661C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830661CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830661D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830661D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830661D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830661D8 size=72
    let mut pc: u32 = 0x830661D8;
    'dispatch: loop {
        match pc {
            0x830661D8 => {
    //   block [0x830661D8..0x83066220)
	// 830661D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830661DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830661E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830661E4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830661E8: 806B0038  lwz r3, 0x38(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 830661EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830661F0: 4182000C  beq 0x830661fc
	if ctx.cr[0].eq {
	pc = 0x830661FC; continue 'dispatch;
	}
	// 830661F4: 4BFDA64D  bl 0x83040840
	ctx.lr = 0x830661F8;
	sub_83040840(ctx, base);
	// 830661F8: 48000018  b 0x83066210
	pc = 0x83066210; continue 'dispatch;
	// 830661FC: 806B003C  lwz r3, 0x3c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 83066200: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83066204: 41820008  beq 0x8306620c
	if ctx.cr[0].eq {
	pc = 0x8306620C; continue 'dispatch;
	}
	// 83066208: 4BFD9EA9  bl 0x830400b0
	ctx.lr = 0x8306620C;
	sub_830400B0(ctx, base);
	// 8306620C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83066210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83066214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83066218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8306621C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83066220 size=8
    let mut pc: u32 = 0x83066220;
    'dispatch: loop {
        match pc {
            0x83066220 => {
    //   block [0x83066220..0x83066228)
	// 83066220: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83066224: 82166A70  lwz r16, 0x6a70(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(27248 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83066228 size=168
    let mut pc: u32 = 0x83066228;
    'dispatch: loop {
        match pc {
            0x83066228 => {
    //   block [0x83066228..0x830662D0)
	// 83066228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306622C: 48141F41  bl 0x831a816c
	ctx.lr = 0x83066230;
	sub_831A8130(ctx, base);
	// 83066230: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83066234: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83066238: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8306623C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83066240: 396B6900  addi r11, r11, 0x6900
	ctx.r[11].s64 = ctx.r[11].s64 + 26880;
	// 83066244: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83066248: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8306624C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83066250: 809E0034  lwz r4, 0x34(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83066254: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83066258: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8306625C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83066260: 4E800421  bctrl
	ctx.lr = 0x83066264;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83066264: 83BE003C  lwz r29, 0x3c(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 83066268: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8306626C: 41820014  beq 0x83066280
	if ctx.cr[0].eq {
	pc = 0x83066280; continue 'dispatch;
	}
	// 83066270: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83066274: 4BFDA2B5  bl 0x83040528
	ctx.lr = 0x83066278;
	sub_83040528(ctx, base);
	// 83066278: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8306627C: 4BF72065  bl 0x82fd82e0
	ctx.lr = 0x83066280;
	sub_82FD82E0(ctx, base);
	// 83066280: 807E0048  lwz r3, 0x48(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 83066284: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83066288: 41820018  beq 0x830662a0
	if ctx.cr[0].eq {
	pc = 0x830662A0; continue 'dispatch;
	}
	// 8306628C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83066290: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83066294: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83066298: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8306629C: 4E800421  bctrl
	ctx.lr = 0x830662A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830662A0: 807E004C  lwz r3, 0x4c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 830662A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830662A8: 41820018  beq 0x830662c0
	if ctx.cr[0].eq {
	pc = 0x830662C0; continue 'dispatch;
	}
	// 830662AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830662B0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830662B4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830662B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830662BC: 4E800421  bctrl
	ctx.lr = 0x830662C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830662C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830662C4: 480180BD  bl 0x8307e380
	ctx.lr = 0x830662C8;
	sub_8307E380(ctx, base);
	// 830662C8: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830662CC: 48141EF0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830662D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830662D0 size=40
    let mut pc: u32 = 0x830662D0;
    'dispatch: loop {
        match pc {
            0x830662D0 => {
    //   block [0x830662D0..0x830662F8)
	// 830662D0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830662D4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830662D8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830662DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830662E0: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830662E4: 4801809D  bl 0x8307e380
	ctx.lr = 0x830662E8;
	sub_8307E380(ctx, base);
	// 830662E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830662EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830662F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830662F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830662F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830662F8 size=76
    let mut pc: u32 = 0x830662F8;
    'dispatch: loop {
        match pc {
            0x830662F8 => {
    //   block [0x830662F8..0x83066344)
	// 830662F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830662FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83066300: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83066304: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83066308: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8306630C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83066310: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83066314: 4BFFFF15  bl 0x83066228
	ctx.lr = 0x83066318;
	sub_83066228(ctx, base);
	// 83066318: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8306631C: 4182000C  beq 0x83066328
	if ctx.cr[0].eq {
	pc = 0x83066328; continue 'dispatch;
	}
	// 83066320: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066324: 4BF71FBD  bl 0x82fd82e0
	ctx.lr = 0x83066328;
	sub_82FD82E0(ctx, base);
	// 83066328: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8306632C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83066330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83066334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83066338: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8306633C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83066340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83066348 size=16
    let mut pc: u32 = 0x83066348;
    'dispatch: loop {
        match pc {
            0x83066348 => {
    //   block [0x83066348..0x83066358)
	// 83066348: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8306634C: 396BDA9C  addi r11, r11, -0x2564
	ctx.r[11].s64 = ctx.r[11].s64 + -9572;
	// 83066350: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83066354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83066358 size=256
    let mut pc: u32 = 0x83066358;
    'dispatch: loop {
        match pc {
            0x83066358 => {
    //   block [0x83066358..0x83066458)
	// 83066358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306635C: 48141E11  bl 0x831a816c
	ctx.lr = 0x83066360;
	sub_831A8130(ctx, base);
	// 83066360: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83066364: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83066368: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8306636C: 3BFE0090  addi r31, r30, 0x90
	ctx.r[31].s64 = ctx.r[30].s64 + 144;
	// 83066370: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83066374: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066378: 418200A0  beq 0x83066418
	if ctx.cr[0].eq {
	pc = 0x83066418; continue 'dispatch;
	}
	// 8306637C: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 83066380: 4BF6A799  bl 0x82fd0b18
	ctx.lr = 0x83066384;
	sub_82FD0B18(ctx, base);
	// 83066384: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 83066388: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8306638C: 4BF6A78D  bl 0x82fd0b18
	ctx.lr = 0x83066390;
	sub_82FD0B18(ctx, base);
	// 83066390: 3880002F  li r4, 0x2f
	ctx.r[4].s64 = 47;
	// 83066394: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066398: 4BF6A781  bl 0x82fd0b18
	ctx.lr = 0x8306639C;
	sub_82FD0B18(ctx, base);
	// 8306639C: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 830663A0: 4BF77FC9  bl 0x82fde368
	ctx.lr = 0x830663A4;
	sub_82FDE368(ctx, base);
	// 830663A4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830663A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830663AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830663B0: 4BF731C1  bl 0x82fd9570
	ctx.lr = 0x830663B4;
	sub_82FD9570(ctx, base);
	// 830663B4: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 830663B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830663BC: 4BF6A75D  bl 0x82fd0b18
	ctx.lr = 0x830663C0;
	sub_82FD0B18(ctx, base);
	// 830663C0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830663C4: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830663C8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830663CC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830663D0: 7FAB532E  sthx r29, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u16) };
	// 830663D4: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830663D8: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830663DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830663E0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830663E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830663E8: 4E800421  bctrl
	ctx.lr = 0x830663EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830663EC: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830663F0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830663F4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830663F8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830663FC: 816A0040  lwz r11, 0x40(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 83066400: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83066404: 4E800421  bctrl
	ctx.lr = 0x83066408;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83066408: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8306640C: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 83066410: B3AB0000  sth r29, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u16 ) };
	// 83066414: 4800003C  b 0x83066450
	pc = 0x83066450; continue 'dispatch;
	// 83066418: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 8306641C: 4BF6A6FD  bl 0x82fd0b18
	ctx.lr = 0x83066420;
	sub_82FD0B18(ctx, base);
	// 83066420: 3880002F  li r4, 0x2f
	ctx.r[4].s64 = 47;
	// 83066424: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066428: 4BF6A6F1  bl 0x82fd0b18
	ctx.lr = 0x8306642C;
	sub_82FD0B18(ctx, base);
	// 8306642C: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83066430: 4BF77F39  bl 0x82fde368
	ctx.lr = 0x83066434;
	sub_82FDE368(ctx, base);
	// 83066434: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83066438: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8306643C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83066440: 4BF73131  bl 0x82fd9570
	ctx.lr = 0x83066444;
	sub_82FD9570(ctx, base);
	// 83066444: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 83066448: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8306644C: 4BF6A6CD  bl 0x82fd0b18
	ctx.lr = 0x83066450;
	sub_82FD0B18(ctx, base);
	// 83066450: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83066454: 48141D68  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83066458 size=20
    let mut pc: u32 = 0x83066458;
    'dispatch: loop {
        match pc {
            0x83066458 => {
    //   block [0x83066458..0x8306646C)
	// 83066458: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 8306645C: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 83066460: 90830084  stw r4, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[4].u32 ) };
	// 83066464: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 83066468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83066470 size=24
    let mut pc: u32 = 0x83066470;
    'dispatch: loop {
        match pc {
            0x83066470 => {
    //   block [0x83066470..0x83066488)
	// 83066470: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 83066474: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 83066478: 90830088  stw r4, 0x88(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), ctx.r[4].u32 ) };
	// 8306647C: 916A0064  stw r11, 0x64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83066480: 916A0088  stw r11, 0x88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 83066484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83066488 size=344
    let mut pc: u32 = 0x83066488;
    'dispatch: loop {
        match pc {
            0x83066488 => {
    //   block [0x83066488..0x830665E0)
	// 83066488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306648C: 48141CD9  bl 0x831a8164
	ctx.lr = 0x83066490;
	sub_831A8130(ctx, base);
	// 83066490: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83066494: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83066498: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8306649C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830664A0: 897F0016  lbz r11, 0x16(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(22 as u32) ) } as u64;
	// 830664A4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830664A8: 41820130  beq 0x830665d8
	if ctx.cr[0].eq {
	pc = 0x830665D8; continue 'dispatch;
	}
	// 830664AC: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 830664B0: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 830664B4: 409A0060  bne cr6, 0x83066514
	if !ctx.cr[6].eq {
	pc = 0x83066514; continue 'dispatch;
	}
	// 830664B8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830664BC: 806B0084  lwz r3, 0x84(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 830664C0: 4BF8BC01  bl 0x82ff20c0
	ctx.lr = 0x830664C4;
	sub_82FF20C0(ctx, base);
	// 830664C4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830664C8: 40820110  bne 0x830665d8
	if !ctx.cr[0].eq {
	pc = 0x830665D8; continue 'dispatch;
	}
	// 830664CC: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830664D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830664D4: 386B007C  addi r3, r11, 0x7c
	ctx.r[3].s64 = ctx.r[11].s64 + 124;
	// 830664D8: 4BF88121  bl 0x82fee5f8
	ctx.lr = 0x830664DC;
	sub_82FEE5F8(ctx, base);
	// 830664DC: 3BDF00B8  addi r30, r31, 0xb8
	ctx.r[30].s64 = ctx.r[31].s64 + 184;
	// 830664E0: 80E1005C  lwz r7, 0x5c(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 830664E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830664E8: 80C10058  lwz r6, 0x58(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 830664EC: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830664F0: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830664F4: 48038475  bl 0x8309e968
	ctx.lr = 0x830664F8;
	sub_8309E968(ctx, base);
	// 830664F8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 830664FC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 83066500: 38AB7EB0  addi r5, r11, 0x7eb0
	ctx.r[5].s64 = ctx.r[11].s64 + 32432;
	// 83066504: 3880006C  li r4, 0x6c
	ctx.r[4].s64 = 108;
	// 83066508: 387F00AC  addi r3, r31, 0xac
	ctx.r[3].s64 = ctx.r[31].s64 + 172;
	// 8306650C: 48037AED  bl 0x8309dff8
	ctx.lr = 0x83066510;
	sub_8309DFF8(ctx, base);
	// 83066510: 480000C8  b 0x830665d8
	pc = 0x830665D8; continue 'dispatch;
	// 83066514: 54CB063E  clrlwi r11, r6, 0x18
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 83066518: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8306651C: 409A0044  bne cr6, 0x83066560
	if !ctx.cr[6].eq {
	pc = 0x83066560; continue 'dispatch;
	}
	// 83066520: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83066524: 3BFF0090  addi r31, r31, 0x90
	ctx.r[31].s64 = ctx.r[31].s64 + 144;
	// 83066528: 388B8224  addi r4, r11, -0x7ddc
	ctx.r[4].s64 = ctx.r[11].s64 + -32220;
	// 8306652C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83066530: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066534: 4BF7303D  bl 0x82fd9570
	ctx.lr = 0x83066538;
	sub_82FD9570(ctx, base);
	// 83066538: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8306653C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83066540: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066544: 4BF7302D  bl 0x82fd9570
	ctx.lr = 0x83066548;
	sub_82FD9570(ctx, base);
	// 83066548: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8306654C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83066550: 388B8238  addi r4, r11, -0x7dc8
	ctx.r[4].s64 = ctx.r[11].s64 + -32200;
	// 83066554: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066558: 4BF73019  bl 0x82fd9570
	ctx.lr = 0x8306655C;
	sub_82FD9570(ctx, base);
	// 8306655C: 4800007C  b 0x830665d8
	pc = 0x830665D8; continue 'dispatch;
	// 83066560: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83066564: 419A0074  beq cr6, 0x830665d8
	if ctx.cr[6].eq {
	pc = 0x830665D8; continue 'dispatch;
	}
	// 83066568: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8306656C: 3BFF0090  addi r31, r31, 0x90
	ctx.r[31].s64 = ctx.r[31].s64 + 144;
	// 83066570: 3B8B8170  addi r28, r11, -0x7e90
	ctx.r[28].s64 = ctx.r[11].s64 + -32400;
	// 83066574: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83066578: 3B6B8168  addi r27, r11, -0x7e98
	ctx.r[27].s64 = ctx.r[11].s64 + -32408;
	// 8306657C: A09D0000  lhz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83066580: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83066584: 2B0B0026  cmplwi cr6, r11, 0x26
	ctx.cr[6].compare_u32(ctx.r[11].u32, 38 as u32, &mut ctx.xer);
	// 83066588: 409A0014  bne cr6, 0x8306659c
	if !ctx.cr[6].eq {
	pc = 0x8306659C; continue 'dispatch;
	}
	// 8306658C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066590: 4BF6A589  bl 0x82fd0b18
	ctx.lr = 0x83066594;
	sub_82FD0B18(ctx, base);
	// 83066594: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83066598: 4800001C  b 0x830665b4
	pc = 0x830665B4; continue 'dispatch;
	// 8306659C: 2B0B003C  cmplwi cr6, r11, 0x3c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 60 as u32, &mut ctx.xer);
	// 830665A0: 409A0024  bne cr6, 0x830665c4
	if !ctx.cr[6].eq {
	pc = 0x830665C4; continue 'dispatch;
	}
	// 830665A4: 38800026  li r4, 0x26
	ctx.r[4].s64 = 38;
	// 830665A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830665AC: 4BF6A56D  bl 0x82fd0b18
	ctx.lr = 0x830665B0;
	sub_82FD0B18(ctx, base);
	// 830665B0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830665B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830665B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830665BC: 4BF72FB5  bl 0x82fd9570
	ctx.lr = 0x830665C0;
	sub_82FD9570(ctx, base);
	// 830665C0: 3880003B  li r4, 0x3b
	ctx.r[4].s64 = 59;
	// 830665C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830665C8: 4BF6A551  bl 0x82fd0b18
	ctx.lr = 0x830665CC;
	sub_82FD0B18(ctx, base);
	// 830665CC: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830665D0: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 830665D4: 4082FFA8  bne 0x8306657c
	if !ctx.cr[0].eq {
	pc = 0x8306657C; continue 'dispatch;
	}
	// 830665D8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830665DC: 48141BD8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830665E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830665E0 size=136
    let mut pc: u32 = 0x830665E0;
    'dispatch: loop {
        match pc {
            0x830665E0 => {
    //   block [0x830665E0..0x83066668)
	// 830665E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830665E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830665E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830665EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830665F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830665F4: 81630078  lwz r11, 0x78(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) } as u64;
	// 830665F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830665FC: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 83066600: 40990050  ble cr6, 0x83066650
	if !ctx.cr[6].gt {
	pc = 0x83066650; continue 'dispatch;
	}
	// 83066604: 3BE30090  addi r31, r3, 0x90
	ctx.r[31].s64 = ctx.r[3].s64 + 144;
	// 83066608: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8306660C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83066610: 388B78F0  addi r4, r11, 0x78f0
	ctx.r[4].s64 = ctx.r[11].s64 + 30960;
	// 83066614: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066618: 4BF72F59  bl 0x82fd9570
	ctx.lr = 0x8306661C;
	sub_82FD9570(ctx, base);
	// 8306661C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83066620: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83066624: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066628: 4BF72F49  bl 0x82fd9570
	ctx.lr = 0x8306662C;
	sub_82FD9570(ctx, base);
	// 8306662C: 3880002D  li r4, 0x2d
	ctx.r[4].s64 = 45;
	// 83066630: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066634: 4BF6A4E5  bl 0x82fd0b18
	ctx.lr = 0x83066638;
	sub_82FD0B18(ctx, base);
	// 83066638: 3880002D  li r4, 0x2d
	ctx.r[4].s64 = 45;
	// 8306663C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066640: 4BF6A4D9  bl 0x82fd0b18
	ctx.lr = 0x83066644;
	sub_82FD0B18(ctx, base);
	// 83066644: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 83066648: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8306664C: 4BF6A4CD  bl 0x82fd0b18
	ctx.lr = 0x83066650;
	sub_82FD0B18(ctx, base);
	// 83066650: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83066654: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83066658: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8306665C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83066660: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83066664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83066668 size=12
    let mut pc: u32 = 0x83066668;
    'dispatch: loop {
        match pc {
            0x83066668 => {
    //   block [0x83066668..0x83066674)
	// 83066668: 89630016  lbz r11, 0x16(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(22 as u32) ) } as u64;
	// 8306666C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83066670: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066674(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83066674 size=12
    let mut pc: u32 = 0x83066674;
    'dispatch: loop {
        match pc {
            0x83066674 => {
    //   block [0x83066674..0x83066680)
	// 83066674: 89630015  lbz r11, 0x15(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(21 as u32) ) } as u64;
	// 83066678: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8306667C: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83066680 size=12
    let mut pc: u32 = 0x83066680;
    'dispatch: loop {
        match pc {
            0x83066680 => {
    //   block [0x83066680..0x8306668C)
	// 83066680: 81630078  lwz r11, 0x78(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) } as u64;
	// 83066684: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 83066688: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8306668C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8306668C size=8
    let mut pc: u32 = 0x8306668C;
    'dispatch: loop {
        match pc {
            0x8306668C => {
    //   block [0x8306668C..0x83066694)
	// 8306668C: 38630090  addi r3, r3, 0x90
	ctx.r[3].s64 = ctx.r[3].s64 + 144;
	// 83066690: 4BF72EE0  b 0x82fd9570
	sub_82FD9570(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066694(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83066694 size=4
    let mut pc: u32 = 0x83066694;
    'dispatch: loop {
        match pc {
            0x83066694 => {
    //   block [0x83066694..0x83066698)
	// 83066694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83066698 size=88
    let mut pc: u32 = 0x83066698;
    'dispatch: loop {
        match pc {
            0x83066698 => {
    //   block [0x83066698..0x830666F0)
	// 83066698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306669C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830666A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830666A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830666A8: 2F060002  cmpwi cr6, r6, 2
	ctx.cr[6].compare_i32(ctx.r[6].s32, 2, &mut ctx.xer);
	// 830666AC: 4198000C  blt cr6, 0x830666b8
	if ctx.cr[6].lt {
	pc = 0x830666B8; continue 'dispatch;
	}
	// 830666B0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830666B4: 99630070  stb r11, 0x70(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), ctx.r[11].u8 ) };
	// 830666B8: 80630080  lwz r3, 0x80(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 830666BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830666C0: 4182001C  beq 0x830666dc
	if ctx.cr[0].eq {
	pc = 0x830666DC; continue 'dispatch;
	}
	// 830666C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830666C8: 83E100C4  lwz r31, 0xc4(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(196 as u32) ) } as u64;
	// 830666CC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830666D0: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 830666D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830666D8: 4E800421  bctrl
	ctx.lr = 0x830666DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830666DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830666E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830666E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830666E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830666EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830666F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830666F0 size=32
    let mut pc: u32 = 0x830666F0;
    'dispatch: loop {
        match pc {
            0x830666F0 => {
    //   block [0x830666F0..0x83066710)
	// 830666F0: 81630080  lwz r11, 0x80(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 830666F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830666F8: 419A0018  beq cr6, 0x83066710
	if ctx.cr[6].eq {
		sub_83066710(ctx, base);
		return;
	}
	// 830666FC: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83066700: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83066704: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83066708: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8306670C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83066710 size=8
    let mut pc: u32 = 0x83066710;
    'dispatch: loop {
        match pc {
            0x83066710 => {
    //   block [0x83066710..0x83066718)
	// 83066710: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83066714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83066718 size=32
    let mut pc: u32 = 0x83066718;
    'dispatch: loop {
        match pc {
            0x83066718 => {
    //   block [0x83066718..0x83066738)
	// 83066718: 81630080  lwz r11, 0x80(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 8306671C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83066720: 419A0018  beq cr6, 0x83066738
	if ctx.cr[6].eq {
		sub_83066738(ctx, base);
		return;
	}
	// 83066724: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83066728: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8306672C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83066730: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83066734: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83066738 size=8
    let mut pc: u32 = 0x83066738;
    'dispatch: loop {
        match pc {
            0x83066738 => {
    //   block [0x83066738..0x83066740)
	// 83066738: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8306673C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83066740 size=8
    let mut pc: u32 = 0x83066740;
    'dispatch: loop {
        match pc {
            0x83066740 => {
    //   block [0x83066740..0x83066748)
	// 83066740: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83066744: 82166B98  lwz r16, 0x6b98(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(27544 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83066748 size=260
    let mut pc: u32 = 0x83066748;
    'dispatch: loop {
        match pc {
            0x83066748 => {
    //   block [0x83066748..0x8306684C)
	// 83066748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306674C: 48141A19  bl 0x831a8164
	ctx.lr = 0x83066750;
	sub_831A8130(ctx, base);
	// 83066750: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83066754: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83066758: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8306675C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83066760: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 83066764: 93BF00B4  stw r29, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[29].u32 ) };
	// 83066768: 4BF71261  bl 0x82fd79c8
	ctx.lr = 0x8306676C;
	sub_82FD79C8(ctx, base);
	// 8306676C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83066770: 3D408216  lis r10, -0x7dea
	ctx.r[10].s64 = -2112487424;
	// 83066774: 396B6B30  addi r11, r11, 0x6b30
	ctx.r[11].s64 = ctx.r[11].s64 + 27440;
	// 83066778: 3D208216  lis r9, -0x7dea
	ctx.r[9].s64 = -2112487424;
	// 8306677C: 3D008216  lis r8, -0x7dea
	ctx.r[8].s64 = -2112487424;
	// 83066780: 3CE08216  lis r7, -0x7dea
	ctx.r[7].s64 = -2112487424;
	// 83066784: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83066788: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8306678C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83066790: 394A6B20  addi r10, r10, 0x6b20
	ctx.r[10].s64 = ctx.r[10].s64 + 27424;
	// 83066794: 39296B04  addi r9, r9, 0x6b04
	ctx.r[9].s64 = ctx.r[9].s64 + 27396;
	// 83066798: 39086AC0  addi r8, r8, 0x6ac0
	ctx.r[8].s64 = ctx.r[8].s64 + 27328;
	// 8306679C: 38E76AB0  addi r7, r7, 0x6ab0
	ctx.r[7].s64 = ctx.r[7].s64 + 27312;
	// 830667A0: 9B9E0074  stb r28, 0x74(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(116 as u32), ctx.r[28].u8 ) };
	// 830667A4: 3B7E0004  addi r27, r30, 4
	ctx.r[27].s64 = ctx.r[30].s64 + 4;
	// 830667A8: 917E0078  stw r11, 0x78(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 830667AC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830667B0: 917E007C  stw r11, 0x7c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 830667B4: 388003FF  li r4, 0x3ff
	ctx.r[4].s64 = 1023;
	// 830667B8: 913E0008  stw r9, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 830667BC: 387E0090  addi r3, r30, 0x90
	ctx.r[3].s64 = ctx.r[30].s64 + 144;
	// 830667C0: 911E000C  stw r8, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 830667C4: 90FE0010  stw r7, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 830667C8: 917E0080  stw r11, 0x80(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 830667CC: 939E0084  stw r28, 0x84(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), ctx.r[28].u32 ) };
	// 830667D0: 939E0088  stw r28, 0x88(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(136 as u32), ctx.r[28].u32 ) };
	// 830667D4: 915B0000  stw r10, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830667D8: 939E008C  stw r28, 0x8c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(140 as u32), ctx.r[28].u32 ) };
	// 830667DC: 4BF7867D  bl 0x82fdee58
	ctx.lr = 0x830667E0;
	sub_82FDEE58(ctx, base);
	// 830667E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830667E4: 387E00AC  addi r3, r30, 0xac
	ctx.r[3].s64 = ctx.r[30].s64 + 172;
	// 830667E8: 480374F9  bl 0x8309dce0
	ctx.lr = 0x830667EC;
	sub_8309DCE0(ctx, base);
	// 830667EC: 387E00B8  addi r3, r30, 0xb8
	ctx.r[3].s64 = ctx.r[30].s64 + 184;
	// 830667F0: 48038109  bl 0x8309e8f8
	ctx.lr = 0x830667F4;
	sub_8309E8F8(ctx, base);
	// 830667F4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830667F8: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 830667FC: 4BF71A9D  bl 0x82fd8298
	ctx.lr = 0x83066800;
	sub_82FD8298(ctx, base);
	// 83066800: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83066804: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83066808: 41820018  beq 0x83066820
	if ctx.cr[0].eq {
	pc = 0x83066820; continue 'dispatch;
	}
	// 8306680C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83066810: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83066814: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 83066818: 48001F89  bl 0x830687a0
	ctx.lr = 0x8306681C;
	sub_830687A0(ctx, base);
	// 8306681C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83066820: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83066824: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83066828: 939E008C  stw r28, 0x8c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(140 as u32), ctx.r[28].u32 ) };
	// 8306682C: 937E00B4  stw r27, 0xb4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(180 as u32), ctx.r[27].u32 ) };
	// 83066830: 4BF6DCB1  bl 0x82fd44e0
	ctx.lr = 0x83066834;
	sub_82FD44E0(ctx, base);
	// 83066834: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83066838: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8306683C: 4BF6DC75  bl 0x82fd44b0
	ctx.lr = 0x83066840;
	sub_82FD44B0(ctx, base);
	// 83066840: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83066844: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83066848: 4814196C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8306684C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8306684C size=40
    let mut pc: u32 = 0x8306684C;
    'dispatch: loop {
        match pc {
            0x8306684C => {
    //   block [0x8306684C..0x83066874)
	// 8306684C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83066850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83066854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83066858: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8306685C: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83066860: 4BF711E9  bl 0x82fd7a48
	ctx.lr = 0x83066864;
	sub_82FD7A48(ctx, base);
	// 83066864: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83066868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8306686C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83066870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066874(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83066874 size=44
    let mut pc: u32 = 0x83066874;
    'dispatch: loop {
        match pc {
            0x83066874 => {
    //   block [0x83066874..0x830668A0)
	// 83066874: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83066878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306687C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83066880: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83066884: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83066888: 386B0090  addi r3, r11, 0x90
	ctx.r[3].s64 = ctx.r[11].s64 + 144;
	// 8306688C: 4BF7864D  bl 0x82fdeed8
	ctx.lr = 0x83066890;
	sub_82FDEED8(ctx, base);
	// 83066890: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83066894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83066898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8306689C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830668A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830668A0 size=44
    let mut pc: u32 = 0x830668A0;
    'dispatch: loop {
        match pc {
            0x830668A0 => {
    //   block [0x830668A0..0x830668CC)
	// 830668A0: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830668A4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830668A8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830668AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830668B0: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830668B4: 386B00AC  addi r3, r11, 0xac
	ctx.r[3].s64 = ctx.r[11].s64 + 172;
	// 830668B8: 4BFEDFD1  bl 0x83054888
	ctx.lr = 0x830668BC;
	sub_83054888(ctx, base);
	// 830668BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830668C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830668C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830668C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830668CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830668CC size=44
    let mut pc: u32 = 0x830668CC;
    'dispatch: loop {
        match pc {
            0x830668CC => {
    //   block [0x830668CC..0x830668F8)
	// 830668CC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830668D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830668D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830668D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830668DC: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830668E0: 386B00B8  addi r3, r11, 0xb8
	ctx.r[3].s64 = ctx.r[11].s64 + 184;
	// 830668E4: 4BFFFA65  bl 0x83066348
	ctx.lr = 0x830668E8;
	sub_83066348(ctx, base);
	// 830668E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830668EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830668F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830668F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830668F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830668F8 size=44
    let mut pc: u32 = 0x830668F8;
    'dispatch: loop {
        match pc {
            0x830668F8 => {
    //   block [0x830668F8..0x83066924)
	// 830668F8: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830668FC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83066900: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83066904: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83066908: 809F00B4  lwz r4, 0xb4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8306690C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83066910: 4BF719D1  bl 0x82fd82e0
	ctx.lr = 0x83066914;
	sub_82FD82E0(ctx, base);
	// 83066914: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83066918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8306691C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83066920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83066928 size=8
    let mut pc: u32 = 0x83066928;
    'dispatch: loop {
        match pc {
            0x83066928 => {
    //   block [0x83066928..0x83066930)
	// 83066928: 3863FFFC  addi r3, r3, -4
	ctx.r[3].s64 = ctx.r[3].s64 + -4;
	// 8306692C: 4800062C  b 0x83066f58
	sub_83066F58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83066930 size=8
    let mut pc: u32 = 0x83066930;
    'dispatch: loop {
        match pc {
            0x83066930 => {
    //   block [0x83066930..0x83066938)
	// 83066930: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 83066934: 48000624  b 0x83066f58
	sub_83066F58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83066938 size=8
    let mut pc: u32 = 0x83066938;
    'dispatch: loop {
        match pc {
            0x83066938 => {
    //   block [0x83066938..0x83066940)
	// 83066938: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 8306693C: 4800061C  b 0x83066f58
	sub_83066F58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83066940 size=8
    let mut pc: u32 = 0x83066940;
    'dispatch: loop {
        match pc {
            0x83066940 => {
    //   block [0x83066940..0x83066948)
	// 83066940: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 83066944: 48000614  b 0x83066f58
	sub_83066F58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83066948 size=88
    let mut pc: u32 = 0x83066948;
    'dispatch: loop {
        match pc {
            0x83066948 => {
    //   block [0x83066948..0x830669A0)
	// 83066948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306694C: 48141821  bl 0x831a816c
	ctx.lr = 0x83066950;
	sub_831A8130(ctx, base);
	// 83066950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83066954: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83066958: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8306695C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83066960: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83066964: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83066968: 386B007C  addi r3, r11, 0x7c
	ctx.r[3].s64 = ctx.r[11].s64 + 124;
	// 8306696C: 4BF87C8D  bl 0x82fee5f8
	ctx.lr = 0x83066970;
	sub_82FEE5F8(ctx, base);
	// 83066970: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066974: 4BF6DA25  bl 0x82fd4398
	ctx.lr = 0x83066978;
	sub_82FD4398(ctx, base);
	// 83066978: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8306697C: 80E1005C  lwz r7, 0x5c(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83066980: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83066984: 80C10058  lwz r6, 0x58(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83066988: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8306698C: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 83066990: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83066994: 4E800421  bctrl
	ctx.lr = 0x83066998;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83066998: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8306699C: 48141820  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830669A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830669A0 size=212
    let mut pc: u32 = 0x830669A0;
    'dispatch: loop {
        match pc {
            0x830669A0 => {
    //   block [0x830669A0..0x83066A74)
	// 830669A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830669A4: 481417C1  bl 0x831a8164
	ctx.lr = 0x830669A8;
	sub_831A8130(ctx, base);
	// 830669A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830669AC: 3BE30090  addi r31, r3, 0x90
	ctx.r[31].s64 = ctx.r[3].s64 + 144;
	// 830669B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830669B4: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 830669B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830669BC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830669C0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 830669C4: 4BF6A155  bl 0x82fd0b18
	ctx.lr = 0x830669C8;
	sub_82FD0B18(ctx, base);
	// 830669C8: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830669CC: 4BF7799D  bl 0x82fde368
	ctx.lr = 0x830669D0;
	sub_82FDE368(ctx, base);
	// 830669D0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830669D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830669D8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830669DC: 4BF72B95  bl 0x82fd9570
	ctx.lr = 0x830669E0;
	sub_82FD9570(ctx, base);
	// 830669E0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830669E4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830669E8: 419A0078  beq cr6, 0x83066a60
	if ctx.cr[6].eq {
	pc = 0x83066A60; continue 'dispatch;
	}
	// 830669EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830669F0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830669F4: 4BF86FE5  bl 0x82fed9d8
	ctx.lr = 0x830669F8;
	sub_82FED9D8(ctx, base);
	// 830669F8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830669FC: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 83066A00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066A04: 4BF6A115  bl 0x82fd0b18
	ctx.lr = 0x83066A08;
	sub_82FD0B18(ctx, base);
	// 83066A08: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83066A0C: 4BF8B25D  bl 0x82ff1c68
	ctx.lr = 0x83066A10;
	sub_82FF1C68(ctx, base);
	// 83066A10: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83066A14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066A18: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83066A1C: 4BF72B55  bl 0x82fd9570
	ctx.lr = 0x83066A20;
	sub_82FD9570(ctx, base);
	// 83066A20: 3880003D  li r4, 0x3d
	ctx.r[4].s64 = 61;
	// 83066A24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066A28: 4BF6A0F1  bl 0x82fd0b18
	ctx.lr = 0x83066A2C;
	sub_82FD0B18(ctx, base);
	// 83066A2C: 38800022  li r4, 0x22
	ctx.r[4].s64 = 34;
	// 83066A30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066A34: 4BF6A0E5  bl 0x82fd0b18
	ctx.lr = 0x83066A38;
	sub_82FD0B18(ctx, base);
	// 83066A38: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83066A3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066A40: 809C000C  lwz r4, 0xc(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 83066A44: 4BF72B2D  bl 0x82fd9570
	ctx.lr = 0x83066A48;
	sub_82FD9570(ctx, base);
	// 83066A48: 38800022  li r4, 0x22
	ctx.r[4].s64 = 34;
	// 83066A4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066A50: 4BF6A0C9  bl 0x82fd0b18
	ctx.lr = 0x83066A54;
	sub_82FD0B18(ctx, base);
	// 83066A54: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83066A58: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83066A5C: 4198FF90  blt cr6, 0x830669ec
	if ctx.cr[6].lt {
	pc = 0x830669EC; continue 'dispatch;
	}
	// 83066A60: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 83066A64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066A68: 4BF6A0B1  bl 0x82fd0b18
	ctx.lr = 0x83066A6C;
	sub_82FD0B18(ctx, base);
	// 83066A6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83066A70: 48141744  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83066A78 size=8
    let mut pc: u32 = 0x83066A78;
    'dispatch: loop {
        match pc {
            0x83066A78 => {
    //   block [0x83066A78..0x83066A80)
	// 83066A78: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83066A7C: 82166C18  lwz r16, 0x6c18(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(27672 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83066A80 size=192
    let mut pc: u32 = 0x83066A80;
    'dispatch: loop {
        match pc {
            0x83066A80 => {
    //   block [0x83066A80..0x83066B40)
	// 83066A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83066A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83066A88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83066A8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83066A90: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83066A94: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83066A98: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83066A9C: 3D408216  lis r10, -0x7dea
	ctx.r[10].s64 = -2112487424;
	// 83066AA0: 3D208216  lis r9, -0x7dea
	ctx.r[9].s64 = -2112487424;
	// 83066AA4: 3D008216  lis r8, -0x7dea
	ctx.r[8].s64 = -2112487424;
	// 83066AA8: 3CE08216  lis r7, -0x7dea
	ctx.r[7].s64 = -2112487424;
	// 83066AAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83066AB0: 396B6B30  addi r11, r11, 0x6b30
	ctx.r[11].s64 = ctx.r[11].s64 + 27440;
	// 83066AB4: 394A6B20  addi r10, r10, 0x6b20
	ctx.r[10].s64 = ctx.r[10].s64 + 27424;
	// 83066AB8: 39296B04  addi r9, r9, 0x6b04
	ctx.r[9].s64 = ctx.r[9].s64 + 27396;
	// 83066ABC: 39086AC0  addi r8, r8, 0x6ac0
	ctx.r[8].s64 = ctx.r[8].s64 + 27328;
	// 83066AC0: 38E76AB0  addi r7, r7, 0x6ab0
	ctx.r[7].s64 = ctx.r[7].s64 + 27312;
	// 83066AC4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83066AC8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83066ACC: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83066AD0: 913E0008  stw r9, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 83066AD4: 911E000C  stw r8, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 83066AD8: 90FE0010  stw r7, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 83066ADC: 807E008C  lwz r3, 0x8c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 83066AE0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83066AE4: 4182000C  beq 0x83066af0
	if ctx.cr[0].eq {
	pc = 0x83066AF0; continue 'dispatch;
	}
	// 83066AE8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83066AEC: 48033C2D  bl 0x8309a718
	ctx.lr = 0x83066AF0;
	sub_8309A718(ctx, base);
	// 83066AF0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83066AF4: 396BDA9C  addi r11, r11, -0x2564
	ctx.r[11].s64 = ctx.r[11].s64 + -9572;
	// 83066AF8: 917E00B8  stw r11, 0xb8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(184 as u32), ctx.r[11].u32 ) };
	// 83066AFC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83066B00: 396B43A8  addi r11, r11, 0x43a8
	ctx.r[11].s64 = ctx.r[11].s64 + 17320;
	// 83066B04: 917E00AC  stw r11, 0xac(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 83066B08: 807E009C  lwz r3, 0x9c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(156 as u32) ) } as u64;
	// 83066B0C: 809E00A8  lwz r4, 0xa8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) } as u64;
	// 83066B10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83066B14: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83066B18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83066B1C: 4E800421  bctrl
	ctx.lr = 0x83066B20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83066B20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83066B24: 4BF70F25  bl 0x82fd7a48
	ctx.lr = 0x83066B28;
	sub_82FD7A48(ctx, base);
	// 83066B28: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83066B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83066B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83066B34: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83066B38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83066B3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83066B40 size=40
    let mut pc: u32 = 0x83066B40;
    'dispatch: loop {
        match pc {
            0x83066B40 => {
    //   block [0x83066B40..0x83066B68)
	// 83066B40: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83066B44: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83066B48: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83066B4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83066B50: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83066B54: 4BF70EF5  bl 0x82fd7a48
	ctx.lr = 0x83066B58;
	sub_82FD7A48(ctx, base);
	// 83066B58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83066B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83066B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83066B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83066B68 size=44
    let mut pc: u32 = 0x83066B68;
    'dispatch: loop {
        match pc {
            0x83066B68 => {
    //   block [0x83066B68..0x83066B94)
	// 83066B68: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83066B6C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83066B70: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83066B74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83066B78: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83066B7C: 386B0090  addi r3, r11, 0x90
	ctx.r[3].s64 = ctx.r[11].s64 + 144;
	// 83066B80: 4BF78359  bl 0x82fdeed8
	ctx.lr = 0x83066B84;
	sub_82FDEED8(ctx, base);
	// 83066B84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83066B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83066B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83066B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066B94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83066B94 size=44
    let mut pc: u32 = 0x83066B94;
    'dispatch: loop {
        match pc {
            0x83066B94 => {
    //   block [0x83066B94..0x83066BC0)
	// 83066B94: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83066B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83066B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83066BA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83066BA4: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83066BA8: 386B00AC  addi r3, r11, 0xac
	ctx.r[3].s64 = ctx.r[11].s64 + 172;
	// 83066BAC: 4BFEDCDD  bl 0x83054888
	ctx.lr = 0x83066BB0;
	sub_83054888(ctx, base);
	// 83066BB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83066BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83066BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83066BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83066BC0 size=44
    let mut pc: u32 = 0x83066BC0;
    'dispatch: loop {
        match pc {
            0x83066BC0 => {
    //   block [0x83066BC0..0x83066BEC)
	// 83066BC0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83066BC4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83066BC8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83066BCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83066BD0: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83066BD4: 386B00B8  addi r3, r11, 0xb8
	ctx.r[3].s64 = ctx.r[11].s64 + 184;
	// 83066BD8: 4BFFF771  bl 0x83066348
	ctx.lr = 0x83066BDC;
	sub_83066348(ctx, base);
	// 83066BDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83066BE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83066BE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83066BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83066BF0 size=680
    let mut pc: u32 = 0x83066BF0;
    'dispatch: loop {
        match pc {
            0x83066BF0 => {
    //   block [0x83066BF0..0x83066E98)
	// 83066BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83066BF4: 4814155D  bl 0x831a8150
	ctx.lr = 0x83066BF8;
	sub_831A8130(ctx, base);
	// 83066BF8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83066BFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83066C00: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83066C04: 3BFE0090  addi r31, r30, 0x90
	ctx.r[31].s64 = ctx.r[30].s64 + 144;
	// 83066C08: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 83066C0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066C10: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 83066C14: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 83066C18: 4BF69F01  bl 0x82fd0b18
	ctx.lr = 0x83066C1C;
	sub_82FD0B18(ctx, base);
	// 83066C1C: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83066C20: 4BF77749  bl 0x82fde368
	ctx.lr = 0x83066C24;
	sub_82FDE368(ctx, base);
	// 83066C24: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83066C28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066C2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83066C30: 4BF72941  bl 0x82fd9570
	ctx.lr = 0x83066C34;
	sub_82FD9570(ctx, base);
	// 83066C34: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 83066C38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066C3C: 4BF69EDD  bl 0x82fd0b18
	ctx.lr = 0x83066C40;
	sub_82FD0B18(ctx, base);
	// 83066C40: 817E008C  lwz r11, 0x8c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 83066C44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83066C48: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83066C4C: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 83066C50: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83066C54: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83066C58: 3ACB8024  addi r22, r11, -0x7fdc
	ctx.r[22].s64 = ctx.r[11].s64 + -32732;
	// 83066C5C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83066C60: 3B0B8158  addi r24, r11, -0x7ea8
	ctx.r[24].s64 = ctx.r[11].s64 + -32424;
	// 83066C64: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83066C68: 3AEB8018  addi r23, r11, -0x7fe8
	ctx.r[23].s64 = ctx.r[11].s64 + -32744;
	// 83066C6C: 419A00EC  beq cr6, 0x83066d58
	if ctx.cr[6].eq {
	pc = 0x83066D58; continue 'dispatch;
	}
	// 83066C70: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83066C74: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83066C78: 4BF86D61  bl 0x82fed9d8
	ctx.lr = 0x83066C7C;
	sub_82FED9D8(ctx, base);
	// 83066C7C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83066C80: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 83066C84: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 83066C88: 833D000C  lwz r25, 0xc(r29)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 83066C8C: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83066C90: 4BF6CFB1  bl 0x82fd3c40
	ctx.lr = 0x83066C94;
	sub_82FD3C40(ctx, base);
	// 83066C94: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83066C98: 4182000C  beq 0x83066ca4
	if ctx.cr[0].eq {
	pc = 0x83066CA4; continue 'dispatch;
	}
	// 83066C9C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83066CA0: 48000028  b 0x83066cc8
	pc = 0x83066CC8; continue 'dispatch;
	// 83066CA4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83066CA8: 4BF8AFC1  bl 0x82ff1c68
	ctx.lr = 0x83066CAC;
	sub_82FF1C68(ctx, base);
	// 83066CAC: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 83066CB0: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 83066CB4: 4BF6AC8D  bl 0x82fd1940
	ctx.lr = 0x83066CB8;
	sub_82FD1940(ctx, base);
	// 83066CB8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83066CBC: 40820038  bne 0x83066cf4
	if !ctx.cr[0].eq {
	pc = 0x83066CF4; continue 'dispatch;
	}
	// 83066CC0: 815D0010  lwz r10, 0x10(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 83066CC4: 808A0010  lwz r4, 0x10(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83066CC8: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83066CCC: 396B0190  addi r11, r11, 0x190
	ctx.r[11].s64 = ctx.r[11].s64 + 400;
	// 83066CD0: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 83066CD4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83066CD8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83066CDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83066CE0: 4E800421  bctrl
	ctx.lr = 0x83066CE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83066CE4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83066CE8: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83066CEC: 807E008C  lwz r3, 0x8c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 83066CF0: 4BF7FF19  bl 0x82fe6c08
	ctx.lr = 0x83066CF4;
	sub_82FE6C08(ctx, base);
	// 83066CF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83066CF8: 4BF8AF71  bl 0x82ff1c68
	ctx.lr = 0x83066CFC;
	sub_82FF1C68(ctx, base);
	// 83066CFC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83066D00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83066D08: 4BF72869  bl 0x82fd9570
	ctx.lr = 0x83066D0C;
	sub_82FD9570(ctx, base);
	// 83066D0C: 3880003D  li r4, 0x3d
	ctx.r[4].s64 = 61;
	// 83066D10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066D14: 4BF69E05  bl 0x82fd0b18
	ctx.lr = 0x83066D18;
	sub_82FD0B18(ctx, base);
	// 83066D18: 38800022  li r4, 0x22
	ctx.r[4].s64 = 34;
	// 83066D1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066D20: 4BF69DF9  bl 0x82fd0b18
	ctx.lr = 0x83066D24;
	sub_82FD0B18(ctx, base);
	// 83066D24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83066D28: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83066D2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066D30: 4BF72841  bl 0x82fd9570
	ctx.lr = 0x83066D34;
	sub_82FD9570(ctx, base);
	// 83066D34: 38800022  li r4, 0x22
	ctx.r[4].s64 = 34;
	// 83066D38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066D3C: 4BF69DDD  bl 0x82fd0b18
	ctx.lr = 0x83066D40;
	sub_82FD0B18(ctx, base);
	// 83066D40: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 83066D44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066D48: 4BF69DD1  bl 0x82fd0b18
	ctx.lr = 0x83066D4C;
	sub_82FD0B18(ctx, base);
	// 83066D4C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 83066D50: 7F1CD840  cmplw cr6, r28, r27
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[27].u32, &mut ctx.xer);
	// 83066D54: 4198FF1C  blt cr6, 0x83066c70
	if ctx.cr[6].lt {
	pc = 0x83066C70; continue 'dispatch;
	}
	// 83066D58: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83066D5C: 386B0190  addi r3, r11, 0x190
	ctx.r[3].s64 = ctx.r[11].s64 + 400;
	// 83066D60: 4BFB0341  bl 0x830170a0
	ctx.lr = 0x83066D64;
	sub_830170A0(ctx, base);
	// 83066D64: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83066D68: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83066D6C: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83066D70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83066D74: 40990104  ble cr6, 0x83066e78
	if !ctx.cr[6].gt {
	pc = 0x83066E78; continue 'dispatch;
	}
	// 83066D78: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83066D7C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83066D80: 4BF91351  bl 0x82ff80d0
	ctx.lr = 0x83066D84;
	sub_82FF80D0(ctx, base);
	// 83066D84: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83066D88: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83066D8C: 807E008C  lwz r3, 0x8c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 83066D90: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83066D94: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83066D98: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83066D9C: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 83066DA0: 480019B1  bl 0x83068750
	ctx.lr = 0x83066DA4;
	sub_83068750(ctx, base);
	// 83066DA4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83066DA8: 408200C0  bne 0x83066e68
	if !ctx.cr[0].eq {
	pc = 0x83066E68; continue 'dispatch;
	}
	// 83066DAC: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83066DB0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83066DB4: 396B0190  addi r11, r11, 0x190
	ctx.r[11].s64 = ctx.r[11].s64 + 400;
	// 83066DB8: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 83066DBC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83066DC0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83066DC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83066DC8: 4E800421  bctrl
	ctx.lr = 0x83066DCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83066DCC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83066DD0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83066DD4: 4BF6CE6D  bl 0x82fd3c40
	ctx.lr = 0x83066DD8;
	sub_82FD3C40(ctx, base);
	// 83066DD8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83066DDC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83066DE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066DE4: 4182000C  beq 0x83066df0
	if ctx.cr[0].eq {
	pc = 0x83066DF0; continue 'dispatch;
	}
	// 83066DE8: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 83066DEC: 48000018  b 0x83066e04
	pc = 0x83066E04; continue 'dispatch;
	// 83066DF0: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 83066DF4: 4BF7277D  bl 0x82fd9570
	ctx.lr = 0x83066DF8;
	sub_82FD9570(ctx, base);
	// 83066DF8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83066DFC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83066E00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066E04: 4BF7276D  bl 0x82fd9570
	ctx.lr = 0x83066E08;
	sub_82FD9570(ctx, base);
	// 83066E08: 3880003D  li r4, 0x3d
	ctx.r[4].s64 = 61;
	// 83066E0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066E10: 4BF69D09  bl 0x82fd0b18
	ctx.lr = 0x83066E14;
	sub_82FD0B18(ctx, base);
	// 83066E14: 38800022  li r4, 0x22
	ctx.r[4].s64 = 34;
	// 83066E18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066E1C: 4BF69CFD  bl 0x82fd0b18
	ctx.lr = 0x83066E20;
	sub_82FD0B18(ctx, base);
	// 83066E20: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83066E24: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83066E28: 4BF912A9  bl 0x82ff80d0
	ctx.lr = 0x83066E2C;
	sub_82FF80D0(ctx, base);
	// 83066E2C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83066E30: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83066E34: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83066E38: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83066E3C: 4BF8121D  bl 0x82fe8058
	ctx.lr = 0x83066E40;
	sub_82FE8058(ctx, base);
	// 83066E40: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83066E44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066E48: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83066E4C: 4BF72725  bl 0x82fd9570
	ctx.lr = 0x83066E50;
	sub_82FD9570(ctx, base);
	// 83066E50: 38800022  li r4, 0x22
	ctx.r[4].s64 = 34;
	// 83066E54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066E58: 4BF69CC1  bl 0x82fd0b18
	ctx.lr = 0x83066E5C;
	sub_82FD0B18(ctx, base);
	// 83066E5C: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 83066E60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066E64: 4BF69CB5  bl 0x82fd0b18
	ctx.lr = 0x83066E68;
	sub_82FD0B18(ctx, base);
	// 83066E68: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83066E6C: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 83066E70: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83066E74: 4198FF04  blt cr6, 0x83066d78
	if ctx.cr[6].lt {
	pc = 0x83066D78; continue 'dispatch;
	}
	// 83066E78: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 83066E7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066E80: 4BF69C99  bl 0x82fd0b18
	ctx.lr = 0x83066E84;
	sub_82FD0B18(ctx, base);
	// 83066E84: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 83066E88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066E8C: 4BF69C8D  bl 0x82fd0b18
	ctx.lr = 0x83066E90;
	sub_82FD0B18(ctx, base);
	// 83066E90: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 83066E94: 4814130C  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83066E98 size=192
    let mut pc: u32 = 0x83066E98;
    'dispatch: loop {
        match pc {
            0x83066E98 => {
    //   block [0x83066E98..0x83066F58)
	// 83066E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83066E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83066EA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83066EA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83066EA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83066EAC: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83066EB0: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 83066EB4: 4099003C  ble cr6, 0x83066ef0
	if !ctx.cr[6].gt {
	pc = 0x83066EF0; continue 'dispatch;
	}
	// 83066EB8: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83066EBC: 813F007C  lwz r9, 0x7c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83066EC0: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83066EC4: 409A0014  bne cr6, 0x83066ed8
	if !ctx.cr[6].eq {
	pc = 0x83066ED8; continue 'dispatch;
	}
	// 83066EC8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83066ECC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83066ED0: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 83066ED4: 48000018  b 0x83066eec
	pc = 0x83066EEC; continue 'dispatch;
	// 83066ED8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83066EDC: 409A0064  bne cr6, 0x83066f40
	if !ctx.cr[6].eq {
	pc = 0x83066F40; continue 'dispatch;
	}
	// 83066EE0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83066EE4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83066EE8: 917F0078  stw r11, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 83066EEC: 4BFFF46D  bl 0x83066358
	ctx.lr = 0x83066EF0;
	sub_83066358(ctx, base);
	// 83066EF0: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83066EF4: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83066EF8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83066EFC: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83066F00: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 83066F04: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 83066F08: 4BFEE609  bl 0x83055510
	ctx.lr = 0x83066F0C;
	sub_83055510(ctx, base);
	// 83066F0C: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83066F10: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 83066F14: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83066F18: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83066F1C: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83066F20: 4182000C  beq 0x83066f2c
	if ctx.cr[0].eq {
	pc = 0x83066F2C; continue 'dispatch;
	}
	// 83066F24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83066F28: 997F0016  stb r11, 0x16(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(22 as u32), ctx.r[11].u8 ) };
	// 83066F2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83066F30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83066F34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83066F38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83066F3C: 4E800020  blr
	return;
	// 83066F40: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83066F44: 4BFFF415  bl 0x83066358
	ctx.lr = 0x83066F48;
	sub_83066358(ctx, base);
	// 83066F48: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83066F4C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83066F50: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 83066F54: 4BFFFFD8  b 0x83066f2c
	pc = 0x83066F2C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83066F58 size=76
    let mut pc: u32 = 0x83066F58;
    'dispatch: loop {
        match pc {
            0x83066F58 => {
    //   block [0x83066F58..0x83066FA4)
	// 83066F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83066F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83066F60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83066F64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83066F68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83066F6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83066F70: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83066F74: 4BFFFB0D  bl 0x83066a80
	ctx.lr = 0x83066F78;
	sub_83066A80(ctx, base);
	// 83066F78: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83066F7C: 4182000C  beq 0x83066f88
	if ctx.cr[0].eq {
	pc = 0x83066F88; continue 'dispatch;
	}
	// 83066F80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066F84: 4BF7135D  bl 0x82fd82e0
	ctx.lr = 0x83066F88;
	sub_82FD82E0(ctx, base);
	// 83066F88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83066F8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83066F90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83066F94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83066F98: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83066F9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83066FA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83066FA8 size=8
    let mut pc: u32 = 0x83066FA8;
    'dispatch: loop {
        match pc {
            0x83066FA8 => {
    //   block [0x83066FA8..0x83066FB0)
	// 83066FA8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83066FAC: 82166C70  lwz r16, 0x6c70(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(27760 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83066FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83066FB0 size=1408
    let mut pc: u32 = 0x83066FB0;
    'dispatch: loop {
        match pc {
            0x83066FB0 => {
    //   block [0x83066FB0..0x83067530)
	// 83066FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83066FB4: 48141181  bl 0x831a8134
	ctx.lr = 0x83066FB8;
	sub_831A8130(ctx, base);
	// 83066FB8: 3BE1FF10  addi r31, r1, -0xf0
	ctx.r[31].s64 = ctx.r[1].s64 + -240;
	// 83066FBC: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83066FC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83066FC4: 7D4F5378  mr r15, r10
	ctx.r[15].u64 = ctx.r[10].u64;
	// 83066FC8: 7C942378  mr r20, r4
	ctx.r[20].u64 = ctx.r[4].u64;
	// 83066FCC: 7CB22B78  mr r18, r5
	ctx.r[18].u64 = ctx.r[5].u64;
	// 83066FD0: 7CD13378  mr r17, r6
	ctx.r[17].u64 = ctx.r[6].u64;
	// 83066FD4: 817E0080  lwz r11, 0x80(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 83066FD8: 7CF63B78  mr r22, r7
	ctx.r[22].u64 = ctx.r[7].u64;
	// 83066FDC: 815E0078  lwz r10, 0x78(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(120 as u32) ) } as u64;
	// 83066FE0: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 83066FE4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83066FE8: 93DF0104  stw r30, 0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[30].u32 ) };
	// 83066FEC: 7D304B78  mr r16, r9
	ctx.r[16].u64 = ctx.r[9].u64;
	// 83066FF0: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 83066FF4: 917E0080  stw r11, 0x80(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 83066FF8: 409A0064  bne cr6, 0x8306705c
	if !ctx.cr[6].eq {
	pc = 0x8306705C; continue 'dispatch;
	}
	// 83066FFC: 81540008  lwz r10, 8(r20)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(8 as u32) ) } as u64;
	// 83067000: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83067004: 388BCD10  addi r4, r11, -0x32f0
	ctx.r[4].s64 = ctx.r[11].s64 + -13040;
	// 83067008: 806A0010  lwz r3, 0x10(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 8306700C: 4BF6CC35  bl 0x82fd3c40
	ctx.lr = 0x83067010;
	sub_82FD3C40(ctx, base);
	// 83067010: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83067014: 4182006C  beq 0x83067080
	if ctx.cr[0].eq {
	pc = 0x83067080; continue 'dispatch;
	}
	// 83067018: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8306701C: 7E449378  mr r4, r18
	ctx.r[4].u64 = ctx.r[18].u64;
	// 83067020: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83067024: 3BABCC98  addi r29, r11, -0x3368
	ctx.r[29].s64 = ctx.r[11].s64 + -13160;
	// 83067028: 4BF70A61  bl 0x82fd7a88
	ctx.lr = 0x8306702C;
	sub_82FD7A88(ctx, base);
	// 8306702C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83067030: 4BF6CC11  bl 0x82fd3c40
	ctx.lr = 0x83067034;
	sub_82FD3C40(ctx, base);
	// 83067034: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83067038: 41820048  beq 0x83067080
	if ctx.cr[0].eq {
	pc = 0x83067080; continue 'dispatch;
	}
	// 8306703C: 817E0080  lwz r11, 0x80(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 83067040: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 83067044: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 83067048: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 8306704C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83067050: 917E0078  stw r11, 0x78(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 83067054: 4BFFFB9D  bl 0x83066bf0
	ctx.lr = 0x83067058;
	sub_83066BF0(ctx, base);
	// 83067058: 48000028  b 0x83067080
	pc = 0x83067080; continue 'dispatch;
	// 8306705C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83067060: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 83067064: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83067068: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 8306706C: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 83067070: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83067074: 409A04B0  bne cr6, 0x83067524
	if !ctx.cr[6].eq {
	pc = 0x83067524; continue 'dispatch;
	}
	// 83067078: 917E007C  stw r11, 0x7c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 8306707C: 4BFFF925  bl 0x830669a0
	ctx.lr = 0x83067080;
	sub_830669A0(ctx, base);
	// 83067080: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83067084: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 83067088: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 8306708C: 7F125840  cmplw cr6, r18, r11
	ctx.cr[6].compare_u32(ctx.r[18].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83067090: 419A00D4  beq cr6, 0x83067164
	if ctx.cr[6].eq {
	pc = 0x83067164; continue 'dispatch;
	}
	// 83067094: 2B110000  cmplwi cr6, r17, 0
	ctx.cr[6].compare_u32(ctx.r[17].u32, 0 as u32, &mut ctx.xer);
	// 83067098: 419A00A8  beq cr6, 0x83067140
	if ctx.cr[6].eq {
	pc = 0x83067140; continue 'dispatch;
	}
	// 8306709C: A1710000  lhz r11, 0(r17)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as u64;
	// 830670A0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830670A4: 4182009C  beq 0x83067140
	if ctx.cr[0].eq {
	pc = 0x83067140; continue 'dispatch;
	}
	// 830670A8: 3B9E0054  addi r28, r30, 0x54
	ctx.r[28].s64 = ctx.r[30].s64 + 84;
	// 830670AC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830670B0: 4BF77F49  bl 0x82fdeff8
	ctx.lr = 0x830670B4;
	sub_82FDEFF8(ctx, base);
	// 830670B4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830670B8: 939F0054  stw r28, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 830670BC: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 830670C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830670C4: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 830670C8: 927D0004  stw r19, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[19].u32 ) };
	// 830670CC: 4BF724A5  bl 0x82fd9570
	ctx.lr = 0x830670D0;
	sub_82FD9570(ctx, base);
	// 830670D0: 3880003A  li r4, 0x3a
	ctx.r[4].s64 = 58;
	// 830670D4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830670D8: 4BF69A41  bl 0x82fd0b18
	ctx.lr = 0x830670DC;
	sub_82FD0B18(ctx, base);
	// 830670DC: 81740008  lwz r11, 8(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(8 as u32) ) } as u64;
	// 830670E0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830670E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830670E8: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830670EC: 4BF72485  bl 0x82fd9570
	ctx.lr = 0x830670F0;
	sub_82FD9570(ctx, base);
	// 830670F0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830670F4: 7E449378  mr r4, r18
	ctx.r[4].u64 = ctx.r[18].u64;
	// 830670F8: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 830670FC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83067100: 7E6B532E  sthx r19, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[19].u16) };
	// 83067104: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067108: 835D0018  lwz r26, 0x18(r29)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8306710C: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83067110: 4BF80F49  bl 0x82fe8058
	ctx.lr = 0x83067114;
	sub_82FE8058(ctx, base);
	// 83067114: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83067118: 817B003C  lwz r11, 0x3c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(60 as u32) ) } as u64;
	// 8306711C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83067120: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83067124: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067128: 4E800421  bctrl
	ctx.lr = 0x8306712C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8306712C: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 83067130: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83067134: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83067138: 4BF77FF1  bl 0x82fdf128
	ctx.lr = 0x8306713C;
	sub_82FDF128(ctx, base);
	// 8306713C: 4800004C  b 0x83067188
	pc = 0x83067188; continue 'dispatch;
	// 83067140: 81740008  lwz r11, 8(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(8 as u32) ) } as u64;
	// 83067144: 7E449378  mr r4, r18
	ctx.r[4].u64 = ctx.r[18].u64;
	// 83067148: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8306714C: 838B0010  lwz r28, 0x10(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83067150: 4BF80F09  bl 0x82fe8058
	ctx.lr = 0x83067154;
	sub_82FE8058(ctx, base);
	// 83067154: 817D003C  lwz r11, 0x3c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 83067158: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8306715C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83067160: 48000018  b 0x83067178
	pc = 0x83067178; continue 'dispatch;
	// 83067164: 81740008  lwz r11, 8(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(8 as u32) ) } as u64;
	// 83067168: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8306716C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067170: 80AB0010  lwz r5, 0x10(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83067174: 816A003C  lwz r11, 0x3c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(60 as u32) ) } as u64;
	// 83067178: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8306717C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067180: 4E800421  bctrl
	ctx.lr = 0x83067184;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067184: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 83067188: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8306718C: 7E7A9B78  mr r26, r19
	ctx.r[26].u64 = ctx.r[19].u64;
	// 83067190: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 83067194: 3AAB8018  addi r21, r11, -0x7fe8
	ctx.r[21].s64 = ctx.r[11].s64 + -32744;
	// 83067198: 419A0160  beq cr6, 0x830672f8
	if ctx.cr[6].eq {
	pc = 0x830672F8; continue 'dispatch;
	}
	// 8306719C: 3F208214  lis r25, -0x7dec
	ctx.r[25].s64 = -2112618496;
	// 830671A0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830671A4: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 830671A8: 4BF86831  bl 0x82fed9d8
	ctx.lr = 0x830671AC;
	sub_82FED9D8(ctx, base);
	// 830671AC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830671B0: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 830671B4: 7E7B9B78  mr r27, r19
	ctx.r[27].u64 = ctx.r[19].u64;
	// 830671B8: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 830671BC: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830671C0: 83AB0020  lwz r29, 0x20(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830671C4: 4BF6CA7D  bl 0x82fd3c40
	ctx.lr = 0x830671C8;
	sub_82FD3C40(ctx, base);
	// 830671C8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830671CC: 4182000C  beq 0x830671d8
	if ctx.cr[0].eq {
	pc = 0x830671D8; continue 'dispatch;
	}
	// 830671D0: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 830671D4: 83AB0034  lwz r29, 0x34(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830671D8: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 830671DC: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 830671E0: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830671E4: 419A0010  beq cr6, 0x830671f4
	if ctx.cr[6].eq {
	pc = 0x830671F4; continue 'dispatch;
	}
	// 830671E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830671EC: 4BF80E6D  bl 0x82fe8058
	ctx.lr = 0x830671F0;
	sub_82FE8058(ctx, base);
	// 830671F0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 830671F4: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830671F8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830671FC: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067200: 4BF8AA69  bl 0x82ff1c68
	ctx.lr = 0x83067204;
	sub_82FF1C68(ctx, base);
	// 83067204: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83067208: 817D0044  lwz r11, 0x44(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(68 as u32) ) } as u64;
	// 8306720C: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83067210: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83067214: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067218: 4E800421  bctrl
	ctx.lr = 0x8306721C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8306721C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83067220: 809C000C  lwz r4, 0xc(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 83067224: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067228: 816B00A8  lwz r11, 0xa8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 8306722C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067230: 4E800421  bctrl
	ctx.lr = 0x83067234;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067234: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067238: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8306723C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 83067240: 816B00CC  lwz r11, 0xcc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(204 as u32) ) } as u64;
	// 83067244: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067248: 4E800421  bctrl
	ctx.lr = 0x8306724C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8306724C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83067250: 41820014  beq 0x83067264
	if ctx.cr[0].eq {
	pc = 0x83067264; continue 'dispatch;
	}
	// 83067254: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067258: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 8306725C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067260: 4E800421  bctrl
	ctx.lr = 0x83067264;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067264: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83067268: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8306726C: 409A0068  bne cr6, 0x830672d4
	if !ctx.cr[6].eq {
	pc = 0x830672D4; continue 'dispatch;
	}
	// 83067270: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83067274: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 83067278: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8306727C: 409A0034  bne cr6, 0x830672b0
	if !ctx.cr[6].eq {
	pc = 0x830672B0; continue 'dispatch;
	}
	// 83067280: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 83067284: 4BF7A76D  bl 0x82fe19f0
	ctx.lr = 0x83067288;
	sub_82FE19F0(ctx, base);
	// 83067288: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8306728C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83067290: 41820014  beq 0x830672a4
	if ctx.cr[0].eq {
	pc = 0x830672A4; continue 'dispatch;
	}
	// 83067294: 388001F4  li r4, 0x1f4
	ctx.r[4].s64 = 500;
	// 83067298: 80BE0030  lwz r5, 0x30(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8306729C: 4BF8A42D  bl 0x82ff16c8
	ctx.lr = 0x830672A0;
	sub_82FF16C8(ctx, base);
	// 830672A0: 48000008  b 0x830672a8
	pc = 0x830672A8; continue 'dispatch;
	// 830672A4: 7E639B78  mr r3, r19
	ctx.r[3].u64 = ctx.r[19].u64;
	// 830672A8: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830672AC: 906B0028  stw r3, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 830672B0: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830672B4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830672B8: 806B0028  lwz r3, 0x28(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 830672BC: 4BF8A555  bl 0x82ff1810
	ctx.lr = 0x830672C0;
	sub_82FF1810(ctx, base);
	// 830672C0: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 830672C4: A159A6B8  lhz r10, -0x5948(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[25].u32.wrapping_add(-22856 as u32) ) } as u64;
	// 830672C8: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830672CC: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 830672D0: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 830672D4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830672D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830672DC: 889C0000  lbz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830672E0: 816B00B8  lwz r11, 0xb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(184 as u32) ) } as u64;
	// 830672E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830672E8: 4E800421  bctrl
	ctx.lr = 0x830672EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830672EC: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 830672F0: 7F1AC040  cmplw cr6, r26, r24
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[24].u32, &mut ctx.xer);
	// 830672F4: 4198FEAC  blt cr6, 0x830671a0
	if ctx.cr[6].lt {
	pc = 0x830671A0; continue 'dispatch;
	}
	// 830672F8: 81740000  lwz r11, 0(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(0 as u32) ) } as u64;
	// 830672FC: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 83067300: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83067304: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067308: 4E800421  bctrl
	ctx.lr = 0x8306730C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8306730C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83067310: 418201AC  beq 0x830674bc
	if ctx.cr[0].eq {
	pc = 0x830674BC; continue 'dispatch;
	}
	// 83067314: 81740000  lwz r11, 0(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067318: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 8306731C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83067320: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067324: 4E800421  bctrl
	ctx.lr = 0x83067328;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067328: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 8306732C: 48000174  b 0x830674a0
	pc = 0x830674A0; continue 'dispatch;
	// 83067330: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067334: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 83067338: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8306733C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067340: 4E800421  bctrl
	ctx.lr = 0x83067344;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067344: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 83067348: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 8306734C: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83067350: 4182000C  beq 0x8306735c
	if ctx.cr[0].eq {
	pc = 0x8306735C; continue 'dispatch;
	}
	// 83067354: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83067358: 409A0134  bne cr6, 0x8306748c
	if !ctx.cr[6].eq {
	pc = 0x8306748C; continue 'dispatch;
	}
	// 8306735C: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067360: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83067364: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83067368: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8306736C: 4E800421  bctrl
	ctx.lr = 0x83067370;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067370: 3B5E0054  addi r26, r30, 0x54
	ctx.r[26].s64 = ctx.r[30].s64 + 84;
	// 83067374: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83067378: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8306737C: 4BF77C7D  bl 0x82fdeff8
	ctx.lr = 0x83067380;
	sub_82FDEFF8(ctx, base);
	// 83067380: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 83067384: 935F005C  stw r26, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[26].u32 ) };
	// 83067388: 931F0058  stw r24, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[24].u32 ) };
	// 8306738C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83067390: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83067394: 38FF0050  addi r7, r31, 0x50
	ctx.r[7].s64 = ctx.r[31].s64 + 80;
	// 83067398: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8306739C: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 830673A0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830673A4: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830673A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830673AC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 830673B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830673B4: 4E800421  bctrl
	ctx.lr = 0x830673B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830673B8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830673BC: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 830673C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830673C4: 7E7B9B78  mr r27, r19
	ctx.r[27].u64 = ctx.r[19].u64;
	// 830673C8: 4BF6C879  bl 0x82fd3c40
	ctx.lr = 0x830673CC;
	sub_82FD3C40(ctx, base);
	// 830673CC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830673D0: 4182000C  beq 0x830673dc
	if ctx.cr[0].eq {
	pc = 0x830673DC; continue 'dispatch;
	}
	// 830673D4: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 830673D8: 838B0034  lwz r28, 0x34(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830673DC: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 830673E0: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 830673E4: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830673E8: 419A0010  beq cr6, 0x830673f8
	if ctx.cr[6].eq {
	pc = 0x830673F8; continue 'dispatch;
	}
	// 830673EC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830673F0: 4BF80C69  bl 0x82fe8058
	ctx.lr = 0x830673F4;
	sub_82FE8058(ctx, base);
	// 830673F4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 830673F8: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830673FC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83067400: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83067404: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067408: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 8306740C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067410: 4E800421  bctrl
	ctx.lr = 0x83067414;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067414: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067418: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8306741C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 83067420: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83067424: 816B00F4  lwz r11, 0xf4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(244 as u32) ) } as u64;
	// 83067428: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8306742C: 4E800421  bctrl
	ctx.lr = 0x83067430;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067430: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83067434: 41820014  beq 0x83067448
	if ctx.cr[0].eq {
	pc = 0x83067448; continue 'dispatch;
	}
	// 83067438: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8306743C: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 83067440: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067444: 4E800421  bctrl
	ctx.lr = 0x83067448;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067448: 80990018  lwz r4, 0x18(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(24 as u32) ) } as u64;
	// 8306744C: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83067450: 41820030  beq 0x83067480
	if ctx.cr[0].eq {
	pc = 0x83067480; continue 'dispatch;
	}
	// 83067454: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067458: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8306745C: 816B00A8  lwz r11, 0xa8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 83067460: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067464: 4E800421  bctrl
	ctx.lr = 0x83067468;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067468: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8306746C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83067470: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83067474: 816B00B8  lwz r11, 0xb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(184 as u32) ) } as u64;
	// 83067478: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8306747C: 4E800421  bctrl
	ctx.lr = 0x83067480;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067480: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83067484: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83067488: 4BF77CA1  bl 0x82fdf128
	ctx.lr = 0x8306748C;
	sub_82FDF128(ctx, base);
	// 8306748C: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067490: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83067494: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83067498: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8306749C: 4E800421  bctrl
	ctx.lr = 0x830674A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830674A0: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 830674A4: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 830674A8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830674AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830674B0: 4E800421  bctrl
	ctx.lr = 0x830674B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830674B4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830674B8: 4082FE78  bne 0x83067330
	if !ctx.cr[0].eq {
	pc = 0x83067330; continue 'dispatch;
	}
	// 830674BC: 3BBE0024  addi r29, r30, 0x24
	ctx.r[29].s64 = ctx.r[30].s64 + 36;
	// 830674C0: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 830674C4: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830674C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830674CC: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 830674D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830674D4: 4E800421  bctrl
	ctx.lr = 0x830674D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830674D8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830674DC: 807E0034  lwz r3, 0x34(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 830674E0: 4BF7F729  bl 0x82fe6c08
	ctx.lr = 0x830674E4;
	sub_82FE6C08(ctx, base);
	// 830674E4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830674E8: 92FD0000  stw r23, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 830674EC: 92FE0028  stw r23, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[23].u32 ) };
	// 830674F0: 997E0016  stb r11, 0x16(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(22 as u32), ctx.r[11].u8 ) };
	// 830674F4: 560A063F  clrlwi. r10, r16, 0x18
	ctx.r[10].u64 = ctx.r[16].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830674F8: 41820030  beq 0x83067528
	if ctx.cr[0].eq {
	pc = 0x83067528; continue 'dispatch;
	}
	// 830674FC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067500: 7E278B78  mr r7, r17
	ctx.r[7].u64 = ctx.r[17].u64;
	// 83067504: 7DE67B78  mr r6, r15
	ctx.r[6].u64 = ctx.r[15].u64;
	// 83067508: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 8306750C: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 83067510: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83067514: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83067518: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8306751C: 4E800421  bctrl
	ctx.lr = 0x83067520;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067520: 48000008  b 0x83067528
	pc = 0x83067528; continue 'dispatch;
	// 83067524: 4BFFF47D  bl 0x830669a0
	ctx.lr = 0x83067528;
	sub_830669A0(ctx, base);
	// 83067528: 383F00F0  addi r1, r31, 0xf0
	ctx.r[1].s64 = ctx.r[31].s64 + 240;
	// 8306752C: 48140C58  b 0x831a8184
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83067530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83067530 size=40
    let mut pc: u32 = 0x83067530;
    'dispatch: loop {
        match pc {
            0x83067530 => {
    //   block [0x83067530..0x83067558)
	// 83067530: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83067534: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83067538: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8306753C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83067540: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83067544: 4BF6CA45  bl 0x82fd3f88
	ctx.lr = 0x83067548;
	sub_82FD3F88(ctx, base);
	// 83067548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8306754C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83067550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83067554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83067558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83067558 size=48
    let mut pc: u32 = 0x83067558;
    'dispatch: loop {
        match pc {
            0x83067558 => {
    //   block [0x83067558..0x83067588)
	// 83067558: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 8306755C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83067560: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83067564: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83067568: 817F0104  lwz r11, 0x104(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 8306756C: 808B0030  lwz r4, 0x30(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 83067570: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83067574: 4806056D  bl 0x830c7ae0
	ctx.lr = 0x83067578;
	sub_830C7AE0(ctx, base);
	// 83067578: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8306757C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83067580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83067584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83067588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83067588 size=40
    let mut pc: u32 = 0x83067588;
    'dispatch: loop {
        match pc {
            0x83067588 => {
    //   block [0x83067588..0x830675B0)
	// 83067588: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 8306758C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83067590: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83067594: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83067598: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 8306759C: 4BF6C9ED  bl 0x82fd3f88
	ctx.lr = 0x830675A0;
	sub_82FD3F88(ctx, base);
	// 830675A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830675A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830675A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830675AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830675B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830675B0 size=8
    let mut pc: u32 = 0x830675B0;
    'dispatch: loop {
        match pc {
            0x830675B0 => {
    //   block [0x830675B0..0x830675B8)
	// 830675B0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830675B4: 82166D90  lwz r16, 0x6d90(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(28048 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830675B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830675B8 size=156
    let mut pc: u32 = 0x830675B8;
    'dispatch: loop {
        match pc {
            0x830675B8 => {
    //   block [0x830675B8..0x83067654)
	// 830675B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830675BC: 48140BAD  bl 0x831a8168
	ctx.lr = 0x830675C0;
	sub_831A8130(ctx, base);
	// 830675C0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830675C4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830675C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830675CC: 90BF00A4  stw r5, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 830675D0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830675D4: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830675D8: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830675DC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830675E0: 90BE0004  stw r5, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 830675E4: 396BD9C8  addi r11, r11, -0x2638
	ctx.r[11].s64 = ctx.r[11].s64 + -9784;
	// 830675E8: 939E000C  stw r28, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 830675EC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 830675F0: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 830675F4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830675F8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830675FC: 93BE0014  stw r29, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 83067600: 93BE0018  stw r29, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 83067604: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 83067608: 997E001C  stb r11, 0x1c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 8306760C: 997E001D  stb r11, 0x1d(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 83067610: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83067614: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83067618: 419A0030  beq cr6, 0x83067648
	if ctx.cr[6].eq {
	pc = 0x83067648; continue 'dispatch;
	}
	// 8306761C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83067620: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 83067624: 4BF70C75  bl 0x82fd8298
	ctx.lr = 0x83067628;
	sub_82FD8298(ctx, base);
	// 83067628: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8306762C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83067630: 41820010  beq 0x83067640
	if ctx.cr[0].eq {
	pc = 0x83067640; continue 'dispatch;
	}
	// 83067634: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 83067638: 4BF76BB9  bl 0x82fde1f0
	ctx.lr = 0x8306763C;
	sub_82FDE1F0(ctx, base);
	// 8306763C: 48000008  b 0x83067644
	pc = 0x83067644; continue 'dispatch;
	// 83067640: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83067644: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 83067648: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8306764C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83067650: 48140B68  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83067654(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83067654 size=40
    let mut pc: u32 = 0x83067654;
    'dispatch: loop {
        match pc {
            0x83067654 => {
    //   block [0x83067654..0x8306767C)
	// 83067654: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83067658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306765C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83067660: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83067664: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83067668: 4BFE50F9  bl 0x8304c760
	ctx.lr = 0x8306766C;
	sub_8304C760(ctx, base);
	// 8306766C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83067670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83067674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83067678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8306767C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8306767C size=44
    let mut pc: u32 = 0x8306767C;
    'dispatch: loop {
        match pc {
            0x8306767C => {
    //   block [0x8306767C..0x830676A8)
	// 8306767C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83067680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83067684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83067688: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8306768C: 809F00A4  lwz r4, 0xa4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83067690: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83067694: 4BF70C4D  bl 0x82fd82e0
	ctx.lr = 0x83067698;
	sub_82FD82E0(ctx, base);
	// 83067698: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8306769C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830676A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830676A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830676A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830676A8 size=100
    let mut pc: u32 = 0x830676A8;
    'dispatch: loop {
        match pc {
            0x830676A8 => {
    //   block [0x830676A8..0x8306770C)
	// 830676A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830676AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830676B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830676B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830676B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830676BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830676C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830676C4: 809F0034  lwz r4, 0x34(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830676C8: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830676CC: 41820018  beq 0x830676e4
	if ctx.cr[0].eq {
	pc = 0x830676E4; continue 'dispatch;
	}
	// 830676D0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830676D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830676D8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830676DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830676E0: 4E800421  bctrl
	ctx.lr = 0x830676E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830676E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830676E8: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830676EC: 4BF69495  bl 0x82fd0b80
	ctx.lr = 0x830676F0;
	sub_82FD0B80(ctx, base);
	// 830676F0: 907F0034  stw r3, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[3].u32 ) };
	// 830676F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830676F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830676FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83067700: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83067704: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83067708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83067710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83067710 size=88
    let mut pc: u32 = 0x83067710;
    'dispatch: loop {
        match pc {
            0x83067710 => {
    //   block [0x83067710..0x83067768)
	// 83067710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83067714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83067718: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8306771C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83067720: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83067724: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83067728: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8306772C: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83067730: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83067734: 41820018  beq 0x8306774c
	if ctx.cr[0].eq {
	pc = 0x8306774C; continue 'dispatch;
	}
	// 83067738: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8306773C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83067740: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067744: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067748: 4E800421  bctrl
	ctx.lr = 0x8306774C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8306774C: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 83067750: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83067754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83067758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8306775C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83067760: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83067764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83067768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83067768 size=100
    let mut pc: u32 = 0x83067768;
    'dispatch: loop {
        match pc {
            0x83067768 => {
    //   block [0x83067768..0x830677CC)
	// 83067768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306776C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83067770: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83067774: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83067778: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8306777C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83067780: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83067784: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83067788: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8306778C: 41820018  beq 0x830677a4
	if ctx.cr[0].eq {
	pc = 0x830677A4; continue 'dispatch;
	}
	// 83067790: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83067794: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067798: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8306779C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830677A0: 4E800421  bctrl
	ctx.lr = 0x830677A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830677A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830677A8: 809F0034  lwz r4, 0x34(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830677AC: 4BF693D5  bl 0x82fd0b80
	ctx.lr = 0x830677B0;
	sub_82FD0B80(ctx, base);
	// 830677B0: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 830677B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830677B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830677BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830677C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830677C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830677C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830677D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830677D0 size=164
    let mut pc: u32 = 0x830677D0;
    'dispatch: loop {
        match pc {
            0x830677D0 => {
    //   block [0x830677D0..0x83067874)
	// 830677D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830677D4: 48140995  bl 0x831a8168
	ctx.lr = 0x830677D8;
	sub_831A8130(ctx, base);
	// 830677D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830677DC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830677E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830677E4: 3880003A  li r4, 0x3a
	ctx.r[4].s64 = 58;
	// 830677E8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830677EC: 4BF6A5C5  bl 0x82fd1db0
	ctx.lr = 0x830677F0;
	sub_82FD1DB0(ctx, base);
	// 830677F0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830677F4: 2F05FFFF  cmpwi cr6, r5, -1
	ctx.cr[6].compare_i32(ctx.r[5].s32, -1, &mut ctx.xer);
	// 830677F8: 419A006C  beq cr6, 0x83067864
	if ctx.cr[6].eq {
	pc = 0x83067864; continue 'dispatch;
	}
	// 830677FC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 83067800: 419A0064  beq cr6, 0x83067864
	if ctx.cr[6].eq {
	pc = 0x83067864; continue 'dispatch;
	}
	// 83067804: 3BFE003C  addi r31, r30, 0x3c
	ctx.r[31].s64 = ctx.r[30].s64 + 60;
	// 83067808: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8306780C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83067810: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83067814: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 83067818: 4BF71D59  bl 0x82fd9570
	ctx.lr = 0x8306781C;
	sub_82FD9570(ctx, base);
	// 8306781C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83067820: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83067824: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83067828: 7FAB532E  sthx r29, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u16) };
	// 8306782C: 807E0038  lwz r3, 0x38(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 83067830: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83067834: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83067838: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8306783C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067840: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83067844: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067848: 4E800421  bctrl
	ctx.lr = 0x8306784C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8306784C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83067850: 807E0038  lwz r3, 0x38(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 83067854: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83067858: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8306785C: 4E800421  bctrl
	ctx.lr = 0x83067860;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067860: 4800000C  b 0x8306786c
	pc = 0x8306786C; continue 'dispatch;
	// 83067864: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83067868: 386B8158  addi r3, r11, -0x7ea8
	ctx.r[3].s64 = ctx.r[11].s64 + -32424;
	// 8306786C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83067870: 48140948  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83067878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83067878 size=248
    let mut pc: u32 = 0x83067878;
    'dispatch: loop {
        match pc {
            0x83067878 => {
    //   block [0x83067878..0x83067970)
	// 83067878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306787C: 481408ED  bl 0x831a8168
	ctx.lr = 0x83067880;
	sub_831A8130(ctx, base);
	// 83067880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83067884: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83067888: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8306788C: 3880003A  li r4, 0x3a
	ctx.r[4].s64 = 58;
	// 83067890: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83067894: 4BF6A51D  bl 0x82fd1db0
	ctx.lr = 0x83067898;
	sub_82FD1DB0(ctx, base);
	// 83067898: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8306789C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830678A0: 419A0034  beq cr6, 0x830678d4
	if ctx.cr[6].eq {
	pc = 0x830678D4; continue 'dispatch;
	}
	// 830678A4: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830678A8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830678AC: 41820028  beq 0x830678d4
	if ctx.cr[0].eq {
	pc = 0x830678D4; continue 'dispatch;
	}
	// 830678B0: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 830678B4: 48000008  b 0x830678bc
	pc = 0x830678BC; continue 'dispatch;
	// 830678B8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830678BC: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830678C0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830678C4: 4082FFF4  bne 0x830678b8
	if !ctx.cr[0].eq {
	pc = 0x830678B8; continue 'dispatch;
	}
	// 830678C8: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 830678CC: 7D650E70  srawi r5, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830678D0: 48000008  b 0x830678d8
	pc = 0x830678D8; continue 'dispatch;
	// 830678D4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830678D8: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 830678DC: 7F0B2800  cmpw cr6, r11, r5
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[5].s32, &mut ctx.xer);
	// 830678E0: 409A0010  bne cr6, 0x830678f0
	if !ctx.cr[6].eq {
	pc = 0x830678F0; continue 'dispatch;
	}
	// 830678E4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830678E8: 386B8158  addi r3, r11, -0x7ea8
	ctx.r[3].s64 = ctx.r[11].s64 + -32424;
	// 830678EC: 4800007C  b 0x83067968
	pc = 0x83067968; continue 'dispatch;
	// 830678F0: 3BFD003C  addi r31, r29, 0x3c
	ctx.r[31].s64 = ctx.r[29].s64 + 60;
	// 830678F4: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 830678F8: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 830678FC: 409A000C  bne cr6, 0x83067908
	if !ctx.cr[6].eq {
	pc = 0x83067908; continue 'dispatch;
	}
	// 83067900: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83067904: 48000018  b 0x8306791c
	pc = 0x8306791C; continue 'dispatch;
	// 83067908: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 8306790C: 7D432850  subf r10, r3, r5
	ctx.r[10].s64 = ctx.r[5].s64 - ctx.r[3].s64;
	// 83067910: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83067914: 38AAFFFF  addi r5, r10, -1
	ctx.r[5].s64 = ctx.r[10].s64 + -1;
	// 83067918: 7C8BF214  add r4, r11, r30
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8306791C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83067920: 4BF71C51  bl 0x82fd9570
	ctx.lr = 0x83067924;
	sub_82FD9570(ctx, base);
	// 83067924: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83067928: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8306792C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83067930: 7F8B532E  sthx r28, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u16) };
	// 83067934: 807D0038  lwz r3, 0x38(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 83067938: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8306793C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83067940: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067944: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067948: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8306794C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067950: 4E800421  bctrl
	ctx.lr = 0x83067954;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067954: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83067958: 807D0038  lwz r3, 0x38(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 8306795C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83067960: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067964: 4E800421  bctrl
	ctx.lr = 0x83067968;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8306796C: 4814084C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83067970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83067970 size=224
    let mut pc: u32 = 0x83067970;
    'dispatch: loop {
        match pc {
            0x83067970 => {
    //   block [0x83067970..0x83067A50)
	// 83067970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83067974: 481407F9  bl 0x831a816c
	ctx.lr = 0x83067978;
	sub_831A8130(ctx, base);
	// 83067978: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8306797C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83067980: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83067984: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83067988: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8306798C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067990: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 83067994: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067998: 4E800421  bctrl
	ctx.lr = 0x8306799C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8306799C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830679A0: 418200A8  beq 0x83067a48
	if ctx.cr[0].eq {
	pc = 0x83067A48; continue 'dispatch;
	}
	// 830679A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830679A8: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 830679AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830679B0: 4E800421  bctrl
	ctx.lr = 0x830679B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830679B4: 57EB063F  clrlwi. r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830679B8: 41820090  beq 0x83067a48
	if ctx.cr[0].eq {
	pc = 0x83067A48; continue 'dispatch;
	}
	// 830679BC: 3BFE003C  addi r31, r30, 0x3c
	ctx.r[31].s64 = ctx.r[30].s64 + 60;
	// 830679C0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830679C4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830679C8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830679CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830679D0: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 830679D4: 4BF71B9D  bl 0x82fd9570
	ctx.lr = 0x830679D8;
	sub_82FD9570(ctx, base);
	// 830679D8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830679DC: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830679E0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830679E4: 7FAB532E  sthx r29, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u16) };
	// 830679E8: 83FF0018  lwz r31, 0x18(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830679EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830679F0: 4BF6A519  bl 0x82fd1f08
	ctx.lr = 0x830679F4;
	sub_82FD1F08(ctx, base);
	// 830679F4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830679F8: 419A0048  beq cr6, 0x83067a40
	if ctx.cr[6].eq {
	pc = 0x83067A40; continue 'dispatch;
	}
	// 830679FC: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067A00: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83067A04: 4182003C  beq 0x83067a40
	if ctx.cr[0].eq {
	pc = 0x83067A40; continue 'dispatch;
	}
	// 83067A08: 807E0038  lwz r3, 0x38(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 83067A0C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83067A10: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83067A14: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067A18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067A1C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83067A20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067A24: 4E800421  bctrl
	ctx.lr = 0x83067A28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067A28: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83067A2C: 807E0038  lwz r3, 0x38(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 83067A30: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83067A34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067A38: 4E800421  bctrl
	ctx.lr = 0x83067A3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067A3C: 4800000C  b 0x83067a48
	pc = 0x83067A48; continue 'dispatch;
	// 83067A40: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83067A44: 386B8158  addi r3, r11, -0x7ea8
	ctx.r[3].s64 = ctx.r[11].s64 + -32424;
	// 83067A48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83067A4C: 48140770  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83067A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83067A50 size=136
    let mut pc: u32 = 0x83067A50;
    'dispatch: loop {
        match pc {
            0x83067A50 => {
    //   block [0x83067A50..0x83067AD8)
	// 83067A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83067A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83067A58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83067A5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83067A60: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067A64: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83067A68: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 83067A6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067A70: 4E800421  bctrl
	ctx.lr = 0x83067A74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067A74: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83067A78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83067A7C: 388BCD64  addi r4, r11, -0x329c
	ctx.r[4].s64 = ctx.r[11].s64 + -12956;
	// 83067A80: 4BF6C1C1  bl 0x82fd3c40
	ctx.lr = 0x83067A84;
	sub_82FD3C40(ctx, base);
	// 83067A84: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83067A88: 40820038  bne 0x83067ac0
	if !ctx.cr[0].eq {
	pc = 0x83067AC0; continue 'dispatch;
	}
	// 83067A8C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83067A90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83067A94: 388BCD78  addi r4, r11, -0x3288
	ctx.r[4].s64 = ctx.r[11].s64 + -12936;
	// 83067A98: 4BF6C1A9  bl 0x82fd3c40
	ctx.lr = 0x83067A9C;
	sub_82FD3C40(ctx, base);
	// 83067A9C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83067AA0: 40820020  bne 0x83067ac0
	if !ctx.cr[0].eq {
	pc = 0x83067AC0; continue 'dispatch;
	}
	// 83067AA4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83067AA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83067AAC: 388BCD38  addi r4, r11, -0x32c8
	ctx.r[4].s64 = ctx.r[11].s64 + -13000;
	// 83067AB0: 4BF6C191  bl 0x82fd3c40
	ctx.lr = 0x83067AB4;
	sub_82FD3C40(ctx, base);
	// 83067AB4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83067AB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83067ABC: 41820008  beq 0x83067ac4
	if ctx.cr[0].eq {
	pc = 0x83067AC4; continue 'dispatch;
	}
	// 83067AC0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83067AC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83067AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83067ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83067AD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83067AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83067AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83067AD8 size=164
    let mut pc: u32 = 0x83067AD8;
    'dispatch: loop {
        match pc {
            0x83067AD8 => {
    //   block [0x83067AD8..0x83067B7C)
	// 83067AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83067ADC: 4814068D  bl 0x831a8168
	ctx.lr = 0x83067AE0;
	sub_831A8130(ctx, base);
	// 83067AE0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83067AE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83067AE8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83067AEC: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 83067AF0: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 83067AF4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83067AF8: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83067AFC: 80FF00D4  lwz r7, 0xd4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 83067B00: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 83067B04: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83067B08: 4BF69D61  bl 0x82fd1868
	ctx.lr = 0x83067B0C;
	sub_82FD1868(ctx, base);
	// 83067B0C: 3BDF003C  addi r30, r31, 0x3c
	ctx.r[30].s64 = ctx.r[31].s64 + 60;
	// 83067B10: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83067B14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83067B18: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83067B1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83067B20: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 83067B24: 4BF71A4D  bl 0x82fd9570
	ctx.lr = 0x83067B28;
	sub_82FD9570(ctx, base);
	// 83067B28: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83067B2C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83067B30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83067B34: 4BF71A3D  bl 0x82fd9570
	ctx.lr = 0x83067B38;
	sub_82FD9570(ctx, base);
	// 83067B38: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83067B3C: 815E0018  lwz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83067B40: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83067B44: 7F8B532E  sthx r28, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u16) };
	// 83067B48: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83067B4C: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83067B50: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067B54: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83067B58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067B5C: 4E800421  bctrl
	ctx.lr = 0x83067B60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067B60: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83067B64: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83067B68: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83067B6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067B70: 4E800421  bctrl
	ctx.lr = 0x83067B74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067B74: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83067B78: 48140640  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83067B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83067B80 size=88
    let mut pc: u32 = 0x83067B80;
    'dispatch: loop {
        match pc {
            0x83067B80 => {
    //   block [0x83067B80..0x83067BD8)
	// 83067B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83067B84: 481405E9  bl 0x831a816c
	ctx.lr = 0x83067B88;
	sub_831A8130(ctx, base);
	// 83067B88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83067B8C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83067B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83067B94: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83067B98: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83067B9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83067BA0: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83067BA4: 4BF719CD  bl 0x82fd9570
	ctx.lr = 0x83067BA8;
	sub_82FD9570(ctx, base);
	// 83067BA8: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83067BAC: 40990024  ble cr6, 0x83067bd0
	if !ctx.cr[6].gt {
	pc = 0x83067BD0; continue 'dispatch;
	}
	// 83067BB0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83067BB4: 3BABD91C  addi r29, r11, -0x26e4
	ctx.r[29].s64 = ctx.r[11].s64 + -9956;
	// 83067BB8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83067BBC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83067BC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83067BC4: 4BF719AD  bl 0x82fd9570
	ctx.lr = 0x83067BC8;
	sub_82FD9570(ctx, base);
	// 83067BC8: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83067BCC: 4082FFEC  bne 0x83067bb8
	if !ctx.cr[0].eq {
	pc = 0x83067BB8; continue 'dispatch;
	}
	// 83067BD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83067BD4: 481405E8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83067BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83067BD8 size=88
    let mut pc: u32 = 0x83067BD8;
    'dispatch: loop {
        match pc {
            0x83067BD8 => {
    //   block [0x83067BD8..0x83067C30)
	// 83067BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83067BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83067BE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83067BE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83067BE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83067BEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83067BF0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83067BF4: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83067BF8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83067BFC: 41820018  beq 0x83067c14
	if ctx.cr[0].eq {
	pc = 0x83067C14; continue 'dispatch;
	}
	// 83067C00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067C04: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83067C08: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067C0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067C10: 4E800421  bctrl
	ctx.lr = 0x83067C14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067C14: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 83067C18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83067C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83067C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83067C24: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83067C28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83067C2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83067C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83067C30 size=460
    let mut pc: u32 = 0x83067C30;
    'dispatch: loop {
        match pc {
            0x83067C30 => {
    //   block [0x83067C30..0x83067DFC)
	// 83067C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83067C34: 48140519  bl 0x831a814c
	ctx.lr = 0x83067C38;
	sub_831A8130(ctx, base);
	// 83067C38: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83067C3C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067C40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83067C44: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83067C48: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83067C4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067C50: 4E800421  bctrl
	ctx.lr = 0x83067C54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067C54: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83067C58: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 83067C5C: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067C60: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83067C64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067C68: 4E800421  bctrl
	ctx.lr = 0x83067C6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067C6C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83067C70: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 83067C74: 3AEB8158  addi r23, r11, -0x7ea8
	ctx.r[23].s64 = ctx.r[11].s64 + -32424;
	// 83067C78: 7C781B79  or. r24, r3, r3
	ctx.r[24].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 83067C7C: 40810118  ble 0x83067d94
	if !ctx.cr[0].gt {
	pc = 0x83067D94; continue 'dispatch;
	}
	// 83067C80: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83067C84: 3AAB8018  addi r21, r11, -0x7fe8
	ctx.r[21].s64 = ctx.r[11].s64 + -32744;
	// 83067C88: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83067C8C: 3ACB8024  addi r22, r11, -0x7fdc
	ctx.r[22].s64 = ctx.r[11].s64 + -32732;
	// 83067C90: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067C94: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83067C98: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83067C9C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83067CA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067CA4: 4E800421  bctrl
	ctx.lr = 0x83067CA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067CA8: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83067CAC: 418200E8  beq 0x83067d94
	if ctx.cr[0].eq {
	pc = 0x83067D94; continue 'dispatch;
	}
	// 83067CB0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067CB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83067CB8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83067CBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067CC0: 4E800421  bctrl
	ctx.lr = 0x83067CC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067CC4: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 83067CC8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83067CCC: 4BF720A5  bl 0x82fd9d70
	ctx.lr = 0x83067CD0;
	sub_82FD9D70(ctx, base);
	// 83067CD0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83067CD4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83067CD8: 4182005C  beq 0x83067d34
	if ctx.cr[0].eq {
	pc = 0x83067D34; continue 'dispatch;
	}
	// 83067CDC: 3880003A  li r4, 0x3a
	ctx.r[4].s64 = 58;
	// 83067CE0: 4BF6A0D1  bl 0x82fd1db0
	ctx.lr = 0x83067CE4;
	sub_82FD1DB0(ctx, base);
	// 83067CE4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067CE8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83067CEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83067CF0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83067CF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067CF8: 4E800421  bctrl
	ctx.lr = 0x83067CFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067CFC: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83067D00: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83067D04: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83067D08: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067D0C: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83067D10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067D14: 4E800421  bctrl
	ctx.lr = 0x83067D18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067D18: 397C0001  addi r11, r28, 1
	ctx.r[11].s64 = ctx.r[28].s64 + 1;
	// 83067D1C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83067D20: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83067D24: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83067D28: 7C8BEA14  add r4, r11, r29
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 83067D2C: 48032235  bl 0x83099f60
	ctx.lr = 0x83067D30;
	sub_83099F60(ctx, base);
	// 83067D30: 48000058  b 0x83067d88
	pc = 0x83067D88; continue 'dispatch;
	// 83067D34: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 83067D38: 4BF6BF09  bl 0x82fd3c40
	ctx.lr = 0x83067D3C;
	sub_82FD3C40(ctx, base);
	// 83067D3C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83067D40: 41820048  beq 0x83067d88
	if ctx.cr[0].eq {
	pc = 0x83067D88; continue 'dispatch;
	}
	// 83067D44: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067D48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83067D4C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83067D50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067D54: 4E800421  bctrl
	ctx.lr = 0x83067D58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067D58: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83067D5C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83067D60: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83067D64: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067D68: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83067D6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067D70: 4E800421  bctrl
	ctx.lr = 0x83067D74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067D74: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83067D78: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 83067D7C: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83067D80: 480321E1  bl 0x83099f60
	ctx.lr = 0x83067D84;
	sub_83099F60(ctx, base);
	// 83067D84: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 83067D88: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 83067D8C: 7F1AC000  cmpw cr6, r26, r24
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[24].s32, &mut ctx.xer);
	// 83067D90: 4198FF00  blt cr6, 0x83067c90
	if ctx.cr[6].lt {
	pc = 0x83067C90; continue 'dispatch;
	}
	// 83067D94: 572B063F  clrlwi. r11, r25, 0x18
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83067D98: 4082002C  bne 0x83067dc4
	if !ctx.cr[0].eq {
	pc = 0x83067DC4; continue 'dispatch;
	}
	// 83067D9C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83067DA0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83067DA4: 41820010  beq 0x83067db4
	if ctx.cr[0].eq {
	pc = 0x83067DB4; continue 'dispatch;
	}
	// 83067DA8: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067DAC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83067DB0: 40820014  bne 0x83067dc4
	if !ctx.cr[0].eq {
	pc = 0x83067DC4; continue 'dispatch;
	}
	// 83067DB4: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 83067DB8: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83067DBC: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83067DC0: 480321A1  bl 0x83099f60
	ctx.lr = 0x83067DC4;
	sub_83099F60(ctx, base);
	// 83067DC4: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83067DC8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83067DCC: 388B80C8  addi r4, r11, -0x7f38
	ctx.r[4].s64 = ctx.r[11].s64 + -32568;
	// 83067DD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067DD4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83067DD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067DDC: 4E800421  bctrl
	ctx.lr = 0x83067DE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067DE0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83067DE4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83067DE8: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83067DEC: 388B7F24  addi r4, r11, 0x7f24
	ctx.r[4].s64 = ctx.r[11].s64 + 32548;
	// 83067DF0: 48032171  bl 0x83099f60
	ctx.lr = 0x83067DF4;
	sub_83099F60(ctx, base);
	// 83067DF4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 83067DF8: 481403A4  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83067E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83067E00 size=124
    let mut pc: u32 = 0x83067E00;
    'dispatch: loop {
        match pc {
            0x83067E00 => {
    //   block [0x83067E00..0x83067E7C)
	// 83067E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83067E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83067E08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83067E0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83067E10: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83067E14: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83067E18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83067E1C: 388BCEA0  addi r4, r11, -0x3160
	ctx.r[4].s64 = ctx.r[11].s64 + -12640;
	// 83067E20: 4BF6BE21  bl 0x82fd3c40
	ctx.lr = 0x83067E24;
	sub_82FD3C40(ctx, base);
	// 83067E24: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83067E28: 40820038  bne 0x83067e60
	if !ctx.cr[0].eq {
	pc = 0x83067E60; continue 'dispatch;
	}
	// 83067E2C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83067E30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83067E34: 388BCEA8  addi r4, r11, -0x3158
	ctx.r[4].s64 = ctx.r[11].s64 + -12632;
	// 83067E38: 4BF6BE09  bl 0x82fd3c40
	ctx.lr = 0x83067E3C;
	sub_82FD3C40(ctx, base);
	// 83067E3C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83067E40: 40820020  bne 0x83067e60
	if !ctx.cr[0].eq {
	pc = 0x83067E60; continue 'dispatch;
	}
	// 83067E44: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83067E48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83067E4C: 388BD034  addi r4, r11, -0x2fcc
	ctx.r[4].s64 = ctx.r[11].s64 + -12236;
	// 83067E50: 4BF6BDF1  bl 0x82fd3c40
	ctx.lr = 0x83067E54;
	sub_82FD3C40(ctx, base);
	// 83067E54: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83067E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83067E5C: 41820008  beq 0x83067e64
	if ctx.cr[0].eq {
	pc = 0x83067E64; continue 'dispatch;
	}
	// 83067E60: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83067E64: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83067E68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83067E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83067E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83067E74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83067E78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83067E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83067E80 size=96
    let mut pc: u32 = 0x83067E80;
    'dispatch: loop {
        match pc {
            0x83067E80 => {
    //   block [0x83067E80..0x83067EE0)
	// 83067E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83067E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83067E88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83067E8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83067E90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83067E94: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83067E98: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83067E9C: 419A002C  beq cr6, 0x83067ec8
	if ctx.cr[6].eq {
	pc = 0x83067EC8; continue 'dispatch;
	}
	// 83067EA0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83067EA4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83067EA8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83067EAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83067EB0: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 83067EB4: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83067EB8: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 83067EBC: 4BFD6715  bl 0x8303e5d0
	ctx.lr = 0x83067EC0;
	sub_8303E5D0(ctx, base);
	// 83067EC0: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 83067EC4: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 83067EC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83067ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83067ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83067ED4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83067ED8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83067EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83067EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83067EE0 size=188
    let mut pc: u32 = 0x83067EE0;
    'dispatch: loop {
        match pc {
            0x83067EE0 => {
    //   block [0x83067EE0..0x83067F9C)
	// 83067EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83067EE4: 48140285  bl 0x831a8168
	ctx.lr = 0x83067EE8;
	sub_831A8130(ctx, base);
	// 83067EE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83067EEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83067EF0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83067EF4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83067EF8: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 83067EFC: 409A0094  bne cr6, 0x83067f90
	if !ctx.cr[6].eq {
	pc = 0x83067F90; continue 'dispatch;
	}
	// 83067F00: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83067F04: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83067F08: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83067F0C: 809E001C  lwz r4, 0x1c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83067F10: 83BE000C  lwz r29, 0xc(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83067F14: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83067F18: 4BF7F071  bl 0x82fe6f88
	ctx.lr = 0x83067F1C;
	sub_82FE6F88(ctx, base);
	// 83067F1C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83067F20: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 83067F24: 41820070  beq 0x83067f94
	if ctx.cr[0].eq {
	pc = 0x83067F94; continue 'dispatch;
	}
	// 83067F28: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 83067F2C: 3963003C  addi r11, r3, 0x3c
	ctx.r[11].s64 = ctx.r[3].s64 + 60;
	// 83067F30: 939F000C  stw r28, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 83067F34: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83067F38: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83067F3C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83067F40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067F44: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83067F48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83067F4C: 4E800421  bctrl
	ctx.lr = 0x83067F50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83067F50: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83067F54: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 83067F58: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83067F5C: 915F0068  stw r10, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 83067F60: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83067F64: 915F006C  stw r10, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 83067F68: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83067F6C: 915F0060  stw r10, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 83067F70: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83067F74: 915F0064  stw r10, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 83067F78: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83067F7C: 915F00B8  stw r10, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[10].u32 ) };
	// 83067F80: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83067F84: 915F005C  stw r10, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 83067F88: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 83067F8C: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 83067F90: 93DF0078  stw r30, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[30].u32 ) };
	// 83067F94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83067F98: 48140220  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83067FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83067FA0 size=76
    let mut pc: u32 = 0x83067FA0;
    'dispatch: loop {
        match pc {
            0x83067FA0 => {
    //   block [0x83067FA0..0x83067FEC)
	// 83067FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83067FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83067FA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83067FAC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83067FB0: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83067FB4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83067FB8: 41820020  beq 0x83067fd8
	if ctx.cr[0].eq {
	pc = 0x83067FD8; continue 'dispatch;
	}
	// 83067FBC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83067FC0: 419A0018  beq cr6, 0x83067fd8
	if ctx.cr[6].eq {
	pc = 0x83067FD8; continue 'dispatch;
	}
	// 83067FC4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83067FC8: 4801ABE9  bl 0x83082bb0
	ctx.lr = 0x83067FCC;
	sub_83082BB0(ctx, base);
	// 83067FCC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83067FD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83067FD4: 40820008  bne 0x83067fdc
	if !ctx.cr[0].eq {
	pc = 0x83067FDC; continue 'dispatch;
	}
	// 83067FD8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83067FDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83067FE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83067FE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83067FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83067FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83067FF0 size=488
    let mut pc: u32 = 0x83067FF0;
    'dispatch: loop {
        match pc {
            0x83067FF0 => {
    //   block [0x83067FF0..0x830681D8)
	// 83067FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83067FF4: 48140175  bl 0x831a8168
	ctx.lr = 0x83067FF8;
	sub_831A8130(ctx, base);
	// 83067FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83067FFC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83068000: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83068004: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83068008: 38ABD1D0  addi r5, r11, -0x2e30
	ctx.r[5].s64 = ctx.r[11].s64 + -11824;
	// 8306800C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83068010: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 83068014: 4BFFF95D  bl 0x83067970
	ctx.lr = 0x83068018;
	sub_83067970(ctx, base);
	// 83068018: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8306801C: 418201B4  beq 0x830681d0
	if ctx.cr[0].eq {
	pc = 0x830681D0; continue 'dispatch;
	}
	// 83068020: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83068024: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83068028: 418201A8  beq 0x830681d0
	if ctx.cr[0].eq {
	pc = 0x830681D0; continue 'dispatch;
	}
	// 8306802C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83068030: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83068034: 388BD54C  addi r4, r11, -0x2ab4
	ctx.r[4].s64 = ctx.r[11].s64 + -10932;
	// 83068038: 4BF6BC09  bl 0x82fd3c40
	ctx.lr = 0x8306803C;
	sub_82FD3C40(ctx, base);
	// 8306803C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83068040: 4082001C  bne 0x8306805c
	if !ctx.cr[0].eq {
	pc = 0x8306805C; continue 'dispatch;
	}
	// 83068044: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83068048: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8306804C: 388B6D38  addi r4, r11, 0x6d38
	ctx.r[4].s64 = ctx.r[11].s64 + 27960;
	// 83068050: 4BF6BBF1  bl 0x82fd3c40
	ctx.lr = 0x83068054;
	sub_82FD3C40(ctx, base);
	// 83068054: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83068058: 41820178  beq 0x830681d0
	if ctx.cr[0].eq {
	pc = 0x830681D0; continue 'dispatch;
	}
	// 8306805C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83068060: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83068064: 386BCEB8  addi r3, r11, -0x3148
	ctx.r[3].s64 = ctx.r[11].s64 + -12616;
	// 83068068: 4BF6BBD9  bl 0x82fd3c40
	ctx.lr = 0x8306806C;
	sub_82FD3C40(ctx, base);
	// 8306806C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83068070: 41820010  beq 0x83068080
	if ctx.cr[0].eq {
	pc = 0x83068080; continue 'dispatch;
	}
	// 83068074: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83068078: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 8306807C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83068080: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83068084: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83068088: 386BCF4C  addi r3, r11, -0x30b4
	ctx.r[3].s64 = ctx.r[11].s64 + -12468;
	// 8306808C: 4BF6BBB5  bl 0x82fd3c40
	ctx.lr = 0x83068090;
	sub_82FD3C40(ctx, base);
	// 83068090: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83068094: 41820010  beq 0x830680a4
	if ctx.cr[0].eq {
	pc = 0x830680A4; continue 'dispatch;
	}
	// 83068098: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8306809C: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 830680A0: 4800012C  b 0x830681cc
	pc = 0x830681CC; continue 'dispatch;
	// 830680A4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830680A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830680AC: 386BCF00  addi r3, r11, -0x3100
	ctx.r[3].s64 = ctx.r[11].s64 + -12544;
	// 830680B0: 4BF6BB91  bl 0x82fd3c40
	ctx.lr = 0x830680B4;
	sub_82FD3C40(ctx, base);
	// 830680B4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830680B8: 41820010  beq 0x830680c8
	if ctx.cr[0].eq {
	pc = 0x830680C8; continue 'dispatch;
	}
	// 830680BC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830680C0: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 830680C4: 48000108  b 0x830681cc
	pc = 0x830681CC; continue 'dispatch;
	// 830680C8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830680CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830680D0: 386BCEC8  addi r3, r11, -0x3138
	ctx.r[3].s64 = ctx.r[11].s64 + -12600;
	// 830680D4: 4BF6BB6D  bl 0x82fd3c40
	ctx.lr = 0x830680D8;
	sub_82FD3C40(ctx, base);
	// 830680D8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830680DC: 41820010  beq 0x830680ec
	if ctx.cr[0].eq {
	pc = 0x830680EC; continue 'dispatch;
	}
	// 830680E0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830680E4: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 830680E8: 480000E4  b 0x830681cc
	pc = 0x830681CC; continue 'dispatch;
	// 830680EC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830680F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830680F4: 386BCEE4  addi r3, r11, -0x311c
	ctx.r[3].s64 = ctx.r[11].s64 + -12572;
	// 830680F8: 4BF6BB49  bl 0x82fd3c40
	ctx.lr = 0x830680FC;
	sub_82FD3C40(ctx, base);
	// 830680FC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83068100: 41820010  beq 0x83068110
	if ctx.cr[0].eq {
	pc = 0x83068110; continue 'dispatch;
	}
	// 83068104: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83068108: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 8306810C: 480000C0  b 0x830681cc
	pc = 0x830681CC; continue 'dispatch;
	// 83068110: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83068114: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83068118: 386BCF14  addi r3, r11, -0x30ec
	ctx.r[3].s64 = ctx.r[11].s64 + -12524;
	// 8306811C: 4BF6BB25  bl 0x82fd3c40
	ctx.lr = 0x83068120;
	sub_82FD3C40(ctx, base);
	// 83068120: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83068124: 41820010  beq 0x83068134
	if ctx.cr[0].eq {
	pc = 0x83068134; continue 'dispatch;
	}
	// 83068128: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8306812C: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 83068130: 4800009C  b 0x830681cc
	pc = 0x830681CC; continue 'dispatch;
	// 83068134: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83068138: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8306813C: 386BCF30  addi r3, r11, -0x30d0
	ctx.r[3].s64 = ctx.r[11].s64 + -12496;
	// 83068140: 4BF6BB01  bl 0x82fd3c40
	ctx.lr = 0x83068144;
	sub_82FD3C40(ctx, base);
	// 83068144: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83068148: 41820010  beq 0x83068158
	if ctx.cr[0].eq {
	pc = 0x83068158; continue 'dispatch;
	}
	// 8306814C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83068150: 616B0080  ori r11, r11, 0x80
	ctx.r[11].u64 = ctx.r[11].u64 | 128;
	// 83068154: 48000078  b 0x830681cc
	pc = 0x830681CC; continue 'dispatch;
	// 83068158: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8306815C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83068160: 386BCF94  addi r3, r11, -0x306c
	ctx.r[3].s64 = ctx.r[11].s64 + -12396;
	// 83068164: 4BF6BADD  bl 0x82fd3c40
	ctx.lr = 0x83068168;
	sub_82FD3C40(ctx, base);
	// 83068168: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8306816C: 41820010  beq 0x8306817c
	if ctx.cr[0].eq {
	pc = 0x8306817C; continue 'dispatch;
	}
	// 83068170: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83068174: 616B0200  ori r11, r11, 0x200
	ctx.r[11].u64 = ctx.r[11].u64 | 512;
	// 83068178: 48000054  b 0x830681cc
	pc = 0x830681CC; continue 'dispatch;
	// 8306817C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83068180: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83068184: 386BCFAC  addi r3, r11, -0x3054
	ctx.r[3].s64 = ctx.r[11].s64 + -12372;
	// 83068188: 4BF6BAB9  bl 0x82fd3c40
	ctx.lr = 0x8306818C;
	sub_82FD3C40(ctx, base);
	// 8306818C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83068190: 41820010  beq 0x830681a0
	if ctx.cr[0].eq {
	pc = 0x830681A0; continue 'dispatch;
	}
	// 83068194: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83068198: 616B0400  ori r11, r11, 0x400
	ctx.r[11].u64 = ctx.r[11].u64 | 1024;
	// 8306819C: 48000030  b 0x830681cc
	pc = 0x830681CC; continue 'dispatch;
	// 830681A0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830681A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830681A8: 386BCE48  addi r3, r11, -0x31b8
	ctx.r[3].s64 = ctx.r[11].s64 + -12728;
	// 830681AC: 4BF6BA95  bl 0x82fd3c40
	ctx.lr = 0x830681B0;
	sub_82FD3C40(ctx, base);
	// 830681B0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830681B4: 4182001C  beq 0x830681d0
	if ctx.cr[0].eq {
	pc = 0x830681D0; continue 'dispatch;
	}
	// 830681B8: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 830681BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830681C0: 409A0010  bne cr6, 0x830681d0
	if !ctx.cr[6].eq {
	pc = 0x830681D0; continue 'dispatch;
	}
	// 830681C4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830681C8: 616B4000  ori r11, r11, 0x4000
	ctx.r[11].u64 = ctx.r[11].u64 | 16384;
	// 830681CC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830681D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830681D4: 4813FFE4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830681D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830681D8 size=80
    let mut pc: u32 = 0x830681D8;
    'dispatch: loop {
        match pc {
            0x830681D8 => {
    //   block [0x830681D8..0x83068228)
	// 830681D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830681DC: 4813FF91  bl 0x831a816c
	ctx.lr = 0x830681E0;
	sub_831A8130(ctx, base);
	// 830681E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830681E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830681E8: 80E40044  lwz r7, 0x44(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(68 as u32) ) } as u64;
	// 830681EC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830681F0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 830681F4: 80C40040  lwz r6, 0x40(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(64 as u32) ) } as u64;
	// 830681F8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830681FC: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83068200: 807F00D0  lwz r3, 0xd0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 83068204: 808B0018  lwz r4, 0x18(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83068208: 48036761  bl 0x8309e968
	ctx.lr = 0x8306820C;
	sub_8309E968(ctx, base);
	// 8306820C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83068210: 80DF00D0  lwz r6, 0xd0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 83068214: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83068218: 387F00C4  addi r3, r31, 0xc4
	ctx.r[3].s64 = ctx.r[31].s64 + 196;
	// 8306821C: 48035DDD  bl 0x8309dff8
	ctx.lr = 0x83068220;
	sub_8309DFF8(ctx, base);
	// 83068220: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83068224: 4813FF98  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83068228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83068228 size=120
    let mut pc: u32 = 0x83068228;
    'dispatch: loop {
        match pc {
            0x83068228 => {
    //   block [0x83068228..0x830682A0)
	// 83068228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306822C: 4813FF31  bl 0x831a815c
	ctx.lr = 0x83068230;
	sub_831A8130(ctx, base);
	// 83068230: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83068234: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83068238: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8306823C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83068240: 80C40040  lwz r6, 0x40(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(64 as u32) ) } as u64;
	// 83068244: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 83068248: 80E40044  lwz r7, 0x44(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(68 as u32) ) } as u64;
	// 8306824C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83068250: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83068254: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 83068258: 807F00D0  lwz r3, 0xd0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 8306825C: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 83068260: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 83068264: 808B0018  lwz r4, 0x18(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83068268: 48036701  bl 0x8309e968
	ctx.lr = 0x8306826C;
	sub_8309E968(ctx, base);
	// 8306826C: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 83068270: 7F2ACB78  mr r10, r25
	ctx.r[10].u64 = ctx.r[25].u64;
	// 83068274: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 83068278: 80DF00D0  lwz r6, 0xd0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 8306827C: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 83068280: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 83068284: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83068288: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8306828C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83068290: 387F00C4  addi r3, r31, 0xc4
	ctx.r[3].s64 = ctx.r[31].s64 + 196;
	// 83068294: 48035F15  bl 0x8309e1a8
	ctx.lr = 0x83068298;
	sub_8309E1A8(ctx, base);
	// 83068298: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8306829C: 4813FF10  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830682A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830682A0 size=8
    let mut pc: u32 = 0x830682A0;
    'dispatch: loop {
        match pc {
            0x830682A0 => {
    //   block [0x830682A0..0x830682A8)
	// 830682A0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830682A4: 82166E24  lwz r16, 0x6e24(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(28196 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830682A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830682A8 size=384
    let mut pc: u32 = 0x830682A8;
    'dispatch: loop {
        match pc {
            0x830682A8 => {
    //   block [0x830682A8..0x83068428)
	// 830682A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830682AC: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 830682B0: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 830682B4: 4813FEA5  bl 0x831a8158
	ctx.lr = 0x830682B8;
	sub_831A8130(ctx, base);
	// 830682B8: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 830682BC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830682C0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830682C4: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 830682C8: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 830682CC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 830682D0: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 830682D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830682D8: 93BF00B4  stw r29, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[29].u32 ) };
	// 830682DC: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 830682E0: 933F00BC  stw r25, 0xbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[25].u32 ) };
	// 830682E4: 935F00C4  stw r26, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[26].u32 ) };
	// 830682E8: 939F00CC  stw r28, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[28].u32 ) };
	// 830682EC: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 830682F0: 931F00D4  stw r24, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[24].u32 ) };
	// 830682F4: 997F0050  stb r11, 0x50(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 830682F8: 419A0084  beq cr6, 0x8306837c
	if ctx.cr[6].eq {
	pc = 0x8306837C; continue 'dispatch;
	}
	// 830682FC: 817B0018  lwz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 83068300: 2F0B0014  cmpwi cr6, r11, 0x14
	ctx.cr[6].compare_i32(ctx.r[11].s32, 20, &mut ctx.xer);
	// 83068304: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83068308: 3BCB8070  addi r30, r11, -0x7f90
	ctx.r[30].s64 = ctx.r[11].s64 + -32656;
	// 8306830C: 409A0024  bne cr6, 0x83068330
	if !ctx.cr[6].eq {
	pc = 0x83068330; continue 'dispatch;
	}
	// 83068310: 80FA0008  lwz r7, 8(r26)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 83068314: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83068318: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8306831C: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 83068320: 38C00069  li r6, 0x69
	ctx.r[6].s64 = 105;
	// 83068324: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83068328: 80E70010  lwz r7, 0x10(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(16 as u32) ) } as u64;
	// 8306832C: 4BFFFEFD  bl 0x83068228
	ctx.lr = 0x83068330;
	sub_83068228(ctx, base);
	// 83068330: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83068334: 80DD00D4  lwz r6, 0xd4(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(212 as u32) ) } as u64;
	// 83068338: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8306833C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83068340: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83068344: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83068348: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8306834C: 4E800421  bctrl
	ctx.lr = 0x83068350;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83068350: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83068354: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83068358: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8306835C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83068360: 997F0050  stb r11, 0x50(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 83068364: 48000020  b 0x83068384
	pc = 0x83068384; continue 'dispatch;
	// 83068368: 48000004  b 0x8306836c
	pc = 0x8306836C; continue 'dispatch;
	// 8306836C: 833F00BC  lwz r25, 0xbc(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 83068370: 835F00C4  lwz r26, 0xc4(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 83068374: 831F00D4  lwz r24, 0xd4(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 83068378: 83BF00B4  lwz r29, 0xb4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8306837C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83068380: 3BCB8070  addi r30, r11, -0x7f90
	ctx.r[30].s64 = ctx.r[11].s64 + -32656;
	// 83068384: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 83068388: 419A0094  beq cr6, 0x8306841c
	if ctx.cr[6].eq {
	pc = 0x8306841C; continue 'dispatch;
	}
	// 8306838C: 83980020  lwz r28, 0x20(r24)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(32 as u32) ) } as u64;
	// 83068390: 2F1C0005  cmpwi cr6, r28, 5
	ctx.cr[6].compare_i32(ctx.r[28].s32, 5, &mut ctx.xer);
	// 83068394: 419A003C  beq cr6, 0x830683d0
	if ctx.cr[6].eq {
	pc = 0x830683D0; continue 'dispatch;
	}
	// 83068398: 2F1C0002  cmpwi cr6, r28, 2
	ctx.cr[6].compare_i32(ctx.r[28].s32, 2, &mut ctx.xer);
	// 8306839C: 419A0034  beq cr6, 0x830683d0
	if ctx.cr[6].eq {
	pc = 0x830683D0; continue 'dispatch;
	}
	// 830683A0: 2F1C0003  cmpwi cr6, r28, 3
	ctx.cr[6].compare_i32(ctx.r[28].s32, 3, &mut ctx.xer);
	// 830683A4: 419A003C  beq cr6, 0x830683e0
	if ctx.cr[6].eq {
	pc = 0x830683E0; continue 'dispatch;
	}
	// 830683A8: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 830683AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830683B0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830683B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830683B8: 38C0003E  li r6, 0x3e
	ctx.r[6].s64 = 62;
	// 830683BC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830683C0: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830683C4: 80EB0010  lwz r7, 0x10(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830683C8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830683CC: 4BFFFE5D  bl 0x83068228
	ctx.lr = 0x830683D0;
	sub_83068228(ctx, base);
	// 830683D0: 2F1C0003  cmpwi cr6, r28, 3
	ctx.cr[6].compare_i32(ctx.r[28].s32, 3, &mut ctx.xer);
	// 830683D4: 419A000C  beq cr6, 0x830683e0
	if ctx.cr[6].eq {
	pc = 0x830683E0; continue 'dispatch;
	}
	// 830683D8: 2F1C0002  cmpwi cr6, r28, 2
	ctx.cr[6].compare_i32(ctx.r[28].s32, 2, &mut ctx.xer);
	// 830683DC: 409A0040  bne cr6, 0x8306841c
	if !ctx.cr[6].eq {
	pc = 0x8306841C; continue 'dispatch;
	}
	// 830683E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830683E4: 8098003C  lwz r4, 0x3c(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(60 as u32) ) } as u64;
	// 830683E8: 4BFFFBB9  bl 0x83067fa0
	ctx.lr = 0x830683EC;
	sub_83067FA0(ctx, base);
	// 830683EC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830683F0: 4082002C  bne 0x8306841c
	if !ctx.cr[0].eq {
	pc = 0x8306841C; continue 'dispatch;
	}
	// 830683F4: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 830683F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830683FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83068400: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83068404: 38C0006B  li r6, 0x6b
	ctx.r[6].s64 = 107;
	// 83068408: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8306840C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83068410: 80EB0010  lwz r7, 0x10(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83068414: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83068418: 4BFFFE11  bl 0x83068228
	ctx.lr = 0x8306841C;
	sub_83068228(ctx, base);
	// 8306841C: 887F0050  lbz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83068420: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 83068424: 4813FD84  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83068428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83068428 size=8
    let mut pc: u32 = 0x83068428;
    'dispatch: loop {
        match pc {
            0x83068428 => {
    //   block [0x83068428..0x83068430)
	// 83068428: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8306842C: 82166E24  lwz r16, 0x6e24(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(28196 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83068430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83068430 size=84
    let mut pc: u32 = 0x83068430;
    'dispatch: loop {
        match pc {
            0x83068430 => {
    //   block [0x83068430..0x83068484)
	// 83068430: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83068434: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83068438: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8306843C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83068440: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83068444: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83068448: 809F00BC  lwz r4, 0xbc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 8306844C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83068450: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 83068454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83068458: 38C00049  li r6, 0x49
	ctx.r[6].s64 = 73;
	// 8306845C: 80EB0010  lwz r7, 0x10(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83068460: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83068464: 38AB7EB0  addi r5, r11, 0x7eb0
	ctx.r[5].s64 = ctx.r[11].s64 + 32432;
	// 83068468: 4BFFFDC1  bl 0x83068228
	ctx.lr = 0x8306846C;
	sub_83068228(ctx, base);
	// 8306846C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83068470: 3C608307  lis r3, -0x7cf9
	ctx.r[3].s64 = -2096693248;
	// 83068474: 38638368  addi r3, r3, -0x7c98
	ctx.r[3].s64 = ctx.r[3].s64 + -31896;
	// 83068478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8306847C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83068480: 4813FD28  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83068484(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83068484 size=8
    let mut pc: u32 = 0x83068484;
    'dispatch: loop {
        match pc {
            0x83068484 => {
    //   block [0x83068484..0x8306848C)
	// 83068484: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83068488: 82166E24  lwz r16, 0x6e24(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(28196 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8306848C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8306848C size=24
    let mut pc: u32 = 0x8306848C;
    'dispatch: loop {
        match pc {
            0x8306848C => {
    //   block [0x8306848C..0x830684A4)
	// 8306848C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83068490: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83068494: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83068498: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8306849C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830684A0: 48148789  bl 0x831b0c28
	ctx.lr = 0x830684A4;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830684AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830684AC size=80
    let mut pc: u32 = 0x830684AC;
    'dispatch: loop {
        match pc {
            0x830684AC => {
    //   block [0x830684AC..0x830684FC)
	// 830684AC: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 830684B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830684B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830684B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830684BC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 830684C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830684C4: 80FF00CC  lwz r7, 0xcc(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 830684C8: 38AB7EB0  addi r5, r11, 0x7eb0
	ctx.r[5].s64 = ctx.r[11].s64 + 32432;
	// 830684CC: 809F00BC  lwz r4, 0xbc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 830684D0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830684D4: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 830684D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830684DC: 38C00057  li r6, 0x57
	ctx.r[6].s64 = 87;
	// 830684E0: 4BFFFD49  bl 0x83068228
	ctx.lr = 0x830684E4;
	sub_83068228(ctx, base);
	// 830684E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830684E8: 3C608307  lis r3, -0x7cf9
	ctx.r[3].s64 = -2096693248;
	// 830684EC: 3863836C  addi r3, r3, -0x7c94
	ctx.r[3].s64 = ctx.r[3].s64 + -31892;
	// 830684F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830684F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830684F8: 4813FCB0  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83068500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83068500 size=188
    let mut pc: u32 = 0x83068500;
    'dispatch: loop {
        match pc {
            0x83068500 => {
    //   block [0x83068500..0x830685BC)
	// 83068500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83068504: 4813FC61  bl 0x831a8164
	ctx.lr = 0x83068508;
	sub_831A8130(ctx, base);
	// 83068508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8306850C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83068510: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83068514: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 83068518: A08B0000  lhz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8306851C: 7C8B2379  or. r11, r4, r4
	ctx.r[11].u64 = ctx.r[4].u64 | ctx.r[4].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83068520: 41820094  beq 0x830685b4
	if ctx.cr[0].eq {
	pc = 0x830685B4; continue 'dispatch;
	}
	// 83068524: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 83068528: 3BAA8168  addi r29, r10, -0x7e98
	ctx.r[29].s64 = ctx.r[10].s64 + -32408;
	// 8306852C: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 83068530: 3B8A8178  addi r28, r10, -0x7e88
	ctx.r[28].s64 = ctx.r[10].s64 + -32392;
	// 83068534: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 83068538: 3B6A8180  addi r27, r10, -0x7e80
	ctx.r[27].s64 = ctx.r[10].s64 + -32384;
	// 8306853C: 2B0B0022  cmplwi cr6, r11, 0x22
	ctx.cr[6].compare_u32(ctx.r[11].u32, 34 as u32, &mut ctx.xer);
	// 83068540: 409A0018  bne cr6, 0x83068558
	if !ctx.cr[6].eq {
	pc = 0x83068558; continue 'dispatch;
	}
	// 83068544: 38800026  li r4, 0x26
	ctx.r[4].s64 = 38;
	// 83068548: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8306854C: 4BF685CD  bl 0x82fd0b18
	ctx.lr = 0x83068550;
	sub_82FD0B18(ctx, base);
	// 83068550: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83068554: 48000038  b 0x8306858c
	pc = 0x8306858C; continue 'dispatch;
	// 83068558: 2B0B003E  cmplwi cr6, r11, 0x3e
	ctx.cr[6].compare_u32(ctx.r[11].u32, 62 as u32, &mut ctx.xer);
	// 8306855C: 409A0018  bne cr6, 0x83068574
	if !ctx.cr[6].eq {
	pc = 0x83068574; continue 'dispatch;
	}
	// 83068560: 38800026  li r4, 0x26
	ctx.r[4].s64 = 38;
	// 83068564: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83068568: 4BF685B1  bl 0x82fd0b18
	ctx.lr = 0x8306856C;
	sub_82FD0B18(ctx, base);
	// 8306856C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83068570: 4800001C  b 0x8306858c
	pc = 0x8306858C; continue 'dispatch;
	// 83068574: 2B0B0026  cmplwi cr6, r11, 0x26
	ctx.cr[6].compare_u32(ctx.r[11].u32, 38 as u32, &mut ctx.xer);
	// 83068578: 409A0024  bne cr6, 0x8306859c
	if !ctx.cr[6].eq {
	pc = 0x8306859C; continue 'dispatch;
	}
	// 8306857C: 38800026  li r4, 0x26
	ctx.r[4].s64 = 38;
	// 83068580: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83068584: 4BF68595  bl 0x82fd0b18
	ctx.lr = 0x83068588;
	sub_82FD0B18(ctx, base);
	// 83068588: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8306858C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83068590: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83068594: 4BF70FDD  bl 0x82fd9570
	ctx.lr = 0x83068598;
	sub_82FD9570(ctx, base);
	// 83068598: 3880003B  li r4, 0x3b
	ctx.r[4].s64 = 59;
	// 8306859C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830685A0: 4BF68579  bl 0x82fd0b18
	ctx.lr = 0x830685A4;
	sub_82FD0B18(ctx, base);
	// 830685A4: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 830685A8: A09E0000  lhz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830685AC: 7C8B2379  or. r11, r4, r4
	ctx.r[11].u64 = ctx.r[4].u64 | ctx.r[4].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830685B0: 4082FF8C  bne 0x8306853c
	if !ctx.cr[0].eq {
	pc = 0x8306853C; continue 'dispatch;
	}
	// 830685B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830685B8: 4813FBFC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830685C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830685C0 size=204
    let mut pc: u32 = 0x830685C0;
    'dispatch: loop {
        match pc {
            0x830685C0 => {
    //   block [0x830685C0..0x8306868C)
	// 830685C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830685C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830685C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830685CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830685D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830685D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830685D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830685DC: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 830685E0: 419A0090  beq cr6, 0x83068670
	if ctx.cr[6].eq {
	pc = 0x83068670; continue 'dispatch;
	}
	// 830685E4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830685E8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830685EC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830685F0: 40980044  bge cr6, 0x83068634
	if !ctx.cr[6].lt {
	pc = 0x83068634; continue 'dispatch;
	}
	// 830685F4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830685F8: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830685FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83068600: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83068604: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83068608: 4E800421  bctrl
	ctx.lr = 0x8306860C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8306860C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83068610: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83068614: 5544103A  slwi r4, r10, 2
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83068618: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8306861C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83068620: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83068624: 4E800421  bctrl
	ctx.lr = 0x83068628;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83068628: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 8306862C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83068630: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83068634: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83068638: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8306863C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83068640: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83068644: 4182002C  beq 0x83068670
	if ctx.cr[0].eq {
	pc = 0x83068670; continue 'dispatch;
	}
	// 83068648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8306864C: 813E000C  lwz r9, 0xc(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83068650: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83068654: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83068658: 7D29582E  lwzx r9, r9, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8306865C: 7D28592E  stwx r9, r8, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 83068660: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83068664: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83068668: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8306866C: 4198FFE0  blt cr6, 0x8306864c
	if ctx.cr[6].lt {
	pc = 0x8306864C; continue 'dispatch;
	}
	// 83068670: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83068674: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83068678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8306867C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83068680: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83068684: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83068688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83068690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83068690 size=8
    let mut pc: u32 = 0x83068690;
    'dispatch: loop {
        match pc {
            0x83068690 => {
    //   block [0x83068690..0x83068698)
	// 83068690: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83068694: 82166EA0  lwz r16, 0x6ea0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(28320 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83068698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83068698 size=124
    let mut pc: u32 = 0x83068698;
    'dispatch: loop {
        match pc {
            0x83068698 => {
    //   block [0x83068698..0x83068714)
	// 83068698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306869C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830686A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830686A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830686A8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830686AC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830686B0: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830686B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830686B8: 396B6E88  addi r11, r11, 0x6e88
	ctx.r[11].s64 = ctx.r[11].s64 + 28296;
	// 830686BC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830686C0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830686C4: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830686C8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830686CC: 41820024  beq 0x830686f0
	if ctx.cr[0].eq {
	pc = 0x830686F0; continue 'dispatch;
	}
	// 830686D0: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830686D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830686D8: 41820018  beq 0x830686f0
	if ctx.cr[0].eq {
	pc = 0x830686F0; continue 'dispatch;
	}
	// 830686DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830686E0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830686E4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830686E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830686EC: 4E800421  bctrl
	ctx.lr = 0x830686F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830686F0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830686F4: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 830686F8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830686FC: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83068700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83068704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83068708: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8306870C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83068710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83068714(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83068714 size=40
    let mut pc: u32 = 0x83068714;
    'dispatch: loop {
        match pc {
            0x83068714 => {
    //   block [0x83068714..0x8306873C)
	// 83068714: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83068718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306871C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83068720: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83068724: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83068728: 4BFE4039  bl 0x8304c760
	ctx.lr = 0x8306872C;
	sub_8304C760(ctx, base);
	// 8306872C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83068730: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83068734: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83068738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83068740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83068740 size=12
    let mut pc: u32 = 0x83068740;
    'dispatch: loop {
        match pc {
            0x83068740 => {
    //   block [0x83068740..0x8306874C)
	// 83068740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83068744: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83068748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83068750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83068750 size=68
    let mut pc: u32 = 0x83068750;
    'dispatch: loop {
        match pc {
            0x83068750 => {
    //   block [0x83068750..0x83068794)
	// 83068750: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83068754: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
	// 83068758: 7F053840  cmplw cr6, r5, r7
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8306875C: 40980030  bge cr6, 0x8306878c
	if !ctx.cr[6].lt {
	pc = 0x8306878C; continue 'dispatch;
	}
	// 83068760: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83068764: 54AB103A  slwi r11, r5, 2
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83068768: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8306876C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83068770: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83068774: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83068778: 419A001C  beq cr6, 0x83068794
	if ctx.cr[6].eq {
		sub_83068794(ctx, base);
		return;
	}
	// 8306877C: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 83068780: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83068784: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 83068788: 4198FFE8  blt cr6, 0x83068770
	if ctx.cr[6].lt {
	pc = 0x83068770; continue 'dispatch;
	}
	// 8306878C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83068790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83068794(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83068794 size=8
    let mut pc: u32 = 0x83068794;
    'dispatch: loop {
        match pc {
            0x83068794 => {
    //   block [0x83068794..0x8306879C)
	// 83068794: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83068798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830687A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830687A0 size=116
    let mut pc: u32 = 0x830687A0;
    'dispatch: loop {
        match pc {
            0x830687A0 => {
    //   block [0x830687A0..0x83068814)
	// 830687A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830687A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830687A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830687AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830687B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830687B4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 830687B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830687BC: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830687C0: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830687C4: 98DF0000  stb r6, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[6].u8 ) };
	// 830687C8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830687CC: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830687D0: 90BF0010  stw r5, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 830687D4: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830687D8: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830687DC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830687E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830687E4: 4E800421  bctrl
	ctx.lr = 0x830687E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830687E8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830687EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830687F0: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 830687F4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 830687F8: 4813F9E9  bl 0x831a81e0
	ctx.lr = 0x830687FC;
	sub_831A81E0(ctx, base);
	// 830687FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83068800: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83068804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83068808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8306880C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83068810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83068818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83068818 size=68
    let mut pc: u32 = 0x83068818;
    'dispatch: loop {
        match pc {
            0x83068818 => {
    //   block [0x83068818..0x8306885C)
	// 83068818: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8306881C: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
	// 83068820: 7F053840  cmplw cr6, r5, r7
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[7].u32, &mut ctx.xer);
	// 83068824: 40980030  bge cr6, 0x83068854
	if !ctx.cr[6].lt {
	pc = 0x83068854; continue 'dispatch;
	}
	// 83068828: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8306882C: 54AB103A  slwi r11, r5, 2
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83068830: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83068834: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83068838: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8306883C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83068840: 419A001C  beq cr6, 0x8306885c
	if ctx.cr[6].eq {
		sub_8306885C(ctx, base);
		return;
	}
	// 83068844: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 83068848: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8306884C: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 83068850: 4198FFE8  blt cr6, 0x83068838
	if ctx.cr[6].lt {
	pc = 0x83068838; continue 'dispatch;
	}
	// 83068854: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83068858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8306885C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8306885C size=8
    let mut pc: u32 = 0x8306885C;
    'dispatch: loop {
        match pc {
            0x8306885C => {
    //   block [0x8306885C..0x83068864)
	// 8306885C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83068860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83068868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83068868 size=16
    let mut pc: u32 = 0x83068868;
    'dispatch: loop {
        match pc {
            0x83068868 => {
    //   block [0x83068868..0x83068878)
	// 83068868: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8306886C: 396B6ED0  addi r11, r11, 0x6ed0
	ctx.r[11].s64 = ctx.r[11].s64 + 28368;
	// 83068870: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83068874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83068878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83068878 size=76
    let mut pc: u32 = 0x83068878;
    'dispatch: loop {
        match pc {
            0x83068878 => {
    //   block [0x83068878..0x830688C4)
	// 83068878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306887C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83068880: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83068884: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83068888: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8306888C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83068890: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83068894: 4BFFFE05  bl 0x83068698
	ctx.lr = 0x83068898;
	sub_83068698(ctx, base);
	// 83068898: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8306889C: 4182000C  beq 0x830688a8
	if ctx.cr[0].eq {
	pc = 0x830688A8; continue 'dispatch;
	}
	// 830688A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830688A4: 4BF6FA3D  bl 0x82fd82e0
	ctx.lr = 0x830688A8;
	sub_82FD82E0(ctx, base);
	// 830688A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830688AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830688B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830688B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830688B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830688BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830688C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830688C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830688C8 size=128
    let mut pc: u32 = 0x830688C8;
    'dispatch: loop {
        match pc {
            0x830688C8 => {
    //   block [0x830688C8..0x83068948)
	// 830688C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830688CC: 4813F8A1  bl 0x831a816c
	ctx.lr = 0x830688D0;
	sub_831A8130(ctx, base);
	// 830688D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830688D4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830688D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830688DC: 396B6ED0  addi r11, r11, 0x6ed0
	ctx.r[11].s64 = ctx.r[11].s64 + 28368;
	// 830688E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830688E4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830688E8: 57C4103A  slwi r4, r30, 2
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830688EC: 98BF0004  stb r5, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 830688F0: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 830688F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830688F8: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 830688FC: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 83068900: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 83068904: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 83068908: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8306890C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83068910: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83068914: 4E800421  bctrl
	ctx.lr = 0x83068918;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83068918: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8306891C: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83068920: 419A001C  beq cr6, 0x8306893c
	if ctx.cr[6].eq {
	pc = 0x8306893C; continue 'dispatch;
	}
	// 83068924: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 83068928: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8306892C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83068930: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 83068934: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83068938: 4082FFF0  bne 0x83068928
	if !ctx.cr[0].eq {
	pc = 0x83068928; continue 'dispatch;
	}
	// 8306893C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83068940: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83068944: 4813F878  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83068948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83068948 size=160
    let mut pc: u32 = 0x83068948;
    'dispatch: loop {
        match pc {
            0x83068948 => {
    //   block [0x83068948..0x830689E8)
	// 83068948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8306894C: 4813F811  bl 0x831a815c
	ctx.lr = 0x83068950;
	sub_831A8130(ctx, base);
	// 83068950: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83068954: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83068958: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8306895C: 7F3ACB78  mr r26, r25
	ctx.r[26].u64 = ctx.r[25].u64;
	// 83068960: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83068964: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83068968: 40990074  ble cr6, 0x830689dc
	if !ctx.cr[6].gt {
	pc = 0x830689DC; continue 'dispatch;
	}
	// 8306896C: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 83068970: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83068974: 7FCBE82E  lwzx r30, r11, r29
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 83068978: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8306897C: 41820044  beq 0x830689c0
	if ctx.cr[0].eq {
	pc = 0x830689C0; continue 'dispatch;
	}
	// 83068980: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83068984: 837E0004  lwz r27, 4(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83068988: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8306898C: 41820020  beq 0x830689ac
	if ctx.cr[0].eq {
	pc = 0x830689AC; continue 'dispatch;
	}
	// 83068990: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83068994: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83068998: 41820014  beq 0x830689ac
	if ctx.cr[0].eq {
	pc = 0x830689AC; continue 'dispatch;
	}
	// 8306899C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830689A0: 480361C1  bl 0x8309eb60
	ctx.lr = 0x830689A4;
	sub_8309EB60(ctx, base);
	// 830689A4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830689A8: 4BF6F939  bl 0x82fd82e0
	ctx.lr = 0x830689AC;
	sub_82FD82E0(ctx, base);
	// 830689AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830689B0: 4BF6F931  bl 0x82fd82e0
	ctx.lr = 0x830689B4;
	sub_82FD82E0(ctx, base);
	// 830689B4: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 830689B8: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 830689BC: 409AFFC4  bne cr6, 0x83068980
	if !ctx.cr[6].eq {
	pc = 0x83068980; continue 'dispatch;
	}
	// 830689C0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830689C4: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 830689C8: 7F2BE92E  stwx r25, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[25].u32) };
	// 830689CC: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 830689D0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830689D4: 7F1A5840  cmplw cr6, r26, r11
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830689D8: 4198FF98  blt cr6, 0x83068970
	if ctx.cr[6].lt {
	pc = 0x83068970; continue 'dispatch;
	}
	// 830689DC: 933F0014  stw r25, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[25].u32 ) };
	// 830689E0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830689E4: 4813F7C8  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


