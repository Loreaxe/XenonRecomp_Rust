pub fn sub_83095FD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095FD4 size=48
    let mut pc: u32 = 0x83095FD4;
    'dispatch: loop {
        match pc {
            0x83095FD4 => {
    //   block [0x83095FD4..0x83096004)
	// 83095FD4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83095FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83095FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83095FE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095FE4: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83095FE8: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83095FEC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83095FF0: 4BF422F1  bl 0x82fd82e0
	ctx.lr = 0x83095FF4;
	sub_82FD82E0(ctx, base);
	// 83095FF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83095FF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83095FFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83096000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83096008 size=12
    let mut pc: u32 = 0x83096008;
    'dispatch: loop {
        match pc {
            0x83096008 => {
    //   block [0x83096008..0x83096014)
	// 83096008: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309600C: 386B85E0  addi r3, r11, -0x7a20
	ctx.r[3].s64 = ctx.r[11].s64 + -31264;
	// 83096010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83096018 size=88
    let mut pc: u32 = 0x83096018;
    'dispatch: loop {
        match pc {
            0x83096018 => {
    //   block [0x83096018..0x83096070)
	// 83096018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309601C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83096020: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83096024: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83096028: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309602C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83096030: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83096034: 396BD190  addi r11, r11, -0x2e70
	ctx.r[11].s64 = ctx.r[11].s64 + -11888;
	// 83096038: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309603C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83096040: 4BF42E39  bl 0x82fd8e78
	ctx.lr = 0x83096044;
	sub_82FD8E78(ctx, base);
	// 83096044: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83096048: 4182000C  beq 0x83096054
	if ctx.cr[0].eq {
	pc = 0x83096054; continue 'dispatch;
	}
	// 8309604C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83096050: 4BF42291  bl 0x82fd82e0
	ctx.lr = 0x83096054;
	sub_82FD82E0(ctx, base);
	// 83096054: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83096058: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309605C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83096060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83096064: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83096068: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309606C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83096070 size=8
    let mut pc: u32 = 0x83096070;
    'dispatch: loop {
        match pc {
            0x83096070 => {
    //   block [0x83096070..0x83096078)
	// 83096070: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83096074: 8216D268  lwz r16, -0x2d98(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-11672 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83096078 size=108
    let mut pc: u32 = 0x83096078;
    'dispatch: loop {
        match pc {
            0x83096078 => {
    //   block [0x83096078..0x830960E4)
	// 83096078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309607C: 481120F1  bl 0x831a816c
	ctx.lr = 0x83096080;
	sub_831A8130(ctx, base);
	// 83096080: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83096084: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83096088: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309608C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83096090: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83096094: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83096098: 93BF00A4  stw r29, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[29].u32 ) };
	// 8309609C: 3D408217  lis r10, -0x7de9
	ctx.r[10].s64 = -2112421888;
	// 830960A0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830960A4: B17E0004  sth r11, 4(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 830960A8: 394AD248  addi r10, r10, -0x2db8
	ctx.r[10].s64 = ctx.r[10].s64 + -11704;
	// 830960AC: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 830960B0: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830960B4: 4BF421E5  bl 0x82fd8298
	ctx.lr = 0x830960B8;
	sub_82FD8298(ctx, base);
	// 830960B8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830960BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830960C0: 41820010  beq 0x830960d0
	if ctx.cr[0].eq {
	pc = 0x830960D0; continue 'dispatch;
	}
	// 830960C4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830960C8: 4BF480E9  bl 0x82fde1b0
	ctx.lr = 0x830960CC;
	sub_82FDE1B0(ctx, base);
	// 830960CC: 48000008  b 0x830960d4
	pc = 0x830960D4; continue 'dispatch;
	// 830960D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830960D4: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 830960D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830960DC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830960E0: 481120DC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830960E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830960E4 size=40
    let mut pc: u32 = 0x830960E4;
    'dispatch: loop {
        match pc {
            0x830960E4 => {
    //   block [0x830960E4..0x8309610C)
	// 830960E4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830960E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830960EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830960F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830960F4: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830960F8: 4BFB6669  bl 0x8304c760
	ctx.lr = 0x830960FC;
	sub_8304C760(ctx, base);
	// 830960FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83096100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83096104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83096108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309610C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309610C size=44
    let mut pc: u32 = 0x8309610C;
    'dispatch: loop {
        match pc {
            0x8309610C => {
    //   block [0x8309610C..0x83096138)
	// 8309610C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83096110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83096114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83096118: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309611C: 809F00A4  lwz r4, 0xa4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83096120: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83096124: 4BF421BD  bl 0x82fd82e0
	ctx.lr = 0x83096128;
	sub_82FD82E0(ctx, base);
	// 83096128: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309612C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83096130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83096134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83096138 size=8
    let mut pc: u32 = 0x83096138;
    'dispatch: loop {
        match pc {
            0x83096138 => {
    //   block [0x83096138..0x83096140)
	// 83096138: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309613C: 8216D2B0  lwz r16, -0x2d50(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-11600 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83096140 size=112
    let mut pc: u32 = 0x83096140;
    'dispatch: loop {
        match pc {
            0x83096140 => {
    //   block [0x83096140..0x830961B0)
	// 83096140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83096144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83096148: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309614C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83096150: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83096154: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83096158: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309615C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83096160: 396BD248  addi r11, r11, -0x2db8
	ctx.r[11].s64 = ctx.r[11].s64 + -11704;
	// 83096164: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83096168: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309616C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83096170: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83096174: 41820018  beq 0x8309618c
	if ctx.cr[0].eq {
	pc = 0x8309618C; continue 'dispatch;
	}
	// 83096178: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309617C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83096180: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83096184: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83096188: 4E800421  bctrl
	ctx.lr = 0x8309618C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309618C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83096190: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 83096194: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83096198: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8309619C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830961A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830961A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830961A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830961AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830961B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830961B0 size=40
    let mut pc: u32 = 0x830961B0;
    'dispatch: loop {
        match pc {
            0x830961B0 => {
    //   block [0x830961B0..0x830961D8)
	// 830961B0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830961B4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830961B8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830961BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830961C0: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830961C4: 4BFB659D  bl 0x8304c760
	ctx.lr = 0x830961C8;
	sub_8304C760(ctx, base);
	// 830961C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830961CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830961D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830961D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830961D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830961D8 size=76
    let mut pc: u32 = 0x830961D8;
    'dispatch: loop {
        match pc {
            0x830961D8 => {
    //   block [0x830961D8..0x83096224)
	// 830961D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830961DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830961E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830961E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830961E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830961EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830961F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830961F4: 4BFFFF4D  bl 0x83096140
	ctx.lr = 0x830961F8;
	sub_83096140(ctx, base);
	// 830961F8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830961FC: 4182000C  beq 0x83096208
	if ctx.cr[0].eq {
	pc = 0x83096208; continue 'dispatch;
	}
	// 83096200: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83096204: 4BF420DD  bl 0x82fd82e0
	ctx.lr = 0x83096208;
	sub_82FD82E0(ctx, base);
	// 83096208: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309620C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83096210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83096214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83096218: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309621C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83096220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83096228 size=8
    let mut pc: u32 = 0x83096228;
    'dispatch: loop {
        match pc {
            0x83096228 => {
    //   block [0x83096228..0x83096230)
	// 83096228: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309622C: 8216D2F0  lwz r16, -0x2d10(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-11536 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83096230 size=108
    let mut pc: u32 = 0x83096230;
    'dispatch: loop {
        match pc {
            0x83096230 => {
    //   block [0x83096230..0x8309629C)
	// 83096230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83096234: 48111F39  bl 0x831a816c
	ctx.lr = 0x83096238;
	sub_831A8130(ctx, base);
	// 83096238: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8309623C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83096240: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83096244: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83096248: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8309624C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83096250: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83096254: 396BD248  addi r11, r11, -0x2db8
	ctx.r[11].s64 = ctx.r[11].s64 + -11704;
	// 83096258: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8309625C: B15E0004  sth r10, 4(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 83096260: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83096264: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83096268: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 8309626C: 4BF4202D  bl 0x82fd8298
	ctx.lr = 0x83096270;
	sub_82FD8298(ctx, base);
	// 83096270: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83096274: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83096278: 41820010  beq 0x83096288
	if ctx.cr[0].eq {
	pc = 0x83096288; continue 'dispatch;
	}
	// 8309627C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83096280: 4BF47F71  bl 0x82fde1f0
	ctx.lr = 0x83096284;
	sub_82FDE1F0(ctx, base);
	// 83096284: 48000008  b 0x8309628c
	pc = 0x8309628C; continue 'dispatch;
	// 83096288: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309628C: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 83096290: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83096294: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83096298: 48111F24  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309629C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309629C size=40
    let mut pc: u32 = 0x8309629C;
    'dispatch: loop {
        match pc {
            0x8309629C => {
    //   block [0x8309629C..0x830962C4)
	// 8309629C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830962A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830962A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830962A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830962AC: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830962B0: 4BFB64B1  bl 0x8304c760
	ctx.lr = 0x830962B4;
	sub_8304C760(ctx, base);
	// 830962B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830962B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830962BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830962C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830962C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830962C4 size=44
    let mut pc: u32 = 0x830962C4;
    'dispatch: loop {
        match pc {
            0x830962C4 => {
    //   block [0x830962C4..0x830962F0)
	// 830962C4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830962C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830962CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830962D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830962D4: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830962D8: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830962DC: 4BF42005  bl 0x82fd82e0
	ctx.lr = 0x830962E0;
	sub_82FD82E0(ctx, base);
	// 830962E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830962E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830962E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830962EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830962F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830962F0 size=8
    let mut pc: u32 = 0x830962F0;
    'dispatch: loop {
        match pc {
            0x830962F0 => {
    //   block [0x830962F0..0x830962F8)
	// 830962F0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830962F4: 8216D340  lwz r16, -0x2cc0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-11456 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830962F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830962F8 size=132
    let mut pc: u32 = 0x830962F8;
    'dispatch: loop {
        match pc {
            0x830962F8 => {
    //   block [0x830962F8..0x8309637C)
	// 830962F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830962FC: 48111E69  bl 0x831a8164
	ctx.lr = 0x83096300;
	sub_831A8130(ctx, base);
	// 83096300: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83096304: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83096308: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309630C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83096310: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83096314: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83096318: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 8309631C: 93BF00BC  stw r29, 0xbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[29].u32 ) };
	// 83096320: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83096324: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 83096328: 396BD248  addi r11, r11, -0x2db8
	ctx.r[11].s64 = ctx.r[11].s64 + -11704;
	// 8309632C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83096330: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 83096334: B15E0004  sth r10, 4(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 83096338: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309633C: 4BF41F5D  bl 0x82fd8298
	ctx.lr = 0x83096340;
	sub_82FD8298(ctx, base);
	// 83096340: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83096344: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83096348: 41820010  beq 0x83096358
	if ctx.cr[0].eq {
	pc = 0x83096358; continue 'dispatch;
	}
	// 8309634C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83096350: 4BF47E61  bl 0x82fde1b0
	ctx.lr = 0x83096354;
	sub_82FDE1B0(ctx, base);
	// 83096354: 48000008  b 0x8309635c
	pc = 0x8309635C; continue 'dispatch;
	// 83096358: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309635C: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 83096360: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83096364: 93630020  stw r27, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[27].u32 ) };
	// 83096368: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309636C: 4BF48135  bl 0x82fde4a0
	ctx.lr = 0x83096370;
	sub_82FDE4A0(ctx, base);
	// 83096370: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83096374: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83096378: 48111E3C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309637C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309637C size=40
    let mut pc: u32 = 0x8309637C;
    'dispatch: loop {
        match pc {
            0x8309637C => {
    //   block [0x8309637C..0x830963A4)
	// 8309637C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83096380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83096384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83096388: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309638C: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83096390: 4BFB63D1  bl 0x8304c760
	ctx.lr = 0x83096394;
	sub_8304C760(ctx, base);
	// 83096394: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83096398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309639C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830963A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830963A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830963A4 size=44
    let mut pc: u32 = 0x830963A4;
    'dispatch: loop {
        match pc {
            0x830963A4 => {
    //   block [0x830963A4..0x830963D0)
	// 830963A4: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830963A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830963AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830963B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830963B4: 809F00BC  lwz r4, 0xbc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 830963B8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830963BC: 4BF41F25  bl 0x82fd82e0
	ctx.lr = 0x830963C0;
	sub_82FD82E0(ctx, base);
	// 830963C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830963C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830963C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830963CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830963D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830963D0 size=12
    let mut pc: u32 = 0x830963D0;
    'dispatch: loop {
        match pc {
            0x830963D0 => {
    //   block [0x830963D0..0x830963DC)
	// 830963D0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830963D4: 386B35A8  addi r3, r11, 0x35a8
	ctx.r[3].s64 = ctx.r[11].s64 + 13736;
	// 830963D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830963E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830963E0 size=124
    let mut pc: u32 = 0x830963E0;
    'dispatch: loop {
        match pc {
            0x830963E0 => {
    //   block [0x830963E0..0x8309645C)
	// 830963E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830963E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830963E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830963EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830963F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830963F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830963F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830963FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83096400: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 83096404: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83096408: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309640C: 4182001C  beq 0x83096428
	if ctx.cr[0].eq {
	pc = 0x83096428; continue 'dispatch;
	}
	// 83096410: A09E0004  lhz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83096414: 4BF62E55  bl 0x82ff9268
	ctx.lr = 0x83096418;
	sub_82FF9268(ctx, base);
	// 83096418: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309641C: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83096420: 4BF637E1  bl 0x82ff9c00
	ctx.lr = 0x83096424;
	sub_82FF9C00(ctx, base);
	// 83096424: 48000020  b 0x83096444
	pc = 0x83096444; continue 'dispatch;
	// 83096428: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 8309642C: 4BF630BD  bl 0x82ff94e8
	ctx.lr = 0x83096430;
	sub_82FF94E8(ctx, base);
	// 83096430: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83096434: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83096438: 388B2E10  addi r4, r11, 0x2e10
	ctx.r[4].s64 = ctx.r[11].s64 + 11792;
	// 8309643C: 4BF63885  bl 0x82ff9cc0
	ctx.lr = 0x83096440;
	sub_82FF9CC0(ctx, base);
	// 83096440: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 83096444: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83096448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309644C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83096450: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83096454: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83096458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83096460 size=8
    let mut pc: u32 = 0x83096460;
    'dispatch: loop {
        match pc {
            0x83096460 => {
    //   block [0x83096460..0x83096468)
	// 83096460: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83096464: 8216D398  lwz r16, -0x2c68(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-11368 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83096468 size=112
    let mut pc: u32 = 0x83096468;
    'dispatch: loop {
        match pc {
            0x83096468 => {
    //   block [0x83096468..0x830964D8)
	// 83096468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309646C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83096470: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83096474: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83096478: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8309647C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83096480: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83096484: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83096488: 396BD380  addi r11, r11, -0x2c80
	ctx.r[11].s64 = ctx.r[11].s64 + -11392;
	// 8309648C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83096490: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83096494: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83096498: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309649C: 41820018  beq 0x830964b4
	if ctx.cr[0].eq {
	pc = 0x830964B4; continue 'dispatch;
	}
	// 830964A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830964A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830964A8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830964AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830964B0: 4E800421  bctrl
	ctx.lr = 0x830964B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830964B4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830964B8: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 830964BC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830964C0: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830964C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830964C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830964CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830964D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830964D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830964D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830964D8 size=40
    let mut pc: u32 = 0x830964D8;
    'dispatch: loop {
        match pc {
            0x830964D8 => {
    //   block [0x830964D8..0x83096500)
	// 830964D8: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830964DC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830964E0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830964E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830964E8: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830964EC: 4BFB6275  bl 0x8304c760
	ctx.lr = 0x830964F0;
	sub_8304C760(ctx, base);
	// 830964F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830964F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830964F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830964FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83096500 size=76
    let mut pc: u32 = 0x83096500;
    'dispatch: loop {
        match pc {
            0x83096500 => {
    //   block [0x83096500..0x8309654C)
	// 83096500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83096504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83096508: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309650C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83096510: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83096514: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83096518: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309651C: 4BFFFF4D  bl 0x83096468
	ctx.lr = 0x83096520;
	sub_83096468(ctx, base);
	// 83096520: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83096524: 4182000C  beq 0x83096530
	if ctx.cr[0].eq {
	pc = 0x83096530; continue 'dispatch;
	}
	// 83096528: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309652C: 4BF41DB5  bl 0x82fd82e0
	ctx.lr = 0x83096530;
	sub_82FD82E0(ctx, base);
	// 83096530: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83096534: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83096538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309653C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83096540: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83096544: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83096548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83096550 size=16
    let mut pc: u32 = 0x83096550;
    'dispatch: loop {
        match pc {
            0x83096550 => {
    //   block [0x83096550..0x83096560)
	// 83096550: 7F032040  cmplw cr6, r3, r4
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[4].u32, &mut ctx.xer);
	// 83096554: 409A000C  bne cr6, 0x83096560
	if !ctx.cr[6].eq {
		sub_83096560(ctx, base);
		return;
	}
	// 83096558: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8309655C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83096560 size=24
    let mut pc: u32 = 0x83096560;
    'dispatch: loop {
        match pc {
            0x83096560 => {
    //   block [0x83096560..0x83096578)
	// 83096560: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83096564: A1440004  lhz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 83096568: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8309656C: 419A000C  beq cr6, 0x83096578
	if ctx.cr[6].eq {
		sub_83096578(ctx, base);
		return;
	}
	// 83096570: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83096574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83096578 size=60
    let mut pc: u32 = 0x83096578;
    'dispatch: loop {
        match pc {
            0x83096578 => {
    //   block [0x83096578..0x830965B4)
	// 83096578: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8309657C: 419A000C  beq cr6, 0x83096588
	if ctx.cr[6].eq {
	pc = 0x83096588; continue 'dispatch;
	}
	// 83096580: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 83096584: 409AFFD4  bne cr6, 0x83096558
	if !ctx.cr[6].eq {
		sub_83096550(ctx, base);
		return;
	}
	// 83096588: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309658C: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83096590: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83096594: 419AFFC4  beq cr6, 0x83096558
	if ctx.cr[6].eq {
		sub_83096550(ctx, base);
		return;
	}
	// 83096598: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309659C: A10A0004  lhz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830965A0: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830965A4: 409AFFCC  bne cr6, 0x83096570
	if !ctx.cr[6].eq {
		sub_83096560(ctx, base);
		return;
	}
	// 830965A8: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830965AC: 806A0008  lwz r3, 8(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 830965B0: 4BF481F0  b 0x82fde7a0
	sub_82FDE7A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830965B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830965B8 size=12
    let mut pc: u32 = 0x830965B8;
    'dispatch: loop {
        match pc {
            0x830965B8 => {
    //   block [0x830965B8..0x830965C4)
	// 830965B8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830965BC: 386B35B0  addi r3, r11, 0x35b0
	ctx.r[3].s64 = ctx.r[11].s64 + 13744;
	// 830965C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830965C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830965C8 size=124
    let mut pc: u32 = 0x830965C8;
    'dispatch: loop {
        match pc {
            0x830965C8 => {
    //   block [0x830965C8..0x83096644)
	// 830965C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830965CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830965D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830965D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830965D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830965DC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830965E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830965E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830965E8: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 830965EC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830965F0: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830965F4: 4182001C  beq 0x83096610
	if ctx.cr[0].eq {
	pc = 0x83096610; continue 'dispatch;
	}
	// 830965F8: A09E0004  lhz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830965FC: 4BF62B9D  bl 0x82ff9198
	ctx.lr = 0x83096600;
	sub_82FF9198(ctx, base);
	// 83096600: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83096604: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83096608: 4BF635F9  bl 0x82ff9c00
	ctx.lr = 0x8309660C;
	sub_82FF9C00(ctx, base);
	// 8309660C: 48000020  b 0x8309662c
	pc = 0x8309662C; continue 'dispatch;
	// 83096610: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 83096614: 4BF62E05  bl 0x82ff9418
	ctx.lr = 0x83096618;
	sub_82FF9418(ctx, base);
	// 83096618: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8309661C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83096620: 388B35A8  addi r4, r11, 0x35a8
	ctx.r[4].s64 = ctx.r[11].s64 + 13736;
	// 83096624: 4BF6369D  bl 0x82ff9cc0
	ctx.lr = 0x83096628;
	sub_82FF9CC0(ctx, base);
	// 83096628: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8309662C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83096630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83096634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83096638: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309663C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83096640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83096648 size=8
    let mut pc: u32 = 0x83096648;
    'dispatch: loop {
        match pc {
            0x83096648 => {
    //   block [0x83096648..0x83096650)
	// 83096648: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309664C: 8216D3E0  lwz r16, -0x2c20(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-11296 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83096650 size=112
    let mut pc: u32 = 0x83096650;
    'dispatch: loop {
        match pc {
            0x83096650 => {
    //   block [0x83096650..0x830966C0)
	// 83096650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83096654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83096658: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309665C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83096660: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83096664: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83096668: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309666C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83096670: 396BD3C8  addi r11, r11, -0x2c38
	ctx.r[11].s64 = ctx.r[11].s64 + -11320;
	// 83096674: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83096678: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309667C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83096680: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83096684: 41820018  beq 0x8309669c
	if ctx.cr[0].eq {
	pc = 0x8309669C; continue 'dispatch;
	}
	// 83096688: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309668C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83096690: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83096694: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83096698: 4E800421  bctrl
	ctx.lr = 0x8309669C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309669C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830966A0: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 830966A4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830966A8: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830966AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830966B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830966B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830966B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830966BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830966C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830966C0 size=40
    let mut pc: u32 = 0x830966C0;
    'dispatch: loop {
        match pc {
            0x830966C0 => {
    //   block [0x830966C0..0x830966E8)
	// 830966C0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830966C4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830966C8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830966CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830966D0: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830966D4: 4BFB608D  bl 0x8304c760
	ctx.lr = 0x830966D8;
	sub_8304C760(ctx, base);
	// 830966D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830966DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830966E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830966E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830966E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830966E8 size=76
    let mut pc: u32 = 0x830966E8;
    'dispatch: loop {
        match pc {
            0x830966E8 => {
    //   block [0x830966E8..0x83096734)
	// 830966E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830966EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830966F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830966F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830966F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830966FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83096700: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83096704: 4BFFFF4D  bl 0x83096650
	ctx.lr = 0x83096708;
	sub_83096650(ctx, base);
	// 83096708: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309670C: 4182000C  beq 0x83096718
	if ctx.cr[0].eq {
	pc = 0x83096718; continue 'dispatch;
	}
	// 83096710: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83096714: 4BF41BCD  bl 0x82fd82e0
	ctx.lr = 0x83096718;
	sub_82FD82E0(ctx, base);
	// 83096718: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309671C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83096720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83096724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83096728: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309672C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83096730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83096738 size=12
    let mut pc: u32 = 0x83096738;
    'dispatch: loop {
        match pc {
            0x83096738 => {
    //   block [0x83096738..0x83096744)
	// 83096738: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8309673C: 386B35B8  addi r3, r11, 0x35b8
	ctx.r[3].s64 = ctx.r[11].s64 + 13752;
	// 83096740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83096748 size=24
    let mut pc: u32 = 0x83096748;
    'dispatch: loop {
        match pc {
            0x83096748 => {
    //   block [0x83096748..0x83096760)
	// 83096748: A9640000  lha r11, 0(r4)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 8309674C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83096750: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83096754: 4182000C  beq 0x83096760
	if ctx.cr[0].eq {
		sub_83096760(ctx, base);
		return;
	}
	// 83096758: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309675C: 4BFB63DC  b 0x8304cb38
	sub_8304CB38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83096760 size=20
    let mut pc: u32 = 0x83096760;
    'dispatch: loop {
        match pc {
            0x83096760 => {
    //   block [0x83096760..0x83096774)
	// 83096760: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 83096764: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83096768: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 8309676C: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 83096770: 4BFB7820  b 0x8304df90
	sub_8304DF90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83096778 size=96
    let mut pc: u32 = 0x83096778;
    'dispatch: loop {
        match pc {
            0x83096778 => {
    //   block [0x83096778..0x830967D8)
	// 83096778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309677C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83096780: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83096784: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83096788: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309678C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83096790: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83096794: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83096798: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309679C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830967A0: 4E800421  bctrl
	ctx.lr = 0x830967A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830967A4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830967A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830967AC: 41820018  beq 0x830967c4
	if ctx.cr[0].eq {
	pc = 0x830967C4; continue 'dispatch;
	}
	// 830967B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830967B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830967B8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830967BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830967C0: 4E800421  bctrl
	ctx.lr = 0x830967C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830967C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830967C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830967CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830967D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830967D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830967D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830967D8 size=12
    let mut pc: u32 = 0x830967D8;
    'dispatch: loop {
        match pc {
            0x830967D8 => {
    //   block [0x830967D8..0x830967E4)
	// 830967D8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830967DC: 386B35C0  addi r3, r11, 0x35c0
	ctx.r[3].s64 = ctx.r[11].s64 + 13760;
	// 830967E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830967E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830967E8 size=168
    let mut pc: u32 = 0x830967E8;
    'dispatch: loop {
        match pc {
            0x830967E8 => {
    //   block [0x830967E8..0x83096890)
	// 830967E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830967EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830967F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830967F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830967F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830967FC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83096800: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83096804: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83096808: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 8309680C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83096810: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83096814: 41820030  beq 0x83096844
	if ctx.cr[0].eq {
	pc = 0x83096844; continue 'dispatch;
	}
	// 83096818: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309681C: 4BF62ADD  bl 0x82ff92f8
	ctx.lr = 0x83096820;
	sub_82FF92F8(ctx, base);
	// 83096820: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83096824: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83096828: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8309682C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83096830: 4BF630D1  bl 0x82ff9900
	ctx.lr = 0x83096834;
	sub_82FF9900(ctx, base);
	// 83096834: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83096838: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309683C: 4BFB62FD  bl 0x8304cb38
	ctx.lr = 0x83096840;
	sub_8304CB38(ctx, base);
	// 83096840: 48000038  b 0x83096878
	pc = 0x83096878; continue 'dispatch;
	// 83096844: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 83096848: 4BF62D31  bl 0x82ff9578
	ctx.lr = 0x8309684C;
	sub_82FF9578(ctx, base);
	// 8309684C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83096850: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83096854: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 83096858: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 8309685C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83096860: 4BF632C9  bl 0x82ff9b28
	ctx.lr = 0x83096864;
	sub_82FF9B28(ctx, base);
	// 83096864: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83096868: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8309686C: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 83096870: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 83096874: 4BFB75D5  bl 0x8304de48
	ctx.lr = 0x83096878;
	sub_8304DE48(ctx, base);
	// 83096878: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309687C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83096880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83096884: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83096888: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309688C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83096890 size=788
    let mut pc: u32 = 0x83096890;
    'dispatch: loop {
        match pc {
            0x83096890 => {
    //   block [0x83096890..0x83096BA4)
	// 83096890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83096894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83096898: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309689C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830968A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830968A4: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830968A8: 388B06B0  addi r4, r11, 0x6b0
	ctx.r[4].s64 = ctx.r[11].s64 + 1712;
	// 830968AC: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 830968B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830968B4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830968B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830968BC: 4E800421  bctrl
	ctx.lr = 0x830968C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830968C0: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 830968C4: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830968C8: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 830968CC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830968D0: 388A06B8  addi r4, r10, 0x6b8
	ctx.r[4].s64 = ctx.r[10].s64 + 1720;
	// 830968D4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830968D8: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 830968DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830968E0: 4E800421  bctrl
	ctx.lr = 0x830968E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830968E4: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 830968E8: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830968EC: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 830968F0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830968F4: 388A06C0  addi r4, r10, 0x6c0
	ctx.r[4].s64 = ctx.r[10].s64 + 1728;
	// 830968F8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830968FC: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83096900: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83096904: 4E800421  bctrl
	ctx.lr = 0x83096908;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83096908: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8309690C: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 83096910: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83096914: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83096918: 388A06C8  addi r4, r10, 0x6c8
	ctx.r[4].s64 = ctx.r[10].s64 + 1736;
	// 8309691C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83096920: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83096924: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83096928: 4E800421  bctrl
	ctx.lr = 0x8309692C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309692C: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83096930: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 83096934: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83096938: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8309693C: 388A06D0  addi r4, r10, 0x6d0
	ctx.r[4].s64 = ctx.r[10].s64 + 1744;
	// 83096940: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83096944: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83096948: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309694C: 4E800421  bctrl
	ctx.lr = 0x83096950;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83096950: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83096954: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 83096958: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 8309695C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83096960: 388A06E0  addi r4, r10, 0x6e0
	ctx.r[4].s64 = ctx.r[10].s64 + 1760;
	// 83096964: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83096968: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309696C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83096970: 4E800421  bctrl
	ctx.lr = 0x83096974;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83096974: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83096978: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 8309697C: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 83096980: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83096984: 388A06EC  addi r4, r10, 0x6ec
	ctx.r[4].s64 = ctx.r[10].s64 + 1772;
	// 83096988: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309698C: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83096990: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83096994: 4E800421  bctrl
	ctx.lr = 0x83096998;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83096998: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8309699C: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830969A0: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 830969A4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830969A8: 388A071C  addi r4, r10, 0x71c
	ctx.r[4].s64 = ctx.r[10].s64 + 1820;
	// 830969AC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830969B0: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 830969B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830969B8: 4E800421  bctrl
	ctx.lr = 0x830969BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830969BC: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 830969C0: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 830969C4: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830969C8: 388B0728  addi r4, r11, 0x728
	ctx.r[4].s64 = ctx.r[11].s64 + 1832;
	// 830969CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830969D0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830969D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830969D8: 4E800421  bctrl
	ctx.lr = 0x830969DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830969DC: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 830969E0: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830969E4: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 830969E8: 388A073C  addi r4, r10, 0x73c
	ctx.r[4].s64 = ctx.r[10].s64 + 1852;
	// 830969EC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830969F0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830969F4: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 830969F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830969FC: 4E800421  bctrl
	ctx.lr = 0x83096A00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83096A00: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83096A04: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 83096A08: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 83096A0C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83096A10: 388A0760  addi r4, r10, 0x760
	ctx.r[4].s64 = ctx.r[10].s64 + 1888;
	// 83096A14: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83096A18: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83096A1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83096A20: 4E800421  bctrl
	ctx.lr = 0x83096A24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83096A24: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83096A28: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 83096A2C: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 83096A30: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83096A34: 388A0774  addi r4, r10, 0x774
	ctx.r[4].s64 = ctx.r[10].s64 + 1908;
	// 83096A38: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83096A3C: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83096A40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83096A44: 4E800421  bctrl
	ctx.lr = 0x83096A48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83096A48: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83096A4C: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 83096A50: 907F0030  stw r3, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 83096A54: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83096A58: 388A0780  addi r4, r10, 0x780
	ctx.r[4].s64 = ctx.r[10].s64 + 1920;
	// 83096A5C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83096A60: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83096A64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83096A68: 4E800421  bctrl
	ctx.lr = 0x83096A6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83096A6C: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83096A70: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 83096A74: 907F0034  stw r3, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[3].u32 ) };
	// 83096A78: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83096A7C: 388A0798  addi r4, r10, 0x798
	ctx.r[4].s64 = ctx.r[10].s64 + 1944;
	// 83096A80: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83096A84: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83096A88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83096A8C: 4E800421  bctrl
	ctx.lr = 0x83096A90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83096A90: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83096A94: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 83096A98: 907F0038  stw r3, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[3].u32 ) };
	// 83096A9C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83096AA0: 388A07C0  addi r4, r10, 0x7c0
	ctx.r[4].s64 = ctx.r[10].s64 + 1984;
	// 83096AA4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83096AA8: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83096AAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83096AB0: 4E800421  bctrl
	ctx.lr = 0x83096AB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83096AB4: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83096AB8: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 83096ABC: 907F003C  stw r3, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[3].u32 ) };
	// 83096AC0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83096AC4: 388A07D4  addi r4, r10, 0x7d4
	ctx.r[4].s64 = ctx.r[10].s64 + 2004;
	// 83096AC8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83096ACC: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83096AD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83096AD4: 4E800421  bctrl
	ctx.lr = 0x83096AD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83096AD8: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83096ADC: 907F0040  stw r3, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[3].u32 ) };
	// 83096AE0: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 83096AE4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83096AE8: 388A07F8  addi r4, r10, 0x7f8
	ctx.r[4].s64 = ctx.r[10].s64 + 2040;
	// 83096AEC: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83096AF0: 81690010  lwz r11, 0x10(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 83096AF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83096AF8: 4E800421  bctrl
	ctx.lr = 0x83096AFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83096AFC: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83096B00: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 83096B04: 907F0044  stw r3, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[3].u32 ) };
	// 83096B08: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83096B0C: 388A080C  addi r4, r10, 0x80c
	ctx.r[4].s64 = ctx.r[10].s64 + 2060;
	// 83096B10: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83096B14: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83096B18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83096B1C: 4E800421  bctrl
	ctx.lr = 0x83096B20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83096B20: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83096B24: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 83096B28: 907F0048  stw r3, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[3].u32 ) };
	// 83096B2C: 388A081C  addi r4, r10, 0x81c
	ctx.r[4].s64 = ctx.r[10].s64 + 2076;
	// 83096B30: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83096B34: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83096B38: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83096B3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83096B40: 4E800421  bctrl
	ctx.lr = 0x83096B44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83096B44: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83096B48: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 83096B4C: 907F004C  stw r3, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[3].u32 ) };
	// 83096B50: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83096B54: 388A0830  addi r4, r10, 0x830
	ctx.r[4].s64 = ctx.r[10].s64 + 2096;
	// 83096B58: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83096B5C: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83096B60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83096B64: 4E800421  bctrl
	ctx.lr = 0x83096B68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83096B68: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83096B6C: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 83096B70: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83096B74: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83096B78: 388A0854  addi r4, r10, 0x854
	ctx.r[4].s64 = ctx.r[10].s64 + 2132;
	// 83096B7C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83096B80: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83096B84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83096B88: 4E800421  bctrl
	ctx.lr = 0x83096B8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83096B8C: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83096B90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83096B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83096B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83096B9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83096BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83096BA8 size=44
    let mut pc: u32 = 0x83096BA8;
    'dispatch: loop {
        match pc {
            0x83096BA8 => {
    //   block [0x83096BA8..0x83096BD4)
	// 83096BA8: 54CA083C  slwi r10, r6, 1
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83096BAC: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83096BB0: 392B2DD8  addi r9, r11, 0x2dd8
	ctx.r[9].s64 = ctx.r[11].s64 + 11736;
	// 83096BB4: 7D6A222E  lhzx r11, r10, r4
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 83096BB8: 7D4B48AE  lbzx r10, r11, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 83096BBC: 554A07FF  clrlwi. r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83096BC0: 40820014  bne 0x83096bd4
	if !ctx.cr[0].eq {
		sub_83096BD4(ctx, base);
		return;
	}
	// 83096BC4: 2B0B005F  cmplwi cr6, r11, 0x5f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 95 as u32, &mut ctx.xer);
	// 83096BC8: 419A000C  beq cr6, 0x83096bd4
	if ctx.cr[6].eq {
		sub_83096BD4(ctx, base);
		return;
	}
	// 83096BCC: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 83096BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096BD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83096BD4 size=12
    let mut pc: u32 = 0x83096BD4;
    'dispatch: loop {
        match pc {
            0x83096BD4 => {
    //   block [0x83096BD4..0x83096BE0)
	// 83096BD4: 38660001  addi r3, r6, 1
	ctx.r[3].s64 = ctx.r[6].s64 + 1;
	// 83096BD8: 7F032800  cmpw cr6, r3, r5
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[5].s32, &mut ctx.xer);
	// 83096BDC: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83096BE0 size=20
    let mut pc: u32 = 0x83096BE0;
    'dispatch: loop {
        match pc {
            0x83096BE0 => {
    //   block [0x83096BE0..0x83096BF4)
	// 83096BE0: 546B083C  slwi r11, r3, 1
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83096BE4: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 83096BE8: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83096BEC: 2B0A003A  cmplwi cr6, r10, 0x3a
	ctx.cr[6].compare_u32(ctx.r[10].u32, 58 as u32, &mut ctx.xer);
	// 83096BF0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096BF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83096BF4 size=12
    let mut pc: u32 = 0x83096BF4;
    'dispatch: loop {
        match pc {
            0x83096BF4 => {
    //   block [0x83096BF4..0x83096C00)
	// 83096BF4: 7D4A48AE  lbzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 83096BF8: 554A077B  rlwinm. r10, r10, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83096BFC: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83096C00 size=20
    let mut pc: u32 = 0x83096C00;
    'dispatch: loop {
        match pc {
            0x83096C00 => {
    //   block [0x83096C00..0x83096C14)
	// 83096C00: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 83096C04: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83096C08: 7F032800  cmpw cr6, r3, r5
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[5].s32, &mut ctx.xer);
	// 83096C0C: 4198FFDC  blt cr6, 0x83096be8
	if ctx.cr[6].lt {
		sub_83096BE0(ctx, base);
		return;
	}
	// 83096C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83096C18 size=28
    let mut pc: u32 = 0x83096C18;
    'dispatch: loop {
        match pc {
            0x83096C18 => {
    //   block [0x83096C18..0x83096C34)
	// 83096C18: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83096C1C: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83096C20: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83096C24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83096C28: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83096C2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83096C30: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83096C38 size=84
    let mut pc: u32 = 0x83096C38;
    'dispatch: loop {
        match pc {
            0x83096C38 => {
    //   block [0x83096C38..0x83096C8C)
	// 83096C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83096C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83096C40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83096C44: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83096C48: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 83096C4C: 4BF4164D  bl 0x82fd8298
	ctx.lr = 0x83096C50;
	sub_82FD8298(ctx, base);
	// 83096C50: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83096C54: 41820024  beq 0x83096c78
	if ctx.cr[0].eq {
	pc = 0x83096C78; continue 'dispatch;
	}
	// 83096C58: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83096C5C: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 83096C60: 396BD248  addi r11, r11, -0x2db8
	ctx.r[11].s64 = ctx.r[11].s64 + -11704;
	// 83096C64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83096C68: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 83096C6C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83096C70: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 83096C74: 48000008  b 0x83096c7c
	pc = 0x83096C7C; continue 'dispatch;
	// 83096C78: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83096C7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83096C80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83096C84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83096C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83096C90 size=84
    let mut pc: u32 = 0x83096C90;
    'dispatch: loop {
        match pc {
            0x83096C90 => {
    //   block [0x83096C90..0x83096CE4)
	// 83096C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83096C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83096C98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83096C9C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83096CA0: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 83096CA4: 4BF415F5  bl 0x82fd8298
	ctx.lr = 0x83096CA8;
	sub_82FD8298(ctx, base);
	// 83096CA8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83096CAC: 41820024  beq 0x83096cd0
	if ctx.cr[0].eq {
	pc = 0x83096CD0; continue 'dispatch;
	}
	// 83096CB0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83096CB4: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 83096CB8: 396BD380  addi r11, r11, -0x2c80
	ctx.r[11].s64 = ctx.r[11].s64 + -11392;
	// 83096CBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83096CC0: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 83096CC4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83096CC8: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 83096CCC: 48000008  b 0x83096cd4
	pc = 0x83096CD4; continue 'dispatch;
	// 83096CD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83096CD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83096CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83096CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83096CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83096CE8 size=140
    let mut pc: u32 = 0x83096CE8;
    'dispatch: loop {
        match pc {
            0x83096CE8 => {
    //   block [0x83096CE8..0x83096D74)
	// 83096CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83096CEC: 48111479  bl 0x831a8164
	ctx.lr = 0x83096CF0;
	sub_831A8130(ctx, base);
	// 83096CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83096CF4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83096CF8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83096CFC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83096D00: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83096D04: 836B0008  lwz r27, 8(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83096D08: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 83096D0C: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83096D10: 419A000C  beq cr6, 0x83096d1c
	if ctx.cr[6].eq {
	pc = 0x83096D1C; continue 'dispatch;
	}
	// 83096D14: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83096D18: 48000054  b 0x83096d6c
	pc = 0x83096D6C; continue 'dispatch;
	// 83096D1C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83096D20: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 83096D24: 419A0044  beq cr6, 0x83096d68
	if ctx.cr[6].eq {
	pc = 0x83096D68; continue 'dispatch;
	}
	// 83096D28: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83096D2C: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83096D30: 4BF95B41  bl 0x8302c870
	ctx.lr = 0x83096D34;
	sub_8302C870(ctx, base);
	// 83096D34: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83096D38: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83096D3C: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83096D40: 4BF95B31  bl 0x8302c870
	ctx.lr = 0x83096D44;
	sub_8302C870(ctx, base);
	// 83096D44: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83096D48: 4BFFF809  bl 0x83096550
	ctx.lr = 0x83096D4C;
	sub_83096550(ctx, base);
	// 83096D4C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 83096D50: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83096D54: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83096D58: 4082FFBC  bne 0x83096d14
	if !ctx.cr[0].eq {
	pc = 0x83096D14; continue 'dispatch;
	}
	// 83096D5C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83096D60: 7F1FD840  cmplw cr6, r31, r27
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[27].u32, &mut ctx.xer);
	// 83096D64: 4198FFC4  blt cr6, 0x83096d28
	if ctx.cr[6].lt {
	pc = 0x83096D28; continue 'dispatch;
	}
	// 83096D68: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83096D6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83096D70: 48111444  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83096D78 size=76
    let mut pc: u32 = 0x83096D78;
    'dispatch: loop {
        match pc {
            0x83096D78 => {
    //   block [0x83096D78..0x83096DC4)
	// 83096D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83096D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83096D80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83096D84: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83096D88: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 83096D8C: 4BF4150D  bl 0x82fd8298
	ctx.lr = 0x83096D90;
	sub_82FD8298(ctx, base);
	// 83096D90: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83096D94: 4182001C  beq 0x83096db0
	if ctx.cr[0].eq {
	pc = 0x83096DB0; continue 'dispatch;
	}
	// 83096D98: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83096D9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83096DA0: 396BD3C8  addi r11, r11, -0x2c38
	ctx.r[11].s64 = ctx.r[11].s64 + -11320;
	// 83096DA4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83096DA8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83096DAC: 48000008  b 0x83096db4
	pc = 0x83096DB4; continue 'dispatch;
	// 83096DB0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83096DB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83096DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83096DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83096DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83096DC8 size=8
    let mut pc: u32 = 0x83096DC8;
    'dispatch: loop {
        match pc {
            0x83096DC8 => {
    //   block [0x83096DC8..0x83096DD0)
	// 83096DC8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83096DCC: 8216D428  lwz r16, -0x2bd8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-11224 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83096DD0 size=84
    let mut pc: u32 = 0x83096DD0;
    'dispatch: loop {
        match pc {
            0x83096DD0 => {
    //   block [0x83096DD0..0x83096E24)
	// 83096DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83096DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83096DD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83096DDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83096DE0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83096DE4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83096DE8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83096DEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83096DF0: 396BD410  addi r11, r11, -0x2bf0
	ctx.r[11].s64 = ctx.r[11].s64 + -11248;
	// 83096DF4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83096DF8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83096DFC: 4BFFF97D  bl 0x83096778
	ctx.lr = 0x83096E00;
	sub_83096778(ctx, base);
	// 83096E00: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83096E04: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 83096E08: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83096E0C: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83096E10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83096E14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83096E18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83096E1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83096E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096E24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83096E24 size=40
    let mut pc: u32 = 0x83096E24;
    'dispatch: loop {
        match pc {
            0x83096E24 => {
    //   block [0x83096E24..0x83096E4C)
	// 83096E24: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83096E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83096E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83096E30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83096E34: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83096E38: 4BFB5929  bl 0x8304c760
	ctx.lr = 0x83096E3C;
	sub_8304C760(ctx, base);
	// 83096E3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83096E40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83096E44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83096E48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83096E50 size=140
    let mut pc: u32 = 0x83096E50;
    'dispatch: loop {
        match pc {
            0x83096E50 => {
    //   block [0x83096E50..0x83096EDC)
	// 83096E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83096E54: 48111311  bl 0x831a8164
	ctx.lr = 0x83096E58;
	sub_831A8130(ctx, base);
	// 83096E58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83096E5C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83096E60: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83096E64: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 83096E68: 815C000C  lwz r10, 0xc(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 83096E6C: 836B0008  lwz r27, 8(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83096E70: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 83096E74: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83096E78: 419A000C  beq cr6, 0x83096e84
	if ctx.cr[6].eq {
	pc = 0x83096E84; continue 'dispatch;
	}
	// 83096E7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83096E80: 48000054  b 0x83096ed4
	pc = 0x83096ED4; continue 'dispatch;
	// 83096E84: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83096E88: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 83096E8C: 419A0044  beq cr6, 0x83096ed0
	if ctx.cr[6].eq {
	pc = 0x83096ED0; continue 'dispatch;
	}
	// 83096E90: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83096E94: 807C000C  lwz r3, 0xc(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 83096E98: 4BF959D9  bl 0x8302c870
	ctx.lr = 0x83096E9C;
	sub_8302C870(ctx, base);
	// 83096E9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83096EA0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83096EA4: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 83096EA8: 4BF959C9  bl 0x8302c870
	ctx.lr = 0x83096EAC;
	sub_8302C870(ctx, base);
	// 83096EAC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83096EB0: 4BFFFE39  bl 0x83096ce8
	ctx.lr = 0x83096EB4;
	sub_83096CE8(ctx, base);
	// 83096EB4: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 83096EB8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83096EBC: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83096EC0: 4082FFBC  bne 0x83096e7c
	if !ctx.cr[0].eq {
	pc = 0x83096E7C; continue 'dispatch;
	}
	// 83096EC4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83096EC8: 7F1FD840  cmplw cr6, r31, r27
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[27].u32, &mut ctx.xer);
	// 83096ECC: 4198FFC4  blt cr6, 0x83096e90
	if ctx.cr[6].lt {
	pc = 0x83096E90; continue 'dispatch;
	}
	// 83096ED0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83096ED4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83096ED8: 481112DC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83096EE0 size=196
    let mut pc: u32 = 0x83096EE0;
    'dispatch: loop {
        match pc {
            0x83096EE0 => {
    //   block [0x83096EE0..0x83096FA4)
	// 83096EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83096EE4: 48111289  bl 0x831a816c
	ctx.lr = 0x83096EE8;
	sub_831A8130(ctx, base);
	// 83096EE8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83096EEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83096EF0: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83096EF4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83096EF8: 4182000C  beq 0x83096f04
	if ctx.cr[0].eq {
	pc = 0x83096F04; continue 'dispatch;
	}
	// 83096EFC: 83AB0008  lwz r29, 8(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83096F00: 48000008  b 0x83096f08
	pc = 0x83096F08; continue 'dispatch;
	// 83096F04: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83096F08: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83096F0C: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83096F10: 41820060  beq 0x83096f70
	if ctx.cr[0].eq {
	pc = 0x83096F70; continue 'dispatch;
	}
	// 83096F14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83096F18: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83096F1C: 4BF95955  bl 0x8302c870
	ctx.lr = 0x83096F20;
	sub_8302C870(ctx, base);
	// 83096F20: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83096F24: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83096F28: 4182000C  beq 0x83096f34
	if ctx.cr[0].eq {
	pc = 0x83096F34; continue 'dispatch;
	}
	// 83096F2C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83096F30: 48000008  b 0x83096f38
	pc = 0x83096F38; continue 'dispatch;
	// 83096F34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83096F38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83096F3C: 419A0028  beq cr6, 0x83096f64
	if ctx.cr[6].eq {
	pc = 0x83096F64; continue 'dispatch;
	}
	// 83096F40: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83096F44: 419A0010  beq cr6, 0x83096f54
	if ctx.cr[6].eq {
	pc = 0x83096F54; continue 'dispatch;
	}
	// 83096F48: 388BFFFF  addi r4, r11, -1
	ctx.r[4].s64 = ctx.r[11].s64 + -1;
	// 83096F4C: 4BF95925  bl 0x8302c870
	ctx.lr = 0x83096F50;
	sub_8302C870(ctx, base);
	// 83096F50: 48000008  b 0x83096f58
	pc = 0x83096F58; continue 'dispatch;
	// 83096F54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83096F58: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83096F5C: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 83096F60: 419A0018  beq cr6, 0x83096f78
	if ctx.cr[6].eq {
	pc = 0x83096F78; continue 'dispatch;
	}
	// 83096F64: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83096F68: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83096F6C: 4198FFA8  blt cr6, 0x83096f14
	if ctx.cr[6].lt {
	pc = 0x83096F14; continue 'dispatch;
	}
	// 83096F70: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83096F74: 48111248  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83096F78: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83096F7C: 80FE0010  lwz r7, 0x10(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83096F80: 38C0012E  li r6, 0x12e
	ctx.r[6].s64 = 302;
	// 83096F84: 388BD458  addi r4, r11, -0x2ba8
	ctx.r[4].s64 = ctx.r[11].s64 + -11176;
	// 83096F88: 38A001A0  li r5, 0x1a0
	ctx.r[5].s64 = 416;
	// 83096F8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83096F90: 4BFFEE89  bl 0x83095e18
	ctx.lr = 0x83096F94;
	sub_83095E18(ctx, base);
	// 83096F94: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83096F98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83096F9C: 388BCA78  addi r4, r11, -0x3588
	ctx.r[4].s64 = ctx.r[11].s64 + -13704;
	// 83096FA0: 48119C89  bl 0x831b0c28
	ctx.lr = 0x83096FA4;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83096FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83096FA8 size=100
    let mut pc: u32 = 0x83096FA8;
    'dispatch: loop {
        match pc {
            0x83096FA8 => {
    //   block [0x83096FA8..0x8309700C)
	// 83096FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83096FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83096FB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83096FB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83096FB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83096FBC: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 83096FC0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83096FC4: 4BF412D5  bl 0x82fd8298
	ctx.lr = 0x83096FC8;
	sub_82FD8298(ctx, base);
	// 83096FC8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83096FCC: 41820028  beq 0x83096ff4
	if ctx.cr[0].eq {
	pc = 0x83096FF4; continue 'dispatch;
	}
	// 83096FD0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83096FD4: 93E30010  stw r31, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 83096FD8: 394BD410  addi r10, r11, -0x2bf0
	ctx.r[10].s64 = ctx.r[11].s64 + -11248;
	// 83096FDC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83096FE0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83096FE4: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83096FE8: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83096FEC: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83096FF0: 48000008  b 0x83096ff8
	pc = 0x83096FF8; continue 'dispatch;
	// 83096FF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83096FF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83096FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83097000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83097004: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83097008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83097010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83097010 size=152
    let mut pc: u32 = 0x83097010;
    'dispatch: loop {
        match pc {
            0x83097010 => {
    //   block [0x83097010..0x830970A8)
	// 83097010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83097014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83097018: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309701C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83097020: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83097024: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83097028: 394BD188  addi r10, r11, -0x2e78
	ctx.r[10].s64 = ctx.r[11].s64 + -11896;
	// 8309702C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83097030: 909F0058  stw r4, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 83097034: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83097038: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8309703C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83097040: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83097044: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83097048: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8309704C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83097050: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83097054: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83097058: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8309705C: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83097060: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83097064: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 83097068: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8309706C: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 83097070: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 83097074: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 83097078: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 8309707C: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 83097080: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 83097084: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83097088: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8309708C: 4BFFF805  bl 0x83096890
	ctx.lr = 0x83097090;
	sub_83096890(ctx, base);
	// 83097090: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83097094: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83097098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309709C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830970A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830970A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830970A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830970A8 size=68
    let mut pc: u32 = 0x830970A8;
    'dispatch: loop {
        match pc {
            0x830970A8 => {
    //   block [0x830970A8..0x830970EC)
	// 830970A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830970AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830970B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830970B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830970B8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830970BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830970C0: 396BD188  addi r11, r11, -0x2e78
	ctx.r[11].s64 = ctx.r[11].s64 + -11896;
	// 830970C4: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830970C8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830970CC: 41820008  beq 0x830970d4
	if ctx.cr[0].eq {
	pc = 0x830970D4; continue 'dispatch;
	}
	// 830970D0: 4BF41211  bl 0x82fd82e0
	ctx.lr = 0x830970D4;
	sub_82FD82E0(ctx, base);
	// 830970D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830970D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830970DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830970E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830970E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830970E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830970F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830970F0 size=44
    let mut pc: u32 = 0x830970F0;
    'dispatch: loop {
        match pc {
            0x830970F0 => {
    //   block [0x830970F0..0x8309711C)
	// 830970F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830970F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830970F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830970FC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83097100: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 83097104: 38810084  addi r4, r1, 0x84
	ctx.r[4].s64 = ctx.r[1].s64 + 132;
	// 83097108: 4BF4FB01  bl 0x82fe6c08
	ctx.lr = 0x8309710C;
	sub_82FE6C08(ctx, base);
	// 8309710C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83097110: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83097114: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83097118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83097120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83097120 size=76
    let mut pc: u32 = 0x83097120;
    'dispatch: loop {
        match pc {
            0x83097120 => {
    //   block [0x83097120..0x8309716C)
	// 83097120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83097124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83097128: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309712C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83097130: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83097134: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83097138: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309713C: 4BFFFC95  bl 0x83096dd0
	ctx.lr = 0x83097140;
	sub_83096DD0(ctx, base);
	// 83097140: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83097144: 4182000C  beq 0x83097150
	if ctx.cr[0].eq {
	pc = 0x83097150; continue 'dispatch;
	}
	// 83097148: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309714C: 4BF41195  bl 0x82fd82e0
	ctx.lr = 0x83097150;
	sub_82FD82E0(ctx, base);
	// 83097150: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83097154: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83097158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309715C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83097160: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83097164: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83097168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83097170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83097170 size=312
    let mut pc: u32 = 0x83097170;
    'dispatch: loop {
        match pc {
            0x83097170 => {
    //   block [0x83097170..0x830972A8)
	// 83097170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83097174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83097178: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309717C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83097180: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83097184: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 83097188: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8309718C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83097190: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83097194: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83097198: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 8309719C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 830971A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 830971A4: 4800002C  b 0x830971d0
	pc = 0x830971D0; continue 'dispatch;
	// 830971A8: 2B0B0039  cmplwi cr6, r11, 0x39
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57 as u32, &mut ctx.xer);
	// 830971AC: 41990034  bgt cr6, 0x830971e0
	if ctx.cr[6].gt {
	pc = 0x830971E0; continue 'dispatch;
	}
	// 830971B0: 1D08000A  mulli r8, r8, 0xa
	ctx.r[8].s64 = ctx.r[8].s64 * 10;
	// 830971B4: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 830971B8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 830971BC: 390BFFD0  addi r8, r11, -0x30
	ctx.r[8].s64 = ctx.r[11].s64 + -48;
	// 830971C0: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 830971C4: 7F1F2800  cmpw cr6, r31, r5
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[5].s32, &mut ctx.xer);
	// 830971C8: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 830971CC: 419A0014  beq cr6, 0x830971e0
	if ctx.cr[6].eq {
	pc = 0x830971E0; continue 'dispatch;
	}
	// 830971D0: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830971D4: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 830971D8: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 830971DC: 4098FFCC  bge cr6, 0x830971a8
	if !ctx.cr[6].lt {
	pc = 0x830971A8; continue 'dispatch;
	}
	// 830971E0: 552B043E  clrlwi r11, r9, 0x10
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 830971E4: 2B0B002E  cmplwi cr6, r11, 0x2e
	ctx.cr[6].compare_u32(ctx.r[11].u32, 46 as u32, &mut ctx.xer);
	// 830971E8: 409A008C  bne cr6, 0x83097274
	if !ctx.cr[6].eq {
	pc = 0x83097274; continue 'dispatch;
	}
	// 830971EC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 830971F0: 7F1F2800  cmpw cr6, r31, r5
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[5].s32, &mut ctx.xer);
	// 830971F4: 40980080  bge cr6, 0x83097274
	if !ctx.cr[6].lt {
	pc = 0x83097274; continue 'dispatch;
	}
	// 830971F8: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830971FC: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 83097200: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83097204: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 83097208: 4198006C  blt cr6, 0x83097274
	if ctx.cr[6].lt {
	pc = 0x83097274; continue 'dispatch;
	}
	// 8309720C: 2B0B0039  cmplwi cr6, r11, 0x39
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57 as u32, &mut ctx.xer);
	// 83097210: 41990030  bgt cr6, 0x83097240
	if ctx.cr[6].gt {
	pc = 0x83097240; continue 'dispatch;
	}
	// 83097214: 1D27000A  mulli r9, r7, 0xa
	ctx.r[9].s64 = ctx.r[7].s64 * 10;
	// 83097218: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8309721C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83097220: 38EBFFD0  addi r7, r11, -0x30
	ctx.r[7].s64 = ctx.r[11].s64 + -48;
	// 83097224: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83097228: 7F1F2800  cmpw cr6, r31, r5
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[5].s32, &mut ctx.xer);
	// 8309722C: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83097230: 419A0010  beq cr6, 0x83097240
	if ctx.cr[6].eq {
	pc = 0x83097240; continue 'dispatch;
	}
	// 83097234: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83097238: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 8309723C: 4098FFD0  bge cr6, 0x8309720c
	if !ctx.cr[6].lt {
	pc = 0x8309720C; continue 'dispatch;
	}
	// 83097240: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 83097244: 419A0030  beq cr6, 0x83097274
	if ctx.cr[6].eq {
	pc = 0x83097274; continue 'dispatch;
	}
	// 83097248: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309724C: 80FE0010  lwz r7, 0x10(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83097250: 38C00140  li r6, 0x140
	ctx.r[6].s64 = 320;
	// 83097254: 388BD458  addi r4, r11, -0x2ba8
	ctx.r[4].s64 = ctx.r[11].s64 + -11176;
	// 83097258: 38A00579  li r5, 0x579
	ctx.r[5].s64 = 1401;
	// 8309725C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83097260: 4BF39DF9  bl 0x82fd1058
	ctx.lr = 0x83097264;
	sub_82FD1058(ctx, base);
	// 83097264: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83097268: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8309726C: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 83097270: 481199B9  bl 0x831b0c28
	ctx.lr = 0x83097274;
	sub_831B0C28(ctx, base);
	// 83097274: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83097278: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309727C: 4BF4F98D  bl 0x82fe6c08
	ctx.lr = 0x83097280;
	sub_82FE6C08(ctx, base);
	// 83097280: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83097284: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83097288: 4BF4F981  bl 0x82fe6c08
	ctx.lr = 0x8309728C;
	sub_82FE6C08(ctx, base);
	// 8309728C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83097290: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83097294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83097298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309729C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830972A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830972A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830972A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830972A8 size=176
    let mut pc: u32 = 0x830972A8;
    'dispatch: loop {
        match pc {
            0x830972A8 => {
    //   block [0x830972A8..0x83097358)
	// 830972A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830972AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830972B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830972B4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830972B8: 90A10094  stw r5, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[5].u32 ) };
	// 830972BC: 2F050006  cmpwi cr6, r5, 6
	ctx.cr[6].compare_i32(ctx.r[5].s32, 6, &mut ctx.xer);
	// 830972C0: 419A0080  beq cr6, 0x83097340
	if ctx.cr[6].eq {
	pc = 0x83097340; continue 'dispatch;
	}
	// 830972C4: 2F050023  cmpwi cr6, r5, 0x23
	ctx.cr[6].compare_i32(ctx.r[5].s32, 35, &mut ctx.xer);
	// 830972C8: 419A0078  beq cr6, 0x83097340
	if ctx.cr[6].eq {
	pc = 0x83097340; continue 'dispatch;
	}
	// 830972CC: 2F050024  cmpwi cr6, r5, 0x24
	ctx.cr[6].compare_i32(ctx.r[5].s32, 36, &mut ctx.xer);
	// 830972D0: 419A0070  beq cr6, 0x83097340
	if ctx.cr[6].eq {
	pc = 0x83097340; continue 'dispatch;
	}
	// 830972D4: 2F050008  cmpwi cr6, r5, 8
	ctx.cr[6].compare_i32(ctx.r[5].s32, 8, &mut ctx.xer);
	// 830972D8: 419A0068  beq cr6, 0x83097340
	if ctx.cr[6].eq {
	pc = 0x83097340; continue 'dispatch;
	}
	// 830972DC: 2F05000B  cmpwi cr6, r5, 0xb
	ctx.cr[6].compare_i32(ctx.r[5].s32, 11, &mut ctx.xer);
	// 830972E0: 419A0060  beq cr6, 0x83097340
	if ctx.cr[6].eq {
	pc = 0x83097340; continue 'dispatch;
	}
	// 830972E4: 2F050015  cmpwi cr6, r5, 0x15
	ctx.cr[6].compare_i32(ctx.r[5].s32, 21, &mut ctx.xer);
	// 830972E8: 419A0058  beq cr6, 0x83097340
	if ctx.cr[6].eq {
	pc = 0x83097340; continue 'dispatch;
	}
	// 830972EC: 2F050004  cmpwi cr6, r5, 4
	ctx.cr[6].compare_i32(ctx.r[5].s32, 4, &mut ctx.xer);
	// 830972F0: 419A0050  beq cr6, 0x83097340
	if ctx.cr[6].eq {
	pc = 0x83097340; continue 'dispatch;
	}
	// 830972F4: 2F050009  cmpwi cr6, r5, 9
	ctx.cr[6].compare_i32(ctx.r[5].s32, 9, &mut ctx.xer);
	// 830972F8: 419A0048  beq cr6, 0x83097340
	if ctx.cr[6].eq {
	pc = 0x83097340; continue 'dispatch;
	}
	// 830972FC: 2F05000A  cmpwi cr6, r5, 0xa
	ctx.cr[6].compare_i32(ctx.r[5].s32, 10, &mut ctx.xer);
	// 83097300: 419A0040  beq cr6, 0x83097340
	if ctx.cr[6].eq {
	pc = 0x83097340; continue 'dispatch;
	}
	// 83097304: 2F050016  cmpwi cr6, r5, 0x16
	ctx.cr[6].compare_i32(ctx.r[5].s32, 22, &mut ctx.xer);
	// 83097308: 419A0038  beq cr6, 0x83097340
	if ctx.cr[6].eq {
	pc = 0x83097340; continue 'dispatch;
	}
	// 8309730C: 2F050017  cmpwi cr6, r5, 0x17
	ctx.cr[6].compare_i32(ctx.r[5].s32, 23, &mut ctx.xer);
	// 83097310: 419A0030  beq cr6, 0x83097340
	if ctx.cr[6].eq {
	pc = 0x83097340; continue 'dispatch;
	}
	// 83097314: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83097318: 80E30010  lwz r7, 0x10(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309731C: 38C0013F  li r6, 0x13f
	ctx.r[6].s64 = 319;
	// 83097320: 388BD458  addi r4, r11, -0x2ba8
	ctx.r[4].s64 = ctx.r[11].s64 + -11176;
	// 83097324: 38A005A6  li r5, 0x5a6
	ctx.r[5].s64 = 1446;
	// 83097328: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8309732C: 4BFFEAED  bl 0x83095e18
	ctx.lr = 0x83097330;
	sub_83095E18(ctx, base);
	// 83097330: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83097334: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83097338: 388BCA78  addi r4, r11, -0x3588
	ctx.r[4].s64 = ctx.r[11].s64 + -13704;
	// 8309733C: 481198ED  bl 0x831b0c28
	ctx.lr = 0x83097340;
	sub_831B0C28(ctx, base);
	// 83097340: 38810094  addi r4, r1, 0x94
	ctx.r[4].s64 = ctx.r[1].s64 + 148;
	// 83097344: 4BF4F8C5  bl 0x82fe6c08
	ctx.lr = 0x83097348;
	sub_82FE6C08(ctx, base);
	// 83097348: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309734C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83097350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83097354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83097358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83097358 size=8
    let mut pc: u32 = 0x83097358;
    'dispatch: loop {
        match pc {
            0x83097358 => {
    //   block [0x83097358..0x83097360)
	// 83097358: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309735C: 8216D4B0  lwz r16, -0x2b50(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-11088 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83097360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83097360 size=2960
    let mut pc: u32 = 0x83097360;
    'dispatch: loop {
        match pc {
            0x83097360 => {
    //   block [0x83097360..0x83097EF0)
	// 83097360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83097364: 48110DDD  bl 0x831a8140
	ctx.lr = 0x83097368;
	sub_831A8130(ctx, base);
	// 83097368: 3BE1FEE0  addi r31, r1, -0x120
	ctx.r[31].s64 = ctx.r[1].s64 + -288;
	// 8309736C: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83097370: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 83097374: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83097378: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 8309737C: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 83097380: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 83097384: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83097388: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8309738C: 80BA0010  lwz r5, 0x10(r26)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 83097390: 7CD53378  mr r21, r6
	ctx.r[21].u64 = ctx.r[6].u64;
	// 83097394: 7EDCB378  mr r28, r22
	ctx.r[28].u64 = ctx.r[22].u64;
	// 83097398: 4BF47AC1  bl 0x82fdee58
	ctx.lr = 0x8309739C;
	sub_82FDEE58(ctx, base);
	// 8309739C: 7F1DA800  cmpw cr6, r29, r21
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[21].s32, &mut ctx.xer);
	// 830973A0: 419A0A8C  beq cr6, 0x83097e2c
	if ctx.cr[6].eq {
	pc = 0x83097E2C; continue 'dispatch;
	}
	// 830973A4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 830973A8: 3A60FFFF  li r19, -1
	ctx.r[19].s64 = -1;
	// 830973AC: 3A8B2DD8  addi r20, r11, 0x2dd8
	ctx.r[20].s64 = ctx.r[11].s64 + 11736;
	// 830973B0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830973B4: 3A4BD0D8  addi r18, r11, -0x2f28
	ctx.r[18].s64 = ctx.r[11].s64 + -12072;
	// 830973B8: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830973BC: 7D6BBA14  add r11, r11, r23
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 830973C0: 48000014  b 0x830973d4
	pc = 0x830973D4; continue 'dispatch;
	// 830973C4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830973C8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830973CC: 7F1DA800  cmpw cr6, r29, r21
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[21].s32, &mut ctx.xer);
	// 830973D0: 419A0A5C  beq cr6, 0x83097e2c
	if ctx.cr[6].eq {
	pc = 0x83097E2C; continue 'dispatch;
	}
	// 830973D4: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830973D8: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 830973DC: 7D29A0AE  lbzx r9, r9, r20
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 830973E0: 55290031  rlwinm. r9, r9, 0, 0, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 830973E4: 4082FFE0  bne 0x830973c4
	if !ctx.cr[0].eq {
	pc = 0x830973C4; continue 'dispatch;
	}
	// 830973E8: 7F1DA800  cmpw cr6, r29, r21
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[21].s32, &mut ctx.xer);
	// 830973EC: 419A0A40  beq cr6, 0x83097e2c
	if ctx.cr[6].eq {
	pc = 0x83097E2C; continue 'dispatch;
	}
	// 830973F0: 554B043E  clrlwi r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 830973F4: 2B0B0080  cmplwi cr6, r11, 0x80
	ctx.cr[6].compare_u32(ctx.r[11].u32, 128 as u32, &mut ctx.xer);
	// 830973F8: 4198000C  blt cr6, 0x83097404
	if ctx.cr[6].lt {
	pc = 0x83097404; continue 'dispatch;
	}
	// 830973FC: 39600019  li r11, 0x19
	ctx.r[11].s64 = 25;
	// 83097400: 48000008  b 0x83097408
	pc = 0x83097408; continue 'dispatch;
	// 83097404: 7D6B90AE  lbzx r11, r11, r18
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[18].u32)) } as u64;
	// 83097408: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8309740C: 396BFFFD  addi r11, r11, -3
	ctx.r[11].s64 = ctx.r[11].s64 + -3;
	// 83097410: 2B0B0016  cmplwi cr6, r11, 0x16
	ctx.cr[6].compare_u32(ctx.r[11].u32, 22 as u32, &mut ctx.xer);
	// 83097414: 41990A10  bgt cr6, 0x83097e24
	if ctx.cr[6].gt {
	pc = 0x83097E24; continue 'dispatch;
	}
	// 83097418: 3D808217  lis r12, -0x7de9
	ctx.r[12].s64 = -2112421888;
	// 8309741C: 398CD158  addi r12, r12, -0x2ea8
	ctx.r[12].s64 = ctx.r[12].s64 + -11944;
	// 83097420: 5560083C  slwi r0, r11, 1
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 83097424: 7C0C022E  lhzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 83097428: 3D808309  lis r12, -0x7cf7
	ctx.r[12].s64 = -2096562176;
	// 8309742C: 398C7440  addi r12, r12, 0x7440
	ctx.r[12].s64 = ctx.r[12].s64 + 29760;
	// 83097430: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 83097434: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 83097438: 60000000  nop
	// 8309743C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	// 83097440: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83097444: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83097448: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8309744C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83097450: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83097454: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83097458: 4E800421  bctrl
	ctx.lr = 0x8309745C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309745C: 7EDCB378  mr r28, r22
	ctx.r[28].u64 = ctx.r[22].u64;
	// 83097460: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83097464: 480009C0  b 0x83097e24
	pc = 0x83097E24; continue 'dispatch;
	// 83097468: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8309746C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83097470: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83097474: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83097478: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309747C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83097480: 4E800421  bctrl
	ctx.lr = 0x83097484;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83097484: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 83097488: 4BFFFFD8  b 0x83097460
	pc = 0x83097460; continue 'dispatch;
	// 8309748C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 83097490: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83097494: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83097498: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309749C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830974A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830974A4: 4BFFFFB4  b 0x83097458
	pc = 0x83097458; continue 'dispatch;
	// 830974A8: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 830974AC: 4BFFFFC0  b 0x8309746c
	pc = 0x8309746C; continue 'dispatch;
	// 830974B0: 3B7D0001  addi r27, r29, 1
	ctx.r[27].s64 = ctx.r[29].s64 + 1;
	// 830974B4: 7F1BA800  cmpw cr6, r27, r21
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[21].s32, &mut ctx.xer);
	// 830974B8: 409A002C  bne cr6, 0x830974e4
	if !ctx.cr[6].eq {
	pc = 0x830974E4; continue 'dispatch;
	}
	// 830974BC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830974C0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 830974C4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830974C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830974CC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830974D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830974D4: 4E800421  bctrl
	ctx.lr = 0x830974D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830974D8: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 830974DC: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 830974E0: 48000944  b 0x83097e24
	pc = 0x83097E24; continue 'dispatch;
	// 830974E4: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830974E8: 7D4BBA14  add r10, r11, r23
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 830974EC: A16A0002  lhz r11, 2(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 830974F0: 2B0B002E  cmplwi cr6, r11, 0x2e
	ctx.cr[6].compare_u32(ctx.r[11].u32, 46 as u32, &mut ctx.xer);
	// 830974F4: 409A0028  bne cr6, 0x8309751c
	if !ctx.cr[6].eq {
	pc = 0x8309751C; continue 'dispatch;
	}
	// 830974F8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830974FC: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 83097500: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83097504: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83097508: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309750C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83097510: 4E800421  bctrl
	ctx.lr = 0x83097514;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83097514: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 83097518: 48000908  b 0x83097e20
	pc = 0x83097E20; continue 'dispatch;
	// 8309751C: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 83097520: 4198000C  blt cr6, 0x8309752c
	if ctx.cr[6].lt {
	pc = 0x8309752C; continue 'dispatch;
	}
	// 83097524: 2B0B0039  cmplwi cr6, r11, 0x39
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57 as u32, &mut ctx.xer);
	// 83097528: 40990288  ble cr6, 0x830977b0
	if !ctx.cr[6].gt {
	pc = 0x830977B0; continue 'dispatch;
	}
	// 8309752C: 2B0B002F  cmplwi cr6, r11, 0x2f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 47 as u32, &mut ctx.xer);
	// 83097530: 419AFF8C  beq cr6, 0x830974bc
	if ctx.cr[6].eq {
	pc = 0x830974BC; continue 'dispatch;
	}
	// 83097534: 2B0B007C  cmplwi cr6, r11, 0x7c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 124 as u32, &mut ctx.xer);
	// 83097538: 419AFF84  beq cr6, 0x830974bc
	if ctx.cr[6].eq {
	pc = 0x830974BC; continue 'dispatch;
	}
	// 8309753C: 7D6BA0AE  lbzx r11, r11, r20
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 83097540: 556B0031  rlwinm. r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83097544: 41820908  beq 0x83097e4c
	if ctx.cr[0].eq {
	pc = 0x83097E4C; continue 'dispatch;
	}
	// 83097548: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8309754C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83097550: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83097554: 7F1DA800  cmpw cr6, r29, r21
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[21].s32, &mut ctx.xer);
	// 83097558: 419A0024  beq cr6, 0x8309757c
	if ctx.cr[6].eq {
	pc = 0x8309757C; continue 'dispatch;
	}
	// 8309755C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83097560: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 83097564: 7D29A0AE  lbzx r9, r9, r20
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 83097568: 55290031  rlwinm. r9, r9, 0, 0, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8309756C: 4082FFE0  bne 0x8309754c
	if !ctx.cr[0].eq {
	pc = 0x8309754C; continue 'dispatch;
	}
	// 83097570: 554B043E  clrlwi r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 83097574: 2B0B007C  cmplwi cr6, r11, 0x7c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 124 as u32, &mut ctx.xer);
	// 83097578: 409A08AC  bne cr6, 0x83097e24
	if !ctx.cr[6].eq {
	pc = 0x83097E24; continue 'dispatch;
	}
	// 8309757C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83097580: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83097584: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83097588: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309758C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83097590: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83097594: 4E800421  bctrl
	ctx.lr = 0x83097598;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83097598: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 8309759C: 48000888  b 0x83097e24
	pc = 0x83097E24; continue 'dispatch;
	// 830975A0: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 830975A4: 4BFFFEEC  b 0x83097490
	pc = 0x83097490; continue 'dispatch;
	// 830975A8: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 830975AC: 4BFFFEE4  b 0x83097490
	pc = 0x83097490; continue 'dispatch;
	// 830975B0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830975B4: 7F1DA800  cmpw cr6, r29, r21
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[21].s32, &mut ctx.xer);
	// 830975B8: 419A08C0  beq cr6, 0x83097e78
	if ctx.cr[6].eq {
	pc = 0x83097E78; continue 'dispatch;
	}
	// 830975BC: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830975C0: 7D6BBA2E  lhzx r11, r11, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 830975C4: 2B0B003A  cmplwi cr6, r11, 0x3a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 58 as u32, &mut ctx.xer);
	// 830975C8: 409A08C8  bne cr6, 0x83097e90
	if !ctx.cr[6].eq {
	pc = 0x83097E90; continue 'dispatch;
	}
	// 830975CC: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 830975D0: 4BFFFEC0  b 0x83097490
	pc = 0x83097490; continue 'dispatch;
	// 830975D4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830975D8: 7F1DA800  cmpw cr6, r29, r21
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[21].s32, &mut ctx.xer);
	// 830975DC: 409A0028  bne cr6, 0x83097604
	if !ctx.cr[6].eq {
	pc = 0x83097604; continue 'dispatch;
	}
	// 830975E0: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 830975E4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830975E8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830975EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830975F0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830975F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830975F8: 4E800421  bctrl
	ctx.lr = 0x830975FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830975FC: 7EDCB378  mr r28, r22
	ctx.r[28].u64 = ctx.r[22].u64;
	// 83097600: 48000824  b 0x83097e24
	pc = 0x83097E24; continue 'dispatch;
	// 83097604: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83097608: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8309760C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83097610: 7D6BBA2E  lhzx r11, r11, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 83097614: 2B0B002F  cmplwi cr6, r11, 0x2f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 47 as u32, &mut ctx.xer);
	// 83097618: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309761C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83097620: 409A000C  bne cr6, 0x8309762c
	if !ctx.cr[6].eq {
	pc = 0x8309762C; continue 'dispatch;
	}
	// 83097624: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 83097628: 4BFFFE78  b 0x830974a0
	pc = 0x830974A0; continue 'dispatch;
	// 8309762C: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 83097630: 4BFFFFC4  b 0x830975f4
	pc = 0x830975F4; continue 'dispatch;
	// 83097634: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 83097638: 4BFFFE58  b 0x83097490
	pc = 0x83097490; continue 'dispatch;
	// 8309763C: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 83097640: 4BFFFE50  b 0x83097490
	pc = 0x83097490; continue 'dispatch;
	// 83097644: 38A00019  li r5, 0x19
	ctx.r[5].s64 = 25;
	// 83097648: 4BFFFE48  b 0x83097490
	pc = 0x83097490; continue 'dispatch;
	// 8309764C: 38A0001A  li r5, 0x1a
	ctx.r[5].s64 = 26;
	// 83097650: 4BFFFE40  b 0x83097490
	pc = 0x83097490; continue 'dispatch;
	// 83097654: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83097658: 7F1DA800  cmpw cr6, r29, r21
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[21].s32, &mut ctx.xer);
	// 8309765C: 419A0838  beq cr6, 0x83097e94
	if ctx.cr[6].eq {
	pc = 0x83097E94; continue 'dispatch;
	}
	// 83097660: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83097664: 7D6BBA2E  lhzx r11, r11, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 83097668: 2B0B003D  cmplwi cr6, r11, 0x3d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 61 as u32, &mut ctx.xer);
	// 8309766C: 409A082C  bne cr6, 0x83097e98
	if !ctx.cr[6].eq {
	pc = 0x83097E98; continue 'dispatch;
	}
	// 83097670: 38A0001B  li r5, 0x1b
	ctx.r[5].s64 = 27;
	// 83097674: 4BFFFE1C  b 0x83097490
	pc = 0x83097490; continue 'dispatch;
	// 83097678: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8309767C: 7F1DA800  cmpw cr6, r29, r21
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[21].s32, &mut ctx.xer);
	// 83097680: 409A000C  bne cr6, 0x8309768c
	if !ctx.cr[6].eq {
	pc = 0x8309768C; continue 'dispatch;
	}
	// 83097684: 38A0001C  li r5, 0x1c
	ctx.r[5].s64 = 28;
	// 83097688: 4BFFFF5C  b 0x830975e4
	pc = 0x830975E4; continue 'dispatch;
	// 8309768C: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83097690: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83097694: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83097698: 7D6BBA2E  lhzx r11, r11, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 8309769C: 2B0B003D  cmplwi cr6, r11, 0x3d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 61 as u32, &mut ctx.xer);
	// 830976A0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830976A4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830976A8: 409A000C  bne cr6, 0x830976b4
	if !ctx.cr[6].eq {
	pc = 0x830976B4; continue 'dispatch;
	}
	// 830976AC: 38A0001D  li r5, 0x1d
	ctx.r[5].s64 = 29;
	// 830976B0: 4BFFFDF0  b 0x830974a0
	pc = 0x830974A0; continue 'dispatch;
	// 830976B4: 38A0001C  li r5, 0x1c
	ctx.r[5].s64 = 28;
	// 830976B8: 4BFFFF3C  b 0x830975f4
	pc = 0x830975F4; continue 'dispatch;
	// 830976BC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830976C0: 7F1DA800  cmpw cr6, r29, r21
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[21].s32, &mut ctx.xer);
	// 830976C4: 409A000C  bne cr6, 0x830976d0
	if !ctx.cr[6].eq {
	pc = 0x830976D0; continue 'dispatch;
	}
	// 830976C8: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 830976CC: 4BFFFF18  b 0x830975e4
	pc = 0x830975E4; continue 'dispatch;
	// 830976D0: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830976D4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830976D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830976DC: 7D6BBA2E  lhzx r11, r11, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 830976E0: 2B0B003D  cmplwi cr6, r11, 0x3d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 61 as u32, &mut ctx.xer);
	// 830976E4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830976E8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830976EC: 409A000C  bne cr6, 0x830976f8
	if !ctx.cr[6].eq {
	pc = 0x830976F8; continue 'dispatch;
	}
	// 830976F0: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 830976F4: 4BFFFDAC  b 0x830974a0
	pc = 0x830974A0; continue 'dispatch;
	// 830976F8: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 830976FC: 4BFFFEF8  b 0x830975f4
	pc = 0x830975F4; continue 'dispatch;
	// 83097700: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83097704: 7F1DA800  cmpw cr6, r29, r21
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[21].s32, &mut ctx.xer);
	// 83097708: 419A0794  beq cr6, 0x83097e9c
	if ctx.cr[6].eq {
	pc = 0x83097E9C; continue 'dispatch;
	}
	// 8309770C: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83097710: 5549043E  clrlwi r9, r10, 0x10
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 83097714: 7D6BBA14  add r11, r11, r23
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 83097718: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 8309771C: 48000014  b 0x83097730
	pc = 0x83097730; continue 'dispatch;
	// 83097720: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83097724: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83097728: 7F1DA800  cmpw cr6, r29, r21
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[21].s32, &mut ctx.xer);
	// 8309772C: 419A0774  beq cr6, 0x83097ea0
	if ctx.cr[6].eq {
	pc = 0x83097EA0; continue 'dispatch;
	}
	// 83097730: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83097734: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83097738: 409AFFE8  bne cr6, 0x83097720
	if !ctx.cr[6].eq {
	pc = 0x83097720; continue 'dispatch;
	}
	// 8309773C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83097740: 38A0002E  li r5, 0x2e
	ctx.r[5].s64 = 46;
	// 83097744: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83097748: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309774C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83097750: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83097754: 4E800421  bctrl
	ctx.lr = 0x83097758;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83097758: 578B083C  slwi r11, r28, 1
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8309775C: 7CBCE850  subf r5, r28, r29
	ctx.r[5].s64 = ctx.r[29].s64 - ctx.r[28].s64;
	// 83097760: 92DF0084  stw r22, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[22].u32 ) };
	// 83097764: 7C8BBA14  add r4, r11, r23
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 83097768: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 8309776C: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 83097770: 4BF41E01  bl 0x82fd9570
	ctx.lr = 0x83097774;
	sub_82FD9570(ctx, base);
	// 83097774: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83097778: 815F0098  lwz r10, 0x98(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 8309777C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83097780: 7ECB532E  sthx r22, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[22].u16) };
	// 83097784: 807E0058  lwz r3, 0x58(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 83097788: 809F0098  lwz r4, 0x98(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 8309778C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83097790: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83097794: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83097798: 4E800421  bctrl
	ctx.lr = 0x8309779C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309779C: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830977A0: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 830977A4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830977A8: 4BF4F461  bl 0x82fe6c08
	ctx.lr = 0x830977AC;
	sub_82FE6C08(ctx, base);
	// 830977AC: 4BFFFCB4  b 0x83097460
	pc = 0x83097460; continue 'dispatch;
	// 830977B0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830977B4: 38A0002F  li r5, 0x2f
	ctx.r[5].s64 = 47;
	// 830977B8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830977BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830977C0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830977C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830977C8: 4E800421  bctrl
	ctx.lr = 0x830977CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830977CC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 830977D0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 830977D4: 7EA5AB78  mr r5, r21
	ctx.r[5].u64 = ctx.r[21].u64;
	// 830977D8: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 830977DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830977E0: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 830977E4: 4BFFF98D  bl 0x83097170
	ctx.lr = 0x830977E8;
	sub_83097170(ctx, base);
	// 830977E8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830977EC: 48000638  b 0x83097e24
	pc = 0x83097E24; continue 'dispatch;
	// 830977F0: 391D0001  addi r8, r29, 1
	ctx.r[8].s64 = ctx.r[29].s64 + 1;
	// 830977F4: 7F08A800  cmpw cr6, r8, r21
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[21].s32, &mut ctx.xer);
	// 830977F8: 419A06AC  beq cr6, 0x83097ea4
	if ctx.cr[6].eq {
	pc = 0x83097EA4; continue 'dispatch;
	}
	// 830977FC: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 83097800: 7EA5AB78  mr r5, r21
	ctx.r[5].u64 = ctx.r[21].u64;
	// 83097804: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 83097808: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309780C: 4BFFF39D  bl 0x83096ba8
	ctx.lr = 0x83097810;
	sub_83096BA8(ctx, base);
	// 83097810: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83097814: 7F1D4000  cmpw cr6, r29, r8
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83097818: 419A0690  beq cr6, 0x83097ea8
	if ctx.cr[6].eq {
	pc = 0x83097EA8; continue 'dispatch;
	}
	// 8309781C: 7F1DA800  cmpw cr6, r29, r21
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[21].s32, &mut ctx.xer);
	// 83097820: 40980010  bge cr6, 0x83097830
	if !ctx.cr[6].lt {
	pc = 0x83097830; continue 'dispatch;
	}
	// 83097824: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83097828: 7F8BBA2E  lhzx r28, r11, r23
	ctx.r[28].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 8309782C: 48000008  b 0x83097834
	pc = 0x83097834; continue 'dispatch;
	// 83097830: 7EDCB378  mr r28, r22
	ctx.r[28].u64 = ctx.r[22].u64;
	// 83097834: 550B083C  slwi r11, r8, 1
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83097838: 92DF0084  stw r22, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[22].u32 ) };
	// 8309783C: 7CA8E850  subf r5, r8, r29
	ctx.r[5].s64 = ctx.r[29].s64 - ctx.r[8].s64;
	// 83097840: 7C8BBA14  add r4, r11, r23
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 83097844: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83097848: 4BF41D29  bl 0x82fd9570
	ctx.lr = 0x8309784C;
	sub_82FD9570(ctx, base);
	// 8309784C: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83097850: 815F0098  lwz r10, 0x98(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 83097854: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83097858: 7ECB532E  sthx r22, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[22].u16) };
	// 8309785C: 807E0058  lwz r3, 0x58(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 83097860: 809F0098  lwz r4, 0x98(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 83097864: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83097868: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309786C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83097870: 4E800421  bctrl
	ctx.lr = 0x83097874;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83097874: 578B043E  clrlwi r11, r28, 0x10
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x0000FFFFu64;
	// 83097878: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8309787C: 927F0054  stw r19, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[19].u32 ) };
	// 83097880: 2B0B003A  cmplwi cr6, r11, 0x3a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 58 as u32, &mut ctx.xer);
	// 83097884: 409A0078  bne cr6, 0x830978fc
	if !ctx.cr[6].eq {
	pc = 0x830978FC; continue 'dispatch;
	}
	// 83097888: 391D0001  addi r8, r29, 1
	ctx.r[8].s64 = ctx.r[29].s64 + 1;
	// 8309788C: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83097890: 7F08A800  cmpw cr6, r8, r21
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[21].s32, &mut ctx.xer);
	// 83097894: 419A0618  beq cr6, 0x83097eac
	if ctx.cr[6].eq {
	pc = 0x83097EAC; continue 'dispatch;
	}
	// 83097898: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 8309789C: 7EA5AB78  mr r5, r21
	ctx.r[5].u64 = ctx.r[21].u64;
	// 830978A0: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 830978A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830978A8: 4BFFF301  bl 0x83096ba8
	ctx.lr = 0x830978AC;
	sub_83096BA8(ctx, base);
	// 830978AC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830978B0: 7F1D4000  cmpw cr6, r29, r8
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[8].s32, &mut ctx.xer);
	// 830978B4: 419A05FC  beq cr6, 0x83097eb0
	if ctx.cr[6].eq {
	pc = 0x83097EB0; continue 'dispatch;
	}
	// 830978B8: 550B083C  slwi r11, r8, 1
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830978BC: 92DF0084  stw r22, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[22].u32 ) };
	// 830978C0: 7CA8E850  subf r5, r8, r29
	ctx.r[5].s64 = ctx.r[29].s64 - ctx.r[8].s64;
	// 830978C4: 7C8BBA14  add r4, r11, r23
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 830978C8: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 830978CC: 4BF41CA5  bl 0x82fd9570
	ctx.lr = 0x830978D0;
	sub_82FD9570(ctx, base);
	// 830978D0: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830978D4: 815F0098  lwz r10, 0x98(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 830978D8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830978DC: 7ECB532E  sthx r22, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[22].u16) };
	// 830978E0: 807E0058  lwz r3, 0x58(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 830978E4: 809F0098  lwz r4, 0x98(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 830978E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830978EC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830978F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830978F4: 4E800421  bctrl
	ctx.lr = 0x830978F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830978F8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830978FC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83097900: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 83097904: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83097908: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309790C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83097910: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83097914: 4E800421  bctrl
	ctx.lr = 0x83097918;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83097918: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 8309791C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83097920: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 83097924: 4BF4F2E5  bl 0x82fe6c08
	ctx.lr = 0x83097928;
	sub_82FE6C08(ctx, base);
	// 83097928: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8309792C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83097930: 4BF4F2D9  bl 0x82fe6c08
	ctx.lr = 0x83097934;
	sub_82FE6C08(ctx, base);
	// 83097934: 480004F0  b 0x83097e24
	pc = 0x83097E24; continue 'dispatch;
	// 83097938: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309793C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83097940: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83097944: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83097948: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309794C: 4182000C  beq 0x83097958
	if ctx.cr[0].eq {
	pc = 0x83097958; continue 'dispatch;
	}
	// 83097950: 38A00014  li r5, 0x14
	ctx.r[5].s64 = 20;
	// 83097954: 4BFFFB4C  b 0x830974a0
	pc = 0x830974A0; continue 'dispatch;
	// 83097958: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 8309795C: 4BFFFB20  b 0x8309747c
	pc = 0x8309747C; continue 'dispatch;
	// 83097960: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83097964: 7EA5AB78  mr r5, r21
	ctx.r[5].u64 = ctx.r[21].u64;
	// 83097968: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8309796C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83097970: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 83097974: 4BFFF235  bl 0x83096ba8
	ctx.lr = 0x83097978;
	sub_83096BA8(ctx, base);
	// 83097978: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8309797C: 7F1D4000  cmpw cr6, r29, r8
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83097980: 419A0534  beq cr6, 0x83097eb4
	if ctx.cr[6].eq {
	pc = 0x83097EB4; continue 'dispatch;
	}
	// 83097984: 7F1DA800  cmpw cr6, r29, r21
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[21].s32, &mut ctx.xer);
	// 83097988: 40980010  bge cr6, 0x83097998
	if !ctx.cr[6].lt {
	pc = 0x83097998; continue 'dispatch;
	}
	// 8309798C: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83097990: 7F6BBA2E  lhzx r27, r11, r23
	ctx.r[27].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 83097994: 48000008  b 0x8309799c
	pc = 0x8309799C; continue 'dispatch;
	// 83097998: 7EDBB378  mr r27, r22
	ctx.r[27].u64 = ctx.r[22].u64;
	// 8309799C: 550B083C  slwi r11, r8, 1
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830979A0: 92DF0084  stw r22, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[22].u32 ) };
	// 830979A4: 7CA8E850  subf r5, r8, r29
	ctx.r[5].s64 = ctx.r[29].s64 - ctx.r[8].s64;
	// 830979A8: 7C8BBA14  add r4, r11, r23
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 830979AC: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 830979B0: 4BF41BC1  bl 0x82fd9570
	ctx.lr = 0x830979B4;
	sub_82FD9570(ctx, base);
	// 830979B4: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830979B8: 815F0098  lwz r10, 0x98(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 830979BC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830979C0: 7ECB532E  sthx r22, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[22].u16) };
	// 830979C4: 807E0058  lwz r3, 0x58(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 830979C8: 809F0098  lwz r4, 0x98(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 830979CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830979D0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830979D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830979D8: 4E800421  bctrl
	ctx.lr = 0x830979DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830979DC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830979E0: 576A043E  clrlwi r10, r27, 0x10
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0x0000FFFFu64;
	// 830979E4: 927F0054  stw r19, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[19].u32 ) };
	// 830979E8: 7ED8B378  mr r24, r22
	ctx.r[24].u64 = ctx.r[22].u64;
	// 830979EC: 7ED9B378  mr r25, r22
	ctx.r[25].u64 = ctx.r[22].u64;
	// 830979F0: 2B0A003A  cmplwi cr6, r10, 0x3a
	ctx.cr[6].compare_u32(ctx.r[10].u32, 58 as u32, &mut ctx.xer);
	// 830979F4: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830979F8: 409A00E8  bne cr6, 0x83097ae0
	if !ctx.cr[6].eq {
	pc = 0x83097AE0; continue 'dispatch;
	}
	// 830979FC: 38DD0001  addi r6, r29, 1
	ctx.r[6].s64 = ctx.r[29].s64 + 1;
	// 83097A00: 7F06A800  cmpw cr6, r6, r21
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[21].s32, &mut ctx.xer);
	// 83097A04: 419A04B4  beq cr6, 0x83097eb8
	if ctx.cr[6].eq {
	pc = 0x83097EB8; continue 'dispatch;
	}
	// 83097A08: 54CA083C  slwi r10, r6, 1
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83097A0C: 7F6ABA2E  lhzx r27, r10, r23
	ctx.r[27].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 83097A10: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 83097A14: 2B0A002A  cmplwi cr6, r10, 0x2a
	ctx.cr[6].compare_u32(ctx.r[10].u32, 42 as u32, &mut ctx.xer);
	// 83097A18: 409A0020  bne cr6, 0x83097a38
	if !ctx.cr[6].eq {
	pc = 0x83097A38; continue 'dispatch;
	}
	// 83097A1C: 3BA60001  addi r29, r6, 1
	ctx.r[29].s64 = ctx.r[6].s64 + 1;
	// 83097A20: 7F1DA800  cmpw cr6, r29, r21
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[21].s32, &mut ctx.xer);
	// 83097A24: 4098000C  bge cr6, 0x83097a30
	if !ctx.cr[6].lt {
	pc = 0x83097A30; continue 'dispatch;
	}
	// 83097A28: 57AA083C  slwi r10, r29, 1
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83097A2C: 7F6ABA2E  lhzx r27, r10, r23
	ctx.r[27].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 83097A30: 3B000001  li r24, 1
	ctx.r[24].s64 = 1;
	// 83097A34: 480000AC  b 0x83097ae0
	pc = 0x83097AE0; continue 'dispatch;
	// 83097A38: 2B0A003A  cmplwi cr6, r10, 0x3a
	ctx.cr[6].compare_u32(ctx.r[10].u32, 58 as u32, &mut ctx.xer);
	// 83097A3C: 409A0020  bne cr6, 0x83097a5c
	if !ctx.cr[6].eq {
	pc = 0x83097A5C; continue 'dispatch;
	}
	// 83097A40: 3BA60001  addi r29, r6, 1
	ctx.r[29].s64 = ctx.r[6].s64 + 1;
	// 83097A44: 7F1DA800  cmpw cr6, r29, r21
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[21].s32, &mut ctx.xer);
	// 83097A48: 4098000C  bge cr6, 0x83097a54
	if !ctx.cr[6].lt {
	pc = 0x83097A54; continue 'dispatch;
	}
	// 83097A4C: 57AA083C  slwi r10, r29, 1
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83097A50: 7F6ABA2E  lhzx r27, r10, r23
	ctx.r[27].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 83097A54: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 83097A58: 48000088  b 0x83097ae0
	pc = 0x83097AE0; continue 'dispatch;
	// 83097A5C: 7EA5AB78  mr r5, r21
	ctx.r[5].u64 = ctx.r[21].u64;
	// 83097A60: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83097A64: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 83097A68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83097A6C: 7CC83378  mr r8, r6
	ctx.r[8].u64 = ctx.r[6].u64;
	// 83097A70: 4BFFF139  bl 0x83096ba8
	ctx.lr = 0x83097A74;
	sub_83096BA8(ctx, base);
	// 83097A74: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83097A78: 7F1D4000  cmpw cr6, r29, r8
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83097A7C: 419A0440  beq cr6, 0x83097ebc
	if ctx.cr[6].eq {
	pc = 0x83097EBC; continue 'dispatch;
	}
	// 83097A80: 7F1DA800  cmpw cr6, r29, r21
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[21].s32, &mut ctx.xer);
	// 83097A84: 40980010  bge cr6, 0x83097a94
	if !ctx.cr[6].lt {
	pc = 0x83097A94; continue 'dispatch;
	}
	// 83097A88: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83097A8C: 7F6BBA2E  lhzx r27, r11, r23
	ctx.r[27].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 83097A90: 48000008  b 0x83097a98
	pc = 0x83097A98; continue 'dispatch;
	// 83097A94: 7EDBB378  mr r27, r22
	ctx.r[27].u64 = ctx.r[22].u64;
	// 83097A98: 550B083C  slwi r11, r8, 1
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83097A9C: 92DF0084  stw r22, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[22].u32 ) };
	// 83097AA0: 7CA8E850  subf r5, r8, r29
	ctx.r[5].s64 = ctx.r[29].s64 - ctx.r[8].s64;
	// 83097AA4: 7C8BBA14  add r4, r11, r23
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 83097AA8: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83097AAC: 4BF41AC5  bl 0x82fd9570
	ctx.lr = 0x83097AB0;
	sub_82FD9570(ctx, base);
	// 83097AB0: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83097AB4: 815F0098  lwz r10, 0x98(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 83097AB8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83097ABC: 7ECB532E  sthx r22, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[22].u16) };
	// 83097AC0: 807E0058  lwz r3, 0x58(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 83097AC4: 809F0098  lwz r4, 0x98(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 83097AC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83097ACC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83097AD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83097AD4: 4E800421  bctrl
	ctx.lr = 0x83097AD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83097AD8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83097ADC: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83097AE0: 576A043E  clrlwi r10, r27, 0x10
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0x0000FFFFu64;
	// 83097AE4: 7D4AA0AE  lbzx r10, r10, r20
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 83097AE8: 554A0031  rlwinm. r10, r10, 0, 0, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83097AEC: 41820030  beq 0x83097b1c
	if ctx.cr[0].eq {
	pc = 0x83097B1C; continue 'dispatch;
	}
	// 83097AF0: 57AA083C  slwi r10, r29, 1
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83097AF4: 7D4ABA14  add r10, r10, r23
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[23].u64;
	// 83097AF8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83097AFC: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83097B00: 7F1DA800  cmpw cr6, r29, r21
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[21].s32, &mut ctx.xer);
	// 83097B04: 419A0018  beq cr6, 0x83097b1c
	if ctx.cr[6].eq {
	pc = 0x83097B1C; continue 'dispatch;
	}
	// 83097B08: A36A0000  lhz r27, 0(r10)
	ctx.r[27].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83097B0C: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 83097B10: 7D29A0AE  lbzx r9, r9, r20
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 83097B14: 55290031  rlwinm. r9, r9, 0, 0, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83097B18: 4082FFE0  bne 0x83097af8
	if !ctx.cr[0].eq {
	pc = 0x83097AF8; continue 'dispatch;
	}
	// 83097B1C: 578A063F  clrlwi. r10, r28, 0x18
	ctx.r[10].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83097B20: 41820080  beq 0x83097ba0
	if ctx.cr[0].eq {
	pc = 0x83097BA0; continue 'dispatch;
	}
	// 83097B24: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83097B28: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83097B2C: 409A000C  bne cr6, 0x83097b38
	if !ctx.cr[6].eq {
	pc = 0x83097B38; continue 'dispatch;
	}
	// 83097B30: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 83097B34: 4800003C  b 0x83097b70
	pc = 0x83097B70; continue 'dispatch;
	// 83097B38: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83097B3C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83097B40: 409A000C  bne cr6, 0x83097b4c
	if !ctx.cr[6].eq {
	pc = 0x83097B4C; continue 'dispatch;
	}
	// 83097B44: 38A00011  li r5, 0x11
	ctx.r[5].s64 = 17;
	// 83097B48: 48000028  b 0x83097b70
	pc = 0x83097B70; continue 'dispatch;
	// 83097B4C: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83097B50: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83097B54: 409A000C  bne cr6, 0x83097b60
	if !ctx.cr[6].eq {
	pc = 0x83097B60; continue 'dispatch;
	}
	// 83097B58: 38A00012  li r5, 0x12
	ctx.r[5].s64 = 18;
	// 83097B5C: 48000014  b 0x83097b70
	pc = 0x83097B70; continue 'dispatch;
	// 83097B60: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83097B64: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83097B68: 409A035C  bne cr6, 0x83097ec4
	if !ctx.cr[6].eq {
	pc = 0x83097EC4; continue 'dispatch;
	}
	// 83097B6C: 38A00013  li r5, 0x13
	ctx.r[5].s64 = 19;
	// 83097B70: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83097B74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83097B78: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83097B7C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83097B80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83097B84: 4E800421  bctrl
	ctx.lr = 0x83097B88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83097B88: 7EDCB378  mr r28, r22
	ctx.r[28].u64 = ctx.r[22].u64;
	// 83097B8C: 570B063F  clrlwi. r11, r24, 0x18
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83097B90: 40820330  bne 0x83097ec0
	if !ctx.cr[0].eq {
	pc = 0x83097EC0; continue 'dispatch;
	}
	// 83097B94: 572B063F  clrlwi. r11, r25, 0x18
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83097B98: 4182028C  beq 0x83097e24
	if ctx.cr[0].eq {
	pc = 0x83097E24; continue 'dispatch;
	}
	// 83097B9C: 48000330  b 0x83097ecc
	pc = 0x83097ECC; continue 'dispatch;
	// 83097BA0: 576A043E  clrlwi r10, r27, 0x10
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0x0000FFFFu64;
	// 83097BA4: 2B0A0028  cmplwi cr6, r10, 0x28
	ctx.cr[6].compare_u32(ctx.r[10].u32, 40 as u32, &mut ctx.xer);
	// 83097BA8: 409A00B0  bne cr6, 0x83097c58
	if !ctx.cr[6].eq {
	pc = 0x83097C58; continue 'dispatch;
	}
	// 83097BAC: 5709063F  clrlwi. r9, r24, 0x18
	ctx.r[9].u64 = ctx.r[24].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83097BB0: 408200A8  bne 0x83097c58
	if !ctx.cr[0].eq {
	pc = 0x83097C58; continue 'dispatch;
	}
	// 83097BB4: 5729063F  clrlwi. r9, r25, 0x18
	ctx.r[9].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83097BB8: 408200A0  bne 0x83097c58
	if !ctx.cr[0].eq {
	pc = 0x83097C58; continue 'dispatch;
	}
	// 83097BBC: 815E0014  lwz r10, 0x14(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83097BC0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83097BC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83097BC8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83097BCC: 409A000C  bne cr6, 0x83097bd8
	if !ctx.cr[6].eq {
	pc = 0x83097BD8; continue 'dispatch;
	}
	// 83097BD0: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 83097BD4: 48000028  b 0x83097bfc
	pc = 0x83097BFC; continue 'dispatch;
	// 83097BD8: 815E0018  lwz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83097BDC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83097BE0: 409A000C  bne cr6, 0x83097bec
	if !ctx.cr[6].eq {
	pc = 0x83097BEC; continue 'dispatch;
	}
	// 83097BE4: 38A0000D  li r5, 0xd
	ctx.r[5].s64 = 13;
	// 83097BE8: 48000014  b 0x83097bfc
	pc = 0x83097BFC; continue 'dispatch;
	// 83097BEC: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83097BF0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83097BF4: 409A0014  bne cr6, 0x83097c08
	if !ctx.cr[6].eq {
	pc = 0x83097C08; continue 'dispatch;
	}
	// 83097BF8: 38A0000E  li r5, 0xe
	ctx.r[5].s64 = 14;
	// 83097BFC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83097C00: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83097C04: 4800001C  b 0x83097c20
	pc = 0x83097C20; continue 'dispatch;
	// 83097C08: 815E0020  lwz r10, 0x20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83097C0C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83097C10: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83097C14: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83097C18: 409A0014  bne cr6, 0x83097c2c
	if !ctx.cr[6].eq {
	pc = 0x83097C2C; continue 'dispatch;
	}
	// 83097C1C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 83097C20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83097C24: 4E800421  bctrl
	ctx.lr = 0x83097C28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83097C28: 48000028  b 0x83097c50
	pc = 0x83097C50; continue 'dispatch;
	// 83097C2C: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 83097C30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83097C34: 4E800421  bctrl
	ctx.lr = 0x83097C38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83097C38: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 83097C3C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83097C40: 4BF4EFC9  bl 0x82fe6c08
	ctx.lr = 0x83097C44;
	sub_82FE6C08(ctx, base);
	// 83097C44: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 83097C48: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83097C4C: 4BF4EFBD  bl 0x82fe6c08
	ctx.lr = 0x83097C50;
	sub_82FE6C08(ctx, base);
	// 83097C50: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83097C54: 4BFFF83C  b 0x83097490
	pc = 0x83097490; continue 'dispatch;
	// 83097C58: 573B063F  clrlwi. r27, r25, 0x18
	ctx.r[27].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 83097C5C: 4082007C  bne 0x83097cd8
	if !ctx.cr[0].eq {
	pc = 0x83097CD8; continue 'dispatch;
	}
	// 83097C60: 2B0A003A  cmplwi cr6, r10, 0x3a
	ctx.cr[6].compare_u32(ctx.r[10].u32, 58 as u32, &mut ctx.xer);
	// 83097C64: 409A0024  bne cr6, 0x83097c88
	if !ctx.cr[6].eq {
	pc = 0x83097C88; continue 'dispatch;
	}
	// 83097C68: 395D0001  addi r10, r29, 1
	ctx.r[10].s64 = ctx.r[29].s64 + 1;
	// 83097C6C: 7F0AA800  cmpw cr6, r10, r21
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[21].s32, &mut ctx.xer);
	// 83097C70: 40980018  bge cr6, 0x83097c88
	if !ctx.cr[6].lt {
	pc = 0x83097C88; continue 'dispatch;
	}
	// 83097C74: 395D0001  addi r10, r29, 1
	ctx.r[10].s64 = ctx.r[29].s64 + 1;
	// 83097C78: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83097C7C: 7D4ABA2E  lhzx r10, r10, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 83097C80: 2B0A003A  cmplwi cr6, r10, 0x3a
	ctx.cr[6].compare_u32(ctx.r[10].u32, 58 as u32, &mut ctx.xer);
	// 83097C84: 419A0054  beq cr6, 0x83097cd8
	if ctx.cr[6].eq {
	pc = 0x83097CD8; continue 'dispatch;
	}
	// 83097C88: 570B063F  clrlwi. r11, r24, 0x18
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83097C8C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83097C90: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83097C94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83097C98: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83097C9C: 41820014  beq 0x83097cb0
	if ctx.cr[0].eq {
	pc = 0x83097CB0; continue 'dispatch;
	}
	// 83097CA0: 38A0000A  li r5, 0xa
	ctx.r[5].s64 = 10;
	// 83097CA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83097CA8: 4E800421  bctrl
	ctx.lr = 0x83097CAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83097CAC: 4800001C  b 0x83097cc8
	pc = 0x83097CC8; continue 'dispatch;
	// 83097CB0: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 83097CB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83097CB8: 4E800421  bctrl
	ctx.lr = 0x83097CBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83097CBC: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 83097CC0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83097CC4: 4BF4EF45  bl 0x82fe6c08
	ctx.lr = 0x83097CC8;
	sub_82FE6C08(ctx, base);
	// 83097CC8: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 83097CCC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83097CD0: 4BF4EF39  bl 0x82fe6c08
	ctx.lr = 0x83097CD4;
	sub_82FE6C08(ctx, base);
	// 83097CD4: 4BFFF8C4  b 0x83097598
	pc = 0x83097598; continue 'dispatch;
	// 83097CD8: 815E0024  lwz r10, 0x24(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83097CDC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83097CE0: 409A000C  bne cr6, 0x83097cec
	if !ctx.cr[6].eq {
	pc = 0x83097CEC; continue 'dispatch;
	}
	// 83097CE4: 38A00021  li r5, 0x21
	ctx.r[5].s64 = 33;
	// 83097CE8: 480000F0  b 0x83097dd8
	pc = 0x83097DD8; continue 'dispatch;
	// 83097CEC: 815E0028  lwz r10, 0x28(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83097CF0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83097CF4: 409A000C  bne cr6, 0x83097d00
	if !ctx.cr[6].eq {
	pc = 0x83097D00; continue 'dispatch;
	}
	// 83097CF8: 38A00022  li r5, 0x22
	ctx.r[5].s64 = 34;
	// 83097CFC: 480000DC  b 0x83097dd8
	pc = 0x83097DD8; continue 'dispatch;
	// 83097D00: 815E002C  lwz r10, 0x2c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83097D04: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83097D08: 409A000C  bne cr6, 0x83097d14
	if !ctx.cr[6].eq {
	pc = 0x83097D14; continue 'dispatch;
	}
	// 83097D0C: 38A00023  li r5, 0x23
	ctx.r[5].s64 = 35;
	// 83097D10: 480000C8  b 0x83097dd8
	pc = 0x83097DD8; continue 'dispatch;
	// 83097D14: 815E0030  lwz r10, 0x30(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83097D18: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83097D1C: 409A000C  bne cr6, 0x83097d28
	if !ctx.cr[6].eq {
	pc = 0x83097D28; continue 'dispatch;
	}
	// 83097D20: 38A00024  li r5, 0x24
	ctx.r[5].s64 = 36;
	// 83097D24: 480000B4  b 0x83097dd8
	pc = 0x83097DD8; continue 'dispatch;
	// 83097D28: 815E0034  lwz r10, 0x34(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83097D2C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83097D30: 409A000C  bne cr6, 0x83097d3c
	if !ctx.cr[6].eq {
	pc = 0x83097D3C; continue 'dispatch;
	}
	// 83097D34: 38A00025  li r5, 0x25
	ctx.r[5].s64 = 37;
	// 83097D38: 480000A0  b 0x83097dd8
	pc = 0x83097DD8; continue 'dispatch;
	// 83097D3C: 815E0038  lwz r10, 0x38(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 83097D40: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83097D44: 409A000C  bne cr6, 0x83097d50
	if !ctx.cr[6].eq {
	pc = 0x83097D50; continue 'dispatch;
	}
	// 83097D48: 38A00026  li r5, 0x26
	ctx.r[5].s64 = 38;
	// 83097D4C: 4800008C  b 0x83097dd8
	pc = 0x83097DD8; continue 'dispatch;
	// 83097D50: 815E003C  lwz r10, 0x3c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 83097D54: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83097D58: 409A000C  bne cr6, 0x83097d64
	if !ctx.cr[6].eq {
	pc = 0x83097D64; continue 'dispatch;
	}
	// 83097D5C: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 83097D60: 48000078  b 0x83097dd8
	pc = 0x83097DD8; continue 'dispatch;
	// 83097D64: 815E0040  lwz r10, 0x40(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83097D68: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83097D6C: 409A000C  bne cr6, 0x83097d78
	if !ctx.cr[6].eq {
	pc = 0x83097D78; continue 'dispatch;
	}
	// 83097D70: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 83097D74: 48000064  b 0x83097dd8
	pc = 0x83097DD8; continue 'dispatch;
	// 83097D78: 815E0044  lwz r10, 0x44(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 83097D7C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83097D80: 409A000C  bne cr6, 0x83097d8c
	if !ctx.cr[6].eq {
	pc = 0x83097D8C; continue 'dispatch;
	}
	// 83097D84: 38A00029  li r5, 0x29
	ctx.r[5].s64 = 41;
	// 83097D88: 48000050  b 0x83097dd8
	pc = 0x83097DD8; continue 'dispatch;
	// 83097D8C: 815E0048  lwz r10, 0x48(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 83097D90: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83097D94: 409A000C  bne cr6, 0x83097da0
	if !ctx.cr[6].eq {
	pc = 0x83097DA0; continue 'dispatch;
	}
	// 83097D98: 38A0002A  li r5, 0x2a
	ctx.r[5].s64 = 42;
	// 83097D9C: 4800003C  b 0x83097dd8
	pc = 0x83097DD8; continue 'dispatch;
	// 83097DA0: 815E004C  lwz r10, 0x4c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 83097DA4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83097DA8: 409A000C  bne cr6, 0x83097db4
	if !ctx.cr[6].eq {
	pc = 0x83097DB4; continue 'dispatch;
	}
	// 83097DAC: 38A0002B  li r5, 0x2b
	ctx.r[5].s64 = 43;
	// 83097DB0: 48000028  b 0x83097dd8
	pc = 0x83097DD8; continue 'dispatch;
	// 83097DB4: 815E0050  lwz r10, 0x50(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 83097DB8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83097DBC: 409A000C  bne cr6, 0x83097dc8
	if !ctx.cr[6].eq {
	pc = 0x83097DC8; continue 'dispatch;
	}
	// 83097DC0: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 83097DC4: 48000014  b 0x83097dd8
	pc = 0x83097DD8; continue 'dispatch;
	// 83097DC8: 815E0054  lwz r10, 0x54(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) } as u64;
	// 83097DCC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83097DD0: 409A00FC  bne cr6, 0x83097ecc
	if !ctx.cr[6].eq {
	pc = 0x83097ECC; continue 'dispatch;
	}
	// 83097DD4: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 83097DD8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83097DDC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83097DE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83097DE4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83097DE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83097DEC: 4E800421  bctrl
	ctx.lr = 0x83097DF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83097DF0: 570B063F  clrlwi. r11, r24, 0x18
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83097DF4: 408200D4  bne 0x83097ec8
	if !ctx.cr[0].eq {
	pc = 0x83097EC8; continue 'dispatch;
	}
	// 83097DF8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83097DFC: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 83097E00: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83097E04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83097E08: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83097E0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83097E10: 4E800421  bctrl
	ctx.lr = 0x83097E14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83097E14: 7EDCB378  mr r28, r22
	ctx.r[28].u64 = ctx.r[22].u64;
	// 83097E18: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 83097E1C: 409A0008  bne cr6, 0x83097e24
	if !ctx.cr[6].eq {
	pc = 0x83097E24; continue 'dispatch;
	}
	// 83097E20: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 83097E24: 7F1DA800  cmpw cr6, r29, r21
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[21].s32, &mut ctx.xer);
	// 83097E28: 409AF590  bne cr6, 0x830973b8
	if !ctx.cr[6].eq {
	pc = 0x830973B8; continue 'dispatch;
	}
	// 83097E2C: 807F008C  lwz r3, 0x8c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 83097E30: 809F0098  lwz r4, 0x98(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 83097E34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83097E38: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83097E3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83097E40: 4E800421  bctrl
	ctx.lr = 0x83097E44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83097E44: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83097E48: 480000A0  b 0x83097ee8
	pc = 0x83097EE8; continue 'dispatch;
	// 83097E4C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83097E50: 80FA0010  lwz r7, 0x10(r26)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 83097E54: 38C0013E  li r6, 0x13e
	ctx.r[6].s64 = 318;
	// 83097E58: 388BD458  addi r4, r11, -0x2ba8
	ctx.r[4].s64 = ctx.r[11].s64 + -11176;
	// 83097E5C: 38A0037D  li r5, 0x37d
	ctx.r[5].s64 = 893;
	// 83097E60: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83097E64: 4BFFDFB5  bl 0x83095e18
	ctx.lr = 0x83097E68;
	sub_83095E18(ctx, base);
	// 83097E68: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83097E6C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83097E70: 388BCA78  addi r4, r11, -0x3588
	ctx.r[4].s64 = ctx.r[11].s64 + -13704;
	// 83097E74: 48118DB5  bl 0x831b0c28
	ctx.lr = 0x83097E78;
	sub_831B0C28(ctx, base);
	// 83097E78: 807F008C  lwz r3, 0x8c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 83097E7C: 809F0098  lwz r4, 0x98(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 83097E80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83097E84: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83097E88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83097E8C: 48000054  b 0x83097ee0
	pc = 0x83097EE0; continue 'dispatch;
	// 83097E90: 4800003C  b 0x83097ecc
	pc = 0x83097ECC; continue 'dispatch;
	// 83097E94: 48000038  b 0x83097ecc
	pc = 0x83097ECC; continue 'dispatch;
	// 83097E98: 48000034  b 0x83097ecc
	pc = 0x83097ECC; continue 'dispatch;
	// 83097E9C: 48000030  b 0x83097ecc
	pc = 0x83097ECC; continue 'dispatch;
	// 83097EA0: 4800002C  b 0x83097ecc
	pc = 0x83097ECC; continue 'dispatch;
	// 83097EA4: 48000028  b 0x83097ecc
	pc = 0x83097ECC; continue 'dispatch;
	// 83097EA8: 48000024  b 0x83097ecc
	pc = 0x83097ECC; continue 'dispatch;
	// 83097EAC: 48000020  b 0x83097ecc
	pc = 0x83097ECC; continue 'dispatch;
	// 83097EB0: 4800001C  b 0x83097ecc
	pc = 0x83097ECC; continue 'dispatch;
	// 83097EB4: 48000018  b 0x83097ecc
	pc = 0x83097ECC; continue 'dispatch;
	// 83097EB8: 48000014  b 0x83097ecc
	pc = 0x83097ECC; continue 'dispatch;
	// 83097EBC: 48000010  b 0x83097ecc
	pc = 0x83097ECC; continue 'dispatch;
	// 83097EC0: 4800000C  b 0x83097ecc
	pc = 0x83097ECC; continue 'dispatch;
	// 83097EC4: 48000008  b 0x83097ecc
	pc = 0x83097ECC; continue 'dispatch;
	// 83097EC8: 48000004  b 0x83097ecc
	pc = 0x83097ECC; continue 'dispatch;
	// 83097ECC: 807F008C  lwz r3, 0x8c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 83097ED0: 809F0098  lwz r4, 0x98(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 83097ED4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83097ED8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83097EDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83097EE0: 4E800421  bctrl
	ctx.lr = 0x83097EE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83097EE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83097EE8: 383F0120  addi r1, r31, 0x120
	ctx.r[1].s64 = ctx.r[31].s64 + 288;
	// 83097EEC: 481102A4  b 0x831a8190
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83097EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83097EF0 size=40
    let mut pc: u32 = 0x83097EF0;
    'dispatch: loop {
        match pc {
            0x83097EF0 => {
    //   block [0x83097EF0..0x83097F18)
	// 83097EF0: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 83097EF4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83097EF8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83097EFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83097F00: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83097F04: 4BF46FD5  bl 0x82fdeed8
	ctx.lr = 0x83097F08;
	sub_82FDEED8(ctx, base);
	// 83097F08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83097F0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83097F10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83097F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83097F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83097F18 size=8
    let mut pc: u32 = 0x83097F18;
    'dispatch: loop {
        match pc {
            0x83097F18 => {
    //   block [0x83097F18..0x83097F20)
	// 83097F18: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83097F1C: 8216D5B0  lwz r16, -0x2a50(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-10832 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83097F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83097F20 size=2960
    let mut pc: u32 = 0x83097F20;
    'dispatch: loop {
        match pc {
            0x83097F20 => {
    //   block [0x83097F20..0x83098AB0)
	// 83097F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83097F24: 4811020D  bl 0x831a8130
	ctx.lr = 0x83097F28;
	sub_831A8130(ctx, base);
	// 83097F28: 3BE1FC80  addi r31, r1, -0x380
	ctx.r[31].s64 = ctx.r[1].s64 + -896;
	// 83097F2C: 9421FC80  stwu r1, -0x380(r1)
	ea = ctx.r[1].u32.wrapping_add(-896 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83097F30: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83097F34: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 83097F38: 7CB02B78  mr r16, r5
	ctx.r[16].u64 = ctx.r[5].u64;
	// 83097F3C: 3A200000  li r17, 0
	ctx.r[17].s64 = 0;
	// 83097F40: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83097F44: 93DF0394  stw r30, 0x394(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(916 as u32), ctx.r[30].u32 ) };
	// 83097F48: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83097F4C: 41820034  beq 0x83097f80
	if ctx.cr[0].eq {
	pc = 0x83097F80; continue 'dispatch;
	}
	// 83097F50: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83097F54: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83097F58: 41820028  beq 0x83097f80
	if ctx.cr[0].eq {
	pc = 0x83097F80; continue 'dispatch;
	}
	// 83097F5C: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 83097F60: 48000008  b 0x83097f68
	pc = 0x83097F68; continue 'dispatch;
	// 83097F64: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83097F68: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83097F6C: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83097F70: 4082FFF4  bne 0x83097f64
	if !ctx.cr[0].eq {
	pc = 0x83097F64; continue 'dispatch;
	}
	// 83097F74: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 83097F78: 7D7D0E70  srawi r29, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83097F7C: 48000008  b 0x83097f84
	pc = 0x83097F84; continue 'dispatch;
	// 83097F80: 7E3D8B78  mr r29, r17
	ctx.r[29].u64 = ctx.r[17].u64;
	// 83097F84: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83097F88: 419A0B20  beq cr6, 0x83098aa8
	if ctx.cr[6].eq {
	pc = 0x83098AA8; continue 'dispatch;
	}
	// 83097F8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83097F90: 80BE0010  lwz r5, 0x10(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83097F94: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 83097F98: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83097F9C: 4BFD0805  bl 0x830687a0
	ctx.lr = 0x83097FA0;
	sub_830687A0(ctx, base);
	// 83097FA0: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 83097FA4: 387F0280  addi r3, r31, 0x280
	ctx.r[3].s64 = ctx.r[31].s64 + 640;
	// 83097FA8: 4BFFF069  bl 0x83097010
	ctx.lr = 0x83097FAC;
	sub_83097010(ctx, base);
	// 83097FAC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83097FB0: 396BD4A0  addi r11, r11, -0x2b60
	ctx.r[11].s64 = ctx.r[11].s64 + -11104;
	// 83097FB4: 917F0280  stw r11, 0x280(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(640 as u32), ctx.r[11].u32 ) };
	// 83097FB8: 38FF0070  addi r7, r31, 0x70
	ctx.r[7].s64 = ctx.r[31].s64 + 112;
	// 83097FBC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83097FC0: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83097FC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83097FC8: 387F0280  addi r3, r31, 0x280
	ctx.r[3].s64 = ctx.r[31].s64 + 640;
	// 83097FCC: 4BFFF395  bl 0x83097360
	ctx.lr = 0x83097FD0;
	sub_83097360(ctx, base);
	// 83097FD0: 39C00001  li r14, 1
	ctx.r[14].s64 = 1;
	// 83097FD4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83097FD8: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83097FDC: 825F0074  lwz r18, 0x74(r31)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 83097FE0: 7DD97378  mr r25, r14
	ctx.r[25].u64 = ctx.r[14].u64;
	// 83097FE4: 4BF402B5  bl 0x82fd8298
	ctx.lr = 0x83097FE8;
	sub_82FD8298(ctx, base);
	// 83097FE8: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83097FEC: 93BF0064  stw r29, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 83097FF0: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83097FF4: 3B8B2990  addi r28, r11, 0x2990
	ctx.r[28].s64 = ctx.r[11].s64 + 10640;
	// 83097FF8: 939F0084  stw r28, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[28].u32 ) };
	// 83097FFC: 41820024  beq 0x83098020
	if ctx.cr[0].eq {
	pc = 0x83098020; continue 'dispatch;
	}
	// 83098000: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83098004: 80DE0010  lwz r6, 0x10(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098008: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 8309800C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83098010: 4BFB47E1  bl 0x8304c7f0
	ctx.lr = 0x83098014;
	sub_8304C7F0(ctx, base);
	// 83098014: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 83098018: 939D0000  stw r28, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8309801C: 48000008  b 0x83098024
	pc = 0x83098024; continue 'dispatch;
	// 83098020: 7E2B8B78  mr r11, r17
	ctx.r[11].u64 = ctx.r[17].u64;
	// 83098024: 7D755B78  mr r21, r11
	ctx.r[21].u64 = ctx.r[11].u64;
	// 83098028: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8309802C: 2B120000  cmplwi cr6, r18, 0
	ctx.cr[6].compare_u32(ctx.r[18].u32, 0 as u32, &mut ctx.xer);
	// 83098030: 419A004C  beq cr6, 0x8309807c
	if ctx.cr[6].eq {
	pc = 0x8309807C; continue 'dispatch;
	}
	// 83098034: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83098038: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309803C: 4BF4025D  bl 0x82fd8298
	ctx.lr = 0x83098040;
	sub_82FD8298(ctx, base);
	// 83098040: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83098044: 93BF0060  stw r29, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 83098048: 4182002C  beq 0x83098074
	if ctx.cr[0].eq {
	pc = 0x83098074; continue 'dispatch;
	}
	// 8309804C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83098050: 80DE0010  lwz r6, 0x10(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098054: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 83098058: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309805C: 4BFB4795  bl 0x8304c7f0
	ctx.lr = 0x83098060;
	sub_8304C7F0(ctx, base);
	// 83098060: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83098064: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 83098068: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 8309806C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83098070: 48000008  b 0x83098078
	pc = 0x83098078; continue 'dispatch;
	// 83098074: 7E2A8B78  mr r10, r17
	ctx.r[10].u64 = ctx.r[17].u64;
	// 83098078: 915E000C  stw r10, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8309807C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83098080: 7E388B78  mr r24, r17
	ctx.r[24].u64 = ctx.r[17].u64;
	// 83098084: 2B120000  cmplwi cr6, r18, 0
	ctx.cr[6].compare_u32(ctx.r[18].u32, 0 as u32, &mut ctx.xer);
	// 83098088: 39EBD3C8  addi r15, r11, -0x2c38
	ctx.r[15].s64 = ctx.r[11].s64 + -11320;
	// 8309808C: 419A06D8  beq cr6, 0x83098764
	if ctx.cr[6].eq {
	pc = 0x83098764; continue 'dispatch;
	}
	// 83098090: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83098094: 3A600002  li r19, 2
	ctx.r[19].s64 = 2;
	// 83098098: 3AEBD380  addi r23, r11, -0x2c80
	ctx.r[23].s64 = ctx.r[11].s64 + -11392;
	// 8309809C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830980A0: 3A8B8158  addi r20, r11, -0x7ea8
	ctx.r[20].s64 = ctx.r[11].s64 + -32424;
	// 830980A4: 48000008  b 0x830980ac
	pc = 0x830980AC; continue 'dispatch;
	// 830980A8: 839F0084  lwz r28, 0x84(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830980AC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830980B0: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 830980B4: 4BF6001D  bl 0x82ff80d0
	ctx.lr = 0x830980B8;
	sub_82FF80D0(ctx, base);
	// 830980B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830980BC: 7E3A8B78  mr r26, r17
	ctx.r[26].u64 = ctx.r[17].u64;
	// 830980C0: 2F0B0023  cmpwi cr6, r11, 0x23
	ctx.cr[6].compare_i32(ctx.r[11].s32, 35, &mut ctx.xer);
	// 830980C4: 41990678  bgt cr6, 0x8309873c
	if ctx.cr[6].gt {
	pc = 0x8309873C; continue 'dispatch;
	}
	// 830980C8: 419A0454  beq cr6, 0x8309851c
	if ctx.cr[6].eq {
	pc = 0x8309851C; continue 'dispatch;
	}
	// 830980CC: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 830980D0: 4199025C  bgt cr6, 0x8309832c
	if ctx.cr[6].gt {
	pc = 0x8309832C; continue 'dispatch;
	}
	// 830980D4: 419A01A0  beq cr6, 0x83098274
	if ctx.cr[6].eq {
	pc = 0x83098274; continue 'dispatch;
	}
	// 830980D8: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 830980DC: 419A0084  beq cr6, 0x83098160
	if ctx.cr[6].eq {
	pc = 0x83098160; continue 'dispatch;
	}
	// 830980E0: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 830980E4: 419A043C  beq cr6, 0x83098520
	if ctx.cr[6].eq {
	pc = 0x83098520; continue 'dispatch;
	}
	// 830980E8: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 830980EC: 419A06C8  beq cr6, 0x830987b4
	if ctx.cr[6].eq {
	pc = 0x830987B4; continue 'dispatch;
	}
	// 830980F0: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 830980F4: 409A0660  bne cr6, 0x83098754
	if !ctx.cr[6].eq {
	pc = 0x83098754; continue 'dispatch;
	}
	// 830980F8: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 830980FC: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098100: 4BF40199  bl 0x82fd8298
	ctx.lr = 0x83098104;
	sub_82FD8298(ctx, base);
	// 83098104: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83098108: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309810C: 41820018  beq 0x83098124
	if ctx.cr[0].eq {
	pc = 0x83098124; continue 'dispatch;
	}
	// 83098110: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83098114: 80BE0010  lwz r5, 0x10(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098118: 4BFFDF61  bl 0x83096078
	ctx.lr = 0x8309811C;
	sub_83096078(ctx, base);
	// 8309811C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83098120: 48000008  b 0x83098128
	pc = 0x83098128; continue 'dispatch;
	// 83098124: 7E3D8B78  mr r29, r17
	ctx.r[29].u64 = ctx.r[17].u64;
	// 83098128: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8309812C: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098130: 4BF40169  bl 0x82fd8298
	ctx.lr = 0x83098134;
	sub_82FD8298(ctx, base);
	// 83098134: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83098138: 41820018  beq 0x83098150
	if ctx.cr[0].eq {
	pc = 0x83098150; continue 'dispatch;
	}
	// 8309813C: B1C30004  sth r14, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[14].u16 ) };
	// 83098140: 92E30000  stw r23, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 83098144: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83098148: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8309814C: 48000008  b 0x83098154
	pc = 0x83098154; continue 'dispatch;
	// 83098150: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 83098154: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83098158: 4BFA2FF9  bl 0x8303b150
	ctx.lr = 0x8309815C;
	sub_8303B150(ctx, base);
	// 8309815C: 480005F8  b 0x83098754
	pc = 0x83098754; continue 'dispatch;
	// 83098160: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 83098164: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098168: 4BF40131  bl 0x82fd8298
	ctx.lr = 0x8309816C;
	sub_82FD8298(ctx, base);
	// 8309816C: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83098170: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83098174: 41820018  beq 0x8309818c
	if ctx.cr[0].eq {
	pc = 0x8309818C; continue 'dispatch;
	}
	// 83098178: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8309817C: 80BE0010  lwz r5, 0x10(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098180: 4BFFDEF9  bl 0x83096078
	ctx.lr = 0x83098184;
	sub_83096078(ctx, base);
	// 83098184: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83098188: 48000008  b 0x83098190
	pc = 0x83098190; continue 'dispatch;
	// 8309818C: 7E3D8B78  mr r29, r17
	ctx.r[29].u64 = ctx.r[17].u64;
	// 83098190: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 83098194: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098198: 4BF40101  bl 0x82fd8298
	ctx.lr = 0x8309819C;
	sub_82FD8298(ctx, base);
	// 8309819C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830981A0: 4182001C  beq 0x830981bc
	if ctx.cr[0].eq {
	pc = 0x830981BC; continue 'dispatch;
	}
	// 830981A4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 830981A8: 92E30000  stw r23, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 830981AC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830981B0: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 830981B4: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 830981B8: 48000008  b 0x830981c0
	pc = 0x830981C0; continue 'dispatch;
	// 830981BC: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 830981C0: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 830981C4: 4BFA2F8D  bl 0x8303b150
	ctx.lr = 0x830981C8;
	sub_8303B150(ctx, base);
	// 830981C8: 572B063F  clrlwi. r11, r25, 0x18
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830981CC: 41820588  beq 0x83098754
	if ctx.cr[0].eq {
	pc = 0x83098754; continue 'dispatch;
	}
	// 830981D0: 3BB80001  addi r29, r24, 1
	ctx.r[29].s64 = ctx.r[24].s64 + 1;
	// 830981D4: 7F1D9040  cmplw cr6, r29, r18
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[18].u32, &mut ctx.xer);
	// 830981D8: 4098057C  bge cr6, 0x83098754
	if !ctx.cr[6].lt {
	pc = 0x83098754; continue 'dispatch;
	}
	// 830981DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830981E0: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 830981E4: 4BF5FEED  bl 0x82ff80d0
	ctx.lr = 0x830981E8;
	sub_82FF80D0(ctx, base);
	// 830981E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830981EC: 2F0B0016  cmpwi cr6, r11, 0x16
	ctx.cr[6].compare_i32(ctx.r[11].s32, 22, &mut ctx.xer);
	// 830981F0: 409A0564  bne cr6, 0x83098754
	if !ctx.cr[6].eq {
	pc = 0x83098754; continue 'dispatch;
	}
	// 830981F4: 7FB8EB78  mr r24, r29
	ctx.r[24].u64 = ctx.r[29].u64;
	// 830981F8: 3972FFFF  addi r11, r18, -1
	ctx.r[11].s64 = ctx.r[18].s64 + -1;
	// 830981FC: 7F185840  cmplw cr6, r24, r11
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83098200: 419A05E0  beq cr6, 0x830987e0
	if ctx.cr[6].eq {
	pc = 0x830987E0; continue 'dispatch;
	}
	// 83098204: 38980001  addi r4, r24, 1
	ctx.r[4].s64 = ctx.r[24].s64 + 1;
	// 83098208: 7F049040  cmplw cr6, r4, r18
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[18].u32, &mut ctx.xer);
	// 8309820C: 40980018  bge cr6, 0x83098224
	if !ctx.cr[6].lt {
	pc = 0x83098224; continue 'dispatch;
	}
	// 83098210: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83098214: 4BF5FEBD  bl 0x82ff80d0
	ctx.lr = 0x83098218;
	sub_82FF80D0(ctx, base);
	// 83098218: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309821C: 2F0B0015  cmpwi cr6, r11, 0x15
	ctx.cr[6].compare_i32(ctx.r[11].s32, 21, &mut ctx.xer);
	// 83098220: 419A05EC  beq cr6, 0x8309880c
	if ctx.cr[6].eq {
	pc = 0x8309880C; continue 'dispatch;
	}
	// 83098224: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 83098228: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309822C: 4BF4006D  bl 0x82fd8298
	ctx.lr = 0x83098230;
	sub_82FD8298(ctx, base);
	// 83098230: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83098234: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83098238: 41820018  beq 0x83098250
	if ctx.cr[0].eq {
	pc = 0x83098250; continue 'dispatch;
	}
	// 8309823C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83098240: 80BE0010  lwz r5, 0x10(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098244: 4BFFDE35  bl 0x83096078
	ctx.lr = 0x83098248;
	sub_83096078(ctx, base);
	// 83098248: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8309824C: 48000008  b 0x83098254
	pc = 0x83098254; continue 'dispatch;
	// 83098250: 7E3D8B78  mr r29, r17
	ctx.r[29].u64 = ctx.r[17].u64;
	// 83098254: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 83098258: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309825C: 4BF4003D  bl 0x82fd8298
	ctx.lr = 0x83098260;
	sub_82FD8298(ctx, base);
	// 83098260: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83098264: 4182FEEC  beq 0x83098150
	if ctx.cr[0].eq {
	pc = 0x83098150; continue 'dispatch;
	}
	// 83098268: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8309826C: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 83098270: 4BFFFED0  b 0x83098140
	pc = 0x83098140; continue 'dispatch;
	// 83098274: 7DDA7378  mr r26, r14
	ctx.r[26].u64 = ctx.r[14].u64;
	// 83098278: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 8309827C: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83098280: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83098284: 4BF5FE4D  bl 0x82ff80d0
	ctx.lr = 0x83098288;
	sub_82FF80D0(ctx, base);
	// 83098288: 83630000  lwz r27, 0(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309828C: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83098290: 2B100000  cmplwi cr6, r16, 0
	ctx.cr[6].compare_u32(ctx.r[16].u32, 0 as u32, &mut ctx.xer);
	// 83098294: 7E9DA378  mr r29, r20
	ctx.r[29].u64 = ctx.r[20].u64;
	// 83098298: 419A0040  beq cr6, 0x830982d8
	if ctx.cr[6].eq {
	pc = 0x830982D8; continue 'dispatch;
	}
	// 8309829C: 2F1BFFFF  cmpwi cr6, r27, -1
	ctx.cr[6].compare_i32(ctx.r[27].s32, -1, &mut ctx.xer);
	// 830982A0: 419A0054  beq cr6, 0x830982f4
	if ctx.cr[6].eq {
	pc = 0x830982F4; continue 'dispatch;
	}
	// 830982A4: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 830982A8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830982AC: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 830982B0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 830982B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830982B8: 4E800421  bctrl
	ctx.lr = 0x830982BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830982BC: 81700008  lwz r11, 8(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(8 as u32) ) } as u64;
	// 830982C0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830982C4: 7E038378  mr r3, r16
	ctx.r[3].u64 = ctx.r[16].u64;
	// 830982C8: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 830982CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830982D0: 48001951  bl 0x83099c20
	ctx.lr = 0x830982D4;
	sub_83099C20(ctx, base);
	// 830982D4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830982D8: 2F1BFFFF  cmpwi cr6, r27, -1
	ctx.cr[6].compare_i32(ctx.r[27].s32, -1, &mut ctx.xer);
	// 830982DC: 419A0018  beq cr6, 0x830982f4
	if ctx.cr[6].eq {
	pc = 0x830982F4; continue 'dispatch;
	}
	// 830982E0: 2B100000  cmplwi cr6, r16, 0
	ctx.cr[6].compare_u32(ctx.r[16].u32, 0 as u32, &mut ctx.xer);
	// 830982E4: 419A0010  beq cr6, 0x830982f4
	if ctx.cr[6].eq {
	pc = 0x830982F4; continue 'dispatch;
	}
	// 830982E8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830982EC: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830982F0: 419A0624  beq cr6, 0x83098914
	if ctx.cr[6].eq {
	pc = 0x83098914; continue 'dispatch;
	}
	// 830982F4: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830982F8: 41820170  beq 0x83098468
	if ctx.cr[0].eq {
	pc = 0x83098468; continue 'dispatch;
	}
	// 830982FC: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 83098300: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098304: 4BF3FF95  bl 0x82fd8298
	ctx.lr = 0x83098308;
	sub_82FD8298(ctx, base);
	// 83098308: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 8309830C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83098310: 4182011C  beq 0x8309842c
	if ctx.cr[0].eq {
	pc = 0x8309842C; continue 'dispatch;
	}
	// 83098314: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83098318: 80DE0010  lwz r6, 0x10(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309831C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83098320: 4BFFDFD9  bl 0x830962f8
	ctx.lr = 0x83098324;
	sub_830962F8(ctx, base);
	// 83098324: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83098328: 48000108  b 0x83098430
	pc = 0x83098430; continue 'dispatch;
	// 8309832C: 2F0B000B  cmpwi cr6, r11, 0xb
	ctx.cr[6].compare_i32(ctx.r[11].s32, 11, &mut ctx.xer);
	// 83098330: 419AFF48  beq cr6, 0x83098278
	if ctx.cr[6].eq {
	pc = 0x83098278; continue 'dispatch;
	}
	// 83098334: 2F0B0015  cmpwi cr6, r11, 0x15
	ctx.cr[6].compare_i32(ctx.r[11].s32, 21, &mut ctx.xer);
	// 83098338: 419A00AC  beq cr6, 0x830983e4
	if ctx.cr[6].eq {
	pc = 0x830983E4; continue 'dispatch;
	}
	// 8309833C: 2F0B0016  cmpwi cr6, r11, 0x16
	ctx.cr[6].compare_i32(ctx.r[11].s32, 22, &mut ctx.xer);
	// 83098340: 419A0550  beq cr6, 0x83098890
	if ctx.cr[6].eq {
	pc = 0x83098890; continue 'dispatch;
	}
	// 83098344: 2F0B0017  cmpwi cr6, r11, 0x17
	ctx.cr[6].compare_i32(ctx.r[11].s32, 23, &mut ctx.xer);
	// 83098348: 409A040C  bne cr6, 0x83098754
	if !ctx.cr[6].eq {
	pc = 0x83098754; continue 'dispatch;
	}
	// 8309834C: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 83098350: 419A04E8  beq cr6, 0x83098838
	if ctx.cr[6].eq {
	pc = 0x83098838; continue 'dispatch;
	}
	// 83098354: 81750008  lwz r11, 8(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(8 as u32) ) } as u64;
	// 83098358: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8309835C: 41820508  beq 0x83098864
	if ctx.cr[0].eq {
	pc = 0x83098864; continue 'dispatch;
	}
	// 83098360: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 83098364: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098368: 4BF3FF31  bl 0x82fd8298
	ctx.lr = 0x8309836C;
	sub_82FD8298(ctx, base);
	// 8309836C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83098370: 41820014  beq 0x83098384
	if ctx.cr[0].eq {
	pc = 0x83098384; continue 'dispatch;
	}
	// 83098374: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83098378: 92A30004  stw r21, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[21].u32 ) };
	// 8309837C: 91E30000  stw r15, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[15].u32 ) };
	// 83098380: 48000008  b 0x83098388
	pc = 0x83098388; continue 'dispatch;
	// 83098384: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 83098388: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309838C: 4BFA2DC5  bl 0x8303b150
	ctx.lr = 0x83098390;
	sub_8303B150(ctx, base);
	// 83098390: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83098394: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098398: 923F0064  stw r17, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[17].u32 ) };
	// 8309839C: 4BF3FEFD  bl 0x82fd8298
	ctx.lr = 0x830983A0;
	sub_82FD8298(ctx, base);
	// 830983A0: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830983A4: 93BF0060  stw r29, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 830983A8: 41820024  beq 0x830983cc
	if ctx.cr[0].eq {
	pc = 0x830983CC; continue 'dispatch;
	}
	// 830983AC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830983B0: 80DE0010  lwz r6, 0x10(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830983B4: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 830983B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830983BC: 4BFB4435  bl 0x8304c7f0
	ctx.lr = 0x830983C0;
	sub_8304C7F0(ctx, base);
	// 830983C0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830983C4: 939D0000  stw r28, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830983C8: 48000008  b 0x830983d0
	pc = 0x830983D0; continue 'dispatch;
	// 830983CC: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 830983D0: 387F0064  addi r3, r31, 0x64
	ctx.r[3].s64 = ctx.r[31].s64 + 100;
	// 830983D4: 7C952378  mr r21, r4
	ctx.r[21].u64 = ctx.r[4].u64;
	// 830983D8: 4BFD0901  bl 0x83068cd8
	ctx.lr = 0x830983DC;
	sub_83068CD8(ctx, base);
	// 830983DC: 7DD97378  mr r25, r14
	ctx.r[25].u64 = ctx.r[14].u64;
	// 830983E0: 48000378  b 0x83098758
	pc = 0x83098758; continue 'dispatch;
	// 830983E4: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 830983E8: 419A04D4  beq cr6, 0x830988bc
	if ctx.cr[6].eq {
	pc = 0x830988BC; continue 'dispatch;
	}
	// 830983EC: 572B063F  clrlwi. r11, r25, 0x18
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830983F0: 408204F8  bne 0x830988e8
	if !ctx.cr[0].eq {
	pc = 0x830988E8; continue 'dispatch;
	}
	// 830983F4: 3972FFFF  addi r11, r18, -1
	ctx.r[11].s64 = ctx.r[18].s64 + -1;
	// 830983F8: 7F185840  cmplw cr6, r24, r11
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830983FC: 409A0358  bne cr6, 0x83098754
	if !ctx.cr[6].eq {
	pc = 0x83098754; continue 'dispatch;
	}
	// 83098400: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83098404: 80FE0010  lwz r7, 0x10(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098408: 38C00137  li r6, 0x137
	ctx.r[6].s64 = 311;
	// 8309840C: 388BD458  addi r4, r11, -0x2ba8
	ctx.r[4].s64 = ctx.r[11].s64 + -11176;
	// 83098410: 38A00296  li r5, 0x296
	ctx.r[5].s64 = 662;
	// 83098414: 387F0220  addi r3, r31, 0x220
	ctx.r[3].s64 = ctx.r[31].s64 + 544;
	// 83098418: 4BFFDA01  bl 0x83095e18
	ctx.lr = 0x8309841C;
	sub_83095E18(ctx, base);
	// 8309841C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83098420: 387F0220  addi r3, r31, 0x220
	ctx.r[3].s64 = ctx.r[31].s64 + 544;
	// 83098424: 388BCA78  addi r4, r11, -0x3588
	ctx.r[4].s64 = ctx.r[11].s64 + -13704;
	// 83098428: 48118801  bl 0x831b0c28
	ctx.lr = 0x8309842C;
	sub_831B0C28(ctx, base);
	// 8309842C: 7E3D8B78  mr r29, r17
	ctx.r[29].u64 = ctx.r[17].u64;
	// 83098430: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 83098434: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098438: 4BF3FE61  bl 0x82fd8298
	ctx.lr = 0x8309843C;
	sub_82FD8298(ctx, base);
	// 8309843C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83098440: 41820018  beq 0x83098458
	if ctx.cr[0].eq {
	pc = 0x83098458; continue 'dispatch;
	}
	// 83098444: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83098448: B1C30004  sth r14, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[14].u16 ) };
	// 8309844C: 92E30000  stw r23, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 83098450: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 83098454: 48000008  b 0x8309845c
	pc = 0x8309845C; continue 'dispatch;
	// 83098458: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8309845C: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83098460: 4BFA2CF1  bl 0x8303b150
	ctx.lr = 0x83098464;
	sub_8303B150(ctx, base);
	// 83098464: 480002F4  b 0x83098758
	pc = 0x83098758; continue 'dispatch;
	// 83098468: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 8309846C: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83098470: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83098474: 4BF5FC5D  bl 0x82ff80d0
	ctx.lr = 0x83098478;
	sub_82FF80D0(ctx, base);
	// 83098478: 81560000  lwz r10, 0(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309847C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83098480: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 83098484: 814A0024  lwz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 83098488: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309848C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83098490: 4E800421  bctrl
	ctx.lr = 0x83098494;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83098494: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83098498: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8309849C: 80FE0010  lwz r7, 0x10(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830984A0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830984A4: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 830984A8: 4BF46781  bl 0x82fdec28
	ctx.lr = 0x830984AC;
	sub_82FDEC28(ctx, base);
	// 830984AC: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 830984B0: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830984B4: 4BF3FDE5  bl 0x82fd8298
	ctx.lr = 0x830984B8;
	sub_82FD8298(ctx, base);
	// 830984B8: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 830984BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830984C0: 41820014  beq 0x830984d4
	if ctx.cr[0].eq {
	pc = 0x830984D4; continue 'dispatch;
	}
	// 830984C4: 389F00B0  addi r4, r31, 0xb0
	ctx.r[4].s64 = ctx.r[31].s64 + 176;
	// 830984C8: 4BFFDD69  bl 0x83096230
	ctx.lr = 0x830984CC;
	sub_83096230(ctx, base);
	// 830984CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830984D0: 48000008  b 0x830984d8
	pc = 0x830984D8; continue 'dispatch;
	// 830984D4: 7E3D8B78  mr r29, r17
	ctx.r[29].u64 = ctx.r[17].u64;
	// 830984D8: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 830984DC: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830984E0: 4BF3FDB9  bl 0x82fd8298
	ctx.lr = 0x830984E4;
	sub_82FD8298(ctx, base);
	// 830984E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830984E8: 41820018  beq 0x83098500
	if ctx.cr[0].eq {
	pc = 0x83098500; continue 'dispatch;
	}
	// 830984EC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830984F0: B1C30004  sth r14, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[14].u16 ) };
	// 830984F4: 92E30000  stw r23, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 830984F8: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 830984FC: 48000008  b 0x83098504
	pc = 0x83098504; continue 'dispatch;
	// 83098500: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 83098504: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83098508: 4BFA2C49  bl 0x8303b150
	ctx.lr = 0x8309850C;
	sub_8303B150(ctx, base);
	// 8309850C: 7E398B78  mr r25, r17
	ctx.r[25].u64 = ctx.r[17].u64;
	// 83098510: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 83098514: 4BF464DD  bl 0x82fde9f0
	ctx.lr = 0x83098518;
	sub_82FDE9F0(ctx, base);
	// 83098518: 48000240  b 0x83098758
	pc = 0x83098758; continue 'dispatch;
	// 8309851C: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 83098520: 3972FFFF  addi r11, r18, -1
	ctx.r[11].s64 = ctx.r[18].s64 + -1;
	// 83098524: 7F185840  cmplw cr6, r24, r11
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83098528: 419A042C  beq cr6, 0x83098954
	if ctx.cr[6].eq {
	pc = 0x83098954; continue 'dispatch;
	}
	// 8309852C: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 83098530: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83098534: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83098538: 4BF5FB99  bl 0x82ff80d0
	ctx.lr = 0x8309853C;
	sub_82FF80D0(ctx, base);
	// 8309853C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83098540: 2F0B000B  cmpwi cr6, r11, 0xb
	ctx.cr[6].compare_i32(ctx.r[11].s32, 11, &mut ctx.xer);
	// 83098544: 419A0014  beq cr6, 0x83098558
	if ctx.cr[6].eq {
	pc = 0x83098558; continue 'dispatch;
	}
	// 83098548: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 8309854C: 419A000C  beq cr6, 0x83098558
	if ctx.cr[6].eq {
	pc = 0x83098558; continue 'dispatch;
	}
	// 83098550: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 83098554: 409A042C  bne cr6, 0x83098980
	if !ctx.cr[6].eq {
	pc = 0x83098980; continue 'dispatch;
	}
	// 83098558: 7E3A8B78  mr r26, r17
	ctx.r[26].u64 = ctx.r[17].u64;
	// 8309855C: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 83098560: 419A0190  beq cr6, 0x830986f0
	if ctx.cr[6].eq {
	pc = 0x830986F0; continue 'dispatch;
	}
	// 83098564: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 83098568: 419A0010  beq cr6, 0x83098578
	if ctx.cr[6].eq {
	pc = 0x83098578; continue 'dispatch;
	}
	// 8309856C: 2F0B000B  cmpwi cr6, r11, 0xb
	ctx.cr[6].compare_i32(ctx.r[11].s32, 11, &mut ctx.xer);
	// 83098570: 419A000C  beq cr6, 0x8309857c
	if ctx.cr[6].eq {
	pc = 0x8309857C; continue 'dispatch;
	}
	// 83098574: 480001E0  b 0x83098754
	pc = 0x83098754; continue 'dispatch;
	// 83098578: 7DDA7378  mr r26, r14
	ctx.r[26].u64 = ctx.r[14].u64;
	// 8309857C: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 83098580: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83098584: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83098588: 4BF5FB49  bl 0x82ff80d0
	ctx.lr = 0x8309858C;
	sub_82FF80D0(ctx, base);
	// 8309858C: 83630000  lwz r27, 0(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83098590: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83098594: 2B100000  cmplwi cr6, r16, 0
	ctx.cr[6].compare_u32(ctx.r[16].u32, 0 as u32, &mut ctx.xer);
	// 83098598: 7E9DA378  mr r29, r20
	ctx.r[29].u64 = ctx.r[20].u64;
	// 8309859C: 419A0040  beq cr6, 0x830985dc
	if ctx.cr[6].eq {
	pc = 0x830985DC; continue 'dispatch;
	}
	// 830985A0: 2F1BFFFF  cmpwi cr6, r27, -1
	ctx.cr[6].compare_i32(ctx.r[27].s32, -1, &mut ctx.xer);
	// 830985A4: 419A0054  beq cr6, 0x830985f8
	if ctx.cr[6].eq {
	pc = 0x830985F8; continue 'dispatch;
	}
	// 830985A8: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 830985AC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830985B0: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 830985B4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 830985B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830985BC: 4E800421  bctrl
	ctx.lr = 0x830985C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830985C0: 81700008  lwz r11, 8(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(8 as u32) ) } as u64;
	// 830985C4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830985C8: 7E038378  mr r3, r16
	ctx.r[3].u64 = ctx.r[16].u64;
	// 830985CC: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 830985D0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830985D4: 4800164D  bl 0x83099c20
	ctx.lr = 0x830985D8;
	sub_83099C20(ctx, base);
	// 830985D8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830985DC: 2F1BFFFF  cmpwi cr6, r27, -1
	ctx.cr[6].compare_i32(ctx.r[27].s32, -1, &mut ctx.xer);
	// 830985E0: 419A0018  beq cr6, 0x830985f8
	if ctx.cr[6].eq {
	pc = 0x830985F8; continue 'dispatch;
	}
	// 830985E4: 2B100000  cmplwi cr6, r16, 0
	ctx.cr[6].compare_u32(ctx.r[16].u32, 0 as u32, &mut ctx.xer);
	// 830985E8: 419A0010  beq cr6, 0x830985f8
	if ctx.cr[6].eq {
	pc = 0x830985F8; continue 'dispatch;
	}
	// 830985EC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830985F0: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830985F4: 419A03B8  beq cr6, 0x830989ac
	if ctx.cr[6].eq {
	pc = 0x830989AC; continue 'dispatch;
	}
	// 830985F8: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830985FC: 41820044  beq 0x83098640
	if ctx.cr[0].eq {
	pc = 0x83098640; continue 'dispatch;
	}
	// 83098600: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 83098604: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098608: 4BF3FC91  bl 0x82fd8298
	ctx.lr = 0x8309860C;
	sub_82FD8298(ctx, base);
	// 8309860C: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83098610: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83098614: 4182001C  beq 0x83098630
	if ctx.cr[0].eq {
	pc = 0x83098630; continue 'dispatch;
	}
	// 83098618: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8309861C: 80DE0010  lwz r6, 0x10(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098620: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83098624: 4BFFDCD5  bl 0x830962f8
	ctx.lr = 0x83098628;
	sub_830962F8(ctx, base);
	// 83098628: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8309862C: 48000008  b 0x83098634
	pc = 0x83098634; continue 'dispatch;
	// 83098630: 7E3D8B78  mr r29, r17
	ctx.r[29].u64 = ctx.r[17].u64;
	// 83098634: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098638: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8309863C: 480000EC  b 0x83098728
	pc = 0x83098728; continue 'dispatch;
	// 83098640: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 83098644: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83098648: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8309864C: 4BF5FA85  bl 0x82ff80d0
	ctx.lr = 0x83098650;
	sub_82FF80D0(ctx, base);
	// 83098650: 81560000  lwz r10, 0(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 83098654: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83098658: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 8309865C: 814A0024  lwz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 83098660: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83098664: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83098668: 4E800421  bctrl
	ctx.lr = 0x8309866C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309866C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83098670: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83098674: 80FE0010  lwz r7, 0x10(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098678: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8309867C: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 83098680: 4BF465A9  bl 0x82fdec28
	ctx.lr = 0x83098684;
	sub_82FDEC28(ctx, base);
	// 83098684: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 83098688: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309868C: 4BF3FC0D  bl 0x82fd8298
	ctx.lr = 0x83098690;
	sub_82FD8298(ctx, base);
	// 83098690: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83098694: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83098698: 41820014  beq 0x830986ac
	if ctx.cr[0].eq {
	pc = 0x830986AC; continue 'dispatch;
	}
	// 8309869C: 389F00B0  addi r4, r31, 0xb0
	ctx.r[4].s64 = ctx.r[31].s64 + 176;
	// 830986A0: 4BFFDB91  bl 0x83096230
	ctx.lr = 0x830986A4;
	sub_83096230(ctx, base);
	// 830986A4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830986A8: 48000008  b 0x830986b0
	pc = 0x830986B0; continue 'dispatch;
	// 830986AC: 7E3D8B78  mr r29, r17
	ctx.r[29].u64 = ctx.r[17].u64;
	// 830986B0: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 830986B4: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830986B8: 4BF3FBE1  bl 0x82fd8298
	ctx.lr = 0x830986BC;
	sub_82FD8298(ctx, base);
	// 830986BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830986C0: 41820018  beq 0x830986d8
	if ctx.cr[0].eq {
	pc = 0x830986D8; continue 'dispatch;
	}
	// 830986C4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830986C8: B2630004  sth r19, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[19].u16 ) };
	// 830986CC: 92E30000  stw r23, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 830986D0: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 830986D4: 48000008  b 0x830986dc
	pc = 0x830986DC; continue 'dispatch;
	// 830986D8: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 830986DC: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 830986E0: 4BFA2A71  bl 0x8303b150
	ctx.lr = 0x830986E4;
	sub_8303B150(ctx, base);
	// 830986E4: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 830986E8: 4BF46309  bl 0x82fde9f0
	ctx.lr = 0x830986EC;
	sub_82FDE9F0(ctx, base);
	// 830986EC: 48000068  b 0x83098754
	pc = 0x83098754; continue 'dispatch;
	// 830986F0: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 830986F4: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830986F8: 4BF3FBA1  bl 0x82fd8298
	ctx.lr = 0x830986FC;
	sub_82FD8298(ctx, base);
	// 830986FC: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83098700: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83098704: 41820018  beq 0x8309871c
	if ctx.cr[0].eq {
	pc = 0x8309871C; continue 'dispatch;
	}
	// 83098708: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8309870C: 80BE0010  lwz r5, 0x10(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098710: 4BFFD969  bl 0x83096078
	ctx.lr = 0x83098714;
	sub_83096078(ctx, base);
	// 83098714: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83098718: 48000008  b 0x83098720
	pc = 0x83098720; continue 'dispatch;
	// 8309871C: 7E3D8B78  mr r29, r17
	ctx.r[29].u64 = ctx.r[17].u64;
	// 83098720: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098724: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 83098728: 4BF3FB71  bl 0x82fd8298
	ctx.lr = 0x8309872C;
	sub_82FD8298(ctx, base);
	// 8309872C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83098730: 4182FA20  beq 0x83098150
	if ctx.cr[0].eq {
	pc = 0x83098150; continue 'dispatch;
	}
	// 83098734: B2630004  sth r19, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[19].u16 ) };
	// 83098738: 4BFFFA08  b 0x83098140
	pc = 0x83098140; continue 'dispatch;
	// 8309873C: 2F0B0024  cmpwi cr6, r11, 0x24
	ctx.cr[6].compare_i32(ctx.r[11].s32, 36, &mut ctx.xer);
	// 83098740: 409A0014  bne cr6, 0x83098754
	if !ctx.cr[6].eq {
	pc = 0x83098754; continue 'dispatch;
	}
	// 83098744: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 83098748: 3972FFFF  addi r11, r18, -1
	ctx.r[11].s64 = ctx.r[18].s64 + -1;
	// 8309874C: 7F185840  cmplw cr6, r24, r11
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83098750: 419A029C  beq cr6, 0x830989ec
	if ctx.cr[6].eq {
	pc = 0x830989EC; continue 'dispatch;
	}
	// 83098754: 7E398B78  mr r25, r17
	ctx.r[25].u64 = ctx.r[17].u64;
	// 83098758: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 8309875C: 7F189040  cmplw cr6, r24, r18
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[18].u32, &mut ctx.xer);
	// 83098760: 4198F948  blt cr6, 0x830980a8
	if ctx.cr[6].lt {
	pc = 0x830980A8; continue 'dispatch;
	}
	// 83098764: 81750008  lwz r11, 8(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(8 as u32) ) } as u64;
	// 83098768: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8309876C: 408202D8  bne 0x83098a44
	if !ctx.cr[0].eq {
	pc = 0x83098A44; continue 'dispatch;
	}
	// 83098770: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83098774: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83098778: 418202A0  beq 0x83098a18
	if ctx.cr[0].eq {
	pc = 0x83098A18; continue 'dispatch;
	}
	// 8309877C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83098780: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83098784: 419A0294  beq cr6, 0x83098a18
	if ctx.cr[6].eq {
	pc = 0x83098A18; continue 'dispatch;
	}
	// 83098788: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309878C: 80FE0010  lwz r7, 0x10(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098790: 38C0013D  li r6, 0x13d
	ctx.r[6].s64 = 317;
	// 83098794: 388BD458  addi r4, r11, -0x2ba8
	ctx.r[4].s64 = ctx.r[11].s64 + -11176;
	// 83098798: 38A002A8  li r5, 0x2a8
	ctx.r[5].s64 = 680;
	// 8309879C: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 830987A0: 4BFFD679  bl 0x83095e18
	ctx.lr = 0x830987A4;
	sub_83095E18(ctx, base);
	// 830987A4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830987A8: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 830987AC: 388BCA78  addi r4, r11, -0x3588
	ctx.r[4].s64 = ctx.r[11].s64 + -13704;
	// 830987B0: 48118479  bl 0x831b0c28
	ctx.lr = 0x830987B4;
	sub_831B0C28(ctx, base);
	// 830987B4: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830987B8: 80FE0010  lwz r7, 0x10(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830987BC: 38C00134  li r6, 0x134
	ctx.r[6].s64 = 308;
	// 830987C0: 388BD458  addi r4, r11, -0x2ba8
	ctx.r[4].s64 = ctx.r[11].s64 + -11176;
	// 830987C4: 38A00224  li r5, 0x224
	ctx.r[5].s64 = 548;
	// 830987C8: 387F0160  addi r3, r31, 0x160
	ctx.r[3].s64 = ctx.r[31].s64 + 352;
	// 830987CC: 4BFFD64D  bl 0x83095e18
	ctx.lr = 0x830987D0;
	sub_83095E18(ctx, base);
	// 830987D0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830987D4: 387F0160  addi r3, r31, 0x160
	ctx.r[3].s64 = ctx.r[31].s64 + 352;
	// 830987D8: 388BCA78  addi r4, r11, -0x3588
	ctx.r[4].s64 = ctx.r[11].s64 + -13704;
	// 830987DC: 4811844D  bl 0x831b0c28
	ctx.lr = 0x830987E0;
	sub_831B0C28(ctx, base);
	// 830987E0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830987E4: 80FE0010  lwz r7, 0x10(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830987E8: 38C00136  li r6, 0x136
	ctx.r[6].s64 = 310;
	// 830987EC: 388BD458  addi r4, r11, -0x2ba8
	ctx.r[4].s64 = ctx.r[11].s64 + -11176;
	// 830987F0: 38A00272  li r5, 0x272
	ctx.r[5].s64 = 626;
	// 830987F4: 387F0200  addi r3, r31, 0x200
	ctx.r[3].s64 = ctx.r[31].s64 + 512;
	// 830987F8: 4BFFD621  bl 0x83095e18
	ctx.lr = 0x830987FC;
	sub_83095E18(ctx, base);
	// 830987FC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83098800: 387F0200  addi r3, r31, 0x200
	ctx.r[3].s64 = ctx.r[31].s64 + 512;
	// 83098804: 388BCA78  addi r4, r11, -0x3588
	ctx.r[4].s64 = ctx.r[11].s64 + -13704;
	// 83098808: 48118421  bl 0x831b0c28
	ctx.lr = 0x8309880C;
	sub_831B0C28(ctx, base);
	// 8309880C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83098810: 80FE0010  lwz r7, 0x10(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098814: 38C00138  li r6, 0x138
	ctx.r[6].s64 = 312;
	// 83098818: 388BD458  addi r4, r11, -0x2ba8
	ctx.r[4].s64 = ctx.r[11].s64 + -11176;
	// 8309881C: 38A0027A  li r5, 0x27a
	ctx.r[5].s64 = 634;
	// 83098820: 387F0240  addi r3, r31, 0x240
	ctx.r[3].s64 = ctx.r[31].s64 + 576;
	// 83098824: 4BFFD5F5  bl 0x83095e18
	ctx.lr = 0x83098828;
	sub_83095E18(ctx, base);
	// 83098828: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8309882C: 387F0240  addi r3, r31, 0x240
	ctx.r[3].s64 = ctx.r[31].s64 + 576;
	// 83098830: 388BCA78  addi r4, r11, -0x3588
	ctx.r[4].s64 = ctx.r[11].s64 + -13704;
	// 83098834: 481183F5  bl 0x831b0c28
	ctx.lr = 0x83098838;
	sub_831B0C28(ctx, base);
	// 83098838: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309883C: 80FE0010  lwz r7, 0x10(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098840: 38C0012F  li r6, 0x12f
	ctx.r[6].s64 = 303;
	// 83098844: 388BD458  addi r4, r11, -0x2ba8
	ctx.r[4].s64 = ctx.r[11].s64 + -11176;
	// 83098848: 38A001C4  li r5, 0x1c4
	ctx.r[5].s64 = 452;
	// 8309884C: 387F0120  addi r3, r31, 0x120
	ctx.r[3].s64 = ctx.r[31].s64 + 288;
	// 83098850: 4BFFD5C9  bl 0x83095e18
	ctx.lr = 0x83098854;
	sub_83095E18(ctx, base);
	// 83098854: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83098858: 387F0120  addi r3, r31, 0x120
	ctx.r[3].s64 = ctx.r[31].s64 + 288;
	// 8309885C: 388BCA78  addi r4, r11, -0x3588
	ctx.r[4].s64 = ctx.r[11].s64 + -13704;
	// 83098860: 481183C9  bl 0x831b0c28
	ctx.lr = 0x83098864;
	sub_831B0C28(ctx, base);
	// 83098864: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83098868: 80FE0010  lwz r7, 0x10(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309886C: 38C00130  li r6, 0x130
	ctx.r[6].s64 = 304;
	// 83098870: 388BD458  addi r4, r11, -0x2ba8
	ctx.r[4].s64 = ctx.r[11].s64 + -11176;
	// 83098874: 38A001CA  li r5, 0x1ca
	ctx.r[5].s64 = 458;
	// 83098878: 387F01E0  addi r3, r31, 0x1e0
	ctx.r[3].s64 = ctx.r[31].s64 + 480;
	// 8309887C: 4BFFD59D  bl 0x83095e18
	ctx.lr = 0x83098880;
	sub_83095E18(ctx, base);
	// 83098880: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83098884: 387F01E0  addi r3, r31, 0x1e0
	ctx.r[3].s64 = ctx.r[31].s64 + 480;
	// 83098888: 388BCA78  addi r4, r11, -0x3588
	ctx.r[4].s64 = ctx.r[11].s64 + -13704;
	// 8309888C: 4811839D  bl 0x831b0c28
	ctx.lr = 0x83098890;
	sub_831B0C28(ctx, base);
	// 83098890: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83098894: 80FE0010  lwz r7, 0x10(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098898: 38C00139  li r6, 0x139
	ctx.r[6].s64 = 313;
	// 8309889C: 388BD458  addi r4, r11, -0x2ba8
	ctx.r[4].s64 = ctx.r[11].s64 + -11176;
	// 830988A0: 38A00288  li r5, 0x288
	ctx.r[5].s64 = 648;
	// 830988A4: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 830988A8: 4BFFD571  bl 0x83095e18
	ctx.lr = 0x830988AC;
	sub_83095E18(ctx, base);
	// 830988AC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830988B0: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 830988B4: 388BCA78  addi r4, r11, -0x3588
	ctx.r[4].s64 = ctx.r[11].s64 + -13704;
	// 830988B8: 48118371  bl 0x831b0c28
	ctx.lr = 0x830988BC;
	sub_831B0C28(ctx, base);
	// 830988BC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830988C0: 80FE0010  lwz r7, 0x10(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830988C4: 38C0013A  li r6, 0x13a
	ctx.r[6].s64 = 314;
	// 830988C8: 388BD458  addi r4, r11, -0x2ba8
	ctx.r[4].s64 = ctx.r[11].s64 + -11176;
	// 830988CC: 38A0028D  li r5, 0x28d
	ctx.r[5].s64 = 653;
	// 830988D0: 387F0260  addi r3, r31, 0x260
	ctx.r[3].s64 = ctx.r[31].s64 + 608;
	// 830988D4: 4BFFD545  bl 0x83095e18
	ctx.lr = 0x830988D8;
	sub_83095E18(ctx, base);
	// 830988D8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830988DC: 387F0260  addi r3, r31, 0x260
	ctx.r[3].s64 = ctx.r[31].s64 + 608;
	// 830988E0: 388BCA78  addi r4, r11, -0x3588
	ctx.r[4].s64 = ctx.r[11].s64 + -13704;
	// 830988E4: 48118345  bl 0x831b0c28
	ctx.lr = 0x830988E8;
	sub_831B0C28(ctx, base);
	// 830988E8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830988EC: 80FE0010  lwz r7, 0x10(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830988F0: 38C0013B  li r6, 0x13b
	ctx.r[6].s64 = 315;
	// 830988F4: 388BD458  addi r4, r11, -0x2ba8
	ctx.r[4].s64 = ctx.r[11].s64 + -11176;
	// 830988F8: 38A00292  li r5, 0x292
	ctx.r[5].s64 = 658;
	// 830988FC: 387F01A0  addi r3, r31, 0x1a0
	ctx.r[3].s64 = ctx.r[31].s64 + 416;
	// 83098900: 4BFFD519  bl 0x83095e18
	ctx.lr = 0x83098904;
	sub_83095E18(ctx, base);
	// 83098904: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83098908: 387F01A0  addi r3, r31, 0x1a0
	ctx.r[3].s64 = ctx.r[31].s64 + 416;
	// 8309890C: 388BCA78  addi r4, r11, -0x3588
	ctx.r[4].s64 = ctx.r[11].s64 + -13704;
	// 83098910: 48118319  bl 0x831b0c28
	ctx.lr = 0x83098914;
	sub_831B0C28(ctx, base);
	// 83098914: 80BE0010  lwz r5, 0x10(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098918: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309891C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83098920: 388BD458  addi r4, r11, -0x2ba8
	ctx.r[4].s64 = ctx.r[11].s64 + -11176;
	// 83098924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83098928: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8309892C: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 83098930: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 83098934: 38C00133  li r6, 0x133
	ctx.r[6].s64 = 307;
	// 83098938: 38A0024D  li r5, 0x24d
	ctx.r[5].s64 = 589;
	// 8309893C: 387F0100  addi r3, r31, 0x100
	ctx.r[3].s64 = ctx.r[31].s64 + 256;
	// 83098940: 4BFFD591  bl 0x83095ed0
	ctx.lr = 0x83098944;
	sub_83095ED0(ctx, base);
	// 83098944: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83098948: 387F0100  addi r3, r31, 0x100
	ctx.r[3].s64 = ctx.r[31].s64 + 256;
	// 8309894C: 388BCA78  addi r4, r11, -0x3588
	ctx.r[4].s64 = ctx.r[11].s64 + -13704;
	// 83098950: 481182D9  bl 0x831b0c28
	ctx.lr = 0x83098954;
	sub_831B0C28(ctx, base);
	// 83098954: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83098958: 80FE0010  lwz r7, 0x10(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309895C: 38C00131  li r6, 0x131
	ctx.r[6].s64 = 305;
	// 83098960: 388BD458  addi r4, r11, -0x2ba8
	ctx.r[4].s64 = ctx.r[11].s64 + -11176;
	// 83098964: 38A001DD  li r5, 0x1dd
	ctx.r[5].s64 = 477;
	// 83098968: 387F0140  addi r3, r31, 0x140
	ctx.r[3].s64 = ctx.r[31].s64 + 320;
	// 8309896C: 4BFFD4AD  bl 0x83095e18
	ctx.lr = 0x83098970;
	sub_83095E18(ctx, base);
	// 83098970: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83098974: 387F0140  addi r3, r31, 0x140
	ctx.r[3].s64 = ctx.r[31].s64 + 320;
	// 83098978: 388BCA78  addi r4, r11, -0x3588
	ctx.r[4].s64 = ctx.r[11].s64 + -13704;
	// 8309897C: 481182AD  bl 0x831b0c28
	ctx.lr = 0x83098980;
	sub_831B0C28(ctx, base);
	// 83098980: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83098984: 80FE0010  lwz r7, 0x10(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098988: 38C00132  li r6, 0x132
	ctx.r[6].s64 = 306;
	// 8309898C: 388BD458  addi r4, r11, -0x2ba8
	ctx.r[4].s64 = ctx.r[11].s64 + -11176;
	// 83098990: 38A001E5  li r5, 0x1e5
	ctx.r[5].s64 = 485;
	// 83098994: 387F0180  addi r3, r31, 0x180
	ctx.r[3].s64 = ctx.r[31].s64 + 384;
	// 83098998: 4BFFD481  bl 0x83095e18
	ctx.lr = 0x8309899C;
	sub_83095E18(ctx, base);
	// 8309899C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830989A0: 387F0180  addi r3, r31, 0x180
	ctx.r[3].s64 = ctx.r[31].s64 + 384;
	// 830989A4: 388BCA78  addi r4, r11, -0x3588
	ctx.r[4].s64 = ctx.r[11].s64 + -13704;
	// 830989A8: 48118281  bl 0x831b0c28
	ctx.lr = 0x830989AC;
	sub_831B0C28(ctx, base);
	// 830989AC: 80BE0010  lwz r5, 0x10(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830989B0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830989B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830989B8: 388BD458  addi r4, r11, -0x2ba8
	ctx.r[4].s64 = ctx.r[11].s64 + -11176;
	// 830989BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830989C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830989C4: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 830989C8: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 830989CC: 38C00133  li r6, 0x133
	ctx.r[6].s64 = 307;
	// 830989D0: 38A00205  li r5, 0x205
	ctx.r[5].s64 = 517;
	// 830989D4: 387F01C0  addi r3, r31, 0x1c0
	ctx.r[3].s64 = ctx.r[31].s64 + 448;
	// 830989D8: 4BFFD4F9  bl 0x83095ed0
	ctx.lr = 0x830989DC;
	sub_83095ED0(ctx, base);
	// 830989DC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830989E0: 387F01C0  addi r3, r31, 0x1c0
	ctx.r[3].s64 = ctx.r[31].s64 + 448;
	// 830989E4: 388BCA78  addi r4, r11, -0x3588
	ctx.r[4].s64 = ctx.r[11].s64 + -13704;
	// 830989E8: 48118241  bl 0x831b0c28
	ctx.lr = 0x830989EC;
	sub_831B0C28(ctx, base);
	// 830989EC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830989F0: 80FE0010  lwz r7, 0x10(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830989F4: 38C00135  li r6, 0x135
	ctx.r[6].s64 = 309;
	// 830989F8: 388BD458  addi r4, r11, -0x2ba8
	ctx.r[4].s64 = ctx.r[11].s64 + -11176;
	// 830989FC: 38A0022C  li r5, 0x22c
	ctx.r[5].s64 = 556;
	// 83098A00: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83098A04: 4BFFD415  bl 0x83095e18
	ctx.lr = 0x83098A08;
	sub_83095E18(ctx, base);
	// 83098A08: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83098A0C: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83098A10: 388BCA78  addi r4, r11, -0x3588
	ctx.r[4].s64 = ctx.r[11].s64 + -13704;
	// 83098A14: 48118215  bl 0x831b0c28
	ctx.lr = 0x83098A18;
	sub_831B0C28(ctx, base);
	// 83098A18: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83098A1C: 80FE0010  lwz r7, 0x10(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098A20: 38C0013C  li r6, 0x13c
	ctx.r[6].s64 = 316;
	// 83098A24: 388BD458  addi r4, r11, -0x2ba8
	ctx.r[4].s64 = ctx.r[11].s64 + -11176;
	// 83098A28: 38A002A5  li r5, 0x2a5
	ctx.r[5].s64 = 677;
	// 83098A2C: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83098A30: 4BFFD3E9  bl 0x83095e18
	ctx.lr = 0x83098A34;
	sub_83095E18(ctx, base);
	// 83098A34: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83098A38: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83098A3C: 388BCA78  addi r4, r11, -0x3588
	ctx.r[4].s64 = ctx.r[11].s64 + -13704;
	// 83098A40: 481181E9  bl 0x831b0c28
	ctx.lr = 0x83098A44;
	sub_831B0C28(ctx, base);
	// 83098A44: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 83098A48: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098A4C: 4BF3F84D  bl 0x82fd8298
	ctx.lr = 0x83098A50;
	sub_82FD8298(ctx, base);
	// 83098A50: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83098A54: 41820014  beq 0x83098a68
	if ctx.cr[0].eq {
	pc = 0x83098A68; continue 'dispatch;
	}
	// 83098A58: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83098A5C: 92A30004  stw r21, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[21].u32 ) };
	// 83098A60: 91E30000  stw r15, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[15].u32 ) };
	// 83098A64: 48000008  b 0x83098a6c
	pc = 0x83098A6C; continue 'dispatch;
	// 83098A68: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 83098A6C: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83098A70: 4BFA26E1  bl 0x8303b150
	ctx.lr = 0x83098A74;
	sub_8303B150(ctx, base);
	// 83098A74: 923F0064  stw r17, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[17].u32 ) };
	// 83098A78: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83098A7C: 387F0064  addi r3, r31, 0x64
	ctx.r[3].s64 = ctx.r[31].s64 + 100;
	// 83098A80: 4BFD0259  bl 0x83068cd8
	ctx.lr = 0x83098A84;
	sub_83068CD8(ctx, base);
	// 83098A84: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83098A88: 396BD188  addi r11, r11, -0x2e78
	ctx.r[11].s64 = ctx.r[11].s64 + -11896;
	// 83098A8C: 917F0280  stw r11, 0x280(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(640 as u32), ctx.r[11].u32 ) };
	// 83098A90: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83098A94: 809F007C  lwz r4, 0x7c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83098A98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83098A9C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83098AA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83098AA4: 4E800421  bctrl
	ctx.lr = 0x83098AA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83098AA8: 383F0380  addi r1, r31, 0x380
	ctx.r[1].s64 = ctx.r[31].s64 + 896;
	// 83098AAC: 4810F6D4  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83098AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83098AB0 size=40
    let mut pc: u32 = 0x83098AB0;
    'dispatch: loop {
        match pc {
            0x83098AB0 => {
    //   block [0x83098AB0..0x83098AD8)
	// 83098AB0: 3BECFC80  addi r31, r12, -0x380
	ctx.r[31].s64 = ctx.r[12].s64 + -896;
	// 83098AB4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83098AB8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83098ABC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83098AC0: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83098AC4: 4BFFE155  bl 0x83096c18
	ctx.lr = 0x83098AC8;
	sub_83096C18(ctx, base);
	// 83098AC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83098ACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83098AD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83098AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83098AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83098AD8 size=40
    let mut pc: u32 = 0x83098AD8;
    'dispatch: loop {
        match pc {
            0x83098AD8 => {
    //   block [0x83098AD8..0x83098B00)
	// 83098AD8: 3BECFC80  addi r31, r12, -0x380
	ctx.r[31].s64 = ctx.r[12].s64 + -896;
	// 83098ADC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83098AE0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83098AE4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83098AE8: 387F0280  addi r3, r31, 0x280
	ctx.r[3].s64 = ctx.r[31].s64 + 640;
	// 83098AEC: 4BFFD315  bl 0x83095e00
	ctx.lr = 0x83098AF0;
	sub_83095E00(ctx, base);
	// 83098AF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83098AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83098AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83098AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83098B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83098B00 size=48
    let mut pc: u32 = 0x83098B00;
    'dispatch: loop {
        match pc {
            0x83098B00 => {
    //   block [0x83098B00..0x83098B30)
	// 83098B00: 3BECFC80  addi r31, r12, -0x380
	ctx.r[31].s64 = ctx.r[12].s64 + -896;
	// 83098B04: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83098B08: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83098B0C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83098B10: 817F0394  lwz r11, 0x394(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(916 as u32) ) } as u64;
	// 83098B14: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098B18: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 83098B1C: 4BF3F7C5  bl 0x82fd82e0
	ctx.lr = 0x83098B20;
	sub_82FD82E0(ctx, base);
	// 83098B20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83098B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83098B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83098B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83098B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83098B30 size=40
    let mut pc: u32 = 0x83098B30;
    'dispatch: loop {
        match pc {
            0x83098B30 => {
    //   block [0x83098B30..0x83098B58)
	// 83098B30: 3BECFC80  addi r31, r12, -0x380
	ctx.r[31].s64 = ctx.r[12].s64 + -896;
	// 83098B34: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83098B38: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83098B3C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83098B40: 387F0064  addi r3, r31, 0x64
	ctx.r[3].s64 = ctx.r[31].s64 + 100;
	// 83098B44: 4BF7FC45  bl 0x83018788
	ctx.lr = 0x83098B48;
	sub_83018788(ctx, base);
	// 83098B48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83098B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83098B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83098B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83098B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83098B58 size=48
    let mut pc: u32 = 0x83098B58;
    'dispatch: loop {
        match pc {
            0x83098B58 => {
    //   block [0x83098B58..0x83098B88)
	// 83098B58: 3BECFC80  addi r31, r12, -0x380
	ctx.r[31].s64 = ctx.r[12].s64 + -896;
	// 83098B5C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83098B60: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83098B64: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83098B68: 817F0394  lwz r11, 0x394(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(916 as u32) ) } as u64;
	// 83098B6C: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098B70: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83098B74: 4BF3F76D  bl 0x82fd82e0
	ctx.lr = 0x83098B78;
	sub_82FD82E0(ctx, base);
	// 83098B78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83098B7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83098B80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83098B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83098B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83098B88 size=48
    let mut pc: u32 = 0x83098B88;
    'dispatch: loop {
        match pc {
            0x83098B88 => {
    //   block [0x83098B88..0x83098BB8)
	// 83098B88: 3BECFC80  addi r31, r12, -0x380
	ctx.r[31].s64 = ctx.r[12].s64 + -896;
	// 83098B8C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83098B90: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83098B94: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83098B98: 817F0394  lwz r11, 0x394(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(916 as u32) ) } as u64;
	// 83098B9C: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098BA0: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83098BA4: 4BF3F73D  bl 0x82fd82e0
	ctx.lr = 0x83098BA8;
	sub_82FD82E0(ctx, base);
	// 83098BA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83098BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83098BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83098BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83098BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83098BB8 size=48
    let mut pc: u32 = 0x83098BB8;
    'dispatch: loop {
        match pc {
            0x83098BB8 => {
    //   block [0x83098BB8..0x83098BE8)
	// 83098BB8: 3BECFC80  addi r31, r12, -0x380
	ctx.r[31].s64 = ctx.r[12].s64 + -896;
	// 83098BBC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83098BC0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83098BC4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83098BC8: 817F0394  lwz r11, 0x394(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(916 as u32) ) } as u64;
	// 83098BCC: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098BD0: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83098BD4: 4BF3F70D  bl 0x82fd82e0
	ctx.lr = 0x83098BD8;
	sub_82FD82E0(ctx, base);
	// 83098BD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83098BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83098BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83098BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83098BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83098BE8 size=48
    let mut pc: u32 = 0x83098BE8;
    'dispatch: loop {
        match pc {
            0x83098BE8 => {
    //   block [0x83098BE8..0x83098C18)
	// 83098BE8: 3BECFC80  addi r31, r12, -0x380
	ctx.r[31].s64 = ctx.r[12].s64 + -896;
	// 83098BEC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83098BF0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83098BF4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83098BF8: 817F0394  lwz r11, 0x394(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(916 as u32) ) } as u64;
	// 83098BFC: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098C00: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83098C04: 4BF3F6DD  bl 0x82fd82e0
	ctx.lr = 0x83098C08;
	sub_82FD82E0(ctx, base);
	// 83098C08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83098C0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83098C10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83098C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83098C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83098C18 size=48
    let mut pc: u32 = 0x83098C18;
    'dispatch: loop {
        match pc {
            0x83098C18 => {
    //   block [0x83098C18..0x83098C48)
	// 83098C18: 3BECFC80  addi r31, r12, -0x380
	ctx.r[31].s64 = ctx.r[12].s64 + -896;
	// 83098C1C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83098C20: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83098C24: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83098C28: 817F0394  lwz r11, 0x394(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(916 as u32) ) } as u64;
	// 83098C2C: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098C30: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83098C34: 4BF3F6AD  bl 0x82fd82e0
	ctx.lr = 0x83098C38;
	sub_82FD82E0(ctx, base);
	// 83098C38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83098C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83098C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83098C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83098C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83098C48 size=48
    let mut pc: u32 = 0x83098C48;
    'dispatch: loop {
        match pc {
            0x83098C48 => {
    //   block [0x83098C48..0x83098C78)
	// 83098C48: 3BECFC80  addi r31, r12, -0x380
	ctx.r[31].s64 = ctx.r[12].s64 + -896;
	// 83098C4C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83098C50: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83098C54: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83098C58: 817F0394  lwz r11, 0x394(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(916 as u32) ) } as u64;
	// 83098C5C: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098C60: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83098C64: 4BF3F67D  bl 0x82fd82e0
	ctx.lr = 0x83098C68;
	sub_82FD82E0(ctx, base);
	// 83098C68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83098C6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83098C70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83098C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83098C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83098C78 size=40
    let mut pc: u32 = 0x83098C78;
    'dispatch: loop {
        match pc {
            0x83098C78 => {
    //   block [0x83098C78..0x83098CA0)
	// 83098C78: 3BECFC80  addi r31, r12, -0x380
	ctx.r[31].s64 = ctx.r[12].s64 + -896;
	// 83098C7C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83098C80: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83098C84: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83098C88: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 83098C8C: 4BF45D65  bl 0x82fde9f0
	ctx.lr = 0x83098C90;
	sub_82FDE9F0(ctx, base);
	// 83098C90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83098C94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83098C98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83098C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83098CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83098CA0 size=48
    let mut pc: u32 = 0x83098CA0;
    'dispatch: loop {
        match pc {
            0x83098CA0 => {
    //   block [0x83098CA0..0x83098CD0)
	// 83098CA0: 3BECFC80  addi r31, r12, -0x380
	ctx.r[31].s64 = ctx.r[12].s64 + -896;
	// 83098CA4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83098CA8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83098CAC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83098CB0: 817F0394  lwz r11, 0x394(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(916 as u32) ) } as u64;
	// 83098CB4: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098CB8: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83098CBC: 4BF3F625  bl 0x82fd82e0
	ctx.lr = 0x83098CC0;
	sub_82FD82E0(ctx, base);
	// 83098CC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83098CC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83098CC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83098CCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83098CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83098CD0 size=48
    let mut pc: u32 = 0x83098CD0;
    'dispatch: loop {
        match pc {
            0x83098CD0 => {
    //   block [0x83098CD0..0x83098D00)
	// 83098CD0: 3BECFC80  addi r31, r12, -0x380
	ctx.r[31].s64 = ctx.r[12].s64 + -896;
	// 83098CD4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83098CD8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83098CDC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83098CE0: 817F0394  lwz r11, 0x394(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(916 as u32) ) } as u64;
	// 83098CE4: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098CE8: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83098CEC: 4BF3F5F5  bl 0x82fd82e0
	ctx.lr = 0x83098CF0;
	sub_82FD82E0(ctx, base);
	// 83098CF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83098CF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83098CF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83098CFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83098D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83098D00 size=40
    let mut pc: u32 = 0x83098D00;
    'dispatch: loop {
        match pc {
            0x83098D00 => {
    //   block [0x83098D00..0x83098D28)
	// 83098D00: 3BECFC80  addi r31, r12, -0x380
	ctx.r[31].s64 = ctx.r[12].s64 + -896;
	// 83098D04: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83098D08: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83098D0C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83098D10: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 83098D14: 4BF45CDD  bl 0x82fde9f0
	ctx.lr = 0x83098D18;
	sub_82FDE9F0(ctx, base);
	// 83098D18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83098D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83098D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83098D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83098D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83098D28 size=48
    let mut pc: u32 = 0x83098D28;
    'dispatch: loop {
        match pc {
            0x83098D28 => {
    //   block [0x83098D28..0x83098D58)
	// 83098D28: 3BECFC80  addi r31, r12, -0x380
	ctx.r[31].s64 = ctx.r[12].s64 + -896;
	// 83098D2C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83098D30: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83098D34: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83098D38: 817F0394  lwz r11, 0x394(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(916 as u32) ) } as u64;
	// 83098D3C: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098D40: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83098D44: 4BF3F59D  bl 0x82fd82e0
	ctx.lr = 0x83098D48;
	sub_82FD82E0(ctx, base);
	// 83098D48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83098D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83098D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83098D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83098D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83098D58 size=48
    let mut pc: u32 = 0x83098D58;
    'dispatch: loop {
        match pc {
            0x83098D58 => {
    //   block [0x83098D58..0x83098D88)
	// 83098D58: 3BECFC80  addi r31, r12, -0x380
	ctx.r[31].s64 = ctx.r[12].s64 + -896;
	// 83098D5C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83098D60: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83098D64: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83098D68: 817F0394  lwz r11, 0x394(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(916 as u32) ) } as u64;
	// 83098D6C: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098D70: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83098D74: 4BF3F56D  bl 0x82fd82e0
	ctx.lr = 0x83098D78;
	sub_82FD82E0(ctx, base);
	// 83098D78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83098D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83098D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83098D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83098D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83098D88 size=8
    let mut pc: u32 = 0x83098D88;
    'dispatch: loop {
        match pc {
            0x83098D88 => {
    //   block [0x83098D88..0x83098D90)
	// 83098D88: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83098D8C: 8216D73C  lwz r16, -0x28c4(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-10436 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83098D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83098D90 size=164
    let mut pc: u32 = 0x83098D90;
    'dispatch: loop {
        match pc {
            0x83098D90 => {
    //   block [0x83098D90..0x83098E34)
	// 83098D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83098D94: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83098D98: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83098D9C: 4810F3C9  bl 0x831a8164
	ctx.lr = 0x83098DA0;
	sub_831A8130(ctx, base);
	// 83098DA0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83098DA4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83098DA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83098DAC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83098DB0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83098DB4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83098DB8: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 83098DBC: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83098DC0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83098DC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83098DC8: 90FE0004  stw r7, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 83098DCC: 396BD410  addi r11, r11, -0x2bf0
	ctx.r[11].s64 = ctx.r[11].s64 + -11248;
	// 83098DD0: 913E0010  stw r9, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 83098DD4: 915E000C  stw r10, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83098DD8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83098DDC: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83098DE0: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 83098DE4: 4BF37D9D  bl 0x82fd0b80
	ctx.lr = 0x83098DE8;
	sub_82FD0B80(ctx, base);
	// 83098DE8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83098DEC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83098DF0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83098DF4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83098DF8: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83098DFC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83098E00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83098E04: 4BFFF11D  bl 0x83097f20
	ctx.lr = 0x83098E08;
	sub_83097F20(ctx, base);
	// 83098E08: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83098E0C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83098E10: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83098E14: 41820014  beq 0x83098e28
	if ctx.cr[0].eq {
	pc = 0x83098E28; continue 'dispatch;
	}
	// 83098E18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83098E1C: 4BFFE0C5  bl 0x83096ee0
	ctx.lr = 0x83098E20;
	sub_83096EE0(ctx, base);
	// 83098E20: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83098E24: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83098E28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83098E2C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83098E30: 4810F384  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83098E34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83098E34 size=8
    let mut pc: u32 = 0x83098E34;
    'dispatch: loop {
        match pc {
            0x83098E34 => {
    //   block [0x83098E34..0x83098E3C)
	// 83098E34: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83098E38: 8216D73C  lwz r16, -0x28c4(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-10436 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83098E3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83098E3C size=24
    let mut pc: u32 = 0x83098E3C;
    'dispatch: loop {
        match pc {
            0x83098E3C => {
    //   block [0x83098E3C..0x83098E54)
	// 83098E3C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83098E40: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83098E44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83098E48: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83098E4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83098E50: 48117DD9  bl 0x831b0c28
	ctx.lr = 0x83098E54;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83098E5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83098E5C size=36
    let mut pc: u32 = 0x83098E5C;
    'dispatch: loop {
        match pc {
            0x83098E5C => {
    //   block [0x83098E5C..0x83098E80)
	// 83098E5C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83098E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83098E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83098E68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83098E6C: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83098E70: 4BFFD909  bl 0x83096778
	ctx.lr = 0x83098E74;
	sub_83096778(ctx, base);
	// 83098E74: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83098E78: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83098E7C: 48117DAD  bl 0x831b0c28
	ctx.lr = 0x83098E80;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83098E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83098E80 size=40
    let mut pc: u32 = 0x83098E80;
    'dispatch: loop {
        match pc {
            0x83098E80 => {
    //   block [0x83098E80..0x83098EA8)
	// 83098E80: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83098E84: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83098E88: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83098E8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83098E90: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83098E94: 4BFB38CD  bl 0x8304c760
	ctx.lr = 0x83098E98;
	sub_8304C760(ctx, base);
	// 83098E98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83098E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83098EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83098EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83098EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83098EA8 size=88
    let mut pc: u32 = 0x83098EA8;
    'dispatch: loop {
        match pc {
            0x83098EA8 => {
    //   block [0x83098EA8..0x83098F00)
	// 83098EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83098EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83098EB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83098EB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83098EB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83098EBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83098EC0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83098EC4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83098EC8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83098ECC: 41820018  beq 0x83098ee4
	if ctx.cr[0].eq {
	pc = 0x83098EE4; continue 'dispatch;
	}
	// 83098ED0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83098ED4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83098ED8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83098EDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83098EE0: 4E800421  bctrl
	ctx.lr = 0x83098EE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83098EE4: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 83098EE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83098EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83098EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83098EF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83098EF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83098EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83098F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83098F00 size=152
    let mut pc: u32 = 0x83098F00;
    'dispatch: loop {
        match pc {
            0x83098F00 => {
    //   block [0x83098F00..0x83098F98)
	// 83098F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83098F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83098F08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83098F0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83098F10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83098F14: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83098F18: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83098F1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83098F20: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83098F24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83098F28: 4E800421  bctrl
	ctx.lr = 0x83098F2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83098F2C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83098F30: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83098F34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83098F38: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83098F3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83098F40: 4E800421  bctrl
	ctx.lr = 0x83098F44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83098F44: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83098F48: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83098F4C: 41820018  beq 0x83098f64
	if ctx.cr[0].eq {
	pc = 0x83098F64; continue 'dispatch;
	}
	// 83098F50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83098F54: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83098F58: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83098F5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83098F60: 4E800421  bctrl
	ctx.lr = 0x83098F64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83098F64: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83098F68: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83098F6C: 41820018  beq 0x83098f84
	if ctx.cr[0].eq {
	pc = 0x83098F84; continue 'dispatch;
	}
	// 83098F70: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83098F74: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83098F78: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83098F7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83098F80: 4E800421  bctrl
	ctx.lr = 0x83098F84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83098F84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83098F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83098F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83098F90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83098F94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83098F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83098F98 size=12
    let mut pc: u32 = 0x83098F98;
    'dispatch: loop {
        match pc {
            0x83098F98 => {
    //   block [0x83098F98..0x83098FA4)
	// 83098F98: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83098F9C: 386B35C8  addi r3, r11, 0x35c8
	ctx.r[3].s64 = ctx.r[11].s64 + 13768;
	// 83098FA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83098FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83098FA8 size=248
    let mut pc: u32 = 0x83098FA8;
    'dispatch: loop {
        match pc {
            0x83098FA8 => {
    //   block [0x83098FA8..0x830990A0)
	// 83098FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83098FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83098FB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83098FB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83098FB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83098FBC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83098FC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83098FC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83098FC8: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 83098FCC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83098FD0: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83098FD4: 41820050  beq 0x83099024
	if ctx.cr[0].eq {
	pc = 0x83099024; continue 'dispatch;
	}
	// 83098FD8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83098FDC: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83098FE0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83098FE4: 4BF6091D  bl 0x82ff9900
	ctx.lr = 0x83098FE8;
	sub_82FF9900(ctx, base);
	// 83098FE8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83098FEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83098FF0: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83098FF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83098FF8: 4BF60909  bl 0x82ff9900
	ctx.lr = 0x83098FFC;
	sub_82FF9900(ctx, base);
	// 83098FFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83099000: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83099004: 4BF60BFD  bl 0x82ff9c00
	ctx.lr = 0x83099008;
	sub_82FF9C00(ctx, base);
	// 83099008: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309900C: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83099010: 4BF602E9  bl 0x82ff92f8
	ctx.lr = 0x83099014;
	sub_82FF92F8(ctx, base);
	// 83099014: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83099018: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309901C: 4BFB3B1D  bl 0x8304cb38
	ctx.lr = 0x83099020;
	sub_8304CB38(ctx, base);
	// 83099020: 48000068  b 0x83099088
	pc = 0x83099088; continue 'dispatch;
	// 83099024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83099028: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8309902C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 83099030: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 83099034: 4BF60AF5  bl 0x82ff9b28
	ctx.lr = 0x83099038;
	sub_82FF9B28(ctx, base);
	// 83099038: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8309903C: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 83099040: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83099044: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 83099048: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309904C: 4BF60ADD  bl 0x82ff9b28
	ctx.lr = 0x83099050;
	sub_82FF9B28(ctx, base);
	// 83099050: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83099054: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83099058: 388B3600  addi r4, r11, 0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + 13824;
	// 8309905C: 4BF60C65  bl 0x82ff9cc0
	ctx.lr = 0x83099060;
	sub_82FF9CC0(ctx, base);
	// 83099060: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83099064: 389E0018  addi r4, r30, 0x18
	ctx.r[4].s64 = ctx.r[30].s64 + 24;
	// 83099068: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309906C: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83099070: 4BF60509  bl 0x82ff9578
	ctx.lr = 0x83099074;
	sub_82FF9578(ctx, base);
	// 83099074: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83099078: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8309907C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83099080: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 83099084: 4BFB48CD  bl 0x8304d950
	ctx.lr = 0x83099088;
	sub_8304D950(ctx, base);
	// 83099088: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309908C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83099090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83099094: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83099098: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309909C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830990A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830990A0 size=124
    let mut pc: u32 = 0x830990A0;
    'dispatch: loop {
        match pc {
            0x830990A0 => {
    //   block [0x830990A0..0x8309911C)
	// 830990A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830990A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830990A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830990AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830990B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830990B4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830990B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830990BC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830990C0: 419A0038  beq cr6, 0x830990f8
	if ctx.cr[6].eq {
	pc = 0x830990F8; continue 'dispatch;
	}
	// 830990C4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830990C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830990CC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830990D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830990D4: 4E800421  bctrl
	ctx.lr = 0x830990D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830990D8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830990DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830990E0: 7D640734  extsh r4, r11
	ctx.r[4].s64 = ctx.r[11].s16 as i64;
	// 830990E4: 4BF60215  bl 0x82ff92f8
	ctx.lr = 0x830990E8;
	sub_82FF92F8(ctx, base);
	// 830990E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830990EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830990F0: 4BF60B11  bl 0x82ff9c00
	ctx.lr = 0x830990F4;
	sub_82FF9C00(ctx, base);
	// 830990F4: 48000010  b 0x83099104
	pc = 0x83099104; continue 'dispatch;
	// 830990F8: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 830990FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83099100: 4BF601F9  bl 0x82ff92f8
	ctx.lr = 0x83099104;
	sub_82FF92F8(ctx, base);
	// 83099104: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83099108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309910C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83099110: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83099114: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83099118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83099120 size=120
    let mut pc: u32 = 0x83099120;
    'dispatch: loop {
        match pc {
            0x83099120 => {
    //   block [0x83099120..0x83099198)
	// 83099120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83099124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83099128: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309912C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83099130: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83099134: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83099138: 4BF60441  bl 0x82ff9578
	ctx.lr = 0x8309913C;
	sub_82FF9578(ctx, base);
	// 8309913C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83099140: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83099144: 41980030  blt cr6, 0x83099174
	if ctx.cr[6].lt {
	pc = 0x83099174; continue 'dispatch;
	}
	// 83099148: 419A0020  beq cr6, 0x83099168
	if ctx.cr[6].eq {
	pc = 0x83099168; continue 'dispatch;
	}
	// 8309914C: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 83099150: 4198000C  blt cr6, 0x8309915c
	if ctx.cr[6].lt {
	pc = 0x8309915C; continue 'dispatch;
	}
	// 83099154: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83099158: 4800002C  b 0x83099184
	pc = 0x83099184; continue 'dispatch;
	// 8309915C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83099160: 388B43F0  addi r4, r11, 0x43f0
	ctx.r[4].s64 = ctx.r[11].s64 + 17392;
	// 83099164: 48000018  b 0x8309917c
	pc = 0x8309917C; continue 'dispatch;
	// 83099168: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8309916C: 388B43E0  addi r4, r11, 0x43e0
	ctx.r[4].s64 = ctx.r[11].s64 + 17376;
	// 83099170: 4800000C  b 0x8309917c
	pc = 0x8309917C; continue 'dispatch;
	// 83099174: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83099178: 388B43E8  addi r4, r11, 0x43e8
	ctx.r[4].s64 = ctx.r[11].s64 + 17384;
	// 8309917C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83099180: 4BF60B41  bl 0x82ff9cc0
	ctx.lr = 0x83099184;
	sub_82FF9CC0(ctx, base);
	// 83099184: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83099188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309918C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83099190: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83099194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83099198 size=8
    let mut pc: u32 = 0x83099198;
    'dispatch: loop {
        match pc {
            0x83099198 => {
    //   block [0x83099198..0x830991A0)
	// 83099198: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309919C: 8216D83C  lwz r16, -0x27c4(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-10180 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830991A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830991A0 size=144
    let mut pc: u32 = 0x830991A0;
    'dispatch: loop {
        match pc {
            0x830991A0 => {
    //   block [0x830991A0..0x83099230)
	// 830991A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830991A4: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 830991A8: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 830991AC: 4810EFC1  bl 0x831a816c
	ctx.lr = 0x830991B0;
	sub_831A8130(ctx, base);
	// 830991B0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830991B4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830991B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830991BC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830991C0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830991C4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830991C8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830991CC: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 830991D0: 90DE0014  stw r6, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 830991D4: 394BD7DC  addi r10, r11, -0x2824
	ctx.r[10].s64 = ctx.r[11].s64 + -10276;
	// 830991D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830991DC: 913E0018  stw r9, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 830991E0: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830991E4: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830991E8: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830991EC: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830991F0: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830991F4: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 830991F8: 4BF37989  bl 0x82fd0b80
	ctx.lr = 0x830991FC;
	sub_82FD0B80(ctx, base);
	// 830991FC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83099200: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83099204: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83099208: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309920C: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83099210: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83099214: 4BF3796D  bl 0x82fd0b80
	ctx.lr = 0x83099218;
	sub_82FD0B80(ctx, base);
	// 83099218: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8309921C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83099220: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 83099224: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83099228: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8309922C: 4810EF90  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83099230 size=8
    let mut pc: u32 = 0x83099230;
    'dispatch: loop {
        match pc {
            0x83099230 => {
    //   block [0x83099230..0x83099238)
	// 83099230: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83099234: 8216D83C  lwz r16, -0x27c4(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-10180 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83099238 size=24
    let mut pc: u32 = 0x83099238;
    'dispatch: loop {
        match pc {
            0x83099238 => {
    //   block [0x83099238..0x83099250)
	// 83099238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309923C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83099240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83099244: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83099248: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309924C: 481179DD  bl 0x831b0c28
	ctx.lr = 0x83099250;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83099258 size=36
    let mut pc: u32 = 0x83099258;
    'dispatch: loop {
        match pc {
            0x83099258 => {
    //   block [0x83099258..0x8309927C)
	// 83099258: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8309925C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83099260: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83099264: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83099268: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8309926C: 4BFFFC95  bl 0x83098f00
	ctx.lr = 0x83099270;
	sub_83098F00(ctx, base);
	// 83099270: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83099274: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83099278: 481179B1  bl 0x831b0c28
	ctx.lr = 0x8309927C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309927C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309927C size=40
    let mut pc: u32 = 0x8309927C;
    'dispatch: loop {
        match pc {
            0x8309927C => {
    //   block [0x8309927C..0x830992A4)
	// 8309927C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83099280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83099284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83099288: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309928C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83099290: 4BFB34D1  bl 0x8304c760
	ctx.lr = 0x83099294;
	sub_8304C760(ctx, base);
	// 83099294: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83099298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309929C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830992A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830992A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830992A8 size=8
    let mut pc: u32 = 0x830992A8;
    'dispatch: loop {
        match pc {
            0x830992A8 => {
    //   block [0x830992A8..0x830992B0)
	// 830992A8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830992AC: 8216D8A8  lwz r16, -0x2758(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-10072 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830992B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830992B0 size=84
    let mut pc: u32 = 0x830992B0;
    'dispatch: loop {
        match pc {
            0x830992B0 => {
    //   block [0x830992B0..0x83099304)
	// 830992B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830992B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830992B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830992BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830992C0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830992C4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830992C8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830992CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830992D0: 396BD7DC  addi r11, r11, -0x2824
	ctx.r[11].s64 = ctx.r[11].s64 + -10276;
	// 830992D4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830992D8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830992DC: 4BFFFC25  bl 0x83098f00
	ctx.lr = 0x830992E0;
	sub_83098F00(ctx, base);
	// 830992E0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830992E4: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 830992E8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830992EC: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830992F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830992F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830992F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830992FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83099300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099304(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83099304 size=40
    let mut pc: u32 = 0x83099304;
    'dispatch: loop {
        match pc {
            0x83099304 => {
    //   block [0x83099304..0x8309932C)
	// 83099304: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83099308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309930C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83099310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83099314: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83099318: 4BFB3449  bl 0x8304c760
	ctx.lr = 0x8309931C;
	sub_8304C760(ctx, base);
	// 8309931C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83099320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83099324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83099328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83099330 size=232
    let mut pc: u32 = 0x83099330;
    'dispatch: loop {
        match pc {
            0x83099330 => {
    //   block [0x83099330..0x83099418)
	// 83099330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83099334: 4810EE31  bl 0x831a8164
	ctx.lr = 0x83099338;
	sub_831A8130(ctx, base);
	// 83099338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309933C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83099340: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83099344: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83099348: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309934C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83099350: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83099354: 4E800421  bctrl
	ctx.lr = 0x83099358;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83099358: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309935C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83099360: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83099364: 7D5D0734  extsh r29, r10
	ctx.r[29].s64 = ctx.r[10].s16 as i64;
	// 83099368: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309936C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83099370: 4E800421  bctrl
	ctx.lr = 0x83099374;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83099374: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 83099378: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8309937C: 419A000C  beq cr6, 0x83099388
	if ctx.cr[6].eq {
	pc = 0x83099388; continue 'dispatch;
	}
	// 83099380: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83099384: 4800008C  b 0x83099410
	pc = 0x83099410; continue 'dispatch;
	// 83099388: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309938C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83099390: 4BF3A8B1  bl 0x82fd3c40
	ctx.lr = 0x83099394;
	sub_82FD3C40(ctx, base);
	// 83099394: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83099398: 4182FFE8  beq 0x83099380
	if ctx.cr[0].eq {
	pc = 0x83099380; continue 'dispatch;
	}
	// 8309939C: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830993A0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830993A4: 4BFFBF2D  bl 0x830952d0
	ctx.lr = 0x830993A8;
	sub_830952D0(ctx, base);
	// 830993A8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830993AC: 4082FFD4  bne 0x83099380
	if !ctx.cr[0].eq {
	pc = 0x83099380; continue 'dispatch;
	}
	// 830993B0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830993B4: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830993B8: 838B0008  lwz r28, 8(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830993BC: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 830993C0: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830993C4: 409AFFBC  bne cr6, 0x83099380
	if !ctx.cr[6].eq {
	pc = 0x83099380; continue 'dispatch;
	}
	// 830993C8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830993CC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 830993D0: 419A003C  beq cr6, 0x8309940c
	if ctx.cr[6].eq {
	pc = 0x8309940C; continue 'dispatch;
	}
	// 830993D4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830993D8: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830993DC: 4BF93495  bl 0x8302c870
	ctx.lr = 0x830993E0;
	sub_8302C870(ctx, base);
	// 830993E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830993E4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 830993E8: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830993EC: 4BF93485  bl 0x8302c870
	ctx.lr = 0x830993F0;
	sub_8302C870(ctx, base);
	// 830993F0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830993F4: 4BFFBEDD  bl 0x830952d0
	ctx.lr = 0x830993F8;
	sub_830952D0(ctx, base);
	// 830993F8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830993FC: 4082FF84  bne 0x83099380
	if !ctx.cr[0].eq {
	pc = 0x83099380; continue 'dispatch;
	}
	// 83099400: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83099404: 7F1DE040  cmplw cr6, r29, r28
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[28].u32, &mut ctx.xer);
	// 83099408: 4198FFCC  blt cr6, 0x830993d4
	if ctx.cr[6].lt {
	pc = 0x830993D4; continue 'dispatch;
	}
	// 8309940C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83099410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83099414: 4810EDA0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83099418 size=76
    let mut pc: u32 = 0x83099418;
    'dispatch: loop {
        match pc {
            0x83099418 => {
    //   block [0x83099418..0x83099464)
	// 83099418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309941C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83099420: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83099424: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83099428: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309942C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83099430: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83099434: 4BFFFE7D  bl 0x830992b0
	ctx.lr = 0x83099438;
	sub_830992B0(ctx, base);
	// 83099438: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309943C: 4182000C  beq 0x83099448
	if ctx.cr[0].eq {
	pc = 0x83099448; continue 'dispatch;
	}
	// 83099440: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83099444: 4BF3EE9D  bl 0x82fd82e0
	ctx.lr = 0x83099448;
	sub_82FD82E0(ctx, base);
	// 83099448: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309944C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83099450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83099454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83099458: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309945C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83099460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83099468 size=16
    let mut pc: u32 = 0x83099468;
    'dispatch: loop {
        match pc {
            0x83099468 => {
    //   block [0x83099468..0x83099478)
	// 83099468: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309946C: 396BD8E4  addi r11, r11, -0x271c
	ctx.r[11].s64 = ctx.r[11].s64 + -10012;
	// 83099470: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83099474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83099478 size=12
    let mut pc: u32 = 0x83099478;
    'dispatch: loop {
        match pc {
            0x83099478 => {
    //   block [0x83099478..0x83099484)
	// 83099478: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8309947C: 386B35D0  addi r3, r11, 0x35d0
	ctx.r[3].s64 = ctx.r[11].s64 + 13776;
	// 83099480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83099488 size=44
    let mut pc: u32 = 0x83099488;
    'dispatch: loop {
        match pc {
            0x83099488 => {
    //   block [0x83099488..0x830994B4)
	// 83099488: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8309948C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83099490: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83099494: 41980044  blt cr6, 0x830994d8
	if ctx.cr[6].lt {
		sub_830994D8(ctx, base);
		return;
	}
	// 83099498: 419A0034  beq cr6, 0x830994cc
	if ctx.cr[6].eq {
		sub_830994CC(ctx, base);
		return;
	}
	// 8309949C: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 830994A0: 41980020  blt cr6, 0x830994c0
	if ctx.cr[6].lt {
		sub_830994C0(ctx, base);
		return;
	}
	// 830994A4: 419A0010  beq cr6, 0x830994b4
	if ctx.cr[6].eq {
		sub_830994B4(ctx, base);
		return;
	}
	// 830994A8: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 830994AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830994B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830994B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830994B4 size=12
    let mut pc: u32 = 0x830994B4;
    'dispatch: loop {
        match pc {
            0x830994B4 => {
    //   block [0x830994B4..0x830994C0)
	// 830994B4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830994B8: 388B351C  addi r4, r11, 0x351c
	ctx.r[4].s64 = ctx.r[11].s64 + 13596;
	// 830994BC: 4BF60804  b 0x82ff9cc0
	sub_82FF9CC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830994C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830994C0 size=12
    let mut pc: u32 = 0x830994C0;
    'dispatch: loop {
        match pc {
            0x830994C0 => {
    //   block [0x830994C0..0x830994CC)
	// 830994C0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830994C4: 388B3580  addi r4, r11, 0x3580
	ctx.r[4].s64 = ctx.r[11].s64 + 13696;
	// 830994C8: 4BF607F8  b 0x82ff9cc0
	sub_82FF9CC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830994CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830994CC size=12
    let mut pc: u32 = 0x830994CC;
    'dispatch: loop {
        match pc {
            0x830994CC => {
    //   block [0x830994CC..0x830994D8)
	// 830994CC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830994D0: 388B3578  addi r4, r11, 0x3578
	ctx.r[4].s64 = ctx.r[11].s64 + 13688;
	// 830994D4: 4BF607EC  b 0x82ff9cc0
	sub_82FF9CC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830994D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830994D8 size=12
    let mut pc: u32 = 0x830994D8;
    'dispatch: loop {
        match pc {
            0x830994D8 => {
    //   block [0x830994D8..0x830994E4)
	// 830994D8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830994DC: 388B3570  addi r4, r11, 0x3570
	ctx.r[4].s64 = ctx.r[11].s64 + 13680;
	// 830994E0: 4BF607E0  b 0x82ff9cc0
	sub_82FF9CC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830994E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830994E8 size=68
    let mut pc: u32 = 0x830994E8;
    'dispatch: loop {
        match pc {
            0x830994E8 => {
    //   block [0x830994E8..0x8309952C)
	// 830994E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830994EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830994F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830994F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830994F8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830994FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83099500: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 83099504: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83099508: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309950C: 41820008  beq 0x83099514
	if ctx.cr[0].eq {
	pc = 0x83099514; continue 'dispatch;
	}
	// 83099510: 4BF3EDD1  bl 0x82fd82e0
	ctx.lr = 0x83099514;
	sub_82FD82E0(ctx, base);
	// 83099514: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83099518: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309951C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83099520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83099524: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83099528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83099530 size=120
    let mut pc: u32 = 0x83099530;
    'dispatch: loop {
        match pc {
            0x83099530 => {
    //   block [0x83099530..0x830995A8)
	// 83099530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83099534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83099538: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309953C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83099540: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83099544: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83099548: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309954C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83099550: 419A0034  beq cr6, 0x83099584
	if ctx.cr[6].eq {
	pc = 0x83099584; continue 'dispatch;
	}
	// 83099554: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83099558: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309955C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83099560: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83099564: 4E800421  bctrl
	ctx.lr = 0x83099568;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83099568: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8309956C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83099570: 4BF5FD89  bl 0x82ff92f8
	ctx.lr = 0x83099574;
	sub_82FF92F8(ctx, base);
	// 83099574: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83099578: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309957C: 4BF60685  bl 0x82ff9c00
	ctx.lr = 0x83099580;
	sub_82FF9C00(ctx, base);
	// 83099580: 48000010  b 0x83099590
	pc = 0x83099590; continue 'dispatch;
	// 83099584: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83099588: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309958C: 4BF5FD6D  bl 0x82ff92f8
	ctx.lr = 0x83099590;
	sub_82FF92F8(ctx, base);
	// 83099590: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83099594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83099598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309959C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830995A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830995A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830995A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830995A8 size=104
    let mut pc: u32 = 0x830995A8;
    'dispatch: loop {
        match pc {
            0x830995A8 => {
    //   block [0x830995A8..0x83099610)
	// 830995A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830995AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830995B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830995B4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830995B8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830995BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830995C0: 4BF5FFB9  bl 0x82ff9578
	ctx.lr = 0x830995C4;
	sub_82FF9578(ctx, base);
	// 830995C4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830995C8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 830995CC: 41980020  blt cr6, 0x830995ec
	if ctx.cr[6].lt {
	pc = 0x830995EC; continue 'dispatch;
	}
	// 830995D0: 419A0010  beq cr6, 0x830995e0
	if ctx.cr[6].eq {
	pc = 0x830995E0; continue 'dispatch;
	}
	// 830995D4: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 830995D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830995DC: 48000020  b 0x830995fc
	pc = 0x830995FC; continue 'dispatch;
	// 830995E0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830995E4: 388B3404  addi r4, r11, 0x3404
	ctx.r[4].s64 = ctx.r[11].s64 + 13316;
	// 830995E8: 4800000C  b 0x830995f4
	pc = 0x830995F4; continue 'dispatch;
	// 830995EC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830995F0: 388B33FC  addi r4, r11, 0x33fc
	ctx.r[4].s64 = ctx.r[11].s64 + 13308;
	// 830995F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830995F8: 4BF606C9  bl 0x82ff9cc0
	ctx.lr = 0x830995FC;
	sub_82FF9CC0(ctx, base);
	// 830995FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83099600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83099604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83099608: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309960C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83099610 size=48
    let mut pc: u32 = 0x83099610;
    'dispatch: loop {
        match pc {
            0x83099610 => {
    //   block [0x83099610..0x83099640)
	// 83099610: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83099614: 9083001C  stw r4, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 83099618: 394BD934  addi r10, r11, -0x26cc
	ctx.r[10].s64 = ctx.r[11].s64 + -9932;
	// 8309961C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83099620: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83099624: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83099628: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8309962C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83099630: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83099634: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83099638: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8309963C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83099640 size=136
    let mut pc: u32 = 0x83099640;
    'dispatch: loop {
        match pc {
            0x83099640 => {
    //   block [0x83099640..0x830996C8)
	// 83099640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83099644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83099648: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309964C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83099650: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83099654: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83099658: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309965C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83099660: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83099664: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83099668: 4E800421  bctrl
	ctx.lr = 0x8309966C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309966C: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83099670: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83099674: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83099678: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309967C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83099680: 4E800421  bctrl
	ctx.lr = 0x83099684;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83099684: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83099688: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309968C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83099690: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83099694: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83099698: 4E800421  bctrl
	ctx.lr = 0x8309969C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309969C: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830996A0: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830996A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830996A8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830996AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830996B0: 4E800421  bctrl
	ctx.lr = 0x830996B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830996B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830996B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830996BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830996C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830996C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830996C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830996C8 size=112
    let mut pc: u32 = 0x830996C8;
    'dispatch: loop {
        match pc {
            0x830996C8 => {
    //   block [0x830996C8..0x83099738)
	// 830996C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830996CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830996D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830996D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830996D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830996DC: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 830996E0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830996E4: 4BF3EBB5  bl 0x82fd8298
	ctx.lr = 0x830996E8;
	sub_82FD8298(ctx, base);
	// 830996E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830996EC: 41820034  beq 0x83099720
	if ctx.cr[0].eq {
	pc = 0x83099720; continue 'dispatch;
	}
	// 830996F0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830996F4: 93E3001C  stw r31, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[31].u32 ) };
	// 830996F8: 394BD934  addi r10, r11, -0x26cc
	ctx.r[10].s64 = ctx.r[11].s64 + -9932;
	// 830996FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83099700: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83099704: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83099708: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8309970C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83099710: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83099714: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83099718: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8309971C: 48000008  b 0x83099724
	pc = 0x83099724; continue 'dispatch;
	// 83099720: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83099724: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83099728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309972C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83099730: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83099734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83099738 size=12
    let mut pc: u32 = 0x83099738;
    'dispatch: loop {
        match pc {
            0x83099738 => {
    //   block [0x83099738..0x83099744)
	// 83099738: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8309973C: 386B35E0  addi r3, r11, 0x35e0
	ctx.r[3].s64 = ctx.r[11].s64 + 13792;
	// 83099740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83099748 size=268
    let mut pc: u32 = 0x83099748;
    'dispatch: loop {
        match pc {
            0x83099748 => {
    //   block [0x83099748..0x83099854)
	// 83099748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309974C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83099750: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83099754: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83099758: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309975C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83099760: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83099764: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83099768: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 8309976C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83099770: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83099774: 41820060  beq 0x830997d4
	if ctx.cr[0].eq {
	pc = 0x830997D4; continue 'dispatch;
	}
	// 83099778: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309977C: 4BF5FB7D  bl 0x82ff92f8
	ctx.lr = 0x83099780;
	sub_82FF92F8(ctx, base);
	// 83099780: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83099784: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83099788: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8309978C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83099790: 4BF60171  bl 0x82ff9900
	ctx.lr = 0x83099794;
	sub_82FF9900(ctx, base);
	// 83099794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83099798: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8309979C: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830997A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830997A4: 4BF6015D  bl 0x82ff9900
	ctx.lr = 0x830997A8;
	sub_82FF9900(ctx, base);
	// 830997A8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830997AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830997B0: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830997B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830997B8: 4BF60149  bl 0x82ff9900
	ctx.lr = 0x830997BC;
	sub_82FF9900(ctx, base);
	// 830997BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830997C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830997C4: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830997C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830997CC: 4BF60135  bl 0x82ff9900
	ctx.lr = 0x830997D0;
	sub_82FF9900(ctx, base);
	// 830997D0: 4800006C  b 0x8309983c
	pc = 0x8309983C; continue 'dispatch;
	// 830997D4: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 830997D8: 4BF5FDA1  bl 0x82ff9578
	ctx.lr = 0x830997DC;
	sub_82FF9578(ctx, base);
	// 830997DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830997E0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 830997E4: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 830997E8: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 830997EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830997F0: 4BF60339  bl 0x82ff9b28
	ctx.lr = 0x830997F4;
	sub_82FF9B28(ctx, base);
	// 830997F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830997F8: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 830997FC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83099800: 389E000C  addi r4, r30, 0xc
	ctx.r[4].s64 = ctx.r[30].s64 + 12;
	// 83099804: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83099808: 4BF60321  bl 0x82ff9b28
	ctx.lr = 0x8309980C;
	sub_82FF9B28(ctx, base);
	// 8309980C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83099810: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 83099814: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83099818: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 8309981C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83099820: 4BF60309  bl 0x82ff9b28
	ctx.lr = 0x83099824;
	sub_82FF9B28(ctx, base);
	// 83099824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83099828: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 8309982C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83099830: 389E0014  addi r4, r30, 0x14
	ctx.r[4].s64 = ctx.r[30].s64 + 20;
	// 83099834: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83099838: 4BF602F1  bl 0x82ff9b28
	ctx.lr = 0x8309983C;
	sub_82FF9B28(ctx, base);
	// 8309983C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83099840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83099844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83099848: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309984C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83099850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83099858 size=8
    let mut pc: u32 = 0x83099858;
    'dispatch: loop {
        match pc {
            0x83099858 => {
    //   block [0x83099858..0x83099860)
	// 83099858: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309985C: 8216D994  lwz r16, -0x266c(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-9836 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83099860 size=216
    let mut pc: u32 = 0x83099860;
    'dispatch: loop {
        match pc {
            0x83099860 => {
    //   block [0x83099860..0x83099938)
	// 83099860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83099864: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83099868: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 8309986C: 4810E8F9  bl 0x831a8164
	ctx.lr = 0x83099870;
	sub_831A8130(ctx, base);
	// 83099870: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83099874: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83099878: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309987C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83099880: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83099884: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83099888: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 8309988C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83099890: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83099894: 911E001C  stw r8, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[8].u32 ) };
	// 83099898: 394BD934  addi r10, r11, -0x26cc
	ctx.r[10].s64 = ctx.r[11].s64 + -9932;
	// 8309989C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830998A0: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830998A4: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830998A8: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 830998AC: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830998B0: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830998B4: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830998B8: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830998BC: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 830998C0: 4BF372C1  bl 0x82fd0b80
	ctx.lr = 0x830998C4;
	sub_82FD0B80(ctx, base);
	// 830998C4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830998C8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830998CC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830998D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830998D4: 809E001C  lwz r4, 0x1c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 830998D8: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830998DC: 4BF372A5  bl 0x82fd0b80
	ctx.lr = 0x830998E0;
	sub_82FD0B80(ctx, base);
	// 830998E0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830998E4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830998E8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830998EC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830998F0: 809E001C  lwz r4, 0x1c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 830998F4: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830998F8: 4BF37289  bl 0x82fd0b80
	ctx.lr = 0x830998FC;
	sub_82FD0B80(ctx, base);
	// 830998FC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83099900: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83099904: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83099908: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8309990C: 809E001C  lwz r4, 0x1c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83099910: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83099914: 4BF3726D  bl 0x82fd0b80
	ctx.lr = 0x83099918;
	sub_82FD0B80(ctx, base);
	// 83099918: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8309991C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83099920: 907E0014  stw r3, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 83099924: 48000008  b 0x8309992c
	pc = 0x8309992C; continue 'dispatch;
	// 83099928: 83DF0094  lwz r30, 0x94(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8309992C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83099930: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83099934: 4810E880  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83099938 size=8
    let mut pc: u32 = 0x83099938;
    'dispatch: loop {
        match pc {
            0x83099938 => {
    //   block [0x83099938..0x83099940)
	// 83099938: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309993C: 8216D994  lwz r16, -0x266c(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-9836 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83099940 size=24
    let mut pc: u32 = 0x83099940;
    'dispatch: loop {
        match pc {
            0x83099940 => {
    //   block [0x83099940..0x83099958)
	// 83099940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83099944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83099948: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309994C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83099950: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83099954: 481172D5  bl 0x831b0c28
	ctx.lr = 0x83099958;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83099960 size=48
    let mut pc: u32 = 0x83099960;
    'dispatch: loop {
        match pc {
            0x83099960 => {
    //   block [0x83099960..0x83099990)
	// 83099960: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83099964: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83099968: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309996C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83099970: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83099974: 4BFFFCCD  bl 0x83099640
	ctx.lr = 0x83099978;
	sub_83099640(ctx, base);
	// 83099978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309997C: 3C60830A  lis r3, -0x7cf6
	ctx.r[3].s64 = -2096496640;
	// 83099980: 38639928  addi r3, r3, -0x66d8
	ctx.r[3].s64 = ctx.r[3].s64 + -26328;
	// 83099984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83099988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309998C: 4810E828  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83099990 size=40
    let mut pc: u32 = 0x83099990;
    'dispatch: loop {
        match pc {
            0x83099990 => {
    //   block [0x83099990..0x830999B8)
	// 83099990: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83099994: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83099998: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309999C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830999A0: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830999A4: 4BFB2DBD  bl 0x8304c760
	ctx.lr = 0x830999A8;
	sub_8304C760(ctx, base);
	// 830999A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830999AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830999B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830999B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830999B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830999B8 size=8
    let mut pc: u32 = 0x830999B8;
    'dispatch: loop {
        match pc {
            0x830999B8 => {
    //   block [0x830999B8..0x830999C0)
	// 830999B8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830999BC: 8216DA28  lwz r16, -0x25d8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-9688 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830999C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830999C0 size=84
    let mut pc: u32 = 0x830999C0;
    'dispatch: loop {
        match pc {
            0x830999C0 => {
    //   block [0x830999C0..0x83099A14)
	// 830999C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830999C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830999C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830999CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830999D0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830999D4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830999D8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830999DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830999E0: 396BD934  addi r11, r11, -0x26cc
	ctx.r[11].s64 = ctx.r[11].s64 + -9932;
	// 830999E4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830999E8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830999EC: 4BFFFC55  bl 0x83099640
	ctx.lr = 0x830999F0;
	sub_83099640(ctx, base);
	// 830999F0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830999F4: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 830999F8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830999FC: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83099A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83099A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83099A08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83099A0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83099A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099A14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83099A14 size=40
    let mut pc: u32 = 0x83099A14;
    'dispatch: loop {
        match pc {
            0x83099A14 => {
    //   block [0x83099A14..0x83099A3C)
	// 83099A14: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83099A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83099A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83099A20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83099A24: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83099A28: 4BFB2D39  bl 0x8304c760
	ctx.lr = 0x83099A2C;
	sub_8304C760(ctx, base);
	// 83099A2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83099A30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83099A34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83099A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83099A40 size=76
    let mut pc: u32 = 0x83099A40;
    'dispatch: loop {
        match pc {
            0x83099A40 => {
    //   block [0x83099A40..0x83099A8C)
	// 83099A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83099A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83099A48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83099A4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83099A50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83099A54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83099A58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83099A5C: 4BFFFF65  bl 0x830999c0
	ctx.lr = 0x83099A60;
	sub_830999C0(ctx, base);
	// 83099A60: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83099A64: 4182000C  beq 0x83099a70
	if ctx.cr[0].eq {
	pc = 0x83099A70; continue 'dispatch;
	}
	// 83099A68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83099A6C: 4BF3E875  bl 0x82fd82e0
	ctx.lr = 0x83099A70;
	sub_82FD82E0(ctx, base);
	// 83099A70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83099A74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83099A78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83099A7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83099A80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83099A84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83099A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83099A90 size=8
    let mut pc: u32 = 0x83099A90;
    'dispatch: loop {
        match pc {
            0x83099A90 => {
    //   block [0x83099A90..0x83099A98)
	// 83099A90: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83099A94: 8216DA60  lwz r16, -0x25a0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-9632 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83099A98 size=132
    let mut pc: u32 = 0x83099A98;
    'dispatch: loop {
        match pc {
            0x83099A98 => {
    //   block [0x83099A98..0x83099B1C)
	// 83099A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83099A9C: 4810E6CD  bl 0x831a8168
	ctx.lr = 0x83099AA0;
	sub_831A8130(ctx, base);
	// 83099AA0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83099AA4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83099AA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83099AAC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83099AB0: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 83099AB4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83099AB8: 3880006D  li r4, 0x6d
	ctx.r[4].s64 = 109;
	// 83099ABC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83099AC0: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83099AC4: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 83099AC8: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 83099ACC: 939E0008  stw r28, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 83099AD0: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83099AD4: 4BF64015  bl 0x82ffdae8
	ctx.lr = 0x83099AD8;
	sub_82FFDAE8(ctx, base);
	// 83099AD8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83099ADC: 93BE0028  stw r29, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 83099AE0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83099AE4: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83099AE8: 939E0024  stw r28, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[28].u32 ) };
	// 83099AEC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83099AF0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83099AF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83099AF8: 4E800421  bctrl
	ctx.lr = 0x83099AFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83099AFC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83099B00: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83099B04: 907E0024  stw r3, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 83099B08: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83099B0C: 4810E6D5  bl 0x831a81e0
	ctx.lr = 0x83099B10;
	sub_831A81E0(ctx, base);
	// 83099B10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83099B14: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83099B18: 4810E6A0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099B1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83099B1C size=44
    let mut pc: u32 = 0x83099B1C;
    'dispatch: loop {
        match pc {
            0x83099B1C => {
    //   block [0x83099B1C..0x83099B48)
	// 83099B1C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83099B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83099B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83099B28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83099B2C: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83099B30: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 83099B34: 4BF6462D  bl 0x82ffe160
	ctx.lr = 0x83099B38;
	sub_82FFE160(ctx, base);
	// 83099B38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83099B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83099B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83099B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83099B48 size=8
    let mut pc: u32 = 0x83099B48;
    'dispatch: loop {
        match pc {
            0x83099B48 => {
    //   block [0x83099B48..0x83099B50)
	// 83099B48: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83099B4C: 8216DA98  lwz r16, -0x2568(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-9576 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83099B50 size=160
    let mut pc: u32 = 0x83099B50;
    'dispatch: loop {
        match pc {
            0x83099B50 => {
    //   block [0x83099B50..0x83099BF0)
	// 83099B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83099B54: 4810E615  bl 0x831a8168
	ctx.lr = 0x83099B58;
	sub_831A8130(ctx, base);
	// 83099B58: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83099B5C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83099B60: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83099B64: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83099B68: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83099B6C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83099B70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83099B74: 40990054  ble cr6, 0x83099bc8
	if !ctx.cr[6].gt {
	pc = 0x83099BC8; continue 'dispatch;
	}
	// 83099B78: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83099B7C: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83099B80: 7D5D582E  lwzx r10, r29, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83099B84: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83099B88: 419A0040  beq cr6, 0x83099bc8
	if ctx.cr[6].eq {
	pc = 0x83099BC8; continue 'dispatch;
	}
	// 83099B8C: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83099B90: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83099B94: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83099B98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83099B9C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83099BA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83099BA4: 4E800421  bctrl
	ctx.lr = 0x83099BA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83099BA8: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83099BAC: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83099BB0: 4BF3E731  bl 0x82fd82e0
	ctx.lr = 0x83099BB4;
	sub_82FD82E0(ctx, base);
	// 83099BB4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83099BB8: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 83099BBC: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 83099BC0: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83099BC4: 4198FFB8  blt cr6, 0x83099b7c
	if ctx.cr[6].lt {
	pc = 0x83099B7C; continue 'dispatch;
	}
	// 83099BC8: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83099BCC: 809E0024  lwz r4, 0x24(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83099BD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83099BD4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83099BD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83099BDC: 4E800421  bctrl
	ctx.lr = 0x83099BE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83099BE0: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 83099BE4: 4BF6457D  bl 0x82ffe160
	ctx.lr = 0x83099BE8;
	sub_82FFE160(ctx, base);
	// 83099BE8: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83099BEC: 4810E5CC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83099BF0 size=44
    let mut pc: u32 = 0x83099BF0;
    'dispatch: loop {
        match pc {
            0x83099BF0 => {
    //   block [0x83099BF0..0x83099C1C)
	// 83099BF0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83099BF4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83099BF8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83099BFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83099C00: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83099C04: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 83099C08: 4BF64559  bl 0x82ffe160
	ctx.lr = 0x83099C0C;
	sub_82FFE160(ctx, base);
	// 83099C0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83099C10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83099C14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83099C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83099C20 size=200
    let mut pc: u32 = 0x83099C20;
    'dispatch: loop {
        match pc {
            0x83099C20 => {
    //   block [0x83099C20..0x83099CE8)
	// 83099C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83099C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83099C28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83099C2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83099C30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83099C34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83099C38: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83099C3C: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 83099C40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83099C44: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83099C48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83099C4C: 4E800421  bctrl
	ctx.lr = 0x83099C50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83099C50: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83099C54: 41820064  beq 0x83099cb8
	if ctx.cr[0].eq {
	pc = 0x83099CB8; continue 'dispatch;
	}
	// 83099C58: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83099C5C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83099C60: 41980058  blt cr6, 0x83099cb8
	if ctx.cr[6].lt {
	pc = 0x83099CB8; continue 'dispatch;
	}
	// 83099C64: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83099C68: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83099C6C: 7CEB5214  add r7, r11, r10
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83099C70: 81070000  lwz r8, 0(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 83099C74: 81280008  lwz r9, 8(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 83099C78: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83099C7C: 41820030  beq 0x83099cac
	if ctx.cr[0].eq {
	pc = 0x83099CAC; continue 'dispatch;
	}
	// 83099C80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83099C84: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83099C88: 419A0024  beq cr6, 0x83099cac
	if ctx.cr[6].eq {
	pc = 0x83099CAC; continue 'dispatch;
	}
	// 83099C8C: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 83099C90: 80AA0000  lwz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83099C94: 7F051840  cmplw cr6, r5, r3
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[3].u32, &mut ctx.xer);
	// 83099C98: 419A003C  beq cr6, 0x83099cd4
	if ctx.cr[6].eq {
	pc = 0x83099CD4; continue 'dispatch;
	}
	// 83099C9C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83099CA0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 83099CA4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83099CA8: 4198FFE8  blt cr6, 0x83099c90
	if ctx.cr[6].lt {
	pc = 0x83099C90; continue 'dispatch;
	}
	// 83099CAC: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 83099CB0: 38E7FFFC  addi r7, r7, -4
	ctx.r[7].s64 = ctx.r[7].s64 + -4;
	// 83099CB4: 4080FFBC  bge 0x83099c70
	if !ctx.cr[0].lt {
	pc = 0x83099C70; continue 'dispatch;
	}
	// 83099CB8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83099CBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83099CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83099CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83099CC8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83099CCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83099CD0: 4E800020  blr
	return;
	// 83099CD4: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 83099CD8: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83099CDC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83099CE0: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83099CE4: 4BFFFFD8  b 0x83099cbc
	pc = 0x83099CBC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83099CE8 size=84
    let mut pc: u32 = 0x83099CE8;
    'dispatch: loop {
        match pc {
            0x83099CE8 => {
    //   block [0x83099CE8..0x83099D3C)
	// 83099CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83099CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83099CF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83099CF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83099CF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83099CFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83099D00: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83099D04: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83099D08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83099D0C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83099D10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83099D14: 4E800421  bctrl
	ctx.lr = 0x83099D18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83099D18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83099D1C: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83099D20: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83099D24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83099D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83099D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83099D30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83099D34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83099D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83099D40 size=164
    let mut pc: u32 = 0x83099D40;
    'dispatch: loop {
        match pc {
            0x83099D40 => {
    //   block [0x83099D40..0x83099DE4)
	// 83099D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83099D44: 4810E421  bl 0x831a8164
	ctx.lr = 0x83099D48;
	sub_831A8130(ctx, base);
	// 83099D48: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83099D4C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83099D50: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83099D54: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83099D58: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83099D5C: 41820034  beq 0x83099d90
	if ctx.cr[0].eq {
	pc = 0x83099D90; continue 'dispatch;
	}
	// 83099D60: 7B8B0020  clrldi r11, r28, 0x20
	ctx.r[11].u64 = ctx.r[28].u64 & 0x00000000FFFFFFFFu64;
	// 83099D64: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 83099D68: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 83099D6C: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 83099D70: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 83099D74: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83099D78: C80B7390  lfd f0, 0x7390(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(29584 as u32) ) };
	// 83099D7C: FC0D0032  fmul f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 83099D80: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 83099D84: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 83099D88: 83A10050  lwz r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83099D8C: 48000008  b 0x83099d94
	pc = 0x83099D94; continue 'dispatch;
	// 83099D90: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 83099D94: 807B0028  lwz r3, 0x28(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(40 as u32) ) } as u64;
	// 83099D98: 57A41838  slwi r4, r29, 3
	ctx.r[4].u32 = ctx.r[29].u32.wrapping_shl(3);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83099D9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83099DA0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83099DA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83099DA8: 4E800421  bctrl
	ctx.lr = 0x83099DAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83099DAC: 57851838  slwi r5, r28, 3
	ctx.r[5].u32 = ctx.r[28].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83099DB0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83099DB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83099DB8: 4810E759  bl 0x831a8510
	ctx.lr = 0x83099DBC;
	sub_831A8510(ctx, base);
	// 83099DBC: 807B0028  lwz r3, 0x28(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(40 as u32) ) } as u64;
	// 83099DC0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83099DC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83099DC8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83099DCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83099DD0: 4E800421  bctrl
	ctx.lr = 0x83099DD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83099DD4: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83099DD8: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 83099DDC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83099DE0: 4810E3D4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83099DE8 size=172
    let mut pc: u32 = 0x83099DE8;
    'dispatch: loop {
        match pc {
            0x83099DE8 => {
    //   block [0x83099DE8..0x83099E94)
	// 83099DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83099DEC: 4810E381  bl 0x831a816c
	ctx.lr = 0x83099DF0;
	sub_831A8130(ctx, base);
	// 83099DF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83099DF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83099DF8: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 83099DFC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83099E00: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83099E04: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 83099E08: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 83099E0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83099E10: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 83099E14: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83099E18: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83099E1C: C80B7390  lfd f0, 0x7390(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(29584 as u32) ) };
	// 83099E20: FC0D0032  fmul f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 83099E24: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 83099E28: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 83099E2C: 83A10050  lwz r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83099E30: 57A4103A  slwi r4, r29, 2
	ctx.r[4].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83099E34: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83099E38: 4E800421  bctrl
	ctx.lr = 0x83099E3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83099E3C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83099E40: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83099E44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83099E48: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83099E4C: 4810E6C5  bl 0x831a8510
	ctx.lr = 0x83099E50;
	sub_831A8510(ctx, base);
	// 83099E50: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83099E54: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83099E58: 7D4BE850  subf r10, r11, r29
	ctx.r[10].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 83099E5C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83099E60: 5545103A  slwi r5, r10, 2
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83099E64: 7C6BF214  add r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 83099E68: 4810E379  bl 0x831a81e0
	ctx.lr = 0x83099E6C;
	sub_831A81E0(ctx, base);
	// 83099E6C: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83099E70: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83099E74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83099E78: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83099E7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83099E80: 4E800421  bctrl
	ctx.lr = 0x83099E84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83099E84: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 83099E88: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 83099E8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83099E90: 4810E32C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83099E98 size=200
    let mut pc: u32 = 0x83099E98;
    'dispatch: loop {
        match pc {
            0x83099E98 => {
    //   block [0x83099E98..0x83099F60)
	// 83099E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83099E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83099EA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83099EA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83099EA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83099EAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83099EB0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83099EB4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83099EB8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83099EBC: 409A0008  bne cr6, 0x83099ec4
	if !ctx.cr[6].eq {
	pc = 0x83099EC4; continue 'dispatch;
	}
	// 83099EC0: 4BFFFF29  bl 0x83099de8
	ctx.lr = 0x83099EC4;
	sub_83099DE8(ctx, base);
	// 83099EC4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83099EC8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83099ECC: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83099ED0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83099ED4: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83099ED8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83099EDC: 409A0048  bne cr6, 0x83099f24
	if !ctx.cr[6].eq {
	pc = 0x83099F24; continue 'dispatch;
	}
	// 83099EE0: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 83099EE4: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83099EE8: 4BF3E3B1  bl 0x82fd8298
	ctx.lr = 0x83099EEC;
	sub_82FD8298(ctx, base);
	// 83099EEC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83099EF0: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83099EF4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83099EF8: 7C6B512E  stwx r3, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[3].u32) };
	// 83099EFC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83099F00: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83099F04: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83099F08: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83099F0C: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 83099F10: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83099F14: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83099F18: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83099F1C: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83099F20: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83099F24: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83099F28: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83099F2C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83099F30: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83099F34: 93CB0008  stw r30, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 83099F38: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83099F3C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83099F40: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 83099F44: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83099F48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83099F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83099F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83099F54: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83099F58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83099F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83099F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83099F60 size=200
    let mut pc: u32 = 0x83099F60;
    'dispatch: loop {
        match pc {
            0x83099F60 => {
    //   block [0x83099F60..0x8309A028)
	// 83099F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83099F64: 4810E205  bl 0x831a8168
	ctx.lr = 0x83099F68;
	sub_831A8130(ctx, base);
	// 83099F68: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83099F6C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83099F70: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83099F74: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83099F78: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83099F7C: 40820030  bne 0x83099fac
	if !ctx.cr[0].eq {
	pc = 0x83099FAC; continue 'dispatch;
	}
	// 83099F80: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83099F84: 80FE0028  lwz r7, 0x28(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83099F88: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 83099F8C: 388BDAD8  addi r4, r11, -0x2528
	ctx.r[4].s64 = ctx.r[11].s64 + -9512;
	// 83099F90: 38A00092  li r5, 0x92
	ctx.r[5].s64 = 146;
	// 83099F94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83099F98: 4BF39DD9  bl 0x82fd3d70
	ctx.lr = 0x83099F9C;
	sub_82FD3D70(ctx, base);
	// 83099F9C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83099FA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83099FA4: 388BC550  addi r4, r11, -0x3ab0
	ctx.r[4].s64 = ctx.r[11].s64 + -15024;
	// 83099FA8: 48116C81  bl 0x831b0c28
	ctx.lr = 0x83099FAC;
	sub_831B0C28(ctx, base);
	// 83099FAC: 815E0024  lwz r10, 0x24(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83099FB0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83099FB4: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 83099FB8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83099FBC: 83EBFFFC  lwz r31, -4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83099FC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83099FC4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83099FC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83099FCC: 4E800421  bctrl
	ctx.lr = 0x83099FD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83099FD0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83099FD4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83099FD8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83099FDC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83099FE0: 409A0010  bne cr6, 0x83099ff0
	if !ctx.cr[6].eq {
	pc = 0x83099FF0; continue 'dispatch;
	}
	// 83099FE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83099FE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83099FEC: 4BFFFD55  bl 0x83099d40
	ctx.lr = 0x83099FF0;
	sub_83099D40(ctx, base);
	// 83099FF0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83099FF4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83099FF8: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83099FFC: 7FAB512E  stwx r29, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u32) };
	// 8309A000: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309A004: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309A008: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8309A00C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8309A010: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8309A014: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309A018: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8309A01C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8309A020: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8309A024: 4810E194  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309A028 size=16
    let mut pc: u32 = 0x8309A028;
    'dispatch: loop {
        match pc {
            0x8309A028 => {
    //   block [0x8309A028..0x8309A038)
	// 8309A028: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309A02C: 396BDB2C  addi r11, r11, -0x24d4
	ctx.r[11].s64 = ctx.r[11].s64 + -9428;
	// 8309A030: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309A034: 4BFB272C  b 0x8304c760
	sub_8304C760(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309A038 size=88
    let mut pc: u32 = 0x8309A038;
    'dispatch: loop {
        match pc {
            0x8309A038 => {
    //   block [0x8309A038..0x8309A090)
	// 8309A038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309A03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309A040: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309A044: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309A048: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309A04C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309A050: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309A054: 396BDB2C  addi r11, r11, -0x24d4
	ctx.r[11].s64 = ctx.r[11].s64 + -9428;
	// 8309A058: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309A05C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309A060: 4BFB2701  bl 0x8304c760
	ctx.lr = 0x8309A064;
	sub_8304C760(ctx, base);
	// 8309A064: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309A068: 4182000C  beq 0x8309a074
	if ctx.cr[0].eq {
	pc = 0x8309A074; continue 'dispatch;
	}
	// 8309A06C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309A070: 4BF3E271  bl 0x82fd82e0
	ctx.lr = 0x8309A074;
	sub_82FD82E0(ctx, base);
	// 8309A074: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309A078: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309A07C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309A080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309A084: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309A088: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309A08C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309A090 size=60
    let mut pc: u32 = 0x8309A090;
    'dispatch: loop {
        match pc {
            0x8309A090 => {
    //   block [0x8309A090..0x8309A0CC)
	// 8309A090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309A094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309A098: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309A09C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309A0A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309A0A4: 480125FD  bl 0x830ac6a0
	ctx.lr = 0x8309A0A8;
	sub_830AC6A0(ctx, base);
	// 8309A0A8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309A0AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309A0B0: 396BDB2C  addi r11, r11, -0x24d4
	ctx.r[11].s64 = ctx.r[11].s64 + -9428;
	// 8309A0B4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309A0B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309A0BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309A0C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309A0C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309A0C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309A0D0 size=12
    let mut pc: u32 = 0x8309A0D0;
    'dispatch: loop {
        match pc {
            0x8309A0D0 => {
    //   block [0x8309A0D0..0x8309A0DC)
	// 8309A0D0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8309A0D4: 386B35E8  addi r3, r11, 0x35e8
	ctx.r[3].s64 = ctx.r[11].s64 + 13800;
	// 8309A0D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309A0E0 size=4
    let mut pc: u32 = 0x8309A0E0;
    'dispatch: loop {
        match pc {
            0x8309A0E0 => {
    //   block [0x8309A0E0..0x8309A0E4)
	// 8309A0E0: 4802DA00  b 0x830c7ae0
	sub_830C7AE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309A0E8 size=96
    let mut pc: u32 = 0x8309A0E8;
    'dispatch: loop {
        match pc {
            0x8309A0E8 => {
    //   block [0x8309A0E8..0x8309A148)
	// 8309A0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309A0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309A0F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309A0F4: 7C872378  mr r7, r4
	ctx.r[7].u64 = ctx.r[4].u64;
	// 8309A0F8: 2B03000D  cmplwi cr6, r3, 0xd
	ctx.cr[6].compare_u32(ctx.r[3].u32, 13 as u32, &mut ctx.xer);
	// 8309A0FC: 41990024  bgt cr6, 0x8309a120
	if ctx.cr[6].gt {
	pc = 0x8309A120; continue 'dispatch;
	}
	// 8309A100: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309A104: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8309A108: 396BDB78  addi r11, r11, -0x2488
	ctx.r[11].s64 = ctx.r[11].s64 + -9352;
	// 8309A10C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8309A110: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309A114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309A118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309A11C: 4E800020  blr
	return;
	// 8309A120: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309A124: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 8309A128: 388BDBD8  addi r4, r11, -0x2428
	ctx.r[4].s64 = ctx.r[11].s64 + -9256;
	// 8309A12C: 38A0005B  li r5, 0x5b
	ctx.r[5].s64 = 91;
	// 8309A130: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8309A134: 4BF36825  bl 0x82fd0958
	ctx.lr = 0x8309A138;
	sub_82FD0958(ctx, base);
	// 8309A138: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8309A13C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8309A140: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 8309A144: 48116AE5  bl 0x831b0c28
	ctx.lr = 0x8309A148;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309A148 size=60
    let mut pc: u32 = 0x8309A148;
    'dispatch: loop {
        match pc {
            0x8309A148 => {
    //   block [0x8309A148..0x8309A184)
	// 8309A148: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309A14C: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 8309A150: 3920FFFE  li r9, -2
	ctx.r[9].s64 = -2;
	// 8309A154: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 8309A158: 394BDC0C  addi r10, r11, -0x23f4
	ctx.r[10].s64 = ctx.r[11].s64 + -9204;
	// 8309A15C: 90C30020  stw r6, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 8309A160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309A164: 91230014  stw r9, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8309A168: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8309A16C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8309A170: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 8309A174: 99630011  stb r11, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 8309A178: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8309A17C: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8309A180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309A188 size=104
    let mut pc: u32 = 0x8309A188;
    'dispatch: loop {
        match pc {
            0x8309A188 => {
    //   block [0x8309A188..0x8309A1F0)
	// 8309A188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309A18C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309A190: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309A194: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309A198: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309A19C: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8309A1A0: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309A1A4: 41820018  beq 0x8309a1bc
	if ctx.cr[0].eq {
	pc = 0x8309A1BC; continue 'dispatch;
	}
	// 8309A1A8: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8309A1AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309A1B0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309A1B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309A1B8: 4E800421  bctrl
	ctx.lr = 0x8309A1BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309A1BC: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8309A1C0: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309A1C4: 41820018  beq 0x8309a1dc
	if ctx.cr[0].eq {
	pc = 0x8309A1DC; continue 'dispatch;
	}
	// 8309A1C8: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8309A1CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309A1D0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309A1D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309A1D8: 4E800421  bctrl
	ctx.lr = 0x8309A1DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309A1DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309A1E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309A1E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309A1E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309A1EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309A1F0 size=12
    let mut pc: u32 = 0x8309A1F0;
    'dispatch: loop {
        match pc {
            0x8309A1F0 => {
    //   block [0x8309A1F0..0x8309A1FC)
	// 8309A1F0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8309A1F4: 386B35F0  addi r3, r11, 0x35f0
	ctx.r[3].s64 = ctx.r[11].s64 + 13808;
	// 8309A1F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309A200 size=324
    let mut pc: u32 = 0x8309A200;
    'dispatch: loop {
        match pc {
            0x8309A200 => {
    //   block [0x8309A200..0x8309A344)
	// 8309A200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309A204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309A208: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309A20C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309A210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309A214: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8309A218: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309A21C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309A220: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 8309A224: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8309A228: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309A22C: 41820074  beq 0x8309a2a0
	if ctx.cr[0].eq {
	pc = 0x8309A2A0; continue 'dispatch;
	}
	// 8309A230: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309A234: 4BF5F0C5  bl 0x82ff92f8
	ctx.lr = 0x8309A238;
	sub_82FF92F8(ctx, base);
	// 8309A238: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309A23C: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309A240: 4BF5F0B9  bl 0x82ff92f8
	ctx.lr = 0x8309A244;
	sub_82FF92F8(ctx, base);
	// 8309A244: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309A248: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309A24C: 4BF5F0AD  bl 0x82ff92f8
	ctx.lr = 0x8309A250;
	sub_82FF92F8(ctx, base);
	// 8309A250: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309A254: 889E0010  lbz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309A258: 4BF5EFA9  bl 0x82ff9200
	ctx.lr = 0x8309A25C;
	sub_82FF9200(ctx, base);
	// 8309A25C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309A260: 889E0011  lbz r4, 0x11(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(17 as u32) ) } as u64;
	// 8309A264: 4BF5EF9D  bl 0x82ff9200
	ctx.lr = 0x8309A268;
	sub_82FF9200(ctx, base);
	// 8309A268: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309A26C: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309A270: 4BF5F089  bl 0x82ff92f8
	ctx.lr = 0x8309A274;
	sub_82FF92F8(ctx, base);
	// 8309A274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8309A278: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8309A27C: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8309A280: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309A284: 4BF5F67D  bl 0x82ff9900
	ctx.lr = 0x8309A288;
	sub_82FF9900(ctx, base);
	// 8309A288: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8309A28C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8309A290: 809E001C  lwz r4, 0x1c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8309A294: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309A298: 4BF5F669  bl 0x82ff9900
	ctx.lr = 0x8309A29C;
	sub_82FF9900(ctx, base);
	// 8309A29C: 48000090  b 0x8309a32c
	pc = 0x8309A32C; continue 'dispatch;
	// 8309A2A0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8309A2A4: 4BF5F2D5  bl 0x82ff9578
	ctx.lr = 0x8309A2A8;
	sub_82FF9578(ctx, base);
	// 8309A2A8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309A2AC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8309A2B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309A2B4: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8309A2B8: 4BF5F2C1  bl 0x82ff9578
	ctx.lr = 0x8309A2BC;
	sub_82FF9578(ctx, base);
	// 8309A2BC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309A2C0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8309A2C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309A2C8: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8309A2CC: 4BF5F2AD  bl 0x82ff9578
	ctx.lr = 0x8309A2D0;
	sub_82FF9578(ctx, base);
	// 8309A2D0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309A2D4: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 8309A2D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309A2DC: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8309A2E0: 4BF5F1A1  bl 0x82ff9480
	ctx.lr = 0x8309A2E4;
	sub_82FF9480(ctx, base);
	// 8309A2E4: 389E0011  addi r4, r30, 0x11
	ctx.r[4].s64 = ctx.r[30].s64 + 17;
	// 8309A2E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309A2EC: 4BF5F195  bl 0x82ff9480
	ctx.lr = 0x8309A2F0;
	sub_82FF9480(ctx, base);
	// 8309A2F0: 389E0014  addi r4, r30, 0x14
	ctx.r[4].s64 = ctx.r[30].s64 + 20;
	// 8309A2F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309A2F8: 4BF5F281  bl 0x82ff9578
	ctx.lr = 0x8309A2FC;
	sub_82FF9578(ctx, base);
	// 8309A2FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8309A300: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 8309A304: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8309A308: 389E0018  addi r4, r30, 0x18
	ctx.r[4].s64 = ctx.r[30].s64 + 24;
	// 8309A30C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309A310: 4BF5F819  bl 0x82ff9b28
	ctx.lr = 0x8309A314;
	sub_82FF9B28(ctx, base);
	// 8309A314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8309A318: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 8309A31C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8309A320: 389E001C  addi r4, r30, 0x1c
	ctx.r[4].s64 = ctx.r[30].s64 + 28;
	// 8309A324: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309A328: 4BF5F801  bl 0x82ff9b28
	ctx.lr = 0x8309A32C;
	sub_82FF9B28(ctx, base);
	// 8309A32C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8309A330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309A334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309A338: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309A33C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309A340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309A348 size=8
    let mut pc: u32 = 0x8309A348;
    'dispatch: loop {
        match pc {
            0x8309A348 => {
    //   block [0x8309A348..0x8309A350)
	// 8309A348: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309A34C: 8216DC38  lwz r16, -0x23c8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-9160 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309A350 size=84
    let mut pc: u32 = 0x8309A350;
    'dispatch: loop {
        match pc {
            0x8309A350 => {
    //   block [0x8309A350..0x8309A3A4)
	// 8309A350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309A354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309A358: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309A35C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309A360: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8309A364: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309A368: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309A36C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309A370: 396BDC0C  addi r11, r11, -0x23f4
	ctx.r[11].s64 = ctx.r[11].s64 + -9204;
	// 8309A374: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8309A378: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309A37C: 4BFFFE0D  bl 0x8309a188
	ctx.lr = 0x8309A380;
	sub_8309A188(ctx, base);
	// 8309A380: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309A384: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 8309A388: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309A38C: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8309A390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309A394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309A398: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309A39C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309A3A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A3A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309A3A4 size=40
    let mut pc: u32 = 0x8309A3A4;
    'dispatch: loop {
        match pc {
            0x8309A3A4 => {
    //   block [0x8309A3A4..0x8309A3CC)
	// 8309A3A4: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8309A3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309A3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309A3B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309A3B4: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8309A3B8: 4BFB23A9  bl 0x8304c760
	ctx.lr = 0x8309A3BC;
	sub_8304C760(ctx, base);
	// 8309A3BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309A3C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309A3C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309A3C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309A3D0 size=76
    let mut pc: u32 = 0x8309A3D0;
    'dispatch: loop {
        match pc {
            0x8309A3D0 => {
    //   block [0x8309A3D0..0x8309A41C)
	// 8309A3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309A3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309A3D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309A3DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309A3E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309A3E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309A3E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309A3EC: 4BFFFF65  bl 0x8309a350
	ctx.lr = 0x8309A3F0;
	sub_8309A350(ctx, base);
	// 8309A3F0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309A3F4: 4182000C  beq 0x8309a400
	if ctx.cr[0].eq {
	pc = 0x8309A400; continue 'dispatch;
	}
	// 8309A3F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309A3FC: 4BF3DEE5  bl 0x82fd82e0
	ctx.lr = 0x8309A400;
	sub_82FD82E0(ctx, base);
	// 8309A400: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309A404: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309A408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309A40C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309A410: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309A414: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309A418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309A420 size=8
    let mut pc: u32 = 0x8309A420;
    'dispatch: loop {
        match pc {
            0x8309A420 => {
    //   block [0x8309A420..0x8309A428)
	// 8309A420: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309A424: 8216DCB4  lwz r16, -0x234c(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-9036 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309A428 size=164
    let mut pc: u32 = 0x8309A428;
    'dispatch: loop {
        match pc {
            0x8309A428 => {
    //   block [0x8309A428..0x8309A4CC)
	// 8309A428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309A42C: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 8309A430: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 8309A434: 4810DD39  bl 0x831a816c
	ctx.lr = 0x8309A438;
	sub_831A8130(ctx, base);
	// 8309A438: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8309A43C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309A440: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309A444: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8309A448: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 8309A44C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8309A450: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309A454: 3920FFFE  li r9, -2
	ctx.r[9].s64 = -2;
	// 8309A458: 90DE0004  stw r6, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8309A45C: 394BDC0C  addi r10, r11, -0x23f4
	ctx.r[10].s64 = ctx.r[11].s64 + -9204;
	// 8309A460: 90BE0008  stw r5, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 8309A464: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309A468: 911E0020  stw r8, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[8].u32 ) };
	// 8309A46C: 913E0014  stw r9, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8309A470: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8309A474: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8309A478: 997E0010  stb r11, 0x10(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 8309A47C: 997E0011  stb r11, 0x11(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 8309A480: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8309A484: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8309A488: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 8309A48C: 4BF366F5  bl 0x82fd0b80
	ctx.lr = 0x8309A490;
	sub_82FD0B80(ctx, base);
	// 8309A490: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8309A494: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8309A498: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8309A49C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309A4A0: 809E0020  lwz r4, 0x20(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 8309A4A4: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8309A4A8: 4BF366D9  bl 0x82fd0b80
	ctx.lr = 0x8309A4AC;
	sub_82FD0B80(ctx, base);
	// 8309A4AC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8309A4B0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8309A4B4: 907E001C  stw r3, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 8309A4B8: 48000008  b 0x8309a4c0
	pc = 0x8309A4C0; continue 'dispatch;
	// 8309A4BC: 83DF0084  lwz r30, 0x84(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8309A4C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309A4C4: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8309A4C8: 4810DCF4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A4CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309A4CC size=8
    let mut pc: u32 = 0x8309A4CC;
    'dispatch: loop {
        match pc {
            0x8309A4CC => {
    //   block [0x8309A4CC..0x8309A4D4)
	// 8309A4CC: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309A4D0: 8216DCB4  lwz r16, -0x234c(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-9036 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A4D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309A4D4 size=24
    let mut pc: u32 = 0x8309A4D4;
    'dispatch: loop {
        match pc {
            0x8309A4D4 => {
    //   block [0x8309A4D4..0x8309A4EC)
	// 8309A4D4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309A4D8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309A4DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309A4E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8309A4E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309A4E8: 48116741  bl 0x831b0c28
	ctx.lr = 0x8309A4EC;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A4F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309A4F4 size=48
    let mut pc: u32 = 0x8309A4F4;
    'dispatch: loop {
        match pc {
            0x8309A4F4 => {
    //   block [0x8309A4F4..0x8309A524)
	// 8309A4F4: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8309A4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309A4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309A500: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309A504: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8309A508: 4BFFFC81  bl 0x8309a188
	ctx.lr = 0x8309A50C;
	sub_8309A188(ctx, base);
	// 8309A50C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309A510: 3C60830A  lis r3, -0x7cf6
	ctx.r[3].s64 = -2096496640;
	// 8309A514: 3863A4BC  addi r3, r3, -0x5b44
	ctx.r[3].s64 = ctx.r[3].s64 + -23364;
	// 8309A518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309A51C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309A520: 4810DC9C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A524(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309A524 size=40
    let mut pc: u32 = 0x8309A524;
    'dispatch: loop {
        match pc {
            0x8309A524 => {
    //   block [0x8309A524..0x8309A54C)
	// 8309A524: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8309A528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309A52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309A530: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309A534: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8309A538: 4BFB2229  bl 0x8304c760
	ctx.lr = 0x8309A53C;
	sub_8304C760(ctx, base);
	// 8309A53C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309A540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309A544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309A548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309A550 size=16
    let mut pc: u32 = 0x8309A550;
    'dispatch: loop {
        match pc {
            0x8309A550 => {
    //   block [0x8309A550..0x8309A560)
	// 8309A550: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309A554: 396BDD38  addi r11, r11, -0x22c8
	ctx.r[11].s64 = ctx.r[11].s64 + -8904;
	// 8309A558: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309A55C: 4BFB2204  b 0x8304c760
	sub_8304C760(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309A560 size=88
    let mut pc: u32 = 0x8309A560;
    'dispatch: loop {
        match pc {
            0x8309A560 => {
    //   block [0x8309A560..0x8309A5B8)
	// 8309A560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309A564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309A568: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309A56C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309A570: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309A574: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309A578: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309A57C: 396BDD38  addi r11, r11, -0x22c8
	ctx.r[11].s64 = ctx.r[11].s64 + -8904;
	// 8309A580: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309A584: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309A588: 4BFB21D9  bl 0x8304c760
	ctx.lr = 0x8309A58C;
	sub_8304C760(ctx, base);
	// 8309A58C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309A590: 4182000C  beq 0x8309a59c
	if ctx.cr[0].eq {
	pc = 0x8309A59C; continue 'dispatch;
	}
	// 8309A594: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309A598: 4BF3DD49  bl 0x82fd82e0
	ctx.lr = 0x8309A59C;
	sub_82FD82E0(ctx, base);
	// 8309A59C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309A5A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309A5A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309A5A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309A5AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309A5B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309A5B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309A5B8 size=60
    let mut pc: u32 = 0x8309A5B8;
    'dispatch: loop {
        match pc {
            0x8309A5B8 => {
    //   block [0x8309A5B8..0x8309A5F4)
	// 8309A5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309A5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309A5C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309A5C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309A5C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309A5CC: 480120D5  bl 0x830ac6a0
	ctx.lr = 0x8309A5D0;
	sub_830AC6A0(ctx, base);
	// 8309A5D0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309A5D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309A5D8: 396BDD38  addi r11, r11, -0x22c8
	ctx.r[11].s64 = ctx.r[11].s64 + -8904;
	// 8309A5DC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309A5E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309A5E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309A5E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309A5EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309A5F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309A5F8 size=12
    let mut pc: u32 = 0x8309A5F8;
    'dispatch: loop {
        match pc {
            0x8309A5F8 => {
    //   block [0x8309A5F8..0x8309A604)
	// 8309A5F8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8309A5FC: 386B35F8  addi r3, r11, 0x35f8
	ctx.r[3].s64 = ctx.r[11].s64 + 13816;
	// 8309A600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309A608 size=64
    let mut pc: u32 = 0x8309A608;
    'dispatch: loop {
        match pc {
            0x8309A608 => {
    //   block [0x8309A608..0x8309A648)
	// 8309A608: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8309A60C: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 8309A610: 394B4220  addi r10, r11, 0x4220
	ctx.r[10].s64 = ctx.r[11].s64 + 16928;
	// 8309A614: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309A618: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8309A61C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8309A620: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8309A624: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8309A628: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8309A62C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8309A630: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8309A634: 9143001C  stw r10, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8309A638: 99630020  stb r11, 0x20(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u8 ) };
	// 8309A63C: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8309A640: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8309A644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309A648 size=208
    let mut pc: u32 = 0x8309A648;
    'dispatch: loop {
        match pc {
            0x8309A648 => {
    //   block [0x8309A648..0x8309A718)
	// 8309A648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309A64C: 4810DB21  bl 0x831a816c
	ctx.lr = 0x8309A650;
	sub_831A8130(ctx, base);
	// 8309A650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309A654: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309A658: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8309A65C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309A660: 419A00AC  beq cr6, 0x8309a70c
	if ctx.cr[6].eq {
	pc = 0x8309A70C; continue 'dispatch;
	}
	// 8309A664: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8309A668: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8309A66C: 409A00A0  bne cr6, 0x8309a70c
	if !ctx.cr[6].eq {
	pc = 0x8309A70C; continue 'dispatch;
	}
	// 8309A670: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8309A674: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309A678: 41820094  beq 0x8309a70c
	if ctx.cr[0].eq {
	pc = 0x8309A70C; continue 'dispatch;
	}
	// 8309A67C: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309A680: 2F0B000F  cmpwi cr6, r11, 0xf
	ctx.cr[6].compare_i32(ctx.r[11].s32, 15, &mut ctx.xer);
	// 8309A684: 409A0010  bne cr6, 0x8309a694
	if !ctx.cr[6].eq {
	pc = 0x8309A694; continue 'dispatch;
	}
	// 8309A688: 4BFE6A01  bl 0x83081088
	ctx.lr = 0x8309A68C;
	sub_83081088(ctx, base);
	// 8309A68C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8309A690: 409A007C  bne cr6, 0x8309a70c
	if !ctx.cr[6].eq {
	pc = 0x8309A70C; continue 'dispatch;
	}
	// 8309A694: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8309A698: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309A69C: 40820020  bne 0x8309a6bc
	if !ctx.cr[0].eq {
	pc = 0x8309A6BC; continue 'dispatch;
	}
	// 8309A6A0: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8309A6A4: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309A6A8: 2F0A0010  cmpwi cr6, r10, 0x10
	ctx.cr[6].compare_i32(ctx.r[10].s32, 16, &mut ctx.xer);
	// 8309A6AC: 419A0010  beq cr6, 0x8309a6bc
	if ctx.cr[6].eq {
	pc = 0x8309A6BC; continue 'dispatch;
	}
	// 8309A6B0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8309A6B4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309A6B8: 41820054  beq 0x8309a70c
	if ctx.cr[0].eq {
	pc = 0x8309A70C; continue 'dispatch;
	}
	// 8309A6BC: 806B002C  lwz r3, 0x2c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8309A6C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8309A6C4: 419A0048  beq cr6, 0x8309a70c
	if ctx.cr[6].eq {
	pc = 0x8309A70C; continue 'dispatch;
	}
	// 8309A6C8: 4BF75E79  bl 0x83010540
	ctx.lr = 0x8309A6CC;
	sub_83010540(ctx, base);
	// 8309A6CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309A6D0: 4182003C  beq 0x8309a70c
	if ctx.cr[0].eq {
	pc = 0x8309A70C; continue 'dispatch;
	}
	// 8309A6D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309A6D8: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309A6DC: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 8309A6E0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8309A6E4: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8309A6E8: 48013CC1  bl 0x830ae3a8
	ctx.lr = 0x8309A6EC;
	sub_830AE3A8(ctx, base);
	// 8309A6EC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8309A6F0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309A6F4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8309A6F8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8309A6FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8309A700: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 8309A704: 48014AC5  bl 0x830af1c8
	ctx.lr = 0x8309A708;
	sub_830AF1C8(ctx, base);
	// 8309A708: 48000008  b 0x8309a710
	pc = 0x8309A710; continue 'dispatch;
	// 8309A70C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309A710: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8309A714: 4810DAA8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309A718 size=96
    let mut pc: u32 = 0x8309A718;
    'dispatch: loop {
        match pc {
            0x8309A718 => {
    //   block [0x8309A718..0x8309A778)
	// 8309A718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309A71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309A720: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309A724: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309A728: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309A72C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309A730: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309A734: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309A738: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309A73C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309A740: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309A744: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309A748: 4E800421  bctrl
	ctx.lr = 0x8309A74C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309A74C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309A750: 4182000C  beq 0x8309a75c
	if ctx.cr[0].eq {
	pc = 0x8309A75C; continue 'dispatch;
	}
	// 8309A754: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309A758: 4BF3DB89  bl 0x82fd82e0
	ctx.lr = 0x8309A75C;
	sub_82FD82E0(ctx, base);
	// 8309A75C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309A760: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309A764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309A768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309A76C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309A770: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309A774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309A778 size=92
    let mut pc: u32 = 0x8309A778;
    'dispatch: loop {
        match pc {
            0x8309A778 => {
    //   block [0x8309A778..0x8309A7D4)
	// 8309A778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309A77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309A780: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309A784: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309A788: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309A78C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309A790: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309A794: 4182000C  beq 0x8309a7a0
	if ctx.cr[0].eq {
	pc = 0x8309A7A0; continue 'dispatch;
	}
	// 8309A798: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8309A79C: 4BFFFF7D  bl 0x8309a718
	ctx.lr = 0x8309A7A0;
	sub_8309A718(ctx, base);
	// 8309A7A0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309A7A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309A7A8: 41820018  beq 0x8309a7c0
	if ctx.cr[0].eq {
	pc = 0x8309A7C0; continue 'dispatch;
	}
	// 8309A7AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309A7B0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8309A7B4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309A7B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309A7BC: 4E800421  bctrl
	ctx.lr = 0x8309A7C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309A7C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309A7C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309A7C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309A7CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309A7D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309A7D8 size=84
    let mut pc: u32 = 0x8309A7D8;
    'dispatch: loop {
        match pc {
            0x8309A7D8 => {
    //   block [0x8309A7D8..0x8309A82C)
	// 8309A7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309A7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309A7E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309A7E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309A7E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309A7EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309A7F0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8309A7F4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309A7F8: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8309A7FC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309A800: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309A804: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309A808: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309A80C: 4E800421  bctrl
	ctx.lr = 0x8309A810;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309A810: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8309A814: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309A818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309A81C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309A820: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309A824: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309A828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309A830 size=8
    let mut pc: u32 = 0x8309A830;
    'dispatch: loop {
        match pc {
            0x8309A830 => {
    //   block [0x8309A830..0x8309A838)
	// 8309A830: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309A834: 8216DDEC  lwz r16, -0x2214(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-8724 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309A838 size=204
    let mut pc: u32 = 0x8309A838;
    'dispatch: loop {
        match pc {
            0x8309A838 => {
    //   block [0x8309A838..0x8309A904)
	// 8309A838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309A83C: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 8309A840: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 8309A844: 4810D921  bl 0x831a8164
	ctx.lr = 0x8309A848;
	sub_831A8130(ctx, base);
	// 8309A848: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8309A84C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309A850: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8309A854: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8309A858: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8309A85C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 8309A860: 939F00A4  stw r28, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[28].u32 ) };
	// 8309A864: 93BF00AC  stw r29, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[29].u32 ) };
	// 8309A868: 937C0000  stw r27, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8309A86C: 4BF3DA2D  bl 0x82fd8298
	ctx.lr = 0x8309A870;
	sub_82FD8298(ctx, base);
	// 8309A870: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8309A874: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8309A878: 4182001C  beq 0x8309a894
	if ctx.cr[0].eq {
	pc = 0x8309A894; continue 'dispatch;
	}
	// 8309A87C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8309A880: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8309A884: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 8309A888: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309A88C: 4BFCDF15  bl 0x830687a0
	ctx.lr = 0x8309A890;
	sub_830687A0(ctx, base);
	// 8309A890: 48000008  b 0x8309a898
	pc = 0x8309A898; continue 'dispatch;
	// 8309A894: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 8309A898: 93DC0004  stw r30, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8309A89C: 937C0008  stw r27, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 8309A8A0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8309A8A4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8309A8A8: 4BF3D9F1  bl 0x82fd8298
	ctx.lr = 0x8309A8AC;
	sub_82FD8298(ctx, base);
	// 8309A8AC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8309A8B0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8309A8B4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8309A8B8: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8309A8BC: 41820034  beq 0x8309a8f0
	if ctx.cr[0].eq {
	pc = 0x8309A8F0; continue 'dispatch;
	}
	// 8309A8C0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8309A8C4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8309A8C8: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 8309A8CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309A8D0: 4BFB1F21  bl 0x8304c7f0
	ctx.lr = 0x8309A8D4;
	sub_8304C7F0(ctx, base);
	// 8309A8D4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8309A8D8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8309A8DC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8309A8E0: 394B2990  addi r10, r11, 0x2990
	ctx.r[10].s64 = ctx.r[11].s64 + 10640;
	// 8309A8E4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8309A8E8: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8309A8EC: 48000008  b 0x8309a8f4
	pc = 0x8309A8F4; continue 'dispatch;
	// 8309A8F0: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 8309A8F4: 917C0008  stw r11, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8309A8F8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309A8FC: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8309A900: 4810D8B4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A904(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309A904 size=8
    let mut pc: u32 = 0x8309A904;
    'dispatch: loop {
        match pc {
            0x8309A904 => {
    //   block [0x8309A904..0x8309A90C)
	// 8309A904: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309A908: 8216DDEC  lwz r16, -0x2214(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-8724 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A90C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309A90C size=24
    let mut pc: u32 = 0x8309A90C;
    'dispatch: loop {
        match pc {
            0x8309A90C => {
    //   block [0x8309A90C..0x8309A924)
	// 8309A90C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309A910: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309A914: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309A918: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8309A91C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309A920: 48116309  bl 0x831b0c28
	ctx.lr = 0x8309A924;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A92C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309A92C size=52
    let mut pc: u32 = 0x8309A92C;
    'dispatch: loop {
        match pc {
            0x8309A92C => {
    //   block [0x8309A92C..0x8309A960)
	// 8309A92C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8309A930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309A934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309A938: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309A93C: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8309A940: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309A944: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309A948: 4182000C  beq 0x8309a954
	if ctx.cr[0].eq {
	pc = 0x8309A954; continue 'dispatch;
	}
	// 8309A94C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8309A950: 4BFFFDC9  bl 0x8309a718
	ctx.lr = 0x8309A954;
	sub_8309A718(ctx, base);
	// 8309A954: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8309A958: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309A95C: 481162CD  bl 0x831b0c28
	ctx.lr = 0x8309A960;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309A960 size=44
    let mut pc: u32 = 0x8309A960;
    'dispatch: loop {
        match pc {
            0x8309A960 => {
    //   block [0x8309A960..0x8309A98C)
	// 8309A960: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8309A964: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309A968: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309A96C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309A970: 809F00AC  lwz r4, 0xac(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 8309A974: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309A978: 4BF3D969  bl 0x82fd82e0
	ctx.lr = 0x8309A97C;
	sub_82FD82E0(ctx, base);
	// 8309A97C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309A980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309A984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309A988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A98C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309A98C size=44
    let mut pc: u32 = 0x8309A98C;
    'dispatch: loop {
        match pc {
            0x8309A98C => {
    //   block [0x8309A98C..0x8309A9B8)
	// 8309A98C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8309A990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309A994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309A998: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309A99C: 809F00AC  lwz r4, 0xac(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 8309A9A0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309A9A4: 4BF3D93D  bl 0x82fd82e0
	ctx.lr = 0x8309A9A8;
	sub_82FD82E0(ctx, base);
	// 8309A9A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309A9AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309A9B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309A9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309A9B8 size=16
    let mut pc: u32 = 0x8309A9B8;
    'dispatch: loop {
        match pc {
            0x8309A9B8 => {
    //   block [0x8309A9B8..0x8309A9C8)
	// 8309A9B8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309A9BC: 396BDE78  addi r11, r11, -0x2188
	ctx.r[11].s64 = ctx.r[11].s64 + -8584;
	// 8309A9C0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309A9C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309A9C8 size=16
    let mut pc: u32 = 0x8309A9C8;
    'dispatch: loop {
        match pc {
            0x8309A9C8 => {
    //   block [0x8309A9C8..0x8309A9D8)
	// 8309A9C8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309A9CC: 396BDE90  addi r11, r11, -0x2170
	ctx.r[11].s64 = ctx.r[11].s64 + -8560;
	// 8309A9D0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309A9D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309A9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309A9D8 size=128
    let mut pc: u32 = 0x8309A9D8;
    'dispatch: loop {
        match pc {
            0x8309A9D8 => {
    //   block [0x8309A9D8..0x8309AA58)
	// 8309A9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309A9DC: 4810D791  bl 0x831a816c
	ctx.lr = 0x8309A9E0;
	sub_831A8130(ctx, base);
	// 8309A9E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309A9E4: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309A9E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309A9EC: 396BDE78  addi r11, r11, -0x2188
	ctx.r[11].s64 = ctx.r[11].s64 + -8584;
	// 8309A9F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309A9F4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8309A9F8: 57C4103A  slwi r4, r30, 2
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8309A9FC: 98BF0004  stb r5, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 8309AA00: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 8309AA04: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309AA08: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8309AA0C: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8309AA10: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 8309AA14: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 8309AA18: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309AA1C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309AA20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309AA24: 4E800421  bctrl
	ctx.lr = 0x8309AA28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309AA28: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8309AA2C: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8309AA30: 419A001C  beq cr6, 0x8309aa4c
	if ctx.cr[6].eq {
	pc = 0x8309AA4C; continue 'dispatch;
	}
	// 8309AA34: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8309AA38: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309AA3C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8309AA40: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 8309AA44: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8309AA48: 4082FFF0  bne 0x8309aa38
	if !ctx.cr[0].eq {
	pc = 0x8309AA38; continue 'dispatch;
	}
	// 8309AA4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309AA50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309AA54: 4810D768  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309AA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309AA58 size=212
    let mut pc: u32 = 0x8309AA58;
    'dispatch: loop {
        match pc {
            0x8309AA58 => {
    //   block [0x8309AA58..0x8309AB2C)
	// 8309AA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309AA5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309AA60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309AA64: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8309AA68: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309AA6C: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8309AA70: 41980030  blt cr6, 0x8309aaa0
	if ctx.cr[6].lt {
	pc = 0x8309AAA0; continue 'dispatch;
	}
	// 8309AA74: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 8309AA78: 80EB0014  lwz r7, 0x14(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309AA7C: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 8309AA80: 388A6B80  addi r4, r10, 0x6b80
	ctx.r[4].s64 = ctx.r[10].s64 + 27520;
	// 8309AA84: 38A00065  li r5, 0x65
	ctx.r[5].s64 = 101;
	// 8309AA88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8309AA8C: 4BF35ECD  bl 0x82fd0958
	ctx.lr = 0x8309AA90;
	sub_82FD0958(ctx, base);
	// 8309AA90: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8309AA94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8309AA98: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 8309AA9C: 4811618D  bl 0x831b0c28
	ctx.lr = 0x8309AAA0;
	sub_831B0C28(ctx, base);
	// 8309AAA0: 3909FFFF  addi r8, r9, -1
	ctx.r[8].s64 = ctx.r[9].s64 + -1;
	// 8309AAA4: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309AAA8: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8309AAAC: 7F044040  cmplw cr6, r4, r8
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8309AAB0: 7C6A482E  lwzx r3, r10, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8309AAB4: 409A0010  bne cr6, 0x8309aac4
	if !ctx.cr[6].eq {
	pc = 0x8309AAC4; continue 'dispatch;
	}
	// 8309AAB8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8309AABC: 7D0A492E  stwx r8, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u32) };
	// 8309AAC0: 48000050  b 0x8309ab10
	pc = 0x8309AB10; continue 'dispatch;
	// 8309AAC4: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 8309AAC8: 7F044040  cmplw cr6, r4, r8
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8309AACC: 4098002C  bge cr6, 0x8309aaf8
	if !ctx.cr[6].lt {
	pc = 0x8309AAF8; continue 'dispatch;
	}
	// 8309AAD0: 810B0010  lwz r8, 0x10(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309AAD4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8309AAD8: 7D0A4214  add r8, r10, r8
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 8309AADC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8309AAE0: 80E80004  lwz r7, 4(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309AAE4: 90E80000  stw r7, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8309AAE8: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309AAEC: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8309AAF0: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8309AAF4: 4198FFDC  blt cr6, 0x8309aad0
	if ctx.cr[6].lt {
	pc = 0x8309AAD0; continue 'dispatch;
	}
	// 8309AAF8: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309AAFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8309AB00: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309AB04: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8309AB08: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8309AB0C: 910AFFFC  stw r8, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[8].u32 ) };
	// 8309AB10: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309AB14: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8309AB18: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8309AB1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309AB20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309AB24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309AB28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309AB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309AB30 size=128
    let mut pc: u32 = 0x8309AB30;
    'dispatch: loop {
        match pc {
            0x8309AB30 => {
    //   block [0x8309AB30..0x8309ABB0)
	// 8309AB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309AB34: 4810D639  bl 0x831a816c
	ctx.lr = 0x8309AB38;
	sub_831A8130(ctx, base);
	// 8309AB38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309AB3C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309AB40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309AB44: 396BDE90  addi r11, r11, -0x2170
	ctx.r[11].s64 = ctx.r[11].s64 + -8560;
	// 8309AB48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309AB4C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8309AB50: 57C4103A  slwi r4, r30, 2
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8309AB54: 98BF0004  stb r5, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 8309AB58: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 8309AB5C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309AB60: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8309AB64: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8309AB68: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 8309AB6C: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 8309AB70: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309AB74: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309AB78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309AB7C: 4E800421  bctrl
	ctx.lr = 0x8309AB80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309AB80: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8309AB84: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8309AB88: 419A001C  beq cr6, 0x8309aba4
	if ctx.cr[6].eq {
	pc = 0x8309ABA4; continue 'dispatch;
	}
	// 8309AB8C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8309AB90: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309AB94: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8309AB98: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 8309AB9C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8309ABA0: 4082FFF0  bne 0x8309ab90
	if !ctx.cr[0].eq {
	pc = 0x8309AB90; continue 'dispatch;
	}
	// 8309ABA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309ABA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309ABAC: 4810D610  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309ABB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309ABB0 size=8
    let mut pc: u32 = 0x8309ABB0;
    'dispatch: loop {
        match pc {
            0x8309ABB0 => {
    //   block [0x8309ABB0..0x8309ABB8)
	// 8309ABB0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309ABB4: 8216DEC8  lwz r16, -0x2138(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-8504 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309ABB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309ABB8 size=164
    let mut pc: u32 = 0x8309ABB8;
    'dispatch: loop {
        match pc {
            0x8309ABB8 => {
    //   block [0x8309ABB8..0x8309AC5C)
	// 8309ABB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309ABBC: 4810D5A9  bl 0x831a8164
	ctx.lr = 0x8309ABC0;
	sub_831A8130(ctx, base);
	// 8309ABC0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8309ABC4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309ABC8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309ABCC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309ABD0: 396BDEA8  addi r11, r11, -0x2158
	ctx.r[11].s64 = ctx.r[11].s64 + -8536;
	// 8309ABD4: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8309ABD8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309ABDC: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309ABE0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309ABE4: 4182004C  beq 0x8309ac30
	if ctx.cr[0].eq {
	pc = 0x8309AC30; continue 'dispatch;
	}
	// 8309ABE8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309ABEC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8309ABF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309ABF4: 4099003C  ble cr6, 0x8309ac30
	if !ctx.cr[6].gt {
	pc = 0x8309AC30; continue 'dispatch;
	}
	// 8309ABF8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8309ABFC: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309AC00: 7F8BE82E  lwzx r28, r11, r29
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8309AC04: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309AC08: 41820014  beq 0x8309ac1c
	if ctx.cr[0].eq {
	pc = 0x8309AC1C; continue 'dispatch;
	}
	// 8309AC0C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309AC10: 480019D1  bl 0x8309c5e0
	ctx.lr = 0x8309AC14;
	sub_8309C5E0(ctx, base);
	// 8309AC14: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309AC18: 4BF3D6C9  bl 0x82fd82e0
	ctx.lr = 0x8309AC1C;
	sub_82FD82E0(ctx, base);
	// 8309AC1C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309AC20: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 8309AC24: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8309AC28: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309AC2C: 4198FFD0  blt cr6, 0x8309abfc
	if ctx.cr[6].lt {
	pc = 0x8309ABFC; continue 'dispatch;
	}
	// 8309AC30: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309AC34: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309AC38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309AC3C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309AC40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309AC44: 4E800421  bctrl
	ctx.lr = 0x8309AC48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309AC48: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309AC4C: 396BDE78  addi r11, r11, -0x2188
	ctx.r[11].s64 = ctx.r[11].s64 + -8584;
	// 8309AC50: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309AC54: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8309AC58: 4810D55C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309AC5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309AC5C size=40
    let mut pc: u32 = 0x8309AC5C;
    'dispatch: loop {
        match pc {
            0x8309AC5C => {
    //   block [0x8309AC5C..0x8309AC84)
	// 8309AC5C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8309AC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309AC64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309AC68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309AC6C: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8309AC70: 4BFFFD49  bl 0x8309a9b8
	ctx.lr = 0x8309AC74;
	sub_8309A9B8(ctx, base);
	// 8309AC74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309AC78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309AC7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309AC80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309AC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309AC88 size=160
    let mut pc: u32 = 0x8309AC88;
    'dispatch: loop {
        match pc {
            0x8309AC88 => {
    //   block [0x8309AC88..0x8309AD28)
	// 8309AC88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309AC8C: 4810D4D1  bl 0x831a815c
	ctx.lr = 0x8309AC90;
	sub_831A8130(ctx, base);
	// 8309AC90: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309AC94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309AC98: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8309AC9C: 7F3ACB78  mr r26, r25
	ctx.r[26].u64 = ctx.r[25].u64;
	// 8309ACA0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309ACA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309ACA8: 40990074  ble cr6, 0x8309ad1c
	if !ctx.cr[6].gt {
	pc = 0x8309AD1C; continue 'dispatch;
	}
	// 8309ACAC: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 8309ACB0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309ACB4: 7FCBE82E  lwzx r30, r11, r29
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8309ACB8: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309ACBC: 41820044  beq 0x8309ad00
	if ctx.cr[0].eq {
	pc = 0x8309AD00; continue 'dispatch;
	}
	// 8309ACC0: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309ACC4: 837E0004  lwz r27, 4(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309ACC8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309ACCC: 41820020  beq 0x8309acec
	if ctx.cr[0].eq {
	pc = 0x8309ACEC; continue 'dispatch;
	}
	// 8309ACD0: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309ACD4: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309ACD8: 41820014  beq 0x8309acec
	if ctx.cr[0].eq {
	pc = 0x8309ACEC; continue 'dispatch;
	}
	// 8309ACDC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309ACE0: 48001901  bl 0x8309c5e0
	ctx.lr = 0x8309ACE4;
	sub_8309C5E0(ctx, base);
	// 8309ACE4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309ACE8: 4BF3D5F9  bl 0x82fd82e0
	ctx.lr = 0x8309ACEC;
	sub_82FD82E0(ctx, base);
	// 8309ACEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309ACF0: 4BF3D5F1  bl 0x82fd82e0
	ctx.lr = 0x8309ACF4;
	sub_82FD82E0(ctx, base);
	// 8309ACF4: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 8309ACF8: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8309ACFC: 409AFFC4  bne cr6, 0x8309acc0
	if !ctx.cr[6].eq {
	pc = 0x8309ACC0; continue 'dispatch;
	}
	// 8309AD00: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309AD04: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 8309AD08: 7F2BE92E  stwx r25, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[25].u32) };
	// 8309AD0C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8309AD10: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309AD14: 7F1A5840  cmplw cr6, r26, r11
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309AD18: 4198FF98  blt cr6, 0x8309acb0
	if ctx.cr[6].lt {
	pc = 0x8309ACB0; continue 'dispatch;
	}
	// 8309AD1C: 933F0014  stw r25, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[25].u32 ) };
	// 8309AD20: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8309AD24: 4810D488  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309AD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309AD28 size=220
    let mut pc: u32 = 0x8309AD28;
    'dispatch: loop {
        match pc {
            0x8309AD28 => {
    //   block [0x8309AD28..0x8309AE04)
	// 8309AD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309AD2C: 4810D43D  bl 0x831a8168
	ctx.lr = 0x8309AD30;
	sub_831A8130(ctx, base);
	// 8309AD30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309AD34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309AD38: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8309AD3C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8309AD40: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309AD44: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309AD48: 1D6B0003  mulli r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 * 3;
	// 8309AD4C: 556BF0BE  srwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8309AD50: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309AD54: 41980008  blt cr6, 0x8309ad5c
	if ctx.cr[6].lt {
	pc = 0x8309AD5C; continue 'dispatch;
	}
	// 8309AD58: 4BFCDC91  bl 0x830689e8
	ctx.lr = 0x8309AD5C;
	sub_830689E8(ctx, base);
	// 8309AD5C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8309AD60: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8309AD64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309AD68: 4BF5F6F9  bl 0x82ffa460
	ctx.lr = 0x8309AD6C;
	sub_82FFA460(ctx, base);
	// 8309AD6C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8309AD70: 41820038  beq 0x8309ada8
	if ctx.cr[0].eq {
	pc = 0x8309ADA8; continue 'dispatch;
	}
	// 8309AD74: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309AD78: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309AD7C: 41820020  beq 0x8309ad9c
	if ctx.cr[0].eq {
	pc = 0x8309AD9C; continue 'dispatch;
	}
	// 8309AD80: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309AD84: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309AD88: 41820014  beq 0x8309ad9c
	if ctx.cr[0].eq {
	pc = 0x8309AD9C; continue 'dispatch;
	}
	// 8309AD8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309AD90: 48001851  bl 0x8309c5e0
	ctx.lr = 0x8309AD94;
	sub_8309C5E0(ctx, base);
	// 8309AD94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309AD98: 4BF3D549  bl 0x82fd82e0
	ctx.lr = 0x8309AD9C;
	sub_82FD82E0(ctx, base);
	// 8309AD9C: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8309ADA0: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8309ADA4: 48000058  b 0x8309adfc
	pc = 0x8309ADFC; continue 'dispatch;
	// 8309ADA8: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8309ADAC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309ADB0: 4BF3D4E9  bl 0x82fd8298
	ctx.lr = 0x8309ADB4;
	sub_82FD8298(ctx, base);
	// 8309ADB4: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309ADB8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309ADBC: 41820024  beq 0x8309ade0
	if ctx.cr[0].eq {
	pc = 0x8309ADE0; continue 'dispatch;
	}
	// 8309ADC0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309ADC4: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8309ADC8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8309ADCC: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8309ADD0: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8309ADD4: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8309ADD8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8309ADDC: 48000008  b 0x8309ade4
	pc = 0x8309ADE4; continue 'dispatch;
	// 8309ADE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8309ADE4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309ADE8: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8309ADEC: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 8309ADF0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309ADF4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8309ADF8: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8309ADFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8309AE00: 4810D3B8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309AE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309AE08 size=184
    let mut pc: u32 = 0x8309AE08;
    'dispatch: loop {
        match pc {
            0x8309AE08 => {
    //   block [0x8309AE08..0x8309AEC0)
	// 8309AE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309AE0C: 4810D359  bl 0x831a8164
	ctx.lr = 0x8309AE10;
	sub_831A8130(ctx, base);
	// 8309AE10: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309AE14: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8309AE18: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8309AE1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309AE20: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8309AE24: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8309AE28: 4BFCDDD9  bl 0x83068c00
	ctx.lr = 0x8309AE2C;
	sub_83068C00(ctx, base);
	// 8309AE2C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8309AE30: 4182003C  beq 0x8309ae6c
	if ctx.cr[0].eq {
	pc = 0x8309AE6C; continue 'dispatch;
	}
	// 8309AE34: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309AE38: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309AE3C: 41820020  beq 0x8309ae5c
	if ctx.cr[0].eq {
	pc = 0x8309AE5C; continue 'dispatch;
	}
	// 8309AE40: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309AE44: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309AE48: 41820014  beq 0x8309ae5c
	if ctx.cr[0].eq {
	pc = 0x8309AE5C; continue 'dispatch;
	}
	// 8309AE4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309AE50: 48001791  bl 0x8309c5e0
	ctx.lr = 0x8309AE54;
	sub_8309C5E0(ctx, base);
	// 8309AE54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309AE58: 4BF3D489  bl 0x82fd82e0
	ctx.lr = 0x8309AE5C;
	sub_82FD82E0(ctx, base);
	// 8309AE5C: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8309AE60: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8309AE64: 939E000C  stw r28, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 8309AE68: 48000050  b 0x8309aeb8
	pc = 0x8309AEB8; continue 'dispatch;
	// 8309AE6C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8309AE70: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309AE74: 4BF3D425  bl 0x82fd8298
	ctx.lr = 0x8309AE78;
	sub_82FD8298(ctx, base);
	// 8309AE78: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309AE7C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309AE80: 41820028  beq 0x8309aea8
	if ctx.cr[0].eq {
	pc = 0x8309AEA8; continue 'dispatch;
	}
	// 8309AE84: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309AE88: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8309AE8C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8309AE90: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8309AE94: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8309AE98: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8309AE9C: 9383000C  stw r28, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 8309AEA0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8309AEA4: 48000008  b 0x8309aeac
	pc = 0x8309AEAC; continue 'dispatch;
	// 8309AEA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8309AEAC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309AEB0: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8309AEB4: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 8309AEB8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8309AEBC: 4810D2F8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309AEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309AEC0 size=8
    let mut pc: u32 = 0x8309AEC0;
    'dispatch: loop {
        match pc {
            0x8309AEC0 => {
    //   block [0x8309AEC0..0x8309AEC8)
	// 8309AEC0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309AEC4: 8216DF10  lwz r16, -0x20f0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-8432 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309AEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309AEC8 size=160
    let mut pc: u32 = 0x8309AEC8;
    'dispatch: loop {
        match pc {
            0x8309AEC8 => {
    //   block [0x8309AEC8..0x8309AF68)
	// 8309AEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309AECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309AED0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309AED4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309AED8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8309AEDC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309AEE0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309AEE4: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 8309AEE8: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8309AEEC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309AEF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8309AEF4: 98BE0004  stb r5, 4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 8309AEF8: 396BDEF8  addi r11, r11, -0x2108
	ctx.r[11].s64 = ctx.r[11].s64 + -8456;
	// 8309AEFC: 909E0010  stw r4, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 8309AF00: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 8309AF04: 90FE0014  stw r7, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 8309AF08: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8309AF0C: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8309AF10: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309AF14: 913E000C  stw r9, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8309AF18: 409A002C  bne cr6, 0x8309af44
	if !ctx.cr[6].eq {
	pc = 0x8309AF44; continue 'dispatch;
	}
	// 8309AF1C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309AF20: 38C0000F  li r6, 0xf
	ctx.r[6].s64 = 15;
	// 8309AF24: 388BB9A0  addi r4, r11, -0x4660
	ctx.r[4].s64 = ctx.r[11].s64 + -18016;
	// 8309AF28: 38A00259  li r5, 0x259
	ctx.r[5].s64 = 601;
	// 8309AF2C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8309AF30: 4BF45FC1  bl 0x82fe0ef0
	ctx.lr = 0x8309AF34;
	sub_82FE0EF0(ctx, base);
	// 8309AF34: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8309AF38: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8309AF3C: 388BC654  addi r4, r11, -0x39ac
	ctx.r[4].s64 = ctx.r[11].s64 + -14764;
	// 8309AF40: 48115CE9  bl 0x831b0c28
	ctx.lr = 0x8309AF44;
	sub_831B0C28(ctx, base);
	// 8309AF44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309AF48: 4BF5F7A1  bl 0x82ffa6e8
	ctx.lr = 0x8309AF4C;
	sub_82FFA6E8(ctx, base);
	// 8309AF4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309AF50: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8309AF54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309AF58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309AF5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309AF60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309AF64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309AF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309AF68 size=40
    let mut pc: u32 = 0x8309AF68;
    'dispatch: loop {
        match pc {
            0x8309AF68 => {
    //   block [0x8309AF68..0x8309AF90)
	// 8309AF68: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8309AF6C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309AF70: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309AF74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309AF78: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8309AF7C: 4BFB17E5  bl 0x8304c760
	ctx.lr = 0x8309AF80;
	sub_8304C760(ctx, base);
	// 8309AF80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309AF84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309AF88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309AF8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309AF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309AF90 size=20
    let mut pc: u32 = 0x8309AF90;
    'dispatch: loop {
        match pc {
            0x8309AF90 => {
    //   block [0x8309AF90..0x8309AFA4)
	// 8309AF90: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8309AF94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8309AF98: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8309AF9C: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8309AFA0: 4BF5F748  b 0x82ffa6e8
	sub_82FFA6E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309AFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309AFA8 size=148
    let mut pc: u32 = 0x8309AFA8;
    'dispatch: loop {
        match pc {
            0x8309AFA8 => {
    //   block [0x8309AFA8..0x8309B03C)
	// 8309AFA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309AFAC: 4810D1BD  bl 0x831a8168
	ctx.lr = 0x8309AFB0;
	sub_831A8130(ctx, base);
	// 8309AFB0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309AFB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309AFB8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8309AFBC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8309AFC0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309AFC4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309AFC8: 41980030  blt cr6, 0x8309aff8
	if ctx.cr[6].lt {
	pc = 0x8309AFF8; continue 'dispatch;
	}
	// 8309AFCC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8309AFD0: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309AFD4: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 8309AFD8: 388B6B80  addi r4, r11, 0x6b80
	ctx.r[4].s64 = ctx.r[11].s64 + 27520;
	// 8309AFDC: 38A00043  li r5, 0x43
	ctx.r[5].s64 = 67;
	// 8309AFE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8309AFE4: 4BF35975  bl 0x82fd0958
	ctx.lr = 0x8309AFE8;
	sub_82FD0958(ctx, base);
	// 8309AFE8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8309AFEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8309AFF0: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 8309AFF4: 48115C35  bl 0x831b0c28
	ctx.lr = 0x8309AFF8;
	sub_831B0C28(ctx, base);
	// 8309AFF8: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309AFFC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B000: 41820028  beq 0x8309b028
	if ctx.cr[0].eq {
	pc = 0x8309B028; continue 'dispatch;
	}
	// 8309B004: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309B008: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8309B00C: 7FAA582E  lwzx r29, r10, r11
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8309B010: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B014: 41820014  beq 0x8309b028
	if ctx.cr[0].eq {
	pc = 0x8309B028; continue 'dispatch;
	}
	// 8309B018: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309B01C: 480015C5  bl 0x8309c5e0
	ctx.lr = 0x8309B020;
	sub_8309C5E0(ctx, base);
	// 8309B020: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309B024: 4BF3D2BD  bl 0x82fd82e0
	ctx.lr = 0x8309B028;
	sub_82FD82E0(ctx, base);
	// 8309B028: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309B02C: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8309B030: 7F8A592E  stwx r28, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[28].u32) };
	// 8309B034: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8309B038: 4810D180  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309B040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309B040 size=124
    let mut pc: u32 = 0x8309B040;
    'dispatch: loop {
        match pc {
            0x8309B040 => {
    //   block [0x8309B040..0x8309B0BC)
	// 8309B040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309B044: 4810D121  bl 0x831a8164
	ctx.lr = 0x8309B048;
	sub_831A8130(ctx, base);
	// 8309B048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309B04C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309B050: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8309B054: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 8309B058: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B05C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309B060: 40990050  ble cr6, 0x8309b0b0
	if !ctx.cr[6].gt {
	pc = 0x8309B0B0; continue 'dispatch;
	}
	// 8309B064: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 8309B068: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309B06C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B070: 41820024  beq 0x8309b094
	if ctx.cr[0].eq {
	pc = 0x8309B094; continue 'dispatch;
	}
	// 8309B074: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309B078: 7F8BF02E  lwzx r28, r11, r30
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8309B07C: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B080: 41820014  beq 0x8309b094
	if ctx.cr[0].eq {
	pc = 0x8309B094; continue 'dispatch;
	}
	// 8309B084: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309B088: 48001559  bl 0x8309c5e0
	ctx.lr = 0x8309B08C;
	sub_8309C5E0(ctx, base);
	// 8309B08C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309B090: 4BF3D251  bl 0x82fd82e0
	ctx.lr = 0x8309B094;
	sub_82FD82E0(ctx, base);
	// 8309B094: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309B098: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8309B09C: 7F6BF12E  stwx r27, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[27].u32) };
	// 8309B0A0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8309B0A4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B0A8: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309B0AC: 4198FFBC  blt cr6, 0x8309b068
	if ctx.cr[6].lt {
	pc = 0x8309B068; continue 'dispatch;
	}
	// 8309B0B0: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 8309B0B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8309B0B8: 4810D0FC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309B0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309B0C0 size=260
    let mut pc: u32 = 0x8309B0C0;
    'dispatch: loop {
        match pc {
            0x8309B0C0 => {
    //   block [0x8309B0C0..0x8309B1C4)
	// 8309B0C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309B0C4: 4810D0A9  bl 0x831a816c
	ctx.lr = 0x8309B0C8;
	sub_831A8130(ctx, base);
	// 8309B0C8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309B0CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309B0D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309B0D4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B0D8: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309B0DC: 41980030  blt cr6, 0x8309b10c
	if ctx.cr[6].lt {
	pc = 0x8309B10C; continue 'dispatch;
	}
	// 8309B0E0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8309B0E4: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309B0E8: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 8309B0EC: 388B6B80  addi r4, r11, 0x6b80
	ctx.r[4].s64 = ctx.r[11].s64 + 27520;
	// 8309B0F0: 38A00090  li r5, 0x90
	ctx.r[5].s64 = 144;
	// 8309B0F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8309B0F8: 4BF35861  bl 0x82fd0958
	ctx.lr = 0x8309B0FC;
	sub_82FD0958(ctx, base);
	// 8309B0FC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8309B100: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8309B104: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 8309B108: 48115B21  bl 0x831b0c28
	ctx.lr = 0x8309B10C;
	sub_831B0C28(ctx, base);
	// 8309B10C: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309B110: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B114: 41820028  beq 0x8309b13c
	if ctx.cr[0].eq {
	pc = 0x8309B13C; continue 'dispatch;
	}
	// 8309B118: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309B11C: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8309B120: 7FAA582E  lwzx r29, r10, r11
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8309B124: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B128: 41820014  beq 0x8309b13c
	if ctx.cr[0].eq {
	pc = 0x8309B13C; continue 'dispatch;
	}
	// 8309B12C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309B130: 480014B1  bl 0x8309c5e0
	ctx.lr = 0x8309B134;
	sub_8309C5E0(ctx, base);
	// 8309B134: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309B138: 4BF3D1A9  bl 0x82fd82e0
	ctx.lr = 0x8309B13C;
	sub_82FD82E0(ctx, base);
	// 8309B13C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B140: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8309B144: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309B148: 409A0018  bne cr6, 0x8309b160
	if !ctx.cr[6].eq {
	pc = 0x8309B160; continue 'dispatch;
	}
	// 8309B14C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309B150: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8309B154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8309B158: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 8309B15C: 48000054  b 0x8309b1b0
	pc = 0x8309B1B0; continue 'dispatch;
	// 8309B160: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8309B164: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309B168: 40980030  bge cr6, 0x8309b198
	if !ctx.cr[6].lt {
	pc = 0x8309B198; continue 'dispatch;
	}
	// 8309B16C: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8309B170: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309B174: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8309B178: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8309B17C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8309B180: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309B184: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8309B188: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B18C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 8309B190: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8309B194: 4198FFDC  blt cr6, 0x8309b170
	if ctx.cr[6].lt {
	pc = 0x8309B170; continue 'dispatch;
	}
	// 8309B198: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B19C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8309B1A0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309B1A4: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8309B1A8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8309B1AC: 912BFFFC  stw r9, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[9].u32 ) };
	// 8309B1B0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B1B4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8309B1B8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8309B1BC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8309B1C0: 4810CFFC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309B1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309B1C8 size=104
    let mut pc: u32 = 0x8309B1C8;
    'dispatch: loop {
        match pc {
            0x8309B1C8 => {
    //   block [0x8309B1C8..0x8309B230)
	// 8309B1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309B1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309B1D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309B1D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309B1D8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B1DC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B1E0: 4182003C  beq 0x8309b21c
	if ctx.cr[0].eq {
	pc = 0x8309B21C; continue 'dispatch;
	}
	// 8309B1E4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8309B1E8: 89430004  lbz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309B1EC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B1F0: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8309B1F4: 41820028  beq 0x8309b21c
	if ctx.cr[0].eq {
	pc = 0x8309B21C; continue 'dispatch;
	}
	// 8309B1F8: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309B1FC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8309B200: 7FEB502E  lwzx r31, r11, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8309B204: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B208: 41820014  beq 0x8309b21c
	if ctx.cr[0].eq {
	pc = 0x8309B21C; continue 'dispatch;
	}
	// 8309B20C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309B210: 480013D1  bl 0x8309c5e0
	ctx.lr = 0x8309B214;
	sub_8309C5E0(ctx, base);
	// 8309B214: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309B218: 4BF3D0C9  bl 0x82fd82e0
	ctx.lr = 0x8309B21C;
	sub_82FD82E0(ctx, base);
	// 8309B21C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309B220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309B224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309B228: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309B22C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309B230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309B230 size=132
    let mut pc: u32 = 0x8309B230;
    'dispatch: loop {
        match pc {
            0x8309B230 => {
    //   block [0x8309B230..0x8309B2B4)
	// 8309B230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309B234: 4810CF35  bl 0x831a8168
	ctx.lr = 0x8309B238;
	sub_831A8130(ctx, base);
	// 8309B238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309B23C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309B240: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309B244: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B248: 4182004C  beq 0x8309b294
	if ctx.cr[0].eq {
	pc = 0x8309B294; continue 'dispatch;
	}
	// 8309B24C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B250: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8309B254: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309B258: 4099003C  ble cr6, 0x8309b294
	if !ctx.cr[6].gt {
	pc = 0x8309B294; continue 'dispatch;
	}
	// 8309B25C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8309B260: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309B264: 7FABF02E  lwzx r29, r11, r30
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8309B268: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B26C: 41820014  beq 0x8309b280
	if ctx.cr[0].eq {
	pc = 0x8309B280; continue 'dispatch;
	}
	// 8309B270: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309B274: 4800136D  bl 0x8309c5e0
	ctx.lr = 0x8309B278;
	sub_8309C5E0(ctx, base);
	// 8309B278: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309B27C: 4BF3D065  bl 0x82fd82e0
	ctx.lr = 0x8309B280;
	sub_82FD82E0(ctx, base);
	// 8309B280: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B284: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8309B288: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8309B28C: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309B290: 4198FFD0  blt cr6, 0x8309b260
	if ctx.cr[6].lt {
	pc = 0x8309B260; continue 'dispatch;
	}
	// 8309B294: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309B298: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309B29C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309B2A0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B2A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309B2A8: 4E800421  bctrl
	ctx.lr = 0x8309B2AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309B2AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8309B2B0: 4810CF08  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309B2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309B2B8 size=76
    let mut pc: u32 = 0x8309B2B8;
    'dispatch: loop {
        match pc {
            0x8309B2B8 => {
    //   block [0x8309B2B8..0x8309B304)
	// 8309B2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309B2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309B2C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309B2C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309B2C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309B2CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309B2D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309B2D4: 4BFFF8E5  bl 0x8309abb8
	ctx.lr = 0x8309B2D8;
	sub_8309ABB8(ctx, base);
	// 8309B2D8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309B2DC: 4182000C  beq 0x8309b2e8
	if ctx.cr[0].eq {
	pc = 0x8309B2E8; continue 'dispatch;
	}
	// 8309B2E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309B2E4: 4BF3CFFD  bl 0x82fd82e0
	ctx.lr = 0x8309B2E8;
	sub_82FD82E0(ctx, base);
	// 8309B2E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309B2EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309B2F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309B2F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309B2F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309B2FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309B300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309B308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309B308 size=8
    let mut pc: u32 = 0x8309B308;
    'dispatch: loop {
        match pc {
            0x8309B308 => {
    //   block [0x8309B308..0x8309B310)
	// 8309B308: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309B30C: 8216DF60  lwz r16, -0x20a0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-8352 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309B310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309B310 size=204
    let mut pc: u32 = 0x8309B310;
    'dispatch: loop {
        match pc {
            0x8309B310 => {
    //   block [0x8309B310..0x8309B3DC)
	// 8309B310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309B314: 4810CE45  bl 0x831a8158
	ctx.lr = 0x8309B318;
	sub_831A8130(ctx, base);
	// 8309B318: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8309B31C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309B320: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 8309B324: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8309B328: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 8309B32C: 81780048  lwz r11, 0x48(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(72 as u32) ) } as u64;
	// 8309B330: 937F00B4  stw r27, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[27].u32 ) };
	// 8309B334: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B338: 4182000C  beq 0x8309b344
	if ctx.cr[0].eq {
	pc = 0x8309B344; continue 'dispatch;
	}
	// 8309B33C: 834B0008  lwz r26, 8(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B340: 48000008  b 0x8309b348
	pc = 0x8309B348; continue 'dispatch;
	// 8309B344: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8309B348: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8309B34C: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8309B350: 419A0084  beq cr6, 0x8309b3d4
	if ctx.cr[6].eq {
	pc = 0x8309B3D4; continue 'dispatch;
	}
	// 8309B354: 80780048  lwz r3, 0x48(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(72 as u32) ) } as u64;
	// 8309B358: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B35C: 41820014  beq 0x8309b370
	if ctx.cr[0].eq {
	pc = 0x8309B370; continue 'dispatch;
	}
	// 8309B360: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8309B364: 4BF9150D  bl 0x8302c870
	ctx.lr = 0x8309B368;
	sub_8302C870(ctx, base);
	// 8309B368: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8309B36C: 48000008  b 0x8309b374
	pc = 0x8309B374; continue 'dispatch;
	// 8309B370: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8309B374: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 8309B378: 809B0014  lwz r4, 0x14(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309B37C: 4BF3CF1D  bl 0x82fd8298
	ctx.lr = 0x8309B380;
	sub_82FD8298(ctx, base);
	// 8309B380: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8309B384: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B388: 4182001C  beq 0x8309b3a4
	if ctx.cr[0].eq {
	pc = 0x8309B3A4; continue 'dispatch;
	}
	// 8309B38C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8309B390: 80DB0014  lwz r6, 0x14(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309B394: 80BB0010  lwz r5, 0x10(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309B398: 480011D1  bl 0x8309c568
	ctx.lr = 0x8309B39C;
	sub_8309C568(ctx, base);
	// 8309B39C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309B3A0: 48000008  b 0x8309b3a8
	pc = 0x8309B3A8; continue 'dispatch;
	// 8309B3A4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8309B3A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8309B3AC: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309B3B0: 4BF9FDA1  bl 0x8303b150
	ctx.lr = 0x8309B3B4;
	sub_8303B150(ctx, base);
	// 8309B3B4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8309B3B8: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 8309B3BC: 807B0008  lwz r3, 8(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B3C0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8309B3C4: 4BFFFA45  bl 0x8309ae08
	ctx.lr = 0x8309B3C8;
	sub_8309AE08(ctx, base);
	// 8309B3C8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8309B3CC: 7F1DD040  cmplw cr6, r29, r26
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8309B3D0: 4198FF84  blt cr6, 0x8309b354
	if ctx.cr[6].lt {
	pc = 0x8309B354; continue 'dispatch;
	}
	// 8309B3D4: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8309B3D8: 4810CDD0  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309B3DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309B3DC size=48
    let mut pc: u32 = 0x8309B3DC;
    'dispatch: loop {
        match pc {
            0x8309B3DC => {
    //   block [0x8309B3DC..0x8309B40C)
	// 8309B3DC: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8309B3E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309B3E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309B3E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309B3EC: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8309B3F0: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309B3F4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309B3F8: 4BF3CEE9  bl 0x82fd82e0
	ctx.lr = 0x8309B3FC;
	sub_82FD82E0(ctx, base);
	// 8309B3FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309B400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309B404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309B408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309B410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309B410 size=8
    let mut pc: u32 = 0x8309B410;
    'dispatch: loop {
        match pc {
            0x8309B410 => {
    //   block [0x8309B410..0x8309B418)
	// 8309B410: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309B414: 8216DFA8  lwz r16, -0x2058(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-8280 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309B418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309B418 size=248
    let mut pc: u32 = 0x8309B418;
    'dispatch: loop {
        match pc {
            0x8309B418 => {
    //   block [0x8309B418..0x8309B510)
	// 8309B418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309B41C: 4810CD49  bl 0x831a8164
	ctx.lr = 0x8309B420;
	sub_831A8130(ctx, base);
	// 8309B420: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8309B424: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309B428: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8309B42C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309B430: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8309B434: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309B438: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309B43C: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 8309B440: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309B444: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309B448: 4E800421  bctrl
	ctx.lr = 0x8309B44C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309B44C: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 8309B450: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8309B454: 419A00B4  beq cr6, 0x8309b508
	if ctx.cr[6].eq {
	pc = 0x8309B508; continue 'dispatch;
	}
	// 8309B458: 38DF0050  addi r6, r31, 0x50
	ctx.r[6].s64 = ctx.r[31].s64 + 80;
	// 8309B45C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B460: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8309B464: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8309B468: 4BFCD799  bl 0x83068c00
	ctx.lr = 0x8309B46C;
	sub_83068C00(ctx, base);
	// 8309B46C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B470: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8309B474: 41820008  beq 0x8309b47c
	if ctx.cr[0].eq {
	pc = 0x8309B47C; continue 'dispatch;
	}
	// 8309B478: 83630000  lwz r27, 0(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309B47C: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309B480: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309B484: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8309B488: 4BF5EFD9  bl 0x82ffa460
	ctx.lr = 0x8309B48C;
	sub_82FFA460(ctx, base);
	// 8309B48C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B490: 4182001C  beq 0x8309b4ac
	if ctx.cr[0].eq {
	pc = 0x8309B4AC; continue 'dispatch;
	}
	// 8309B494: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309B498: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B49C: 41820010  beq 0x8309b4ac
	if ctx.cr[0].eq {
	pc = 0x8309B4AC; continue 'dispatch;
	}
	// 8309B4A0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8309B4A4: 48001FB5  bl 0x8309d458
	ctx.lr = 0x8309B4A8;
	sub_8309D458(ctx, base);
	// 8309B4A8: 48000060  b 0x8309b508
	pc = 0x8309B508; continue 'dispatch;
	// 8309B4AC: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 8309B4B0: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309B4B4: 4BF3CDE5  bl 0x82fd8298
	ctx.lr = 0x8309B4B8;
	sub_82FD8298(ctx, base);
	// 8309B4B8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8309B4BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B4C0: 4182001C  beq 0x8309b4dc
	if ctx.cr[0].eq {
	pc = 0x8309B4DC; continue 'dispatch;
	}
	// 8309B4C4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8309B4C8: 80DE0014  lwz r6, 0x14(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309B4CC: 80BE0010  lwz r5, 0x10(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309B4D0: 48001099  bl 0x8309c568
	ctx.lr = 0x8309B4D4;
	sub_8309C568(ctx, base);
	// 8309B4D4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8309B4D8: 48000008  b 0x8309b4e0
	pc = 0x8309B4E0; continue 'dispatch;
	// 8309B4DC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8309B4E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8309B4E4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309B4E8: 4BF9FC69  bl 0x8303b150
	ctx.lr = 0x8309B4EC;
	sub_8303B150(ctx, base);
	// 8309B4EC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8309B4F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309B4F4: 48001F65  bl 0x8309d458
	ctx.lr = 0x8309B4F8;
	sub_8309D458(ctx, base);
	// 8309B4F8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8309B4FC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8309B500: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309B504: 4BFFF825  bl 0x8309ad28
	ctx.lr = 0x8309B508;
	sub_8309AD28(ctx, base);
	// 8309B508: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8309B50C: 4810CCA8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309B510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309B510 size=48
    let mut pc: u32 = 0x8309B510;
    'dispatch: loop {
        match pc {
            0x8309B510 => {
    //   block [0x8309B510..0x8309B540)
	// 8309B510: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8309B514: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309B518: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309B51C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309B520: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8309B524: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309B528: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309B52C: 4BF3CDB5  bl 0x82fd82e0
	ctx.lr = 0x8309B530;
	sub_82FD82E0(ctx, base);
	// 8309B530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309B534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309B538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309B53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309B540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309B540 size=100
    let mut pc: u32 = 0x8309B540;
    'dispatch: loop {
        match pc {
            0x8309B540 => {
    //   block [0x8309B540..0x8309B5A4)
	// 8309B540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309B544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309B548: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309B54C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309B550: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309B554: 4BFFF735  bl 0x8309ac88
	ctx.lr = 0x8309B558;
	sub_8309AC88(ctx, base);
	// 8309B558: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309B55C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B560: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309B564: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B568: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309B56C: 4E800421  bctrl
	ctx.lr = 0x8309B570;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309B570: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8309B574: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B578: 41820018  beq 0x8309b590
	if ctx.cr[0].eq {
	pc = 0x8309B590; continue 'dispatch;
	}
	// 8309B57C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309B580: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8309B584: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B588: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309B58C: 4E800421  bctrl
	ctx.lr = 0x8309B590;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309B590: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309B594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309B598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309B59C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309B5A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309B5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309B5A8 size=156
    let mut pc: u32 = 0x8309B5A8;
    'dispatch: loop {
        match pc {
            0x8309B5A8 => {
    //   block [0x8309B5A8..0x8309B644)
	// 8309B5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309B5AC: 4810CBB5  bl 0x831a8160
	ctx.lr = 0x8309B5B0;
	sub_831A8130(ctx, base);
	// 8309B5B0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309B5B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309B5B8: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8309B5BC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309B5C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309B5C4: 40990078  ble cr6, 0x8309b63c
	if !ctx.cr[6].gt {
	pc = 0x8309B63C; continue 'dispatch;
	}
	// 8309B5C8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8309B5CC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B5D0: 7FEBE82E  lwzx r31, r11, r29
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8309B5D4: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B5D8: 41820044  beq 0x8309b61c
	if ctx.cr[0].eq {
	pc = 0x8309B61C; continue 'dispatch;
	}
	// 8309B5DC: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309B5E0: 837F0004  lwz r27, 4(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309B5E4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B5E8: 41820020  beq 0x8309b608
	if ctx.cr[0].eq {
	pc = 0x8309B608; continue 'dispatch;
	}
	// 8309B5EC: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309B5F0: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B5F4: 41820014  beq 0x8309b608
	if ctx.cr[0].eq {
	pc = 0x8309B608; continue 'dispatch;
	}
	// 8309B5F8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309B5FC: 48000FE5  bl 0x8309c5e0
	ctx.lr = 0x8309B600;
	sub_8309C5E0(ctx, base);
	// 8309B600: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309B604: 4BF3CCDD  bl 0x82fd82e0
	ctx.lr = 0x8309B608;
	sub_82FD82E0(ctx, base);
	// 8309B608: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309B60C: 4BF3CCD5  bl 0x82fd82e0
	ctx.lr = 0x8309B610;
	sub_82FD82E0(ctx, base);
	// 8309B610: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 8309B614: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8309B618: 409AFFC4  bne cr6, 0x8309b5dc
	if !ctx.cr[6].eq {
	pc = 0x8309B5DC; continue 'dispatch;
	}
	// 8309B61C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8309B624: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 8309B628: 7D4BE92E  stwx r10, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[10].u32) };
	// 8309B62C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8309B630: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309B634: 7F1A5840  cmplw cr6, r26, r11
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309B638: 4198FF94  blt cr6, 0x8309b5cc
	if ctx.cr[6].lt {
	pc = 0x8309B5CC; continue 'dispatch;
	}
	// 8309B63C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8309B640: 4810CB70  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309B648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309B648 size=96
    let mut pc: u32 = 0x8309B648;
    'dispatch: loop {
        match pc {
            0x8309B648 => {
    //   block [0x8309B648..0x8309B6A8)
	// 8309B648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309B64C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309B650: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309B654: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309B658: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309B65C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B660: 4BFFFF49  bl 0x8309b5a8
	ctx.lr = 0x8309B664;
	sub_8309B5A8(ctx, base);
	// 8309B664: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309B668: 4BFFF621  bl 0x8309ac88
	ctx.lr = 0x8309B66C;
	sub_8309AC88(ctx, base);
	// 8309B66C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309B670: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309B674: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B678: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309B67C: 4E800421  bctrl
	ctx.lr = 0x8309B680;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309B680: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309B684: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309B688: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B68C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309B690: 4E800421  bctrl
	ctx.lr = 0x8309B694;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309B694: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309B698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309B69C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309B6A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309B6A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309B6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309B6A8 size=8
    let mut pc: u32 = 0x8309B6A8;
    'dispatch: loop {
        match pc {
            0x8309B6A8 => {
    //   block [0x8309B6A8..0x8309B6B0)
	// 8309B6A8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309B6AC: 8216DFF8  lwz r16, -0x2008(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-8200 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309B6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309B6B0 size=140
    let mut pc: u32 = 0x8309B6B0;
    'dispatch: loop {
        match pc {
            0x8309B6B0 => {
    //   block [0x8309B6B0..0x8309B73C)
	// 8309B6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309B6B4: 4810CAB9  bl 0x831a816c
	ctx.lr = 0x8309B6B8;
	sub_831A8130(ctx, base);
	// 8309B6B8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8309B6BC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309B6C0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309B6C4: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309B6C8: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309B6CC: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8309B6D0: 4BF9FA81  bl 0x8303b150
	ctx.lr = 0x8309B6D4;
	sub_8303B150(ctx, base);
	// 8309B6D4: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8309B6D8: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309B6DC: 4BF3CBBD  bl 0x82fd8298
	ctx.lr = 0x8309B6E0;
	sub_82FD8298(ctx, base);
	// 8309B6E0: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8309B6E4: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8309B6E8: 41820044  beq 0x8309b72c
	if ctx.cr[0].eq {
	pc = 0x8309B72C; continue 'dispatch;
	}
	// 8309B6EC: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8309B6F0: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309B6F4: 4BF3CBA5  bl 0x82fd8298
	ctx.lr = 0x8309B6F8;
	sub_82FD8298(ctx, base);
	// 8309B6F8: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8309B6FC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B700: 41820010  beq 0x8309b710
	if ctx.cr[0].eq {
	pc = 0x8309B710; continue 'dispatch;
	}
	// 8309B704: 4BF63E8D  bl 0x82fff590
	ctx.lr = 0x8309B708;
	sub_82FFF590(ctx, base);
	// 8309B708: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8309B70C: 48000008  b 0x8309b714
	pc = 0x8309B714; continue 'dispatch;
	// 8309B710: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8309B714: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8309B718: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 8309B71C: 80FE0014  lwz r7, 0x14(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309B720: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309B724: 4BF9F9CD  bl 0x8303b0f0
	ctx.lr = 0x8309B728;
	sub_8303B0F0(ctx, base);
	// 8309B728: 48000008  b 0x8309b730
	pc = 0x8309B730; continue 'dispatch;
	// 8309B72C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309B730: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8309B734: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8309B738: 4810CA84  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309B73C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309B73C size=48
    let mut pc: u32 = 0x8309B73C;
    'dispatch: loop {
        match pc {
            0x8309B73C => {
    //   block [0x8309B73C..0x8309B76C)
	// 8309B73C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8309B740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309B744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309B748: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309B74C: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8309B750: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309B754: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309B758: 4BF3CB89  bl 0x82fd82e0
	ctx.lr = 0x8309B75C;
	sub_82FD82E0(ctx, base);
	// 8309B75C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309B760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309B764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309B768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309B76C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309B76C size=48
    let mut pc: u32 = 0x8309B76C;
    'dispatch: loop {
        match pc {
            0x8309B76C => {
    //   block [0x8309B76C..0x8309B79C)
	// 8309B76C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8309B770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309B774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309B778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309B77C: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8309B780: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309B784: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8309B788: 4BF3CB59  bl 0x82fd82e0
	ctx.lr = 0x8309B78C;
	sub_82FD82E0(ctx, base);
	// 8309B78C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309B790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309B794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309B798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309B7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309B7A0 size=100
    let mut pc: u32 = 0x8309B7A0;
    'dispatch: loop {
        match pc {
            0x8309B7A0 => {
    //   block [0x8309B7A0..0x8309B804)
	// 8309B7A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309B7A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309B7A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309B7AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309B7B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309B7B4: 4BFFFDF5  bl 0x8309b5a8
	ctx.lr = 0x8309B7B8;
	sub_8309B5A8(ctx, base);
	// 8309B7B8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309B7BC: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B7C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309B7C4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B7C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309B7CC: 4E800421  bctrl
	ctx.lr = 0x8309B7D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309B7D0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309B7D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B7D8: 41820018  beq 0x8309b7f0
	if ctx.cr[0].eq {
	pc = 0x8309B7F0; continue 'dispatch;
	}
	// 8309B7DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309B7E0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8309B7E4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B7E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309B7EC: 4E800421  bctrl
	ctx.lr = 0x8309B7F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309B7F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309B7F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309B7F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309B7FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309B800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309B808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309B808 size=8
    let mut pc: u32 = 0x8309B808;
    'dispatch: loop {
        match pc {
            0x8309B808 => {
    //   block [0x8309B808..0x8309B810)
	// 8309B808: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309B80C: 8216E040  lwz r16, -0x1fc0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-8128 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309B810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309B810 size=96
    let mut pc: u32 = 0x8309B810;
    'dispatch: loop {
        match pc {
            0x8309B810 => {
    //   block [0x8309B810..0x8309B870)
	// 8309B810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309B814: 4810C959  bl 0x831a816c
	ctx.lr = 0x8309B818;
	sub_831A8130(ctx, base);
	// 8309B818: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8309B81C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309B820: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309B824: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309B828: 396BDEF8  addi r11, r11, -0x2108
	ctx.r[11].s64 = ctx.r[11].s64 + -8456;
	// 8309B82C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8309B830: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309B834: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309B838: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B83C: 41820020  beq 0x8309b85c
	if ctx.cr[0].eq {
	pc = 0x8309B85C; continue 'dispatch;
	}
	// 8309B840: 83BE0010  lwz r29, 0x10(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309B844: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B848: 41820014  beq 0x8309b85c
	if ctx.cr[0].eq {
	pc = 0x8309B85C; continue 'dispatch;
	}
	// 8309B84C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309B850: 4BFFFCF1  bl 0x8309b540
	ctx.lr = 0x8309B854;
	sub_8309B540(ctx, base);
	// 8309B854: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309B858: 4BF3CA89  bl 0x82fd82e0
	ctx.lr = 0x8309B85C;
	sub_82FD82E0(ctx, base);
	// 8309B85C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309B860: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 8309B864: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309B868: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8309B86C: 4810C950  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309B870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309B870 size=40
    let mut pc: u32 = 0x8309B870;
    'dispatch: loop {
        match pc {
            0x8309B870 => {
    //   block [0x8309B870..0x8309B898)
	// 8309B870: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8309B874: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309B878: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309B87C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309B880: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8309B884: 4BFB0EDD  bl 0x8304c760
	ctx.lr = 0x8309B888;
	sub_8304C760(ctx, base);
	// 8309B888: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309B88C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309B890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309B894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309B898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309B898 size=8
    let mut pc: u32 = 0x8309B898;
    'dispatch: loop {
        match pc {
            0x8309B898 => {
    //   block [0x8309B898..0x8309B8A0)
	// 8309B898: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309B89C: 8216E078  lwz r16, -0x1f88(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-8072 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309B8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309B8A0 size=164
    let mut pc: u32 = 0x8309B8A0;
    'dispatch: loop {
        match pc {
            0x8309B8A0 => {
    //   block [0x8309B8A0..0x8309B944)
	// 8309B8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309B8A4: 4810C8C1  bl 0x831a8164
	ctx.lr = 0x8309B8A8;
	sub_831A8130(ctx, base);
	// 8309B8A8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8309B8AC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309B8B0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309B8B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309B8B8: 396BDF40  addi r11, r11, -0x20c0
	ctx.r[11].s64 = ctx.r[11].s64 + -8384;
	// 8309B8BC: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8309B8C0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309B8C4: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309B8C8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B8CC: 4182004C  beq 0x8309b918
	if ctx.cr[0].eq {
	pc = 0x8309B918; continue 'dispatch;
	}
	// 8309B8D0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B8D4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8309B8D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309B8DC: 4099003C  ble cr6, 0x8309b918
	if !ctx.cr[6].gt {
	pc = 0x8309B918; continue 'dispatch;
	}
	// 8309B8E0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8309B8E4: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309B8E8: 7F8BE82E  lwzx r28, r11, r29
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8309B8EC: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B8F0: 41820014  beq 0x8309b904
	if ctx.cr[0].eq {
	pc = 0x8309B904; continue 'dispatch;
	}
	// 8309B8F4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309B8F8: 4BFFFC49  bl 0x8309b540
	ctx.lr = 0x8309B8FC;
	sub_8309B540(ctx, base);
	// 8309B8FC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309B900: 4BF3C9E1  bl 0x82fd82e0
	ctx.lr = 0x8309B904;
	sub_82FD82E0(ctx, base);
	// 8309B904: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B908: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 8309B90C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8309B910: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309B914: 4198FFD0  blt cr6, 0x8309b8e4
	if ctx.cr[6].lt {
	pc = 0x8309B8E4; continue 'dispatch;
	}
	// 8309B918: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309B91C: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309B920: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309B924: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B928: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309B92C: 4E800421  bctrl
	ctx.lr = 0x8309B930;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309B930: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309B934: 396BDE90  addi r11, r11, -0x2170
	ctx.r[11].s64 = ctx.r[11].s64 + -8560;
	// 8309B938: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309B93C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8309B940: 4810C874  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309B944(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309B944 size=40
    let mut pc: u32 = 0x8309B944;
    'dispatch: loop {
        match pc {
            0x8309B944 => {
    //   block [0x8309B944..0x8309B96C)
	// 8309B944: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8309B948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309B94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309B950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309B954: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8309B958: 4BFFF071  bl 0x8309a9c8
	ctx.lr = 0x8309B95C;
	sub_8309A9C8(ctx, base);
	// 8309B95C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309B960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309B964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309B968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309B970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309B970 size=148
    let mut pc: u32 = 0x8309B970;
    'dispatch: loop {
        match pc {
            0x8309B970 => {
    //   block [0x8309B970..0x8309BA04)
	// 8309B970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309B974: 4810C7F5  bl 0x831a8168
	ctx.lr = 0x8309B978;
	sub_831A8130(ctx, base);
	// 8309B978: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309B97C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309B980: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8309B984: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8309B988: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309B98C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309B990: 41980030  blt cr6, 0x8309b9c0
	if ctx.cr[6].lt {
	pc = 0x8309B9C0; continue 'dispatch;
	}
	// 8309B994: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8309B998: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309B99C: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 8309B9A0: 388B6B80  addi r4, r11, 0x6b80
	ctx.r[4].s64 = ctx.r[11].s64 + 27520;
	// 8309B9A4: 38A00043  li r5, 0x43
	ctx.r[5].s64 = 67;
	// 8309B9A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8309B9AC: 4BF34FAD  bl 0x82fd0958
	ctx.lr = 0x8309B9B0;
	sub_82FD0958(ctx, base);
	// 8309B9B0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8309B9B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8309B9B8: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 8309B9BC: 4811526D  bl 0x831b0c28
	ctx.lr = 0x8309B9C0;
	sub_831B0C28(ctx, base);
	// 8309B9C0: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309B9C4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B9C8: 41820028  beq 0x8309b9f0
	if ctx.cr[0].eq {
	pc = 0x8309B9F0; continue 'dispatch;
	}
	// 8309B9CC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309B9D0: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8309B9D4: 7FAA582E  lwzx r29, r10, r11
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8309B9D8: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309B9DC: 41820014  beq 0x8309b9f0
	if ctx.cr[0].eq {
	pc = 0x8309B9F0; continue 'dispatch;
	}
	// 8309B9E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309B9E4: 4BFFFB5D  bl 0x8309b540
	ctx.lr = 0x8309B9E8;
	sub_8309B540(ctx, base);
	// 8309B9E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309B9EC: 4BF3C8F5  bl 0x82fd82e0
	ctx.lr = 0x8309B9F0;
	sub_82FD82E0(ctx, base);
	// 8309B9F0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309B9F4: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8309B9F8: 7F8A592E  stwx r28, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[28].u32) };
	// 8309B9FC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8309BA00: 4810C7B8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309BA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309BA08 size=124
    let mut pc: u32 = 0x8309BA08;
    'dispatch: loop {
        match pc {
            0x8309BA08 => {
    //   block [0x8309BA08..0x8309BA84)
	// 8309BA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309BA0C: 4810C759  bl 0x831a8164
	ctx.lr = 0x8309BA10;
	sub_831A8130(ctx, base);
	// 8309BA10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309BA14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309BA18: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8309BA1C: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 8309BA20: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309BA24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309BA28: 40990050  ble cr6, 0x8309ba78
	if !ctx.cr[6].gt {
	pc = 0x8309BA78; continue 'dispatch;
	}
	// 8309BA2C: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 8309BA30: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309BA34: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309BA38: 41820024  beq 0x8309ba5c
	if ctx.cr[0].eq {
	pc = 0x8309BA5C; continue 'dispatch;
	}
	// 8309BA3C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309BA40: 7F8BF02E  lwzx r28, r11, r30
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8309BA44: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309BA48: 41820014  beq 0x8309ba5c
	if ctx.cr[0].eq {
	pc = 0x8309BA5C; continue 'dispatch;
	}
	// 8309BA4C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309BA50: 4BFFFAF1  bl 0x8309b540
	ctx.lr = 0x8309BA54;
	sub_8309B540(ctx, base);
	// 8309BA54: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309BA58: 4BF3C889  bl 0x82fd82e0
	ctx.lr = 0x8309BA5C;
	sub_82FD82E0(ctx, base);
	// 8309BA5C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309BA60: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8309BA64: 7F6BF12E  stwx r27, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[27].u32) };
	// 8309BA68: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8309BA6C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309BA70: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309BA74: 4198FFBC  blt cr6, 0x8309ba30
	if ctx.cr[6].lt {
	pc = 0x8309BA30; continue 'dispatch;
	}
	// 8309BA78: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 8309BA7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8309BA80: 4810C734  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309BA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309BA88 size=260
    let mut pc: u32 = 0x8309BA88;
    'dispatch: loop {
        match pc {
            0x8309BA88 => {
    //   block [0x8309BA88..0x8309BB8C)
	// 8309BA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309BA8C: 4810C6E1  bl 0x831a816c
	ctx.lr = 0x8309BA90;
	sub_831A8130(ctx, base);
	// 8309BA90: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309BA94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309BA98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309BA9C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309BAA0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309BAA4: 41980030  blt cr6, 0x8309bad4
	if ctx.cr[6].lt {
	pc = 0x8309BAD4; continue 'dispatch;
	}
	// 8309BAA8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8309BAAC: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309BAB0: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 8309BAB4: 388B6B80  addi r4, r11, 0x6b80
	ctx.r[4].s64 = ctx.r[11].s64 + 27520;
	// 8309BAB8: 38A00090  li r5, 0x90
	ctx.r[5].s64 = 144;
	// 8309BABC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8309BAC0: 4BF34E99  bl 0x82fd0958
	ctx.lr = 0x8309BAC4;
	sub_82FD0958(ctx, base);
	// 8309BAC4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8309BAC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8309BACC: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 8309BAD0: 48115159  bl 0x831b0c28
	ctx.lr = 0x8309BAD4;
	sub_831B0C28(ctx, base);
	// 8309BAD4: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309BAD8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309BADC: 41820028  beq 0x8309bb04
	if ctx.cr[0].eq {
	pc = 0x8309BB04; continue 'dispatch;
	}
	// 8309BAE0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309BAE4: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8309BAE8: 7FAA582E  lwzx r29, r10, r11
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8309BAEC: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309BAF0: 41820014  beq 0x8309bb04
	if ctx.cr[0].eq {
	pc = 0x8309BB04; continue 'dispatch;
	}
	// 8309BAF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309BAF8: 4BFFFA49  bl 0x8309b540
	ctx.lr = 0x8309BAFC;
	sub_8309B540(ctx, base);
	// 8309BAFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309BB00: 4BF3C7E1  bl 0x82fd82e0
	ctx.lr = 0x8309BB04;
	sub_82FD82E0(ctx, base);
	// 8309BB04: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309BB08: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8309BB0C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309BB10: 409A0018  bne cr6, 0x8309bb28
	if !ctx.cr[6].eq {
	pc = 0x8309BB28; continue 'dispatch;
	}
	// 8309BB14: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309BB18: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8309BB1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8309BB20: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 8309BB24: 48000054  b 0x8309bb78
	pc = 0x8309BB78; continue 'dispatch;
	// 8309BB28: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8309BB2C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309BB30: 40980030  bge cr6, 0x8309bb60
	if !ctx.cr[6].lt {
	pc = 0x8309BB60; continue 'dispatch;
	}
	// 8309BB34: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8309BB38: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309BB3C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8309BB40: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8309BB44: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8309BB48: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309BB4C: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8309BB50: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309BB54: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 8309BB58: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8309BB5C: 4198FFDC  blt cr6, 0x8309bb38
	if ctx.cr[6].lt {
	pc = 0x8309BB38; continue 'dispatch;
	}
	// 8309BB60: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309BB64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8309BB68: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309BB6C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8309BB70: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8309BB74: 912BFFFC  stw r9, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[9].u32 ) };
	// 8309BB78: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309BB7C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8309BB80: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8309BB84: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8309BB88: 4810C634  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309BB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309BB90 size=104
    let mut pc: u32 = 0x8309BB90;
    'dispatch: loop {
        match pc {
            0x8309BB90 => {
    //   block [0x8309BB90..0x8309BBF8)
	// 8309BB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309BB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309BB98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309BB9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309BBA0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309BBA4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309BBA8: 4182003C  beq 0x8309bbe4
	if ctx.cr[0].eq {
	pc = 0x8309BBE4; continue 'dispatch;
	}
	// 8309BBAC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8309BBB0: 89430004  lbz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309BBB4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309BBB8: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8309BBBC: 41820028  beq 0x8309bbe4
	if ctx.cr[0].eq {
	pc = 0x8309BBE4; continue 'dispatch;
	}
	// 8309BBC0: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309BBC4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8309BBC8: 7FEB502E  lwzx r31, r11, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8309BBCC: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309BBD0: 41820014  beq 0x8309bbe4
	if ctx.cr[0].eq {
	pc = 0x8309BBE4; continue 'dispatch;
	}
	// 8309BBD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309BBD8: 4BFFF969  bl 0x8309b540
	ctx.lr = 0x8309BBDC;
	sub_8309B540(ctx, base);
	// 8309BBDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309BBE0: 4BF3C701  bl 0x82fd82e0
	ctx.lr = 0x8309BBE4;
	sub_82FD82E0(ctx, base);
	// 8309BBE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309BBE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309BBEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309BBF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309BBF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309BBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309BBF8 size=132
    let mut pc: u32 = 0x8309BBF8;
    'dispatch: loop {
        match pc {
            0x8309BBF8 => {
    //   block [0x8309BBF8..0x8309BC7C)
	// 8309BBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309BBFC: 4810C56D  bl 0x831a8168
	ctx.lr = 0x8309BC00;
	sub_831A8130(ctx, base);
	// 8309BC00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309BC04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309BC08: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309BC0C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309BC10: 4182004C  beq 0x8309bc5c
	if ctx.cr[0].eq {
	pc = 0x8309BC5C; continue 'dispatch;
	}
	// 8309BC14: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309BC18: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8309BC1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309BC20: 4099003C  ble cr6, 0x8309bc5c
	if !ctx.cr[6].gt {
	pc = 0x8309BC5C; continue 'dispatch;
	}
	// 8309BC24: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8309BC28: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309BC2C: 7FABF02E  lwzx r29, r11, r30
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8309BC30: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309BC34: 41820014  beq 0x8309bc48
	if ctx.cr[0].eq {
	pc = 0x8309BC48; continue 'dispatch;
	}
	// 8309BC38: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309BC3C: 4BFFF905  bl 0x8309b540
	ctx.lr = 0x8309BC40;
	sub_8309B540(ctx, base);
	// 8309BC40: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309BC44: 4BF3C69D  bl 0x82fd82e0
	ctx.lr = 0x8309BC48;
	sub_82FD82E0(ctx, base);
	// 8309BC48: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309BC4C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8309BC50: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8309BC54: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309BC58: 4198FFD0  blt cr6, 0x8309bc28
	if ctx.cr[6].lt {
	pc = 0x8309BC28; continue 'dispatch;
	}
	// 8309BC5C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309BC60: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309BC64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309BC68: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309BC6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309BC70: 4E800421  bctrl
	ctx.lr = 0x8309BC74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309BC74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8309BC78: 4810C540  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309BC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309BC80 size=76
    let mut pc: u32 = 0x8309BC80;
    'dispatch: loop {
        match pc {
            0x8309BC80 => {
    //   block [0x8309BC80..0x8309BCCC)
	// 8309BC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309BC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309BC88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309BC8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309BC90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309BC94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309BC98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309BC9C: 4BFFFB75  bl 0x8309b810
	ctx.lr = 0x8309BCA0;
	sub_8309B810(ctx, base);
	// 8309BCA0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309BCA4: 4182000C  beq 0x8309bcb0
	if ctx.cr[0].eq {
	pc = 0x8309BCB0; continue 'dispatch;
	}
	// 8309BCA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309BCAC: 4BF3C635  bl 0x82fd82e0
	ctx.lr = 0x8309BCB0;
	sub_82FD82E0(ctx, base);
	// 8309BCB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309BCB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309BCB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309BCBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309BCC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309BCC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309BCC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309BCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309BCD0 size=76
    let mut pc: u32 = 0x8309BCD0;
    'dispatch: loop {
        match pc {
            0x8309BCD0 => {
    //   block [0x8309BCD0..0x8309BD1C)
	// 8309BCD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309BCD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309BCD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309BCDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309BCE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309BCE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309BCE8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309BCEC: 4BFFFBB5  bl 0x8309b8a0
	ctx.lr = 0x8309BCF0;
	sub_8309B8A0(ctx, base);
	// 8309BCF0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309BCF4: 4182000C  beq 0x8309bd00
	if ctx.cr[0].eq {
	pc = 0x8309BD00; continue 'dispatch;
	}
	// 8309BCF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309BCFC: 4BF3C5E5  bl 0x82fd82e0
	ctx.lr = 0x8309BD00;
	sub_82FD82E0(ctx, base);
	// 8309BD00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309BD04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309BD08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309BD0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309BD10: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309BD14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309BD18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309BD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309BD20 size=8
    let mut pc: u32 = 0x8309BD20;
    'dispatch: loop {
        match pc {
            0x8309BD20 => {
    //   block [0x8309BD20..0x8309BD28)
	// 8309BD20: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309BD24: 8216E0B0  lwz r16, -0x1f50(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-8016 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309BD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309BD28 size=240
    let mut pc: u32 = 0x8309BD28;
    'dispatch: loop {
        match pc {
            0x8309BD28 => {
    //   block [0x8309BD28..0x8309BE18)
	// 8309BD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309BD2C: 4810C439  bl 0x831a8164
	ctx.lr = 0x8309BD30;
	sub_831A8130(ctx, base);
	// 8309BD30: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 8309BD34: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309BD38: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8309BD3C: 807C000C  lwz r3, 0xc(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309BD40: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309BD44: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8309BD48: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309BD4C: 408200C4  bne 0x8309be10
	if !ctx.cr[0].eq {
	pc = 0x8309BE10; continue 'dispatch;
	}
	// 8309BD50: 4BF51E99  bl 0x82fedbe8
	ctx.lr = 0x8309BD54;
	sub_82FEDBE8(ctx, base);
	// 8309BD54: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8309BD58: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8309BD5C: 80DC0014  lwz r6, 0x14(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309BD60: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8309BD64: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8309BD68: 4BFFF161  bl 0x8309aec8
	ctx.lr = 0x8309BD6C;
	sub_8309AEC8(ctx, base);
	// 8309BD6C: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8309BD70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309BD74: 409A001C  bne cr6, 0x8309bd90
	if !ctx.cr[6].eq {
	pc = 0x8309BD90; continue 'dispatch;
	}
	// 8309BD78: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 8309BD7C: 815F006C  lwz r10, 0x6c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8309BD80: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309BD84: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309BD88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309BD8C: 419A0008  beq cr6, 0x8309bd94
	if ctx.cr[6].eq {
	pc = 0x8309BD94; continue 'dispatch;
	}
	// 8309BD90: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8309BD94: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309BD98: 41820058  beq 0x8309bdf0
	if ctx.cr[0].eq {
	pc = 0x8309BDF0; continue 'dispatch;
	}
	// 8309BD9C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8309BDA0: 4BF71BD9  bl 0x8300d978
	ctx.lr = 0x8309BDA4;
	sub_8300D978(ctx, base);
	// 8309BDA4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309BDA8: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309BDAC: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309BDB0: 83BE0008  lwz r29, 8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309BDB4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8309BDB8: 4BF5E6A9  bl 0x82ffa460
	ctx.lr = 0x8309BDBC;
	sub_82FFA460(ctx, base);
	// 8309BDBC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309BDC0: 41820010  beq 0x8309bdd0
	if ctx.cr[0].eq {
	pc = 0x8309BDD0; continue 'dispatch;
	}
	// 8309BDC4: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309BDC8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309BDCC: 40820018  bne 0x8309bde4
	if !ctx.cr[0].eq {
	pc = 0x8309BDE4; continue 'dispatch;
	}
	// 8309BDD0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8309BDD4: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309BDD8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8309BDDC: 4BFFEF4D  bl 0x8309ad28
	ctx.lr = 0x8309BDE0;
	sub_8309AD28(ctx, base);
	// 8309BDE0: 4BFFFF8C  b 0x8309bd6c
	pc = 0x8309BD6C; continue 'dispatch;
	// 8309BDE4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8309BDE8: 48001671  bl 0x8309d458
	ctx.lr = 0x8309BDEC;
	sub_8309D458(ctx, base);
	// 8309BDEC: 4BFFFF80  b 0x8309bd6c
	pc = 0x8309BD6C; continue 'dispatch;
	// 8309BDF0: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8309BDF4: 419A0014  beq cr6, 0x8309be08
	if ctx.cr[6].eq {
	pc = 0x8309BE08; continue 'dispatch;
	}
	// 8309BDF8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8309BDFC: 4BFFF745  bl 0x8309b540
	ctx.lr = 0x8309BE00;
	sub_8309B540(ctx, base);
	// 8309BE00: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8309BE04: 4BF3C4DD  bl 0x82fd82e0
	ctx.lr = 0x8309BE08;
	sub_82FD82E0(ctx, base);
	// 8309BE08: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8309BE0C: 4BFFFA05  bl 0x8309b810
	ctx.lr = 0x8309BE10;
	sub_8309B810(ctx, base);
	// 8309BE10: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 8309BE14: 4810C3A0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309BE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309BE18 size=40
    let mut pc: u32 = 0x8309BE18;
    'dispatch: loop {
        match pc {
            0x8309BE18 => {
    //   block [0x8309BE18..0x8309BE40)
	// 8309BE18: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 8309BE1C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309BE20: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309BE24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309BE28: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8309BE2C: 4BFFF9E5  bl 0x8309b810
	ctx.lr = 0x8309BE30;
	sub_8309B810(ctx, base);
	// 8309BE30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309BE34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309BE38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309BE3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309BE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309BE40 size=8
    let mut pc: u32 = 0x8309BE40;
    'dispatch: loop {
        match pc {
            0x8309BE40 => {
    //   block [0x8309BE40..0x8309BE48)
	// 8309BE40: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309BE44: 8216E120  lwz r16, -0x1ee0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-7904 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309BE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309BE48 size=376
    let mut pc: u32 = 0x8309BE48;
    'dispatch: loop {
        match pc {
            0x8309BE48 => {
    //   block [0x8309BE48..0x8309BFC0)
	// 8309BE48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309BE4C: 4810C321  bl 0x831a816c
	ctx.lr = 0x8309BE50;
	sub_831A8130(ctx, base);
	// 8309BE50: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8309BE54: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309BE58: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309BE5C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8309BE60: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309BE64: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8309BE68: 4BF3C431  bl 0x82fd8298
	ctx.lr = 0x8309BE6C;
	sub_82FD8298(ctx, base);
	// 8309BE6C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8309BE70: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8309BE74: 4182002C  beq 0x8309bea0
	if ctx.cr[0].eq {
	pc = 0x8309BEA0; continue 'dispatch;
	}
	// 8309BE78: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8309BE7C: 80DE0014  lwz r6, 0x14(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309BE80: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 8309BE84: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309BE88: 4BFFEB51  bl 0x8309a9d8
	ctx.lr = 0x8309BE8C;
	sub_8309A9D8(ctx, base);
	// 8309BE8C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309BE90: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 8309BE94: 396BDEA8  addi r11, r11, -0x2158
	ctx.r[11].s64 = ctx.r[11].s64 + -8536;
	// 8309BE98: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309BE9C: 48000008  b 0x8309bea4
	pc = 0x8309BEA4; continue 'dispatch;
	// 8309BEA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8309BEA4: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8309BEA8: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309BEAC: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8309BEB0: 4BF3C3E9  bl 0x82fd8298
	ctx.lr = 0x8309BEB4;
	sub_82FD8298(ctx, base);
	// 8309BEB4: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8309BEB8: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8309BEBC: 41820048  beq 0x8309bf04
	if ctx.cr[0].eq {
	pc = 0x8309BF04; continue 'dispatch;
	}
	// 8309BEC0: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8309BEC4: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309BEC8: 4BF3C3D1  bl 0x82fd8298
	ctx.lr = 0x8309BECC;
	sub_82FD8298(ctx, base);
	// 8309BECC: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8309BED0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309BED4: 41820010  beq 0x8309bee4
	if ctx.cr[0].eq {
	pc = 0x8309BEE4; continue 'dispatch;
	}
	// 8309BED8: 4BF636B9  bl 0x82fff590
	ctx.lr = 0x8309BEDC;
	sub_82FFF590(ctx, base);
	// 8309BEDC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8309BEE0: 48000008  b 0x8309bee8
	pc = 0x8309BEE8; continue 'dispatch;
	// 8309BEE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8309BEE8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8309BEEC: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 8309BEF0: 80FE0014  lwz r7, 0x14(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309BEF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309BEF8: 4BF9F1F9  bl 0x8303b0f0
	ctx.lr = 0x8309BEFC;
	sub_8303B0F0(ctx, base);
	// 8309BEFC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8309BF00: 48000008  b 0x8309bf08
	pc = 0x8309BF08; continue 'dispatch;
	// 8309BF04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309BF08: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 8309BF0C: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309BF10: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8309BF14: 4BF3C385  bl 0x82fd8298
	ctx.lr = 0x8309BF18;
	sub_82FD8298(ctx, base);
	// 8309BF18: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8309BF1C: 93BF0054  stw r29, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 8309BF20: 41820048  beq 0x8309bf68
	if ctx.cr[0].eq {
	pc = 0x8309BF68; continue 'dispatch;
	}
	// 8309BF24: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8309BF28: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309BF2C: 4BF3C36D  bl 0x82fd8298
	ctx.lr = 0x8309BF30;
	sub_82FD8298(ctx, base);
	// 8309BF30: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8309BF34: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309BF38: 41820010  beq 0x8309bf48
	if ctx.cr[0].eq {
	pc = 0x8309BF48; continue 'dispatch;
	}
	// 8309BF3C: 4BF63655  bl 0x82fff590
	ctx.lr = 0x8309BF40;
	sub_82FFF590(ctx, base);
	// 8309BF40: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8309BF44: 48000008  b 0x8309bf4c
	pc = 0x8309BF4C; continue 'dispatch;
	// 8309BF48: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8309BF4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8309BF50: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 8309BF54: 80FE0014  lwz r7, 0x14(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309BF58: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309BF5C: 4BF478BD  bl 0x82fe3818
	ctx.lr = 0x8309BF60;
	sub_82FE3818(ctx, base);
	// 8309BF60: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8309BF64: 48000008  b 0x8309bf6c
	pc = 0x8309BF6C; continue 'dispatch;
	// 8309BF68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309BF6C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8309BF70: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309BF74: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8309BF78: 4BF3C321  bl 0x82fd8298
	ctx.lr = 0x8309BF7C;
	sub_82FD8298(ctx, base);
	// 8309BF7C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8309BF80: 93BF0054  stw r29, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 8309BF84: 4182002C  beq 0x8309bfb0
	if ctx.cr[0].eq {
	pc = 0x8309BFB0; continue 'dispatch;
	}
	// 8309BF88: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8309BF8C: 80DE0014  lwz r6, 0x14(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309BF90: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 8309BF94: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309BF98: 4BFFEB99  bl 0x8309ab30
	ctx.lr = 0x8309BF9C;
	sub_8309AB30(ctx, base);
	// 8309BF9C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309BFA0: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 8309BFA4: 396BDF40  addi r11, r11, -0x20c0
	ctx.r[11].s64 = ctx.r[11].s64 + -8384;
	// 8309BFA8: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309BFAC: 48000008  b 0x8309bfb4
	pc = 0x8309BFB4; continue 'dispatch;
	// 8309BFB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8309BFB4: 915E000C  stw r10, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8309BFB8: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8309BFBC: 4810C200  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309BFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309BFC0 size=48
    let mut pc: u32 = 0x8309BFC0;
    'dispatch: loop {
        match pc {
            0x8309BFC0 => {
    //   block [0x8309BFC0..0x8309BFF0)
	// 8309BFC0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8309BFC4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309BFC8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309BFCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309BFD0: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8309BFD4: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309BFD8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309BFDC: 4BF3C305  bl 0x82fd82e0
	ctx.lr = 0x8309BFE0;
	sub_82FD82E0(ctx, base);
	// 8309BFE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309BFE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309BFE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309BFEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309BFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309BFF0 size=48
    let mut pc: u32 = 0x8309BFF0;
    'dispatch: loop {
        match pc {
            0x8309BFF0 => {
    //   block [0x8309BFF0..0x8309C020)
	// 8309BFF0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8309BFF4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309BFF8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309BFFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309C000: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8309C004: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309C008: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309C00C: 4BF3C2D5  bl 0x82fd82e0
	ctx.lr = 0x8309C010;
	sub_82FD82E0(ctx, base);
	// 8309C010: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309C014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309C018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309C01C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309C020 size=48
    let mut pc: u32 = 0x8309C020;
    'dispatch: loop {
        match pc {
            0x8309C020 => {
    //   block [0x8309C020..0x8309C050)
	// 8309C020: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8309C024: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309C028: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309C02C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309C030: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8309C034: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309C038: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8309C03C: 4BF3C2A5  bl 0x82fd82e0
	ctx.lr = 0x8309C040;
	sub_82FD82E0(ctx, base);
	// 8309C040: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309C044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309C048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309C04C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309C050 size=48
    let mut pc: u32 = 0x8309C050;
    'dispatch: loop {
        match pc {
            0x8309C050 => {
    //   block [0x8309C050..0x8309C080)
	// 8309C050: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8309C054: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309C058: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309C05C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309C060: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8309C064: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309C068: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8309C06C: 4BF3C275  bl 0x82fd82e0
	ctx.lr = 0x8309C070;
	sub_82FD82E0(ctx, base);
	// 8309C070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309C074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309C078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309C07C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309C080 size=48
    let mut pc: u32 = 0x8309C080;
    'dispatch: loop {
        match pc {
            0x8309C080 => {
    //   block [0x8309C080..0x8309C0B0)
	// 8309C080: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8309C084: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309C088: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309C08C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309C090: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8309C094: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309C098: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309C09C: 4BF3C245  bl 0x82fd82e0
	ctx.lr = 0x8309C0A0;
	sub_82FD82E0(ctx, base);
	// 8309C0A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309C0A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309C0A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309C0AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309C0B0 size=48
    let mut pc: u32 = 0x8309C0B0;
    'dispatch: loop {
        match pc {
            0x8309C0B0 => {
    //   block [0x8309C0B0..0x8309C0E0)
	// 8309C0B0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8309C0B4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309C0B8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309C0BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309C0C0: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8309C0C4: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309C0C8: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8309C0CC: 4BF3C215  bl 0x82fd82e0
	ctx.lr = 0x8309C0D0;
	sub_82FD82E0(ctx, base);
	// 8309C0D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309C0D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309C0D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309C0DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309C0E0 size=164
    let mut pc: u32 = 0x8309C0E0;
    'dispatch: loop {
        match pc {
            0x8309C0E0 => {
    //   block [0x8309C0E0..0x8309C184)
	// 8309C0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309C0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309C0E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309C0EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309C0F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309C0F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309C0F8: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309C0FC: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309C100: 41820014  beq 0x8309c114
	if ctx.cr[0].eq {
	pc = 0x8309C114; continue 'dispatch;
	}
	// 8309C104: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309C108: 4BFFF699  bl 0x8309b7a0
	ctx.lr = 0x8309C10C;
	sub_8309B7A0(ctx, base);
	// 8309C10C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309C110: 4BF3C1D1  bl 0x82fd82e0
	ctx.lr = 0x8309C114;
	sub_82FD82E0(ctx, base);
	// 8309C114: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309C118: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309C11C: 41820014  beq 0x8309c130
	if ctx.cr[0].eq {
	pc = 0x8309C130; continue 'dispatch;
	}
	// 8309C120: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309C124: 4BFFF41D  bl 0x8309b540
	ctx.lr = 0x8309C128;
	sub_8309B540(ctx, base);
	// 8309C128: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309C12C: 4BF3C1B5  bl 0x82fd82e0
	ctx.lr = 0x8309C130;
	sub_82FD82E0(ctx, base);
	// 8309C130: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309C134: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309C138: 41820014  beq 0x8309c14c
	if ctx.cr[0].eq {
	pc = 0x8309C14C; continue 'dispatch;
	}
	// 8309C13C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309C140: 4BFFF761  bl 0x8309b8a0
	ctx.lr = 0x8309C144;
	sub_8309B8A0(ctx, base);
	// 8309C144: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309C148: 4BF3C199  bl 0x82fd82e0
	ctx.lr = 0x8309C14C;
	sub_82FD82E0(ctx, base);
	// 8309C14C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C150: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309C154: 41820018  beq 0x8309c16c
	if ctx.cr[0].eq {
	pc = 0x8309C16C; continue 'dispatch;
	}
	// 8309C158: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C15C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8309C160: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C164: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309C168: 4E800421  bctrl
	ctx.lr = 0x8309C16C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309C16C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309C170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309C174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309C178: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309C17C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309C180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309C188 size=8
    let mut pc: u32 = 0x8309C188;
    'dispatch: loop {
        match pc {
            0x8309C188 => {
    //   block [0x8309C188..0x8309C190)
	// 8309C188: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309C18C: 8216E1EC  lwz r16, -0x1e14(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-7700 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309C190 size=108
    let mut pc: u32 = 0x8309C190;
    'dispatch: loop {
        match pc {
            0x8309C190 => {
    //   block [0x8309C190..0x8309C1FC)
	// 8309C190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309C194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309C198: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 8309C19C: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 8309C1A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309C1A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309C1A8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8309C1AC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309C1B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309C1B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309C1B8: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8309C1BC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309C1C0: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8309C1C4: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8309C1C8: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8309C1CC: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8309C1D0: 909E0014  stw r4, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 8309C1D4: 4BFFFC75  bl 0x8309be48
	ctx.lr = 0x8309C1D8;
	sub_8309BE48(ctx, base);
	// 8309C1D8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8309C1DC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8309C1E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309C1E4: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8309C1E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309C1EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309C1F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309C1F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309C1F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C1FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309C1FC size=8
    let mut pc: u32 = 0x8309C1FC;
    'dispatch: loop {
        match pc {
            0x8309C1FC => {
    //   block [0x8309C1FC..0x8309C204)
	// 8309C1FC: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309C200: 8216E1EC  lwz r16, -0x1e14(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-7700 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C204(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309C204 size=24
    let mut pc: u32 = 0x8309C204;
    'dispatch: loop {
        match pc {
            0x8309C204 => {
    //   block [0x8309C204..0x8309C21C)
	// 8309C204: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309C208: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309C20C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309C210: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8309C214: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309C218: 48114A11  bl 0x831b0c28
	ctx.lr = 0x8309C21C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C224(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309C224 size=36
    let mut pc: u32 = 0x8309C224;
    'dispatch: loop {
        match pc {
            0x8309C224 => {
    //   block [0x8309C224..0x8309C248)
	// 8309C224: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8309C228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309C22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309C230: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309C234: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8309C238: 4BFFFEA9  bl 0x8309c0e0
	ctx.lr = 0x8309C23C;
	sub_8309C0E0(ctx, base);
	// 8309C23C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8309C240: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309C244: 481149E5  bl 0x831b0c28
	ctx.lr = 0x8309C248;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309C248 size=4
    let mut pc: u32 = 0x8309C248;
    'dispatch: loop {
        match pc {
            0x8309C248 => {
    //   block [0x8309C248..0x8309C24C)
	// 8309C248: 4BFFFE98  b 0x8309c0e0
	sub_8309C0E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309C250 size=84
    let mut pc: u32 = 0x8309C250;
    'dispatch: loop {
        match pc {
            0x8309C250 => {
    //   block [0x8309C250..0x8309C2A4)
	// 8309C250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309C254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309C258: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309C25C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309C260: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309C264: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309C268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309C26C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8309C270: 90DF0000  stw r6, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 8309C274: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8309C278: 909F0008  stw r4, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 8309C27C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8309C280: 4BF897E9  bl 0x83025a68
	ctx.lr = 0x8309C284;
	sub_83025A68(ctx, base);
	// 8309C284: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309C288: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8309C28C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309C290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309C294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309C298: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309C29C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309C2A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309C2A8 size=8
    let mut pc: u32 = 0x8309C2A8;
    'dispatch: loop {
        match pc {
            0x8309C2A8 => {
    //   block [0x8309C2A8..0x8309C2B0)
	// 8309C2A8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309C2AC: 8216E230  lwz r16, -0x1dd0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-7632 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309C2B0 size=160
    let mut pc: u32 = 0x8309C2B0;
    'dispatch: loop {
        match pc {
            0x8309C2B0 => {
    //   block [0x8309C2B0..0x8309C350)
	// 8309C2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309C2B4: 4810BEB5  bl 0x831a8168
	ctx.lr = 0x8309C2B8;
	sub_831A8130(ctx, base);
	// 8309C2B8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8309C2BC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309C2C0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309C2C4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8309C2C8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8309C2CC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8309C2D0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8309C2D4: 90BE0004  stw r5, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 8309C2D8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309C2DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309C2E0: 93BF00AC  stw r29, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[29].u32 ) };
	// 8309C2E4: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 8309C2E8: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8309C2EC: 4BF3BFAD  bl 0x82fd8298
	ctx.lr = 0x8309C2F0;
	sub_82FD8298(ctx, base);
	// 8309C2F0: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8309C2F4: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 8309C2F8: 41820044  beq 0x8309c33c
	if ctx.cr[0].eq {
	pc = 0x8309C33C; continue 'dispatch;
	}
	// 8309C2FC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8309C300: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8309C304: 4BF3BF95  bl 0x82fd8298
	ctx.lr = 0x8309C308;
	sub_82FD8298(ctx, base);
	// 8309C308: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8309C30C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309C310: 41820010  beq 0x8309c320
	if ctx.cr[0].eq {
	pc = 0x8309C320; continue 'dispatch;
	}
	// 8309C314: 4BF6327D  bl 0x82fff590
	ctx.lr = 0x8309C318;
	sub_82FFF590(ctx, base);
	// 8309C318: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8309C31C: 48000008  b 0x8309c324
	pc = 0x8309C324; continue 'dispatch;
	// 8309C320: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8309C324: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8309C328: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 8309C32C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309C330: 4BFFFF21  bl 0x8309c250
	ctx.lr = 0x8309C334;
	sub_8309C250(ctx, base);
	// 8309C334: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8309C338: 48000008  b 0x8309c340
	pc = 0x8309C340; continue 'dispatch;
	// 8309C33C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309C340: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309C344: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8309C348: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8309C34C: 4810BE6C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309C350 size=44
    let mut pc: u32 = 0x8309C350;
    'dispatch: loop {
        match pc {
            0x8309C350 => {
    //   block [0x8309C350..0x8309C37C)
	// 8309C350: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8309C354: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309C358: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309C35C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309C360: 809F00AC  lwz r4, 0xac(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 8309C364: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309C368: 4BF3BF79  bl 0x82fd82e0
	ctx.lr = 0x8309C36C;
	sub_82FD82E0(ctx, base);
	// 8309C36C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309C370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309C374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309C378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


