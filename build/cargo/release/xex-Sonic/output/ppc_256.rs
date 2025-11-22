pub fn sub_830A33D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A33D0 size=48
    let mut pc: u32 = 0x830A33D0;
    'dispatch: loop {
        match pc {
            0x830A33D0 => {
    //   block [0x830A33D0..0x830A3400)
	// 830A33D0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A33D4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A33D8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A33DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A33E0: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A33E4: 808B003C  lwz r4, 0x3c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A33E8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A33EC: 4BF34EF5  bl 0x82fd82e0
	ctx.lr = 0x830A33F0;
	sub_82FD82E0(ctx, base);
	// 830A33F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A33F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A33F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A33FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A3400 size=8
    let mut pc: u32 = 0x830A3400;
    'dispatch: loop {
        match pc {
            0x830A3400 => {
    //   block [0x830A3400..0x830A3408)
	// 830A3400: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A3404: 8216F338  lwz r16, -0xcc8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-3272 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3408 size=112
    let mut pc: u32 = 0x830A3408;
    'dispatch: loop {
        match pc {
            0x830A3408 => {
    //   block [0x830A3408..0x830A3478)
	// 830A3408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A340C: 48104D5D  bl 0x831a8168
	ctx.lr = 0x830A3410;
	sub_831A8130(ctx, base);
	// 830A3410: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A3414: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3418: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A341C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 830A3420: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830A3424: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830A3428: 809E003C  lwz r4, 0x3c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A342C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830A3430: 4BF34E69  bl 0x82fd8298
	ctx.lr = 0x830A3434;
	sub_82FD8298(ctx, base);
	// 830A3434: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A3438: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A343C: 41820020  beq 0x830a345c
	if ctx.cr[0].eq {
	pc = 0x830A345C; continue 'dispatch;
	}
	// 830A3440: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 830A3444: 80FE003C  lwz r7, 0x3c(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A3448: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A344C: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 830A3450: 4800DDD9  bl 0x830b1228
	ctx.lr = 0x830A3454;
	sub_830B1228(ctx, base);
	// 830A3454: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A3458: 48000008  b 0x830a3460
	pc = 0x830A3460; continue 'dispatch;
	// 830A345C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A3460: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A3464: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A3468: 4BF97CE9  bl 0x8303b150
	ctx.lr = 0x830A346C;
	sub_8303B150(ctx, base);
	// 830A346C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A3470: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A3474: 48104D44  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3478 size=48
    let mut pc: u32 = 0x830A3478;
    'dispatch: loop {
        match pc {
            0x830A3478 => {
    //   block [0x830A3478..0x830A34A8)
	// 830A3478: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A347C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3480: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3484: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3488: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A348C: 808B003C  lwz r4, 0x3c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A3490: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A3494: 4BF34E4D  bl 0x82fd82e0
	ctx.lr = 0x830A3498;
	sub_82FD82E0(ctx, base);
	// 830A3498: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A349C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A34A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A34A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A34A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A34A8 size=8
    let mut pc: u32 = 0x830A34A8;
    'dispatch: loop {
        match pc {
            0x830A34A8 => {
    //   block [0x830A34A8..0x830A34B0)
	// 830A34A8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A34AC: 8216F388  lwz r16, -0xc78(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-3192 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A34B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A34B0 size=168
    let mut pc: u32 = 0x830A34B0;
    'dispatch: loop {
        match pc {
            0x830A34B0 => {
    //   block [0x830A34B0..0x830A3558)
	// 830A34B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A34B4: 48104CB9  bl 0x831a816c
	ctx.lr = 0x830A34B8;
	sub_831A8130(ctx, base);
	// 830A34B8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A34BC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A34C0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A34C4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830A34C8: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A34CC: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830A34D0: 4182003C  beq 0x830a350c
	if ctx.cr[0].eq {
	pc = 0x830A350C; continue 'dispatch;
	}
	// 830A34D4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830A34D8: 809E003C  lwz r4, 0x3c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A34DC: 4BF34DBD  bl 0x82fd8298
	ctx.lr = 0x830A34E0;
	sub_82FD8298(ctx, base);
	// 830A34E0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A34E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A34E8: 41820018  beq 0x830a3500
	if ctx.cr[0].eq {
	pc = 0x830A3500; continue 'dispatch;
	}
	// 830A34EC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A34F0: 80DE003C  lwz r6, 0x3c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A34F4: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 830A34F8: 4800DDC9  bl 0x830b12c0
	ctx.lr = 0x830A34FC;
	sub_830B12C0(ctx, base);
	// 830A34FC: 48000008  b 0x830a3504
	pc = 0x830A3504; continue 'dispatch;
	// 830A3500: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A3504: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A3508: 48000038  b 0x830a3540
	pc = 0x830A3540; continue 'dispatch;
	// 830A350C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830A3510: 809E003C  lwz r4, 0x3c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A3514: 4BF34D85  bl 0x82fd8298
	ctx.lr = 0x830A3518;
	sub_82FD8298(ctx, base);
	// 830A3518: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A351C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A3520: 41820018  beq 0x830a3538
	if ctx.cr[0].eq {
	pc = 0x830A3538; continue 'dispatch;
	}
	// 830A3524: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A3528: 80DE003C  lwz r6, 0x3c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A352C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 830A3530: 4800DD91  bl 0x830b12c0
	ctx.lr = 0x830A3534;
	sub_830B12C0(ctx, base);
	// 830A3534: 48000008  b 0x830a353c
	pc = 0x830A353C; continue 'dispatch;
	// 830A3538: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A353C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A3540: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A3544: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A3548: 4BF97C09  bl 0x8303b150
	ctx.lr = 0x830A354C;
	sub_8303B150(ctx, base);
	// 830A354C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A3550: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A3554: 48104C68  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3558 size=48
    let mut pc: u32 = 0x830A3558;
    'dispatch: loop {
        match pc {
            0x830A3558 => {
    //   block [0x830A3558..0x830A3588)
	// 830A3558: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A355C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3560: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3564: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3568: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A356C: 808B003C  lwz r4, 0x3c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A3570: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A3574: 4BF34D6D  bl 0x82fd82e0
	ctx.lr = 0x830A3578;
	sub_82FD82E0(ctx, base);
	// 830A3578: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A357C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A3580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3588 size=48
    let mut pc: u32 = 0x830A3588;
    'dispatch: loop {
        match pc {
            0x830A3588 => {
    //   block [0x830A3588..0x830A35B8)
	// 830A3588: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A358C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3590: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3594: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3598: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A359C: 808B003C  lwz r4, 0x3c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A35A0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A35A4: 4BF34D3D  bl 0x82fd82e0
	ctx.lr = 0x830A35A8;
	sub_82FD82E0(ctx, base);
	// 830A35A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A35AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A35B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A35B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A35B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A35B8 size=8
    let mut pc: u32 = 0x830A35B8;
    'dispatch: loop {
        match pc {
            0x830A35B8 => {
    //   block [0x830A35B8..0x830A35C0)
	// 830A35B8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A35BC: 8216F3E0  lwz r16, -0xc20(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-3104 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A35C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A35C0 size=108
    let mut pc: u32 = 0x830A35C0;
    'dispatch: loop {
        match pc {
            0x830A35C0 => {
    //   block [0x830A35C0..0x830A362C)
	// 830A35C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A35C4: 48104BA5  bl 0x831a8168
	ctx.lr = 0x830A35C8;
	sub_831A8130(ctx, base);
	// 830A35C8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A35CC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A35D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A35D4: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 830A35D8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830A35DC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830A35E0: 809E003C  lwz r4, 0x3c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A35E4: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830A35E8: 4BF34CB1  bl 0x82fd8298
	ctx.lr = 0x830A35EC;
	sub_82FD8298(ctx, base);
	// 830A35EC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A35F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A35F4: 4182001C  beq 0x830a3610
	if ctx.cr[0].eq {
	pc = 0x830A3610; continue 'dispatch;
	}
	// 830A35F8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830A35FC: 80DE003C  lwz r6, 0x3c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A3600: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A3604: 4800DD75  bl 0x830b1378
	ctx.lr = 0x830A3608;
	sub_830B1378(ctx, base);
	// 830A3608: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A360C: 48000008  b 0x830a3614
	pc = 0x830A3614; continue 'dispatch;
	// 830A3610: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A3614: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A3618: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A361C: 4BF97B35  bl 0x8303b150
	ctx.lr = 0x830A3620;
	sub_8303B150(ctx, base);
	// 830A3620: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A3624: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A3628: 48104B90  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A362C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A362C size=48
    let mut pc: u32 = 0x830A362C;
    'dispatch: loop {
        match pc {
            0x830A362C => {
    //   block [0x830A362C..0x830A365C)
	// 830A362C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A3630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A363C: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A3640: 808B003C  lwz r4, 0x3c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A3644: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A3648: 4BF34C99  bl 0x82fd82e0
	ctx.lr = 0x830A364C;
	sub_82FD82E0(ctx, base);
	// 830A364C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A3650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A3654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A3660 size=8
    let mut pc: u32 = 0x830A3660;
    'dispatch: loop {
        match pc {
            0x830A3660 => {
    //   block [0x830A3660..0x830A3668)
	// 830A3660: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A3664: 8216F430  lwz r16, -0xbd0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-3024 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3668 size=156
    let mut pc: u32 = 0x830A3668;
    'dispatch: loop {
        match pc {
            0x830A3668 => {
    //   block [0x830A3668..0x830A3704)
	// 830A3668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A366C: 48104B01  bl 0x831a816c
	ctx.lr = 0x830A3670;
	sub_831A8130(ctx, base);
	// 830A3670: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A3674: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3678: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A367C: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A3680: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 830A3684: 41820038  beq 0x830a36bc
	if ctx.cr[0].eq {
	pc = 0x830A36BC; continue 'dispatch;
	}
	// 830A3688: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 830A368C: 809D003C  lwz r4, 0x3c(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A3690: 4BF34C09  bl 0x82fd8298
	ctx.lr = 0x830A3694;
	sub_82FD8298(ctx, base);
	// 830A3694: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A3698: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A369C: 41820014  beq 0x830a36b0
	if ctx.cr[0].eq {
	pc = 0x830A36B0; continue 'dispatch;
	}
	// 830A36A0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830A36A4: 80BD003C  lwz r5, 0x3c(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A36A8: 4800DDE9  bl 0x830b1490
	ctx.lr = 0x830A36AC;
	sub_830B1490(ctx, base);
	// 830A36AC: 48000008  b 0x830a36b4
	pc = 0x830A36B4; continue 'dispatch;
	// 830A36B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A36B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A36B8: 48000034  b 0x830a36ec
	pc = 0x830A36EC; continue 'dispatch;
	// 830A36BC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 830A36C0: 809D003C  lwz r4, 0x3c(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A36C4: 4BF34BD5  bl 0x82fd8298
	ctx.lr = 0x830A36C8;
	sub_82FD8298(ctx, base);
	// 830A36C8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A36CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A36D0: 41820014  beq 0x830a36e4
	if ctx.cr[0].eq {
	pc = 0x830A36E4; continue 'dispatch;
	}
	// 830A36D4: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 830A36D8: 80BD003C  lwz r5, 0x3c(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A36DC: 4800DDB5  bl 0x830b1490
	ctx.lr = 0x830A36E0;
	sub_830B1490(ctx, base);
	// 830A36E0: 48000008  b 0x830a36e8
	pc = 0x830A36E8; continue 'dispatch;
	// 830A36E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A36E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A36EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A36F0: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A36F4: 4BF97A5D  bl 0x8303b150
	ctx.lr = 0x830A36F8;
	sub_8303B150(ctx, base);
	// 830A36F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A36FC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A3700: 48104ABC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3704(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3704 size=48
    let mut pc: u32 = 0x830A3704;
    'dispatch: loop {
        match pc {
            0x830A3704 => {
    //   block [0x830A3704..0x830A3734)
	// 830A3704: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A3708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A370C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3710: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3714: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A3718: 808B003C  lwz r4, 0x3c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A371C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A3720: 4BF34BC1  bl 0x82fd82e0
	ctx.lr = 0x830A3724;
	sub_82FD82E0(ctx, base);
	// 830A3724: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A3728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A372C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3734(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3734 size=48
    let mut pc: u32 = 0x830A3734;
    'dispatch: loop {
        match pc {
            0x830A3734 => {
    //   block [0x830A3734..0x830A3764)
	// 830A3734: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A3738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A373C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3740: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3744: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A3748: 808B003C  lwz r4, 0x3c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A374C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A3750: 4BF34B91  bl 0x82fd82e0
	ctx.lr = 0x830A3754;
	sub_82FD82E0(ctx, base);
	// 830A3754: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A3758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A375C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A3768 size=8
    let mut pc: u32 = 0x830A3768;
    'dispatch: loop {
        match pc {
            0x830A3768 => {
    //   block [0x830A3768..0x830A3770)
	// 830A3768: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A376C: 8216F490  lwz r16, -0xb70(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-2928 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3770 size=156
    let mut pc: u32 = 0x830A3770;
    'dispatch: loop {
        match pc {
            0x830A3770 => {
    //   block [0x830A3770..0x830A380C)
	// 830A3770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3774: 481049F9  bl 0x831a816c
	ctx.lr = 0x830A3778;
	sub_831A8130(ctx, base);
	// 830A3778: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A377C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3780: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A3784: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A3788: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 830A378C: 41820038  beq 0x830a37c4
	if ctx.cr[0].eq {
	pc = 0x830A37C4; continue 'dispatch;
	}
	// 830A3790: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 830A3794: 809D003C  lwz r4, 0x3c(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A3798: 4BF34B01  bl 0x82fd8298
	ctx.lr = 0x830A379C;
	sub_82FD8298(ctx, base);
	// 830A379C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A37A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A37A4: 41820014  beq 0x830a37b8
	if ctx.cr[0].eq {
	pc = 0x830A37B8; continue 'dispatch;
	}
	// 830A37A8: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 830A37AC: 80BD003C  lwz r5, 0x3c(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A37B0: 480026D9  bl 0x830a5e88
	ctx.lr = 0x830A37B4;
	sub_830A5E88(ctx, base);
	// 830A37B4: 48000008  b 0x830a37bc
	pc = 0x830A37BC; continue 'dispatch;
	// 830A37B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A37BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A37C0: 48000034  b 0x830a37f4
	pc = 0x830A37F4; continue 'dispatch;
	// 830A37C4: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 830A37C8: 809D003C  lwz r4, 0x3c(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A37CC: 4BF34ACD  bl 0x82fd8298
	ctx.lr = 0x830A37D0;
	sub_82FD8298(ctx, base);
	// 830A37D0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A37D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A37D8: 41820014  beq 0x830a37ec
	if ctx.cr[0].eq {
	pc = 0x830A37EC; continue 'dispatch;
	}
	// 830A37DC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 830A37E0: 80BD003C  lwz r5, 0x3c(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A37E4: 480026A5  bl 0x830a5e88
	ctx.lr = 0x830A37E8;
	sub_830A5E88(ctx, base);
	// 830A37E8: 48000008  b 0x830a37f0
	pc = 0x830A37F0; continue 'dispatch;
	// 830A37EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A37F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A37F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A37F8: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A37FC: 4BF97955  bl 0x8303b150
	ctx.lr = 0x830A3800;
	sub_8303B150(ctx, base);
	// 830A3800: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A3804: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A3808: 481049B4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A380C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A380C size=48
    let mut pc: u32 = 0x830A380C;
    'dispatch: loop {
        match pc {
            0x830A380C => {
    //   block [0x830A380C..0x830A383C)
	// 830A380C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A3810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3818: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A381C: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A3820: 808B003C  lwz r4, 0x3c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A3824: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A3828: 4BF34AB9  bl 0x82fd82e0
	ctx.lr = 0x830A382C;
	sub_82FD82E0(ctx, base);
	// 830A382C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A3830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A3834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A383C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A383C size=48
    let mut pc: u32 = 0x830A383C;
    'dispatch: loop {
        match pc {
            0x830A383C => {
    //   block [0x830A383C..0x830A386C)
	// 830A383C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A3840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3848: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A384C: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A3850: 808B003C  lwz r4, 0x3c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A3854: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A3858: 4BF34A89  bl 0x82fd82e0
	ctx.lr = 0x830A385C;
	sub_82FD82E0(ctx, base);
	// 830A385C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A3860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A3864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A3870 size=8
    let mut pc: u32 = 0x830A3870;
    'dispatch: loop {
        match pc {
            0x830A3870 => {
    //   block [0x830A3870..0x830A3878)
	// 830A3870: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A3874: 8216F4F0  lwz r16, -0xb10(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-2832 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3878 size=168
    let mut pc: u32 = 0x830A3878;
    'dispatch: loop {
        match pc {
            0x830A3878 => {
    //   block [0x830A3878..0x830A3920)
	// 830A3878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A387C: 481048F1  bl 0x831a816c
	ctx.lr = 0x830A3880;
	sub_831A8130(ctx, base);
	// 830A3880: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A3884: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3888: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A388C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830A3890: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A3894: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830A3898: 4182003C  beq 0x830a38d4
	if ctx.cr[0].eq {
	pc = 0x830A38D4; continue 'dispatch;
	}
	// 830A389C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 830A38A0: 809E003C  lwz r4, 0x3c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A38A4: 4BF349F5  bl 0x82fd8298
	ctx.lr = 0x830A38A8;
	sub_82FD8298(ctx, base);
	// 830A38A8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A38AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A38B0: 41820018  beq 0x830a38c8
	if ctx.cr[0].eq {
	pc = 0x830A38C8; continue 'dispatch;
	}
	// 830A38B4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A38B8: 80DE003C  lwz r6, 0x3c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A38BC: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 830A38C0: 4800E089  bl 0x830b1948
	ctx.lr = 0x830A38C4;
	sub_830B1948(ctx, base);
	// 830A38C4: 48000008  b 0x830a38cc
	pc = 0x830A38CC; continue 'dispatch;
	// 830A38C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A38CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A38D0: 48000038  b 0x830a3908
	pc = 0x830A3908; continue 'dispatch;
	// 830A38D4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 830A38D8: 809E003C  lwz r4, 0x3c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A38DC: 4BF349BD  bl 0x82fd8298
	ctx.lr = 0x830A38E0;
	sub_82FD8298(ctx, base);
	// 830A38E0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A38E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A38E8: 41820018  beq 0x830a3900
	if ctx.cr[0].eq {
	pc = 0x830A3900; continue 'dispatch;
	}
	// 830A38EC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A38F0: 80DE003C  lwz r6, 0x3c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A38F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A38F8: 4800E051  bl 0x830b1948
	ctx.lr = 0x830A38FC;
	sub_830B1948(ctx, base);
	// 830A38FC: 48000008  b 0x830a3904
	pc = 0x830A3904; continue 'dispatch;
	// 830A3900: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A3904: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A3908: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A390C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A3910: 4BF97841  bl 0x8303b150
	ctx.lr = 0x830A3914;
	sub_8303B150(ctx, base);
	// 830A3914: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A3918: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A391C: 481048A0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3920 size=48
    let mut pc: u32 = 0x830A3920;
    'dispatch: loop {
        match pc {
            0x830A3920 => {
    //   block [0x830A3920..0x830A3950)
	// 830A3920: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A3924: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3928: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A392C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3930: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A3934: 808B003C  lwz r4, 0x3c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A3938: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A393C: 4BF349A5  bl 0x82fd82e0
	ctx.lr = 0x830A3940;
	sub_82FD82E0(ctx, base);
	// 830A3940: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A3944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A3948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A394C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3950 size=48
    let mut pc: u32 = 0x830A3950;
    'dispatch: loop {
        match pc {
            0x830A3950 => {
    //   block [0x830A3950..0x830A3980)
	// 830A3950: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A3954: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3958: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A395C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3960: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A3964: 808B003C  lwz r4, 0x3c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A3968: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A396C: 4BF34975  bl 0x82fd82e0
	ctx.lr = 0x830A3970;
	sub_82FD82E0(ctx, base);
	// 830A3970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A3974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A3978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A397C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A3980 size=8
    let mut pc: u32 = 0x830A3980;
    'dispatch: loop {
        match pc {
            0x830A3980 => {
    //   block [0x830A3980..0x830A3988)
	// 830A3980: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A3984: 8216F548  lwz r16, -0xab8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-2744 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3988 size=108
    let mut pc: u32 = 0x830A3988;
    'dispatch: loop {
        match pc {
            0x830A3988 => {
    //   block [0x830A3988..0x830A39F4)
	// 830A3988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A398C: 481047E1  bl 0x831a816c
	ctx.lr = 0x830A3990;
	sub_831A8130(ctx, base);
	// 830A3990: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A3994: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3998: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A399C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830A39A0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830A39A4: 809E003C  lwz r4, 0x3c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A39A8: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830A39AC: 4BF348ED  bl 0x82fd8298
	ctx.lr = 0x830A39B0;
	sub_82FD8298(ctx, base);
	// 830A39B0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A39B4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A39B8: 41820020  beq 0x830a39d8
	if ctx.cr[0].eq {
	pc = 0x830A39D8; continue 'dispatch;
	}
	// 830A39BC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 830A39C0: 80FE003C  lwz r7, 0x3c(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A39C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A39C8: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 830A39CC: 4800E02D  bl 0x830b19f8
	ctx.lr = 0x830A39D0;
	sub_830B19F8(ctx, base);
	// 830A39D0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A39D4: 48000008  b 0x830a39dc
	pc = 0x830A39DC; continue 'dispatch;
	// 830A39D8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A39DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A39E0: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A39E4: 4BF9776D  bl 0x8303b150
	ctx.lr = 0x830A39E8;
	sub_8303B150(ctx, base);
	// 830A39E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A39EC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A39F0: 481047CC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A39F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A39F4 size=48
    let mut pc: u32 = 0x830A39F4;
    'dispatch: loop {
        match pc {
            0x830A39F4 => {
    //   block [0x830A39F4..0x830A3A24)
	// 830A39F4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A39F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A39FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3A00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3A04: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A3A08: 808B003C  lwz r4, 0x3c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A3A0C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A3A10: 4BF348D1  bl 0x82fd82e0
	ctx.lr = 0x830A3A14;
	sub_82FD82E0(ctx, base);
	// 830A3A14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A3A18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A3A1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A3A28 size=8
    let mut pc: u32 = 0x830A3A28;
    'dispatch: loop {
        match pc {
            0x830A3A28 => {
    //   block [0x830A3A28..0x830A3A30)
	// 830A3A28: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A3A2C: 8216F590  lwz r16, -0xa70(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-2672 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3A30 size=108
    let mut pc: u32 = 0x830A3A30;
    'dispatch: loop {
        match pc {
            0x830A3A30 => {
    //   block [0x830A3A30..0x830A3A9C)
	// 830A3A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3A34: 48104739  bl 0x831a816c
	ctx.lr = 0x830A3A38;
	sub_831A8130(ctx, base);
	// 830A3A38: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A3A3C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3A40: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A3A44: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830A3A48: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830A3A4C: 809E003C  lwz r4, 0x3c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A3A50: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830A3A54: 4BF34845  bl 0x82fd8298
	ctx.lr = 0x830A3A58;
	sub_82FD8298(ctx, base);
	// 830A3A58: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A3A5C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A3A60: 41820020  beq 0x830a3a80
	if ctx.cr[0].eq {
	pc = 0x830A3A80; continue 'dispatch;
	}
	// 830A3A64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830A3A68: 80FE003C  lwz r7, 0x3c(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A3A6C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A3A70: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 830A3A74: 4800DF85  bl 0x830b19f8
	ctx.lr = 0x830A3A78;
	sub_830B19F8(ctx, base);
	// 830A3A78: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A3A7C: 48000008  b 0x830a3a84
	pc = 0x830A3A84; continue 'dispatch;
	// 830A3A80: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A3A84: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A3A88: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A3A8C: 4BF976C5  bl 0x8303b150
	ctx.lr = 0x830A3A90;
	sub_8303B150(ctx, base);
	// 830A3A90: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A3A94: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A3A98: 48104724  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3A9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3A9C size=48
    let mut pc: u32 = 0x830A3A9C;
    'dispatch: loop {
        match pc {
            0x830A3A9C => {
    //   block [0x830A3A9C..0x830A3ACC)
	// 830A3A9C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A3AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3AA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3AAC: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A3AB0: 808B003C  lwz r4, 0x3c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A3AB4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A3AB8: 4BF34829  bl 0x82fd82e0
	ctx.lr = 0x830A3ABC;
	sub_82FD82E0(ctx, base);
	// 830A3ABC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A3AC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A3AC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3AC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A3AD0 size=8
    let mut pc: u32 = 0x830A3AD0;
    'dispatch: loop {
        match pc {
            0x830A3AD0 => {
    //   block [0x830A3AD0..0x830A3AD8)
	// 830A3AD0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A3AD4: 8216F5D8  lwz r16, -0xa28(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-2600 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3AD8 size=116
    let mut pc: u32 = 0x830A3AD8;
    'dispatch: loop {
        match pc {
            0x830A3AD8 => {
    //   block [0x830A3AD8..0x830A3B4C)
	// 830A3AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3ADC: 48104689  bl 0x831a8164
	ctx.lr = 0x830A3AE0;
	sub_831A8130(ctx, base);
	// 830A3AE0: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830A3AE4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3AE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A3AEC: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830A3AF0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830A3AF4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830A3AF8: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 830A3AFC: 809E003C  lwz r4, 0x3c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A3B00: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 830A3B04: 4BF34795  bl 0x82fd8298
	ctx.lr = 0x830A3B08;
	sub_82FD8298(ctx, base);
	// 830A3B08: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A3B0C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A3B10: 41820020  beq 0x830a3b30
	if ctx.cr[0].eq {
	pc = 0x830A3B30; continue 'dispatch;
	}
	// 830A3B14: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 830A3B18: 80FE003C  lwz r7, 0x3c(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A3B1C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830A3B20: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A3B24: 4800E045  bl 0x830b1b68
	ctx.lr = 0x830A3B28;
	sub_830B1B68(ctx, base);
	// 830A3B28: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A3B2C: 48000008  b 0x830a3b34
	pc = 0x830A3B34; continue 'dispatch;
	// 830A3B30: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A3B34: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A3B38: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A3B3C: 4BF97615  bl 0x8303b150
	ctx.lr = 0x830A3B40;
	sub_8303B150(ctx, base);
	// 830A3B40: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A3B44: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 830A3B48: 4810466C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3B4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3B4C size=48
    let mut pc: u32 = 0x830A3B4C;
    'dispatch: loop {
        match pc {
            0x830A3B4C => {
    //   block [0x830A3B4C..0x830A3B7C)
	// 830A3B4C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830A3B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3B58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3B5C: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830A3B60: 808B003C  lwz r4, 0x3c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A3B64: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A3B68: 4BF34779  bl 0x82fd82e0
	ctx.lr = 0x830A3B6C;
	sub_82FD82E0(ctx, base);
	// 830A3B6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A3B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A3B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A3B80 size=8
    let mut pc: u32 = 0x830A3B80;
    'dispatch: loop {
        match pc {
            0x830A3B80 => {
    //   block [0x830A3B80..0x830A3B88)
	// 830A3B80: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A3B84: 8216F620  lwz r16, -0x9e0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-2528 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3B88 size=124
    let mut pc: u32 = 0x830A3B88;
    'dispatch: loop {
        match pc {
            0x830A3B88 => {
    //   block [0x830A3B88..0x830A3C04)
	// 830A3B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3B8C: 481045D5  bl 0x831a8160
	ctx.lr = 0x830A3B90;
	sub_831A8130(ctx, base);
	// 830A3B90: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830A3B94: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3B98: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A3B9C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 830A3BA0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830A3BA4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830A3BA8: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 830A3BAC: 809E003C  lwz r4, 0x3c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A3BB0: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 830A3BB4: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 830A3BB8: 4BF346E1  bl 0x82fd8298
	ctx.lr = 0x830A3BBC;
	sub_82FD8298(ctx, base);
	// 830A3BBC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A3BC0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A3BC4: 41820024  beq 0x830a3be8
	if ctx.cr[0].eq {
	pc = 0x830A3BE8; continue 'dispatch;
	}
	// 830A3BC8: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 830A3BCC: 811E003C  lwz r8, 0x3c(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A3BD0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 830A3BD4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830A3BD8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A3BDC: 4800E035  bl 0x830b1c10
	ctx.lr = 0x830A3BE0;
	sub_830B1C10(ctx, base);
	// 830A3BE0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A3BE4: 48000008  b 0x830a3bec
	pc = 0x830A3BEC; continue 'dispatch;
	// 830A3BE8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A3BEC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A3BF0: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A3BF4: 4BF9755D  bl 0x8303b150
	ctx.lr = 0x830A3BF8;
	sub_8303B150(ctx, base);
	// 830A3BF8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A3BFC: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 830A3C00: 481045B0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3C04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3C04 size=48
    let mut pc: u32 = 0x830A3C04;
    'dispatch: loop {
        match pc {
            0x830A3C04 => {
    //   block [0x830A3C04..0x830A3C34)
	// 830A3C04: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830A3C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3C10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3C14: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830A3C18: 808B003C  lwz r4, 0x3c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A3C1C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A3C20: 4BF346C1  bl 0x82fd82e0
	ctx.lr = 0x830A3C24;
	sub_82FD82E0(ctx, base);
	// 830A3C24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A3C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A3C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3C38 size=88
    let mut pc: u32 = 0x830A3C38;
    'dispatch: loop {
        match pc {
            0x830A3C38 => {
    //   block [0x830A3C38..0x830A3C90)
	// 830A3C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3C40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A3C44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A3C48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3C4C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830A3C50: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830A3C54: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830A3C58: 896BBC38  lbz r11, -0x43c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-17352 as u32) ) } as u64;
	// 830A3C5C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A3C60: 40820008  bne 0x830a3c68
	if !ctx.cr[0].eq {
	pc = 0x830A3C68; continue 'dispatch;
	}
	// 830A3C64: 4BFFF2BD  bl 0x830a2f20
	ctx.lr = 0x830A3C68;
	sub_830A2F20(ctx, base);
	// 830A3C68: 4800CE49  bl 0x830b0ab0
	ctx.lr = 0x830A3C6C;
	sub_830B0AB0(ctx, base);
	// 830A3C6C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830A3C70: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830A3C74: 4800C76D  bl 0x830b03e0
	ctx.lr = 0x830A3C78;
	sub_830B03E0(ctx, base);
	// 830A3C78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A3C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A3C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3C84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A3C88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A3C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3C90 size=72
    let mut pc: u32 = 0x830A3C90;
    'dispatch: loop {
        match pc {
            0x830A3C90 => {
    //   block [0x830A3C90..0x830A3CD8)
	// 830A3C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3C98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A3C9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3CA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A3CA4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A3CA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A3CAC: 409A0014  bne cr6, 0x830a3cc0
	if !ctx.cr[6].eq {
	pc = 0x830A3CC0; continue 'dispatch;
	}
	// 830A3CB0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830A3CB4: 3880005E  li r4, 0x5e
	ctx.r[4].s64 = 94;
	// 830A3CB8: 4BFFFBC1  bl 0x830a3878
	ctx.lr = 0x830A3CBC;
	sub_830A3878(ctx, base);
	// 830A3CBC: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 830A3CC0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A3CC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A3CC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A3CCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3CD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A3CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3CD8 size=72
    let mut pc: u32 = 0x830A3CD8;
    'dispatch: loop {
        match pc {
            0x830A3CD8 => {
    //   block [0x830A3CD8..0x830A3D20)
	// 830A3CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3CE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A3CE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3CE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A3CEC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A3CF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A3CF4: 409A0014  bne cr6, 0x830a3d08
	if !ctx.cr[6].eq {
	pc = 0x830A3D08; continue 'dispatch;
	}
	// 830A3CF8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830A3CFC: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 830A3D00: 4BFFFB79  bl 0x830a3878
	ctx.lr = 0x830A3D04;
	sub_830A3878(ctx, base);
	// 830A3D04: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 830A3D08: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A3D0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A3D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A3D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3D18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A3D1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3D20 size=72
    let mut pc: u32 = 0x830A3D20;
    'dispatch: loop {
        match pc {
            0x830A3D20 => {
    //   block [0x830A3D20..0x830A3D68)
	// 830A3D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3D28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A3D2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3D30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A3D34: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A3D38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A3D3C: 409A0014  bne cr6, 0x830a3d50
	if !ctx.cr[6].eq {
	pc = 0x830A3D50; continue 'dispatch;
	}
	// 830A3D40: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830A3D44: 38800041  li r4, 0x41
	ctx.r[4].s64 = 65;
	// 830A3D48: 4BFFFB31  bl 0x830a3878
	ctx.lr = 0x830A3D4C;
	sub_830A3878(ctx, base);
	// 830A3D4C: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 830A3D50: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A3D54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A3D58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A3D5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3D60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A3D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3D68 size=72
    let mut pc: u32 = 0x830A3D68;
    'dispatch: loop {
        match pc {
            0x830A3D68 => {
    //   block [0x830A3D68..0x830A3DB0)
	// 830A3D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3D70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A3D74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3D78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A3D7C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A3D80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A3D84: 409A0014  bne cr6, 0x830a3d98
	if !ctx.cr[6].eq {
	pc = 0x830A3D98; continue 'dispatch;
	}
	// 830A3D88: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830A3D8C: 3880007A  li r4, 0x7a
	ctx.r[4].s64 = 122;
	// 830A3D90: 4BFFFAE9  bl 0x830a3878
	ctx.lr = 0x830A3D94;
	sub_830A3878(ctx, base);
	// 830A3D94: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 830A3D98: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A3D9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A3DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A3DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3DA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A3DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3DB0 size=72
    let mut pc: u32 = 0x830A3DB0;
    'dispatch: loop {
        match pc {
            0x830A3DB0 => {
    //   block [0x830A3DB0..0x830A3DF8)
	// 830A3DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3DB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A3DBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3DC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A3DC4: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A3DC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A3DCC: 409A0014  bne cr6, 0x830a3de0
	if !ctx.cr[6].eq {
	pc = 0x830A3DE0; continue 'dispatch;
	}
	// 830A3DD0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830A3DD4: 3880005A  li r4, 0x5a
	ctx.r[4].s64 = 90;
	// 830A3DD8: 4BFFFAA1  bl 0x830a3878
	ctx.lr = 0x830A3DDC;
	sub_830A3878(ctx, base);
	// 830A3DDC: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 830A3DE0: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A3DE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A3DE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A3DEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3DF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A3DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3DF8 size=72
    let mut pc: u32 = 0x830A3DF8;
    'dispatch: loop {
        match pc {
            0x830A3DF8 => {
    //   block [0x830A3DF8..0x830A3E40)
	// 830A3DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3E00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A3E04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3E08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A3E0C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A3E10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A3E14: 409A0014  bne cr6, 0x830a3e28
	if !ctx.cr[6].eq {
	pc = 0x830A3E28; continue 'dispatch;
	}
	// 830A3E18: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830A3E1C: 38800062  li r4, 0x62
	ctx.r[4].s64 = 98;
	// 830A3E20: 4BFFFA59  bl 0x830a3878
	ctx.lr = 0x830A3E24;
	sub_830A3878(ctx, base);
	// 830A3E24: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 830A3E28: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A3E2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A3E30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A3E34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3E38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A3E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3E40 size=72
    let mut pc: u32 = 0x830A3E40;
    'dispatch: loop {
        match pc {
            0x830A3E40 => {
    //   block [0x830A3E40..0x830A3E88)
	// 830A3E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3E48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A3E4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3E50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A3E54: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A3E58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A3E5C: 409A0014  bne cr6, 0x830a3e70
	if !ctx.cr[6].eq {
	pc = 0x830A3E70; continue 'dispatch;
	}
	// 830A3E60: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830A3E64: 38800042  li r4, 0x42
	ctx.r[4].s64 = 66;
	// 830A3E68: 4BFFFA11  bl 0x830a3878
	ctx.lr = 0x830A3E6C;
	sub_830A3878(ctx, base);
	// 830A3E6C: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 830A3E70: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A3E74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A3E78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A3E7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3E80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A3E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3E88 size=72
    let mut pc: u32 = 0x830A3E88;
    'dispatch: loop {
        match pc {
            0x830A3E88 => {
    //   block [0x830A3E88..0x830A3ED0)
	// 830A3E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3E90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A3E94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3E98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A3E9C: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A3EA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A3EA4: 409A0014  bne cr6, 0x830a3eb8
	if !ctx.cr[6].eq {
	pc = 0x830A3EB8; continue 'dispatch;
	}
	// 830A3EA8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830A3EAC: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 830A3EB0: 4BFFF9C9  bl 0x830a3878
	ctx.lr = 0x830A3EB4;
	sub_830A3878(ctx, base);
	// 830A3EB4: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 830A3EB8: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A3EBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A3EC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A3EC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3EC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A3ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3ED0 size=72
    let mut pc: u32 = 0x830A3ED0;
    'dispatch: loop {
        match pc {
            0x830A3ED0 => {
    //   block [0x830A3ED0..0x830A3F18)
	// 830A3ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3ED8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A3EDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3EE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A3EE4: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A3EE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A3EEC: 409A0014  bne cr6, 0x830a3f00
	if !ctx.cr[6].eq {
	pc = 0x830A3F00; continue 'dispatch;
	}
	// 830A3EF0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830A3EF4: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 830A3EF8: 4BFFF981  bl 0x830a3878
	ctx.lr = 0x830A3EFC;
	sub_830A3878(ctx, base);
	// 830A3EFC: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 830A3F00: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A3F04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A3F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A3F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3F10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A3F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3F18 size=68
    let mut pc: u32 = 0x830A3F18;
    'dispatch: loop {
        match pc {
            0x830A3F18 => {
    //   block [0x830A3F18..0x830A3F5C)
	// 830A3F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3F20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A3F24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3F28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A3F2C: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A3F30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A3F34: 409A0010  bne cr6, 0x830a3f44
	if !ctx.cr[6].eq {
	pc = 0x830A3F44; continue 'dispatch;
	}
	// 830A3F38: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 830A3F3C: 4BFFF365  bl 0x830a32a0
	ctx.lr = 0x830A3F40;
	sub_830A32A0(ctx, base);
	// 830A3F40: 907F0030  stw r3, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 830A3F44: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A3F48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A3F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A3F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3F54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A3F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3F60 size=116
    let mut pc: u32 = 0x830A3F60;
    'dispatch: loop {
        match pc {
            0x830A3F60 => {
    //   block [0x830A3F60..0x830A3FD4)
	// 830A3F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3F64: 48104209  bl 0x831a816c
	ctx.lr = 0x830A3F68;
	sub_831A8130(ctx, base);
	// 830A3F68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3F6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A3F70: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830A3F74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A3F78: 409A0050  bne cr6, 0x830a3fc8
	if !ctx.cr[6].eq {
	pc = 0x830A3FC8; continue 'dispatch;
	}
	// 830A3F7C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A3F80: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A3F84: 3BCBF160  addi r30, r11, -0xea0
	ctx.r[30].s64 = ctx.r[11].s64 + -3744;
	// 830A3F88: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A3F8C: 4BFFFCAD  bl 0x830a3c38
	ctx.lr = 0x830A3F90;
	sub_830A3C38(ctx, base);
	// 830A3F90: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A3F94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A3F98: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A3F9C: 4BFFF515  bl 0x830a34b0
	ctx.lr = 0x830A3FA0;
	sub_830A34B0(ctx, base);
	// 830A3FA0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A3FA4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830A3FA8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A3FAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A3FB0: 4BFFFC89  bl 0x830a3c38
	ctx.lr = 0x830A3FB4;
	sub_830A3C38(ctx, base);
	// 830A3FB4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A3FB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A3FBC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A3FC0: 4BFFF601  bl 0x830a35c0
	ctx.lr = 0x830A3FC4;
	sub_830A35C0(ctx, base);
	// 830A3FC4: 907F0034  stw r3, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[3].u32 ) };
	// 830A3FC8: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830A3FCC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A3FD0: 481041EC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3FD8 size=836
    let mut pc: u32 = 0x830A3FD8;
    'dispatch: loop {
        match pc {
            0x830A3FD8 => {
    //   block [0x830A3FD8..0x830A431C)
	// 830A3FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3FDC: 48104185  bl 0x831a8160
	ctx.lr = 0x830A3FE0;
	sub_831A8130(ctx, base);
	// 830A3FE0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3FE4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A3FE8: 817E0038  lwz r11, 0x38(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 830A3FEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A3FF0: 409A0320  bne cr6, 0x830a4310
	if !ctx.cr[6].eq {
	pc = 0x830A4310; continue 'dispatch;
	}
	// 830A3FF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A3FF8: 4BFFF779  bl 0x830a3770
	ctx.lr = 0x830A3FFC;
	sub_830A3770(ctx, base);
	// 830A3FFC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A4000: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A4004: 3B6BF15C  addi r27, r11, -0xea4
	ctx.r[27].s64 = ctx.r[11].s64 + -3748;
	// 830A4008: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A400C: 389BFFEC  addi r4, r27, -0x14
	ctx.r[4].s64 = ctx.r[27].s64 + -20;
	// 830A4010: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A4014: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A4018: 4BFFFC21  bl 0x830a3c38
	ctx.lr = 0x830A401C;
	sub_830A3C38(ctx, base);
	// 830A401C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A4020: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A4024: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A4028: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A402C: 4E800421  bctrl
	ctx.lr = 0x830A4030;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A4030: 389B0004  addi r4, r27, 4
	ctx.r[4].s64 = ctx.r[27].s64 + 4;
	// 830A4034: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A4038: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A403C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A4040: 4BFFFBF9  bl 0x830a3c38
	ctx.lr = 0x830A4044;
	sub_830A3C38(ctx, base);
	// 830A4044: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A4048: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A404C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A4050: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A4054: 4E800421  bctrl
	ctx.lr = 0x830A4058;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A4058: 389B0008  addi r4, r27, 8
	ctx.r[4].s64 = ctx.r[27].s64 + 8;
	// 830A405C: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A4060: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A4064: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A4068: 4BFFFBD1  bl 0x830a3c38
	ctx.lr = 0x830A406C;
	sub_830A3C38(ctx, base);
	// 830A406C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A4070: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A4074: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A4078: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A407C: 4E800421  bctrl
	ctx.lr = 0x830A4080;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A4080: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A4084: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A4088: 4BFFF6E9  bl 0x830a3770
	ctx.lr = 0x830A408C;
	sub_830A3770(ctx, base);
	// 830A408C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A4090: 38A0094D  li r5, 0x94d
	ctx.r[5].s64 = 2381;
	// 830A4094: 3880094D  li r4, 0x94d
	ctx.r[4].s64 = 2381;
	// 830A4098: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A409C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A40A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A40A4: 4E800421  bctrl
	ctx.lr = 0x830A40A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A40A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A40AC: 38A009CD  li r5, 0x9cd
	ctx.r[5].s64 = 2509;
	// 830A40B0: 388009CD  li r4, 0x9cd
	ctx.r[4].s64 = 2509;
	// 830A40B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A40B8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A40BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A40C0: 4E800421  bctrl
	ctx.lr = 0x830A40C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A40C4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A40C8: 38A00A4D  li r5, 0xa4d
	ctx.r[5].s64 = 2637;
	// 830A40CC: 38800A4D  li r4, 0xa4d
	ctx.r[4].s64 = 2637;
	// 830A40D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A40D4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A40D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A40DC: 4E800421  bctrl
	ctx.lr = 0x830A40E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A40E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A40E4: 38A00ACD  li r5, 0xacd
	ctx.r[5].s64 = 2765;
	// 830A40E8: 38800ACD  li r4, 0xacd
	ctx.r[4].s64 = 2765;
	// 830A40EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A40F0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A40F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A40F8: 4E800421  bctrl
	ctx.lr = 0x830A40FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A40FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A4100: 38A00B4D  li r5, 0xb4d
	ctx.r[5].s64 = 2893;
	// 830A4104: 38800B4D  li r4, 0xb4d
	ctx.r[4].s64 = 2893;
	// 830A4108: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A410C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A4110: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A4114: 4E800421  bctrl
	ctx.lr = 0x830A4118;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A4118: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A411C: 38A00BCD  li r5, 0xbcd
	ctx.r[5].s64 = 3021;
	// 830A4120: 38800BCD  li r4, 0xbcd
	ctx.r[4].s64 = 3021;
	// 830A4124: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A4128: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A412C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A4130: 4E800421  bctrl
	ctx.lr = 0x830A4134;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A4134: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A4138: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A413C: 38A00C4D  li r5, 0xc4d
	ctx.r[5].s64 = 3149;
	// 830A4140: 38800C4D  li r4, 0xc4d
	ctx.r[4].s64 = 3149;
	// 830A4144: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A4148: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A414C: 4E800421  bctrl
	ctx.lr = 0x830A4150;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A4150: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A4154: 38A00CCD  li r5, 0xccd
	ctx.r[5].s64 = 3277;
	// 830A4158: 38800CCD  li r4, 0xccd
	ctx.r[4].s64 = 3277;
	// 830A415C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A4160: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A4164: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A4168: 4E800421  bctrl
	ctx.lr = 0x830A416C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A416C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A4170: 38A00D4D  li r5, 0xd4d
	ctx.r[5].s64 = 3405;
	// 830A4174: 38800D4D  li r4, 0xd4d
	ctx.r[4].s64 = 3405;
	// 830A4178: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A417C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A4180: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A4184: 4E800421  bctrl
	ctx.lr = 0x830A4188;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A4188: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A418C: 38A00E3A  li r5, 0xe3a
	ctx.r[5].s64 = 3642;
	// 830A4190: 38800E3A  li r4, 0xe3a
	ctx.r[4].s64 = 3642;
	// 830A4194: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A4198: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A419C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A41A0: 4E800421  bctrl
	ctx.lr = 0x830A41A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A41A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A41A8: 38A00F84  li r5, 0xf84
	ctx.r[5].s64 = 3972;
	// 830A41AC: 38800F84  li r4, 0xf84
	ctx.r[4].s64 = 3972;
	// 830A41B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A41B4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A41B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A41BC: 4E800421  bctrl
	ctx.lr = 0x830A41C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A41C0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A41C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A41C8: 4BFFF5A9  bl 0x830a3770
	ctx.lr = 0x830A41CC;
	sub_830A3770(ctx, base);
	// 830A41CC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830A41D0: 389B0004  addi r4, r27, 4
	ctx.r[4].s64 = ctx.r[27].s64 + 4;
	// 830A41D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A41D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A41DC: 835C0000  lwz r26, 0(r28)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A41E0: 4BFFFA59  bl 0x830a3c38
	ctx.lr = 0x830A41E4;
	sub_830A3C38(ctx, base);
	// 830A41E4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A41E8: 817A0030  lwz r11, 0x30(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A41EC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830A41F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A41F4: 4E800421  bctrl
	ctx.lr = 0x830A41F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A41F8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A41FC: 38A011FF  li r5, 0x11ff
	ctx.r[5].s64 = 4607;
	// 830A4200: 38801160  li r4, 0x1160
	ctx.r[4].s64 = 4448;
	// 830A4204: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830A4208: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A420C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A4210: 4E800421  bctrl
	ctx.lr = 0x830A4214;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A4214: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A4218: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 830A421C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830A4220: 6144FF9F  ori r4, r10, 0xff9f
	ctx.r[4].u64 = ctx.r[10].u64 | 65439;
	// 830A4224: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A4228: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 830A422C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A4230: 4E800421  bctrl
	ctx.lr = 0x830A4234;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A4234: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A4238: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A423C: 4BFFF42D  bl 0x830a3668
	ctx.lr = 0x830A4240;
	sub_830A3668(ctx, base);
	// 830A4240: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 830A4244: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830A4248: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A424C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A4250: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 830A4254: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A4258: 4E800421  bctrl
	ctx.lr = 0x830A425C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A425C: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 830A4260: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A4264: 83BA0000  lwz r29, 0(r26)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A4268: 4BFFF039  bl 0x830a32a0
	ctx.lr = 0x830A426C;
	sub_830A32A0(ctx, base);
	// 830A426C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A4270: 817D0044  lwz r11, 0x44(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(68 as u32) ) } as u64;
	// 830A4274: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830A4278: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830A427C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A4280: 4E800421  bctrl
	ctx.lr = 0x830A4284;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A4284: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A4288: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A428C: 4BFFF3DD  bl 0x830a3668
	ctx.lr = 0x830A4290;
	sub_830A3668(ctx, base);
	// 830A4290: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A4294: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830A4298: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A429C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A42A0: 837D0000  lwz r27, 0(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A42A4: 4BFFF995  bl 0x830a3c38
	ctx.lr = 0x830A42A8;
	sub_830A3C38(ctx, base);
	// 830A42A8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830A42AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830A42B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A42B4: 4BFFF30D  bl 0x830a35c0
	ctx.lr = 0x830A42B8;
	sub_830A35C0(ctx, base);
	// 830A42B8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A42BC: 817B0044  lwz r11, 0x44(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(68 as u32) ) } as u64;
	// 830A42C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A42C4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830A42C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A42CC: 4E800421  bctrl
	ctx.lr = 0x830A42D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A42D0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A42D4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830A42D8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830A42DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A42E0: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 830A42E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A42E8: 4E800421  bctrl
	ctx.lr = 0x830A42EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A42EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A42F0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A42F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A42F8: 4BFFF1B9  bl 0x830a34b0
	ctx.lr = 0x830A42FC;
	sub_830A34B0(ctx, base);
	// 830A42FC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830A4300: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830A4304: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A4308: 4BFFF2B9  bl 0x830a35c0
	ctx.lr = 0x830A430C;
	sub_830A35C0(ctx, base);
	// 830A430C: 907E0038  stw r3, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[3].u32 ) };
	// 830A4310: 807E0038  lwz r3, 0x38(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 830A4314: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830A4318: 48103E98  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A4320 size=8
    let mut pc: u32 = 0x830A4320;
    'dispatch: loop {
        match pc {
            0x830A4320 => {
    //   block [0x830A4320..0x830A4328)
	// 830A4320: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A4324: 8216F668  lwz r16, -0x998(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-2456 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4328 size=124
    let mut pc: u32 = 0x830A4328;
    'dispatch: loop {
        match pc {
            0x830A4328 => {
    //   block [0x830A4328..0x830A43A4)
	// 830A4328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A432C: 48103E35  bl 0x831a8160
	ctx.lr = 0x830A4330;
	sub_831A8130(ctx, base);
	// 830A4330: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830A4334: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4338: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A433C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 830A4340: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 830A4344: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 830A4348: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 830A434C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4350: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 830A4354: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 830A4358: 4BF33F41  bl 0x82fd8298
	ctx.lr = 0x830A435C;
	sub_82FD8298(ctx, base);
	// 830A435C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A4360: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A4364: 41820020  beq 0x830a4384
	if ctx.cr[0].eq {
	pc = 0x830A4384; continue 'dispatch;
	}
	// 830A4368: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 830A436C: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4370: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A4374: 38800019  li r4, 0x19
	ctx.r[4].s64 = 25;
	// 830A4378: 48000D01  bl 0x830a5078
	ctx.lr = 0x830A437C;
	sub_830A5078(ctx, base);
	// 830A437C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A4380: 48000008  b 0x830a4388
	pc = 0x830A4388; continue 'dispatch;
	// 830A4384: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830A4388: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830A438C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A4390: 937E000C  stw r27, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 830A4394: 4BD622BD  bl 0x82e06650
	ctx.lr = 0x830A4398;
	sub_82E06650(ctx, base);
	// 830A4398: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A439C: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 830A43A0: 48103E10  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A43A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A43A4 size=48
    let mut pc: u32 = 0x830A43A4;
    'dispatch: loop {
        match pc {
            0x830A43A4 => {
    //   block [0x830A43A4..0x830A43D4)
	// 830A43A4: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830A43A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A43AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A43B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A43B4: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830A43B8: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A43BC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A43C0: 4BF33F21  bl 0x82fd82e0
	ctx.lr = 0x830A43C4;
	sub_82FD82E0(ctx, base);
	// 830A43C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A43C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A43CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A43D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A43D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A43D8 size=8
    let mut pc: u32 = 0x830A43D8;
    'dispatch: loop {
        match pc {
            0x830A43D8 => {
    //   block [0x830A43D8..0x830A43E0)
	// 830A43D8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A43DC: 8216F6B0  lwz r16, -0x950(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-2384 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A43E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A43E0 size=116
    let mut pc: u32 = 0x830A43E0;
    'dispatch: loop {
        match pc {
            0x830A43E0 => {
    //   block [0x830A43E0..0x830A4454)
	// 830A43E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A43E4: 48103D79  bl 0x831a815c
	ctx.lr = 0x830A43E8;
	sub_831A8130(ctx, base);
	// 830A43E8: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 830A43EC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A43F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A43F4: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 830A43F8: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 830A43FC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830A4400: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 830A4404: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4408: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 830A440C: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 830A4410: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 830A4414: 4BF33E85  bl 0x82fd8298
	ctx.lr = 0x830A4418;
	sub_82FD8298(ctx, base);
	// 830A4418: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A441C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A4420: 41820024  beq 0x830a4444
	if ctx.cr[0].eq {
	pc = 0x830A4444; continue 'dispatch;
	}
	// 830A4424: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 830A4428: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A442C: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 830A4430: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 830A4434: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A4438: 3880001A  li r4, 0x1a
	ctx.r[4].s64 = 26;
	// 830A443C: 48000E15  bl 0x830a5250
	ctx.lr = 0x830A4440;
	sub_830A5250(ctx, base);
	// 830A4440: 48000008  b 0x830a4448
	pc = 0x830A4448; continue 'dispatch;
	// 830A4444: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A4448: 9323000C  stw r25, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[25].u32 ) };
	// 830A444C: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 830A4450: 48103D5C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4454(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4454 size=48
    let mut pc: u32 = 0x830A4454;
    'dispatch: loop {
        match pc {
            0x830A4454 => {
    //   block [0x830A4454..0x830A4484)
	// 830A4454: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 830A4458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A445C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A4460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4464: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 830A4468: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A446C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A4470: 4BF33E71  bl 0x82fd82e0
	ctx.lr = 0x830A4474;
	sub_82FD82E0(ctx, base);
	// 830A4474: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A4478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A447C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A4480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A4488 size=8
    let mut pc: u32 = 0x830A4488;
    'dispatch: loop {
        match pc {
            0x830A4488 => {
    //   block [0x830A4488..0x830A4490)
	// 830A4488: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A448C: 8216F6E8  lwz r16, -0x918(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-2328 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4490 size=116
    let mut pc: u32 = 0x830A4490;
    'dispatch: loop {
        match pc {
            0x830A4490 => {
    //   block [0x830A4490..0x830A4504)
	// 830A4490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4494: 48103CD9  bl 0x831a816c
	ctx.lr = 0x830A4498;
	sub_831A8130(ctx, base);
	// 830A4498: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A449C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A44A0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A44A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830A44A8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830A44AC: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 830A44B0: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A44B4: 909D0004  stw r4, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 830A44B8: 4BF33DE1  bl 0x82fd8298
	ctx.lr = 0x830A44BC;
	sub_82FD8298(ctx, base);
	// 830A44BC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830A44C0: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 830A44C4: 4182002C  beq 0x830a44f0
	if ctx.cr[0].eq {
	pc = 0x830A44F0; continue 'dispatch;
	}
	// 830A44C8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830A44CC: 80DD0004  lwz r6, 4(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A44D0: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 830A44D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A44D8: 4BFA8319  bl 0x8304c7f0
	ctx.lr = 0x830A44DC;
	sub_8304C7F0(ctx, base);
	// 830A44DC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830A44E0: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830A44E4: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 830A44E8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A44EC: 48000008  b 0x830a44f4
	pc = 0x830A44F4; continue 'dispatch;
	// 830A44F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A44F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A44F8: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830A44FC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A4500: 48103CBC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4504(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4504 size=48
    let mut pc: u32 = 0x830A4504;
    'dispatch: loop {
        match pc {
            0x830A4504 => {
    //   block [0x830A4504..0x830A4534)
	// 830A4504: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A4508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A450C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A4510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4514: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A4518: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A451C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A4520: 4BF33DC1  bl 0x82fd82e0
	ctx.lr = 0x830A4524;
	sub_82FD82E0(ctx, base);
	// 830A4524: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A4528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A452C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A4530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A4538 size=8
    let mut pc: u32 = 0x830A4538;
    'dispatch: loop {
        match pc {
            0x830A4538 => {
    //   block [0x830A4538..0x830A4540)
	// 830A4538: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A453C: 8216F720  lwz r16, -0x8e0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-2272 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4540 size=96
    let mut pc: u32 = 0x830A4540;
    'dispatch: loop {
        match pc {
            0x830A4540 => {
    //   block [0x830A4540..0x830A45A0)
	// 830A4540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4544: 48103C29  bl 0x831a816c
	ctx.lr = 0x830A4548;
	sub_831A8130(ctx, base);
	// 830A4548: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A454C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4550: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A4554: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 830A4558: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A455C: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 830A4560: 4BF33D39  bl 0x82fd8298
	ctx.lr = 0x830A4564;
	sub_82FD8298(ctx, base);
	// 830A4564: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A4568: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A456C: 41820018  beq 0x830a4584
	if ctx.cr[0].eq {
	pc = 0x830A4584; continue 'dispatch;
	}
	// 830A4570: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A4574: 80BD0004  lwz r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4578: 48000829  bl 0x830a4da0
	ctx.lr = 0x830A457C;
	sub_830A4DA0(ctx, base);
	// 830A457C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A4580: 48000008  b 0x830a4588
	pc = 0x830A4588; continue 'dispatch;
	// 830A4584: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830A4588: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A458C: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A4590: 4BF96BC1  bl 0x8303b150
	ctx.lr = 0x830A4594;
	sub_8303B150(ctx, base);
	// 830A4594: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A4598: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A459C: 48103C20  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A45A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A45A0 size=48
    let mut pc: u32 = 0x830A45A0;
    'dispatch: loop {
        match pc {
            0x830A45A0 => {
    //   block [0x830A45A0..0x830A45D0)
	// 830A45A0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A45A4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A45A8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A45AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A45B0: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A45B4: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A45B8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A45BC: 4BF33D25  bl 0x82fd82e0
	ctx.lr = 0x830A45C0;
	sub_82FD82E0(ctx, base);
	// 830A45C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A45C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A45C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A45CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A45D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A45D0 size=8
    let mut pc: u32 = 0x830A45D0;
    'dispatch: loop {
        match pc {
            0x830A45D0 => {
    //   block [0x830A45D0..0x830A45D8)
	// 830A45D0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A45D4: 8216F768  lwz r16, -0x898(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-2200 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A45D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A45D8 size=104
    let mut pc: u32 = 0x830A45D8;
    'dispatch: loop {
        match pc {
            0x830A45D8 => {
    //   block [0x830A45D8..0x830A4640)
	// 830A45D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A45DC: 48103B91  bl 0x831a816c
	ctx.lr = 0x830A45E0;
	sub_831A8130(ctx, base);
	// 830A45E0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A45E4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A45E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A45EC: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 830A45F0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830A45F4: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A45F8: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830A45FC: 4BF33C9D  bl 0x82fd8298
	ctx.lr = 0x830A4600;
	sub_82FD8298(ctx, base);
	// 830A4600: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A4604: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A4608: 4182001C  beq 0x830a4624
	if ctx.cr[0].eq {
	pc = 0x830A4624; continue 'dispatch;
	}
	// 830A460C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A4610: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4614: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830A4618: 48000A11  bl 0x830a5028
	ctx.lr = 0x830A461C;
	sub_830A5028(ctx, base);
	// 830A461C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A4620: 48000008  b 0x830a4628
	pc = 0x830A4628; continue 'dispatch;
	// 830A4624: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A4628: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A462C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A4630: 4BF96B21  bl 0x8303b150
	ctx.lr = 0x830A4634;
	sub_8303B150(ctx, base);
	// 830A4634: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A4638: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A463C: 48103B80  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4640 size=48
    let mut pc: u32 = 0x830A4640;
    'dispatch: loop {
        match pc {
            0x830A4640 => {
    //   block [0x830A4640..0x830A4670)
	// 830A4640: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A4644: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4648: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A464C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4650: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A4654: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4658: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A465C: 4BF33C85  bl 0x82fd82e0
	ctx.lr = 0x830A4660;
	sub_82FD82E0(ctx, base);
	// 830A4660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A4664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A4668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A466C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A4670 size=8
    let mut pc: u32 = 0x830A4670;
    'dispatch: loop {
        match pc {
            0x830A4670 => {
    //   block [0x830A4670..0x830A4678)
	// 830A4670: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A4674: 8216F7B0  lwz r16, -0x850(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-2128 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4678 size=104
    let mut pc: u32 = 0x830A4678;
    'dispatch: loop {
        match pc {
            0x830A4678 => {
    //   block [0x830A4678..0x830A46E0)
	// 830A4678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A467C: 48103AF1  bl 0x831a816c
	ctx.lr = 0x830A4680;
	sub_831A8130(ctx, base);
	// 830A4680: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A4684: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4688: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A468C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 830A4690: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830A4694: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4698: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830A469C: 4BF33BFD  bl 0x82fd8298
	ctx.lr = 0x830A46A0;
	sub_82FD8298(ctx, base);
	// 830A46A0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A46A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A46A8: 4182001C  beq 0x830a46c4
	if ctx.cr[0].eq {
	pc = 0x830A46C4; continue 'dispatch;
	}
	// 830A46AC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A46B0: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A46B4: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 830A46B8: 48000971  bl 0x830a5028
	ctx.lr = 0x830A46BC;
	sub_830A5028(ctx, base);
	// 830A46BC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A46C0: 48000008  b 0x830a46c8
	pc = 0x830A46C8; continue 'dispatch;
	// 830A46C4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A46C8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A46CC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A46D0: 4BF96A81  bl 0x8303b150
	ctx.lr = 0x830A46D4;
	sub_8303B150(ctx, base);
	// 830A46D4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A46D8: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A46DC: 48103AE0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A46E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A46E0 size=48
    let mut pc: u32 = 0x830A46E0;
    'dispatch: loop {
        match pc {
            0x830A46E0 => {
    //   block [0x830A46E0..0x830A4710)
	// 830A46E0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A46E4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A46E8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A46EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A46F0: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A46F4: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A46F8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A46FC: 4BF33BE5  bl 0x82fd82e0
	ctx.lr = 0x830A4700;
	sub_82FD82E0(ctx, base);
	// 830A4700: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A4704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A4708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A470C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A4710 size=8
    let mut pc: u32 = 0x830A4710;
    'dispatch: loop {
        match pc {
            0x830A4710 => {
    //   block [0x830A4710..0x830A4718)
	// 830A4710: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A4714: 8216F7F8  lwz r16, -0x808(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-2056 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4718 size=112
    let mut pc: u32 = 0x830A4718;
    'dispatch: loop {
        match pc {
            0x830A4718 => {
    //   block [0x830A4718..0x830A4788)
	// 830A4718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A471C: 48103A4D  bl 0x831a8168
	ctx.lr = 0x830A4720;
	sub_831A8130(ctx, base);
	// 830A4720: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A4724: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4728: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A472C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 830A4730: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A4734: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830A4738: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A473C: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 830A4740: 4BF33B59  bl 0x82fd8298
	ctx.lr = 0x830A4744;
	sub_82FD8298(ctx, base);
	// 830A4744: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A4748: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A474C: 4182001C  beq 0x830a4768
	if ctx.cr[0].eq {
	pc = 0x830A4768; continue 'dispatch;
	}
	// 830A4750: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830A4754: 80DD0004  lwz r6, 4(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4758: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 830A475C: 480008CD  bl 0x830a5028
	ctx.lr = 0x830A4760;
	sub_830A5028(ctx, base);
	// 830A4760: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A4764: 48000008  b 0x830a476c
	pc = 0x830A476C; continue 'dispatch;
	// 830A4768: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830A476C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A4770: 939E000C  stw r28, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 830A4774: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A4778: 4BF969D9  bl 0x8303b150
	ctx.lr = 0x830A477C;
	sub_8303B150(ctx, base);
	// 830A477C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A4780: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A4784: 48103A34  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4788 size=48
    let mut pc: u32 = 0x830A4788;
    'dispatch: loop {
        match pc {
            0x830A4788 => {
    //   block [0x830A4788..0x830A47B8)
	// 830A4788: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A478C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4790: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A4794: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4798: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A479C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A47A0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A47A4: 4BF33B3D  bl 0x82fd82e0
	ctx.lr = 0x830A47A8;
	sub_82FD82E0(ctx, base);
	// 830A47A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A47AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A47B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A47B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A47B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A47B8 size=8
    let mut pc: u32 = 0x830A47B8;
    'dispatch: loop {
        match pc {
            0x830A47B8 => {
    //   block [0x830A47B8..0x830A47C0)
	// 830A47B8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A47BC: 8216F840  lwz r16, -0x7c0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-1984 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A47C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A47C0 size=104
    let mut pc: u32 = 0x830A47C0;
    'dispatch: loop {
        match pc {
            0x830A47C0 => {
    //   block [0x830A47C0..0x830A4828)
	// 830A47C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A47C4: 481039A9  bl 0x831a816c
	ctx.lr = 0x830A47C8;
	sub_831A8130(ctx, base);
	// 830A47C8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A47CC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A47D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A47D4: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 830A47D8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830A47DC: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A47E0: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830A47E4: 4BF33AB5  bl 0x82fd8298
	ctx.lr = 0x830A47E8;
	sub_82FD8298(ctx, base);
	// 830A47E8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A47EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A47F0: 4182001C  beq 0x830a480c
	if ctx.cr[0].eq {
	pc = 0x830A480C; continue 'dispatch;
	}
	// 830A47F4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A47F8: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A47FC: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 830A4800: 48000AE9  bl 0x830a52e8
	ctx.lr = 0x830A4804;
	sub_830A52E8(ctx, base);
	// 830A4804: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A4808: 48000008  b 0x830a4810
	pc = 0x830A4810; continue 'dispatch;
	// 830A480C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A4810: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A4814: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A4818: 4BF96939  bl 0x8303b150
	ctx.lr = 0x830A481C;
	sub_8303B150(ctx, base);
	// 830A481C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A4820: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A4824: 48103998  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4828 size=48
    let mut pc: u32 = 0x830A4828;
    'dispatch: loop {
        match pc {
            0x830A4828 => {
    //   block [0x830A4828..0x830A4858)
	// 830A4828: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A482C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4830: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A4834: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4838: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A483C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4840: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A4844: 4BF33A9D  bl 0x82fd82e0
	ctx.lr = 0x830A4848;
	sub_82FD82E0(ctx, base);
	// 830A4848: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A484C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A4850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A4854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A4858 size=8
    let mut pc: u32 = 0x830A4858;
    'dispatch: loop {
        match pc {
            0x830A4858 => {
    //   block [0x830A4858..0x830A4860)
	// 830A4858: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A485C: 8216F888  lwz r16, -0x778(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-1912 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4860 size=108
    let mut pc: u32 = 0x830A4860;
    'dispatch: loop {
        match pc {
            0x830A4860 => {
    //   block [0x830A4860..0x830A48CC)
	// 830A4860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4864: 48103909  bl 0x831a816c
	ctx.lr = 0x830A4868;
	sub_831A8130(ctx, base);
	// 830A4868: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A486C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4870: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A4874: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 830A4878: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830A487C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4880: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830A4884: 4BF33A15  bl 0x82fd8298
	ctx.lr = 0x830A4888;
	sub_82FD8298(ctx, base);
	// 830A4888: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A488C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A4890: 41820020  beq 0x830a48b0
	if ctx.cr[0].eq {
	pc = 0x830A48B0; continue 'dispatch;
	}
	// 830A4894: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 830A4898: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A489C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A48A0: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 830A48A4: 480007D5  bl 0x830a5078
	ctx.lr = 0x830A48A8;
	sub_830A5078(ctx, base);
	// 830A48A8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A48AC: 48000008  b 0x830a48b4
	pc = 0x830A48B4; continue 'dispatch;
	// 830A48B0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A48B4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A48B8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A48BC: 4BF96895  bl 0x8303b150
	ctx.lr = 0x830A48C0;
	sub_8303B150(ctx, base);
	// 830A48C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A48C4: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A48C8: 481038F4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A48CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A48CC size=48
    let mut pc: u32 = 0x830A48CC;
    'dispatch: loop {
        match pc {
            0x830A48CC => {
    //   block [0x830A48CC..0x830A48FC)
	// 830A48CC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A48D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A48D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A48D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A48DC: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A48E0: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A48E4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A48E8: 4BF339F9  bl 0x82fd82e0
	ctx.lr = 0x830A48EC;
	sub_82FD82E0(ctx, base);
	// 830A48EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A48F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A48F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A48F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A4900 size=8
    let mut pc: u32 = 0x830A4900;
    'dispatch: loop {
        match pc {
            0x830A4900 => {
    //   block [0x830A4900..0x830A4908)
	// 830A4900: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A4904: 8216F8D0  lwz r16, -0x730(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-1840 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4908 size=96
    let mut pc: u32 = 0x830A4908;
    'dispatch: loop {
        match pc {
            0x830A4908 => {
    //   block [0x830A4908..0x830A4968)
	// 830A4908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A490C: 48103861  bl 0x831a816c
	ctx.lr = 0x830A4910;
	sub_831A8130(ctx, base);
	// 830A4910: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A4914: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4918: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A491C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 830A4920: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4924: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 830A4928: 4BF33971  bl 0x82fd8298
	ctx.lr = 0x830A492C;
	sub_82FD8298(ctx, base);
	// 830A492C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A4930: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A4934: 41820018  beq 0x830a494c
	if ctx.cr[0].eq {
	pc = 0x830A494C; continue 'dispatch;
	}
	// 830A4938: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 830A493C: 80BD0004  lwz r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4940: 48000711  bl 0x830a5050
	ctx.lr = 0x830A4944;
	sub_830A5050(ctx, base);
	// 830A4944: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A4948: 48000008  b 0x830a4950
	pc = 0x830A4950; continue 'dispatch;
	// 830A494C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830A4950: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A4954: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A4958: 4BF967F9  bl 0x8303b150
	ctx.lr = 0x830A495C;
	sub_8303B150(ctx, base);
	// 830A495C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A4960: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A4964: 48103858  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4968 size=48
    let mut pc: u32 = 0x830A4968;
    'dispatch: loop {
        match pc {
            0x830A4968 => {
    //   block [0x830A4968..0x830A4998)
	// 830A4968: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A496C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4970: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A4974: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4978: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A497C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4980: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A4984: 4BF3395D  bl 0x82fd82e0
	ctx.lr = 0x830A4988;
	sub_82FD82E0(ctx, base);
	// 830A4988: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A498C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A4990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A4994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A4998 size=8
    let mut pc: u32 = 0x830A4998;
    'dispatch: loop {
        match pc {
            0x830A4998 => {
    //   block [0x830A4998..0x830A49A0)
	// 830A4998: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A499C: 8216F918  lwz r16, -0x6e8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-1768 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A49A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A49A0 size=120
    let mut pc: u32 = 0x830A49A0;
    'dispatch: loop {
        match pc {
            0x830A49A0 => {
    //   block [0x830A49A0..0x830A4A18)
	// 830A49A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A49A4: 481037C9  bl 0x831a816c
	ctx.lr = 0x830A49A8;
	sub_831A8130(ctx, base);
	// 830A49A8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A49AC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A49B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A49B4: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 830A49B8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830A49BC: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A49C0: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830A49C4: 4BF338D5  bl 0x82fd8298
	ctx.lr = 0x830A49C8;
	sub_82FD8298(ctx, base);
	// 830A49C8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A49CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A49D0: 4182002C  beq 0x830a49fc
	if ctx.cr[0].eq {
	pc = 0x830A49FC; continue 'dispatch;
	}
	// 830A49D4: 57AB063E  clrlwi r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 830A49D8: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A49DC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830A49E0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830A49E4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 830A49E8: 396B0009  addi r11, r11, 9
	ctx.r[11].s64 = ctx.r[11].s64 + 9;
	// 830A49EC: 7D640734  extsh r4, r11
	ctx.r[4].s64 = ctx.r[11].s16 as i64;
	// 830A49F0: 48000661  bl 0x830a5050
	ctx.lr = 0x830A49F4;
	sub_830A5050(ctx, base);
	// 830A49F4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A49F8: 48000008  b 0x830a4a00
	pc = 0x830A4A00; continue 'dispatch;
	// 830A49FC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A4A00: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A4A04: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A4A08: 4BF96749  bl 0x8303b150
	ctx.lr = 0x830A4A0C;
	sub_8303B150(ctx, base);
	// 830A4A0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A4A10: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A4A14: 481037A8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4A18 size=48
    let mut pc: u32 = 0x830A4A18;
    'dispatch: loop {
        match pc {
            0x830A4A18 => {
    //   block [0x830A4A18..0x830A4A48)
	// 830A4A18: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A4A1C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4A20: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A4A24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4A28: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A4A2C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4A30: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A4A34: 4BF338AD  bl 0x82fd82e0
	ctx.lr = 0x830A4A38;
	sub_82FD82E0(ctx, base);
	// 830A4A38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A4A3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A4A40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A4A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A4A48 size=8
    let mut pc: u32 = 0x830A4A48;
    'dispatch: loop {
        match pc {
            0x830A4A48 => {
    //   block [0x830A4A48..0x830A4A50)
	// 830A4A48: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A4A4C: 8216F960  lwz r16, -0x6a0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-1696 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4A50 size=104
    let mut pc: u32 = 0x830A4A50;
    'dispatch: loop {
        match pc {
            0x830A4A50 => {
    //   block [0x830A4A50..0x830A4AB8)
	// 830A4A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4A54: 48103719  bl 0x831a816c
	ctx.lr = 0x830A4A58;
	sub_831A8130(ctx, base);
	// 830A4A58: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A4A5C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4A60: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A4A64: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 830A4A68: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830A4A6C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4A70: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830A4A74: 4BF33825  bl 0x82fd8298
	ctx.lr = 0x830A4A78;
	sub_82FD8298(ctx, base);
	// 830A4A78: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A4A7C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A4A80: 4182001C  beq 0x830a4a9c
	if ctx.cr[0].eq {
	pc = 0x830A4A9C; continue 'dispatch;
	}
	// 830A4A84: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A4A88: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4A8C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 830A4A90: 48000619  bl 0x830a50a8
	ctx.lr = 0x830A4A94;
	sub_830A50A8(ctx, base);
	// 830A4A94: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A4A98: 48000008  b 0x830a4aa0
	pc = 0x830A4AA0; continue 'dispatch;
	// 830A4A9C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A4AA0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A4AA4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A4AA8: 4BF966A9  bl 0x8303b150
	ctx.lr = 0x830A4AAC;
	sub_8303B150(ctx, base);
	// 830A4AAC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A4AB0: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A4AB4: 48103708  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4AB8 size=48
    let mut pc: u32 = 0x830A4AB8;
    'dispatch: loop {
        match pc {
            0x830A4AB8 => {
    //   block [0x830A4AB8..0x830A4AE8)
	// 830A4AB8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A4ABC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4AC0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A4AC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4AC8: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A4ACC: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4AD0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A4AD4: 4BF3380D  bl 0x82fd82e0
	ctx.lr = 0x830A4AD8;
	sub_82FD82E0(ctx, base);
	// 830A4AD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A4ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A4AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A4AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A4AE8 size=8
    let mut pc: u32 = 0x830A4AE8;
    'dispatch: loop {
        match pc {
            0x830A4AE8 => {
    //   block [0x830A4AE8..0x830A4AF0)
	// 830A4AE8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A4AEC: 8216F9A8  lwz r16, -0x658(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-1624 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4AF0 size=124
    let mut pc: u32 = 0x830A4AF0;
    'dispatch: loop {
        match pc {
            0x830A4AF0 => {
    //   block [0x830A4AF0..0x830A4B6C)
	// 830A4AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4AF4: 48103671  bl 0x831a8164
	ctx.lr = 0x830A4AF8;
	sub_831A8130(ctx, base);
	// 830A4AF8: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830A4AFC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4B00: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A4B04: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 830A4B08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A4B0C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830A4B10: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 830A4B14: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4B18: 93BF00A4  stw r29, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[29].u32 ) };
	// 830A4B1C: 4BF3377D  bl 0x82fd8298
	ctx.lr = 0x830A4B20;
	sub_82FD8298(ctx, base);
	// 830A4B20: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A4B24: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A4B28: 41820018  beq 0x830a4b40
	if ctx.cr[0].eq {
	pc = 0x830A4B40; continue 'dispatch;
	}
	// 830A4B2C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A4B30: 80BD0004  lwz r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4B34: 4800051D  bl 0x830a5050
	ctx.lr = 0x830A4B38;
	sub_830A5050(ctx, base);
	// 830A4B38: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A4B3C: 48000008  b 0x830a4b44
	pc = 0x830A4B44; continue 'dispatch;
	// 830A4B40: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830A4B44: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830A4B48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A4B4C: 939E000C  stw r28, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 830A4B50: 4BD61B01  bl 0x82e06650
	ctx.lr = 0x830A4B54;
	sub_82E06650(ctx, base);
	// 830A4B54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A4B58: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A4B5C: 4BF965F5  bl 0x8303b150
	ctx.lr = 0x830A4B60;
	sub_8303B150(ctx, base);
	// 830A4B60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A4B64: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 830A4B68: 4810364C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4B6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4B6C size=48
    let mut pc: u32 = 0x830A4B6C;
    'dispatch: loop {
        match pc {
            0x830A4B6C => {
    //   block [0x830A4B6C..0x830A4B9C)
	// 830A4B6C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830A4B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A4B78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4B7C: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830A4B80: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4B84: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A4B88: 4BF33759  bl 0x82fd82e0
	ctx.lr = 0x830A4B8C;
	sub_82FD82E0(ctx, base);
	// 830A4B8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A4B90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A4B94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A4B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A4BA0 size=8
    let mut pc: u32 = 0x830A4BA0;
    'dispatch: loop {
        match pc {
            0x830A4BA0 => {
    //   block [0x830A4BA0..0x830A4BA8)
	// 830A4BA0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A4BA4: 8216F9F0  lwz r16, -0x610(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-1552 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4BA8 size=104
    let mut pc: u32 = 0x830A4BA8;
    'dispatch: loop {
        match pc {
            0x830A4BA8 => {
    //   block [0x830A4BA8..0x830A4C10)
	// 830A4BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4BAC: 481035C1  bl 0x831a816c
	ctx.lr = 0x830A4BB0;
	sub_831A8130(ctx, base);
	// 830A4BB0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A4BB4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4BB8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A4BBC: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 830A4BC0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830A4BC4: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4BC8: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830A4BCC: 4BF336CD  bl 0x82fd8298
	ctx.lr = 0x830A4BD0;
	sub_82FD8298(ctx, base);
	// 830A4BD0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A4BD4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A4BD8: 4182001C  beq 0x830a4bf4
	if ctx.cr[0].eq {
	pc = 0x830A4BF4; continue 'dispatch;
	}
	// 830A4BDC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A4BE0: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4BE4: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 830A4BE8: 48000441  bl 0x830a5028
	ctx.lr = 0x830A4BEC;
	sub_830A5028(ctx, base);
	// 830A4BEC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A4BF0: 48000008  b 0x830a4bf8
	pc = 0x830A4BF8; continue 'dispatch;
	// 830A4BF4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A4BF8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A4BFC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A4C00: 4BF96551  bl 0x8303b150
	ctx.lr = 0x830A4C04;
	sub_8303B150(ctx, base);
	// 830A4C04: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A4C08: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A4C0C: 481035B0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4C10 size=48
    let mut pc: u32 = 0x830A4C10;
    'dispatch: loop {
        match pc {
            0x830A4C10 => {
    //   block [0x830A4C10..0x830A4C40)
	// 830A4C10: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A4C14: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4C18: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A4C1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4C20: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A4C24: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4C28: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A4C2C: 4BF336B5  bl 0x82fd82e0
	ctx.lr = 0x830A4C30;
	sub_82FD82E0(ctx, base);
	// 830A4C30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A4C34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A4C38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A4C3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A4C40 size=8
    let mut pc: u32 = 0x830A4C40;
    'dispatch: loop {
        match pc {
            0x830A4C40 => {
    //   block [0x830A4C40..0x830A4C48)
	// 830A4C40: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A4C44: 8216FA38  lwz r16, -0x5c8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-1480 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4C48 size=104
    let mut pc: u32 = 0x830A4C48;
    'dispatch: loop {
        match pc {
            0x830A4C48 => {
    //   block [0x830A4C48..0x830A4CB0)
	// 830A4C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4C4C: 48103521  bl 0x831a816c
	ctx.lr = 0x830A4C50;
	sub_831A8130(ctx, base);
	// 830A4C50: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A4C54: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4C58: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A4C5C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 830A4C60: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830A4C64: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4C68: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830A4C6C: 4BF3362D  bl 0x82fd8298
	ctx.lr = 0x830A4C70;
	sub_82FD8298(ctx, base);
	// 830A4C70: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A4C74: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A4C78: 4182001C  beq 0x830a4c94
	if ctx.cr[0].eq {
	pc = 0x830A4C94; continue 'dispatch;
	}
	// 830A4C7C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A4C80: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4C84: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 830A4C88: 48000451  bl 0x830a50d8
	ctx.lr = 0x830A4C8C;
	sub_830A50D8(ctx, base);
	// 830A4C8C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A4C90: 48000008  b 0x830a4c98
	pc = 0x830A4C98; continue 'dispatch;
	// 830A4C94: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A4C98: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A4C9C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A4CA0: 4BF964B1  bl 0x8303b150
	ctx.lr = 0x830A4CA4;
	sub_8303B150(ctx, base);
	// 830A4CA4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A4CA8: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A4CAC: 48103510  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4CB0 size=48
    let mut pc: u32 = 0x830A4CB0;
    'dispatch: loop {
        match pc {
            0x830A4CB0 => {
    //   block [0x830A4CB0..0x830A4CE0)
	// 830A4CB0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A4CB4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4CB8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A4CBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4CC0: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A4CC4: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4CC8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A4CCC: 4BF33615  bl 0x82fd82e0
	ctx.lr = 0x830A4CD0;
	sub_82FD82E0(ctx, base);
	// 830A4CD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A4CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A4CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A4CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A4CE0 size=8
    let mut pc: u32 = 0x830A4CE0;
    'dispatch: loop {
        match pc {
            0x830A4CE0 => {
    //   block [0x830A4CE0..0x830A4CE8)
	// 830A4CE0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A4CE4: 8216FA80  lwz r16, -0x580(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-1408 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4CE8 size=120
    let mut pc: u32 = 0x830A4CE8;
    'dispatch: loop {
        match pc {
            0x830A4CE8 => {
    //   block [0x830A4CE8..0x830A4D60)
	// 830A4CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4CEC: 48103479  bl 0x831a8164
	ctx.lr = 0x830A4CF0;
	sub_831A8130(ctx, base);
	// 830A4CF0: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830A4CF4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4CF8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A4CFC: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 830A4D00: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830A4D04: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830A4D08: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4D0C: 93BF00A4  stw r29, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[29].u32 ) };
	// 830A4D10: 4BF33589  bl 0x82fd8298
	ctx.lr = 0x830A4D14;
	sub_82FD8298(ctx, base);
	// 830A4D14: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A4D18: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A4D1C: 41820018  beq 0x830a4d34
	if ctx.cr[0].eq {
	pc = 0x830A4D34; continue 'dispatch;
	}
	// 830A4D20: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	// 830A4D24: 80BD0004  lwz r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4D28: 48000329  bl 0x830a5050
	ctx.lr = 0x830A4D2C;
	sub_830A5050(ctx, base);
	// 830A4D2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A4D30: 48000008  b 0x830a4d38
	pc = 0x830A4D38; continue 'dispatch;
	// 830A4D34: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830A4D38: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830A4D3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A4D40: 939E000C  stw r28, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 830A4D44: 4BD6190D  bl 0x82e06650
	ctx.lr = 0x830A4D48;
	sub_82E06650(ctx, base);
	// 830A4D48: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A4D4C: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A4D50: 4BF96401  bl 0x8303b150
	ctx.lr = 0x830A4D54;
	sub_8303B150(ctx, base);
	// 830A4D54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A4D58: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 830A4D5C: 48103458  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4D60 size=48
    let mut pc: u32 = 0x830A4D60;
    'dispatch: loop {
        match pc {
            0x830A4D60 => {
    //   block [0x830A4D60..0x830A4D90)
	// 830A4D60: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830A4D64: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4D68: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A4D6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4D70: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830A4D74: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4D78: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A4D7C: 4BF33565  bl 0x82fd82e0
	ctx.lr = 0x830A4D80;
	sub_82FD82E0(ctx, base);
	// 830A4D80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A4D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A4D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A4D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A4D90 size=16
    let mut pc: u32 = 0x830A4D90;
    'dispatch: loop {
        match pc {
            0x830A4D90 => {
    //   block [0x830A4D90..0x830A4DA0)
	// 830A4D90: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A4D94: 396BFAC0  addi r11, r11, -0x540
	ctx.r[11].s64 = ctx.r[11].s64 + -1344;
	// 830A4D98: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A4D9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A4DA0 size=32
    let mut pc: u32 = 0x830A4DA0;
    'dispatch: loop {
        match pc {
            0x830A4DA0 => {
    //   block [0x830A4DA0..0x830A4DC0)
	// 830A4DA0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A4DA4: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 830A4DA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A4DAC: B0830008  sth r4, 8(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u16 ) };
	// 830A4DB0: 396BFAC0  addi r11, r11, -0x540
	ctx.r[11].s64 = ctx.r[11].s64 + -1344;
	// 830A4DB4: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830A4DB8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A4DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4DC0 size=56
    let mut pc: u32 = 0x830A4DC0;
    'dispatch: loop {
        match pc {
            0x830A4DC0 => {
    //   block [0x830A4DC0..0x830A4DF8)
	// 830A4DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A4DC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4DCC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A4DD0: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4DD4: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830A4DD8: 388BFAF0  addi r4, r11, -0x510
	ctx.r[4].s64 = ctx.r[11].s64 + -1296;
	// 830A4DDC: 38A0004B  li r5, 0x4b
	ctx.r[5].s64 = 75;
	// 830A4DE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A4DE4: 4BF2C275  bl 0x82fd1058
	ctx.lr = 0x830A4DE8;
	sub_82FD1058(ctx, base);
	// 830A4DE8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A4DEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A4DF0: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830A4DF4: 4810BE35  bl 0x831b0c28
	ctx.lr = 0x830A4DF8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4DF8 size=56
    let mut pc: u32 = 0x830A4DF8;
    'dispatch: loop {
        match pc {
            0x830A4DF8 => {
    //   block [0x830A4DF8..0x830A4E30)
	// 830A4DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A4E00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4E04: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A4E08: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4E0C: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830A4E10: 388BFAF0  addi r4, r11, -0x510
	ctx.r[4].s64 = ctx.r[11].s64 + -1296;
	// 830A4E14: 38A00051  li r5, 0x51
	ctx.r[5].s64 = 81;
	// 830A4E18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A4E1C: 4BF2C23D  bl 0x82fd1058
	ctx.lr = 0x830A4E20;
	sub_82FD1058(ctx, base);
	// 830A4E20: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A4E24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A4E28: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830A4E2C: 4810BDFD  bl 0x831b0c28
	ctx.lr = 0x830A4E30;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4E30 size=56
    let mut pc: u32 = 0x830A4E30;
    'dispatch: loop {
        match pc {
            0x830A4E30 => {
    //   block [0x830A4E30..0x830A4E68)
	// 830A4E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A4E38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4E3C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A4E40: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4E44: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830A4E48: 388BFAF0  addi r4, r11, -0x510
	ctx.r[4].s64 = ctx.r[11].s64 + -1296;
	// 830A4E4C: 38A00057  li r5, 0x57
	ctx.r[5].s64 = 87;
	// 830A4E50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A4E54: 4BF2C205  bl 0x82fd1058
	ctx.lr = 0x830A4E58;
	sub_82FD1058(ctx, base);
	// 830A4E58: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A4E5C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A4E60: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830A4E64: 4810BDC5  bl 0x831b0c28
	ctx.lr = 0x830A4E68;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4E68 size=56
    let mut pc: u32 = 0x830A4E68;
    'dispatch: loop {
        match pc {
            0x830A4E68 => {
    //   block [0x830A4E68..0x830A4EA0)
	// 830A4E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A4E70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4E74: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A4E78: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4E7C: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830A4E80: 388BFAF0  addi r4, r11, -0x510
	ctx.r[4].s64 = ctx.r[11].s64 + -1296;
	// 830A4E84: 38A0005D  li r5, 0x5d
	ctx.r[5].s64 = 93;
	// 830A4E88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A4E8C: 4BF2C1CD  bl 0x82fd1058
	ctx.lr = 0x830A4E90;
	sub_82FD1058(ctx, base);
	// 830A4E90: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A4E94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A4E98: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830A4E9C: 4810BD8D  bl 0x831b0c28
	ctx.lr = 0x830A4EA0;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4EA0 size=56
    let mut pc: u32 = 0x830A4EA0;
    'dispatch: loop {
        match pc {
            0x830A4EA0 => {
    //   block [0x830A4EA0..0x830A4ED8)
	// 830A4EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A4EA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4EAC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A4EB0: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4EB4: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830A4EB8: 388BFAF0  addi r4, r11, -0x510
	ctx.r[4].s64 = ctx.r[11].s64 + -1296;
	// 830A4EBC: 38A00063  li r5, 0x63
	ctx.r[5].s64 = 99;
	// 830A4EC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A4EC4: 4BF2C195  bl 0x82fd1058
	ctx.lr = 0x830A4EC8;
	sub_82FD1058(ctx, base);
	// 830A4EC8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A4ECC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A4ED0: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830A4ED4: 4810BD55  bl 0x831b0c28
	ctx.lr = 0x830A4ED8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4ED8 size=56
    let mut pc: u32 = 0x830A4ED8;
    'dispatch: loop {
        match pc {
            0x830A4ED8 => {
    //   block [0x830A4ED8..0x830A4F10)
	// 830A4ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A4EE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4EE4: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A4EE8: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4EEC: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830A4EF0: 388BFAF0  addi r4, r11, -0x510
	ctx.r[4].s64 = ctx.r[11].s64 + -1296;
	// 830A4EF4: 38A00069  li r5, 0x69
	ctx.r[5].s64 = 105;
	// 830A4EF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A4EFC: 4BF2C15D  bl 0x82fd1058
	ctx.lr = 0x830A4F00;
	sub_82FD1058(ctx, base);
	// 830A4F00: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A4F04: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A4F08: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830A4F0C: 4810BD1D  bl 0x831b0c28
	ctx.lr = 0x830A4F10;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4F10 size=56
    let mut pc: u32 = 0x830A4F10;
    'dispatch: loop {
        match pc {
            0x830A4F10 => {
    //   block [0x830A4F10..0x830A4F48)
	// 830A4F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A4F18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4F1C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A4F20: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4F24: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830A4F28: 388BFAF0  addi r4, r11, -0x510
	ctx.r[4].s64 = ctx.r[11].s64 + -1296;
	// 830A4F2C: 38A0006F  li r5, 0x6f
	ctx.r[5].s64 = 111;
	// 830A4F30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A4F34: 4BF2C125  bl 0x82fd1058
	ctx.lr = 0x830A4F38;
	sub_82FD1058(ctx, base);
	// 830A4F38: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A4F3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A4F40: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830A4F44: 4810BCE5  bl 0x831b0c28
	ctx.lr = 0x830A4F48;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4F48 size=56
    let mut pc: u32 = 0x830A4F48;
    'dispatch: loop {
        match pc {
            0x830A4F48 => {
    //   block [0x830A4F48..0x830A4F80)
	// 830A4F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A4F50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4F54: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A4F58: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4F5C: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830A4F60: 388BFAF0  addi r4, r11, -0x510
	ctx.r[4].s64 = ctx.r[11].s64 + -1296;
	// 830A4F64: 38A00075  li r5, 0x75
	ctx.r[5].s64 = 117;
	// 830A4F68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A4F6C: 4BF2C0ED  bl 0x82fd1058
	ctx.lr = 0x830A4F70;
	sub_82FD1058(ctx, base);
	// 830A4F70: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A4F74: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A4F78: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830A4F7C: 4810BCAD  bl 0x831b0c28
	ctx.lr = 0x830A4F80;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4F80 size=56
    let mut pc: u32 = 0x830A4F80;
    'dispatch: loop {
        match pc {
            0x830A4F80 => {
    //   block [0x830A4F80..0x830A4FB8)
	// 830A4F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A4F88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4F8C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A4F90: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4F94: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830A4F98: 388BFAF0  addi r4, r11, -0x510
	ctx.r[4].s64 = ctx.r[11].s64 + -1296;
	// 830A4F9C: 38A0007B  li r5, 0x7b
	ctx.r[5].s64 = 123;
	// 830A4FA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A4FA4: 4BF2C0B5  bl 0x82fd1058
	ctx.lr = 0x830A4FA8;
	sub_82FD1058(ctx, base);
	// 830A4FA8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A4FAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A4FB0: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830A4FB4: 4810BC75  bl 0x831b0c28
	ctx.lr = 0x830A4FB8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4FB8 size=56
    let mut pc: u32 = 0x830A4FB8;
    'dispatch: loop {
        match pc {
            0x830A4FB8 => {
    //   block [0x830A4FB8..0x830A4FF0)
	// 830A4FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A4FC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4FC4: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A4FC8: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A4FCC: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830A4FD0: 388BFAF0  addi r4, r11, -0x510
	ctx.r[4].s64 = ctx.r[11].s64 + -1296;
	// 830A4FD4: 38A00081  li r5, 0x81
	ctx.r[5].s64 = 129;
	// 830A4FD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A4FDC: 4BF2C07D  bl 0x82fd1058
	ctx.lr = 0x830A4FE0;
	sub_82FD1058(ctx, base);
	// 830A4FE0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A4FE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A4FE8: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830A4FEC: 4810BC3D  bl 0x831b0c28
	ctx.lr = 0x830A4FF0;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A4FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A4FF0 size=56
    let mut pc: u32 = 0x830A4FF0;
    'dispatch: loop {
        match pc {
            0x830A4FF0 => {
    //   block [0x830A4FF0..0x830A5028)
	// 830A4FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A4FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A4FF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A4FFC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A5000: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A5004: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830A5008: 388BFAF0  addi r4, r11, -0x510
	ctx.r[4].s64 = ctx.r[11].s64 + -1296;
	// 830A500C: 38A00087  li r5, 0x87
	ctx.r[5].s64 = 135;
	// 830A5010: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A5014: 4BF2C045  bl 0x82fd1058
	ctx.lr = 0x830A5018;
	sub_82FD1058(ctx, base);
	// 830A5018: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A501C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A5020: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830A5024: 4810BC05  bl 0x831b0c28
	ctx.lr = 0x830A5028;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A5028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A5028 size=36
    let mut pc: u32 = 0x830A5028;
    'dispatch: loop {
        match pc {
            0x830A5028 => {
    //   block [0x830A5028..0x830A504C)
	// 830A5028: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A502C: 90C30004  stw r6, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 830A5030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A5034: B0830008  sth r4, 8(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u16 ) };
	// 830A5038: 396BFB1C  addi r11, r11, -0x4e4
	ctx.r[11].s64 = ctx.r[11].s64 + -1252;
	// 830A503C: 90A30010  stw r5, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 830A5040: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830A5044: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A5048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A5050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A5050 size=36
    let mut pc: u32 = 0x830A5050;
    'dispatch: loop {
        match pc {
            0x830A5050 => {
    //   block [0x830A5050..0x830A5074)
	// 830A5050: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A5054: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 830A5058: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A505C: B0830008  sth r4, 8(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u16 ) };
	// 830A5060: 396BFB4C  addi r11, r11, -0x4b4
	ctx.r[11].s64 = ctx.r[11].s64 + -1204;
	// 830A5064: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830A5068: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A506C: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 830A5070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A5078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A5078 size=44
    let mut pc: u32 = 0x830A5078;
    'dispatch: loop {
        match pc {
            0x830A5078 => {
    //   block [0x830A5078..0x830A50A4)
	// 830A5078: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A507C: 90E30004  stw r7, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 830A5080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A5084: B0830008  sth r4, 8(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u16 ) };
	// 830A5088: 396BFB7C  addi r11, r11, -0x484
	ctx.r[11].s64 = ctx.r[11].s64 + -1156;
	// 830A508C: 90A30014  stw r5, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 830A5090: 90C30018  stw r6, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[6].u32 ) };
	// 830A5094: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830A5098: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 830A509C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A50A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A50A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A50A8 size=36
    let mut pc: u32 = 0x830A50A8;
    'dispatch: loop {
        match pc {
            0x830A50A8 => {
    //   block [0x830A50A8..0x830A50CC)
	// 830A50A8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A50AC: 90C30004  stw r6, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 830A50B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A50B4: B0830008  sth r4, 8(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u16 ) };
	// 830A50B8: 396BFBAC  addi r11, r11, -0x454
	ctx.r[11].s64 = ctx.r[11].s64 + -1108;
	// 830A50BC: 90A30010  stw r5, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 830A50C0: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830A50C4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A50C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A50D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A50D0 size=8
    let mut pc: u32 = 0x830A50D0;
    'dispatch: loop {
        match pc {
            0x830A50D0 => {
    //   block [0x830A50D0..0x830A50D8)
	// 830A50D0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A50D4: 8216FC18  lwz r16, -0x3e8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-1000 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A50D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A50D8 size=104
    let mut pc: u32 = 0x830A50D8;
    'dispatch: loop {
        match pc {
            0x830A50D8 => {
    //   block [0x830A50D8..0x830A5140)
	// 830A50D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A50DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A50E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A50E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A50E8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830A50EC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A50F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A50F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830A50F8: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830A50FC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830A5100: 90DE0004  stw r6, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 830A5104: B09E0008  sth r4, 8(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[4].u16 ) };
	// 830A5108: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830A510C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A5110: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 830A5114: 396BFBDC  addi r11, r11, -0x424
	ctx.r[11].s64 = ctx.r[11].s64 + -1060;
	// 830A5118: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A511C: 4BF2BA65  bl 0x82fd0b80
	ctx.lr = 0x830A5120;
	sub_82FD0B80(ctx, base);
	// 830A5120: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 830A5124: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A5128: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830A512C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A5130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A5134: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A5138: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A513C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A5140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A5140 size=40
    let mut pc: u32 = 0x830A5140;
    'dispatch: loop {
        match pc {
            0x830A5140 => {
    //   block [0x830A5140..0x830A5168)
	// 830A5140: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830A5144: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A5148: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A514C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A5150: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830A5154: 4BFFFC3D  bl 0x830a4d90
	ctx.lr = 0x830A5158;
	sub_830A4D90(ctx, base);
	// 830A5158: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A515C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A5160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A5164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A5168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A5168 size=8
    let mut pc: u32 = 0x830A5168;
    'dispatch: loop {
        match pc {
            0x830A5168 => {
    //   block [0x830A5168..0x830A5170)
	// 830A5168: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A516C: 8216FC50  lwz r16, -0x3b0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-944 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A5170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A5170 size=104
    let mut pc: u32 = 0x830A5170;
    'dispatch: loop {
        match pc {
            0x830A5170 => {
    //   block [0x830A5170..0x830A51D8)
	// 830A5170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A5174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A5178: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A517C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A5180: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830A5184: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A5188: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A518C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A5190: 396BFBDC  addi r11, r11, -0x424
	ctx.r[11].s64 = ctx.r[11].s64 + -1060;
	// 830A5194: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830A5198: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A519C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A51A0: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A51A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A51A8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A51AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A51B0: 4E800421  bctrl
	ctx.lr = 0x830A51B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A51B4: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A51B8: 396BFAC0  addi r11, r11, -0x540
	ctx.r[11].s64 = ctx.r[11].s64 + -1344;
	// 830A51BC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A51C0: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830A51C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A51C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A51CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A51D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A51D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A51D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A51D8 size=40
    let mut pc: u32 = 0x830A51D8;
    'dispatch: loop {
        match pc {
            0x830A51D8 => {
    //   block [0x830A51D8..0x830A5200)
	// 830A51D8: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830A51DC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A51E0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A51E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A51E8: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830A51EC: 4BFFFBA5  bl 0x830a4d90
	ctx.lr = 0x830A51F0;
	sub_830A4D90(ctx, base);
	// 830A51F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A51F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A51F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A51FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A5200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A5200 size=76
    let mut pc: u32 = 0x830A5200;
    'dispatch: loop {
        match pc {
            0x830A5200 => {
    //   block [0x830A5200..0x830A524C)
	// 830A5200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A5204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A5208: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A520C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A5210: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A5214: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A5218: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A521C: 4BFFFF55  bl 0x830a5170
	ctx.lr = 0x830A5220;
	sub_830A5170(ctx, base);
	// 830A5220: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A5224: 4182000C  beq 0x830a5230
	if ctx.cr[0].eq {
	pc = 0x830A5230; continue 'dispatch;
	}
	// 830A5228: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A522C: 4BF330B5  bl 0x82fd82e0
	ctx.lr = 0x830A5230;
	sub_82FD82E0(ctx, base);
	// 830A5230: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A5234: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A5238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A523C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A5240: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A5244: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A5248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A5250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A5250 size=48
    let mut pc: u32 = 0x830A5250;
    'dispatch: loop {
        match pc {
            0x830A5250 => {
    //   block [0x830A5250..0x830A5280)
	// 830A5250: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A5254: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 830A5258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A525C: B0830008  sth r4, 8(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u16 ) };
	// 830A5260: 396BFC80  addi r11, r11, -0x380
	ctx.r[11].s64 = ctx.r[11].s64 + -896;
	// 830A5264: 90A30010  stw r5, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 830A5268: 90C30014  stw r6, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 830A526C: 90E30018  stw r7, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[7].u32 ) };
	// 830A5270: 9103001C  stw r8, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[8].u32 ) };
	// 830A5274: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830A5278: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A527C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A5280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A5280 size=68
    let mut pc: u32 = 0x830A5280;
    'dispatch: loop {
        match pc {
            0x830A5280 => {
    //   block [0x830A5280..0x830A52C4)
	// 830A5280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A5284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A5288: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A528C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A5290: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A5294: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A5298: 396BFAC0  addi r11, r11, -0x540
	ctx.r[11].s64 = ctx.r[11].s64 + -1344;
	// 830A529C: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830A52A0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A52A4: 41820008  beq 0x830a52ac
	if ctx.cr[0].eq {
	pc = 0x830A52AC; continue 'dispatch;
	}
	// 830A52A8: 4BF33039  bl 0x82fd82e0
	ctx.lr = 0x830A52AC;
	sub_82FD82E0(ctx, base);
	// 830A52AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A52B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A52B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A52B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A52BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A52C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A52C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A52C8 size=12
    let mut pc: u32 = 0x830A52C8;
    'dispatch: loop {
        match pc {
            0x830A52C8 => {
    //   block [0x830A52C8..0x830A52D4)
	// 830A52C8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A52CC: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A52D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A52D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A52D8 size=8
    let mut pc: u32 = 0x830A52D8;
    'dispatch: loop {
        match pc {
            0x830A52D8 => {
    //   block [0x830A52D8..0x830A52E0)
	// 830A52D8: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A52DC: 4BF87594  b 0x8302c870
	sub_8302C870(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A52E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A52E0 size=8
    let mut pc: u32 = 0x830A52E0;
    'dispatch: loop {
        match pc {
            0x830A52E0 => {
    //   block [0x830A52E0..0x830A52E8)
	// 830A52E0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A52E4: 8216FCF0  lwz r16, -0x310(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-784 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A52E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A52E8 size=148
    let mut pc: u32 = 0x830A52E8;
    'dispatch: loop {
        match pc {
            0x830A52E8 => {
    //   block [0x830A52E8..0x830A537C)
	// 830A52E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A52EC: 48102E79  bl 0x831a8164
	ctx.lr = 0x830A52F0;
	sub_831A8130(ctx, base);
	// 830A52F0: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830A52F4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A52F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A52FC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 830A5300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830A5304: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830A5308: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 830A530C: 939F00BC  stw r28, 0xbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[28].u32 ) };
	// 830A5310: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 830A5314: B09E0008  sth r4, 8(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[4].u16 ) };
	// 830A5318: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830A531C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A5320: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830A5324: 396BFCB0  addi r11, r11, -0x350
	ctx.r[11].s64 = ctx.r[11].s64 + -848;
	// 830A5328: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830A532C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A5330: 4BF32F69  bl 0x82fd8298
	ctx.lr = 0x830A5334;
	sub_82FD8298(ctx, base);
	// 830A5334: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830A5338: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 830A533C: 4182002C  beq 0x830a5368
	if ctx.cr[0].eq {
	pc = 0x830A5368; continue 'dispatch;
	}
	// 830A5340: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 830A5344: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A5348: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830A534C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A5350: 4BFA74A1  bl 0x8304c7f0
	ctx.lr = 0x830A5354;
	sub_8304C7F0(ctx, base);
	// 830A5354: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830A5358: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 830A535C: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 830A5360: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A5364: 48000008  b 0x830a536c
	pc = 0x830A536C; continue 'dispatch;
	// 830A5368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A536C: 915E0010  stw r10, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 830A5370: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A5374: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 830A5378: 48102E3C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A537C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A537C size=40
    let mut pc: u32 = 0x830A537C;
    'dispatch: loop {
        match pc {
            0x830A537C => {
    //   block [0x830A537C..0x830A53A4)
	// 830A537C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830A5380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A5384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A5388: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A538C: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830A5390: 4BFFFA01  bl 0x830a4d90
	ctx.lr = 0x830A5394;
	sub_830A4D90(ctx, base);
	// 830A5394: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A5398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A539C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A53A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A53A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A53A4 size=44
    let mut pc: u32 = 0x830A53A4;
    'dispatch: loop {
        match pc {
            0x830A53A4 => {
    //   block [0x830A53A4..0x830A53D0)
	// 830A53A4: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830A53A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A53AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A53B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A53B4: 809F00BC  lwz r4, 0xbc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 830A53B8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A53BC: 4BF32F25  bl 0x82fd82e0
	ctx.lr = 0x830A53C0;
	sub_82FD82E0(ctx, base);
	// 830A53C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A53C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A53C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A53CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A53D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A53D0 size=8
    let mut pc: u32 = 0x830A53D0;
    'dispatch: loop {
        match pc {
            0x830A53D0 => {
    //   block [0x830A53D0..0x830A53D8)
	// 830A53D0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A53D4: 8216FD38  lwz r16, -0x2c8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-712 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A53D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A53D8 size=112
    let mut pc: u32 = 0x830A53D8;
    'dispatch: loop {
        match pc {
            0x830A53D8 => {
    //   block [0x830A53D8..0x830A5448)
	// 830A53D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A53DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A53E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A53E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A53E8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830A53EC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A53F0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A53F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A53F8: 396BFCB0  addi r11, r11, -0x350
	ctx.r[11].s64 = ctx.r[11].s64 + -848;
	// 830A53FC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830A5400: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A5404: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A5408: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A540C: 41820018  beq 0x830a5424
	if ctx.cr[0].eq {
	pc = 0x830A5424; continue 'dispatch;
	}
	// 830A5410: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5414: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830A5418: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A541C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A5420: 4E800421  bctrl
	ctx.lr = 0x830A5424;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5424: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A5428: 396BFAC0  addi r11, r11, -0x540
	ctx.r[11].s64 = ctx.r[11].s64 + -1344;
	// 830A542C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A5430: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830A5434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A5438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A543C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A5440: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A5444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A5448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A5448 size=40
    let mut pc: u32 = 0x830A5448;
    'dispatch: loop {
        match pc {
            0x830A5448 => {
    //   block [0x830A5448..0x830A5470)
	// 830A5448: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830A544C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A5450: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A5454: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A5458: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830A545C: 4BFFF935  bl 0x830a4d90
	ctx.lr = 0x830A5460;
	sub_830A4D90(ctx, base);
	// 830A5460: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A5464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A5468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A546C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A5470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A5470 size=76
    let mut pc: u32 = 0x830A5470;
    'dispatch: loop {
        match pc {
            0x830A5470 => {
    //   block [0x830A5470..0x830A54BC)
	// 830A5470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A5474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A5478: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A547C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A5480: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A5484: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A5488: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A548C: 4BFFFF4D  bl 0x830a53d8
	ctx.lr = 0x830A5490;
	sub_830A53D8(ctx, base);
	// 830A5490: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A5494: 4182000C  beq 0x830a54a0
	if ctx.cr[0].eq {
	pc = 0x830A54A0; continue 'dispatch;
	}
	// 830A5498: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A549C: 4BF32E45  bl 0x82fd82e0
	ctx.lr = 0x830A54A0;
	sub_82FD82E0(ctx, base);
	// 830A54A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A54A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A54A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A54AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A54B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A54B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A54B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A54C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A54C0 size=8
    let mut pc: u32 = 0x830A54C0;
    'dispatch: loop {
        match pc {
            0x830A54C0 => {
    //   block [0x830A54C0..0x830A54C8)
	// 830A54C0: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A54C4: 4BF95C8C  b 0x8303b150
	sub_8303B150(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A54C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A54C8 size=24
    let mut pc: u32 = 0x830A54C8;
    'dispatch: loop {
        match pc {
            0x830A54C8 => {
    //   block [0x830A54C8..0x830A54E0)
	// 830A54C8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A54CC: B0830004  sth r4, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u16 ) };
	// 830A54D0: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 830A54D4: 396BFDF8  addi r11, r11, -0x208
	ctx.r[11].s64 = ctx.r[11].s64 + -520;
	// 830A54D8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A54DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A54E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A54E0 size=56
    let mut pc: u32 = 0x830A54E0;
    'dispatch: loop {
        match pc {
            0x830A54E0 => {
    //   block [0x830A54E0..0x830A5518)
	// 830A54E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A54E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A54E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A54EC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A54F0: 80E30008  lwz r7, 8(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A54F4: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830A54F8: 388BFE40  addi r4, r11, -0x1c0
	ctx.r[4].s64 = ctx.r[11].s64 + -448;
	// 830A54FC: 38A000DE  li r5, 0xde
	ctx.r[5].s64 = 222;
	// 830A5500: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A5504: 4BF2BB55  bl 0x82fd1058
	ctx.lr = 0x830A5508;
	sub_82FD1058(ctx, base);
	// 830A5508: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A550C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A5510: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830A5514: 4810B715  bl 0x831b0c28
	ctx.lr = 0x830A5518;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A5518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A5518 size=56
    let mut pc: u32 = 0x830A5518;
    'dispatch: loop {
        match pc {
            0x830A5518 => {
    //   block [0x830A5518..0x830A5550)
	// 830A5518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A551C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A5520: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A5524: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A5528: 80E30008  lwz r7, 8(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A552C: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830A5530: 388BFE40  addi r4, r11, -0x1c0
	ctx.r[4].s64 = ctx.r[11].s64 + -448;
	// 830A5534: 38A000E6  li r5, 0xe6
	ctx.r[5].s64 = 230;
	// 830A5538: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A553C: 4BF2BB1D  bl 0x82fd1058
	ctx.lr = 0x830A5540;
	sub_82FD1058(ctx, base);
	// 830A5540: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A5544: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A5548: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830A554C: 4810B6DD  bl 0x831b0c28
	ctx.lr = 0x830A5550;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A5550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A5550 size=56
    let mut pc: u32 = 0x830A5550;
    'dispatch: loop {
        match pc {
            0x830A5550 => {
    //   block [0x830A5550..0x830A5588)
	// 830A5550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A5554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A5558: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A555C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A5560: 80E30008  lwz r7, 8(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A5564: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830A5568: 388BFE40  addi r4, r11, -0x1c0
	ctx.r[4].s64 = ctx.r[11].s64 + -448;
	// 830A556C: 38A000EB  li r5, 0xeb
	ctx.r[5].s64 = 235;
	// 830A5570: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A5574: 4BF2BAE5  bl 0x82fd1058
	ctx.lr = 0x830A5578;
	sub_82FD1058(ctx, base);
	// 830A5578: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A557C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A5580: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830A5584: 4810B6A5  bl 0x831b0c28
	ctx.lr = 0x830A5588;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A5588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A5588 size=56
    let mut pc: u32 = 0x830A5588;
    'dispatch: loop {
        match pc {
            0x830A5588 => {
    //   block [0x830A5588..0x830A55C0)
	// 830A5588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A558C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A5590: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A5594: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A5598: 80E30008  lwz r7, 8(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A559C: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830A55A0: 388BFE40  addi r4, r11, -0x1c0
	ctx.r[4].s64 = ctx.r[11].s64 + -448;
	// 830A55A4: 38A000F0  li r5, 0xf0
	ctx.r[5].s64 = 240;
	// 830A55A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A55AC: 4BF2BAAD  bl 0x82fd1058
	ctx.lr = 0x830A55B0;
	sub_82FD1058(ctx, base);
	// 830A55B0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A55B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A55B8: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830A55BC: 4810B66D  bl 0x831b0c28
	ctx.lr = 0x830A55C0;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A55C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A55C0 size=56
    let mut pc: u32 = 0x830A55C0;
    'dispatch: loop {
        match pc {
            0x830A55C0 => {
    //   block [0x830A55C0..0x830A55F8)
	// 830A55C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A55C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A55C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A55CC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A55D0: 80E30008  lwz r7, 8(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A55D4: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830A55D8: 388BFE40  addi r4, r11, -0x1c0
	ctx.r[4].s64 = ctx.r[11].s64 + -448;
	// 830A55DC: 38A000F5  li r5, 0xf5
	ctx.r[5].s64 = 245;
	// 830A55E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A55E4: 4BF2BA75  bl 0x82fd1058
	ctx.lr = 0x830A55E8;
	sub_82FD1058(ctx, base);
	// 830A55E8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A55EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A55F0: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830A55F4: 4810B635  bl 0x831b0c28
	ctx.lr = 0x830A55F8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A55F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A55F8 size=56
    let mut pc: u32 = 0x830A55F8;
    'dispatch: loop {
        match pc {
            0x830A55F8 => {
    //   block [0x830A55F8..0x830A5630)
	// 830A55F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A55FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A5600: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A5604: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A5608: 80E30008  lwz r7, 8(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A560C: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830A5610: 388BFE40  addi r4, r11, -0x1c0
	ctx.r[4].s64 = ctx.r[11].s64 + -448;
	// 830A5614: 38A000FA  li r5, 0xfa
	ctx.r[5].s64 = 250;
	// 830A5618: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A561C: 4BF2BA3D  bl 0x82fd1058
	ctx.lr = 0x830A5620;
	sub_82FD1058(ctx, base);
	// 830A5620: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A5624: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A5628: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830A562C: 4810B5FD  bl 0x831b0c28
	ctx.lr = 0x830A5630;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A5630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A5630 size=56
    let mut pc: u32 = 0x830A5630;
    'dispatch: loop {
        match pc {
            0x830A5630 => {
    //   block [0x830A5630..0x830A5668)
	// 830A5630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A5634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A5638: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A563C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A5640: 80E30008  lwz r7, 8(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A5644: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830A5648: 388BFE40  addi r4, r11, -0x1c0
	ctx.r[4].s64 = ctx.r[11].s64 + -448;
	// 830A564C: 38A000FF  li r5, 0xff
	ctx.r[5].s64 = 255;
	// 830A5650: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A5654: 4BF2BA05  bl 0x82fd1058
	ctx.lr = 0x830A5658;
	sub_82FD1058(ctx, base);
	// 830A5658: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A565C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A5660: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830A5664: 4810B5C5  bl 0x831b0c28
	ctx.lr = 0x830A5668;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A5668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A5668 size=16
    let mut pc: u32 = 0x830A5668;
    'dispatch: loop {
        match pc {
            0x830A5668 => {
    //   block [0x830A5668..0x830A5678)
	// 830A5668: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A566C: 396BFDF8  addi r11, r11, -0x208
	ctx.r[11].s64 = ctx.r[11].s64 + -520;
	// 830A5670: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A5674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A5678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A5678 size=504
    let mut pc: u32 = 0x830A5678;
    'dispatch: loop {
        match pc {
            0x830A5678 => {
    //   block [0x830A5678..0x830A5870)
	// 830A5678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A567C: 48102AED  bl 0x831a8168
	ctx.lr = 0x830A5680;
	sub_831A8130(ctx, base);
	// 830A5680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A5684: 48000044  b 0x830a56c8
	pc = 0x830A56C8; continue 'dispatch;
	// 830A5688: 3D808217  lis r12, -0x7de9
	ctx.r[12].s64 = -2112421888;
	// 830A568C: 398CFD78  addi r12, r12, -0x288
	ctx.r[12].s64 = ctx.r[12].s64 + -648;
	// 830A5690: 7C0C58AE  lbzx r0, r12, r11
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830A5694: 5400103A  slwi r0, r0, 2
	ctx.r[0].u32 = ctx.r[0].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 830A5698: 3D80830A  lis r12, -0x7cf6
	ctx.r[12].s64 = -2096496640;
	// 830A569C: 398C56B0  addi r12, r12, 0x56b0
	ctx.r[12].s64 = ctx.r[12].s64 + 22192;
	// 830A56A0: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 830A56A4: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 830A56A8: 60000000  nop
	// 830A56AC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	// 830A56B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A56B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A56B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A56BC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A56C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A56C4: 4E800421  bctrl
	ctx.lr = 0x830A56C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A56C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A56CC: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A56D0: 2B0B001A  cmplwi cr6, r11, 0x1a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 26 as u32, &mut ctx.xer);
	// 830A56D4: 4099FFB4  ble cr6, 0x830a5688
	if !ctx.cr[6].gt {
	pc = 0x830A5688; continue 'dispatch;
	}
	// 830A56D8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 830A56DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830A56E0: 48102AD8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 830A56E4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A56E8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A56EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A56F0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A56F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A56F8: 4E800421  bctrl
	ctx.lr = 0x830A56FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A56FC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830A5700: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830A5704: 41820030  beq 0x830a5734
	if ctx.cr[0].eq {
	pc = 0x830A5734; continue 'dispatch;
	}
	// 830A5708: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A570C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A5710: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A5714: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A5718: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A571C: 4E800421  bctrl
	ctx.lr = 0x830A5720;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5720: 4BFFFF59  bl 0x830a5678
	ctx.lr = 0x830A5724;
	sub_830A5678(ctx, base);
	// 830A5724: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 830A5728: 7FA3EA14  add r29, r3, r29
	ctx.r[29].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 830A572C: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 830A5730: 4198FFD8  blt cr6, 0x830a5708
	if ctx.cr[6].lt {
	pc = 0x830A5708; continue 'dispatch;
	}
	// 830A5734: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A5738: 4BFFFFA4  b 0x830a56dc
	pc = 0x830A56DC; continue 'dispatch;
	// 830A573C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5740: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A5744: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A5748: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A574C: 4E800421  bctrl
	ctx.lr = 0x830A5750;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5750: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830A5754: 4082000C  bne 0x830a5760
	if !ctx.cr[0].eq {
	pc = 0x830A5760; continue 'dispatch;
	}
	// 830A5758: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A575C: 4BFFFF80  b 0x830a56dc
	pc = 0x830A56DC; continue 'dispatch;
	// 830A5760: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5764: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A5768: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A576C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A5770: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A5774: 4E800421  bctrl
	ctx.lr = 0x830A5778;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5778: 4BFFFF01  bl 0x830a5678
	ctx.lr = 0x830A577C;
	sub_830A5678(ctx, base);
	// 830A577C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A5780: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 830A5784: 2B1C0001  cmplwi cr6, r28, 1
	ctx.cr[6].compare_u32(ctx.r[28].u32, 1 as u32, &mut ctx.xer);
	// 830A5788: 4099FFAC  ble cr6, 0x830a5734
	if !ctx.cr[6].gt {
	pc = 0x830A5734; continue 'dispatch;
	}
	// 830A578C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5790: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A5794: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A5798: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A579C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A57A0: 4E800421  bctrl
	ctx.lr = 0x830A57A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A57A4: 4BFFFED5  bl 0x830a5678
	ctx.lr = 0x830A57A8;
	sub_830A5678(ctx, base);
	// 830A57A8: 7F03E800  cmpw cr6, r3, r29
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[29].s32, &mut ctx.xer);
	// 830A57AC: 40980008  bge cr6, 0x830a57b4
	if !ctx.cr[6].lt {
	pc = 0x830A57B4; continue 'dispatch;
	}
	// 830A57B0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A57B4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 830A57B8: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 830A57BC: 4198FFD0  blt cr6, 0x830a578c
	if ctx.cr[6].lt {
	pc = 0x830A578C; continue 'dispatch;
	}
	// 830A57C0: 4BFFFF74  b 0x830a5734
	pc = 0x830A5734; continue 'dispatch;
	// 830A57C4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A57C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A57CC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830A57D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A57D4: 4E800421  bctrl
	ctx.lr = 0x830A57D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A57D8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830A57DC: 4180FF7C  blt 0x830a5758
	if ctx.cr[0].lt {
	pc = 0x830A5758; continue 'dispatch;
	}
	// 830A57E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A57E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A57E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A57EC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A57F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A57F4: 4E800421  bctrl
	ctx.lr = 0x830A57F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A57F8: 4BFFFE81  bl 0x830a5678
	ctx.lr = 0x830A57FC;
	sub_830A5678(ctx, base);
	// 830A57FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5800: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A5804: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A5808: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830A580C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A5810: 4E800421  bctrl
	ctx.lr = 0x830A5814;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5814: 7C7E19D6  mullw r3, r30, r3
	ctx.r[3].s64 = (ctx.r[30].s32 as i64) * (ctx.r[3].s32 as i64);
	// 830A5818: 4BFFFEC4  b 0x830a56dc
	pc = 0x830A56DC; continue 'dispatch;
	// 830A581C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830A5820: 4BFFFEBC  b 0x830a56dc
	pc = 0x830A56DC; continue 'dispatch;
	// 830A5824: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5828: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A582C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A5830: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A5834: 4E800421  bctrl
	ctx.lr = 0x830A5838;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5838: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A583C: 4182FF1C  beq 0x830a5758
	if ctx.cr[0].eq {
	pc = 0x830A5758; continue 'dispatch;
	}
	// 830A5840: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5844: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A5848: 4182FF10  beq 0x830a5758
	if ctx.cr[0].eq {
	pc = 0x830A5758; continue 'dispatch;
	}
	// 830A584C: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 830A5850: 48000008  b 0x830a5858
	pc = 0x830A5858; continue 'dispatch;
	// 830A5854: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830A5858: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A585C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A5860: 4082FFF4  bne 0x830a5854
	if !ctx.cr[0].eq {
	pc = 0x830A5854; continue 'dispatch;
	}
	// 830A5864: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 830A5868: 7D630E70  srawi r3, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830A586C: 4BFFFE70  b 0x830a56dc
	pc = 0x830A56DC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A5870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A5870 size=900
    let mut pc: u32 = 0x830A5870;
    'dispatch: loop {
        match pc {
            0x830A5870 => {
    //   block [0x830A5870..0x830A5BF4)
	// 830A5870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A5874: 481028E9  bl 0x831a815c
	ctx.lr = 0x830A5878;
	sub_831A8130(ctx, base);
	// 830A5878: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A587C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A5880: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A5884: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 830A5888: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 830A588C: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A5890: 2B0B001A  cmplwi cr6, r11, 0x1a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 26 as u32, &mut ctx.xer);
	// 830A5894: 419900A8  bgt cr6, 0x830a593c
	if ctx.cr[6].gt {
	pc = 0x830A593C; continue 'dispatch;
	}
	// 830A5898: 3F608217  lis r27, -0x7de9
	ctx.r[27].s64 = -2112421888;
	// 830A589C: 3D808217  lis r12, -0x7de9
	ctx.r[12].s64 = -2112421888;
	// 830A58A0: 398CFDB8  addi r12, r12, -0x248
	ctx.r[12].s64 = ctx.r[12].s64 + -584;
	// 830A58A4: 7C0C58AE  lbzx r0, r12, r11
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830A58A8: 5400103A  slwi r0, r0, 2
	ctx.r[0].u32 = ctx.r[0].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 830A58AC: 3D80830A  lis r12, -0x7cf6
	ctx.r[12].s64 = -2096496640;
	// 830A58B0: 398C58C4  addi r12, r12, 0x58c4
	ctx.r[12].s64 = ctx.r[12].s64 + 22724;
	// 830A58B4: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 830A58B8: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 830A58BC: 60000000  nop
	// 830A58C0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	// 830A58C4: 817BAC98  lwz r11, -0x5368(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-21352 as u32) ) } as u64;
	// 830A58C8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830A58CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A58D0: 7D6AC838  and r10, r11, r25
	ctx.r[10].u64 = ctx.r[11].u64 & ctx.r[25].u64;
	// 830A58D4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 830A58D8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830A58DC: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A58E0: 4182000C  beq 0x830a58ec
	if ctx.cr[0].eq {
	pc = 0x830A58EC; continue 'dispatch;
	}
	// 830A58E4: 480006CD  bl 0x830a5fb0
	ctx.lr = 0x830A58E8;
	sub_830A5FB0(ctx, base);
	// 830A58E8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830A58EC: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A58F0: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A58F4: 480010B5  bl 0x830a69a8
	ctx.lr = 0x830A58F8;
	sub_830A69A8(ctx, base);
	// 830A58F8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A58FC: 817D0030  lwz r11, 0x30(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A5900: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A5904: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A5908: 4E800421  bctrl
	ctx.lr = 0x830A590C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A590C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5910: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A5914: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A5918: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A591C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A5920: 4E800421  bctrl
	ctx.lr = 0x830A5924;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5924: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A5928: 41820270  beq 0x830a5b98
	if ctx.cr[0].eq {
	pc = 0x830A5B98; continue 'dispatch;
	}
	// 830A592C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A5930: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A5934: 2B0B001A  cmplwi cr6, r11, 0x1a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 26 as u32, &mut ctx.xer);
	// 830A5938: 4099FF64  ble cr6, 0x830a589c
	if !ctx.cr[6].gt {
	pc = 0x830A589C; continue 'dispatch;
	}
	// 830A593C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A5940: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830A5944: 48102868  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 830A5948: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A594C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A5950: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 830A5954: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A5958: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A595C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A5960: 4E800421  bctrl
	ctx.lr = 0x830A5964;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5964: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830A5968: 4081005C  ble 0x830a59c4
	if !ctx.cr[0].gt {
	pc = 0x830A59C4; continue 'dispatch;
	}
	// 830A596C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5970: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830A5974: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A5978: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A597C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A5980: 4E800421  bctrl
	ctx.lr = 0x830A5984;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5984: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A5988: 4182001C  beq 0x830a59a4
	if ctx.cr[0].eq {
	pc = 0x830A59A4; continue 'dispatch;
	}
	// 830A598C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 830A5990: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 830A5994: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A5998: 4BFFFED9  bl 0x830a5870
	ctx.lr = 0x830A599C;
	sub_830A5870(ctx, base);
	// 830A599C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830A59A0: 40820024  bne 0x830a59c4
	if !ctx.cr[0].eq {
	pc = 0x830A59C4; continue 'dispatch;
	}
	// 830A59A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A59A8: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 830A59AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A59B0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A59B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A59B8: 4E800421  bctrl
	ctx.lr = 0x830A59BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A59BC: 7F1B1800  cmpw cr6, r27, r3
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[3].s32, &mut ctx.xer);
	// 830A59C0: 4198FFAC  blt cr6, 0x830a596c
	if ctx.cr[6].lt {
	pc = 0x830A596C; continue 'dispatch;
	}
	// 830A59C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A59C8: 4BFFFF78  b 0x830a5940
	pc = 0x830A5940; continue 'dispatch;
	// 830A59CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A59D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A59D4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A59D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A59DC: 4E800421  bctrl
	ctx.lr = 0x830A59E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A59E0: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 830A59E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A59E8: 4182FF58  beq 0x830a5940
	if ctx.cr[0].eq {
	pc = 0x830A5940; continue 'dispatch;
	}
	// 830A59EC: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 830A59F0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A59F4: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 830A59F8: 419A0044  beq cr6, 0x830a5a3c
	if ctx.cr[6].eq {
	pc = 0x830A5A3C; continue 'dispatch;
	}
	// 830A59FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5A00: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A5A04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A5A08: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A5A0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A5A10: 4E800421  bctrl
	ctx.lr = 0x830A5A14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5A14: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A5A18: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 830A5A1C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 830A5A20: 4BFFFE51  bl 0x830a5870
	ctx.lr = 0x830A5A24;
	sub_830A5870(ctx, base);
	// 830A5A24: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 830A5A28: 419A0014  beq cr6, 0x830a5a3c
	if ctx.cr[6].eq {
	pc = 0x830A5A3C; continue 'dispatch;
	}
	// 830A5A2C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830A5A30: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 830A5A34: 7F1DD840  cmplw cr6, r29, r27
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[27].u32, &mut ctx.xer);
	// 830A5A38: 4198FFC4  blt cr6, 0x830a59fc
	if ctx.cr[6].lt {
	pc = 0x830A59FC; continue 'dispatch;
	}
	// 830A5A3C: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A5A40: 4182FF00  beq 0x830a5940
	if ctx.cr[0].eq {
	pc = 0x830A5940; continue 'dispatch;
	}
	// 830A5A44: 4BFFFEF8  b 0x830a593c
	pc = 0x830A593C; continue 'dispatch;
	// 830A5A48: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5A4C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A5A50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A5A54: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A5A58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A5A5C: 4E800421  bctrl
	ctx.lr = 0x830A5A60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5A60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A5A64: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 830A5A68: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 830A5A6C: 4BFFFE05  bl 0x830a5870
	ctx.lr = 0x830A5A70;
	sub_830A5870(ctx, base);
	// 830A5A70: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5A74: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A5A78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A5A7C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A5A80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A5A84: 4E800421  bctrl
	ctx.lr = 0x830A5A88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5A88: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 830A5A8C: 419AFEB0  beq cr6, 0x830a593c
	if ctx.cr[6].eq {
	pc = 0x830A593C; continue 'dispatch;
	}
	// 830A5A90: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 830A5A94: 419A0124  beq cr6, 0x830a5bb8
	if ctx.cr[6].eq {
	pc = 0x830A5BB8; continue 'dispatch;
	}
	// 830A5A98: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5A9C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830A5AA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A5AA4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A5AA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A5AAC: 4E800421  bctrl
	ctx.lr = 0x830A5AB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5AB0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A5AB4: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 830A5AB8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 830A5ABC: 4BFFFDB5  bl 0x830a5870
	ctx.lr = 0x830A5AC0;
	sub_830A5870(ctx, base);
	// 830A5AC0: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 830A5AC4: 419A00F4  beq cr6, 0x830a5bb8
	if ctx.cr[6].eq {
	pc = 0x830A5BB8; continue 'dispatch;
	}
	// 830A5AC8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830A5ACC: 419AFE70  beq cr6, 0x830a593c
	if ctx.cr[6].eq {
	pc = 0x830A593C; continue 'dispatch;
	}
	// 830A5AD0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 830A5AD4: 419AFE68  beq cr6, 0x830a593c
	if ctx.cr[6].eq {
	pc = 0x830A593C; continue 'dispatch;
	}
	// 830A5AD8: 48000068  b 0x830a5b40
	pc = 0x830A5B40; continue 'dispatch;
	// 830A5ADC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5AE0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A5AE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A5AE8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A5AEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A5AF0: 4E800421  bctrl
	ctx.lr = 0x830A5AF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5AF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A5AF8: 4182FE44  beq 0x830a593c
	if ctx.cr[0].eq {
	pc = 0x830A593C; continue 'dispatch;
	}
	// 830A5AFC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 830A5B00: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 830A5B04: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A5B08: 4BFFFD69  bl 0x830a5870
	ctx.lr = 0x830A5B0C;
	sub_830A5870(ctx, base);
	// 830A5B0C: 4BFFFE30  b 0x830a593c
	pc = 0x830A593C; continue 'dispatch;
	// 830A5B10: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5B14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A5B18: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A5B1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A5B20: 4E800421  bctrl
	ctx.lr = 0x830A5B24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5B24: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5B28: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830A5B2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A5B30: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 830A5B34: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A5B38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A5B3C: 4E800421  bctrl
	ctx.lr = 0x830A5B40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5B40: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830A5B44: 4BFFFDFC  b 0x830a5940
	pc = 0x830A5940; continue 'dispatch;
	// 830A5B48: 817BAC98  lwz r11, -0x5368(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-21352 as u32) ) } as u64;
	// 830A5B4C: 7D6AC838  and r10, r11, r25
	ctx.r[10].u64 = ctx.r[11].u64 & ctx.r[25].u64;
	// 830A5B50: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 830A5B54: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830A5B58: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A5B5C: 41820020  beq 0x830a5b7c
	if ctx.cr[0].eq {
	pc = 0x830A5B7C; continue 'dispatch;
	}
	// 830A5B60: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830A5B64: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5B68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A5B6C: 48000445  bl 0x830a5fb0
	ctx.lr = 0x830A5B70;
	sub_830A5FB0(ctx, base);
	// 830A5B70: 817D0030  lwz r11, 0x30(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A5B74: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A5B78: 48000010  b 0x830a5b88
	pc = 0x830A5B88; continue 'dispatch;
	// 830A5B7C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5B80: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830A5B84: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A5B88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A5B8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A5B90: 4E800421  bctrl
	ctx.lr = 0x830A5B94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5B94: 4BFFFFAC  b 0x830a5b40
	pc = 0x830A5B40; continue 'dispatch;
	// 830A5B98: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5B9C: 3CA00010  lis r5, 0x10
	ctx.r[5].s64 = 1048576;
	// 830A5BA0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A5BA4: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 830A5BA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A5BAC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A5BB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A5BB4: 4E800421  bctrl
	ctx.lr = 0x830A5BB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5BB8: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 830A5BBC: 4BFFFD84  b 0x830a5940
	pc = 0x830A5940; continue 'dispatch;
	// 830A5BC0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5BC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A5BC8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A5BCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A5BD0: 4E800421  bctrl
	ctx.lr = 0x830A5BD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5BD4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830A5BD8: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5BDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A5BE0: A0AB0000  lhz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5BE4: 814A002C  lwz r10, 0x2c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A5BE8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 830A5BEC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 830A5BF0: 4BFFFF4C  b 0x830a5b3c
	pc = 0x830A5B3C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A5BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A5BF8 size=264
    let mut pc: u32 = 0x830A5BF8;
    'dispatch: loop {
        match pc {
            0x830A5BF8 => {
    //   block [0x830A5BF8..0x830A5D00)
	// 830A5BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A5BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A5C00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A5C04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A5C08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A5C0C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A5C10: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830A5C14: 409A000C  bne cr6, 0x830a5c20
	if !ctx.cr[6].eq {
	pc = 0x830A5C20; continue 'dispatch;
	}
	// 830A5C18: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A5C1C: 480000CC  b 0x830a5ce8
	pc = 0x830A5CE8; continue 'dispatch;
	// 830A5C20: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A5C24: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 830A5C28: 419A0010  beq cr6, 0x830a5c38
	if ctx.cr[6].eq {
	pc = 0x830A5C38; continue 'dispatch;
	}
	// 830A5C2C: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A5C30: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 830A5C34: 409AFFE4  bne cr6, 0x830a5c18
	if !ctx.cr[6].eq {
	pc = 0x830A5C18; continue 'dispatch;
	}
	// 830A5C38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5C3C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A5C40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A5C44: 4E800421  bctrl
	ctx.lr = 0x830A5C48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5C48: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A5C4C: 41820034  beq 0x830a5c80
	if ctx.cr[0].eq {
	pc = 0x830A5C80; continue 'dispatch;
	}
	// 830A5C50: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5C54: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A5C58: 41820028  beq 0x830a5c80
	if ctx.cr[0].eq {
	pc = 0x830A5C80; continue 'dispatch;
	}
	// 830A5C5C: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 830A5C60: 48000008  b 0x830a5c68
	pc = 0x830A5C68; continue 'dispatch;
	// 830A5C64: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830A5C68: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5C6C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A5C70: 4082FFF4  bne 0x830a5c64
	if !ctx.cr[0].eq {
	pc = 0x830A5C64; continue 'dispatch;
	}
	// 830A5C74: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 830A5C78: 7D7F0E70  srawi r31, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830A5C7C: 48000008  b 0x830a5c84
	pc = 0x830A5C84; continue 'dispatch;
	// 830A5C80: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830A5C84: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5C88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A5C8C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A5C90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A5C94: 4E800421  bctrl
	ctx.lr = 0x830A5C98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5C98: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A5C9C: 41820034  beq 0x830a5cd0
	if ctx.cr[0].eq {
	pc = 0x830A5CD0; continue 'dispatch;
	}
	// 830A5CA0: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5CA4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A5CA8: 41820028  beq 0x830a5cd0
	if ctx.cr[0].eq {
	pc = 0x830A5CD0; continue 'dispatch;
	}
	// 830A5CAC: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 830A5CB0: 48000008  b 0x830a5cb8
	pc = 0x830A5CB8; continue 'dispatch;
	// 830A5CB4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830A5CB8: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5CBC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A5CC0: 4082FFF4  bne 0x830a5cb4
	if !ctx.cr[0].eq {
	pc = 0x830A5CB4; continue 'dispatch;
	}
	// 830A5CC4: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 830A5CC8: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830A5CCC: 48000008  b 0x830a5cd4
	pc = 0x830A5CD4; continue 'dispatch;
	// 830A5CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830A5CD4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830A5CD8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830A5CDC: 41980008  blt cr6, 0x830a5ce4
	if ctx.cr[6].lt {
	pc = 0x830A5CE4; continue 'dispatch;
	}
	// 830A5CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830A5CE4: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830A5CE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A5CEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A5CF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A5CF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A5CF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A5CFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A5D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A5D00 size=68
    let mut pc: u32 = 0x830A5D00;
    'dispatch: loop {
        match pc {
            0x830A5D00 => {
    //   block [0x830A5D00..0x830A5D44)
	// 830A5D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A5D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A5D08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A5D0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A5D10: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A5D14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A5D18: 396BFDF8  addi r11, r11, -0x208
	ctx.r[11].s64 = ctx.r[11].s64 + -520;
	// 830A5D1C: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830A5D20: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A5D24: 41820008  beq 0x830a5d2c
	if ctx.cr[0].eq {
	pc = 0x830A5D2C; continue 'dispatch;
	}
	// 830A5D28: 4BF325B9  bl 0x82fd82e0
	ctx.lr = 0x830A5D2C;
	sub_82FD82E0(ctx, base);
	// 830A5D2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A5D30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A5D34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A5D38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A5D3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A5D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A5D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A5D48 size=316
    let mut pc: u32 = 0x830A5D48;
    'dispatch: loop {
        match pc {
            0x830A5D48 => {
    //   block [0x830A5D48..0x830A5E84)
	// 830A5D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A5D4C: 48102411  bl 0x831a815c
	ctx.lr = 0x830A5D50;
	sub_831A8130(ctx, base);
	// 830A5D50: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A5D54: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 830A5D58: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830A5D5C: 48000060  b 0x830a5dbc
	pc = 0x830A5DBC; continue 'dispatch;
	// 830A5D60: 3D808217  lis r12, -0x7de9
	ctx.r[12].s64 = -2112421888;
	// 830A5D64: 398CFDD8  addi r12, r12, -0x228
	ctx.r[12].s64 = ctx.r[12].s64 + -552;
	// 830A5D68: 7C0C58AE  lbzx r0, r12, r11
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830A5D6C: 3D80830A  lis r12, -0x7cf6
	ctx.r[12].s64 = -2096496640;
	// 830A5D70: 398C5D88  addi r12, r12, 0x5d88
	ctx.r[12].s64 = ctx.r[12].s64 + 23944;
	// 830A5D74: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 830A5D78: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 830A5D7C: 60000000  nop
	// 830A5D80: 60000000  nop
	// 830A5D84: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	// 830A5D88: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5D8C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A5D90: 4800001C  b 0x830a5dac
	pc = 0x830A5DAC; continue 'dispatch;
	// 830A5D94: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830A5D98: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5D9C: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A5DA0: 7D6BCB78  or r11, r11, r25
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[25].u64;
	// 830A5DA4: 7D794878  andc r25, r11, r9
	ctx.r[25].u64 = ctx.r[11].u64 & !ctx.r[9].u64;
	// 830A5DA8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A5DAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A5DB0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A5DB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A5DB8: 4E800421  bctrl
	ctx.lr = 0x830A5DBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5DBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A5DC0: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A5DC4: 2B0B001A  cmplwi cr6, r11, 0x1a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 26 as u32, &mut ctx.xer);
	// 830A5DC8: 4099FF98  ble cr6, 0x830a5d60
	if !ctx.cr[6].gt {
	pc = 0x830A5D60; continue 'dispatch;
	}
	// 830A5DCC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A5DD0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830A5DD4: 481023D8  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 830A5DD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A5DDC: 933B0000  stw r25, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 830A5DE0: 4BFFFFF0  b 0x830a5dd0
	pc = 0x830A5DD0; continue 'dispatch;
	// 830A5DE4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5DE8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830A5DEC: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 830A5DF0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830A5DF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A5DF8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A5DFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A5E00: 4E800421  bctrl
	ctx.lr = 0x830A5E04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5E04: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830A5E08: 40810070  ble 0x830a5e78
	if !ctx.cr[0].gt {
	pc = 0x830A5E78; continue 'dispatch;
	}
	// 830A5E0C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5E10: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830A5E14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A5E18: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A5E1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A5E20: 4E800421  bctrl
	ctx.lr = 0x830A5E24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5E24: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830A5E28: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 830A5E2C: 4BFFFF1D  bl 0x830a5d48
	ctx.lr = 0x830A5E30;
	sub_830A5D48(ctx, base);
	// 830A5E30: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A5E34: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830A5E38: 419A0018  beq cr6, 0x830a5e50
	if ctx.cr[6].eq {
	pc = 0x830A5E50; continue 'dispatch;
	}
	// 830A5E3C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A5E40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A5E44: 4BFFFDB5  bl 0x830a5bf8
	ctx.lr = 0x830A5E48;
	sub_830A5BF8(ctx, base);
	// 830A5E48: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A5E4C: 4182000C  beq 0x830a5e58
	if ctx.cr[0].eq {
	pc = 0x830A5E58; continue 'dispatch;
	}
	// 830A5E50: 835B0000  lwz r26, 0(r27)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5E54: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 830A5E58: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5E5C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 830A5E60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A5E64: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A5E68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A5E6C: 4E800421  bctrl
	ctx.lr = 0x830A5E70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5E70: 7F1C1800  cmpw cr6, r28, r3
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[3].s32, &mut ctx.xer);
	// 830A5E74: 4198FF98  blt cr6, 0x830a5e0c
	if ctx.cr[6].lt {
	pc = 0x830A5E0C; continue 'dispatch;
	}
	// 830A5E78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A5E7C: 935B0000  stw r26, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 830A5E80: 4BFFFF50  b 0x830a5dd0
	pc = 0x830A5DD0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A5E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A5E88 size=116
    let mut pc: u32 = 0x830A5E88;
    'dispatch: loop {
        match pc {
            0x830A5E88 => {
    //   block [0x830A5E88..0x830A5EFC)
	// 830A5E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A5E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A5E90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A5E94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A5E98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A5E9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A5EA0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830A5EA4: 4BFFF625  bl 0x830a54c8
	ctx.lr = 0x830A5EA8;
	sub_830A54C8(ctx, base);
	// 830A5EA8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A5EAC: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 830A5EB0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 830A5EB4: 394BFE78  addi r10, r11, -0x188
	ctx.r[10].s64 = ctx.r[11].s64 + -392;
	// 830A5EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830A5EBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A5EC0: 913F0018  stw r9, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 830A5EC4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830A5EC8: 997F000C  stb r11, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 830A5ECC: 997F000D  stb r11, 0xd(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 830A5ED0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830A5ED4: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830A5ED8: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 830A5EDC: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 830A5EE0: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 830A5EE4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A5EE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A5EEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A5EF0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A5EF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A5EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A5F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A5F00 size=8
    let mut pc: u32 = 0x830A5F00;
    'dispatch: loop {
        match pc {
            0x830A5F00 => {
    //   block [0x830A5F00..0x830A5F08)
	// 830A5F00: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A5F04: 8216FEC8  lwz r16, -0x138(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-312 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A5F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A5F08 size=124
    let mut pc: u32 = 0x830A5F08;
    'dispatch: loop {
        match pc {
            0x830A5F08 => {
    //   block [0x830A5F08..0x830A5F84)
	// 830A5F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A5F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A5F10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A5F14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A5F18: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830A5F1C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A5F20: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A5F24: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A5F28: 396BFE78  addi r11, r11, -0x188
	ctx.r[11].s64 = ctx.r[11].s64 + -392;
	// 830A5F2C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830A5F30: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A5F34: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A5F38: 809E001C  lwz r4, 0x1c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A5F3C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5F40: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A5F44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A5F48: 4E800421  bctrl
	ctx.lr = 0x830A5F4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5F4C: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A5F50: 809E0020  lwz r4, 0x20(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A5F54: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A5F58: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A5F5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A5F60: 4E800421  bctrl
	ctx.lr = 0x830A5F64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A5F64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A5F68: 4BFFF701  bl 0x830a5668
	ctx.lr = 0x830A5F6C;
	sub_830A5668(ctx, base);
	// 830A5F6C: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830A5F70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A5F74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A5F78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A5F7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A5F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A5F84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A5F84 size=40
    let mut pc: u32 = 0x830A5F84;
    'dispatch: loop {
        match pc {
            0x830A5F84 => {
    //   block [0x830A5F84..0x830A5FAC)
	// 830A5F84: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830A5F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A5F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A5F90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A5F94: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830A5F98: 4BFFF6D1  bl 0x830a5668
	ctx.lr = 0x830A5F9C;
	sub_830A5668(ctx, base);
	// 830A5F9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A5FA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A5FA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A5FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A5FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A5FB0 size=124
    let mut pc: u32 = 0x830A5FB0;
    'dispatch: loop {
        match pc {
            0x830A5FB0 => {
    //   block [0x830A5FB0..0x830A602C)
	// 830A5FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A5FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A5FB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A5FBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A5FC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A5FC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A5FC8: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A5FCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A5FD0: 409A0040  bne cr6, 0x830a6010
	if !ctx.cr[6].eq {
	pc = 0x830A6010; continue 'dispatch;
	}
	// 830A5FD4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830A5FD8: 419A0038  beq cr6, 0x830a6010
	if ctx.cr[6].eq {
	pc = 0x830A6010; continue 'dispatch;
	}
	// 830A5FDC: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A5FE0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830A5FE4: 396BFFFB  addi r11, r11, -5
	ctx.r[11].s64 = ctx.r[11].s64 + -5;
	// 830A5FE8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830A5FEC: 5564DFFE  rlwinm r4, r11, 0x1b, 0x1f, 0x1f
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830A5FF0: 4BFFD781  bl 0x830a3770
	ctx.lr = 0x830A5FF4;
	sub_830A3770(ctx, base);
	// 830A5FF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A5FF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A5FFC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6000: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A6004: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A6008: 4E800421  bctrl
	ctx.lr = 0x830A600C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A600C: 93FE0024  stw r31, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[31].u32 ) };
	// 830A6010: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A6014: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A6018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A601C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A6020: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A6024: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A6028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A6030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A6030 size=128
    let mut pc: u32 = 0x830A6030;
    'dispatch: loop {
        match pc {
            0x830A6030 => {
    //   block [0x830A6030..0x830A60B0)
	// 830A6030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A6034: 48102135  bl 0x831a8168
	ctx.lr = 0x830A6038;
	sub_831A8130(ctx, base);
	// 830A6038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A603C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A6040: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830A6044: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830A6048: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A604C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A6050: 419A004C  beq cr6, 0x830a609c
	if ctx.cr[6].eq {
	pc = 0x830A609C; continue 'dispatch;
	}
	// 830A6054: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A6058: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830A605C: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A6060: 4182001C  beq 0x830a607c
	if ctx.cr[0].eq {
	pc = 0x830A607C; continue 'dispatch;
	}
	// 830A6064: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A6068: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A606C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A6070: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A6074: 4E800421  bctrl
	ctx.lr = 0x830A6078;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A6078: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 830A607C: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A6080: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 830A6084: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6088: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A608C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A6090: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A6094: 4E800421  bctrl
	ctx.lr = 0x830A6098;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A6098: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 830A609C: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 830A60A0: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 830A60A4: 939F0020  stw r28, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[28].u32 ) };
	// 830A60A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830A60AC: 4810210C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A60B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A60B0 size=204
    let mut pc: u32 = 0x830A60B0;
    'dispatch: loop {
        match pc {
            0x830A60B0 => {
    //   block [0x830A60B0..0x830A617C)
	// 830A60B0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 830A60B4: 8963000C  lbz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830A60B8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A60BC: 408200B8  bne 0x830a6174
	if !ctx.cr[0].eq {
	pc = 0x830A6174; continue 'dispatch;
	}
	// 830A60C0: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A60C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A60C8: 419A00AC  beq cr6, 0x830a6174
	if ctx.cr[6].eq {
	pc = 0x830A6174; continue 'dispatch;
	}
	// 830A60CC: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A60D0: 348BFFFC  addic. r4, r11, -4
	ctx.xer.ca = (ctx.r[11].u32 > (!(-4 as u32)));
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 830A60D4: 41800098  blt 0x830a616c
	if ctx.cr[0].lt {
	pc = 0x830A616C; continue 'dispatch;
	}
	// 830A60D8: 3BE40002  addi r31, r4, 2
	ctx.r[31].s64 = ctx.r[4].s64 + 2;
	// 830A60DC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 830A60E0: 41980080  blt cr6, 0x830a6160
	if ctx.cr[6].lt {
	pc = 0x830A6160; continue 'dispatch;
	}
	// 830A60E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A60E8: 57E5F87E  srwi r5, r31, 1
	ctx.r[5].u32 = ctx.r[31].u32.wrapping_shr(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 830A60EC: 81230020  lwz r9, 0x20(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A60F0: 396A0008  addi r11, r10, 8
	ctx.r[11].s64 = ctx.r[10].s64 + 8;
	// 830A60F4: 7CCA4A14  add r6, r10, r9
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 830A60F8: 7CEB482E  lwzx r7, r11, r9
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 830A60FC: 81060000  lwz r8, 0(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6100: 7F083800  cmpw cr6, r8, r7
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[7].s32, &mut ctx.xer);
	// 830A6104: 4199001C  bgt cr6, 0x830a6120
	if ctx.cr[6].gt {
	pc = 0x830A6120; continue 'dispatch;
	}
	// 830A6108: 409A004C  bne cr6, 0x830a6154
	if !ctx.cr[6].eq {
	pc = 0x830A6154; continue 'dispatch;
	}
	// 830A610C: 7CEB4A14  add r7, r11, r9
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 830A6110: 80C60004  lwz r6, 4(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A6114: 80E70004  lwz r7, 4(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A6118: 7F063800  cmpw cr6, r6, r7
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[7].s32, &mut ctx.xer);
	// 830A611C: 40990038  ble cr6, 0x830a6154
	if !ctx.cr[6].gt {
	pc = 0x830A6154; continue 'dispatch;
	}
	// 830A6120: 7CEB482E  lwzx r7, r11, r9
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 830A6124: 7D0B492E  stwx r8, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u32) };
	// 830A6128: 81230020  lwz r9, 0x20(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A612C: 7CEA492E  stwx r7, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[7].u32) };
	// 830A6130: 81230020  lwz r9, 0x20(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6134: 7D0B4A14  add r8, r11, r9
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 830A6138: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 830A613C: 80E90004  lwz r7, 4(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A6140: 81280004  lwz r9, 4(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A6144: 90E80004  stw r7, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 830A6148: 81030020  lwz r8, 0x20(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A614C: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 830A6150: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 830A6154: 34A5FFFF  addic. r5, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 830A6158: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 830A615C: 4082FF90  bne 0x830a60ec
	if !ctx.cr[0].eq {
	pc = 0x830A60EC; continue 'dispatch;
	}
	// 830A6160: 3484FFFE  addic. r4, r4, -2
	ctx.xer.ca = (ctx.r[4].u32 > (!(-2 as u32)));
	ctx.r[4].s64 = ctx.r[4].s64 + -2;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 830A6164: 3BFFFFFE  addi r31, r31, -2
	ctx.r[31].s64 = ctx.r[31].s64 + -2;
	// 830A6168: 4080FF74  bge 0x830a60dc
	if !ctx.cr[0].lt {
	pc = 0x830A60DC; continue 'dispatch;
	}
	// 830A616C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830A6170: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 830A6174: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 830A6178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A6180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A6180 size=260
    let mut pc: u32 = 0x830A6180;
    'dispatch: loop {
        match pc {
            0x830A6180 => {
    //   block [0x830A6180..0x830A6284)
	// 830A6180: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 830A6184: 8963000D  lbz r11, 0xd(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 830A6188: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A618C: 408200F0  bne 0x830a627c
	if !ctx.cr[0].eq {
	pc = 0x830A627C; continue 'dispatch;
	}
	// 830A6190: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6194: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A6198: 419A00E4  beq cr6, 0x830a627c
	if ctx.cr[6].eq {
	pc = 0x830A627C; continue 'dispatch;
	}
	// 830A619C: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A61A0: 2B0A0002  cmplwi cr6, r10, 2
	ctx.cr[6].compare_u32(ctx.r[10].u32, 2 as u32, &mut ctx.xer);
	// 830A61A4: 409900D8  ble cr6, 0x830a627c
	if !ctx.cr[6].gt {
	pc = 0x830A627C; continue 'dispatch;
	}
	// 830A61A8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830A61AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830A61B0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830A61B4: 419A00BC  beq cr6, 0x830a6270
	if ctx.cr[6].eq {
	pc = 0x830A6270; continue 'dispatch;
	}
	// 830A61B8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A61BC: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830A61C0: 419A0034  beq cr6, 0x830a61f4
	if ctx.cr[6].eq {
	pc = 0x830A61F4; continue 'dispatch;
	}
	// 830A61C4: 81430020  lwz r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A61C8: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830A61CC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830A61D0: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 830A61D4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830A61D8: 7D29502E  lwzx r9, r9, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830A61DC: 7D25512E  stwx r9, r5, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 830A61E0: 81430020  lwz r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A61E4: 7D28502E  lwzx r9, r8, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830A61E8: 7D455214  add r10, r5, r10
	ctx.r[10].u64 = ctx.r[5].u64 + ctx.r[10].u64;
	// 830A61EC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 830A61F0: 48000008  b 0x830a61f8
	pc = 0x830A61F8; continue 'dispatch;
	// 830A61F4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830A61F8: 81430020  lwz r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A61FC: 81230014  lwz r9, 0x14(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6200: 7D455214  add r10, r5, r10
	ctx.r[10].u64 = ctx.r[5].u64 + ctx.r[10].u64;
	// 830A6204: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830A6208: 808A0004  lwz r4, 4(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A620C: 40980050  bge cr6, 0x830a625c
	if !ctx.cr[6].lt {
	pc = 0x830A625C; continue 'dispatch;
	}
	// 830A6210: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 830A6214: 80C30020  lwz r6, 0x20(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6218: 39240001  addi r9, r4, 1
	ctx.r[9].s64 = ctx.r[4].s64 + 1;
	// 830A621C: 7D473214  add r10, r7, r6
	ctx.r[10].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 830A6220: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6224: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 830A6228: 41980034  blt cr6, 0x830a625c
	if ctx.cr[6].lt {
	pc = 0x830A625C; continue 'dispatch;
	}
	// 830A622C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A6230: 419A000C  beq cr6, 0x830a623c
	if ctx.cr[6].eq {
	pc = 0x830A623C; continue 'dispatch;
	}
	// 830A6234: 7F045000  cmpw cr6, r4, r10
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[10].s32, &mut ctx.xer);
	// 830A6238: 40980010  bge cr6, 0x830a6248
	if !ctx.cr[6].lt {
	pc = 0x830A6248; continue 'dispatch;
	}
	// 830A623C: 7D253214  add r9, r5, r6
	ctx.r[9].u64 = ctx.r[5].u64 + ctx.r[6].u64;
	// 830A6240: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 830A6244: 91490004  stw r10, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830A6248: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A624C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830A6250: 38E70008  addi r7, r7, 8
	ctx.r[7].s64 = ctx.r[7].s64 + 8;
	// 830A6254: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830A6258: 4198FFBC  blt cr6, 0x830a6214
	if ctx.cr[6].lt {
	pc = 0x830A6214; continue 'dispatch;
	}
	// 830A625C: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6260: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 830A6264: 38A50008  addi r5, r5, 8
	ctx.r[5].s64 = ctx.r[5].s64 + 8;
	// 830A6268: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830A626C: 4198FF50  blt cr6, 0x830a61bc
	if ctx.cr[6].lt {
	pc = 0x830A61BC; continue 'dispatch;
	}
	// 830A6270: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830A6274: 93E30014  stw r31, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 830A6278: 9963000D  stb r11, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 830A627C: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 830A6280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A6288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A6288 size=720
    let mut pc: u32 = 0x830A6288;
    'dispatch: loop {
        match pc {
            0x830A6288 => {
    //   block [0x830A6288..0x830A6558)
	// 830A6288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A628C: 48101EDD  bl 0x831a8168
	ctx.lr = 0x830A6290;
	sub_831A8130(ctx, base);
	// 830A6290: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A6294: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A6298: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A629C: A15F0004  lhz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A62A0: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A62A4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830A62A8: 419A0030  beq cr6, 0x830a62d8
	if ctx.cr[6].eq {
	pc = 0x830A62D8; continue 'dispatch;
	}
	// 830A62AC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A62B0: 80FF0028  lwz r7, 0x28(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A62B4: 38C0011C  li r6, 0x11c
	ctx.r[6].s64 = 284;
	// 830A62B8: 388BFF00  addi r4, r11, -0x100
	ctx.r[4].s64 = ctx.r[11].s64 + -256;
	// 830A62BC: 38A00123  li r5, 0x123
	ctx.r[5].s64 = 291;
	// 830A62C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A62C4: 4BF2AA05  bl 0x82fd0cc8
	ctx.lr = 0x830A62C8;
	sub_82FD0CC8(ctx, base);
	// 830A62C8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A62CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A62D0: 388BC3FC  addi r4, r11, -0x3c04
	ctx.r[4].s64 = ctx.r[11].s64 + -15364;
	// 830A62D4: 4810A955  bl 0x831b0c28
	ctx.lr = 0x830A62D8;
	sub_831B0C28(ctx, base);
	// 830A62D8: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A62DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A62E0: 419A0270  beq cr6, 0x830a6550
	if ctx.cr[6].eq {
	pc = 0x830A6550; continue 'dispatch;
	}
	// 830A62E4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A62E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A62EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A62F0: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830A62F4: 915F0024  stw r10, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 830A62F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A62FC: 4E800421  bctrl
	ctx.lr = 0x830A6300;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A6300: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6304: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A6308: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830A630C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A6310: 4E800421  bctrl
	ctx.lr = 0x830A6314;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A6314: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6318: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A631C: 409A0068  bne cr6, 0x830a6384
	if !ctx.cr[6].eq {
	pc = 0x830A6384; continue 'dispatch;
	}
	// 830A6320: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A6324: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A6328: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830A632C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 830A6330: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6334: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A6338: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A633C: 4E800421  bctrl
	ctx.lr = 0x830A6340;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A6340: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 830A6344: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A634C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A6350: 4099002C  ble cr6, 0x830a637c
	if !ctx.cr[6].gt {
	pc = 0x830A637C; continue 'dispatch;
	}
	// 830A6354: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830A6358: 813E0020  lwz r9, 0x20(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A635C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830A6360: 811F0020  lwz r8, 0x20(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6364: 7D2B482E  lwzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 830A6368: 7D2B412E  stwx r9, r11, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	// 830A636C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 830A6370: 813E0014  lwz r9, 0x14(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6374: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830A6378: 4198FFE0  blt cr6, 0x830a6358
	if ctx.cr[6].lt {
	pc = 0x830A6358; continue 'dispatch;
	}
	// 830A637C: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6380: 480001CC  b 0x830a654c
	pc = 0x830A654C; continue 'dispatch;
	// 830A6384: 815E0014  lwz r10, 0x14(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6388: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A638C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A6390: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 830A6394: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830A6398: 41980014  blt cr6, 0x830a63ac
	if ctx.cr[6].lt {
	pc = 0x830A63AC; continue 'dispatch;
	}
	// 830A639C: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A63A0: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A63A4: 7F8B5214  add r28, r11, r10
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830A63A8: 48000008  b 0x830a63b0
	pc = 0x830A63B0; continue 'dispatch;
	// 830A63AC: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 830A63B0: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A63B4: 5784103A  slwi r4, r28, 2
	ctx.r[4].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830A63B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A63BC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A63C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A63C4: 4E800421  bctrl
	ctx.lr = 0x830A63C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A63C8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A63CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A63D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A63D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830A63D8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A63DC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 830A63E0: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A63E4: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830A63E8: 41980050  blt cr6, 0x830a6438
	if ctx.cr[6].lt {
	pc = 0x830A6438; continue 'dispatch;
	}
	// 830A63EC: 813E0014  lwz r9, 0x14(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A63F0: 7F034840  cmplw cr6, r3, r9
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830A63F4: 4098012C  bge cr6, 0x830a6520
	if !ctx.cr[6].lt {
	pc = 0x830A6520; continue 'dispatch;
	}
	// 830A63F8: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830A63FC: 4198003C  blt cr6, 0x830a6438
	if ctx.cr[6].lt {
	pc = 0x830A6438; continue 'dispatch;
	}
	// 830A6400: 7CC93378  mr r9, r6
	ctx.r[9].u64 = ctx.r[6].u64;
	// 830A6404: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 830A6408: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 830A640C: 38630002  addi r3, r3, 2
	ctx.r[3].s64 = ctx.r[3].s64 + 2;
	// 830A6410: 38C60008  addi r6, r6, 8
	ctx.r[6].s64 = ctx.r[6].s64 + 8;
	// 830A6414: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 830A6418: 80FE0020  lwz r7, 0x20(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A641C: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830A6420: 7CE9382E  lwzx r7, r9, r7
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 830A6424: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 830A6428: 90E80000  stw r7, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 830A642C: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 830A6430: 4082FFE8  bne 0x830a6418
	if !ctx.cr[0].eq {
	pc = 0x830A6418; continue 'dispatch;
	}
	// 830A6434: 4BFFFFAC  b 0x830a63e0
	pc = 0x830A63E0; continue 'dispatch;
	// 830A6438: 815E0014  lwz r10, 0x14(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A643C: 7F035040  cmplw cr6, r3, r10
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830A6440: 4198003C  blt cr6, 0x830a647c
	if ctx.cr[6].lt {
	pc = 0x830A647C; continue 'dispatch;
	}
	// 830A6444: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 830A6448: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 830A644C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 830A6450: 38840002  addi r4, r4, 2
	ctx.r[4].s64 = ctx.r[4].s64 + 2;
	// 830A6454: 38A50008  addi r5, r5, 8
	ctx.r[5].s64 = ctx.r[5].s64 + 8;
	// 830A6458: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 830A645C: 80FF0020  lwz r7, 0x20(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6460: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830A6464: 7CE9382E  lwzx r7, r9, r7
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 830A6468: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 830A646C: 90E80000  stw r7, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 830A6470: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 830A6474: 4082FFE8  bne 0x830a645c
	if !ctx.cr[0].eq {
	pc = 0x830A645C; continue 'dispatch;
	}
	// 830A6478: 4BFFFF68  b 0x830a63e0
	pc = 0x830A63E0; continue 'dispatch;
	// 830A647C: 813E0020  lwz r9, 0x20(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6480: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6484: 7D064A14  add r8, r6, r9
	ctx.r[8].u64 = ctx.r[6].u64 + ctx.r[9].u64;
	// 830A6488: 7CE55214  add r7, r5, r10
	ctx.r[7].u64 = ctx.r[5].u64 + ctx.r[10].u64;
	// 830A648C: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6490: 81270000  lwz r9, 0(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6494: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 830A6498: 41980050  blt cr6, 0x830a64e8
	if ctx.cr[6].lt {
	pc = 0x830A64E8; continue 'dispatch;
	}
	// 830A649C: 409A0014  bne cr6, 0x830a64b0
	if !ctx.cr[6].eq {
	pc = 0x830A64B0; continue 'dispatch;
	}
	// 830A64A0: 81480004  lwz r10, 4(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A64A4: 81270004  lwz r9, 4(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A64A8: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 830A64AC: 4198003C  blt cr6, 0x830a64e8
	if ctx.cr[6].lt {
	pc = 0x830A64E8; continue 'dispatch;
	}
	// 830A64B0: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 830A64B4: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 830A64B8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 830A64BC: 38840002  addi r4, r4, 2
	ctx.r[4].s64 = ctx.r[4].s64 + 2;
	// 830A64C0: 38A50008  addi r5, r5, 8
	ctx.r[5].s64 = ctx.r[5].s64 + 8;
	// 830A64C4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 830A64C8: 80FF0020  lwz r7, 0x20(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A64CC: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830A64D0: 7CE9382E  lwzx r7, r9, r7
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 830A64D4: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 830A64D8: 90E80000  stw r7, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 830A64DC: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 830A64E0: 4082FFE8  bne 0x830a64c8
	if !ctx.cr[0].eq {
	pc = 0x830A64C8; continue 'dispatch;
	}
	// 830A64E4: 4BFFFEFC  b 0x830a63e0
	pc = 0x830A63E0; continue 'dispatch;
	// 830A64E8: 7CC93378  mr r9, r6
	ctx.r[9].u64 = ctx.r[6].u64;
	// 830A64EC: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 830A64F0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 830A64F4: 38630002  addi r3, r3, 2
	ctx.r[3].s64 = ctx.r[3].s64 + 2;
	// 830A64F8: 38C60008  addi r6, r6, 8
	ctx.r[6].s64 = ctx.r[6].s64 + 8;
	// 830A64FC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 830A6500: 80FE0020  lwz r7, 0x20(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6504: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830A6508: 7CE9382E  lwzx r7, r9, r7
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 830A650C: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 830A6510: 90E80000  stw r7, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 830A6514: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 830A6518: 4082FFE8  bne 0x830a6500
	if !ctx.cr[0].eq {
	pc = 0x830A6500; continue 'dispatch;
	}
	// 830A651C: 4BFFFEC4  b 0x830a63e0
	pc = 0x830A63E0; continue 'dispatch;
	// 830A6520: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A6524: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6528: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A652C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A6530: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A6534: 4E800421  bctrl
	ctx.lr = 0x830A6538;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A6538: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A653C: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6540: 93BF0020  stw r29, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 830A6544: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830A6548: 939F0018  stw r28, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[28].u32 ) };
	// 830A654C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830A6550: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830A6554: 48101C64  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A6558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A6558 size=632
    let mut pc: u32 = 0x830A6558;
    'dispatch: loop {
        match pc {
            0x830A6558 => {
    //   block [0x830A6558..0x830A67D0)
	// 830A6558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A655C: 48101C05  bl 0x831a8160
	ctx.lr = 0x830A6560;
	sub_831A8130(ctx, base);
	// 830A6560: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A6564: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A6568: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830A656C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6570: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A6574: 419A0254  beq cr6, 0x830a67c8
	if ctx.cr[6].eq {
	pc = 0x830A67C8; continue 'dispatch;
	}
	// 830A6578: 817C0020  lwz r11, 0x20(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A657C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A6580: 419A0248  beq cr6, 0x830a67c8
	if ctx.cr[6].eq {
	pc = 0x830A67C8; continue 'dispatch;
	}
	// 830A6584: A17C0004  lhz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A6588: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 830A658C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6590: 409A0014  bne cr6, 0x830a65a4
	if !ctx.cr[6].eq {
	pc = 0x830A65A4; continue 'dispatch;
	}
	// 830A6594: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 830A6598: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A659C: 4E800421  bctrl
	ctx.lr = 0x830A65A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A65A0: 48000228  b 0x830a67c8
	pc = 0x830A67C8; continue 'dispatch;
	// 830A65A4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830A65A8: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830A65AC: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 830A65B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A65B4: 4E800421  bctrl
	ctx.lr = 0x830A65B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A65B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A65BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A65C0: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 830A65C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A65C8: 4E800421  bctrl
	ctx.lr = 0x830A65CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A65CC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A65D0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830A65D4: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830A65D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A65DC: 4E800421  bctrl
	ctx.lr = 0x830A65E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A65E0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A65E4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830A65E8: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 830A65EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A65F0: 4E800421  bctrl
	ctx.lr = 0x830A65F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A65F4: 813C0014  lwz r9, 0x14(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A65F8: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A65FC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A6600: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 830A6604: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830A6608: 41980010  blt cr6, 0x830a6618
	if ctx.cr[6].lt {
	pc = 0x830A6618; continue 'dispatch;
	}
	// 830A660C: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A6610: 7F4A5A14  add r26, r10, r11
	ctx.r[26].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 830A6614: 48000008  b 0x830a661c
	pc = 0x830A661C; continue 'dispatch;
	// 830A6618: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	// 830A661C: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A6620: 5744103A  slwi r4, r26, 2
	ctx.r[4].u32 = ctx.r[26].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830A6624: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6628: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A662C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A6630: 4E800421  bctrl
	ctx.lr = 0x830A6634;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A6634: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6638: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 830A663C: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 830A6640: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 830A6644: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A6648: 40990100  ble cr6, 0x830a6748
	if !ctx.cr[6].gt {
	pc = 0x830A6748; continue 'dispatch;
	}
	// 830A664C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830A6650: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A6654: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 830A6658: 813C0014  lwz r9, 0x14(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A665C: 7F1D4840  cmplw cr6, r29, r9
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830A6660: 409800E8  bge cr6, 0x830a6748
	if !ctx.cr[6].lt {
	pc = 0x830A6748; continue 'dispatch;
	}
	// 830A6664: 80FF0020  lwz r7, 0x20(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6668: 813C0020  lwz r9, 0x20(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A666C: 7CAA3A14  add r5, r10, r7
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 830A6670: 7CC34A14  add r6, r3, r9
	ctx.r[6].u64 = ctx.r[3].u64 + ctx.r[9].u64;
	// 830A6674: 80850004  lwz r4, 4(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A6678: 80E60000  lwz r7, 0(r6)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A667C: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6680: 80C60004  lwz r6, 4(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A6684: 7F043800  cmpw cr6, r4, r7
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[7].s32, &mut ctx.xer);
	// 830A6688: 40980038  bge cr6, 0x830a66c0
	if !ctx.cr[6].lt {
	pc = 0x830A66C0; continue 'dispatch;
	}
	// 830A668C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 830A6690: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 830A6694: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6698: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 830A669C: 38FE0001  addi r7, r30, 1
	ctx.r[7].s64 = ctx.r[30].s64 + 1;
	// 830A66A0: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 830A66A4: 3BC70001  addi r30, r7, 1
	ctx.r[30].s64 = ctx.r[7].s64 + 1;
	// 830A66A8: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 830A66AC: 7D2A482E  lwzx r9, r10, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 830A66B0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 830A66B4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 830A66B8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 830A66BC: 48000080  b 0x830a673c
	pc = 0x830A673C; continue 'dispatch;
	// 830A66C0: 7F093000  cmpw cr6, r9, r6
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[6].s32, &mut ctx.xer);
	// 830A66C4: 41990070  bgt cr6, 0x830a6734
	if ctx.cr[6].gt {
	pc = 0x830A6734; continue 'dispatch;
	}
	// 830A66C8: 7F074800  cmpw cr6, r7, r9
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[9].s32, &mut ctx.xer);
	// 830A66CC: 41990024  bgt cr6, 0x830a66f0
	if ctx.cr[6].gt {
	pc = 0x830A66F0; continue 'dispatch;
	}
	// 830A66D0: 7F043000  cmpw cr6, r4, r6
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[6].s32, &mut ctx.xer);
	// 830A66D4: 41990010  bgt cr6, 0x830a66e4
	if ctx.cr[6].gt {
	pc = 0x830A66E4; continue 'dispatch;
	}
	// 830A66D8: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 830A66DC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 830A66E0: 4800005C  b 0x830a673c
	pc = 0x830A673C; continue 'dispatch;
	// 830A66E4: 39260001  addi r9, r6, 1
	ctx.r[9].s64 = ctx.r[6].s64 + 1;
	// 830A66E8: 91250000  stw r9, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 830A66EC: 48000048  b 0x830a6734
	pc = 0x830A6734; continue 'dispatch;
	// 830A66F0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 830A66F4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 830A66F8: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 830A66FC: 7F043000  cmpw cr6, r4, r6
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[6].s32, &mut ctx.xer);
	// 830A6700: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 830A6704: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 830A6708: 41990018  bgt cr6, 0x830a6720
	if ctx.cr[6].gt {
	pc = 0x830A6720; continue 'dispatch;
	}
	// 830A670C: 38DE0001  addi r6, r30, 1
	ctx.r[6].s64 = ctx.r[30].s64 + 1;
	// 830A6710: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 830A6714: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 830A6718: 3BC60001  addi r30, r6, 1
	ctx.r[30].s64 = ctx.r[6].s64 + 1;
	// 830A671C: 48000020  b 0x830a673c
	pc = 0x830A673C; continue 'dispatch;
	// 830A6720: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6724: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 830A6728: 38BE0001  addi r5, r30, 1
	ctx.r[5].s64 = ctx.r[30].s64 + 1;
	// 830A672C: 3BC50001  addi r30, r5, 1
	ctx.r[30].s64 = ctx.r[5].s64 + 1;
	// 830A6730: 7CCA492E  stwx r6, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[6].u32) };
	// 830A6734: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 830A6738: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 830A673C: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6740: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830A6744: 4198FF14  blt cr6, 0x830a6658
	if ctx.cr[6].lt {
	pc = 0x830A6658; continue 'dispatch;
	}
	// 830A6748: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A674C: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830A6750: 40980054  bge cr6, 0x830a67a4
	if !ctx.cr[6].lt {
	pc = 0x830A67A4; continue 'dispatch;
	}
	// 830A6754: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A6758: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A675C: 7D4ADA14  add r10, r10, r27
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[27].u64;
	// 830A6760: 39280001  addi r9, r8, 1
	ctx.r[9].s64 = ctx.r[8].s64 + 1;
	// 830A6764: 811F0020  lwz r8, 0x20(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6768: 38FE0001  addi r7, r30, 1
	ctx.r[7].s64 = ctx.r[30].s64 + 1;
	// 830A676C: 3BC70001  addi r30, r7, 1
	ctx.r[30].s64 = ctx.r[7].s64 + 1;
	// 830A6770: 7D0B402E  lwzx r8, r11, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 830A6774: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 830A6778: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 830A677C: 39090001  addi r8, r9, 1
	ctx.r[8].s64 = ctx.r[9].s64 + 1;
	// 830A6780: 80DF0020  lwz r6, 0x20(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6784: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 830A6788: 7D2B302E  lwzx r9, r11, r6
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 830A678C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 830A6790: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 830A6794: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 830A6798: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A679C: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830A67A0: 4198FFC0  blt cr6, 0x830a6760
	if ctx.cr[6].lt {
	pc = 0x830A6760; continue 'dispatch;
	}
	// 830A67A4: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A67A8: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A67AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A67B0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A67B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A67B8: 4E800421  bctrl
	ctx.lr = 0x830A67BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A67BC: 937F0020  stw r27, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[27].u32 ) };
	// 830A67C0: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 830A67C4: 935F0018  stw r26, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[26].u32 ) };
	// 830A67C8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830A67CC: 481019E4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A67D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A67D0 size=472
    let mut pc: u32 = 0x830A67D0;
    'dispatch: loop {
        match pc {
            0x830A67D0 => {
    //   block [0x830A67D0..0x830A69A8)
	// 830A67D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A67D4: 48101989  bl 0x831a815c
	ctx.lr = 0x830A67D8;
	sub_831A8130(ctx, base);
	// 830A67D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A67DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A67E0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830A67E4: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A67E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A67EC: 419A01B4  beq cr6, 0x830a69a0
	if ctx.cr[6].eq {
	pc = 0x830A69A0; continue 'dispatch;
	}
	// 830A67F0: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A67F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A67F8: 419A01A8  beq cr6, 0x830a69a0
	if ctx.cr[6].eq {
	pc = 0x830A69A0; continue 'dispatch;
	}
	// 830A67FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6800: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830A6804: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830A6808: 939F0024  stw r28, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[28].u32 ) };
	// 830A680C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A6810: 4E800421  bctrl
	ctx.lr = 0x830A6814;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A6814: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6818: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A681C: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 830A6820: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A6824: 4E800421  bctrl
	ctx.lr = 0x830A6828;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A6828: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A682C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A6830: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830A6834: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A6838: 4E800421  bctrl
	ctx.lr = 0x830A683C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A683C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6840: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A6844: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 830A6848: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A684C: 4E800421  bctrl
	ctx.lr = 0x830A6850;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A6850: 813D0014  lwz r9, 0x14(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6854: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6858: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A685C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 830A6860: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830A6864: 41980010  blt cr6, 0x830a6874
	if ctx.cr[6].lt {
	pc = 0x830A6874; continue 'dispatch;
	}
	// 830A6868: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A686C: 7F2A5A14  add r25, r10, r11
	ctx.r[25].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 830A6870: 48000008  b 0x830a6878
	pc = 0x830A6878; continue 'dispatch;
	// 830A6874: 7D795B78  mr r25, r11
	ctx.r[25].u64 = ctx.r[11].u64;
	// 830A6878: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A687C: 5724103A  slwi r4, r25, 2
	ctx.r[4].u32 = ctx.r[25].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830A6880: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6884: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A6888: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A688C: 4E800421  bctrl
	ctx.lr = 0x830A6890;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A6890: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6894: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 830A6898: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 830A689C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830A68A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A68A4: 409900D8  ble cr6, 0x830a697c
	if !ctx.cr[6].gt {
	pc = 0x830A697C; continue 'dispatch;
	}
	// 830A68A8: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 830A68AC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830A68B0: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 830A68B4: 837D0014  lwz r27, 0x14(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A68B8: 7F04D840  cmplw cr6, r4, r27
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[27].u32, &mut ctx.xer);
	// 830A68BC: 409800C0  bge cr6, 0x830a697c
	if !ctx.cr[6].lt {
	pc = 0x830A697C; continue 'dispatch;
	}
	// 830A68C0: 815D0020  lwz r10, 0x20(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A68C4: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A68C8: 7CC35214  add r6, r3, r10
	ctx.r[6].u64 = ctx.r[3].u64 + ctx.r[10].u64;
	// 830A68CC: 7D274A14  add r9, r7, r9
	ctx.r[9].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 830A68D0: 81060000  lwz r8, 0(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A68D4: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A68D8: 80A90000  lwz r5, 0(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A68DC: 81260004  lwz r9, 4(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A68E0: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 830A68E4: 41980084  blt cr6, 0x830a6968
	if ctx.cr[6].lt {
	pc = 0x830A6968; continue 'dispatch;
	}
	// 830A68E8: 7F054800  cmpw cr6, r5, r9
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[9].s32, &mut ctx.xer);
	// 830A68EC: 4199006C  bgt cr6, 0x830a6958
	if ctx.cr[6].gt {
	pc = 0x830A6958; continue 'dispatch;
	}
	// 830A68F0: 7F082800  cmpw cr6, r8, r5
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[5].s32, &mut ctx.xer);
	// 830A68F4: 41990028  bgt cr6, 0x830a691c
	if ctx.cr[6].gt {
	pc = 0x830A691C; continue 'dispatch;
	}
	// 830A68F8: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 830A68FC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 830A6900: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 830A6904: 41990020  bgt cr6, 0x830a6924
	if ctx.cr[6].gt {
	pc = 0x830A6924; continue 'dispatch;
	}
	// 830A6908: 393E0001  addi r9, r30, 1
	ctx.r[9].s64 = ctx.r[30].s64 + 1;
	// 830A690C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830A6910: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 830A6914: 3BC90001  addi r30, r9, 1
	ctx.r[30].s64 = ctx.r[9].s64 + 1;
	// 830A6918: 48000050  b 0x830a6968
	pc = 0x830A6968; continue 'dispatch;
	// 830A691C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 830A6920: 4BFFFFDC  b 0x830a68fc
	pc = 0x830A68FC; continue 'dispatch;
	// 830A6924: 395E0001  addi r10, r30, 1
	ctx.r[10].s64 = ctx.r[30].s64 + 1;
	// 830A6928: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 830A692C: 38840002  addi r4, r4, 2
	ctx.r[4].s64 = ctx.r[4].s64 + 2;
	// 830A6930: 3BCA0001  addi r30, r10, 1
	ctx.r[30].s64 = ctx.r[10].s64 + 1;
	// 830A6934: 815D0014  lwz r10, 0x14(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6938: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 830A693C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 830A6940: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830A6944: 40980024  bge cr6, 0x830a6968
	if !ctx.cr[6].lt {
	pc = 0x830A6968; continue 'dispatch;
	}
	// 830A6948: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A694C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 830A6950: 7D27512E  stwx r9, r7, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 830A6954: 4800001C  b 0x830a6970
	pc = 0x830A6970; continue 'dispatch;
	// 830A6958: 38840002  addi r4, r4, 2
	ctx.r[4].s64 = ctx.r[4].s64 + 2;
	// 830A695C: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 830A6960: 7F04D840  cmplw cr6, r4, r27
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[27].u32, &mut ctx.xer);
	// 830A6964: 4198000C  blt cr6, 0x830a6970
	if ctx.cr[6].lt {
	pc = 0x830A6970; continue 'dispatch;
	}
	// 830A6968: 38E70008  addi r7, r7, 8
	ctx.r[7].s64 = ctx.r[7].s64 + 8;
	// 830A696C: 3B9C0002  addi r28, r28, 2
	ctx.r[28].s64 = ctx.r[28].s64 + 2;
	// 830A6970: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6974: 7F1C5040  cmplw cr6, r28, r10
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830A6978: 4198FF3C  blt cr6, 0x830a68b4
	if ctx.cr[6].lt {
	pc = 0x830A68B4; continue 'dispatch;
	}
	// 830A697C: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A6980: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6984: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6988: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A698C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A6990: 4E800421  bctrl
	ctx.lr = 0x830A6994;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A6994: 935F0020  stw r26, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[26].u32 ) };
	// 830A6998: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 830A699C: 933F0018  stw r25, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[25].u32 ) };
	// 830A69A0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830A69A4: 48101808  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A69A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A69A8 size=352
    let mut pc: u32 = 0x830A69A8;
    'dispatch: loop {
        match pc {
            0x830A69A8 => {
    //   block [0x830A69A8..0x830A6B08)
	// 830A69A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A69AC: 481017B9  bl 0x831a8164
	ctx.lr = 0x830A69B0;
	sub_831A8130(ctx, base);
	// 830A69B0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A69B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A69B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A69BC: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 830A69C0: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A69C4: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 830A69C8: 419A0034  beq cr6, 0x830a69fc
	if ctx.cr[6].eq {
	pc = 0x830A69FC; continue 'dispatch;
	}
	// 830A69CC: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 830A69D0: 419A002C  beq cr6, 0x830a69fc
	if ctx.cr[6].eq {
	pc = 0x830A69FC; continue 'dispatch;
	}
	// 830A69D4: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A69D8: 38C0011F  li r6, 0x11f
	ctx.r[6].s64 = 287;
	// 830A69DC: 388BFF00  addi r4, r11, -0x100
	ctx.r[4].s64 = ctx.r[11].s64 + -256;
	// 830A69E0: 38A00220  li r5, 0x220
	ctx.r[5].s64 = 544;
	// 830A69E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A69E8: 4BF2A2E1  bl 0x82fd0cc8
	ctx.lr = 0x830A69EC;
	sub_82FD0CC8(ctx, base);
	// 830A69EC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A69F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A69F4: 388BC3FC  addi r4, r11, -0x3c04
	ctx.r[4].s64 = ctx.r[11].s64 + -15364;
	// 830A69F8: 4810A231  bl 0x831b0c28
	ctx.lr = 0x830A69FC;
	sub_831B0C28(ctx, base);
	// 830A69FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6A00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A6A04: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830A6A08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A6A0C: 4E800421  bctrl
	ctx.lr = 0x830A6A10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A6A10: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6A14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A6A18: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 830A6A1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A6A20: 4E800421  bctrl
	ctx.lr = 0x830A6A24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A6A24: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6A28: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6A2C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A6A30: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A6A34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A6A38: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830A6A3C: 836BFFFC  lwz r27, -4(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 830A6A40: 4BFFCD31  bl 0x830a3770
	ctx.lr = 0x830A6A44;
	sub_830A3770(ctx, base);
	// 830A6A44: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6A48: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A6A4C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6A50: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830A6A54: 4081001C  ble 0x830a6a70
	if !ctx.cr[0].gt {
	pc = 0x830A6A70; continue 'dispatch;
	}
	// 830A6A58: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6A5C: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 830A6A60: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A6A64: 816A002C  lwz r11, 0x2c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A6A68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A6A6C: 4E800421  bctrl
	ctx.lr = 0x830A6A70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A6A70: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6A74: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 830A6A78: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 830A6A7C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 830A6A80: 4099004C  ble cr6, 0x830a6acc
	if !ctx.cr[6].gt {
	pc = 0x830A6ACC; continue 'dispatch;
	}
	// 830A6A84: 3B800004  li r28, 4
	ctx.r[28].s64 = 4;
	// 830A6A88: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6A8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A6A90: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6A94: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 830A6A98: 812A002C  lwz r9, 0x2c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A6A9C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A6AA0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6AA4: 38AAFFFF  addi r5, r10, -1
	ctx.r[5].s64 = ctx.r[10].s64 + -1;
	// 830A6AA8: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	// 830A6AAC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 830A6AB0: 4E800421  bctrl
	ctx.lr = 0x830A6AB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A6AB4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6AB8: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 830A6ABC: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 830A6AC0: 3B9C0008  addi r28, r28, 8
	ctx.r[28].s64 = ctx.r[28].s64 + 8;
	// 830A6AC4: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830A6AC8: 4198FFC0  blt cr6, 0x830a6a88
	if ctx.cr[6].lt {
	pc = 0x830A6A88; continue 'dispatch;
	}
	// 830A6ACC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A6AD0: 80ABFD68  lwz r5, -0x298(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-664 as u32) ) } as u64;
	// 830A6AD4: 7F1B2800  cmpw cr6, r27, r5
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[5].s32, &mut ctx.xer);
	// 830A6AD8: 419A001C  beq cr6, 0x830a6af4
	if ctx.cr[6].eq {
	pc = 0x830A6AF4; continue 'dispatch;
	}
	// 830A6ADC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6AE0: 389B0001  addi r4, r27, 1
	ctx.r[4].s64 = ctx.r[27].s64 + 1;
	// 830A6AE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A6AE8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A6AEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A6AF0: 4E800421  bctrl
	ctx.lr = 0x830A6AF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A6AF4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830A6AF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A6AFC: 997E000D  stb r11, 0xd(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 830A6B00: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 830A6B04: 481016B0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A6B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830A6B08 size=200
    let mut pc: u32 = 0x830A6B08;
    'dispatch: loop {
        match pc {
            0x830A6B08 => {
    //   block [0x830A6B08..0x830A6BD0)
	// 830A6B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A6B0C: 48101661  bl 0x831a816c
	ctx.lr = 0x830A6B10;
	sub_831A8130(ctx, base);
	// 830A6B10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A6B14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A6B18: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 830A6B1C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6B20: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 830A6B24: 7FCB2214  add r30, r11, r4
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 830A6B28: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 830A6B2C: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 830A6B30: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 830A6B34: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 830A6B38: C80B7390  lfd f0, 0x7390(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(29584 as u32) ) };
	// 830A6B3C: FC0D0032  fmul f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 830A6B40: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 830A6B44: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 830A6B48: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A6B4C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830A6B50: 40980008  bge cr6, 0x830a6b58
	if !ctx.cr[6].lt {
	pc = 0x830A6B58; continue 'dispatch;
	}
	// 830A6B54: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 830A6B58: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A6B5C: 57C4103A  slwi r4, r30, 2
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830A6B60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6B64: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A6B68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A6B6C: 4E800421  bctrl
	ctx.lr = 0x830A6B70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A6B70: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6B74: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A6B78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A6B7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A6B80: 40990028  ble cr6, 0x830a6ba8
	if !ctx.cr[6].gt {
	pc = 0x830A6BA8; continue 'dispatch;
	}
	// 830A6B84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830A6B88: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6B8C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830A6B90: 7D29582E  lwzx r9, r9, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830A6B94: 7D2BE92E  stwx r9, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[9].u32) };
	// 830A6B98: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 830A6B9C: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6BA0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830A6BA4: 4198FFE4  blt cr6, 0x830a6b88
	if ctx.cr[6].lt {
	pc = 0x830A6B88; continue 'dispatch;
	}
	// 830A6BA8: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A6BAC: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6BB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6BB4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A6BB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A6BBC: 4E800421  bctrl
	ctx.lr = 0x830A6BC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A6BC0: 93BF0020  stw r29, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 830A6BC4: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 830A6BC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830A6BCC: 481015F0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A6BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830A6BD0 size=240
    let mut pc: u32 = 0x830A6BD0;
    'dispatch: loop {
        match pc {
            0x830A6BD0 => {
    //   block [0x830A6BD0..0x830A6CC0)
	// 830A6BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A6BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A6BD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A6BDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A6BE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A6BE4: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 830A6BE8: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A6BEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6BF0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A6BF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A6BF8: 4E800421  bctrl
	ctx.lr = 0x830A6BFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A6BFC: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6C00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830A6C04: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 830A6C08: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 830A6C0C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 830A6C10: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A6C14: 7CEA592E  stwx r7, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[7].u32) };
	// 830A6C18: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 830A6C1C: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 830A6C20: 4198FFF0  blt cr6, 0x830a6c10
	if ctx.cr[6].lt {
	pc = 0x830A6C10; continue 'dispatch;
	}
	// 830A6C24: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6C28: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 830A6C2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A6C30: 40990020  ble cr6, 0x830a6c50
	if !ctx.cr[6].gt {
	pc = 0x830A6C50; continue 'dispatch;
	}
	// 830A6C34: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6C38: 7D475A14  add r10, r7, r11
	ctx.r[10].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 830A6C3C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6C40: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A6C44: 2F0B0100  cmpwi cr6, r11, 0x100
	ctx.cr[6].compare_i32(ctx.r[11].s32, 256, &mut ctx.xer);
	// 830A6C48: 41980050  blt cr6, 0x830a6c98
	if ctx.cr[6].lt {
	pc = 0x830A6C98; continue 'dispatch;
	}
	// 830A6C4C: 90DF0010  stw r6, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[6].u32 ) };
	// 830A6C50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A6C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A6C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A6C5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A6C60: 4E800020  blr
	return;
	// 830A6C64: 2F0B0100  cmpwi cr6, r11, 0x100
	ctx.cr[6].compare_i32(ctx.r[11].s32, 256, &mut ctx.xer);
	// 830A6C68: 40980038  bge cr6, 0x830a6ca0
	if !ctx.cr[6].lt {
	pc = 0x830A6CA0; continue 'dispatch;
	}
	// 830A6C6C: 556506FE  clrlwi r5, r11, 0x1b
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830A6C70: 813F001C  lwz r9, 0x1c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A6C74: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830A6C78: 7D6A2E70  srawi r10, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 830A6C7C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830A6C80: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 830A6C84: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A6C88: 7C852830  slw r5, r4, r5
	if (ctx.r[5].u8 & 0x20) != 0 {
		ctx.r[5].u64 = 0;
	} else {
		ctx.r[5].u64 = ((ctx.r[4].u32) << ((ctx.r[5].u8 & 0x1F) as u32)) as u64;
	}
	// 830A6C8C: 7C8A482E  lwzx r4, r10, r9
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 830A6C90: 7CA52378  or r5, r5, r4
	ctx.r[5].u64 = ctx.r[5].u64 | ctx.r[4].u64;
	// 830A6C94: 7CAA492E  stwx r5, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[5].u32) };
	// 830A6C98: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 830A6C9C: 4099FFC8  ble cr6, 0x830a6c64
	if !ctx.cr[6].gt {
	pc = 0x830A6C64; continue 'dispatch;
	}
	// 830A6CA0: 2F080100  cmpwi cr6, r8, 0x100
	ctx.cr[6].compare_i32(ctx.r[8].s32, 256, &mut ctx.xer);
	// 830A6CA4: 4098FFA8  bge cr6, 0x830a6c4c
	if !ctx.cr[6].lt {
	pc = 0x830A6C4C; continue 'dispatch;
	}
	// 830A6CA8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6CAC: 38C60002  addi r6, r6, 2
	ctx.r[6].s64 = ctx.r[6].s64 + 2;
	// 830A6CB0: 38E70008  addi r7, r7, 8
	ctx.r[7].s64 = ctx.r[7].s64 + 8;
	// 830A6CB4: 7F065840  cmplw cr6, r6, r11
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830A6CB8: 4198FF7C  blt cr6, 0x830a6c34
	if ctx.cr[6].lt {
	pc = 0x830A6C34; continue 'dispatch;
	}
	// 830A6CBC: 4BFFFF94  b 0x830a6c50
	pc = 0x830A6C50; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A6CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A6CC0 size=76
    let mut pc: u32 = 0x830A6CC0;
    'dispatch: loop {
        match pc {
            0x830A6CC0 => {
    //   block [0x830A6CC0..0x830A6D0C)
	// 830A6CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A6CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A6CC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A6CCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A6CD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A6CD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A6CD8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A6CDC: 4BFFF22D  bl 0x830a5f08
	ctx.lr = 0x830A6CE0;
	sub_830A5F08(ctx, base);
	// 830A6CE0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A6CE4: 4182000C  beq 0x830a6cf0
	if ctx.cr[0].eq {
	pc = 0x830A6CF0; continue 'dispatch;
	}
	// 830A6CE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A6CEC: 4BF315F5  bl 0x82fd82e0
	ctx.lr = 0x830A6CF0;
	sub_82FD82E0(ctx, base);
	// 830A6CF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A6CF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A6CF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A6CFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A6D00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A6D04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A6D08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A6D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A6D10 size=304
    let mut pc: u32 = 0x830A6D10;
    'dispatch: loop {
        match pc {
            0x830A6D10 => {
    //   block [0x830A6D10..0x830A6E40)
	// 830A6D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A6D14: 48101455  bl 0x831a8168
	ctx.lr = 0x830A6D18;
	sub_831A8130(ctx, base);
	// 830A6D18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A6D1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A6D20: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830A6D24: 7F042800  cmpw cr6, r4, r5
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[5].s32, &mut ctx.xer);
	// 830A6D28: 939F0024  stw r28, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[28].u32 ) };
	// 830A6D2C: 41990010  bgt cr6, 0x830a6d3c
	if ctx.cr[6].gt {
	pc = 0x830A6D3C; continue 'dispatch;
	}
	// 830A6D30: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A6D34: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830A6D38: 4800000C  b 0x830a6d44
	pc = 0x830A6D44; continue 'dispatch;
	// 830A6D3C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830A6D40: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830A6D44: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6D48: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A6D4C: 40820044  bne 0x830a6d90
	if !ctx.cr[0].eq {
	pc = 0x830A6D90; continue 'dispatch;
	}
	// 830A6D50: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A6D54: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A6D58: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830A6D5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6D60: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A6D64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A6D68: 4E800421  bctrl
	ctx.lr = 0x830A6D6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A6D6C: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 830A6D70: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 830A6D74: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 830A6D78: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6D7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830A6D80: 93A90004  stw r29, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 830A6D84: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830A6D88: 995F000C  stb r10, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 830A6D8C: 480000AC  b 0x830a6e38
	pc = 0x830A6E38; continue 'dispatch;
	// 830A6D90: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6D94: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A6D98: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 830A6D9C: 812AFFFC  lwz r9, -4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) } as u64;
	// 830A6DA0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 830A6DA4: 7F09F000  cmpw cr6, r9, r30
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[30].s32, &mut ctx.xer);
	// 830A6DA8: 409A000C  bne cr6, 0x830a6db4
	if !ctx.cr[6].eq {
	pc = 0x830A6DB4; continue 'dispatch;
	}
	// 830A6DAC: 93AAFFFC  stw r29, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[29].u32 ) };
	// 830A6DB0: 48000088  b 0x830a6e38
	pc = 0x830A6E38; continue 'dispatch;
	// 830A6DB4: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A6DB8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830A6DBC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830A6DC0: 41980010  blt cr6, 0x830a6dd0
	if ctx.cr[6].lt {
	pc = 0x830A6DD0; continue 'dispatch;
	}
	// 830A6DC4: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 830A6DC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A6DCC: 4BFFFD3D  bl 0x830a6b08
	ctx.lr = 0x830A6DD0;
	sub_830A6B08(ctx, base);
	// 830A6DD0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6DD4: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6DD8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A6DDC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830A6DE0: 814BFFFC  lwz r10, -4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 830A6DE4: 7F0AF000  cmpw cr6, r10, r30
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[30].s32, &mut ctx.xer);
	// 830A6DE8: 41980008  blt cr6, 0x830a6df0
	if ctx.cr[6].lt {
	pc = 0x830A6DF0; continue 'dispatch;
	}
	// 830A6DEC: 9B9F000C  stb r28, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[28].u8 ) };
	// 830A6DF0: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 830A6DF4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6DF8: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6DFC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830A6E00: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830A6E04: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830A6E08: 7FA9512E  stwx r29, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u32) };
	// 830A6E0C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6E10: 895F000C  lbz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830A6E14: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830A6E18: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A6E1C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830A6E20: 40820018  bne 0x830a6e38
	if !ctx.cr[0].eq {
	pc = 0x830A6E38; continue 'dispatch;
	}
	// 830A6E24: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6E28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A6E2C: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830A6E30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A6E34: 4E800421  bctrl
	ctx.lr = 0x830A6E38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A6E38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830A6E3C: 4810137C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A6E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830A6E40 size=368
    let mut pc: u32 = 0x830A6E40;
    'dispatch: loop {
        match pc {
            0x830A6E40 => {
    //   block [0x830A6E40..0x830A6FB0)
	// 830A6E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A6E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A6E48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A6E4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A6E50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A6E54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A6E58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A6E5C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A6E60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A6E64: 409A0008  bne cr6, 0x830a6e6c
	if !ctx.cr[6].eq {
	pc = 0x830A6E6C; continue 'dispatch;
	}
	// 830A6E68: 4BFFFD69  bl 0x830a6bd0
	ctx.lr = 0x830A6E6C;
	sub_830A6BD0(ctx, base);
	// 830A6E6C: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A6E70: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 830A6E74: 409A0098  bne cr6, 0x830a6f0c
	if !ctx.cr[6].eq {
	pc = 0x830A6F0C; continue 'dispatch;
	}
	// 830A6E78: 2F1E0100  cmpwi cr6, r30, 0x100
	ctx.cr[6].compare_i32(ctx.r[30].s32, 256, &mut ctx.xer);
	// 830A6E7C: 4098003C  bge cr6, 0x830a6eb8
	if !ctx.cr[6].lt {
	pc = 0x830A6EB8; continue 'dispatch;
	}
	// 830A6E80: 57CB06FE  clrlwi r11, r30, 0x1b
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x0000001Fu64;
	// 830A6E84: 813F001C  lwz r9, 0x1c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A6E88: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830A6E8C: 7FC82E70  srawi r8, r30, 5
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[30].s32 >> 5) as i64;
	// 830A6E90: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 830A6E94: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 830A6E98: 7D28482E  lwzx r9, r8, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 830A6E9C: 7D4B5830  slw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 830A6EA0: 7D6B4838  and r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	// 830A6EA4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830A6EA8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830A6EAC: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830A6EB0: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 830A6EB4: 480000E4  b 0x830a6f98
	pc = 0x830A6F98; continue 'dispatch;
	// 830A6EB8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A6EBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A6EC0: 811F0014  lwz r8, 0x14(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6EC4: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 830A6EC8: 409800D0  bge cr6, 0x830a6f98
	if !ctx.cr[6].lt {
	pc = 0x830A6F98; continue 'dispatch;
	}
	// 830A6ECC: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6ED0: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A6ED4: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 830A6ED8: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6EDC: 7F09F000  cmpw cr6, r9, r30
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[30].s32, &mut ctx.xer);
	// 830A6EE0: 41990010  bgt cr6, 0x830a6ef0
	if ctx.cr[6].gt {
	pc = 0x830A6EF0; continue 'dispatch;
	}
	// 830A6EE4: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A6EE8: 7F1E4800  cmpw cr6, r30, r9
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[9].s32, &mut ctx.xer);
	// 830A6EEC: 40990018  ble cr6, 0x830a6f04
	if !ctx.cr[6].gt {
	pc = 0x830A6F04; continue 'dispatch;
	}
	// 830A6EF0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830A6EF4: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 830A6EF8: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 830A6EFC: 4198FFDC  blt cr6, 0x830a6ed8
	if ctx.cr[6].lt {
	pc = 0x830A6ED8; continue 'dispatch;
	}
	// 830A6F00: 48000098  b 0x830a6f98
	pc = 0x830A6F98; continue 'dispatch;
	// 830A6F04: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830A6F08: 48000090  b 0x830a6f98
	pc = 0x830A6F98; continue 'dispatch;
	// 830A6F0C: 2F1E0100  cmpwi cr6, r30, 0x100
	ctx.cr[6].compare_i32(ctx.r[30].s32, 256, &mut ctx.xer);
	// 830A6F10: 40980038  bge cr6, 0x830a6f48
	if !ctx.cr[6].lt {
	pc = 0x830A6F48; continue 'dispatch;
	}
	// 830A6F14: 57CB06FE  clrlwi r11, r30, 0x1b
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x0000001Fu64;
	// 830A6F18: 813F001C  lwz r9, 0x1c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A6F1C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830A6F20: 7FC82E70  srawi r8, r30, 5
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[30].s32 >> 5) as i64;
	// 830A6F24: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 830A6F28: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 830A6F2C: 7D28482E  lwzx r9, r8, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 830A6F30: 7D4B5830  slw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 830A6F34: 7D6B4838  and r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	// 830A6F38: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830A6F3C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830A6F40: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830A6F44: 48000054  b 0x830a6f98
	pc = 0x830A6F98; continue 'dispatch;
	// 830A6F48: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A6F4C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830A6F50: 811F0014  lwz r8, 0x14(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A6F54: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 830A6F58: 40980040  bge cr6, 0x830a6f98
	if !ctx.cr[6].lt {
	pc = 0x830A6F98; continue 'dispatch;
	}
	// 830A6F5C: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A6F60: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A6F64: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 830A6F68: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6F6C: 7F09F000  cmpw cr6, r9, r30
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[30].s32, &mut ctx.xer);
	// 830A6F70: 41990010  bgt cr6, 0x830a6f80
	if ctx.cr[6].gt {
	pc = 0x830A6F80; continue 'dispatch;
	}
	// 830A6F74: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A6F78: 7F1E4800  cmpw cr6, r30, r9
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[9].s32, &mut ctx.xer);
	// 830A6F7C: 40990018  ble cr6, 0x830a6f94
	if !ctx.cr[6].gt {
	pc = 0x830A6F94; continue 'dispatch;
	}
	// 830A6F80: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830A6F84: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 830A6F88: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 830A6F8C: 4198FFDC  blt cr6, 0x830a6f68
	if ctx.cr[6].lt {
	pc = 0x830A6F68; continue 'dispatch;
	}
	// 830A6F90: 48000008  b 0x830a6f98
	pc = 0x830A6F98; continue 'dispatch;
	// 830A6F94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A6F98: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A6F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A6FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A6FA4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A6FA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A6FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A6FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A6FB0 size=20
    let mut pc: u32 = 0x830A6FB0;
    'dispatch: loop {
        match pc {
            0x830A6FB0 => {
    //   block [0x830A6FB0..0x830A6FC4)
	// 830A6FB0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A6FB4: 546A043E  clrlwi r10, r3, 0x10
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 830A6FB8: 396BFF38  addi r11, r11, -0xc8
	ctx.r[11].s64 = ctx.r[11].s64 + -200;
	// 830A6FBC: 7C6A58AE  lbzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830A6FC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A6FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A6FC8 size=108
    let mut pc: u32 = 0x830A6FC8;
    'dispatch: loop {
        match pc {
            0x830A6FC8 => {
    //   block [0x830A6FC8..0x830A7034)
	// 830A6FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A6FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A6FD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A6FD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A6FD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A6FDC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830A6FE0: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 830A6FE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A6FE8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A6FEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A6FF0: 4E800421  bctrl
	ctx.lr = 0x830A6FF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A6FF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A6FF8: 3D7FFFFF  addis r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -65536;
	// 830A6FFC: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 830A7000: 556A05BE  clrlwi r10, r11, 0x16
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000003FFu64;
	// 830A7004: 7D6B5670  srawi r11, r11, 0xa
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 10) as i64;
	// 830A7008: 3D4A0001  addis r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 65536;
	// 830A700C: 3D6B0001  addis r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 65536;
	// 830A7010: 394ADC00  addi r10, r10, -0x2400
	ctx.r[10].s64 = ctx.r[10].s64 + -9216;
	// 830A7014: 396BD800  addi r11, r11, -0x2800
	ctx.r[11].s64 = ctx.r[11].s64 + -10240;
	// 830A7018: B1430002  sth r10, 2(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(2 as u32), ctx.r[10].u16 ) };
	// 830A701C: B1630000  sth r11, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 830A7020: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A7024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A7028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A702C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A7030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A7038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A7038 size=324
    let mut pc: u32 = 0x830A7038;
    'dispatch: loop {
        match pc {
            0x830A7038 => {
    //   block [0x830A7038..0x830A717C)
	// 830A7038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A703C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A7040: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A7044: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A7048: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A704C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830A7050: 419A000C  beq cr6, 0x830a705c
	if ctx.cr[6].eq {
	pc = 0x830A705C; continue 'dispatch;
	}
	// 830A7054: 4BF2A225  bl 0x82fd1278
	ctx.lr = 0x830A7058;
	sub_82FD1278(ctx, base);
	// 830A7058: 4800000C  b 0x830a7064
	pc = 0x830A7064; continue 'dispatch;
	// 830A705C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A7060: 4BF29B21  bl 0x82fd0b80
	ctx.lr = 0x830A7064;
	sub_82FD0B80(ctx, base);
	// 830A7064: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830A7068: 419A0100  beq cr6, 0x830a7168
	if ctx.cr[6].eq {
	pc = 0x830A7168; continue 'dispatch;
	}
	// 830A706C: A15F0000  lhz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A7070: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 830A7074: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 830A7078: 480000E0  b 0x830a7158
	pc = 0x830A7158; continue 'dispatch;
	// 830A707C: A10B0000  lhz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A7080: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830A7084: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 830A7088: 2B0A000C  cmplwi cr6, r10, 0xc
	ctx.cr[6].compare_u32(ctx.r[10].u32, 12 as u32, &mut ctx.xer);
	// 830A708C: 419A00C8  beq cr6, 0x830a7154
	if ctx.cr[6].eq {
	pc = 0x830A7154; continue 'dispatch;
	}
	// 830A7090: 2B0A000D  cmplwi cr6, r10, 0xd
	ctx.cr[6].compare_u32(ctx.r[10].u32, 13 as u32, &mut ctx.xer);
	// 830A7094: 419A00C0  beq cr6, 0x830a7154
	if ctx.cr[6].eq {
	pc = 0x830A7154; continue 'dispatch;
	}
	// 830A7098: 2B0A000A  cmplwi cr6, r10, 0xa
	ctx.cr[6].compare_u32(ctx.r[10].u32, 10 as u32, &mut ctx.xer);
	// 830A709C: 419A00B8  beq cr6, 0x830a7154
	if ctx.cr[6].eq {
	pc = 0x830A7154; continue 'dispatch;
	}
	// 830A70A0: 2B0A0020  cmplwi cr6, r10, 0x20
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32 as u32, &mut ctx.xer);
	// 830A70A4: 419A00B0  beq cr6, 0x830a7154
	if ctx.cr[6].eq {
	pc = 0x830A7154; continue 'dispatch;
	}
	// 830A70A8: 2B0A0009  cmplwi cr6, r10, 9
	ctx.cr[6].compare_u32(ctx.r[10].u32, 9 as u32, &mut ctx.xer);
	// 830A70AC: 419A00A8  beq cr6, 0x830a7154
	if ctx.cr[6].eq {
	pc = 0x830A7154; continue 'dispatch;
	}
	// 830A70B0: 2B0A0023  cmplwi cr6, r10, 0x23
	ctx.cr[6].compare_u32(ctx.r[10].u32, 35 as u32, &mut ctx.xer);
	// 830A70B4: 419A0080  beq cr6, 0x830a7134
	if ctx.cr[6].eq {
	pc = 0x830A7134; continue 'dispatch;
	}
	// 830A70B8: 2B0A005C  cmplwi cr6, r10, 0x5c
	ctx.cr[6].compare_u32(ctx.r[10].u32, 92 as u32, &mut ctx.xer);
	// 830A70BC: 409A0090  bne cr6, 0x830a714c
	if !ctx.cr[6].eq {
	pc = 0x830A714C; continue 'dispatch;
	}
	// 830A70C0: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A70C4: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A70C8: 41820084  beq 0x830a714c
	if ctx.cr[0].eq {
	pc = 0x830A714C; continue 'dispatch;
	}
	// 830A70CC: 552A043E  clrlwi r10, r9, 0x10
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 830A70D0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830A70D4: 2B0A0023  cmplwi cr6, r10, 0x23
	ctx.cr[6].compare_u32(ctx.r[10].u32, 35 as u32, &mut ctx.xer);
	// 830A70D8: 419A006C  beq cr6, 0x830a7144
	if ctx.cr[6].eq {
	pc = 0x830A7144; continue 'dispatch;
	}
	// 830A70DC: 2B0A0009  cmplwi cr6, r10, 9
	ctx.cr[6].compare_u32(ctx.r[10].u32, 9 as u32, &mut ctx.xer);
	// 830A70E0: 419A0064  beq cr6, 0x830a7144
	if ctx.cr[6].eq {
	pc = 0x830A7144; continue 'dispatch;
	}
	// 830A70E4: 2B0A000A  cmplwi cr6, r10, 0xa
	ctx.cr[6].compare_u32(ctx.r[10].u32, 10 as u32, &mut ctx.xer);
	// 830A70E8: 419A005C  beq cr6, 0x830a7144
	if ctx.cr[6].eq {
	pc = 0x830A7144; continue 'dispatch;
	}
	// 830A70EC: 2B0A000C  cmplwi cr6, r10, 0xc
	ctx.cr[6].compare_u32(ctx.r[10].u32, 12 as u32, &mut ctx.xer);
	// 830A70F0: 419A0054  beq cr6, 0x830a7144
	if ctx.cr[6].eq {
	pc = 0x830A7144; continue 'dispatch;
	}
	// 830A70F4: 2B0A000D  cmplwi cr6, r10, 0xd
	ctx.cr[6].compare_u32(ctx.r[10].u32, 13 as u32, &mut ctx.xer);
	// 830A70F8: 419A004C  beq cr6, 0x830a7144
	if ctx.cr[6].eq {
	pc = 0x830A7144; continue 'dispatch;
	}
	// 830A70FC: 2B0A0020  cmplwi cr6, r10, 0x20
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32 as u32, &mut ctx.xer);
	// 830A7100: 419A0044  beq cr6, 0x830a7144
	if ctx.cr[6].eq {
	pc = 0x830A7144; continue 'dispatch;
	}
	// 830A7104: 3940005C  li r10, 0x5c
	ctx.r[10].s64 = 92;
	// 830A7108: B1470000  sth r10, 0(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 830A710C: 39470002  addi r10, r7, 2
	ctx.r[10].s64 = ctx.r[7].s64 + 2;
	// 830A7110: 38EA0002  addi r7, r10, 2
	ctx.r[7].s64 = ctx.r[10].s64 + 2;
	// 830A7114: B12A0000  sth r9, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 830A7118: 4800003C  b 0x830a7154
	pc = 0x830A7154; continue 'dispatch;
	// 830A711C: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 830A7120: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830A7124: 2B0A000A  cmplwi cr6, r10, 0xa
	ctx.cr[6].compare_u32(ctx.r[10].u32, 10 as u32, &mut ctx.xer);
	// 830A7128: 419A002C  beq cr6, 0x830a7154
	if ctx.cr[6].eq {
	pc = 0x830A7154; continue 'dispatch;
	}
	// 830A712C: 2B0A000D  cmplwi cr6, r10, 0xd
	ctx.cr[6].compare_u32(ctx.r[10].u32, 13 as u32, &mut ctx.xer);
	// 830A7130: 419A0024  beq cr6, 0x830a7154
	if ctx.cr[6].eq {
	pc = 0x830A7154; continue 'dispatch;
	}
	// 830A7134: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A7138: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A713C: 4082FFE0  bne 0x830a711c
	if !ctx.cr[0].eq {
	pc = 0x830A711C; continue 'dispatch;
	}
	// 830A7140: 48000014  b 0x830a7154
	pc = 0x830A7154; continue 'dispatch;
	// 830A7144: B1270000  sth r9, 0(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 830A7148: 48000008  b 0x830a7150
	pc = 0x830A7150; continue 'dispatch;
	// 830A714C: B1070000  sth r8, 0(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 830A7150: 38E70002  addi r7, r7, 2
	ctx.r[7].s64 = ctx.r[7].s64 + 2;
	// 830A7154: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A7158: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A715C: 4082FF20  bne 0x830a707c
	if !ctx.cr[0].eq {
	pc = 0x830A707C; continue 'dispatch;
	}
	// 830A7160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830A7164: B1670000  sth r11, 0(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 830A7168: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A716C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A7170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A7174: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A7178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A7180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A7180 size=8
    let mut pc: u32 = 0x830A7180;
    'dispatch: loop {
        match pc {
            0x830A7180 => {
    //   block [0x830A7180..0x830A7188)
	// 830A7180: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A7184: 8217FFD8  lwz r16, -0x28(r23)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(-40 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A7188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A7188 size=72
    let mut pc: u32 = 0x830A7188;
    'dispatch: loop {
        match pc {
            0x830A7188 => {
    //   block [0x830A7188..0x830A71D0)
	// 830A7188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A718C: 48100FE1  bl 0x831a816c
	ctx.lr = 0x830A7190;
	sub_831A8130(ctx, base);
	// 830A7190: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830A7194: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A7198: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A719C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 830A71A0: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 830A71A4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830A71A8: 4BF31D89  bl 0x82fd8f30
	ctx.lr = 0x830A71AC;
	sub_82FD8F30(ctx, base);
	// 830A71AC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A71B0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A71B4: 396BAD3C  addi r11, r11, -0x52c4
	ctx.r[11].s64 = ctx.r[11].s64 + -21188;
	// 830A71B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A71BC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A71C0: 4BF320F9  bl 0x82fd92b8
	ctx.lr = 0x830A71C4;
	sub_82FD92B8(ctx, base);
	// 830A71C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A71C8: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830A71CC: 48100FF0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A71D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A71D0 size=40
    let mut pc: u32 = 0x830A71D0;
    'dispatch: loop {
        match pc {
            0x830A71D0 => {
    //   block [0x830A71D0..0x830A71F8)
	// 830A71D0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830A71D4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A71D8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A71DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A71E0: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830A71E4: 4BF31C95  bl 0x82fd8e78
	ctx.lr = 0x830A71E8;
	sub_82FD8E78(ctx, base);
	// 830A71E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A71EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A71F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A71F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A71F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A71F8 size=120
    let mut pc: u32 = 0x830A71F8;
    'dispatch: loop {
        match pc {
            0x830A71F8 => {
    //   block [0x830A71F8..0x830A7270)
	// 830A71F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A71FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A7200: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A7204: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A7208: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A720C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A7210: 394B0008  addi r10, r11, 8
	ctx.r[10].s64 = ctx.r[11].s64 + 8;
	// 830A7214: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830A7218: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 830A721C: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 830A7220: 909F0004  stw r4, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 830A7224: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830A7228: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 830A722C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830A7230: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830A7234: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 830A7238: B17F0018  sth r11, 0x18(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u16 ) };
	// 830A723C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 830A7240: B17F0020  sth r11, 0x20(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u16 ) };
	// 830A7244: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 830A7248: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 830A724C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830A7250: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 830A7254: 4BF4E4F5  bl 0x82ff5748
	ctx.lr = 0x830A7258;
	sub_82FF5748(ctx, base);
	// 830A7258: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A725C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A7260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A7264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A7268: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A726C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A7270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A7270 size=8
    let mut pc: u32 = 0x830A7270;
    'dispatch: loop {
        match pc {
            0x830A7270 => {
    //   block [0x830A7270..0x830A7278)
	// 830A7270: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A7274: 82180090  lwz r16, 0x90(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(144 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A7278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A7278 size=132
    let mut pc: u32 = 0x830A7278;
    'dispatch: loop {
        match pc {
            0x830A7278 => {
    //   block [0x830A7278..0x830A72FC)
	// 830A7278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A727C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A7280: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A7284: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A7288: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830A728C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A7290: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A7294: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A7298: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 830A729C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830A72A0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A72A4: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A72A8: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A72AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A72B0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A72B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A72B8: 4E800421  bctrl
	ctx.lr = 0x830A72BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A72BC: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A72C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A72C4: 41820018  beq 0x830a72dc
	if ctx.cr[0].eq {
	pc = 0x830A72DC; continue 'dispatch;
	}
	// 830A72C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A72CC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830A72D0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A72D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A72D8: 4E800421  bctrl
	ctx.lr = 0x830A72DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A72DC: 387E0034  addi r3, r30, 0x34
	ctx.r[3].s64 = ctx.r[30].s64 + 52;
	// 830A72E0: 4BF4E4A9  bl 0x82ff5788
	ctx.lr = 0x830A72E4;
	sub_82FF5788(ctx, base);
	// 830A72E4: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830A72E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A72EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A72F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A72F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A72F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A72FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A72FC size=44
    let mut pc: u32 = 0x830A72FC;
    'dispatch: loop {
        match pc {
            0x830A72FC => {
    //   block [0x830A72FC..0x830A7328)
	// 830A72FC: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830A7300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A7304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A7308: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A730C: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830A7310: 386B0034  addi r3, r11, 0x34
	ctx.r[3].s64 = ctx.r[11].s64 + 52;
	// 830A7314: 4BF4E475  bl 0x82ff5788
	ctx.lr = 0x830A7318;
	sub_82FF5788(ctx, base);
	// 830A7318: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A731C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A7320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A7324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A7328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A7328 size=1432
    let mut pc: u32 = 0x830A7328;
    'dispatch: loop {
        match pc {
            0x830A7328 => {
    //   block [0x830A7328..0x830A78C0)
	// 830A7328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A732C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A7330: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A7334: 80E3001C  lwz r7, 0x1c(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A7338: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A733C: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 830A7340: 41980018  blt cr6, 0x830a7358
	if ctx.cr[6].lt {
	pc = 0x830A7358; continue 'dispatch;
	}
	// 830A7344: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 830A7348: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830A734C: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 830A7350: B1430020  sth r10, 0x20(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u16 ) };
	// 830A7354: 4800055C  b 0x830a78b0
	pc = 0x830A78B0; continue 'dispatch;
	// 830A7358: 81030028  lwz r8, 0x28(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A735C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A7360: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830A7364: A1230018  lhz r9, 0x18(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A7368: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 830A736C: 7D4A422E  lhzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 830A7370: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830A7374: 91430024  stw r10, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 830A7378: 409A017C  bne cr6, 0x830a74f4
	if !ctx.cr[6].eq {
	pc = 0x830A74F4; continue 'dispatch;
	}
	// 830A737C: 2F0A002D  cmpwi cr6, r10, 0x2d
	ctx.cr[6].compare_i32(ctx.r[10].s32, 45, &mut ctx.xer);
	// 830A7380: 419A012C  beq cr6, 0x830a74ac
	if ctx.cr[6].eq {
	pc = 0x830A74AC; continue 'dispatch;
	}
	// 830A7384: 2F0A005B  cmpwi cr6, r10, 0x5b
	ctx.cr[6].compare_i32(ctx.r[10].s32, 91, &mut ctx.xer);
	// 830A7388: 419A005C  beq cr6, 0x830a73e4
	if ctx.cr[6].eq {
	pc = 0x830A73E4; continue 'dispatch;
	}
	// 830A738C: 2F0A005C  cmpwi cr6, r10, 0x5c
	ctx.cr[6].compare_i32(ctx.r[10].s32, 92, &mut ctx.xer);
	// 830A7390: 409A0094  bne cr6, 0x830a7424
	if !ctx.cr[6].eq {
	pc = 0x830A7424; continue 'dispatch;
	}
	// 830A7394: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 830A7398: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 830A739C: 41980030  blt cr6, 0x830a73cc
	if ctx.cr[6].lt {
	pc = 0x830A73CC; continue 'dispatch;
	}
	// 830A73A0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A73A4: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A73A8: 38C0007A  li r6, 0x7a
	ctx.r[6].s64 = 122;
	// 830A73AC: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A73B0: 38A000E7  li r5, 0xe7
	ctx.r[5].s64 = 231;
	// 830A73B4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830A73B8: 4BFFFDD1  bl 0x830a7188
	ctx.lr = 0x830A73BC;
	sub_830A7188(ctx, base);
	// 830A73BC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A73C0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830A73C4: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A73C8: 48109861  bl 0x831b0c28
	ctx.lr = 0x830A73CC;
	sub_831B0C28(ctx, base);
	// 830A73CC: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830A73D0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830A73D4: 7D29422E  lhzx r9, r9, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 830A73D8: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830A73DC: 91230024  stw r9, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[9].u32 ) };
	// 830A73E0: 4BFFFF70  b 0x830a7350
	pc = 0x830A7350; continue 'dispatch;
	// 830A73E4: 3D208217  lis r9, -0x7de9
	ctx.r[9].s64 = -2112421888;
	// 830A73E8: 80C3000C  lwz r6, 0xc(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830A73EC: 8129ACB8  lwz r9, -0x5348(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-21320 as u32) ) } as u64;
	// 830A73F0: 7CC64838  and r6, r6, r9
	ctx.r[6].u64 = ctx.r[6].u64 & ctx.r[9].u64;
	// 830A73F4: 7D264850  subf r9, r6, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[6].s64;
	// 830A73F8: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 830A73FC: 5529DFFF  rlwinm. r9, r9, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 830A7400: 40820024  bne 0x830a7424
	if !ctx.cr[0].eq {
	pc = 0x830A7424; continue 'dispatch;
	}
	// 830A7404: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 830A7408: 4098001C  bge cr6, 0x830a7424
	if !ctx.cr[6].lt {
	pc = 0x830A7424; continue 'dispatch;
	}
	// 830A740C: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830A7410: 7D29422E  lhzx r9, r9, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 830A7414: 2B09003A  cmplwi cr6, r9, 0x3a
	ctx.cr[6].compare_u32(ctx.r[9].u32, 58 as u32, &mut ctx.xer);
	// 830A7418: 409A000C  bne cr6, 0x830a7424
	if !ctx.cr[6].eq {
	pc = 0x830A7424; continue 'dispatch;
	}
	// 830A741C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 830A7420: 480000C8  b 0x830a74e8
	pc = 0x830A74E8; continue 'dispatch;
	// 830A7424: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 830A7428: 554B042A  rlwinm r11, r10, 0, 0x10, 0x15
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830A742C: 6129D800  ori r9, r9, 0xd800
	ctx.r[9].u64 = ctx.r[9].u64 | 55296;
	// 830A7430: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 830A7434: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830A7438: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A743C: 41820050  beq 0x830a748c
	if ctx.cr[0].eq {
	pc = 0x830A748C; continue 'dispatch;
	}
	// 830A7440: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A7444: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 830A7448: 40980044  bge cr6, 0x830a748c
	if !ctx.cr[6].lt {
	pc = 0x830A748C; continue 'dispatch;
	}
	// 830A744C: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830A7450: 3CE00000  lis r7, 0
	ctx.r[7].s64 = 0;
	// 830A7454: 60E7DC00  ori r7, r7, 0xdc00
	ctx.r[7].u64 = ctx.r[7].u64 | 56320;
	// 830A7458: 7D29422E  lhzx r9, r9, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 830A745C: 5528002A  rlwinm r8, r9, 0, 0, 0x15
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 830A7460: 7D083850  subf r8, r8, r7
	ctx.r[8].s64 = ctx.r[7].s64 - ctx.r[8].s64;
	// 830A7464: 7D080034  cntlzw r8, r8
	ctx.r[8].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 830A7468: 5508DFFF  rlwinm. r8, r8, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 830A746C: 41820028  beq 0x830a7494
	if ctx.cr[0].eq {
	pc = 0x830A7494; continue 'dispatch;
	}
	// 830A7470: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 830A7474: 3D6AFFFF  addis r11, r10, -1
	ctx.r[11].s64 = ctx.r[10].s64 + -65536;
	// 830A7478: 396B2809  addi r11, r11, 0x2809
	ctx.r[11].s64 = ctx.r[11].s64 + 10249;
	// 830A747C: 556B502A  slwi r11, r11, 0xa
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(10);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A7480: 91030010  stw r8, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 830A7484: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 830A7488: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 830A748C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A7490: 4BFFFEC0  b 0x830a7350
	pc = 0x830A7350; continue 'dispatch;
	// 830A7494: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A7498: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A749C: 388BC6CC  addi r4, r11, -0x3934
	ctx.r[4].s64 = ctx.r[11].s64 + -14644;
	// 830A74A0: 396000F9  li r11, 0xf9
	ctx.r[11].s64 = 249;
	// 830A74A4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830A74A8: 48109781  bl 0x831b0c28
	ctx.lr = 0x830A74AC;
	sub_831B0C28(ctx, base);
	// 830A74AC: 3D408217  lis r10, -0x7de9
	ctx.r[10].s64 = -2112421888;
	// 830A74B0: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830A74B4: 814AACB8  lwz r10, -0x5348(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-21320 as u32) ) } as u64;
	// 830A74B8: 7D295038  and r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 830A74BC: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 830A74C0: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 830A74C4: 554ADFFF  rlwinm. r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830A74C8: 4182FFC4  beq 0x830a748c
	if ctx.cr[0].eq {
	pc = 0x830A748C; continue 'dispatch;
	}
	// 830A74CC: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 830A74D0: 4098FFBC  bge cr6, 0x830a748c
	if !ctx.cr[6].lt {
	pc = 0x830A748C; continue 'dispatch;
	}
	// 830A74D4: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A74D8: 7D4A422E  lhzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 830A74DC: 2B0A005B  cmplwi cr6, r10, 0x5b
	ctx.cr[6].compare_u32(ctx.r[10].u32, 91 as u32, &mut ctx.xer);
	// 830A74E0: 409AFFAC  bne cr6, 0x830a748c
	if !ctx.cr[6].eq {
	pc = 0x830A748C; continue 'dispatch;
	}
	// 830A74E4: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 830A74E8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830A74EC: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830A74F0: 4BFFFE60  b 0x830a7350
	pc = 0x830A7350; continue 'dispatch;
	// 830A74F4: 2F0A002E  cmpwi cr6, r10, 0x2e
	ctx.cr[6].compare_i32(ctx.r[10].s32, 46, &mut ctx.xer);
	// 830A74F8: 4199029C  bgt cr6, 0x830a7794
	if ctx.cr[6].gt {
	pc = 0x830A7794; continue 'dispatch;
	}
	// 830A74FC: 419A0290  beq cr6, 0x830a778c
	if ctx.cr[6].eq {
	pc = 0x830A778C; continue 'dispatch;
	}
	// 830A7500: 2F0A0024  cmpwi cr6, r10, 0x24
	ctx.cr[6].compare_i32(ctx.r[10].s32, 36, &mut ctx.xer);
	// 830A7504: 419A0280  beq cr6, 0x830a7784
	if ctx.cr[6].eq {
	pc = 0x830A7784; continue 'dispatch;
	}
	// 830A7508: 2F0A0028  cmpwi cr6, r10, 0x28
	ctx.cr[6].compare_i32(ctx.r[10].s32, 40, &mut ctx.xer);
	// 830A750C: 419A0034  beq cr6, 0x830a7540
	if ctx.cr[6].eq {
	pc = 0x830A7540; continue 'dispatch;
	}
	// 830A7510: 2F0A0029  cmpwi cr6, r10, 0x29
	ctx.cr[6].compare_i32(ctx.r[10].s32, 41, &mut ctx.xer);
	// 830A7514: 419A0024  beq cr6, 0x830a7538
	if ctx.cr[6].eq {
	pc = 0x830A7538; continue 'dispatch;
	}
	// 830A7518: 2F0A002A  cmpwi cr6, r10, 0x2a
	ctx.cr[6].compare_i32(ctx.r[10].s32, 42, &mut ctx.xer);
	// 830A751C: 419A0014  beq cr6, 0x830a7530
	if ctx.cr[6].eq {
	pc = 0x830A7530; continue 'dispatch;
	}
	// 830A7520: 2F0A002B  cmpwi cr6, r10, 0x2b
	ctx.cr[6].compare_i32(ctx.r[10].s32, 43, &mut ctx.xer);
	// 830A7524: 409A0298  bne cr6, 0x830a77bc
	if !ctx.cr[6].eq {
	pc = 0x830A77BC; continue 'dispatch;
	}
	// 830A7528: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 830A752C: 48000380  b 0x830a78ac
	pc = 0x830A78AC; continue 'dispatch;
	// 830A7530: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 830A7534: 48000378  b 0x830a78ac
	pc = 0x830A78AC; continue 'dispatch;
	// 830A7538: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 830A753C: 48000370  b 0x830a78ac
	pc = 0x830A78AC; continue 'dispatch;
	// 830A7540: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 830A7544: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 830A7548: 40980364  bge cr6, 0x830a78ac
	if !ctx.cr[6].lt {
	pc = 0x830A78AC; continue 'dispatch;
	}
	// 830A754C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A7550: 7D4A422E  lhzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 830A7554: 2B0A003F  cmplwi cr6, r10, 0x3f
	ctx.cr[6].compare_u32(ctx.r[10].u32, 63 as u32, &mut ctx.xer);
	// 830A7558: 409A0354  bne cr6, 0x830a78ac
	if !ctx.cr[6].eq {
	pc = 0x830A78AC; continue 'dispatch;
	}
	// 830A755C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830A7560: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 830A7564: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830A7568: 41980030  blt cr6, 0x830a7598
	if ctx.cr[6].lt {
	pc = 0x830A7598; continue 'dispatch;
	}
	// 830A756C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A7570: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A7574: 38C0007B  li r6, 0x7b
	ctx.r[6].s64 = 123;
	// 830A7578: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A757C: 38A0013A  li r5, 0x13a
	ctx.r[5].s64 = 314;
	// 830A7580: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830A7584: 4BFFFC05  bl 0x830a7188
	ctx.lr = 0x830A7588;
	sub_830A7188(ctx, base);
	// 830A7588: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A758C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830A7590: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A7594: 48109695  bl 0x831b0c28
	ctx.lr = 0x830A7598;
	sub_831B0C28(ctx, base);
	// 830A7598: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830A759C: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 830A75A0: 7D29422E  lhzx r9, r9, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 830A75A4: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 830A75A8: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 830A75AC: 2F0B0021  cmpwi cr6, r11, 0x21
	ctx.cr[6].compare_i32(ctx.r[11].s32, 33, &mut ctx.xer);
	// 830A75B0: 419A01CC  beq cr6, 0x830a777c
	if ctx.cr[6].eq {
	pc = 0x830A777C; continue 'dispatch;
	}
	// 830A75B4: 2F0B0023  cmpwi cr6, r11, 0x23
	ctx.cr[6].compare_i32(ctx.r[11].s32, 35, &mut ctx.xer);
	// 830A75B8: 419A0150  beq cr6, 0x830a7708
	if ctx.cr[6].eq {
	pc = 0x830A7708; continue 'dispatch;
	}
	// 830A75BC: 2F0B003A  cmpwi cr6, r11, 0x3a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 58, &mut ctx.xer);
	// 830A75C0: 419A0140  beq cr6, 0x830a7700
	if ctx.cr[6].eq {
	pc = 0x830A7700; continue 'dispatch;
	}
	// 830A75C4: 2F0B003C  cmpwi cr6, r11, 0x3c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 60, &mut ctx.xer);
	// 830A75C8: 419A00A8  beq cr6, 0x830a7670
	if ctx.cr[6].eq {
	pc = 0x830A7670; continue 'dispatch;
	}
	// 830A75CC: 2F0B003D  cmpwi cr6, r11, 0x3d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 61, &mut ctx.xer);
	// 830A75D0: 419A0098  beq cr6, 0x830a7668
	if ctx.cr[6].eq {
	pc = 0x830A7668; continue 'dispatch;
	}
	// 830A75D4: 2F0B003E  cmpwi cr6, r11, 0x3e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 62, &mut ctx.xer);
	// 830A75D8: 419A0088  beq cr6, 0x830a7660
	if ctx.cr[6].eq {
	pc = 0x830A7660; continue 'dispatch;
	}
	// 830A75DC: 2F0B005B  cmpwi cr6, r11, 0x5b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 91, &mut ctx.xer);
	// 830A75E0: 419A0078  beq cr6, 0x830a7658
	if ctx.cr[6].eq {
	pc = 0x830A7658; continue 'dispatch;
	}
	// 830A75E4: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 830A75E8: 419A0060  beq cr6, 0x830a7648
	if ctx.cr[6].eq {
	pc = 0x830A7648; continue 'dispatch;
	}
	// 830A75EC: 2B0B0061  cmplwi cr6, r11, 0x61
	ctx.cr[6].compare_u32(ctx.r[11].u32, 97 as u32, &mut ctx.xer);
	// 830A75F0: 4198000C  blt cr6, 0x830a75fc
	if ctx.cr[6].lt {
	pc = 0x830A75FC; continue 'dispatch;
	}
	// 830A75F4: 2B0B007A  cmplwi cr6, r11, 0x7a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 122 as u32, &mut ctx.xer);
	// 830A75F8: 40990050  ble cr6, 0x830a7648
	if !ctx.cr[6].gt {
	pc = 0x830A7648; continue 'dispatch;
	}
	// 830A75FC: 2B0B0041  cmplwi cr6, r11, 0x41
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65 as u32, &mut ctx.xer);
	// 830A7600: 4198000C  blt cr6, 0x830a760c
	if ctx.cr[6].lt {
	pc = 0x830A760C; continue 'dispatch;
	}
	// 830A7604: 2B0B005A  cmplwi cr6, r11, 0x5a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 90 as u32, &mut ctx.xer);
	// 830A7608: 40990040  ble cr6, 0x830a7648
	if !ctx.cr[6].gt {
	pc = 0x830A7648; continue 'dispatch;
	}
	// 830A760C: 2B0B0028  cmplwi cr6, r11, 0x28
	ctx.cr[6].compare_u32(ctx.r[11].u32, 40 as u32, &mut ctx.xer);
	// 830A7610: 409A000C  bne cr6, 0x830a761c
	if !ctx.cr[6].eq {
	pc = 0x830A761C; continue 'dispatch;
	}
	// 830A7614: 39200017  li r9, 0x17
	ctx.r[9].s64 = 23;
	// 830A7618: 48000294  b 0x830a78ac
	pc = 0x830A78AC; continue 'dispatch;
	// 830A761C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A7620: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A7624: 38C0007B  li r6, 0x7b
	ctx.r[6].s64 = 123;
	// 830A7628: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A762C: 38A00177  li r5, 0x177
	ctx.r[5].s64 = 375;
	// 830A7630: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 830A7634: 4BFFFB55  bl 0x830a7188
	ctx.lr = 0x830A7638;
	sub_830A7188(ctx, base);
	// 830A7638: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A763C: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 830A7640: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A7644: 481095E5  bl 0x831b0c28
	ctx.lr = 0x830A7648;
	sub_831B0C28(ctx, base);
	// 830A7648: 396AFFFF  addi r11, r10, -1
	ctx.r[11].s64 = ctx.r[10].s64 + -1;
	// 830A764C: 39200016  li r9, 0x16
	ctx.r[9].s64 = 22;
	// 830A7650: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830A7654: 48000258  b 0x830a78ac
	pc = 0x830A78AC; continue 'dispatch;
	// 830A7658: 39200013  li r9, 0x13
	ctx.r[9].s64 = 19;
	// 830A765C: 48000250  b 0x830a78ac
	pc = 0x830A78AC; continue 'dispatch;
	// 830A7660: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 830A7664: 48000248  b 0x830a78ac
	pc = 0x830A78AC; continue 'dispatch;
	// 830A7668: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 830A766C: 48000240  b 0x830a78ac
	pc = 0x830A78AC; continue 'dispatch;
	// 830A7670: 7F0A3800  cmpw cr6, r10, r7
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[7].s32, &mut ctx.xer);
	// 830A7674: 41980030  blt cr6, 0x830a76a4
	if ctx.cr[6].lt {
	pc = 0x830A76A4; continue 'dispatch;
	}
	// 830A7678: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A767C: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A7680: 38C0007B  li r6, 0x7b
	ctx.r[6].s64 = 123;
	// 830A7684: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A7688: 38A00150  li r5, 0x150
	ctx.r[5].s64 = 336;
	// 830A768C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 830A7690: 4BFFFAF9  bl 0x830a7188
	ctx.lr = 0x830A7694;
	sub_830A7188(ctx, base);
	// 830A7694: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A7698: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 830A769C: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A76A0: 48109589  bl 0x831b0c28
	ctx.lr = 0x830A76A4;
	sub_831B0C28(ctx, base);
	// 830A76A4: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A76A8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830A76AC: 7D6B422E  lhzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 830A76B0: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 830A76B4: 2B0B003D  cmplwi cr6, r11, 0x3d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 61 as u32, &mut ctx.xer);
	// 830A76B8: 409A000C  bne cr6, 0x830a76c4
	if !ctx.cr[6].eq {
	pc = 0x830A76C4; continue 'dispatch;
	}
	// 830A76BC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 830A76C0: 480001EC  b 0x830a78ac
	pc = 0x830A78AC; continue 'dispatch;
	// 830A76C4: 2B0B0021  cmplwi cr6, r11, 0x21
	ctx.cr[6].compare_u32(ctx.r[11].u32, 33 as u32, &mut ctx.xer);
	// 830A76C8: 409A000C  bne cr6, 0x830a76d4
	if !ctx.cr[6].eq {
	pc = 0x830A76D4; continue 'dispatch;
	}
	// 830A76CC: 39200011  li r9, 0x11
	ctx.r[9].s64 = 17;
	// 830A76D0: 480001DC  b 0x830a78ac
	pc = 0x830A78AC; continue 'dispatch;
	// 830A76D4: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A76D8: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A76DC: 38C0007C  li r6, 0x7c
	ctx.r[6].s64 = 124;
	// 830A76E0: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A76E4: 38A0015B  li r5, 0x15b
	ctx.r[5].s64 = 347;
	// 830A76E8: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 830A76EC: 4BFFFA9D  bl 0x830a7188
	ctx.lr = 0x830A76F0;
	sub_830A7188(ctx, base);
	// 830A76F0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A76F4: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 830A76F8: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A76FC: 4810952D  bl 0x831b0c28
	ctx.lr = 0x830A7700;
	sub_831B0C28(ctx, base);
	// 830A7700: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 830A7704: 480001A8  b 0x830a78ac
	pc = 0x830A78AC; continue 'dispatch;
	// 830A7708: 7F0A3800  cmpw cr6, r10, r7
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[7].s32, &mut ctx.xer);
	// 830A770C: 40980030  bge cr6, 0x830a773c
	if !ctx.cr[6].lt {
	pc = 0x830A773C; continue 'dispatch;
	}
	// 830A7710: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A7714: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A7718: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830A771C: 7D2A422E  lhzx r9, r10, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 830A7720: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830A7724: 2B090029  cmplwi cr6, r9, 0x29
	ctx.cr[6].compare_u32(ctx.r[9].u32, 41 as u32, &mut ctx.xer);
	// 830A7728: 419A0014  beq cr6, 0x830a773c
	if ctx.cr[6].eq {
	pc = 0x830A773C; continue 'dispatch;
	}
	// 830A772C: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A7730: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A7734: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 830A7738: 4198FFD8  blt cr6, 0x830a7710
	if ctx.cr[6].lt {
	pc = 0x830A7710; continue 'dispatch;
	}
	// 830A773C: 552B043E  clrlwi r11, r9, 0x10
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 830A7740: 2B0B0029  cmplwi cr6, r11, 0x29
	ctx.cr[6].compare_u32(ctx.r[11].u32, 41 as u32, &mut ctx.xer);
	// 830A7744: 419A0030  beq cr6, 0x830a7774
	if ctx.cr[6].eq {
	pc = 0x830A7774; continue 'dispatch;
	}
	// 830A7748: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A774C: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A7750: 38C0007D  li r6, 0x7d
	ctx.r[6].s64 = 125;
	// 830A7754: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A7758: 38A00167  li r5, 0x167
	ctx.r[5].s64 = 359;
	// 830A775C: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 830A7760: 4BFFFA29  bl 0x830a7188
	ctx.lr = 0x830A7764;
	sub_830A7188(ctx, base);
	// 830A7764: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A7768: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 830A776C: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A7770: 481094B9  bl 0x831b0c28
	ctx.lr = 0x830A7774;
	sub_831B0C28(ctx, base);
	// 830A7774: 39200015  li r9, 0x15
	ctx.r[9].s64 = 21;
	// 830A7778: 48000134  b 0x830a78ac
	pc = 0x830A78AC; continue 'dispatch;
	// 830A777C: 3920000F  li r9, 0xf
	ctx.r[9].s64 = 15;
	// 830A7780: 4800012C  b 0x830a78ac
	pc = 0x830A78AC; continue 'dispatch;
	// 830A7784: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 830A7788: 48000124  b 0x830a78ac
	pc = 0x830A78AC; continue 'dispatch;
	// 830A778C: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 830A7790: 4800011C  b 0x830a78ac
	pc = 0x830A78AC; continue 'dispatch;
	// 830A7794: 2F0A003F  cmpwi cr6, r10, 0x3f
	ctx.cr[6].compare_i32(ctx.r[10].s32, 63, &mut ctx.xer);
	// 830A7798: 419A0110  beq cr6, 0x830a78a8
	if ctx.cr[6].eq {
	pc = 0x830A78A8; continue 'dispatch;
	}
	// 830A779C: 2F0A005B  cmpwi cr6, r10, 0x5b
	ctx.cr[6].compare_i32(ctx.r[10].s32, 91, &mut ctx.xer);
	// 830A77A0: 419A0100  beq cr6, 0x830a78a0
	if ctx.cr[6].eq {
	pc = 0x830A78A0; continue 'dispatch;
	}
	// 830A77A4: 2F0A005C  cmpwi cr6, r10, 0x5c
	ctx.cr[6].compare_i32(ctx.r[10].s32, 92, &mut ctx.xer);
	// 830A77A8: 419A00A8  beq cr6, 0x830a7850
	if ctx.cr[6].eq {
	pc = 0x830A7850; continue 'dispatch;
	}
	// 830A77AC: 2F0A005E  cmpwi cr6, r10, 0x5e
	ctx.cr[6].compare_i32(ctx.r[10].s32, 94, &mut ctx.xer);
	// 830A77B0: 419A0098  beq cr6, 0x830a7848
	if ctx.cr[6].eq {
	pc = 0x830A7848; continue 'dispatch;
	}
	// 830A77B4: 2F0A007C  cmpwi cr6, r10, 0x7c
	ctx.cr[6].compare_i32(ctx.r[10].s32, 124, &mut ctx.xer);
	// 830A77B8: 419A0088  beq cr6, 0x830a7840
	if ctx.cr[6].eq {
	pc = 0x830A7840; continue 'dispatch;
	}
	// 830A77BC: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 830A77C0: 5546042A  rlwinm r6, r10, 0, 0x10, 0x15
	ctx.r[6].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830A77C4: 6125D800  ori r5, r9, 0xd800
	ctx.r[5].u64 = ctx.r[9].u64 | 55296;
	// 830A77C8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830A77CC: 7CC62850  subf r6, r6, r5
	ctx.r[6].s64 = ctx.r[5].s64 - ctx.r[6].s64;
	// 830A77D0: 7CC60034  cntlzw r6, r6
	ctx.r[6].u64 = if ctx.r[6].u32 == 0 { 32 } else { ctx.r[6].u32.leading_zeros() as u64 };
	// 830A77D4: 54C6DFFF  rlwinm. r6, r6, 0x1b, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[6].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 830A77D8: 418200D4  beq 0x830a78ac
	if ctx.cr[0].eq {
	pc = 0x830A78AC; continue 'dispatch;
	}
	// 830A77DC: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 830A77E0: 409800CC  bge cr6, 0x830a78ac
	if !ctx.cr[6].lt {
	pc = 0x830A78AC; continue 'dispatch;
	}
	// 830A77E4: 5567083C  slwi r7, r11, 1
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 830A77E8: 3CC00000  lis r6, 0
	ctx.r[6].s64 = 0;
	// 830A77EC: 60C6DC00  ori r6, r6, 0xdc00
	ctx.r[6].u64 = ctx.r[6].u64 | 56320;
	// 830A77F0: 7D07422E  lhzx r8, r7, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 830A77F4: 5507002A  rlwinm r7, r8, 0, 0, 0x15
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 830A77F8: 7CE73050  subf r7, r7, r6
	ctx.r[7].s64 = ctx.r[6].s64 - ctx.r[7].s64;
	// 830A77FC: 7CE70034  cntlzw r7, r7
	ctx.r[7].u64 = if ctx.r[7].u32 == 0 { 32 } else { ctx.r[7].u32.leading_zeros() as u64 };
	// 830A7800: 54E7DFFF  rlwinm. r7, r7, 0x1b, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 830A7804: 41820024  beq 0x830a7828
	if ctx.cr[0].eq {
	pc = 0x830A7828; continue 'dispatch;
	}
	// 830A7808: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 830A780C: 3D6AFFFF  addis r11, r10, -1
	ctx.r[11].s64 = ctx.r[10].s64 + -65536;
	// 830A7810: 396B2809  addi r11, r11, 0x2809
	ctx.r[11].s64 = ctx.r[11].s64 + 10249;
	// 830A7814: 556B502A  slwi r11, r11, 0xa
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(10);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A7818: 90E30010  stw r7, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 830A781C: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 830A7820: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 830A7824: 48000088  b 0x830a78ac
	pc = 0x830A78AC; continue 'dispatch;
	// 830A7828: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A782C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A7830: 388BC6CC  addi r4, r11, -0x3934
	ctx.r[4].s64 = ctx.r[11].s64 + -14644;
	// 830A7834: 396000F9  li r11, 0xf9
	ctx.r[11].s64 = 249;
	// 830A7838: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830A783C: 481093ED  bl 0x831b0c28
	ctx.lr = 0x830A7840;
	sub_831B0C28(ctx, base);
	// 830A7840: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 830A7844: 48000068  b 0x830a78ac
	pc = 0x830A78AC; continue 'dispatch;
	// 830A7848: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 830A784C: 48000060  b 0x830a78ac
	pc = 0x830A78AC; continue 'dispatch;
	// 830A7850: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 830A7854: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 830A7858: 41980030  blt cr6, 0x830a7888
	if ctx.cr[6].lt {
	pc = 0x830A7888; continue 'dispatch;
	}
	// 830A785C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A7860: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A7864: 38C0007A  li r6, 0x7a
	ctx.r[6].s64 = 122;
	// 830A7868: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A786C: 38A0017E  li r5, 0x17e
	ctx.r[5].s64 = 382;
	// 830A7870: 38610100  addi r3, r1, 0x100
	ctx.r[3].s64 = ctx.r[1].s64 + 256;
	// 830A7874: 4BFFF915  bl 0x830a7188
	ctx.lr = 0x830A7878;
	sub_830A7188(ctx, base);
	// 830A7878: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A787C: 38610100  addi r3, r1, 0x100
	ctx.r[3].s64 = ctx.r[1].s64 + 256;
	// 830A7880: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A7884: 481093A5  bl 0x831b0c28
	ctx.lr = 0x830A7888;
	sub_831B0C28(ctx, base);
	// 830A7888: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A788C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830A7890: 7D4A422E  lhzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 830A7894: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830A7898: 91430024  stw r10, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 830A789C: 48000010  b 0x830a78ac
	pc = 0x830A78AC; continue 'dispatch;
	// 830A78A0: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 830A78A4: 48000008  b 0x830a78ac
	pc = 0x830A78AC; continue 'dispatch;
	// 830A78A8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 830A78AC: B1230020  sth r9, 0x20(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[9].u16 ) };
	// 830A78B0: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 830A78B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A78B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A78BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A78C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A78C0 size=52
    let mut pc: u32 = 0x830A78C0;
    'dispatch: loop {
        match pc {
            0x830A78C0 => {
    //   block [0x830A78C0..0x830A78F4)
	// 830A78C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A78C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A78C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A78CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A78D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A78D4: 4BFFFA55  bl 0x830a7328
	ctx.lr = 0x830A78D8;
	sub_830A7328(ctx, base);
	// 830A78D8: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A78DC: 4BFFC3B5  bl 0x830a3c90
	ctx.lr = 0x830A78E0;
	sub_830A3C90(ctx, base);
	// 830A78E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A78E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A78E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A78EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A78F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A78F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A78F8 size=52
    let mut pc: u32 = 0x830A78F8;
    'dispatch: loop {
        match pc {
            0x830A78F8 => {
    //   block [0x830A78F8..0x830A792C)
	// 830A78F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A78FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A7900: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A7904: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A7908: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A790C: 4BFFFA1D  bl 0x830a7328
	ctx.lr = 0x830A7910;
	sub_830A7328(ctx, base);
	// 830A7910: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A7914: 4BFFC3C5  bl 0x830a3cd8
	ctx.lr = 0x830A7918;
	sub_830A3CD8(ctx, base);
	// 830A7918: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A791C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A7920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A7924: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A7928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A7930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A7930 size=52
    let mut pc: u32 = 0x830A7930;
    'dispatch: loop {
        match pc {
            0x830A7930 => {
    //   block [0x830A7930..0x830A7964)
	// 830A7930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A7934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A7938: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A793C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A7940: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A7944: 4BFFF9E5  bl 0x830a7328
	ctx.lr = 0x830A7948;
	sub_830A7328(ctx, base);
	// 830A7948: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A794C: 4BFFC3D5  bl 0x830a3d20
	ctx.lr = 0x830A7950;
	sub_830A3D20(ctx, base);
	// 830A7950: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A7954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A7958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A795C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A7960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A7968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A7968 size=52
    let mut pc: u32 = 0x830A7968;
    'dispatch: loop {
        match pc {
            0x830A7968 => {
    //   block [0x830A7968..0x830A799C)
	// 830A7968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A796C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A7970: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A7974: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A7978: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A797C: 4BFFF9AD  bl 0x830a7328
	ctx.lr = 0x830A7980;
	sub_830A7328(ctx, base);
	// 830A7980: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A7984: 4BFFC42D  bl 0x830a3db0
	ctx.lr = 0x830A7988;
	sub_830A3DB0(ctx, base);
	// 830A7988: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A798C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A7990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A7994: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A7998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A79A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A79A0 size=52
    let mut pc: u32 = 0x830A79A0;
    'dispatch: loop {
        match pc {
            0x830A79A0 => {
    //   block [0x830A79A0..0x830A79D4)
	// 830A79A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A79A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A79A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A79AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A79B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A79B4: 4BFFF975  bl 0x830a7328
	ctx.lr = 0x830A79B8;
	sub_830A7328(ctx, base);
	// 830A79B8: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A79BC: 4BFFC3AD  bl 0x830a3d68
	ctx.lr = 0x830A79C0;
	sub_830A3D68(ctx, base);
	// 830A79C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A79C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A79C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A79CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A79D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A79D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A79D8 size=52
    let mut pc: u32 = 0x830A79D8;
    'dispatch: loop {
        match pc {
            0x830A79D8 => {
    //   block [0x830A79D8..0x830A7A0C)
	// 830A79D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A79DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A79E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A79E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A79E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A79EC: 4BFFF93D  bl 0x830a7328
	ctx.lr = 0x830A79F0;
	sub_830A7328(ctx, base);
	// 830A79F0: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A79F4: 4BFFC405  bl 0x830a3df8
	ctx.lr = 0x830A79F8;
	sub_830A3DF8(ctx, base);
	// 830A79F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A79FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A7A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A7A04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A7A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A7A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A7A10 size=52
    let mut pc: u32 = 0x830A7A10;
    'dispatch: loop {
        match pc {
            0x830A7A10 => {
    //   block [0x830A7A10..0x830A7A44)
	// 830A7A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A7A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A7A18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A7A1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A7A20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A7A24: 4BFFF905  bl 0x830a7328
	ctx.lr = 0x830A7A28;
	sub_830A7328(ctx, base);
	// 830A7A28: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A7A2C: 4BFFC415  bl 0x830a3e40
	ctx.lr = 0x830A7A30;
	sub_830A3E40(ctx, base);
	// 830A7A30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A7A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A7A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A7A3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A7A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A7A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A7A48 size=52
    let mut pc: u32 = 0x830A7A48;
    'dispatch: loop {
        match pc {
            0x830A7A48 => {
    //   block [0x830A7A48..0x830A7A7C)
	// 830A7A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A7A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A7A50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A7A54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A7A58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A7A5C: 4BFFF8CD  bl 0x830a7328
	ctx.lr = 0x830A7A60;
	sub_830A7328(ctx, base);
	// 830A7A60: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A7A64: 4BFFC425  bl 0x830a3e88
	ctx.lr = 0x830A7A68;
	sub_830A3E88(ctx, base);
	// 830A7A68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A7A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A7A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A7A74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A7A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A7A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A7A80 size=52
    let mut pc: u32 = 0x830A7A80;
    'dispatch: loop {
        match pc {
            0x830A7A80 => {
    //   block [0x830A7A80..0x830A7AB4)
	// 830A7A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A7A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A7A88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A7A8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A7A90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A7A94: 4BFFF895  bl 0x830a7328
	ctx.lr = 0x830A7A98;
	sub_830A7328(ctx, base);
	// 830A7A98: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A7A9C: 4BFFC435  bl 0x830a3ed0
	ctx.lr = 0x830A7AA0;
	sub_830A3ED0(ctx, base);
	// 830A7AA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A7AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A7AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A7AAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A7AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A7AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A7AB8 size=100
    let mut pc: u32 = 0x830A7AB8;
    'dispatch: loop {
        match pc {
            0x830A7AB8 => {
    //   block [0x830A7AB8..0x830A7B1C)
	// 830A7AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A7ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A7AC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A7AC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A7AC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A7ACC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A7AD0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A7AD4: 4BFFF855  bl 0x830a7328
	ctx.lr = 0x830A7AD8;
	sub_830A7328(ctx, base);
	// 830A7AD8: A17F0020  lhz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A7ADC: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 830A7AE0: 409A0014  bne cr6, 0x830a7af4
	if !ctx.cr[6].eq {
	pc = 0x830A7AF4; continue 'dispatch;
	}
	// 830A7AE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A7AE8: 4BFFF841  bl 0x830a7328
	ctx.lr = 0x830A7AEC;
	sub_830A7328(ctx, base);
	// 830A7AEC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830A7AF0: 48000008  b 0x830a7af8
	pc = 0x830A7AF8; continue 'dispatch;
	// 830A7AF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A7AF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A7AFC: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A7B00: 4BFFB9B1  bl 0x830a34b0
	ctx.lr = 0x830A7B04;
	sub_830A34B0(ctx, base);
	// 830A7B04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A7B08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A7B0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A7B10: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A7B14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A7B18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A7B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A7B20 size=116
    let mut pc: u32 = 0x830A7B20;
    'dispatch: loop {
        match pc {
            0x830A7B20 => {
    //   block [0x830A7B20..0x830A7B94)
	// 830A7B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A7B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A7B28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A7B2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A7B30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A7B34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A7B38: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A7B3C: 4BFFF7ED  bl 0x830a7328
	ctx.lr = 0x830A7B40;
	sub_830A7328(ctx, base);
	// 830A7B40: A17F0020  lhz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A7B44: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 830A7B48: 409A0014  bne cr6, 0x830a7b5c
	if !ctx.cr[6].eq {
	pc = 0x830A7B5C; continue 'dispatch;
	}
	// 830A7B4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A7B50: 4BFFF7D9  bl 0x830a7328
	ctx.lr = 0x830A7B54;
	sub_830A7328(ctx, base);
	// 830A7B54: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830A7B58: 48000008  b 0x830a7b60
	pc = 0x830A7B60; continue 'dispatch;
	// 830A7B5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A7B60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A7B64: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A7B68: 4BFFB949  bl 0x830a34b0
	ctx.lr = 0x830A7B6C;
	sub_830A34B0(ctx, base);
	// 830A7B6C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830A7B70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A7B74: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A7B78: 4BFFBA49  bl 0x830a35c0
	ctx.lr = 0x830A7B7C;
	sub_830A35C0(ctx, base);
	// 830A7B7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A7B80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A7B84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A7B88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A7B8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A7B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A7B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A7B98 size=208
    let mut pc: u32 = 0x830A7B98;
    'dispatch: loop {
        match pc {
            0x830A7B98 => {
    //   block [0x830A7B98..0x830A7C68)
	// 830A7B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A7B9C: 481005C9  bl 0x831a8164
	ctx.lr = 0x830A7BA0;
	sub_831A8130(ctx, base);
	// 830A7BA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A7BA4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A7BA8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830A7BAC: 4BFFF77D  bl 0x830a7328
	ctx.lr = 0x830A7BB0;
	sub_830A7328(ctx, base);
	// 830A7BB0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A7BB4: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A7BB8: 4BFFBAB1  bl 0x830a3668
	ctx.lr = 0x830A7BBC;
	sub_830A3668(ctx, base);
	// 830A7BBC: A17E0020  lhz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A7BC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A7BC4: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 830A7BC8: 409A004C  bne cr6, 0x830a7c14
	if !ctx.cr[6].eq {
	pc = 0x830A7C14; continue 'dispatch;
	}
	// 830A7BCC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A7BD0: 4BFFF759  bl 0x830a7328
	ctx.lr = 0x830A7BD4;
	sub_830A7328(ctx, base);
	// 830A7BD4: 83BE0030  lwz r29, 0x30(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A7BD8: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 830A7BDC: 837F0000  lwz r27, 0(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A7BE0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A7BE4: 4BFFB6BD  bl 0x830a32a0
	ctx.lr = 0x830A7BE8;
	sub_830A32A0(ctx, base);
	// 830A7BE8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A7BEC: 817B0044  lwz r11, 0x44(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(68 as u32) ) } as u64;
	// 830A7BF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A7BF4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A7BF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A7BFC: 4E800421  bctrl
	ctx.lr = 0x830A7C00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A7C00: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A7C04: 80BE0030  lwz r5, 0x30(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A7C08: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830A7C0C: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 830A7C10: 48000040  b 0x830a7c50
	pc = 0x830A7C50; continue 'dispatch;
	// 830A7C14: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A7C18: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830A7C1C: 80BE0030  lwz r5, 0x30(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A7C20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A7C24: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 830A7C28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A7C2C: 4E800421  bctrl
	ctx.lr = 0x830A7C30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A7C30: 83DE0030  lwz r30, 0x30(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A7C34: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 830A7C38: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A7C3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A7C40: 4BFFB661  bl 0x830a32a0
	ctx.lr = 0x830A7C44;
	sub_830A32A0(ctx, base);
	// 830A7C44: 817D0044  lwz r11, 0x44(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(68 as u32) ) } as u64;
	// 830A7C48: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A7C4C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830A7C50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A7C54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A7C58: 4E800421  bctrl
	ctx.lr = 0x830A7C5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A7C5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A7C60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830A7C64: 48100550  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A7C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A7C68 size=160
    let mut pc: u32 = 0x830A7C68;
    'dispatch: loop {
        match pc {
            0x830A7C68 => {
    //   block [0x830A7C68..0x830A7D08)
	// 830A7C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A7C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A7C70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A7C74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A7C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A7C7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A7C80: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A7C84: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A7C88: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 830A7C8C: 40980050  bge cr6, 0x830a7cdc
	if !ctx.cr[6].lt {
	pc = 0x830A7CDC; continue 'dispatch;
	}
	// 830A7C90: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A7C94: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830A7C98: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 830A7C9C: 7FC9522E  lhzx r30, r9, r10
	ctx.r[30].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830A7CA0: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 830A7CA4: 57CB0034  rlwinm r11, r30, 0, 0, 0x1a
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	// 830A7CA8: 2B0B0040  cmplwi cr6, r11, 0x40
	ctx.cr[6].compare_u32(ctx.r[11].u32, 64 as u32, &mut ctx.xer);
	// 830A7CAC: 409A0030  bne cr6, 0x830a7cdc
	if !ctx.cr[6].eq {
	pc = 0x830A7CDC; continue 'dispatch;
	}
	// 830A7CB0: 4BFFF679  bl 0x830a7328
	ctx.lr = 0x830A7CB4;
	sub_830A7328(ctx, base);
	// 830A7CB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A7CB8: 389EFFC0  addi r4, r30, -0x40
	ctx.r[4].s64 = ctx.r[30].s64 + -64;
	// 830A7CBC: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A7CC0: 4BFFBBB9  bl 0x830a3878
	ctx.lr = 0x830A7CC4;
	sub_830A3878(ctx, base);
	// 830A7CC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830A7CC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A7CCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A7CD0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A7CD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A7CD8: 4E800020  blr
	return;
	// 830A7CDC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A7CE0: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A7CE4: 38C00084  li r6, 0x84
	ctx.r[6].s64 = 132;
	// 830A7CE8: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A7CEC: 38A00307  li r5, 0x307
	ctx.r[5].s64 = 775;
	// 830A7CF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A7CF4: 4BFFF495  bl 0x830a7188
	ctx.lr = 0x830A7CF8;
	sub_830A7188(ctx, base);
	// 830A7CF8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A7CFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A7D00: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A7D04: 48108F25  bl 0x831b0c28
	ctx.lr = 0x830A7D08;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A7D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A7D08 size=60
    let mut pc: u32 = 0x830A7D08;
    'dispatch: loop {
        match pc {
            0x830A7D08 => {
    //   block [0x830A7D08..0x830A7D44)
	// 830A7D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A7D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A7D10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A7D14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A7D18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A7D1C: 4BFFF60D  bl 0x830a7328
	ctx.lr = 0x830A7D20;
	sub_830A7328(ctx, base);
	// 830A7D20: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A7D24: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A7D28: 38800069  li r4, 0x69
	ctx.r[4].s64 = 105;
	// 830A7D2C: 4BFFBB4D  bl 0x830a3878
	ctx.lr = 0x830A7D30;
	sub_830A3878(ctx, base);
	// 830A7D30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A7D34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A7D38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A7D3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A7D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A7D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A7D48 size=52
    let mut pc: u32 = 0x830A7D48;
    'dispatch: loop {
        match pc {
            0x830A7D48 => {
    //   block [0x830A7D48..0x830A7D7C)
	// 830A7D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A7D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A7D50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A7D54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A7D58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A7D5C: 4BFFF5CD  bl 0x830a7328
	ctx.lr = 0x830A7D60;
	sub_830A7328(ctx, base);
	// 830A7D60: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A7D64: 4BFFC275  bl 0x830a3fd8
	ctx.lr = 0x830A7D68;
	sub_830A3FD8(ctx, base);
	// 830A7D68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A7D6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A7D70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A7D74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A7D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A7D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A7D80 size=52
    let mut pc: u32 = 0x830A7D80;
    'dispatch: loop {
        match pc {
            0x830A7D80 => {
    //   block [0x830A7D80..0x830A7DB4)
	// 830A7D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A7D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A7D88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A7D8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A7D90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A7D94: 4BFFF595  bl 0x830a7328
	ctx.lr = 0x830A7D98;
	sub_830A7328(ctx, base);
	// 830A7D98: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A7D9C: 4BFFC1C5  bl 0x830a3f60
	ctx.lr = 0x830A7DA0;
	sub_830A3F60(ctx, base);
	// 830A7DA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A7DA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A7DA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A7DAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A7DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A7DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A7DB8 size=16
    let mut pc: u32 = 0x830A7DB8;
    'dispatch: loop {
        match pc {
            0x830A7DB8 => {
    //   block [0x830A7DB8..0x830A7DC8)
	// 830A7DB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A7DBC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A7DC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A7DC4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A7DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A7DC8 size=372
    let mut pc: u32 = 0x830A7DC8;
    'dispatch: loop {
        match pc {
            0x830A7DC8 => {
    //   block [0x830A7DC8..0x830A7F3C)
	// 830A7DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A7DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A7DD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A7DD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A7DD8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A7DDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A7DE0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A7DE4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A7DE8: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 830A7DEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A7DF0: 4E800421  bctrl
	ctx.lr = 0x830A7DF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A7DF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A7DF8: 480000A4  b 0x830a7e9c
	pc = 0x830A7E9C; continue 'dispatch;
	// 830A7DFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A7E00: 409A0018  bne cr6, 0x830a7e18
	if !ctx.cr[6].eq {
	pc = 0x830A7E18; continue 'dispatch;
	}
	// 830A7E04: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A7E08: 2F0A002D  cmpwi cr6, r10, 0x2d
	ctx.cr[6].compare_i32(ctx.r[10].s32, 45, &mut ctx.xer);
	// 830A7E0C: 419A0014  beq cr6, 0x830a7e20
	if ctx.cr[6].eq {
	pc = 0x830A7E20; continue 'dispatch;
	}
	// 830A7E10: 2F0A0026  cmpwi cr6, r10, 0x26
	ctx.cr[6].compare_i32(ctx.r[10].s32, 38, &mut ctx.xer);
	// 830A7E14: 419A000C  beq cr6, 0x830a7e20
	if ctx.cr[6].eq {
	pc = 0x830A7E20; continue 'dispatch;
	}
	// 830A7E18: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 830A7E1C: 409A00F4  bne cr6, 0x830a7f10
	if !ctx.cr[6].eq {
	pc = 0x830A7F10; continue 'dispatch;
	}
	// 830A7E20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A7E24: 4BFFF505  bl 0x830a7328
	ctx.lr = 0x830A7E28;
	sub_830A7328(ctx, base);
	// 830A7E28: A17F0020  lhz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A7E2C: 2B0B0009  cmplwi cr6, r11, 9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 9 as u32, &mut ctx.xer);
	// 830A7E30: 409A009C  bne cr6, 0x830a7ecc
	if !ctx.cr[6].eq {
	pc = 0x830A7ECC; continue 'dispatch;
	}
	// 830A7E34: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A7E38: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A7E3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A7E40: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 830A7E44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A7E48: 4E800421  bctrl
	ctx.lr = 0x830A7E4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A7E4C: A17F0020  lhz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A7E50: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A7E54: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 830A7E58: 409A0010  bne cr6, 0x830a7e68
	if !ctx.cr[6].eq {
	pc = 0x830A7E68; continue 'dispatch;
	}
	// 830A7E5C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A7E60: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A7E64: 4800002C  b 0x830a7e90
	pc = 0x830A7E90; continue 'dispatch;
	// 830A7E68: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A7E6C: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 830A7E70: 409A0010  bne cr6, 0x830a7e80
	if !ctx.cr[6].eq {
	pc = 0x830A7E80; continue 'dispatch;
	}
	// 830A7E74: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A7E78: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A7E7C: 48000014  b 0x830a7e90
	pc = 0x830A7E90; continue 'dispatch;
	// 830A7E80: 2F0B0026  cmpwi cr6, r11, 0x26
	ctx.cr[6].compare_i32(ctx.r[11].s32, 38, &mut ctx.xer);
	// 830A7E84: 409A0074  bne cr6, 0x830a7ef8
	if !ctx.cr[6].eq {
	pc = 0x830A7EF8; continue 'dispatch;
	}
	// 830A7E88: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A7E8C: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 830A7E90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A7E94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A7E98: 4E800421  bctrl
	ctx.lr = 0x830A7E9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A7E9C: A17F0020  lhz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A7EA0: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 830A7EA4: 409AFF58  bne cr6, 0x830a7dfc
	if !ctx.cr[6].eq {
	pc = 0x830A7DFC; continue 'dispatch;
	}
	// 830A7EA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A7EAC: 4BFFF47D  bl 0x830a7328
	ctx.lr = 0x830A7EB0;
	sub_830A7328(ctx, base);
	// 830A7EB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A7EB4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830A7EB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A7EBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A7EC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A7EC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A7EC8: 4E800020  blr
	return;
	// 830A7ECC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A7ED0: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A7ED4: 38C0008F  li r6, 0x8f
	ctx.r[6].s64 = 143;
	// 830A7ED8: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A7EDC: 38A004FB  li r5, 0x4fb
	ctx.r[5].s64 = 1275;
	// 830A7EE0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830A7EE4: 4BFFF2A5  bl 0x830a7188
	ctx.lr = 0x830A7EE8;
	sub_830A7188(ctx, base);
	// 830A7EE8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A7EEC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830A7EF0: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A7EF4: 48108D35  bl 0x831b0c28
	ctx.lr = 0x830A7EF8;
	sub_831B0C28(ctx, base);
	// 830A7EF8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A7EFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A7F00: 388BCAB4  addi r4, r11, -0x354c
	ctx.r[4].s64 = ctx.r[11].s64 + -13644;
	// 830A7F04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830A7F08: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830A7F0C: 48108D1D  bl 0x831b0c28
	ctx.lr = 0x830A7F10;
	sub_831B0C28(ctx, base);
	// 830A7F10: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A7F14: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A7F18: 38C00090  li r6, 0x90
	ctx.r[6].s64 = 144;
	// 830A7F1C: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A7F20: 38A0050D  li r5, 0x50d
	ctx.r[5].s64 = 1293;
	// 830A7F24: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830A7F28: 4BFFF261  bl 0x830a7188
	ctx.lr = 0x830A7F2C;
	sub_830A7188(ctx, base);
	// 830A7F2C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A7F30: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830A7F34: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A7F38: 48108CF1  bl 0x831b0c28
	ctx.lr = 0x830A7F3C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A7F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A7F40 size=88
    let mut pc: u32 = 0x830A7F40;
    'dispatch: loop {
        match pc {
            0x830A7F40 => {
    //   block [0x830A7F40..0x830A7F98)
	// 830A7F40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830A7F44: 3D408217  lis r10, -0x7de9
	ctx.r[10].s64 = -2112421888;
	// 830A7F48: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A7F4C: 2F040044  cmpwi cr6, r4, 0x44
	ctx.cr[6].compare_i32(ctx.r[4].s32, 68, &mut ctx.xer);
	// 830A7F50: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830A7F54: 814AACA8  lwz r10, -0x5358(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-21336 as u32) ) } as u64;
	// 830A7F58: 7D295038  and r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 830A7F5C: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 830A7F60: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 830A7F64: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 830A7F68: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 830A7F6C: 419A00F4  beq cr6, 0x830a8060
	if ctx.cr[6].eq {
		sub_830A8060(ctx, base);
		return;
	}
	// 830A7F70: 2F040053  cmpwi cr6, r4, 0x53
	ctx.cr[6].compare_i32(ctx.r[4].s32, 83, &mut ctx.xer);
	// 830A7F74: 419A00C4  beq cr6, 0x830a8038
	if ctx.cr[6].eq {
		sub_830A8038(ctx, base);
		return;
	}
	// 830A7F78: 2F040057  cmpwi cr6, r4, 0x57
	ctx.cr[6].compare_i32(ctx.r[4].s32, 87, &mut ctx.xer);
	// 830A7F7C: 419A0094  beq cr6, 0x830a8010
	if ctx.cr[6].eq {
		sub_830A8010(ctx, base);
		return;
	}
	// 830A7F80: 2F040064  cmpwi cr6, r4, 0x64
	ctx.cr[6].compare_i32(ctx.r[4].s32, 100, &mut ctx.xer);
	// 830A7F84: 419A0064  beq cr6, 0x830a7fe8
	if ctx.cr[6].eq {
		sub_830A7FE8(ctx, base);
		return;
	}
	// 830A7F88: 2F040073  cmpwi cr6, r4, 0x73
	ctx.cr[6].compare_i32(ctx.r[4].s32, 115, &mut ctx.xer);
	// 830A7F8C: 419A0034  beq cr6, 0x830a7fc0
	if ctx.cr[6].eq {
		sub_830A7FC0(ctx, base);
		return;
	}
	// 830A7F90: 2F040077  cmpwi cr6, r4, 0x77
	ctx.cr[6].compare_i32(ctx.r[4].s32, 119, &mut ctx.xer);
	// 830A7F94: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A7F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A7F98 size=32
    let mut pc: u32 = 0x830A7F98;
    'dispatch: loop {
        match pc {
            0x830A7F98 => {
    //   block [0x830A7F98..0x830A7FB8)
	// 830A7F98: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830A7F9C: 806B0030  lwz r3, 0x30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A7FA0: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830A7FA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A7FA8: 394AFF38  addi r10, r10, -0xc8
	ctx.r[10].s64 = ctx.r[10].s64 + -200;
	// 830A7FAC: 4182000C  beq 0x830a7fb8
	if ctx.cr[0].eq {
		sub_830A7FB8(ctx, base);
		return;
	}
	// 830A7FB0: 388A0054  addi r4, r10, 0x54
	ctx.r[4].s64 = ctx.r[10].s64 + 84;
	// 830A7FB4: 4BFFBC84  b 0x830a3c38
	sub_830A3C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A7FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A7FB8 size=8
    let mut pc: u32 = 0x830A7FB8;
    'dispatch: loop {
        match pc {
            0x830A7FB8 => {
    //   block [0x830A7FB8..0x830A7FC0)
	// 830A7FB8: 388A001C  addi r4, r10, 0x1c
	ctx.r[4].s64 = ctx.r[10].s64 + 28;
	// 830A7FBC: 4BFFBC7C  b 0x830a3c38
	sub_830A3C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A7FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A7FC0 size=32
    let mut pc: u32 = 0x830A7FC0;
    'dispatch: loop {
        match pc {
            0x830A7FC0 => {
    //   block [0x830A7FC0..0x830A7FE0)
	// 830A7FC0: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830A7FC4: 806B0030  lwz r3, 0x30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A7FC8: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830A7FCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A7FD0: 394AFF38  addi r10, r10, -0xc8
	ctx.r[10].s64 = ctx.r[10].s64 + -200;
	// 830A7FD4: 4182000C  beq 0x830a7fe0
	if ctx.cr[0].eq {
		sub_830A7FE0(ctx, base);
		return;
	}
	// 830A7FD8: 388A0064  addi r4, r10, 0x64
	ctx.r[4].s64 = ctx.r[10].s64 + 100;
	// 830A7FDC: 4BFFBC5C  b 0x830a3c38
	sub_830A3C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A7FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A7FE0 size=8
    let mut pc: u32 = 0x830A7FE0;
    'dispatch: loop {
        match pc {
            0x830A7FE0 => {
    //   block [0x830A7FE0..0x830A7FE8)
	// 830A7FE0: 388A0038  addi r4, r10, 0x38
	ctx.r[4].s64 = ctx.r[10].s64 + 56;
	// 830A7FE4: 4BFFBC54  b 0x830a3c38
	sub_830A3C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A7FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A7FE8 size=32
    let mut pc: u32 = 0x830A7FE8;
    'dispatch: loop {
        match pc {
            0x830A7FE8 => {
    //   block [0x830A7FE8..0x830A8008)
	// 830A7FE8: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830A7FEC: 806B0030  lwz r3, 0x30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A7FF0: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830A7FF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A7FF8: 41820010  beq 0x830a8008
	if ctx.cr[0].eq {
		sub_830A8008(ctx, base);
		return;
	}
	// 830A7FFC: 394AFF38  addi r10, r10, -0xc8
	ctx.r[10].s64 = ctx.r[10].s64 + -200;
	// 830A8000: 388A0074  addi r4, r10, 0x74
	ctx.r[4].s64 = ctx.r[10].s64 + 116;
	// 830A8004: 4BFFBC34  b 0x830a3c38
	sub_830A3C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A8008 size=8
    let mut pc: u32 = 0x830A8008;
    'dispatch: loop {
        match pc {
            0x830A8008 => {
    //   block [0x830A8008..0x830A8010)
	// 830A8008: 388AFF38  addi r4, r10, -0xc8
	ctx.r[4].s64 = ctx.r[10].s64 + -200;
	// 830A800C: 4BFFBC2C  b 0x830a3c38
	sub_830A3C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A8010 size=32
    let mut pc: u32 = 0x830A8010;
    'dispatch: loop {
        match pc {
            0x830A8010 => {
    //   block [0x830A8010..0x830A8030)
	// 830A8010: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830A8014: 806B0030  lwz r3, 0x30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A8018: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830A801C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830A8020: 394AFF38  addi r10, r10, -0xc8
	ctx.r[10].s64 = ctx.r[10].s64 + -200;
	// 830A8024: 4182000C  beq 0x830a8030
	if ctx.cr[0].eq {
		sub_830A8030(ctx, base);
		return;
	}
	// 830A8028: 388A0054  addi r4, r10, 0x54
	ctx.r[4].s64 = ctx.r[10].s64 + 84;
	// 830A802C: 4BFFBC0C  b 0x830a3c38
	sub_830A3C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A8030 size=8
    let mut pc: u32 = 0x830A8030;
    'dispatch: loop {
        match pc {
            0x830A8030 => {
    //   block [0x830A8030..0x830A8038)
	// 830A8030: 388A001C  addi r4, r10, 0x1c
	ctx.r[4].s64 = ctx.r[10].s64 + 28;
	// 830A8034: 4BFFBC04  b 0x830a3c38
	sub_830A3C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A8038 size=32
    let mut pc: u32 = 0x830A8038;
    'dispatch: loop {
        match pc {
            0x830A8038 => {
    //   block [0x830A8038..0x830A8058)
	// 830A8038: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830A803C: 806B0030  lwz r3, 0x30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A8040: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830A8044: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830A8048: 394AFF38  addi r10, r10, -0xc8
	ctx.r[10].s64 = ctx.r[10].s64 + -200;
	// 830A804C: 4182000C  beq 0x830a8058
	if ctx.cr[0].eq {
		sub_830A8058(ctx, base);
		return;
	}
	// 830A8050: 388A0064  addi r4, r10, 0x64
	ctx.r[4].s64 = ctx.r[10].s64 + 100;
	// 830A8054: 4BFFBBE4  b 0x830a3c38
	sub_830A3C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A8058 size=8
    let mut pc: u32 = 0x830A8058;
    'dispatch: loop {
        match pc {
            0x830A8058 => {
    //   block [0x830A8058..0x830A8060)
	// 830A8058: 388A0038  addi r4, r10, 0x38
	ctx.r[4].s64 = ctx.r[10].s64 + 56;
	// 830A805C: 4BFFBBDC  b 0x830a3c38
	sub_830A3C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A8060 size=32
    let mut pc: u32 = 0x830A8060;
    'dispatch: loop {
        match pc {
            0x830A8060 => {
    //   block [0x830A8060..0x830A8080)
	// 830A8060: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830A8064: 806B0030  lwz r3, 0x30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A8068: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830A806C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830A8070: 41820010  beq 0x830a8080
	if ctx.cr[0].eq {
		sub_830A8080(ctx, base);
		return;
	}
	// 830A8074: 394AFF38  addi r10, r10, -0xc8
	ctx.r[10].s64 = ctx.r[10].s64 + -200;
	// 830A8078: 388A0074  addi r4, r10, 0x74
	ctx.r[4].s64 = ctx.r[10].s64 + 116;
	// 830A807C: 4BFFBBBC  b 0x830a3c38
	sub_830A3C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A8080 size=8
    let mut pc: u32 = 0x830A8080;
    'dispatch: loop {
        match pc {
            0x830A8080 => {
    //   block [0x830A8080..0x830A8088)
	// 830A8080: 388AFF38  addi r4, r10, -0xc8
	ctx.r[4].s64 = ctx.r[10].s64 + -200;
	// 830A8084: 4BFFBBB4  b 0x830a3c38
	sub_830A3C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A8088 size=4
    let mut pc: u32 = 0x830A8088;
    'dispatch: loop {
        match pc {
            0x830A8088 => {
    //   block [0x830A8088..0x830A808C)
	// 830A8088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A8090 size=1352
    let mut pc: u32 = 0x830A8090;
    'dispatch: loop {
        match pc {
            0x830A8090 => {
    //   block [0x830A8090..0x830A85D8)
	// 830A8090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A8094: 481000D9  bl 0x831a816c
	ctx.lr = 0x830A8098;
	sub_831A8130(ctx, base);
	// 830A8098: 9421FE50  stwu r1, -0x1b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-432 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A809C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A80A0: A17E0020  lhz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A80A4: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 830A80A8: 419A0030  beq cr6, 0x830a80d8
	if ctx.cr[6].eq {
	pc = 0x830A80D8; continue 'dispatch;
	}
	// 830A80AC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A80B0: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A80B4: 38C0007A  li r6, 0x7a
	ctx.r[6].s64 = 122;
	// 830A80B8: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A80BC: 38A0053D  li r5, 0x53d
	ctx.r[5].s64 = 1341;
	// 830A80C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A80C4: 4BFFF0C5  bl 0x830a7188
	ctx.lr = 0x830A80C8;
	sub_830A7188(ctx, base);
	// 830A80C8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A80CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A80D0: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A80D4: 48108B55  bl 0x831b0c28
	ctx.lr = 0x830A80D8;
	sub_831B0C28(ctx, base);
	// 830A80D8: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A80DC: 2F030072  cmpwi cr6, r3, 0x72
	ctx.cr[6].compare_i32(ctx.r[3].s32, 114, &mut ctx.xer);
	// 830A80E0: 41990050  bgt cr6, 0x830a8130
	if ctx.cr[6].gt {
	pc = 0x830A8130; continue 'dispatch;
	}
	// 830A80E4: 419A0044  beq cr6, 0x830a8128
	if ctx.cr[6].eq {
	pc = 0x830A8128; continue 'dispatch;
	}
	// 830A80E8: 2F030041  cmpwi cr6, r3, 0x41
	ctx.cr[6].compare_i32(ctx.r[3].s32, 65, &mut ctx.xer);
	// 830A80EC: 419A006C  beq cr6, 0x830a8158
	if ctx.cr[6].eq {
	pc = 0x830A8158; continue 'dispatch;
	}
	// 830A80F0: 2F03005A  cmpwi cr6, r3, 0x5a
	ctx.cr[6].compare_i32(ctx.r[3].s32, 90, &mut ctx.xer);
	// 830A80F4: 419A0064  beq cr6, 0x830a8158
	if ctx.cr[6].eq {
	pc = 0x830A8158; continue 'dispatch;
	}
	// 830A80F8: 2F030065  cmpwi cr6, r3, 0x65
	ctx.cr[6].compare_i32(ctx.r[3].s32, 101, &mut ctx.xer);
	// 830A80FC: 419A0024  beq cr6, 0x830a8120
	if ctx.cr[6].eq {
	pc = 0x830A8120; continue 'dispatch;
	}
	// 830A8100: 2F030066  cmpwi cr6, r3, 0x66
	ctx.cr[6].compare_i32(ctx.r[3].s32, 102, &mut ctx.xer);
	// 830A8104: 419A0014  beq cr6, 0x830a8118
	if ctx.cr[6].eq {
	pc = 0x830A8118; continue 'dispatch;
	}
	// 830A8108: 2F03006E  cmpwi cr6, r3, 0x6e
	ctx.cr[6].compare_i32(ctx.r[3].s32, 110, &mut ctx.xer);
	// 830A810C: 409A04C4  bne cr6, 0x830a85d0
	if !ctx.cr[6].eq {
	pc = 0x830A85D0; continue 'dispatch;
	}
	// 830A8110: 3860000A  li r3, 0xa
	ctx.r[3].s64 = 10;
	// 830A8114: 480004BC  b 0x830a85d0
	pc = 0x830A85D0; continue 'dispatch;
	// 830A8118: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 830A811C: 480004B4  b 0x830a85d0
	pc = 0x830A85D0; continue 'dispatch;
	// 830A8120: 3860001B  li r3, 0x1b
	ctx.r[3].s64 = 27;
	// 830A8124: 480004AC  b 0x830a85d0
	pc = 0x830A85D0; continue 'dispatch;
	// 830A8128: 3860000D  li r3, 0xd
	ctx.r[3].s64 = 13;
	// 830A812C: 480004A4  b 0x830a85d0
	pc = 0x830A85D0; continue 'dispatch;
	// 830A8130: 2F030074  cmpwi cr6, r3, 0x74
	ctx.cr[6].compare_i32(ctx.r[3].s32, 116, &mut ctx.xer);
	// 830A8134: 419A0498  beq cr6, 0x830a85cc
	if ctx.cr[6].eq {
	pc = 0x830A85CC; continue 'dispatch;
	}
	// 830A8138: 2F030075  cmpwi cr6, r3, 0x75
	ctx.cr[6].compare_i32(ctx.r[3].s32, 117, &mut ctx.xer);
	// 830A813C: 419A03C8  beq cr6, 0x830a8504
	if ctx.cr[6].eq {
	pc = 0x830A8504; continue 'dispatch;
	}
	// 830A8140: 2F030076  cmpwi cr6, r3, 0x76
	ctx.cr[6].compare_i32(ctx.r[3].s32, 118, &mut ctx.xer);
	// 830A8144: 419A02C4  beq cr6, 0x830a8408
	if ctx.cr[6].eq {
	pc = 0x830A8408; continue 'dispatch;
	}
	// 830A8148: 2F030078  cmpwi cr6, r3, 0x78
	ctx.cr[6].compare_i32(ctx.r[3].s32, 120, &mut ctx.xer);
	// 830A814C: 419A0038  beq cr6, 0x830a8184
	if ctx.cr[6].eq {
	pc = 0x830A8184; continue 'dispatch;
	}
	// 830A8150: 2F03007A  cmpwi cr6, r3, 0x7a
	ctx.cr[6].compare_i32(ctx.r[3].s32, 122, &mut ctx.xer);
	// 830A8154: 409A047C  bne cr6, 0x830a85d0
	if !ctx.cr[6].eq {
	pc = 0x830A85D0; continue 'dispatch;
	}
	// 830A8158: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A815C: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A8160: 38C00096  li r6, 0x96
	ctx.r[6].s64 = 150;
	// 830A8164: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A8168: 38A005A6  li r5, 0x5a6
	ctx.r[5].s64 = 1446;
	// 830A816C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A8170: 4BFFF019  bl 0x830a7188
	ctx.lr = 0x830A8174;
	sub_830A7188(ctx, base);
	// 830A8174: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A8178: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A817C: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A8180: 48108AA9  bl 0x831b0c28
	ctx.lr = 0x830A8184;
	sub_831B0C28(ctx, base);
	// 830A8184: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A8188: 4BFFF1A1  bl 0x830a7328
	ctx.lr = 0x830A818C;
	sub_830A7328(ctx, base);
	// 830A818C: A17E0020  lhz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A8190: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A8194: 41820030  beq 0x830a81c4
	if ctx.cr[0].eq {
	pc = 0x830A81C4; continue 'dispatch;
	}
	// 830A8198: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A819C: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A81A0: 38C00092  li r6, 0x92
	ctx.r[6].s64 = 146;
	// 830A81A4: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A81A8: 38A00555  li r5, 0x555
	ctx.r[5].s64 = 1365;
	// 830A81AC: 38610150  addi r3, r1, 0x150
	ctx.r[3].s64 = ctx.r[1].s64 + 336;
	// 830A81B0: 4BFFEFD9  bl 0x830a7188
	ctx.lr = 0x830A81B4;
	sub_830A7188(ctx, base);
	// 830A81B4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A81B8: 38610150  addi r3, r1, 0x150
	ctx.r[3].s64 = ctx.r[1].s64 + 336;
	// 830A81BC: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A81C0: 48108A69  bl 0x831b0c28
	ctx.lr = 0x830A81C4;
	sub_831B0C28(ctx, base);
	// 830A81C4: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A81C8: 2F0B007B  cmpwi cr6, r11, 0x7b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 123, &mut ctx.xer);
	// 830A81CC: 409A011C  bne cr6, 0x830a82e8
	if !ctx.cr[6].eq {
	pc = 0x830A82E8; continue 'dispatch;
	}
	// 830A81D0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830A81D4: 48000060  b 0x830a8234
	pc = 0x830A8234; continue 'dispatch;
	// 830A81D8: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A81DC: 394BFFD0  addi r10, r11, -0x30
	ctx.r[10].s64 = ctx.r[11].s64 + -48;
	// 830A81E0: 2B0A0036  cmplwi cr6, r10, 0x36
	ctx.cr[6].compare_u32(ctx.r[10].u32, 54 as u32, &mut ctx.xer);
	// 830A81E4: 4199003C  bgt cr6, 0x830a8220
	if ctx.cr[6].gt {
	pc = 0x830A8220; continue 'dispatch;
	}
	// 830A81E8: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 830A81EC: 4199000C  bgt cr6, 0x830a81f8
	if ctx.cr[6].gt {
	pc = 0x830A81F8; continue 'dispatch;
	}
	// 830A81F0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 830A81F4: 48000030  b 0x830a8224
	pc = 0x830A8224; continue 'dispatch;
	// 830A81F8: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 830A81FC: 41980078  blt cr6, 0x830a8274
	if ctx.cr[6].lt {
	pc = 0x830A8274; continue 'dispatch;
	}
	// 830A8200: 2F0B0046  cmpwi cr6, r11, 0x46
	ctx.cr[6].compare_i32(ctx.r[11].s32, 70, &mut ctx.xer);
	// 830A8204: 4199000C  bgt cr6, 0x830a8210
	if ctx.cr[6].gt {
	pc = 0x830A8210; continue 'dispatch;
	}
	// 830A8208: 396BFFC9  addi r11, r11, -0x37
	ctx.r[11].s64 = ctx.r[11].s64 + -55;
	// 830A820C: 48000018  b 0x830a8224
	pc = 0x830A8224; continue 'dispatch;
	// 830A8210: 2F0B0061  cmpwi cr6, r11, 0x61
	ctx.cr[6].compare_i32(ctx.r[11].s32, 97, &mut ctx.xer);
	// 830A8214: 41980060  blt cr6, 0x830a8274
	if ctx.cr[6].lt {
	pc = 0x830A8274; continue 'dispatch;
	}
	// 830A8218: 396BFFA9  addi r11, r11, -0x57
	ctx.r[11].s64 = ctx.r[11].s64 + -87;
	// 830A821C: 48000008  b 0x830a8224
	pc = 0x830A8224; continue 'dispatch;
	// 830A8220: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 830A8224: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A8228: 4198004C  blt cr6, 0x830a8274
	if ctx.cr[6].lt {
	pc = 0x830A8274; continue 'dispatch;
	}
	// 830A822C: 57EA2036  slwi r10, r31, 4
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A8230: 7FEA5A14  add r31, r10, r11
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 830A8234: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A8238: 4BFFF0F1  bl 0x830a7328
	ctx.lr = 0x830A823C;
	sub_830A7328(ctx, base);
	// 830A823C: A17E0020  lhz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A8240: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A8244: 4182FF94  beq 0x830a81d8
	if ctx.cr[0].eq {
	pc = 0x830A81D8; continue 'dispatch;
	}
	// 830A8248: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A824C: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A8250: 38C00092  li r6, 0x92
	ctx.r[6].s64 = 146;
	// 830A8254: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A8258: 38A0055F  li r5, 0x55f
	ctx.r[5].s64 = 1375;
	// 830A825C: 38610110  addi r3, r1, 0x110
	ctx.r[3].s64 = ctx.r[1].s64 + 272;
	// 830A8260: 4BFFEF29  bl 0x830a7188
	ctx.lr = 0x830A8264;
	sub_830A7188(ctx, base);
	// 830A8264: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A8268: 38610110  addi r3, r1, 0x110
	ctx.r[3].s64 = ctx.r[1].s64 + 272;
	// 830A826C: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A8270: 481089B9  bl 0x831b0c28
	ctx.lr = 0x830A8274;
	sub_831B0C28(ctx, base);
	// 830A8274: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A8278: 2F0B007D  cmpwi cr6, r11, 0x7d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 125, &mut ctx.xer);
	// 830A827C: 419A0030  beq cr6, 0x830a82ac
	if ctx.cr[6].eq {
	pc = 0x830A82AC; continue 'dispatch;
	}
	// 830A8280: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A8284: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A8288: 38C00094  li r6, 0x94
	ctx.r[6].s64 = 148;
	// 830A828C: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A8290: 38A00568  li r5, 0x568
	ctx.r[5].s64 = 1384;
	// 830A8294: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 830A8298: 4BFFEEF1  bl 0x830a7188
	ctx.lr = 0x830A829C;
	sub_830A7188(ctx, base);
	// 830A829C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A82A0: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 830A82A4: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A82A8: 48108981  bl 0x831b0c28
	ctx.lr = 0x830A82AC;
	sub_831B0C28(ctx, base);
	// 830A82AC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A82B0: 816BFD68  lwz r11, -0x298(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-664 as u32) ) } as u64;
	// 830A82B4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830A82B8: 409902E0  ble cr6, 0x830a8598
	if !ctx.cr[6].gt {
	pc = 0x830A8598; continue 'dispatch;
	}
	// 830A82BC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A82C0: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A82C4: 38C00095  li r6, 0x95
	ctx.r[6].s64 = 149;
	// 830A82C8: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A82CC: 38A0056B  li r5, 0x56b
	ctx.r[5].s64 = 1387;
	// 830A82D0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830A82D4: 4BFFEEB5  bl 0x830a7188
	ctx.lr = 0x830A82D8;
	sub_830A7188(ctx, base);
	// 830A82D8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A82DC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830A82E0: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A82E4: 48108945  bl 0x831b0c28
	ctx.lr = 0x830A82E8;
	sub_831B0C28(ctx, base);
	// 830A82E8: 394BFFD0  addi r10, r11, -0x30
	ctx.r[10].s64 = ctx.r[11].s64 + -48;
	// 830A82EC: 2B0A0036  cmplwi cr6, r10, 0x36
	ctx.cr[6].compare_u32(ctx.r[10].u32, 54 as u32, &mut ctx.xer);
	// 830A82F0: 4199003C  bgt cr6, 0x830a832c
	if ctx.cr[6].gt {
	pc = 0x830A832C; continue 'dispatch;
	}
	// 830A82F4: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 830A82F8: 4199000C  bgt cr6, 0x830a8304
	if ctx.cr[6].gt {
	pc = 0x830A8304; continue 'dispatch;
	}
	// 830A82FC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 830A8300: 48000030  b 0x830a8330
	pc = 0x830A8330; continue 'dispatch;
	// 830A8304: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 830A8308: 419800D4  blt cr6, 0x830a83dc
	if ctx.cr[6].lt {
	pc = 0x830A83DC; continue 'dispatch;
	}
	// 830A830C: 2F0B0046  cmpwi cr6, r11, 0x46
	ctx.cr[6].compare_i32(ctx.r[11].s32, 70, &mut ctx.xer);
	// 830A8310: 4199000C  bgt cr6, 0x830a831c
	if ctx.cr[6].gt {
	pc = 0x830A831C; continue 'dispatch;
	}
	// 830A8314: 396BFFC9  addi r11, r11, -0x37
	ctx.r[11].s64 = ctx.r[11].s64 + -55;
	// 830A8318: 48000018  b 0x830a8330
	pc = 0x830A8330; continue 'dispatch;
	// 830A831C: 2F0B0061  cmpwi cr6, r11, 0x61
	ctx.cr[6].compare_i32(ctx.r[11].s32, 97, &mut ctx.xer);
	// 830A8320: 419800BC  blt cr6, 0x830a83dc
	if ctx.cr[6].lt {
	pc = 0x830A83DC; continue 'dispatch;
	}
	// 830A8324: 396BFFA9  addi r11, r11, -0x57
	ctx.r[11].s64 = ctx.r[11].s64 + -87;
	// 830A8328: 48000008  b 0x830a8330
	pc = 0x830A8330; continue 'dispatch;
	// 830A832C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 830A8330: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A8334: 419800A8  blt cr6, 0x830a83dc
	if ctx.cr[6].lt {
	pc = 0x830A83DC; continue 'dispatch;
	}
	// 830A8338: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A833C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 830A8340: 4BFFEFE9  bl 0x830a7328
	ctx.lr = 0x830A8344;
	sub_830A7328(ctx, base);
	// 830A8344: A17E0020  lhz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A8348: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A834C: 40820064  bne 0x830a83b0
	if !ctx.cr[0].eq {
	pc = 0x830A83B0; continue 'dispatch;
	}
	// 830A8350: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A8354: 394BFFD0  addi r10, r11, -0x30
	ctx.r[10].s64 = ctx.r[11].s64 + -48;
	// 830A8358: 2B0A0036  cmplwi cr6, r10, 0x36
	ctx.cr[6].compare_u32(ctx.r[10].u32, 54 as u32, &mut ctx.xer);
	// 830A835C: 4199003C  bgt cr6, 0x830a8398
	if ctx.cr[6].gt {
	pc = 0x830A8398; continue 'dispatch;
	}
	// 830A8360: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 830A8364: 4199000C  bgt cr6, 0x830a8370
	if ctx.cr[6].gt {
	pc = 0x830A8370; continue 'dispatch;
	}
	// 830A8368: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 830A836C: 48000030  b 0x830a839c
	pc = 0x830A839C; continue 'dispatch;
	// 830A8370: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 830A8374: 4198003C  blt cr6, 0x830a83b0
	if ctx.cr[6].lt {
	pc = 0x830A83B0; continue 'dispatch;
	}
	// 830A8378: 2F0B0046  cmpwi cr6, r11, 0x46
	ctx.cr[6].compare_i32(ctx.r[11].s32, 70, &mut ctx.xer);
	// 830A837C: 4199000C  bgt cr6, 0x830a8388
	if ctx.cr[6].gt {
	pc = 0x830A8388; continue 'dispatch;
	}
	// 830A8380: 396BFFC9  addi r11, r11, -0x37
	ctx.r[11].s64 = ctx.r[11].s64 + -55;
	// 830A8384: 48000018  b 0x830a839c
	pc = 0x830A839C; continue 'dispatch;
	// 830A8388: 2F0B0061  cmpwi cr6, r11, 0x61
	ctx.cr[6].compare_i32(ctx.r[11].s32, 97, &mut ctx.xer);
	// 830A838C: 41980024  blt cr6, 0x830a83b0
	if ctx.cr[6].lt {
	pc = 0x830A83B0; continue 'dispatch;
	}
	// 830A8390: 396BFFA9  addi r11, r11, -0x57
	ctx.r[11].s64 = ctx.r[11].s64 + -87;
	// 830A8394: 48000008  b 0x830a839c
	pc = 0x830A839C; continue 'dispatch;
	// 830A8398: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 830A839C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A83A0: 41980010  blt cr6, 0x830a83b0
	if ctx.cr[6].lt {
	pc = 0x830A83B0; continue 'dispatch;
	}
	// 830A83A4: 57EA2036  slwi r10, r31, 4
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A83A8: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 830A83AC: 48000224  b 0x830a85d0
	pc = 0x830A85D0; continue 'dispatch;
	// 830A83B0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A83B4: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A83B8: 38C00092  li r6, 0x92
	ctx.r[6].s64 = 146;
	// 830A83BC: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A83C0: 38A00578  li r5, 0x578
	ctx.r[5].s64 = 1400;
	// 830A83C4: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 830A83C8: 4BFFEDC1  bl 0x830a7188
	ctx.lr = 0x830A83CC;
	sub_830A7188(ctx, base);
	// 830A83CC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A83D0: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 830A83D4: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A83D8: 48108851  bl 0x831b0c28
	ctx.lr = 0x830A83DC;
	sub_831B0C28(ctx, base);
	// 830A83DC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A83E0: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A83E4: 38C00092  li r6, 0x92
	ctx.r[6].s64 = 146;
	// 830A83E8: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A83EC: 38A00572  li r5, 0x572
	ctx.r[5].s64 = 1394;
	// 830A83F0: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 830A83F4: 4BFFED95  bl 0x830a7188
	ctx.lr = 0x830A83F8;
	sub_830A7188(ctx, base);
	// 830A83F8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A83FC: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 830A8400: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A8404: 48108825  bl 0x831b0c28
	ctx.lr = 0x830A8408;
	sub_831B0C28(ctx, base);
	// 830A8408: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830A840C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A8410: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A8414: 4BFFEF15  bl 0x830a7328
	ctx.lr = 0x830A8418;
	sub_830A7328(ctx, base);
	// 830A8418: A17E0020  lhz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A841C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A8420: 408200B8  bne 0x830a84d8
	if !ctx.cr[0].eq {
	pc = 0x830A84D8; continue 'dispatch;
	}
	// 830A8424: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A8428: 394BFFD0  addi r10, r11, -0x30
	ctx.r[10].s64 = ctx.r[11].s64 + -48;
	// 830A842C: 2B0A0036  cmplwi cr6, r10, 0x36
	ctx.cr[6].compare_u32(ctx.r[10].u32, 54 as u32, &mut ctx.xer);
	// 830A8430: 4199003C  bgt cr6, 0x830a846c
	if ctx.cr[6].gt {
	pc = 0x830A846C; continue 'dispatch;
	}
	// 830A8434: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 830A8438: 4199000C  bgt cr6, 0x830a8444
	if ctx.cr[6].gt {
	pc = 0x830A8444; continue 'dispatch;
	}
	// 830A843C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 830A8440: 48000030  b 0x830a8470
	pc = 0x830A8470; continue 'dispatch;
	// 830A8444: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 830A8448: 41980090  blt cr6, 0x830a84d8
	if ctx.cr[6].lt {
	pc = 0x830A84D8; continue 'dispatch;
	}
	// 830A844C: 2F0B0046  cmpwi cr6, r11, 0x46
	ctx.cr[6].compare_i32(ctx.r[11].s32, 70, &mut ctx.xer);
	// 830A8450: 4199000C  bgt cr6, 0x830a845c
	if ctx.cr[6].gt {
	pc = 0x830A845C; continue 'dispatch;
	}
	// 830A8454: 396BFFC9  addi r11, r11, -0x37
	ctx.r[11].s64 = ctx.r[11].s64 + -55;
	// 830A8458: 48000018  b 0x830a8470
	pc = 0x830A8470; continue 'dispatch;
	// 830A845C: 2F0B0061  cmpwi cr6, r11, 0x61
	ctx.cr[6].compare_i32(ctx.r[11].s32, 97, &mut ctx.xer);
	// 830A8460: 41980078  blt cr6, 0x830a84d8
	if ctx.cr[6].lt {
	pc = 0x830A84D8; continue 'dispatch;
	}
	// 830A8464: 396BFFA9  addi r11, r11, -0x57
	ctx.r[11].s64 = ctx.r[11].s64 + -87;
	// 830A8468: 48000008  b 0x830a8470
	pc = 0x830A8470; continue 'dispatch;
	// 830A846C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 830A8470: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A8474: 41980064  blt cr6, 0x830a84d8
	if ctx.cr[6].lt {
	pc = 0x830A84D8; continue 'dispatch;
	}
	// 830A8478: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830A847C: 409A000C  bne cr6, 0x830a8488
	if !ctx.cr[6].eq {
	pc = 0x830A8488; continue 'dispatch;
	}
	// 830A8480: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 830A8484: 4800000C  b 0x830a8490
	pc = 0x830A8490; continue 'dispatch;
	// 830A8488: 57EA2036  slwi r10, r31, 4
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A848C: 7FEA5A14  add r31, r10, r11
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 830A8490: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830A8494: 2F1D0006  cmpwi cr6, r29, 6
	ctx.cr[6].compare_i32(ctx.r[29].s32, 6, &mut ctx.xer);
	// 830A8498: 4198FF78  blt cr6, 0x830a8410
	if ctx.cr[6].lt {
	pc = 0x830A8410; continue 'dispatch;
	}
	// 830A849C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A84A0: 816BFD68  lwz r11, -0x298(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-664 as u32) ) } as u64;
	// 830A84A4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830A84A8: 409900F0  ble cr6, 0x830a8598
	if !ctx.cr[6].gt {
	pc = 0x830A8598; continue 'dispatch;
	}
	// 830A84AC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A84B0: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A84B4: 38C00092  li r6, 0x92
	ctx.r[6].s64 = 146;
	// 830A84B8: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A84BC: 38A0059E  li r5, 0x59e
	ctx.r[5].s64 = 1438;
	// 830A84C0: 38610130  addi r3, r1, 0x130
	ctx.r[3].s64 = ctx.r[1].s64 + 304;
	// 830A84C4: 4BFFECC5  bl 0x830a7188
	ctx.lr = 0x830A84C8;
	sub_830A7188(ctx, base);
	// 830A84C8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A84CC: 38610130  addi r3, r1, 0x130
	ctx.r[3].s64 = ctx.r[1].s64 + 304;
	// 830A84D0: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A84D4: 48108755  bl 0x831b0c28
	ctx.lr = 0x830A84D8;
	sub_831B0C28(ctx, base);
	// 830A84D8: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A84DC: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A84E0: 38C00092  li r6, 0x92
	ctx.r[6].s64 = 146;
	// 830A84E4: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A84E8: 38A00598  li r5, 0x598
	ctx.r[5].s64 = 1432;
	// 830A84EC: 386100F0  addi r3, r1, 0xf0
	ctx.r[3].s64 = ctx.r[1].s64 + 240;
	// 830A84F0: 4BFFEC99  bl 0x830a7188
	ctx.lr = 0x830A84F4;
	sub_830A7188(ctx, base);
	// 830A84F4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A84F8: 386100F0  addi r3, r1, 0xf0
	ctx.r[3].s64 = ctx.r[1].s64 + 240;
	// 830A84FC: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A8500: 48108729  bl 0x831b0c28
	ctx.lr = 0x830A8504;
	sub_831B0C28(ctx, base);
	// 830A8504: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830A8508: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A850C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A8510: 4BFFEE19  bl 0x830a7328
	ctx.lr = 0x830A8514;
	sub_830A7328(ctx, base);
	// 830A8514: A17E0020  lhz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A8518: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A851C: 40820084  bne 0x830a85a0
	if !ctx.cr[0].eq {
	pc = 0x830A85A0; continue 'dispatch;
	}
	// 830A8520: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A8524: 394BFFD0  addi r10, r11, -0x30
	ctx.r[10].s64 = ctx.r[11].s64 + -48;
	// 830A8528: 2B0A0036  cmplwi cr6, r10, 0x36
	ctx.cr[6].compare_u32(ctx.r[10].u32, 54 as u32, &mut ctx.xer);
	// 830A852C: 4199003C  bgt cr6, 0x830a8568
	if ctx.cr[6].gt {
	pc = 0x830A8568; continue 'dispatch;
	}
	// 830A8530: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 830A8534: 4199000C  bgt cr6, 0x830a8540
	if ctx.cr[6].gt {
	pc = 0x830A8540; continue 'dispatch;
	}
	// 830A8538: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 830A853C: 48000030  b 0x830a856c
	pc = 0x830A856C; continue 'dispatch;
	// 830A8540: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 830A8544: 4198005C  blt cr6, 0x830a85a0
	if ctx.cr[6].lt {
	pc = 0x830A85A0; continue 'dispatch;
	}
	// 830A8548: 2F0B0046  cmpwi cr6, r11, 0x46
	ctx.cr[6].compare_i32(ctx.r[11].s32, 70, &mut ctx.xer);
	// 830A854C: 4199000C  bgt cr6, 0x830a8558
	if ctx.cr[6].gt {
	pc = 0x830A8558; continue 'dispatch;
	}
	// 830A8550: 396BFFC9  addi r11, r11, -0x37
	ctx.r[11].s64 = ctx.r[11].s64 + -55;
	// 830A8554: 48000018  b 0x830a856c
	pc = 0x830A856C; continue 'dispatch;
	// 830A8558: 2F0B0061  cmpwi cr6, r11, 0x61
	ctx.cr[6].compare_i32(ctx.r[11].s32, 97, &mut ctx.xer);
	// 830A855C: 41980044  blt cr6, 0x830a85a0
	if ctx.cr[6].lt {
	pc = 0x830A85A0; continue 'dispatch;
	}
	// 830A8560: 396BFFA9  addi r11, r11, -0x57
	ctx.r[11].s64 = ctx.r[11].s64 + -87;
	// 830A8564: 48000008  b 0x830a856c
	pc = 0x830A856C; continue 'dispatch;
	// 830A8568: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 830A856C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A8570: 41980030  blt cr6, 0x830a85a0
	if ctx.cr[6].lt {
	pc = 0x830A85A0; continue 'dispatch;
	}
	// 830A8574: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830A8578: 409A000C  bne cr6, 0x830a8584
	if !ctx.cr[6].eq {
	pc = 0x830A8584; continue 'dispatch;
	}
	// 830A857C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 830A8580: 4800000C  b 0x830a858c
	pc = 0x830A858C; continue 'dispatch;
	// 830A8584: 57EA2036  slwi r10, r31, 4
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A8588: 7FEA5A14  add r31, r10, r11
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 830A858C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830A8590: 2F1D0004  cmpwi cr6, r29, 4
	ctx.cr[6].compare_i32(ctx.r[29].s32, 4, &mut ctx.xer);
	// 830A8594: 4198FF78  blt cr6, 0x830a850c
	if ctx.cr[6].lt {
	pc = 0x830A850C; continue 'dispatch;
	}
	// 830A8598: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A859C: 48000034  b 0x830a85d0
	pc = 0x830A85D0; continue 'dispatch;
	// 830A85A0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A85A4: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A85A8: 38C00092  li r6, 0x92
	ctx.r[6].s64 = 146;
	// 830A85AC: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A85B0: 38A00587  li r5, 0x587
	ctx.r[5].s64 = 1415;
	// 830A85B4: 38610170  addi r3, r1, 0x170
	ctx.r[3].s64 = ctx.r[1].s64 + 368;
	// 830A85B8: 4BFFEBD1  bl 0x830a7188
	ctx.lr = 0x830A85BC;
	sub_830A7188(ctx, base);
	// 830A85BC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A85C0: 38610170  addi r3, r1, 0x170
	ctx.r[3].s64 = ctx.r[1].s64 + 368;
	// 830A85C4: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A85C8: 48108661  bl 0x831b0c28
	ctx.lr = 0x830A85CC;
	sub_831B0C28(ctx, base);
	// 830A85CC: 38600009  li r3, 9
	ctx.r[3].s64 = 9;
	// 830A85D0: 382101B0  addi r1, r1, 0x1b0
	ctx.r[1].s64 = ctx.r[1].s64 + 432;
	// 830A85D4: 480FFBE8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A85D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A85D8 size=48
    let mut pc: u32 = 0x830A85D8;
    'dispatch: loop {
        match pc {
            0x830A85D8 => {
    //   block [0x830A85D8..0x830A8608)
	// 830A85D8: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A85DC: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830A85E0: 4098001C  bge cr6, 0x830a85fc
	if !ctx.cr[6].lt {
	pc = 0x830A85FC; continue 'dispatch;
	}
	// 830A85E4: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A85E8: 548A083C  slwi r10, r4, 1
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A85EC: 7D6B522E  lhzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830A85F0: 2B0B003F  cmplwi cr6, r11, 0x3f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 63 as u32, &mut ctx.xer);
	// 830A85F4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830A85F8: 419A0008  beq cr6, 0x830a8600
	if ctx.cr[6].eq {
	pc = 0x830A8600; continue 'dispatch;
	}
	// 830A85FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830A8600: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830A8604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A8608 size=16
    let mut pc: u32 = 0x830A8608;
    'dispatch: loop {
        match pc {
            0x830A8608 => {
    //   block [0x830A8608..0x830A8618)
	// 830A8608: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A860C: 396B00FC  addi r11, r11, 0xfc
	ctx.r[11].s64 = ctx.r[11].s64 + 252;
	// 830A8610: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A8614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A8618 size=128
    let mut pc: u32 = 0x830A8618;
    'dispatch: loop {
        match pc {
            0x830A8618 => {
    //   block [0x830A8618..0x830A8698)
	// 830A8618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A861C: 480FFB51  bl 0x831a816c
	ctx.lr = 0x830A8620;
	sub_831A8130(ctx, base);
	// 830A8620: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A8624: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A8628: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830A862C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830A8630: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A8634: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830A8638: 41980030  blt cr6, 0x830a8668
	if ctx.cr[6].lt {
	pc = 0x830A8668; continue 'dispatch;
	}
	// 830A863C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 830A8640: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A8644: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 830A8648: 388B6B80  addi r4, r11, 0x6b80
	ctx.r[4].s64 = ctx.r[11].s64 + 27520;
	// 830A864C: 38A00043  li r5, 0x43
	ctx.r[5].s64 = 67;
	// 830A8650: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A8654: 4BF28305  bl 0x82fd0958
	ctx.lr = 0x830A8658;
	sub_82FD0958(ctx, base);
	// 830A8658: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A865C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A8660: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 830A8664: 481085C5  bl 0x831b0c28
	ctx.lr = 0x830A8668;
	sub_831B0C28(ctx, base);
	// 830A8668: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A866C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A8670: 41820014  beq 0x830a8684
	if ctx.cr[0].eq {
	pc = 0x830A8684; continue 'dispatch;
	}
	// 830A8674: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A8678: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A867C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830A8680: 4BF2FC61  bl 0x82fd82e0
	ctx.lr = 0x830A8684;
	sub_82FD82E0(ctx, base);
	// 830A8684: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A8688: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A868C: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 830A8690: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830A8694: 480FFB28  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A8698 size=104
    let mut pc: u32 = 0x830A8698;
    'dispatch: loop {
        match pc {
            0x830A8698 => {
    //   block [0x830A8698..0x830A8700)
	// 830A8698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A869C: 480FFACD  bl 0x831a8168
	ctx.lr = 0x830A86A0;
	sub_831A8130(ctx, base);
	// 830A86A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A86A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A86A8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830A86AC: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 830A86B0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A86B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A86B8: 4099003C  ble cr6, 0x830a86f4
	if !ctx.cr[6].gt {
	pc = 0x830A86F4; continue 'dispatch;
	}
	// 830A86BC: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 830A86C0: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A86C4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A86C8: 41820010  beq 0x830a86d8
	if ctx.cr[0].eq {
	pc = 0x830A86D8; continue 'dispatch;
	}
	// 830A86CC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A86D0: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 830A86D4: 4BF2FC0D  bl 0x82fd82e0
	ctx.lr = 0x830A86D8;
	sub_82FD82E0(ctx, base);
	// 830A86D8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A86DC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830A86E0: 7F8BF12E  stwx r28, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[28].u32) };
	// 830A86E4: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 830A86E8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A86EC: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830A86F0: 4198FFD0  blt cr6, 0x830a86c0
	if ctx.cr[6].lt {
	pc = 0x830A86C0; continue 'dispatch;
	}
	// 830A86F4: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 830A86F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830A86FC: 480FFABC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A8700 size=264
    let mut pc: u32 = 0x830A8700;
    'dispatch: loop {
        match pc {
            0x830A8700 => {
    //   block [0x830A8700..0x830A8808)
	// 830A8700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A8704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A8708: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A870C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A8710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A8714: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A8718: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A871C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A8720: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830A8724: 41980030  blt cr6, 0x830a8754
	if ctx.cr[6].lt {
	pc = 0x830A8754; continue 'dispatch;
	}
	// 830A8728: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 830A872C: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A8730: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 830A8734: 388B6B80  addi r4, r11, 0x6b80
	ctx.r[4].s64 = ctx.r[11].s64 + 27520;
	// 830A8738: 38A00090  li r5, 0x90
	ctx.r[5].s64 = 144;
	// 830A873C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A8740: 4BF28219  bl 0x82fd0958
	ctx.lr = 0x830A8744;
	sub_82FD0958(ctx, base);
	// 830A8744: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A8748: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A874C: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 830A8750: 481084D9  bl 0x831b0c28
	ctx.lr = 0x830A8754;
	sub_831B0C28(ctx, base);
	// 830A8754: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A8758: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A875C: 41820014  beq 0x830a8770
	if ctx.cr[0].eq {
	pc = 0x830A8770; continue 'dispatch;
	}
	// 830A8760: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A8764: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A8768: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830A876C: 4BF2FB75  bl 0x82fd82e0
	ctx.lr = 0x830A8770;
	sub_82FD82E0(ctx, base);
	// 830A8770: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A8774: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 830A8778: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830A877C: 409A0018  bne cr6, 0x830a8794
	if !ctx.cr[6].eq {
	pc = 0x830A8794; continue 'dispatch;
	}
	// 830A8780: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A8784: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A8788: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830A878C: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 830A8790: 48000054  b 0x830a87e4
	pc = 0x830A87E4; continue 'dispatch;
	// 830A8794: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830A8798: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830A879C: 40980030  bge cr6, 0x830a87cc
	if !ctx.cr[6].lt {
	pc = 0x830A87CC; continue 'dispatch;
	}
	// 830A87A0: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A87A4: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A87A8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830A87AC: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 830A87B0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 830A87B4: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A87B8: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 830A87BC: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A87C0: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 830A87C4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830A87C8: 4198FFDC  blt cr6, 0x830a87a4
	if ctx.cr[6].lt {
	pc = 0x830A87A4; continue 'dispatch;
	}
	// 830A87CC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A87D0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830A87D4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A87D8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A87DC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830A87E0: 912BFFFC  stw r9, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[9].u32 ) };
	// 830A87E4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A87E8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 830A87EC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830A87F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830A87F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A87F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A87FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A8800: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A8804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A8808 size=12
    let mut pc: u32 = 0x830A8808;
    'dispatch: loop {
        match pc {
            0x830A8808 => {
    //   block [0x830A8808..0x830A8814)
	// 830A8808: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A880C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A8810: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8814(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A8814 size=20
    let mut pc: u32 = 0x830A8814;
    'dispatch: loop {
        match pc {
            0x830A8814 => {
    //   block [0x830A8814..0x830A8828)
	// 830A8814: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 830A8818: 89430004  lbz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A881C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A8820: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830A8824: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A8828 size=16
    let mut pc: u32 = 0x830A8828;
    'dispatch: loop {
        match pc {
            0x830A8828 => {
    //   block [0x830A8828..0x830A8838)
	// 830A8828: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A882C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A8830: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830A8834: 4BF2FAAC  b 0x82fd82e0
	sub_82FD82E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A8838 size=4
    let mut pc: u32 = 0x830A8838;
    'dispatch: loop {
        match pc {
            0x830A8838 => {
    //   block [0x830A8838..0x830A883C)
	// 830A8838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A8840 size=112
    let mut pc: u32 = 0x830A8840;
    'dispatch: loop {
        match pc {
            0x830A8840 => {
    //   block [0x830A8840..0x830A88B0)
	// 830A8840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A8844: 480FF929  bl 0x831a816c
	ctx.lr = 0x830A8848;
	sub_831A8130(ctx, base);
	// 830A8848: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A884C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A8850: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A8854: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A8858: 41820038  beq 0x830a8890
	if ctx.cr[0].eq {
	pc = 0x830A8890; continue 'dispatch;
	}
	// 830A885C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A8860: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A8864: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A8868: 40990028  ble cr6, 0x830a8890
	if !ctx.cr[6].gt {
	pc = 0x830A8890; continue 'dispatch;
	}
	// 830A886C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830A8870: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A8874: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 830A8878: 4BF2FA69  bl 0x82fd82e0
	ctx.lr = 0x830A887C;
	sub_82FD82E0(ctx, base);
	// 830A887C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A8880: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830A8884: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 830A8888: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830A888C: 4198FFE4  blt cr6, 0x830a8870
	if ctx.cr[6].lt {
	pc = 0x830A8870; continue 'dispatch;
	}
	// 830A8890: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A8894: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A8898: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A889C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A88A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A88A4: 4E800421  bctrl
	ctx.lr = 0x830A88A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A88A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A88AC: 480FF910  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A88B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A88B0 size=128
    let mut pc: u32 = 0x830A88B0;
    'dispatch: loop {
        match pc {
            0x830A88B0 => {
    //   block [0x830A88B0..0x830A8930)
	// 830A88B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A88B4: 480FF8B9  bl 0x831a816c
	ctx.lr = 0x830A88B8;
	sub_831A8130(ctx, base);
	// 830A88B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A88BC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A88C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A88C4: 396B00FC  addi r11, r11, 0xfc
	ctx.r[11].s64 = ctx.r[11].s64 + 252;
	// 830A88C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A88CC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A88D0: 57C4103A  slwi r4, r30, 2
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830A88D4: 98BF0004  stb r5, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 830A88D8: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 830A88DC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A88E0: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 830A88E4: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 830A88E8: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 830A88EC: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 830A88F0: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A88F4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A88F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A88FC: 4E800421  bctrl
	ctx.lr = 0x830A8900;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A8900: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830A8904: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 830A8908: 419A001C  beq cr6, 0x830a8924
	if ctx.cr[6].eq {
	pc = 0x830A8924; continue 'dispatch;
	}
	// 830A890C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 830A8910: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A8914: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830A8918: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 830A891C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 830A8920: 4082FFF0  bne 0x830a8910
	if !ctx.cr[0].eq {
	pc = 0x830A8910; continue 'dispatch;
	}
	// 830A8924: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A8928: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A892C: 480FF890  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A8930 size=76
    let mut pc: u32 = 0x830A8930;
    'dispatch: loop {
        match pc {
            0x830A8930 => {
    //   block [0x830A8930..0x830A897C)
	// 830A8930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A8934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A8938: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A893C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A8940: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A8944: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A8948: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A894C: 4BFFE92D  bl 0x830a7278
	ctx.lr = 0x830A8950;
	sub_830A7278(ctx, base);
	// 830A8950: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A8954: 4182000C  beq 0x830a8960
	if ctx.cr[0].eq {
	pc = 0x830A8960; continue 'dispatch;
	}
	// 830A8958: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A895C: 4BF2F985  bl 0x82fd82e0
	ctx.lr = 0x830A8960;
	sub_82FD82E0(ctx, base);
	// 830A8960: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A8964: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A8968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A896C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A8970: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A8974: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A8978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A8980 size=8
    let mut pc: u32 = 0x830A8980;
    'dispatch: loop {
        match pc {
            0x830A8980 => {
    //   block [0x830A8980..0x830A8988)
	// 830A8980: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A8984: 82180138  lwz r16, 0x138(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(312 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A8988 size=144
    let mut pc: u32 = 0x830A8988;
    'dispatch: loop {
        match pc {
            0x830A8988 => {
    //   block [0x830A8988..0x830A8A18)
	// 830A8988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A898C: 480FF7DD  bl 0x831a8168
	ctx.lr = 0x830A8990;
	sub_831A8130(ctx, base);
	// 830A8990: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A8994: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A8998: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A899C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A89A0: 396B0114  addi r11, r11, 0x114
	ctx.r[11].s64 = ctx.r[11].s64 + 276;
	// 830A89A4: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830A89A8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A89AC: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A89B0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A89B4: 41820038  beq 0x830a89ec
	if ctx.cr[0].eq {
	pc = 0x830A89EC; continue 'dispatch;
	}
	// 830A89B8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A89BC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830A89C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A89C4: 40990028  ble cr6, 0x830a89ec
	if !ctx.cr[6].gt {
	pc = 0x830A89EC; continue 'dispatch;
	}
	// 830A89C8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A89CC: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A89D0: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 830A89D4: 4BF2F90D  bl 0x82fd82e0
	ctx.lr = 0x830A89D8;
	sub_82FD82E0(ctx, base);
	// 830A89D8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A89DC: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 830A89E0: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 830A89E4: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830A89E8: 4198FFE4  blt cr6, 0x830a89cc
	if ctx.cr[6].lt {
	pc = 0x830A89CC; continue 'dispatch;
	}
	// 830A89EC: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A89F0: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A89F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A89F8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A89FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A8A00: 4E800421  bctrl
	ctx.lr = 0x830A8A04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A8A04: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A8A08: 396B00FC  addi r11, r11, 0xfc
	ctx.r[11].s64 = ctx.r[11].s64 + 252;
	// 830A8A0C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A8A10: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A8A14: 480FF7A4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A8A18 size=40
    let mut pc: u32 = 0x830A8A18;
    'dispatch: loop {
        match pc {
            0x830A8A18 => {
    //   block [0x830A8A18..0x830A8A40)
	// 830A8A18: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A8A1C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A8A20: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A8A24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A8A28: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A8A2C: 4BFFFBDD  bl 0x830a8608
	ctx.lr = 0x830A8A30;
	sub_830A8608(ctx, base);
	// 830A8A30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A8A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A8A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A8A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A8A40 size=76
    let mut pc: u32 = 0x830A8A40;
    'dispatch: loop {
        match pc {
            0x830A8A40 => {
    //   block [0x830A8A40..0x830A8A8C)
	// 830A8A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A8A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A8A48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A8A4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A8A50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A8A54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A8A58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A8A5C: 4BFFFF2D  bl 0x830a8988
	ctx.lr = 0x830A8A60;
	sub_830A8988(ctx, base);
	// 830A8A60: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A8A64: 4182000C  beq 0x830a8a70
	if ctx.cr[0].eq {
	pc = 0x830A8A70; continue 'dispatch;
	}
	// 830A8A68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A8A6C: 4BF2F875  bl 0x82fd82e0
	ctx.lr = 0x830A8A70;
	sub_82FD82E0(ctx, base);
	// 830A8A70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A8A74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A8A78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A8A7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A8A80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A8A84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A8A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A8A90 size=8
    let mut pc: u32 = 0x830A8A90;
    'dispatch: loop {
        match pc {
            0x830A8A90 => {
    //   block [0x830A8A90..0x830A8A98)
	// 830A8A90: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A8A94: 82180170  lwz r16, 0x170(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(368 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A8A98 size=216
    let mut pc: u32 = 0x830A8A98;
    'dispatch: loop {
        match pc {
            0x830A8A98 => {
    //   block [0x830A8A98..0x830A8B70)
	// 830A8A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A8A9C: 480FF6C9  bl 0x831a8164
	ctx.lr = 0x830A8AA0;
	sub_831A8130(ctx, base);
	// 830A8AA0: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830A8AA4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A8AA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A8AAC: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A8AB0: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A8AB4: 3B8BFFD0  addi r28, r11, -0x30
	ctx.r[28].s64 = ctx.r[11].s64 + -48;
	// 830A8AB8: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 830A8ABC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830A8AC0: 4BFFAEC9  bl 0x830a3988
	ctx.lr = 0x830A8AC4;
	sub_830A3988(ctx, base);
	// 830A8AC4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830A8AC8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 830A8ACC: 997E0008  stb r11, 8(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 830A8AD0: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A8AD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A8AD8: 409A004C  bne cr6, 0x830a8b24
	if !ctx.cr[6].eq {
	pc = 0x830A8B24; continue 'dispatch;
	}
	// 830A8ADC: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830A8AE0: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A8AE4: 4BF2F7B5  bl 0x82fd8298
	ctx.lr = 0x830A8AE8;
	sub_82FD8298(ctx, base);
	// 830A8AE8: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830A8AEC: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 830A8AF0: 4182002C  beq 0x830a8b1c
	if ctx.cr[0].eq {
	pc = 0x830A8B1C; continue 'dispatch;
	}
	// 830A8AF4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830A8AF8: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A8AFC: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 830A8B00: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A8B04: 4BFFFDAD  bl 0x830a88b0
	ctx.lr = 0x830A8B08;
	sub_830A88B0(ctx, base);
	// 830A8B08: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A8B0C: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 830A8B10: 396B0114  addi r11, r11, 0x114
	ctx.r[11].s64 = ctx.r[11].s64 + 276;
	// 830A8B14: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A8B18: 48000008  b 0x830a8b20
	pc = 0x830A8B20; continue 'dispatch;
	// 830A8B1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A8B20: 915E002C  stw r10, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 830A8B24: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 830A8B28: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A8B2C: 4BF2F76D  bl 0x82fd8298
	ctx.lr = 0x830A8B30;
	sub_82FD8298(ctx, base);
	// 830A8B30: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A8B34: 4182001C  beq 0x830a8b50
	if ctx.cr[0].eq {
	pc = 0x830A8B50; continue 'dispatch;
	}
	// 830A8B38: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A8B3C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A8B40: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830A8B44: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 830A8B48: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830A8B4C: 48000008  b 0x830a8b54
	pc = 0x830A8B54; continue 'dispatch;
	// 830A8B50: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A8B54: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A8B58: 4BF925F9  bl 0x8303b150
	ctx.lr = 0x830A8B5C;
	sub_8303B150(ctx, base);
	// 830A8B5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A8B60: 4BFFE7C9  bl 0x830a7328
	ctx.lr = 0x830A8B64;
	sub_830A7328(ctx, base);
	// 830A8B64: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830A8B68: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 830A8B6C: 480FF648  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A8B70 size=48
    let mut pc: u32 = 0x830A8B70;
    'dispatch: loop {
        match pc {
            0x830A8B70 => {
    //   block [0x830A8B70..0x830A8BA0)
	// 830A8B70: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830A8B74: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A8B78: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A8B7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A8B80: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830A8B84: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A8B88: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A8B8C: 4BF2F755  bl 0x82fd82e0
	ctx.lr = 0x830A8B90;
	sub_82FD82E0(ctx, base);
	// 830A8B90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A8B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A8B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A8B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A8BA0 size=8
    let mut pc: u32 = 0x830A8BA0;
    'dispatch: loop {
        match pc {
            0x830A8BA0 => {
    //   block [0x830A8BA0..0x830A8BA8)
	// 830A8BA0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A8BA4: 821801B8  lwz r16, 0x1b8(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(440 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A8BA8 size=300
    let mut pc: u32 = 0x830A8BA8;
    'dispatch: loop {
        match pc {
            0x830A8BA8 => {
    //   block [0x830A8BA8..0x830A8CD4)
	// 830A8BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A8BAC: 480FF5B5  bl 0x831a8160
	ctx.lr = 0x830A8BB0;
	sub_831A8130(ctx, base);
	// 830A8BB0: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 830A8BB4: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A8BB8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A8BBC: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 830A8BC0: 4BFFE769  bl 0x830a7328
	ctx.lr = 0x830A8BC4;
	sub_830A7328(ctx, base);
	// 830A8BC4: A17E0020  lhz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A8BC8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A8BCC: 408200DC  bne 0x830a8ca8
	if !ctx.cr[0].eq {
	pc = 0x830A8CA8; continue 'dispatch;
	}
	// 830A8BD0: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A8BD4: 2F0B007B  cmpwi cr6, r11, 0x7b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 123, &mut ctx.xer);
	// 830A8BD8: 409A00D0  bne cr6, 0x830a8ca8
	if !ctx.cr[6].eq {
	pc = 0x830A8CA8; continue 'dispatch;
	}
	// 830A8BDC: 837E0010  lwz r27, 0x10(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A8BE0: 3880007D  li r4, 0x7d
	ctx.r[4].s64 = 125;
	// 830A8BE4: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A8BE8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 830A8BEC: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A8BF0: 4BF29209  bl 0x82fd1df8
	ctx.lr = 0x830A8BF4;
	sub_82FD1DF8(ctx, base);
	// 830A8BF4: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830A8BF8: 40800030  bge 0x830a8c28
	if !ctx.cr[0].lt {
	pc = 0x830A8C28; continue 'dispatch;
	}
	// 830A8BFC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A8C00: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A8C04: 38C00086  li r6, 0x86
	ctx.r[6].s64 = 134;
	// 830A8C08: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A8C0C: 38A0042D  li r5, 0x42d
	ctx.r[5].s64 = 1069;
	// 830A8C10: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A8C14: 4BFFE575  bl 0x830a7188
	ctx.lr = 0x830A8C18;
	sub_830A7188(ctx, base);
	// 830A8C18: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A8C1C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A8C20: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A8C24: 48108005  bl 0x831b0c28
	ctx.lr = 0x830A8C28;
	sub_831B0C28(ctx, base);
	// 830A8C28: 395C0001  addi r10, r28, 1
	ctx.r[10].s64 = ctx.r[28].s64 + 1;
	// 830A8C2C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A8C30: 7D7BE050  subf r11, r27, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[27].s64;
	// 830A8C34: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830A8C38: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830A8C3C: 915E0010  stw r10, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 830A8C40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A8C44: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A8C48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A8C4C: 4E800421  bctrl
	ctx.lr = 0x830A8C50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A8C50: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A8C54: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A8C58: 90FF0054  stw r7, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 830A8C5C: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 830A8C60: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 830A8C64: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 830A8C68: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A8C6C: 4BF2941D  bl 0x82fd2088
	ctx.lr = 0x830A8C70;
	sub_82FD2088(ctx, base);
	// 830A8C70: 397AFF90  addi r11, r26, -0x70
	ctx.r[11].s64 = ctx.r[26].s64 + -112;
	// 830A8C74: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A8C78: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A8C7C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830A8C80: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830A8C84: 69650001  xori r5, r11, 1
	ctx.r[5].u64 = ctx.r[11].u64 ^ 1;
	// 830A8C88: 4BFFAFB1  bl 0x830a3c38
	ctx.lr = 0x830A8C8C;
	sub_830A3C38(ctx, base);
	// 830A8C8C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A8C90: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A8C94: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830A8C98: 4BF29E29  bl 0x82fd2ac0
	ctx.lr = 0x830A8C9C;
	sub_82FD2AC0(ctx, base);
	// 830A8C9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A8CA0: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 830A8CA4: 480FF50C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 830A8CA8: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A8CAC: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A8CB0: 38C00085  li r6, 0x85
	ctx.r[6].s64 = 133;
	// 830A8CB4: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A8CB8: 38A00427  li r5, 0x427
	ctx.r[5].s64 = 1063;
	// 830A8CBC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A8CC0: 4BFFE4C9  bl 0x830a7188
	ctx.lr = 0x830A8CC4;
	sub_830A7188(ctx, base);
	// 830A8CC4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A8CC8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A8CCC: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A8CD0: 48107F59  bl 0x831b0c28
	ctx.lr = 0x830A8CD4;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8CD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A8CD4 size=40
    let mut pc: u32 = 0x830A8CD4;
    'dispatch: loop {
        match pc {
            0x830A8CD4 => {
    //   block [0x830A8CD4..0x830A8CFC)
	// 830A8CD4: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 830A8CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A8CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A8CE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A8CE4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830A8CE8: 4BF2A171  bl 0x82fd2e58
	ctx.lr = 0x830A8CEC;
	sub_82FD2E58(ctx, base);
	// 830A8CEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A8CF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A8CF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A8CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A8D00 size=8
    let mut pc: u32 = 0x830A8D00;
    'dispatch: loop {
        match pc {
            0x830A8D00 => {
    //   block [0x830A8D00..0x830A8D08)
	// 830A8D00: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A8D04: 82180208  lwz r16, 0x208(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(520 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A8D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A8D08 size=1404
    let mut pc: u32 = 0x830A8D08;
    'dispatch: loop {
        match pc {
            0x830A8D08 => {
    //   block [0x830A8D08..0x830A9284)
	// 830A8D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A8D0C: 480FF43D  bl 0x831a8148
	ctx.lr = 0x830A8D10;
	sub_831A8130(ctx, base);
	// 830A8D10: 3BE1FF20  addi r31, r1, -0xe0
	ctx.r[31].s64 = ctx.r[1].s64 + -224;
	// 830A8D14: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A8D18: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A8D1C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830A8D20: 7C952378  mr r21, r4
	ctx.r[21].u64 = ctx.r[4].u64;
	// 830A8D24: B17E0018  sth r11, 0x18(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u16 ) };
	// 830A8D28: 4BFFE601  bl 0x830a7328
	ctx.lr = 0x830A8D2C;
	sub_830A7328(ctx, base);
	// 830A8D2C: A17E0020  lhz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A8D30: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 830A8D34: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 830A8D38: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A8D3C: 40820058  bne 0x830a8d94
	if !ctx.cr[0].eq {
	pc = 0x830A8D94; continue 'dispatch;
	}
	// 830A8D40: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A8D44: 2F0B005E  cmpwi cr6, r11, 0x5e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 94, &mut ctx.xer);
	// 830A8D48: 409A004C  bne cr6, 0x830a8d94
	if !ctx.cr[6].eq {
	pc = 0x830A8D94; continue 'dispatch;
	}
	// 830A8D4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A8D50: 3A800001  li r20, 1
	ctx.r[20].s64 = 1;
	// 830A8D54: 4BFFE5D5  bl 0x830a7328
	ctx.lr = 0x830A8D58;
	sub_830A7328(ctx, base);
	// 830A8D58: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A8D5C: 56AB063F  clrlwi. r11, r21, 0x18
	ctx.r[11].u64 = ctx.r[21].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A8D60: 4182000C  beq 0x830a8d6c
	if ctx.cr[0].eq {
	pc = 0x830A8D6C; continue 'dispatch;
	}
	// 830A8D64: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830A8D68: 48000034  b 0x830a8d9c
	pc = 0x830A8D9C; continue 'dispatch;
	// 830A8D6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A8D70: 4BFFAA01  bl 0x830a3770
	ctx.lr = 0x830A8D74;
	sub_830A3770(ctx, base);
	// 830A8D74: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A8D78: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 830A8D7C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A8D80: 80ABFD68  lwz r5, -0x298(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-664 as u32) ) } as u64;
	// 830A8D84: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A8D88: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A8D8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A8D90: 4E800421  bctrl
	ctx.lr = 0x830A8D94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A8D94: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A8D98: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A8D9C: 4BFFA9D5  bl 0x830a3770
	ctx.lr = 0x830A8DA0;
	sub_830A3770(ctx, base);
	// 830A8DA0: A17E0020  lhz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A8DA4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 830A8DA8: 3B000001  li r24, 1
	ctx.r[24].s64 = 1;
	// 830A8DAC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 830A8DB0: 419A0350  beq cr6, 0x830a9100
	if ctx.cr[6].eq {
	pc = 0x830A9100; continue 'dispatch;
	}
	// 830A8DB4: 3EC08217  lis r22, -0x7de9
	ctx.r[22].s64 = -2112421888;
	// 830A8DB8: A17E0020  lhz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A8DBC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A8DC0: 40820018  bne 0x830a8dd8
	if !ctx.cr[0].eq {
	pc = 0x830A8DD8; continue 'dispatch;
	}
	// 830A8DC4: 815E0024  lwz r10, 0x24(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A8DC8: 2F0A005D  cmpwi cr6, r10, 0x5d
	ctx.cr[6].compare_i32(ctx.r[10].s32, 93, &mut ctx.xer);
	// 830A8DCC: 409A000C  bne cr6, 0x830a8dd8
	if !ctx.cr[6].eq {
	pc = 0x830A8DD8; continue 'dispatch;
	}
	// 830A8DD0: 570A063F  clrlwi. r10, r24, 0x18
	ctx.r[10].u64 = ctx.r[24].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830A8DD4: 4182032C  beq 0x830a9100
	if ctx.cr[0].eq {
	pc = 0x830A9100; continue 'dispatch;
	}
	// 830A8DD8: 833E0024  lwz r25, 0x24(r30)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A8DDC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A8DE0: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 830A8DE4: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 830A8DE8: 409A0104  bne cr6, 0x830a8eec
	if !ctx.cr[6].eq {
	pc = 0x830A8EEC; continue 'dispatch;
	}
	// 830A8DEC: 2F190063  cmpwi cr6, r25, 0x63
	ctx.cr[6].compare_i32(ctx.r[25].s32, 99, &mut ctx.xer);
	// 830A8DF0: 41990038  bgt cr6, 0x830a8e28
	if ctx.cr[6].gt {
	pc = 0x830A8E28; continue 'dispatch;
	}
	// 830A8DF4: 419A0098  beq cr6, 0x830a8e8c
	if ctx.cr[6].eq {
	pc = 0x830A8E8C; continue 'dispatch;
	}
	// 830A8DF8: 2F190043  cmpwi cr6, r25, 0x43
	ctx.cr[6].compare_i32(ctx.r[25].s32, 67, &mut ctx.xer);
	// 830A8DFC: 419A0090  beq cr6, 0x830a8e8c
	if ctx.cr[6].eq {
	pc = 0x830A8E8C; continue 'dispatch;
	}
	// 830A8E00: 2F190044  cmpwi cr6, r25, 0x44
	ctx.cr[6].compare_i32(ctx.r[25].s32, 68, &mut ctx.xer);
	// 830A8E04: 419A00B0  beq cr6, 0x830a8eb4
	if ctx.cr[6].eq {
	pc = 0x830A8EB4; continue 'dispatch;
	}
	// 830A8E08: 2F190049  cmpwi cr6, r25, 0x49
	ctx.cr[6].compare_i32(ctx.r[25].s32, 73, &mut ctx.xer);
	// 830A8E0C: 419A0080  beq cr6, 0x830a8e8c
	if ctx.cr[6].eq {
	pc = 0x830A8E8C; continue 'dispatch;
	}
	// 830A8E10: 2F190050  cmpwi cr6, r25, 0x50
	ctx.cr[6].compare_i32(ctx.r[25].s32, 80, &mut ctx.xer);
	// 830A8E14: 419A0058  beq cr6, 0x830a8e6c
	if ctx.cr[6].eq {
	pc = 0x830A8E6C; continue 'dispatch;
	}
	// 830A8E18: 2F190053  cmpwi cr6, r25, 0x53
	ctx.cr[6].compare_i32(ctx.r[25].s32, 83, &mut ctx.xer);
	// 830A8E1C: 419A0098  beq cr6, 0x830a8eb4
	if ctx.cr[6].eq {
	pc = 0x830A8EB4; continue 'dispatch;
	}
	// 830A8E20: 2F190057  cmpwi cr6, r25, 0x57
	ctx.cr[6].compare_i32(ctx.r[25].s32, 87, &mut ctx.xer);
	// 830A8E24: 48000028  b 0x830a8e4c
	pc = 0x830A8E4C; continue 'dispatch;
	// 830A8E28: 2F190064  cmpwi cr6, r25, 0x64
	ctx.cr[6].compare_i32(ctx.r[25].s32, 100, &mut ctx.xer);
	// 830A8E2C: 419A0088  beq cr6, 0x830a8eb4
	if ctx.cr[6].eq {
	pc = 0x830A8EB4; continue 'dispatch;
	}
	// 830A8E30: 2F190069  cmpwi cr6, r25, 0x69
	ctx.cr[6].compare_i32(ctx.r[25].s32, 105, &mut ctx.xer);
	// 830A8E34: 419A0058  beq cr6, 0x830a8e8c
	if ctx.cr[6].eq {
	pc = 0x830A8E8C; continue 'dispatch;
	}
	// 830A8E38: 2F190070  cmpwi cr6, r25, 0x70
	ctx.cr[6].compare_i32(ctx.r[25].s32, 112, &mut ctx.xer);
	// 830A8E3C: 419A0030  beq cr6, 0x830a8e6c
	if ctx.cr[6].eq {
	pc = 0x830A8E6C; continue 'dispatch;
	}
	// 830A8E40: 2F190073  cmpwi cr6, r25, 0x73
	ctx.cr[6].compare_i32(ctx.r[25].s32, 115, &mut ctx.xer);
	// 830A8E44: 419A0070  beq cr6, 0x830a8eb4
	if ctx.cr[6].eq {
	pc = 0x830A8EB4; continue 'dispatch;
	}
	// 830A8E48: 2F190077  cmpwi cr6, r25, 0x77
	ctx.cr[6].compare_i32(ctx.r[25].s32, 119, &mut ctx.xer);
	// 830A8E4C: 419A0068  beq cr6, 0x830a8eb4
	if ctx.cr[6].eq {
	pc = 0x830A8EB4; continue 'dispatch;
	}
	// 830A8E50: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A8E54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A8E58: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A8E5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A8E60: 4E800421  bctrl
	ctx.lr = 0x830A8E64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A8E64: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 830A8E68: 48000188  b 0x830a8ff0
	pc = 0x830A8FF0; continue 'dispatch;
	// 830A8E6C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830A8E70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A8E74: 4BFFFD35  bl 0x830a8ba8
	ctx.lr = 0x830A8E78;
	sub_830A8BA8(ctx, base);
	// 830A8E78: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 830A8E7C: 418202BC  beq 0x830a9138
	if ctx.cr[0].eq {
	pc = 0x830A9138; continue 'dispatch;
	}
	// 830A8E80: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A8E84: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A8E88: 48000050  b 0x830a8ed8
	pc = 0x830A8ED8; continue 'dispatch;
	// 830A8E8C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A8E90: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 830A8E94: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830A8E98: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A8E9C: 816B0078  lwz r11, 0x78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) } as u64;
	// 830A8EA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A8EA4: 4E800421  bctrl
	ctx.lr = 0x830A8EA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A8EA8: 7C791B79  or. r25, r3, r3
	ctx.r[25].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 830A8EAC: 40800144  bge 0x830a8ff0
	if !ctx.cr[0].lt {
	pc = 0x830A8FF0; continue 'dispatch;
	}
	// 830A8EB0: 48000034  b 0x830a8ee4
	pc = 0x830A8EE4; continue 'dispatch;
	// 830A8EB4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A8EB8: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830A8EBC: 83BA0000  lwz r29, 0(r26)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A8EC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A8EC4: 816B007C  lwz r11, 0x7c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830A8EC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A8ECC: 4E800421  bctrl
	ctx.lr = 0x830A8ED0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A8ED0: 817D0030  lwz r11, 0x30(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A8ED4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A8ED8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830A8EDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A8EE0: 4E800421  bctrl
	ctx.lr = 0x830A8EE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A8EE4: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 830A8EE8: 48000108  b 0x830a8ff0
	pc = 0x830A8FF0; continue 'dispatch;
	// 830A8EEC: 2B0B0014  cmplwi cr6, r11, 0x14
	ctx.cr[6].compare_u32(ctx.r[11].u32, 20 as u32, &mut ctx.xer);
	// 830A8EF0: 409A0100  bne cr6, 0x830a8ff0
	if !ctx.cr[6].eq {
	pc = 0x830A8FF0; continue 'dispatch;
	}
	// 830A8EF4: 3880003A  li r4, 0x3a
	ctx.r[4].s64 = 58;
	// 830A8EF8: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A8EFC: 80BE0010  lwz r5, 0x10(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A8F00: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A8F04: 4BF28EF5  bl 0x82fd1df8
	ctx.lr = 0x830A8F08;
	sub_82FD1DF8(ctx, base);
	// 830A8F08: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830A8F0C: 41800258  blt 0x830a9164
	if ctx.cr[0].lt {
	pc = 0x830A9164; continue 'dispatch;
	}
	// 830A8F10: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A8F14: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 830A8F18: 815E0028  lwz r10, 0x28(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A8F1C: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830A8F20: 7D49522E  lhzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830A8F24: 2B0A005E  cmplwi cr6, r10, 0x5e
	ctx.cr[6].compare_u32(ctx.r[10].u32, 94 as u32, &mut ctx.xer);
	// 830A8F28: 409A0010  bne cr6, 0x830a8f38
	if !ctx.cr[6].eq {
	pc = 0x830A8F38; continue 'dispatch;
	}
	// 830A8F2C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830A8F30: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 830A8F34: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830A8F38: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A8F3C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A8F40: 7D6BE050  subf r11, r11, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[11].s64;
	// 830A8F44: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830A8F48: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830A8F4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A8F50: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A8F54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A8F58: 4E800421  bctrl
	ctx.lr = 0x830A8F5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A8F5C: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A8F60: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A8F64: 90FF0054  stw r7, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 830A8F68: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 830A8F6C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 830A8F70: 80BE0010  lwz r5, 0x10(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A8F74: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A8F78: 4BF29111  bl 0x82fd2088
	ctx.lr = 0x830A8F7C;
	sub_82FD2088(ctx, base);
	// 830A8F7C: 576B063E  clrlwi r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	// 830A8F80: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A8F84: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A8F88: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830A8F8C: 5565DFFE  rlwinm r5, r11, 0x1b, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830A8F90: 4BFFACA9  bl 0x830a3c38
	ctx.lr = 0x830A8F94;
	sub_830A3C38(ctx, base);
	// 830A8F94: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 830A8F98: 418201F8  beq 0x830a9190
	if ctx.cr[0].eq {
	pc = 0x830A9190; continue 'dispatch;
	}
	// 830A8F9C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A8FA0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830A8FA4: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A8FA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A8FAC: 4E800421  bctrl
	ctx.lr = 0x830A8FB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A8FB0: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A8FB4: 395C0001  addi r10, r28, 1
	ctx.r[10].s64 = ctx.r[28].s64 + 1;
	// 830A8FB8: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 830A8FBC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830A8FC0: 409801FC  bge cr6, 0x830a91bc
	if !ctx.cr[6].lt {
	pc = 0x830A91BC; continue 'dispatch;
	}
	// 830A8FC4: 397C0001  addi r11, r28, 1
	ctx.r[11].s64 = ctx.r[28].s64 + 1;
	// 830A8FC8: 815E0028  lwz r10, 0x28(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A8FCC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A8FD0: 7D6B522E  lhzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830A8FD4: 2B0B005D  cmplwi cr6, r11, 0x5d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 93 as u32, &mut ctx.xer);
	// 830A8FD8: 409A01E4  bne cr6, 0x830a91bc
	if !ctx.cr[6].eq {
	pc = 0x830A91BC; continue 'dispatch;
	}
	// 830A8FDC: 397C0002  addi r11, r28, 2
	ctx.r[11].s64 = ctx.r[28].s64 + 2;
	// 830A8FE0: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830A8FE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A8FE8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830A8FEC: 4BF29AD5  bl 0x82fd2ac0
	ctx.lr = 0x830A8FF0;
	sub_82FD2AC0(ctx, base);
	// 830A8FF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A8FF4: 4BFFE335  bl 0x830a7328
	ctx.lr = 0x830A8FF8;
	sub_830A7328(ctx, base);
	// 830A8FF8: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A8FFC: 408200BC  bne 0x830a90b8
	if !ctx.cr[0].eq {
	pc = 0x830A90B8; continue 'dispatch;
	}
	// 830A9000: A17E0020  lhz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A9004: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A9008: 40820094  bne 0x830a909c
	if !ctx.cr[0].eq {
	pc = 0x830A909C; continue 'dispatch;
	}
	// 830A900C: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A9010: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 830A9014: 409A0088  bne cr6, 0x830a909c
	if !ctx.cr[6].eq {
	pc = 0x830A909C; continue 'dispatch;
	}
	// 830A9018: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A901C: 4BFFE30D  bl 0x830a7328
	ctx.lr = 0x830A9020;
	sub_830A7328(ctx, base);
	// 830A9020: A17E0020  lhz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A9024: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 830A9028: 419A01C0  beq cr6, 0x830a91e8
	if ctx.cr[6].eq {
	pc = 0x830A91E8; continue 'dispatch;
	}
	// 830A902C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A9030: 409A0038  bne cr6, 0x830a9068
	if !ctx.cr[6].eq {
	pc = 0x830A9068; continue 'dispatch;
	}
	// 830A9034: 815E0024  lwz r10, 0x24(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A9038: 2F0A005D  cmpwi cr6, r10, 0x5d
	ctx.cr[6].compare_i32(ctx.r[10].s32, 93, &mut ctx.xer);
	// 830A903C: 409A002C  bne cr6, 0x830a9068
	if !ctx.cr[6].eq {
	pc = 0x830A9068; continue 'dispatch;
	}
	// 830A9040: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9044: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 830A9048: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830A904C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830A9050: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A9054: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A9058: 4E800421  bctrl
	ctx.lr = 0x830A905C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A905C: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 830A9060: 3880002D  li r4, 0x2d
	ctx.r[4].s64 = 45;
	// 830A9064: 48000040  b 0x830a90a4
	pc = 0x830A90A4; continue 'dispatch;
	// 830A9068: 83BE0024  lwz r29, 0x24(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A906C: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 830A9070: 409A001C  bne cr6, 0x830a908c
	if !ctx.cr[6].eq {
	pc = 0x830A908C; continue 'dispatch;
	}
	// 830A9074: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9078: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A907C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A9080: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A9084: 4E800421  bctrl
	ctx.lr = 0x830A9088;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A9088: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A908C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A9090: 4BFFE299  bl 0x830a7328
	ctx.lr = 0x830A9094;
	sub_830A7328(ctx, base);
	// 830A9094: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A9098: 48000008  b 0x830a90a0
	pc = 0x830A90A0; continue 'dispatch;
	// 830A909C: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 830A90A0: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830A90A4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A90A8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830A90AC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A90B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A90B4: 4E800421  bctrl
	ctx.lr = 0x830A90B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A90B8: 8176ACBC  lwz r11, -0x5344(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-21316 as u32) ) } as u64;
	// 830A90BC: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830A90C0: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 830A90C4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 830A90C8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830A90CC: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A90D0: 41820024  beq 0x830a90f4
	if ctx.cr[0].eq {
	pc = 0x830A90F4; continue 'dispatch;
	}
	// 830A90D4: A17E0020  lhz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A90D8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A90DC: 40820018  bne 0x830a90f4
	if !ctx.cr[0].eq {
	pc = 0x830A90F4; continue 'dispatch;
	}
	// 830A90E0: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A90E4: 2F0B002C  cmpwi cr6, r11, 0x2c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 44, &mut ctx.xer);
	// 830A90E8: 409A000C  bne cr6, 0x830a90f4
	if !ctx.cr[6].eq {
	pc = 0x830A90F4; continue 'dispatch;
	}
	// 830A90EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A90F0: 4BFFE239  bl 0x830a7328
	ctx.lr = 0x830A90F4;
	sub_830A7328(ctx, base);
	// 830A90F4: A17E0020  lhz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A90F8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 830A90FC: 409AFCBC  bne cr6, 0x830a8db8
	if !ctx.cr[6].eq {
	pc = 0x830A8DB8; continue 'dispatch;
	}
	// 830A9100: A17E0020  lhz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A9104: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 830A9108: 409A010C  bne cr6, 0x830a9214
	if !ctx.cr[6].eq {
	pc = 0x830A9214; continue 'dispatch;
	}
	// 830A910C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A9110: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A9114: 38C0008A  li r6, 0x8a
	ctx.r[6].s64 = 138;
	// 830A9118: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A911C: 38A004DD  li r5, 0x4dd
	ctx.r[5].s64 = 1245;
	// 830A9120: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A9124: 4BFFE065  bl 0x830a7188
	ctx.lr = 0x830A9128;
	sub_830A7188(ctx, base);
	// 830A9128: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A912C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A9130: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A9134: 48107AF5  bl 0x831b0c28
	ctx.lr = 0x830A9138;
	sub_831B0C28(ctx, base);
	// 830A9138: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A913C: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A9140: 38C00088  li r6, 0x88
	ctx.r[6].s64 = 136;
	// 830A9144: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A9148: 38A00484  li r5, 0x484
	ctx.r[5].s64 = 1156;
	// 830A914C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A9150: 4BFFE039  bl 0x830a7188
	ctx.lr = 0x830A9154;
	sub_830A7188(ctx, base);
	// 830A9154: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A9158: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A915C: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A9160: 48107AC9  bl 0x831b0c28
	ctx.lr = 0x830A9164;
	sub_831B0C28(ctx, base);
	// 830A9164: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A9168: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A916C: 38C00089  li r6, 0x89
	ctx.r[6].s64 = 137;
	// 830A9170: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A9174: 38A00494  li r5, 0x494
	ctx.r[5].s64 = 1172;
	// 830A9178: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A917C: 4BFFE00D  bl 0x830a7188
	ctx.lr = 0x830A9180;
	sub_830A7188(ctx, base);
	// 830A9180: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A9184: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A9188: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A918C: 48107A9D  bl 0x831b0c28
	ctx.lr = 0x830A9190;
	sub_831B0C28(ctx, base);
	// 830A9190: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A9194: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A9198: 38C0008B  li r6, 0x8b
	ctx.r[6].s64 = 139;
	// 830A919C: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A91A0: 38A004A9  li r5, 0x4a9
	ctx.r[5].s64 = 1193;
	// 830A91A4: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A91A8: 4BFFDFE1  bl 0x830a7188
	ctx.lr = 0x830A91AC;
	sub_830A7188(ctx, base);
	// 830A91AC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A91B0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A91B4: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A91B8: 48107A71  bl 0x831b0c28
	ctx.lr = 0x830A91BC;
	sub_831B0C28(ctx, base);
	// 830A91BC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A91C0: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A91C4: 38C00089  li r6, 0x89
	ctx.r[6].s64 = 137;
	// 830A91C8: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A91CC: 38A004B0  li r5, 0x4b0
	ctx.r[5].s64 = 1200;
	// 830A91D0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A91D4: 4BFFDFB5  bl 0x830a7188
	ctx.lr = 0x830A91D8;
	sub_830A7188(ctx, base);
	// 830A91D8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A91DC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A91E0: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A91E4: 48107A45  bl 0x831b0c28
	ctx.lr = 0x830A91E8;
	sub_831B0C28(ctx, base);
	// 830A91E8: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A91EC: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A91F0: 38C0008A  li r6, 0x8a
	ctx.r[6].s64 = 138;
	// 830A91F4: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A91F8: 38A004C1  li r5, 0x4c1
	ctx.r[5].s64 = 1217;
	// 830A91FC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A9200: 4BFFDF89  bl 0x830a7188
	ctx.lr = 0x830A9204;
	sub_830A7188(ctx, base);
	// 830A9204: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A9208: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A920C: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A9210: 48107A19  bl 0x831b0c28
	ctx.lr = 0x830A9214;
	sub_831B0C28(ctx, base);
	// 830A9214: 56AB063F  clrlwi. r11, r21, 0x18
	ctx.r[11].u64 = ctx.r[21].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A9218: 40820028  bne 0x830a9240
	if !ctx.cr[0].eq {
	pc = 0x830A9240; continue 'dispatch;
	}
	// 830A921C: 568B063F  clrlwi. r11, r20, 0x18
	ctx.r[11].u64 = ctx.r[20].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A9220: 41820020  beq 0x830a9240
	if ctx.cr[0].eq {
	pc = 0x830A9240; continue 'dispatch;
	}
	// 830A9224: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9228: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830A922C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 830A9230: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A9234: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A9238: 4E800421  bctrl
	ctx.lr = 0x830A923C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A923C: 7EFABB78  mr r26, r23
	ctx.r[26].u64 = ctx.r[23].u64;
	// 830A9240: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9244: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830A9248: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830A924C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A9250: 4E800421  bctrl
	ctx.lr = 0x830A9254;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A9254: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9258: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830A925C: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 830A9260: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A9264: 4E800421  bctrl
	ctx.lr = 0x830A9268;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A9268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830A926C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A9270: B17E0018  sth r11, 0x18(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u16 ) };
	// 830A9274: 4BFFE0B5  bl 0x830a7328
	ctx.lr = 0x830A9278;
	sub_830A7328(ctx, base);
	// 830A9278: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830A927C: 383F00E0  addi r1, r31, 0xe0
	ctx.r[1].s64 = ctx.r[31].s64 + 224;
	// 830A9280: 480FEF18  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A9284(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A9284 size=40
    let mut pc: u32 = 0x830A9284;
    'dispatch: loop {
        match pc {
            0x830A9284 => {
    //   block [0x830A9284..0x830A92AC)
	// 830A9284: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 830A9288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A928C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A9290: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A9294: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830A9298: 4BF29BC1  bl 0x82fd2e58
	ctx.lr = 0x830A929C;
	sub_82FD2E58(ctx, base);
	// 830A929C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A92A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A92A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A92A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A92B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A92B0 size=8
    let mut pc: u32 = 0x830A92B0;
    'dispatch: loop {
        match pc {
            0x830A92B0 => {
    //   block [0x830A92B0..0x830A92B8)
	// 830A92B0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A92B4: 82180260  lwz r16, 0x260(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(608 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A92B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A92B8 size=796
    let mut pc: u32 = 0x830A92B8;
    'dispatch: loop {
        match pc {
            0x830A92B8 => {
    //   block [0x830A92B8..0x830A95D4)
	// 830A92B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A92BC: 480FEEB1  bl 0x831a816c
	ctx.lr = 0x830A92C0;
	sub_831A8130(ctx, base);
	// 830A92C0: 3BE1FF20  addi r31, r1, -0xe0
	ctx.r[31].s64 = ctx.r[1].s64 + -224;
	// 830A92C4: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A92C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A92CC: A17E0020  lhz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A92D0: 2F0B000D  cmpwi cr6, r11, 0xd
	ctx.cr[6].compare_i32(ctx.r[11].s32, 13, &mut ctx.xer);
	// 830A92D4: 41990274  bgt cr6, 0x830a9548
	if ctx.cr[6].gt {
	pc = 0x830A9548; continue 'dispatch;
	}
	// 830A92D8: 419A0264  beq cr6, 0x830a953c
	if ctx.cr[6].eq {
	pc = 0x830A953C; continue 'dispatch;
	}
	// 830A92DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A92E0: 419A0204  beq cr6, 0x830a94e4
	if ctx.cr[6].eq {
	pc = 0x830A94E4; continue 'dispatch;
	}
	// 830A92E4: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 830A92E8: 419A01F0  beq cr6, 0x830a94d8
	if ctx.cr[6].eq {
	pc = 0x830A94D8; continue 'dispatch;
	}
	// 830A92EC: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 830A92F0: 419A01D0  beq cr6, 0x830a94c0
	if ctx.cr[6].eq {
	pc = 0x830A94C0; continue 'dispatch;
	}
	// 830A92F4: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 830A92F8: 419A01AC  beq cr6, 0x830a94a4
	if ctx.cr[6].eq {
	pc = 0x830A94A4; continue 'dispatch;
	}
	// 830A92FC: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 830A9300: 409A0268  bne cr6, 0x830a9568
	if !ctx.cr[6].eq {
	pc = 0x830A9568; continue 'dispatch;
	}
	// 830A9304: 809E0024  lwz r4, 0x24(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A9308: 2F040058  cmpwi cr6, r4, 0x58
	ctx.cr[6].compare_i32(ctx.r[4].s32, 88, &mut ctx.xer);
	// 830A930C: 4199007C  bgt cr6, 0x830a9388
	if ctx.cr[6].gt {
	pc = 0x830A9388; continue 'dispatch;
	}
	// 830A9310: 419A006C  beq cr6, 0x830a937c
	if ctx.cr[6].eq {
	pc = 0x830A937C; continue 'dispatch;
	}
	// 830A9314: 2F040049  cmpwi cr6, r4, 0x49
	ctx.cr[6].compare_i32(ctx.r[4].s32, 73, &mut ctx.xer);
	// 830A9318: 4199004C  bgt cr6, 0x830a9364
	if ctx.cr[6].gt {
	pc = 0x830A9364; continue 'dispatch;
	}
	// 830A931C: 419A003C  beq cr6, 0x830a9358
	if ctx.cr[6].eq {
	pc = 0x830A9358; continue 'dispatch;
	}
	// 830A9320: 2F040030  cmpwi cr6, r4, 0x30
	ctx.cr[6].compare_i32(ctx.r[4].s32, 48, &mut ctx.xer);
	// 830A9324: 4198009C  blt cr6, 0x830a93c0
	if ctx.cr[6].lt {
	pc = 0x830A93C0; continue 'dispatch;
	}
	// 830A9328: 2F040039  cmpwi cr6, r4, 0x39
	ctx.cr[6].compare_i32(ctx.r[4].s32, 57, &mut ctx.xer);
	// 830A932C: 40990020  ble cr6, 0x830a934c
	if !ctx.cr[6].gt {
	pc = 0x830A934C; continue 'dispatch;
	}
	// 830A9330: 2F040043  cmpwi cr6, r4, 0x43
	ctx.cr[6].compare_i32(ctx.r[4].s32, 67, &mut ctx.xer);
	// 830A9334: 419A000C  beq cr6, 0x830a9340
	if ctx.cr[6].eq {
	pc = 0x830A9340; continue 'dispatch;
	}
	// 830A9338: 2F040044  cmpwi cr6, r4, 0x44
	ctx.cr[6].compare_i32(ctx.r[4].s32, 68, &mut ctx.xer);
	// 830A933C: 48000080  b 0x830a93bc
	pc = 0x830A93BC; continue 'dispatch;
	// 830A9340: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9344: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 830A9348: 48000278  b 0x830a95c0
	pc = 0x830A95C0; continue 'dispatch;
	// 830A934C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9350: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 830A9354: 4800026C  b 0x830a95c0
	pc = 0x830A95C0; continue 'dispatch;
	// 830A9358: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A935C: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 830A9360: 48000260  b 0x830a95c0
	pc = 0x830A95C0; continue 'dispatch;
	// 830A9364: 2F040050  cmpwi cr6, r4, 0x50
	ctx.cr[6].compare_i32(ctx.r[4].s32, 80, &mut ctx.xer);
	// 830A9368: 419A00B4  beq cr6, 0x830a941c
	if ctx.cr[6].eq {
	pc = 0x830A941C; continue 'dispatch;
	}
	// 830A936C: 2F040053  cmpwi cr6, r4, 0x53
	ctx.cr[6].compare_i32(ctx.r[4].s32, 83, &mut ctx.xer);
	// 830A9370: 419A0100  beq cr6, 0x830a9470
	if ctx.cr[6].eq {
	pc = 0x830A9470; continue 'dispatch;
	}
	// 830A9374: 2F040057  cmpwi cr6, r4, 0x57
	ctx.cr[6].compare_i32(ctx.r[4].s32, 87, &mut ctx.xer);
	// 830A9378: 48000044  b 0x830a93bc
	pc = 0x830A93BC; continue 'dispatch;
	// 830A937C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9380: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 830A9384: 4800023C  b 0x830a95c0
	pc = 0x830A95C0; continue 'dispatch;
	// 830A9388: 2F040063  cmpwi cr6, r4, 0x63
	ctx.cr[6].compare_i32(ctx.r[4].s32, 99, &mut ctx.xer);
	// 830A938C: 419A010C  beq cr6, 0x830a9498
	if ctx.cr[6].eq {
	pc = 0x830A9498; continue 'dispatch;
	}
	// 830A9390: 2F040064  cmpwi cr6, r4, 0x64
	ctx.cr[6].compare_i32(ctx.r[4].s32, 100, &mut ctx.xer);
	// 830A9394: 419A00DC  beq cr6, 0x830a9470
	if ctx.cr[6].eq {
	pc = 0x830A9470; continue 'dispatch;
	}
	// 830A9398: 2F040067  cmpwi cr6, r4, 0x67
	ctx.cr[6].compare_i32(ctx.r[4].s32, 103, &mut ctx.xer);
	// 830A939C: 419A00C8  beq cr6, 0x830a9464
	if ctx.cr[6].eq {
	pc = 0x830A9464; continue 'dispatch;
	}
	// 830A93A0: 2F040069  cmpwi cr6, r4, 0x69
	ctx.cr[6].compare_i32(ctx.r[4].s32, 105, &mut ctx.xer);
	// 830A93A4: 419A00B4  beq cr6, 0x830a9458
	if ctx.cr[6].eq {
	pc = 0x830A9458; continue 'dispatch;
	}
	// 830A93A8: 2F040070  cmpwi cr6, r4, 0x70
	ctx.cr[6].compare_i32(ctx.r[4].s32, 112, &mut ctx.xer);
	// 830A93AC: 419A0070  beq cr6, 0x830a941c
	if ctx.cr[6].eq {
	pc = 0x830A941C; continue 'dispatch;
	}
	// 830A93B0: 2F040073  cmpwi cr6, r4, 0x73
	ctx.cr[6].compare_i32(ctx.r[4].s32, 115, &mut ctx.xer);
	// 830A93B4: 419A00BC  beq cr6, 0x830a9470
	if ctx.cr[6].eq {
	pc = 0x830A9470; continue 'dispatch;
	}
	// 830A93B8: 2F040077  cmpwi cr6, r4, 0x77
	ctx.cr[6].compare_i32(ctx.r[4].s32, 119, &mut ctx.xer);
	// 830A93BC: 419A00B4  beq cr6, 0x830a9470
	if ctx.cr[6].eq {
	pc = 0x830A9470; continue 'dispatch;
	}
	// 830A93C0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A93C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A93C8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A93CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A93D0: 4E800421  bctrl
	ctx.lr = 0x830A93D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A93D4: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 830A93D8: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830A93DC: 4098000C  bge cr6, 0x830a93e8
	if !ctx.cr[6].lt {
	pc = 0x830A93E8; continue 'dispatch;
	}
	// 830A93E0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A93E4: 4800011C  b 0x830a9500
	pc = 0x830A9500; continue 'dispatch;
	// 830A93E8: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A93EC: 4BFFDBDD  bl 0x830a6fc8
	ctx.lr = 0x830A93F0;
	sub_830A6FC8(ctx, base);
	// 830A93F0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A93F4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A93F8: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 830A93FC: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 830A9400: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A9404: 4BFFA62D  bl 0x830a3a30
	ctx.lr = 0x830A9408;
	sub_830A3A30(ctx, base);
	// 830A9408: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A940C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A9410: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830A9414: 4BF296AD  bl 0x82fd2ac0
	ctx.lr = 0x830A9418;
	sub_82FD2AC0(ctx, base);
	// 830A9418: 48000070  b 0x830a9488
	pc = 0x830A9488; continue 'dispatch;
	// 830A941C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A9420: 4BFFF789  bl 0x830a8ba8
	ctx.lr = 0x830A9424;
	sub_830A8BA8(ctx, base);
	// 830A9424: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830A9428: 40820060  bne 0x830a9488
	if !ctx.cr[0].eq {
	pc = 0x830A9488; continue 'dispatch;
	}
	// 830A942C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A9430: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A9434: 38C00088  li r6, 0x88
	ctx.r[6].s64 = 136;
	// 830A9438: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A943C: 38A003FB  li r5, 0x3fb
	ctx.r[5].s64 = 1019;
	// 830A9440: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A9444: 4BFFDD45  bl 0x830a7188
	ctx.lr = 0x830A9448;
	sub_830A7188(ctx, base);
	// 830A9448: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A944C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A9450: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A9454: 481077D5  bl 0x831b0c28
	ctx.lr = 0x830A9458;
	sub_831B0C28(ctx, base);
	// 830A9458: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A945C: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A9460: 48000160  b 0x830a95c0
	pc = 0x830A95C0; continue 'dispatch;
	// 830A9464: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9468: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 830A946C: 48000154  b 0x830a95c0
	pc = 0x830A95C0; continue 'dispatch;
	// 830A9470: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9474: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A9478: 816B007C  lwz r11, 0x7c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830A947C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A9480: 4E800421  bctrl
	ctx.lr = 0x830A9484;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A9484: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A9488: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A948C: 4BFFDE9D  bl 0x830a7328
	ctx.lr = 0x830A9490;
	sub_830A7328(ctx, base);
	// 830A9490: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A9494: 48000138  b 0x830a95cc
	pc = 0x830A95CC; continue 'dispatch;
	// 830A9498: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A949C: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830A94A0: 48000120  b 0x830a95c0
	pc = 0x830A95C0; continue 'dispatch;
	// 830A94A4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A94A8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830A94AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A94B0: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 830A94B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A94B8: 4E800421  bctrl
	ctx.lr = 0x830A94BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A94BC: 48000110  b 0x830a95cc
	pc = 0x830A95CC; continue 'dispatch;
	// 830A94C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A94C4: 4BFFDE65  bl 0x830a7328
	ctx.lr = 0x830A94C8;
	sub_830A7328(ctx, base);
	// 830A94C8: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A94CC: 4BFFAA4D  bl 0x830a3f18
	ctx.lr = 0x830A94D0;
	sub_830A3F18(ctx, base);
	// 830A94D0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A94D4: 4BFFFFBC  b 0x830a9490
	pc = 0x830A9490; continue 'dispatch;
	// 830A94D8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A94DC: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 830A94E0: 480000E0  b 0x830a95c0
	pc = 0x830A95C0; continue 'dispatch;
	// 830A94E4: 809E0024  lwz r4, 0x24(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A94E8: 2F04007B  cmpwi cr6, r4, 0x7b
	ctx.cr[6].compare_i32(ctx.r[4].s32, 123, &mut ctx.xer);
	// 830A94EC: 419A0024  beq cr6, 0x830a9510
	if ctx.cr[6].eq {
	pc = 0x830A9510; continue 'dispatch;
	}
	// 830A94F0: 2F04007D  cmpwi cr6, r4, 0x7d
	ctx.cr[6].compare_i32(ctx.r[4].s32, 125, &mut ctx.xer);
	// 830A94F4: 419A001C  beq cr6, 0x830a9510
	if ctx.cr[6].eq {
	pc = 0x830A9510; continue 'dispatch;
	}
	// 830A94F8: 2F04005D  cmpwi cr6, r4, 0x5d
	ctx.cr[6].compare_i32(ctx.r[4].s32, 93, &mut ctx.xer);
	// 830A94FC: 419A0014  beq cr6, 0x830a9510
	if ctx.cr[6].eq {
	pc = 0x830A9510; continue 'dispatch;
	}
	// 830A9500: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A9504: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A9508: 4BFFA371  bl 0x830a3878
	ctx.lr = 0x830A950C;
	sub_830A3878(ctx, base);
	// 830A950C: 4BFFFF78  b 0x830a9484
	pc = 0x830A9484; continue 'dispatch;
	// 830A9510: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A9514: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A9518: 38C00087  li r6, 0x87
	ctx.r[6].s64 = 135;
	// 830A951C: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A9520: 38A00415  li r5, 0x415
	ctx.r[5].s64 = 1045;
	// 830A9524: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 830A9528: 4BFFDC61  bl 0x830a7188
	ctx.lr = 0x830A952C;
	sub_830A7188(ctx, base);
	// 830A952C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A9530: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 830A9534: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A9538: 481076F1  bl 0x831b0c28
	ctx.lr = 0x830A953C;
	sub_831B0C28(ctx, base);
	// 830A953C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9540: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 830A9544: 4800007C  b 0x830a95c0
	pc = 0x830A95C0; continue 'dispatch;
	// 830A9548: 2F0B0012  cmpwi cr6, r11, 0x12
	ctx.cr[6].compare_i32(ctx.r[11].s32, 18, &mut ctx.xer);
	// 830A954C: 419A006C  beq cr6, 0x830a95b8
	if ctx.cr[6].eq {
	pc = 0x830A95B8; continue 'dispatch;
	}
	// 830A9550: 2F0B0013  cmpwi cr6, r11, 0x13
	ctx.cr[6].compare_i32(ctx.r[11].s32, 19, &mut ctx.xer);
	// 830A9554: 419A0058  beq cr6, 0x830a95ac
	if ctx.cr[6].eq {
	pc = 0x830A95AC; continue 'dispatch;
	}
	// 830A9558: 2F0B0016  cmpwi cr6, r11, 0x16
	ctx.cr[6].compare_i32(ctx.r[11].s32, 22, &mut ctx.xer);
	// 830A955C: 419A0044  beq cr6, 0x830a95a0
	if ctx.cr[6].eq {
	pc = 0x830A95A0; continue 'dispatch;
	}
	// 830A9560: 2F0B0017  cmpwi cr6, r11, 0x17
	ctx.cr[6].compare_i32(ctx.r[11].s32, 23, &mut ctx.xer);
	// 830A9564: 419A0030  beq cr6, 0x830a9594
	if ctx.cr[6].eq {
	pc = 0x830A9594; continue 'dispatch;
	}
	// 830A9568: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A956C: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A9570: 38C00087  li r6, 0x87
	ctx.r[6].s64 = 135;
	// 830A9574: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A9578: 38A0041B  li r5, 0x41b
	ctx.r[5].s64 = 1051;
	// 830A957C: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 830A9580: 4BFFDC09  bl 0x830a7188
	ctx.lr = 0x830A9584;
	sub_830A7188(ctx, base);
	// 830A9584: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A9588: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 830A958C: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A9590: 48107699  bl 0x831b0c28
	ctx.lr = 0x830A9594;
	sub_831B0C28(ctx, base);
	// 830A9594: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9598: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 830A959C: 48000024  b 0x830a95c0
	pc = 0x830A95C0; continue 'dispatch;
	// 830A95A0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A95A4: 816B0068  lwz r11, 0x68(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 830A95A8: 48000018  b 0x830a95c0
	pc = 0x830A95C0; continue 'dispatch;
	// 830A95AC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A95B0: 816B0074  lwz r11, 0x74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 830A95B4: 4800000C  b 0x830a95c0
	pc = 0x830A95C0; continue 'dispatch;
	// 830A95B8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A95BC: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 830A95C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A95C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A95C8: 4E800421  bctrl
	ctx.lr = 0x830A95CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A95CC: 383F00E0  addi r1, r31, 0xe0
	ctx.r[1].s64 = ctx.r[31].s64 + 224;
	// 830A95D0: 480FEBEC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A95D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A95D4 size=40
    let mut pc: u32 = 0x830A95D4;
    'dispatch: loop {
        match pc {
            0x830A95D4 => {
    //   block [0x830A95D4..0x830A95FC)
	// 830A95D4: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 830A95D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A95DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A95E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A95E4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830A95E8: 4BF29871  bl 0x82fd2e58
	ctx.lr = 0x830A95EC;
	sub_82FD2E58(ctx, base);
	// 830A95EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A95F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A95F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A95F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A9600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A9600 size=1208
    //   switch @ 0x830A9638: r11 with 12 label(s)
    //       case  0 → 0x830A96B8
    //       case  1 → 0x830A9648
    //       case  2 → 0x830A9660
    //       case  3 → 0x830A973C
    //       case  4 → 0x830A966C
    //       case  5 → 0x830A9688
    //       case  6 → 0x830A9690
    //       case  7 → 0x830A9698
    //       case  8 → 0x830A973C
    //       case  9 → 0x830A973C
    //       case 10 → 0x830A973C
    //       case 11 → 0x830A96A0
    let mut pc: u32 = 0x830A9600;
    'dispatch: loop {
        match pc {
            0x830A9600 => {
    //   block [0x830A9600..0x830A9648)
	// 830A9600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A9604: 480FEB65  bl 0x831a8168
	ctx.lr = 0x830A9608;
	sub_831A8130(ctx, base);
	// 830A9608: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A960C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A9610: A17F0020  lhz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A9614: 396BFFF6  addi r11, r11, -0xa
	ctx.r[11].s64 = ctx.r[11].s64 + -10;
	// 830A9618: 2B0B000B  cmplwi cr6, r11, 0xb
	ctx.cr[6].compare_u32(ctx.r[11].u32, 11 as u32, &mut ctx.xer);
	// 830A961C: 41990120  bgt cr6, 0x830a973c
	if ctx.cr[6].gt {
	pc = 0x830A973C; continue 'dispatch;
	}
	// 830A9620: 3D808218  lis r12, -0x7de8
	ctx.r[12].s64 = -2112356352;
	// 830A9624: 398CFFC0  addi r12, r12, -0x40
	ctx.r[12].s64 = ctx.r[12].s64 + -64;
	// 830A9628: 7C0C58AE  lbzx r0, r12, r11
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830A962C: 3D80830B  lis r12, -0x7cf5
	ctx.r[12].s64 = -2096431104;
	// 830A9630: 398C9648  addi r12, r12, -0x69b8
	ctx.r[12].s64 = ctx.r[12].s64 + -27064;
	// 830A9634: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 830A9638: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 830A963C: 60000000  nop
	// 830A9640: 60000000  nop
	// 830A9644: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            0x830A9648 => {
    //   block [0x830A9648..0x830A9660)
	// 830A9648: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A964C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830A9650: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A9654: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A9658: 4E800421  bctrl
	ctx.lr = 0x830A965C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A965C: 48000418  b 0x830a9a74
	pc = 0x830A9A74; continue 'dispatch;
            }
            0x830A9660 => {
    //   block [0x830A9660..0x830A966C)
	// 830A9660: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9664: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A9668: 4BFFFFE8  b 0x830a9650
	pc = 0x830A9650; continue 'dispatch;
            }
            0x830A966C => {
    //   block [0x830A966C..0x830A9688)
	// 830A966C: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 830A9670: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9674: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A9678: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A967C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A9680: 4E800421  bctrl
	ctx.lr = 0x830A9684;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A9684: 480003F0  b 0x830a9a74
	pc = 0x830A9A74; continue 'dispatch;
            }
            0x830A9688 => {
    //   block [0x830A9688..0x830A9690)
	// 830A9688: 38800015  li r4, 0x15
	ctx.r[4].s64 = 21;
	// 830A968C: 4BFFFFE4  b 0x830a9670
	pc = 0x830A9670; continue 'dispatch;
            }
            0x830A9690 => {
    //   block [0x830A9690..0x830A9698)
	// 830A9690: 38800016  li r4, 0x16
	ctx.r[4].s64 = 22;
	// 830A9694: 4BFFFFDC  b 0x830a9670
	pc = 0x830A9670; continue 'dispatch;
            }
            0x830A9698 => {
    //   block [0x830A9698..0x830A96A0)
	// 830A9698: 38800017  li r4, 0x17
	ctx.r[4].s64 = 23;
	// 830A969C: 4BFFFFD4  b 0x830a9670
	pc = 0x830A9670; continue 'dispatch;
            }
            0x830A96A0 => {
    //   block [0x830A96A0..0x830A96B8)
	// 830A96A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A96A4: 4BFFDC85  bl 0x830a7328
	ctx.lr = 0x830A96A8;
	sub_830A7328(ctx, base);
	// 830A96A8: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 830A96AC: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A96B0: 4BFF9BF1  bl 0x830a32a0
	ctx.lr = 0x830A96B4;
	sub_830A32A0(ctx, base);
	// 830A96B4: 480003C0  b 0x830a9a74
	pc = 0x830A9A74; continue 'dispatch;
            }
            0x830A96B8 => {
    //   block [0x830A96B8..0x830A973C)
	// 830A96B8: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A96BC: 2F0B003C  cmpwi cr6, r11, 0x3c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 60, &mut ctx.xer);
	// 830A96C0: 419A0070  beq cr6, 0x830a9730
	if ctx.cr[6].eq {
	pc = 0x830A9730; continue 'dispatch;
	}
	// 830A96C4: 2F0B003E  cmpwi cr6, r11, 0x3e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 62, &mut ctx.xer);
	// 830A96C8: 419A005C  beq cr6, 0x830a9724
	if ctx.cr[6].eq {
	pc = 0x830A9724; continue 'dispatch;
	}
	// 830A96CC: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 830A96D0: 419A0048  beq cr6, 0x830a9718
	if ctx.cr[6].eq {
	pc = 0x830A9718; continue 'dispatch;
	}
	// 830A96D4: 2F0B0042  cmpwi cr6, r11, 0x42
	ctx.cr[6].compare_i32(ctx.r[11].s32, 66, &mut ctx.xer);
	// 830A96D8: 419A0028  beq cr6, 0x830a9700
	if ctx.cr[6].eq {
	pc = 0x830A9700; continue 'dispatch;
	}
	// 830A96DC: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 830A96E0: 419A002C  beq cr6, 0x830a970c
	if ctx.cr[6].eq {
	pc = 0x830A970C; continue 'dispatch;
	}
	// 830A96E4: 2F0B0062  cmpwi cr6, r11, 0x62
	ctx.cr[6].compare_i32(ctx.r[11].s32, 98, &mut ctx.xer);
	// 830A96E8: 419A0018  beq cr6, 0x830a9700
	if ctx.cr[6].eq {
	pc = 0x830A9700; continue 'dispatch;
	}
	// 830A96EC: 2F0B007A  cmpwi cr6, r11, 0x7a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 122, &mut ctx.xer);
	// 830A96F0: 409A004C  bne cr6, 0x830a973c
	if !ctx.cr[6].eq {
	pc = 0x830A973C; continue 'dispatch;
	}
	// 830A96F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A96F8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A96FC: 4BFFFF54  b 0x830a9650
	pc = 0x830A9650; continue 'dispatch;
	// 830A9700: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9704: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A9708: 4BFFFF48  b 0x830a9650
	pc = 0x830A9650; continue 'dispatch;
	// 830A970C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9710: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A9714: 4BFFFF3C  b 0x830a9650
	pc = 0x830A9650; continue 'dispatch;
	// 830A9718: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A971C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A9720: 4BFFFF30  b 0x830a9650
	pc = 0x830A9650; continue 'dispatch;
	// 830A9724: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9728: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A972C: 4BFFFF24  b 0x830a9650
	pc = 0x830A9650; continue 'dispatch;
	// 830A9730: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9734: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A9738: 4BFFFF18  b 0x830a9650
	pc = 0x830A9650; continue 'dispatch;
            }
            0x830A973C => {
    //   block [0x830A973C..0x830A9AB8)
	// 830A973C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A9740: 4BFFFB79  bl 0x830a92b8
	ctx.lr = 0x830A9744;
	sub_830A92B8(ctx, base);
	// 830A9744: A17F0020  lhz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A9748: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830A974C: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830A9750: 41820050  beq 0x830a97a0
	if ctx.cr[0].eq {
	pc = 0x830A97A0; continue 'dispatch;
	}
	// 830A9754: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830A9758: 419A002C  beq cr6, 0x830a9784
	if ctx.cr[6].eq {
	pc = 0x830A9784; continue 'dispatch;
	}
	// 830A975C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 830A9760: 419A0018  beq cr6, 0x830a9778
	if ctx.cr[6].eq {
	pc = 0x830A9778; continue 'dispatch;
	}
	// 830A9764: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 830A9768: 409A0308  bne cr6, 0x830a9a70
	if !ctx.cr[6].eq {
	pc = 0x830A9A70; continue 'dispatch;
	}
	// 830A976C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9770: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 830A9774: 48000018  b 0x830a978c
	pc = 0x830A978C; continue 'dispatch;
	// 830A9778: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A977C: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 830A9780: 4800000C  b 0x830a978c
	pc = 0x830A978C; continue 'dispatch;
	// 830A9784: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9788: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A978C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830A9790: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A9794: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A9798: 4E800421  bctrl
	ctx.lr = 0x830A979C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A979C: 480002D8  b 0x830a9a74
	pc = 0x830A9A74; continue 'dispatch;
	// 830A97A0: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A97A4: 2F0B007B  cmpwi cr6, r11, 0x7b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 123, &mut ctx.xer);
	// 830A97A8: 409A02C8  bne cr6, 0x830a9a70
	if !ctx.cr[6].eq {
	pc = 0x830A9A70; continue 'dispatch;
	}
	// 830A97AC: 811F001C  lwz r8, 0x1c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A97B0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A97B4: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 830A97B8: 409802B8  bge cr6, 0x830a9a70
	if !ctx.cr[6].lt {
	pc = 0x830A9A70; continue 'dispatch;
	}
	// 830A97BC: 80FF0028  lwz r7, 0x28(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A97C0: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830A97C4: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 830A97C8: 7D693A2E  lhzx r11, r9, r7
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 830A97CC: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 830A97D0: 392BFFD0  addi r9, r11, -0x30
	ctx.r[9].s64 = ctx.r[11].s64 + -48;
	// 830A97D4: 2B090009  cmplwi cr6, r9, 9
	ctx.cr[6].compare_u32(ctx.r[9].u32, 9 as u32, &mut ctx.xer);
	// 830A97D8: 419902A4  bgt cr6, 0x830a9a7c
	if ctx.cr[6].gt {
	pc = 0x830A9A7C; continue 'dispatch;
	}
	// 830A97DC: 7D3D4B78  mr r29, r9
	ctx.r[29].u64 = ctx.r[9].u64;
	// 830A97E0: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 830A97E4: 40980040  bge cr6, 0x830a9824
	if !ctx.cr[6].lt {
	pc = 0x830A9824; continue 'dispatch;
	}
	// 830A97E8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A97EC: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A97F0: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 830A97F4: 7D6A3A2E  lhzx r11, r10, r7
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 830A97F8: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 830A97FC: 394BFFD0  addi r10, r11, -0x30
	ctx.r[10].s64 = ctx.r[11].s64 + -48;
	// 830A9800: 2B0A0009  cmplwi cr6, r10, 9
	ctx.cr[6].compare_u32(ctx.r[10].u32, 9 as u32, &mut ctx.xer);
	// 830A9804: 41990020  bgt cr6, 0x830a9824
	if ctx.cr[6].gt {
	pc = 0x830A9824; continue 'dispatch;
	}
	// 830A9808: 1D5D000A  mulli r10, r29, 0xa
	ctx.r[10].s64 = ctx.r[29].s64 * 10;
	// 830A980C: 80DF001C  lwz r6, 0x1c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A9810: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 830A9814: 5529003E  slwi r9, r9, 0
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830A9818: 3BAAFFD0  addi r29, r10, -0x30
	ctx.r[29].s64 = ctx.r[10].s64 + -48;
	// 830A981C: 7F093000  cmpw cr6, r9, r6
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[6].s32, &mut ctx.xer);
	// 830A9820: 4198FFC8  blt cr6, 0x830a97e8
	if ctx.cr[6].lt {
	pc = 0x830A97E8; continue 'dispatch;
	}
	// 830A9824: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830A9828: 40980040  bge cr6, 0x830a9868
	if !ctx.cr[6].lt {
	pc = 0x830A9868; continue 'dispatch;
	}
	// 830A982C: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A9830: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A9834: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A9838: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A983C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830A9840: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830A9844: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 830A9848: 38A00382  li r5, 0x382
	ctx.r[5].s64 = 898;
	// 830A984C: 38C0009D  li r6, 0x9d
	ctx.r[6].s64 = 157;
	// 830A9850: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 830A9854: 4BFDE9B5  bl 0x83088208
	ctx.lr = 0x830A9858;
	sub_83088208(ctx, base);
	// 830A9858: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A985C: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 830A9860: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A9864: 481073C5  bl 0x831b0c28
	ctx.lr = 0x830A9868;
	sub_831B0C28(ctx, base);
	// 830A9868: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 830A986C: 2F0B002C  cmpwi cr6, r11, 0x2c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 44, &mut ctx.xer);
	// 830A9870: 409A013C  bne cr6, 0x830a99ac
	if !ctx.cr[6].eq {
	pc = 0x830A99AC; continue 'dispatch;
	}
	// 830A9874: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A9878: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 830A987C: 41980040  blt cr6, 0x830a98bc
	if ctx.cr[6].lt {
	pc = 0x830A98BC; continue 'dispatch;
	}
	// 830A9880: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A9884: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A9888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A988C: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A9890: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830A9894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830A9898: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 830A989C: 38A0038D  li r5, 0x38d
	ctx.r[5].s64 = 909;
	// 830A98A0: 38C0009B  li r6, 0x9b
	ctx.r[6].s64 = 155;
	// 830A98A4: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 830A98A8: 4BFDE961  bl 0x83088208
	ctx.lr = 0x830A98AC;
	sub_83088208(ctx, base);
	// 830A98AC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A98B0: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 830A98B4: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A98B8: 48107371  bl 0x831b0c28
	ctx.lr = 0x830A98BC;
	sub_831B0C28(ctx, base);
	// 830A98BC: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830A98C0: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 830A98C4: 7D693A2E  lhzx r11, r9, r7
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 830A98C8: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 830A98CC: 392BFFD0  addi r9, r11, -0x30
	ctx.r[9].s64 = ctx.r[11].s64 + -48;
	// 830A98D0: 2B090009  cmplwi cr6, r9, 9
	ctx.cr[6].compare_u32(ctx.r[9].u32, 9 as u32, &mut ctx.xer);
	// 830A98D4: 419900D4  bgt cr6, 0x830a99a8
	if ctx.cr[6].gt {
	pc = 0x830A99A8; continue 'dispatch;
	}
	// 830A98D8: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 830A98DC: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 830A98E0: 40980040  bge cr6, 0x830a9920
	if !ctx.cr[6].lt {
	pc = 0x830A9920; continue 'dispatch;
	}
	// 830A98E4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A98E8: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A98EC: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 830A98F0: 7D6A3A2E  lhzx r11, r10, r7
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 830A98F4: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 830A98F8: 394BFFD0  addi r10, r11, -0x30
	ctx.r[10].s64 = ctx.r[11].s64 + -48;
	// 830A98FC: 2B0A0009  cmplwi cr6, r10, 9
	ctx.cr[6].compare_u32(ctx.r[10].u32, 9 as u32, &mut ctx.xer);
	// 830A9900: 41990020  bgt cr6, 0x830a9920
	if ctx.cr[6].gt {
	pc = 0x830A9920; continue 'dispatch;
	}
	// 830A9904: 1D5E000A  mulli r10, r30, 0xa
	ctx.r[10].s64 = ctx.r[30].s64 * 10;
	// 830A9908: 811F001C  lwz r8, 0x1c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A990C: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 830A9910: 5529003E  slwi r9, r9, 0
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830A9914: 3BCAFFD0  addi r30, r10, -0x30
	ctx.r[30].s64 = ctx.r[10].s64 + -48;
	// 830A9918: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 830A991C: 4198FFC8  blt cr6, 0x830a98e4
	if ctx.cr[6].lt {
	pc = 0x830A98E4; continue 'dispatch;
	}
	// 830A9920: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830A9924: 40980040  bge cr6, 0x830a9964
	if !ctx.cr[6].lt {
	pc = 0x830A9964; continue 'dispatch;
	}
	// 830A9928: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A992C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A9930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A9934: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A9938: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830A993C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830A9940: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 830A9944: 38A0039A  li r5, 0x39a
	ctx.r[5].s64 = 922;
	// 830A9948: 38C0009D  li r6, 0x9d
	ctx.r[6].s64 = 157;
	// 830A994C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830A9950: 4BFDE8B9  bl 0x83088208
	ctx.lr = 0x830A9954;
	sub_83088208(ctx, base);
	// 830A9954: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A9958: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830A995C: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A9960: 481072C9  bl 0x831b0c28
	ctx.lr = 0x830A9964;
	sub_831B0C28(ctx, base);
	// 830A9964: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 830A9968: 40990044  ble cr6, 0x830a99ac
	if !ctx.cr[6].gt {
	pc = 0x830A99AC; continue 'dispatch;
	}
	// 830A996C: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A9970: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A9974: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A9978: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A997C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830A9980: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830A9984: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 830A9988: 38A0039C  li r5, 0x39c
	ctx.r[5].s64 = 924;
	// 830A998C: 38C0009C  li r6, 0x9c
	ctx.r[6].s64 = 156;
	// 830A9990: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 830A9994: 4BFDE875  bl 0x83088208
	ctx.lr = 0x830A9998;
	sub_83088208(ctx, base);
	// 830A9998: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A999C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 830A99A0: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A99A4: 48107285  bl 0x831b0c28
	ctx.lr = 0x830A99A8;
	sub_831B0C28(ctx, base);
	// 830A99A8: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 830A99AC: 2F0B007D  cmpwi cr6, r11, 0x7d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 125, &mut ctx.xer);
	// 830A99B0: 419A0040  beq cr6, 0x830a99f0
	if ctx.cr[6].eq {
	pc = 0x830A99F0; continue 'dispatch;
	}
	// 830A99B4: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A99B8: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A99BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A99C0: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A99C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830A99C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830A99CC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 830A99D0: 38A003A4  li r5, 0x3a4
	ctx.r[5].s64 = 932;
	// 830A99D4: 38C0009A  li r6, 0x9a
	ctx.r[6].s64 = 154;
	// 830A99D8: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 830A99DC: 4BFDE82D  bl 0x83088208
	ctx.lr = 0x830A99E0;
	sub_83088208(ctx, base);
	// 830A99E0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A99E4: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 830A99E8: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A99EC: 4810723D  bl 0x831b0c28
	ctx.lr = 0x830A99F0;
	sub_831B0C28(ctx, base);
	// 830A99F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A99F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A99F8: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A99FC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A9A00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A9A04: 4E800421  bctrl
	ctx.lr = 0x830A9A08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A9A08: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830A9A0C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A9A10: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A9A14: 4182001C  beq 0x830a9a30
	if ctx.cr[0].eq {
	pc = 0x830A9A30; continue 'dispatch;
	}
	// 830A9A18: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830A9A1C: 4BFF9A95  bl 0x830a34b0
	ctx.lr = 0x830A9A20;
	sub_830A34B0(ctx, base);
	// 830A9A20: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A9A24: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830A9A28: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830A9A2C: 4800000C  b 0x830a9a38
	pc = 0x830A9A38; continue 'dispatch;
	// 830A9A30: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A9A34: 4BFF9A7D  bl 0x830a34b0
	ctx.lr = 0x830A9A38;
	sub_830A34B0(ctx, base);
	// 830A9A38: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830A9A3C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A9A40: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9A44: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A9A48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A9A4C: 4E800421  bctrl
	ctx.lr = 0x830A9A50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A9A50: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9A54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A9A58: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830A9A5C: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A9A60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A9A64: 4E800421  bctrl
	ctx.lr = 0x830A9A68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A9A68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A9A6C: 4BFFD8BD  bl 0x830a7328
	ctx.lr = 0x830A9A70;
	sub_830A7328(ctx, base);
	// 830A9A70: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830A9A74: 38210140  addi r1, r1, 0x140
	ctx.r[1].s64 = ctx.r[1].s64 + 320;
	// 830A9A78: 480FE740  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 830A9A7C: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A9A80: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A9A84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A9A88: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A9A8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830A9A90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830A9A94: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 830A9A98: 38C00099  li r6, 0x99
	ctx.r[6].s64 = 153;
	// 830A9A9C: 38A00385  li r5, 0x385
	ctx.r[5].s64 = 901;
	// 830A9AA0: 38610100  addi r3, r1, 0x100
	ctx.r[3].s64 = ctx.r[1].s64 + 256;
	// 830A9AA4: 4BFDE765  bl 0x83088208
	ctx.lr = 0x830A9AA8;
	sub_83088208(ctx, base);
	// 830A9AA8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A9AAC: 38610100  addi r3, r1, 0x100
	ctx.r[3].s64 = ctx.r[1].s64 + 256;
	// 830A9AB0: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A9AB4: 48107175  bl 0x831b0c28
	ctx.lr = 0x830A9AB8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A9AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A9AB8 size=232
    let mut pc: u32 = 0x830A9AB8;
    'dispatch: loop {
        match pc {
            0x830A9AB8 => {
    //   block [0x830A9AB8..0x830A9BA0)
	// 830A9AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A9ABC: 480FE6A5  bl 0x831a8160
	ctx.lr = 0x830A9AC0;
	sub_831A8130(ctx, base);
	// 830A9AC0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A9AC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A9AC8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 830A9ACC: A17F0020  lhz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A9AD0: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 830A9AD4: 419A00B8  beq cr6, 0x830a9b8c
	if ctx.cr[6].eq {
	pc = 0x830A9B8C; continue 'dispatch;
	}
	// 830A9AD8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 830A9ADC: 419A00B0  beq cr6, 0x830a9b8c
	if ctx.cr[6].eq {
	pc = 0x830A9B8C; continue 'dispatch;
	}
	// 830A9AE0: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 830A9AE4: 409A000C  bne cr6, 0x830a9af0
	if !ctx.cr[6].eq {
	pc = 0x830A9AF0; continue 'dispatch;
	}
	// 830A9AE8: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A9AEC: 408200A0  bne 0x830a9b8c
	if !ctx.cr[0].eq {
	pc = 0x830A9B8C; continue 'dispatch;
	}
	// 830A9AF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A9AF4: 4BFFFB0D  bl 0x830a9600
	ctx.lr = 0x830A9AF8;
	sub_830A9600(ctx, base);
	// 830A9AF8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830A9AFC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830A9B00: 48000078  b 0x830a9b78
	pc = 0x830A9B78; continue 'dispatch;
	// 830A9B04: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 830A9B08: 419A007C  beq cr6, 0x830a9b84
	if ctx.cr[6].eq {
	pc = 0x830A9B84; continue 'dispatch;
	}
	// 830A9B0C: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 830A9B10: 409A000C  bne cr6, 0x830a9b1c
	if !ctx.cr[6].eq {
	pc = 0x830A9B1C; continue 'dispatch;
	}
	// 830A9B14: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A9B18: 4082006C  bne 0x830a9b84
	if !ctx.cr[0].eq {
	pc = 0x830A9B84; continue 'dispatch;
	}
	// 830A9B1C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830A9B20: 409A0030  bne cr6, 0x830a9b50
	if !ctx.cr[6].eq {
	pc = 0x830A9B50; continue 'dispatch;
	}
	// 830A9B24: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830A9B28: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A9B2C: 4BFF9B3D  bl 0x830a3668
	ctx.lr = 0x830A9B30;
	sub_830A3668(ctx, base);
	// 830A9B30: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A9B34: 80BF0030  lwz r5, 0x30(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A9B38: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830A9B3C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9B40: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 830A9B44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A9B48: 4E800421  bctrl
	ctx.lr = 0x830A9B4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A9B4C: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 830A9B50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A9B54: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9B58: 835F0030  lwz r26, 0x30(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A9B5C: 4BFFFAA5  bl 0x830a9600
	ctx.lr = 0x830A9B60;
	sub_830A9600(ctx, base);
	// 830A9B60: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A9B64: 817D0044  lwz r11, 0x44(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(68 as u32) ) } as u64;
	// 830A9B68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A9B6C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 830A9B70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A9B74: 4E800421  bctrl
	ctx.lr = 0x830A9B78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A9B78: A17F0020  lhz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A9B7C: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 830A9B80: 409AFF84  bne cr6, 0x830a9b04
	if !ctx.cr[6].eq {
	pc = 0x830A9B04; continue 'dispatch;
	}
	// 830A9B84: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830A9B88: 48000010  b 0x830a9b98
	pc = 0x830A9B98; continue 'dispatch;
	// 830A9B8C: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 830A9B90: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A9B94: 4BFF970D  bl 0x830a32a0
	ctx.lr = 0x830A9B98;
	sub_830A32A0(ctx, base);
	// 830A9B98: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830A9B9C: 480FE614  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A9BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A9BA0 size=164
    let mut pc: u32 = 0x830A9BA0;
    'dispatch: loop {
        match pc {
            0x830A9BA0 => {
    //   block [0x830A9BA0..0x830A9C44)
	// 830A9BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A9BA4: 480FE5BD  bl 0x831a8160
	ctx.lr = 0x830A9BA8;
	sub_831A8130(ctx, base);
	// 830A9BA8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A9BAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A9BB0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 830A9BB4: 4BFFFF05  bl 0x830a9ab8
	ctx.lr = 0x830A9BB8;
	sub_830A9AB8(ctx, base);
	// 830A9BB8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A9BBC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A9BC0: 4800006C  b 0x830a9c2c
	pc = 0x830A9C2C; continue 'dispatch;
	// 830A9BC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A9BC8: 4BFFD761  bl 0x830a7328
	ctx.lr = 0x830A9BCC;
	sub_830A7328(ctx, base);
	// 830A9BCC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830A9BD0: 409A0030  bne cr6, 0x830a9c00
	if !ctx.cr[6].eq {
	pc = 0x830A9C00; continue 'dispatch;
	}
	// 830A9BD4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A9BD8: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A9BDC: 4BFF9A8D  bl 0x830a3668
	ctx.lr = 0x830A9BE0;
	sub_830A3668(ctx, base);
	// 830A9BE0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A9BE4: 80BF0030  lwz r5, 0x30(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A9BE8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A9BEC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9BF0: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 830A9BF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A9BF8: 4E800421  bctrl
	ctx.lr = 0x830A9BFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A9BFC: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 830A9C00: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830A9C04: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9C08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A9C0C: 835F0030  lwz r26, 0x30(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A9C10: 4BFFFEA9  bl 0x830a9ab8
	ctx.lr = 0x830A9C14;
	sub_830A9AB8(ctx, base);
	// 830A9C14: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A9C18: 817C0044  lwz r11, 0x44(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(68 as u32) ) } as u64;
	// 830A9C1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A9C20: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 830A9C24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A9C28: 4E800421  bctrl
	ctx.lr = 0x830A9C2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A9C2C: A17F0020  lhz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A9C30: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 830A9C34: 419AFF90  beq cr6, 0x830a9bc4
	if ctx.cr[6].eq {
	pc = 0x830A9BC4; continue 'dispatch;
	}
	// 830A9C38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A9C3C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830A9C40: 480FE570  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A9C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A9C48 size=156
    let mut pc: u32 = 0x830A9C48;
    'dispatch: loop {
        match pc {
            0x830A9C48 => {
    //   block [0x830A9C48..0x830A9CE4)
	// 830A9C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A9C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A9C50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A9C54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A9C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A9C5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A9C60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A9C64: 4BFFD6C5  bl 0x830a7328
	ctx.lr = 0x830A9C68;
	sub_830A7328(ctx, base);
	// 830A9C68: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A9C6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A9C70: 4BFFFF31  bl 0x830a9ba0
	ctx.lr = 0x830A9C74;
	sub_830A9BA0(ctx, base);
	// 830A9C74: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830A9C78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A9C7C: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A9C80: 4BFF96E1  bl 0x830a3360
	ctx.lr = 0x830A9C84;
	sub_830A3360(ctx, base);
	// 830A9C84: A17F0020  lhz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A9C88: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A9C8C: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 830A9C90: 419A0030  beq cr6, 0x830a9cc0
	if ctx.cr[6].eq {
	pc = 0x830A9CC0; continue 'dispatch;
	}
	// 830A9C94: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A9C98: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A9C9C: 38C0007E  li r6, 0x7e
	ctx.r[6].s64 = 126;
	// 830A9CA0: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A9CA4: 38A001DF  li r5, 0x1df
	ctx.r[5].s64 = 479;
	// 830A9CA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A9CAC: 4BFFD4DD  bl 0x830a7188
	ctx.lr = 0x830A9CB0;
	sub_830A7188(ctx, base);
	// 830A9CB0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A9CB4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A9CB8: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A9CBC: 48106F6D  bl 0x831b0c28
	ctx.lr = 0x830A9CC0;
	sub_831B0C28(ctx, base);
	// 830A9CC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A9CC4: 4BFFD665  bl 0x830a7328
	ctx.lr = 0x830A9CC8;
	sub_830A7328(ctx, base);
	// 830A9CC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A9CCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830A9CD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A9CD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A9CD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A9CDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A9CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A9CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A9CE8 size=164
    let mut pc: u32 = 0x830A9CE8;
    'dispatch: loop {
        match pc {
            0x830A9CE8 => {
    //   block [0x830A9CE8..0x830A9D8C)
	// 830A9CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A9CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A9CF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A9CF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A9CF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A9CFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A9D00: 4BFFD629  bl 0x830a7328
	ctx.lr = 0x830A9D04;
	sub_830A7328(ctx, base);
	// 830A9D04: 83DF0014  lwz r30, 0x14(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A9D08: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830A9D0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A9D10: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 830A9D14: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830A9D18: 4BFFFE89  bl 0x830a9ba0
	ctx.lr = 0x830A9D1C;
	sub_830A9BA0(ctx, base);
	// 830A9D1C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A9D20: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A9D24: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830A9D28: 4BFF96E1  bl 0x830a3408
	ctx.lr = 0x830A9D2C;
	sub_830A3408(ctx, base);
	// 830A9D2C: A17F0020  lhz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A9D30: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A9D34: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 830A9D38: 419A0030  beq cr6, 0x830a9d68
	if ctx.cr[6].eq {
	pc = 0x830A9D68; continue 'dispatch;
	}
	// 830A9D3C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A9D40: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A9D44: 38C0007E  li r6, 0x7e
	ctx.r[6].s64 = 126;
	// 830A9D48: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A9D4C: 38A00254  li r5, 0x254
	ctx.r[5].s64 = 596;
	// 830A9D50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A9D54: 4BFFD435  bl 0x830a7188
	ctx.lr = 0x830A9D58;
	sub_830A7188(ctx, base);
	// 830A9D58: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A9D5C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A9D60: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A9D64: 48106EC5  bl 0x831b0c28
	ctx.lr = 0x830A9D68;
	sub_831B0C28(ctx, base);
	// 830A9D68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A9D6C: 4BFFD5BD  bl 0x830a7328
	ctx.lr = 0x830A9D70;
	sub_830A7328(ctx, base);
	// 830A9D70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A9D74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830A9D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A9D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A9D80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A9D84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A9D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A9D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A9D90 size=152
    let mut pc: u32 = 0x830A9D90;
    'dispatch: loop {
        match pc {
            0x830A9D90 => {
    //   block [0x830A9D90..0x830A9E28)
	// 830A9D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A9D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A9D98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A9D9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A9DA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A9DA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A9DA8: 4BFFD581  bl 0x830a7328
	ctx.lr = 0x830A9DAC;
	sub_830A7328(ctx, base);
	// 830A9DAC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A9DB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A9DB4: 4BFFFDED  bl 0x830a9ba0
	ctx.lr = 0x830A9DB8;
	sub_830A9BA0(ctx, base);
	// 830A9DB8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A9DBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A9DC0: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A9DC4: 4BFF9645  bl 0x830a3408
	ctx.lr = 0x830A9DC8;
	sub_830A3408(ctx, base);
	// 830A9DC8: A17F0020  lhz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A9DCC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A9DD0: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 830A9DD4: 419A0030  beq cr6, 0x830a9e04
	if ctx.cr[6].eq {
	pc = 0x830A9E04; continue 'dispatch;
	}
	// 830A9DD8: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A9DDC: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A9DE0: 38C0007E  li r6, 0x7e
	ctx.r[6].s64 = 126;
	// 830A9DE4: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A9DE8: 38A00261  li r5, 0x261
	ctx.r[5].s64 = 609;
	// 830A9DEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A9DF0: 4BFFD399  bl 0x830a7188
	ctx.lr = 0x830A9DF4;
	sub_830A7188(ctx, base);
	// 830A9DF4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A9DF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A9DFC: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A9E00: 48106E29  bl 0x831b0c28
	ctx.lr = 0x830A9E04;
	sub_831B0C28(ctx, base);
	// 830A9E04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A9E08: 4BFFD521  bl 0x830a7328
	ctx.lr = 0x830A9E0C;
	sub_830A7328(ctx, base);
	// 830A9E0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A9E10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830A9E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A9E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A9E1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A9E20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A9E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A9E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A9E28 size=8
    let mut pc: u32 = 0x830A9E28;
    'dispatch: loop {
        match pc {
            0x830A9E28 => {
    //   block [0x830A9E28..0x830A9E30)
	// 830A9E28: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A9E2C: 821802A8  lwz r16, 0x2a8(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(680 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A9E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A9E30 size=776
    let mut pc: u32 = 0x830A9E30;
    'dispatch: loop {
        match pc {
            0x830A9E30 => {
    //   block [0x830A9E30..0x830AA138)
	// 830A9E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A9E34: 480FE32D  bl 0x831a8160
	ctx.lr = 0x830A9E38;
	sub_831A8130(ctx, base);
	// 830A9E38: 3BE1FF30  addi r31, r1, -0xd0
	ctx.r[31].s64 = ctx.r[1].s64 + -208;
	// 830A9E3C: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A9E40: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A9E44: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A9E48: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A9E4C: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 830A9E50: 93DF00E4  stw r30, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[30].u32 ) };
	// 830A9E54: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830A9E58: 41980030  blt cr6, 0x830a9e88
	if ctx.cr[6].lt {
	pc = 0x830A9E88; continue 'dispatch;
	}
	// 830A9E5C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A9E60: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A9E64: 38C00081  li r6, 0x81
	ctx.r[6].s64 = 129;
	// 830A9E68: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A9E6C: 38A0026B  li r5, 0x26b
	ctx.r[5].s64 = 619;
	// 830A9E70: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A9E74: 4BFFD315  bl 0x830a7188
	ctx.lr = 0x830A9E78;
	sub_830A7188(ctx, base);
	// 830A9E78: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A9E7C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A9E80: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A9E84: 48106DA5  bl 0x831b0c28
	ctx.lr = 0x830A9E88;
	sub_831B0C28(ctx, base);
	// 830A9E88: 813E0028  lwz r9, 0x28(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A9E8C: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A9E90: 3B60FFFF  li r27, -1
	ctx.r[27].s64 = -1;
	// 830A9E94: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 830A9E98: 7D6B4A2E  lhzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 830A9E9C: 392BFFCF  addi r9, r11, -0x31
	ctx.r[9].s64 = ctx.r[11].s64 + -49;
	// 830A9EA0: 2B090008  cmplwi cr6, r9, 8
	ctx.cr[6].compare_u32(ctx.r[9].u32, 8 as u32, &mut ctx.xer);
	// 830A9EA4: 41990158  bgt cr6, 0x830a9ffc
	if ctx.cr[6].gt {
	pc = 0x830A9FFC; continue 'dispatch;
	}
	// 830A9EA8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830A9EAC: 3B6BFFD0  addi r27, r11, -0x30
	ctx.r[27].s64 = ctx.r[11].s64 + -48;
	// 830A9EB0: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A9EB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A9EB8: 995E0008  stb r10, 8(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 830A9EBC: 409A004C  bne cr6, 0x830a9f08
	if !ctx.cr[6].eq {
	pc = 0x830A9F08; continue 'dispatch;
	}
	// 830A9EC0: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830A9EC4: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A9EC8: 4BF2E3D1  bl 0x82fd8298
	ctx.lr = 0x830A9ECC;
	sub_82FD8298(ctx, base);
	// 830A9ECC: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830A9ED0: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 830A9ED4: 4182002C  beq 0x830a9f00
	if ctx.cr[0].eq {
	pc = 0x830A9F00; continue 'dispatch;
	}
	// 830A9ED8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830A9EDC: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A9EE0: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 830A9EE4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A9EE8: 4BFFE9C9  bl 0x830a88b0
	ctx.lr = 0x830A9EEC;
	sub_830A88B0(ctx, base);
	// 830A9EEC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A9EF0: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 830A9EF4: 396B0114  addi r11, r11, 0x114
	ctx.r[11].s64 = ctx.r[11].s64 + 276;
	// 830A9EF8: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A9EFC: 48000008  b 0x830a9f04
	pc = 0x830A9F04; continue 'dispatch;
	// 830A9F00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A9F04: 915E002C  stw r10, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 830A9F08: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 830A9F0C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A9F10: 4BF2E389  bl 0x82fd8298
	ctx.lr = 0x830A9F14;
	sub_82FD8298(ctx, base);
	// 830A9F14: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A9F18: 41820018  beq 0x830a9f30
	if ctx.cr[0].eq {
	pc = 0x830A9F30; continue 'dispatch;
	}
	// 830A9F1C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A9F20: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A9F24: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 830A9F28: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830A9F2C: 48000008  b 0x830a9f34
	pc = 0x830A9F34; continue 'dispatch;
	// 830A9F30: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A9F34: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A9F38: 4BF91219  bl 0x8303b150
	ctx.lr = 0x830A9F3C;
	sub_8303B150(ctx, base);
	// 830A9F3C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A9F40: 815E0028  lwz r10, 0x28(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A9F44: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830A9F48: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830A9F4C: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830A9F50: 7D49522E  lhzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830A9F54: 2B0A0029  cmplwi cr6, r10, 0x29
	ctx.cr[6].compare_u32(ctx.r[10].u32, 41 as u32, &mut ctx.xer);
	// 830A9F58: 419A0030  beq cr6, 0x830a9f88
	if ctx.cr[6].eq {
	pc = 0x830A9F88; continue 'dispatch;
	}
	// 830A9F5C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A9F60: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A9F64: 38C0007E  li r6, 0x7e
	ctx.r[6].s64 = 126;
	// 830A9F68: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A9F6C: 38A0027E  li r5, 0x27e
	ctx.r[5].s64 = 638;
	// 830A9F70: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A9F74: 4BFFD215  bl 0x830a7188
	ctx.lr = 0x830A9F78;
	sub_830A7188(ctx, base);
	// 830A9F78: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A9F7C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A9F80: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A9F84: 48106CA5  bl 0x831b0c28
	ctx.lr = 0x830A9F88;
	sub_831B0C28(ctx, base);
	// 830A9F88: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830A9F8C: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830A9F90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A9F94: 4BFFD395  bl 0x830a7328
	ctx.lr = 0x830A9F98;
	sub_830A7328(ctx, base);
	// 830A9F98: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A9F9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A9FA0: 4BFFFC01  bl 0x830a9ba0
	ctx.lr = 0x830A9FA4;
	sub_830A9BA0(ctx, base);
	// 830A9FA4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A9FA8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830A9FAC: A17D0004  lhz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A9FB0: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 830A9FB4: 409A0124  bne cr6, 0x830aa0d8
	if !ctx.cr[6].eq {
	pc = 0x830AA0D8; continue 'dispatch;
	}
	// 830A9FB8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A9FBC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A9FC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A9FC4: 4E800421  bctrl
	ctx.lr = 0x830A9FC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A9FC8: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 830A9FCC: 419A00D4  beq cr6, 0x830aa0a0
	if ctx.cr[6].eq {
	pc = 0x830AA0A0; continue 'dispatch;
	}
	// 830A9FD0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830A9FD4: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A9FD8: 38C00083  li r6, 0x83
	ctx.r[6].s64 = 131;
	// 830A9FDC: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830A9FE0: 38A002A0  li r5, 0x2a0
	ctx.r[5].s64 = 672;
	// 830A9FE4: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 830A9FE8: 4BFFD1A1  bl 0x830a7188
	ctx.lr = 0x830A9FEC;
	sub_830A7188(ctx, base);
	// 830A9FEC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A9FF0: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 830A9FF4: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830A9FF8: 48106C31  bl 0x831b0c28
	ctx.lr = 0x830A9FFC;
	sub_831B0C28(ctx, base);
	// 830A9FFC: 2F0B003F  cmpwi cr6, r11, 0x3f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 63, &mut ctx.xer);
	// 830AA000: 409A000C  bne cr6, 0x830aa00c
	if !ctx.cr[6].eq {
	pc = 0x830AA00C; continue 'dispatch;
	}
	// 830AA004: 396AFFFF  addi r11, r10, -1
	ctx.r[11].s64 = ctx.r[10].s64 + -1;
	// 830AA008: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830AA00C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AA010: 4BFFD319  bl 0x830a7328
	ctx.lr = 0x830AA014;
	sub_830A7328(ctx, base);
	// 830AA014: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AA018: 4BFFF5E9  bl 0x830a9600
	ctx.lr = 0x830AA01C;
	sub_830A9600(ctx, base);
	// 830AA01C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 830AA020: A17A0004  lhz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AA024: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 830AA028: 419A0040  beq cr6, 0x830aa068
	if ctx.cr[6].eq {
	pc = 0x830AA068; continue 'dispatch;
	}
	// 830AA02C: 2F0B0013  cmpwi cr6, r11, 0x13
	ctx.cr[6].compare_i32(ctx.r[11].s32, 19, &mut ctx.xer);
	// 830AA030: 4099000C  ble cr6, 0x830aa03c
	if !ctx.cr[6].gt {
	pc = 0x830AA03C; continue 'dispatch;
	}
	// 830AA034: 2F0B0017  cmpwi cr6, r11, 0x17
	ctx.cr[6].compare_i32(ctx.r[11].s32, 23, &mut ctx.xer);
	// 830AA038: 4099FF58  ble cr6, 0x830a9f90
	if !ctx.cr[6].gt {
	pc = 0x830A9F90; continue 'dispatch;
	}
	// 830AA03C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AA040: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AA044: 38C00082  li r6, 0x82
	ctx.r[6].s64 = 130;
	// 830AA048: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830AA04C: 38A00295  li r5, 0x295
	ctx.r[5].s64 = 661;
	// 830AA050: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830AA054: 4BFFD135  bl 0x830a7188
	ctx.lr = 0x830AA058;
	sub_830A7188(ctx, base);
	// 830AA058: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AA05C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830AA060: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830AA064: 48106BC5  bl 0x831b0c28
	ctx.lr = 0x830AA068;
	sub_831B0C28(ctx, base);
	// 830AA068: A17E0020  lhz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830AA06C: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 830AA070: 419AFF20  beq cr6, 0x830a9f90
	if ctx.cr[6].eq {
	pc = 0x830A9F90; continue 'dispatch;
	}
	// 830AA074: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AA078: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AA07C: 38C0007E  li r6, 0x7e
	ctx.r[6].s64 = 126;
	// 830AA080: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830AA084: 38A00292  li r5, 0x292
	ctx.r[5].s64 = 658;
	// 830AA088: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 830AA08C: 4BFFD0FD  bl 0x830a7188
	ctx.lr = 0x830AA090;
	sub_830A7188(ctx, base);
	// 830AA090: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AA094: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 830AA098: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830AA09C: 48106B8D  bl 0x831b0c28
	ctx.lr = 0x830AA0A0;
	sub_831B0C28(ctx, base);
	// 830AA0A0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AA0A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830AA0A8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830AA0AC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AA0B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AA0B4: 4E800421  bctrl
	ctx.lr = 0x830AA0B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AA0B8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AA0BC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830AA0C0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AA0C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830AA0C8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AA0CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AA0D0: 4E800421  bctrl
	ctx.lr = 0x830AA0D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AA0D4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830AA0D8: A17E0020  lhz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830AA0DC: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 830AA0E0: 419A0030  beq cr6, 0x830aa110
	if ctx.cr[6].eq {
	pc = 0x830AA110; continue 'dispatch;
	}
	// 830AA0E4: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AA0E8: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AA0EC: 38C0007E  li r6, 0x7e
	ctx.r[6].s64 = 126;
	// 830AA0F0: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830AA0F4: 38A002A7  li r5, 0x2a7
	ctx.r[5].s64 = 679;
	// 830AA0F8: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 830AA0FC: 4BFFD08D  bl 0x830a7188
	ctx.lr = 0x830AA100;
	sub_830A7188(ctx, base);
	// 830AA100: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AA104: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 830AA108: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830AA10C: 48106B1D  bl 0x831b0c28
	ctx.lr = 0x830AA110;
	sub_831B0C28(ctx, base);
	// 830AA110: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AA114: 4BFFD215  bl 0x830a7328
	ctx.lr = 0x830AA118;
	sub_830A7328(ctx, base);
	// 830AA118: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 830AA11C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 830AA120: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AA124: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 830AA128: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830AA12C: 4BFF9A5D  bl 0x830a3b88
	ctx.lr = 0x830AA130;
	sub_830A3B88(ctx, base);
	// 830AA130: 383F00D0  addi r1, r31, 0xd0
	ctx.r[1].s64 = ctx.r[31].s64 + 208;
	// 830AA134: 480FE07C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AA138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AA138 size=48
    let mut pc: u32 = 0x830AA138;
    'dispatch: loop {
        match pc {
            0x830AA138 => {
    //   block [0x830AA138..0x830AA168)
	// 830AA138: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 830AA13C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AA140: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AA144: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AA148: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 830AA14C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AA150: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830AA154: 4BF2E18D  bl 0x82fd82e0
	ctx.lr = 0x830AA158;
	sub_82FD82E0(ctx, base);
	// 830AA158: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AA15C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AA160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AA164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AA168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AA168 size=576
    let mut pc: u32 = 0x830AA168;
    'dispatch: loop {
        match pc {
            0x830AA168 => {
    //   block [0x830AA168..0x830AA3A8)
	// 830AA168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AA16C: 480FE001  bl 0x831a816c
	ctx.lr = 0x830AA170;
	sub_831A8130(ctx, base);
	// 830AA170: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AA174: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AA178: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830AA17C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830AA180: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 830AA184: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830AA188: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AA18C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830AA190: 4098004C  bge cr6, 0x830aa1dc
	if !ctx.cr[6].lt {
	pc = 0x830AA1DC; continue 'dispatch;
	}
	// 830AA194: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830AA198: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AA19C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830AA1A0: 7C6B522E  lhzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830AA1A4: 4BFDE55D  bl 0x83088700
	ctx.lr = 0x830AA1A8;
	sub_83088700(ctx, base);
	// 830AA1A8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AA1AC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830AA1B0: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830AA1B4: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830AA1B8: 7D29522E  lhzx r9, r9, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830AA1BC: 41820020  beq 0x830aa1dc
	if ctx.cr[0].eq {
	pc = 0x830AA1DC; continue 'dispatch;
	}
	// 830AA1C0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830AA1C4: 7C7DEB78  or r29, r3, r29
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[29].u64;
	// 830AA1C8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830AA1CC: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830AA1D0: 811F001C  lwz r8, 0x1c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830AA1D4: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 830AA1D8: 4198FFC0  blt cr6, 0x830aa198
	if ctx.cr[6].lt {
	pc = 0x830AA198; continue 'dispatch;
	}
	// 830AA1DC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AA1E0: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830AA1E4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 830AA1E8: 41980030  blt cr6, 0x830aa218
	if ctx.cr[6].lt {
	pc = 0x830AA218; continue 'dispatch;
	}
	// 830AA1EC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AA1F0: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AA1F4: 38C0007F  li r6, 0x7f
	ctx.r[6].s64 = 127;
	// 830AA1F8: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830AA1FC: 38A002C4  li r5, 0x2c4
	ctx.r[5].s64 = 708;
	// 830AA200: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AA204: 4BFFCF85  bl 0x830a7188
	ctx.lr = 0x830AA208;
	sub_830A7188(ctx, base);
	// 830AA208: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AA20C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AA210: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830AA214: 48106A15  bl 0x831b0c28
	ctx.lr = 0x830AA218;
	sub_831B0C28(ctx, base);
	// 830AA218: 2F09002D  cmpwi cr6, r9, 0x2d
	ctx.cr[6].compare_i32(ctx.r[9].s32, 45, &mut ctx.xer);
	// 830AA21C: 409A0098  bne cr6, 0x830aa2b4
	if !ctx.cr[6].eq {
	pc = 0x830AA2B4; continue 'dispatch;
	}
	// 830AA220: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830AA224: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 830AA228: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830AA22C: 4098004C  bge cr6, 0x830aa278
	if !ctx.cr[6].lt {
	pc = 0x830AA278; continue 'dispatch;
	}
	// 830AA230: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830AA234: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AA238: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830AA23C: 7C6B522E  lhzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830AA240: 4BFDE4C1  bl 0x83088700
	ctx.lr = 0x830AA244;
	sub_83088700(ctx, base);
	// 830AA244: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AA248: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830AA24C: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830AA250: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830AA254: 7D29522E  lhzx r9, r9, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830AA258: 41820020  beq 0x830aa278
	if ctx.cr[0].eq {
	pc = 0x830AA278; continue 'dispatch;
	}
	// 830AA25C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830AA260: 7C7EF378  or r30, r3, r30
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[30].u64;
	// 830AA264: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830AA268: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830AA26C: 811F001C  lwz r8, 0x1c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830AA270: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 830AA274: 4198FFC0  blt cr6, 0x830aa234
	if ctx.cr[6].lt {
	pc = 0x830AA234; continue 'dispatch;
	}
	// 830AA278: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AA27C: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830AA280: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 830AA284: 41980030  blt cr6, 0x830aa2b4
	if ctx.cr[6].lt {
	pc = 0x830AA2B4; continue 'dispatch;
	}
	// 830AA288: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AA28C: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AA290: 38C0007F  li r6, 0x7f
	ctx.r[6].s64 = 127;
	// 830AA294: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830AA298: 38A002D5  li r5, 0x2d5
	ctx.r[5].s64 = 725;
	// 830AA29C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AA2A0: 4BFFCEE9  bl 0x830a7188
	ctx.lr = 0x830AA2A4;
	sub_830A7188(ctx, base);
	// 830AA2A4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AA2A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AA2AC: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830AA2B0: 48106979  bl 0x831b0c28
	ctx.lr = 0x830AA2B4;
	sub_831B0C28(ctx, base);
	// 830AA2B4: 2F09003A  cmpwi cr6, r9, 0x3a
	ctx.cr[6].compare_i32(ctx.r[9].s32, 58, &mut ctx.xer);
	// 830AA2B8: 409A007C  bne cr6, 0x830aa334
	if !ctx.cr[6].eq {
	pc = 0x830AA334; continue 'dispatch;
	}
	// 830AA2BC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830AA2C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AA2C4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830AA2C8: 4BFFD061  bl 0x830a7328
	ctx.lr = 0x830AA2CC;
	sub_830A7328(ctx, base);
	// 830AA2CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AA2D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AA2D4: 4BFFF8CD  bl 0x830a9ba0
	ctx.lr = 0x830AA2D8;
	sub_830A9BA0(ctx, base);
	// 830AA2D8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830AA2DC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AA2E0: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AA2E4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 830AA2E8: 4BFF97F1  bl 0x830a3ad8
	ctx.lr = 0x830AA2EC;
	sub_830A3AD8(ctx, base);
	// 830AA2EC: A17F0020  lhz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830AA2F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830AA2F4: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 830AA2F8: 419A0030  beq cr6, 0x830aa328
	if ctx.cr[6].eq {
	pc = 0x830AA328; continue 'dispatch;
	}
	// 830AA2FC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AA300: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AA304: 38C0007E  li r6, 0x7e
	ctx.r[6].s64 = 126;
	// 830AA308: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830AA30C: 38A002E1  li r5, 0x2e1
	ctx.r[5].s64 = 737;
	// 830AA310: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AA314: 4BFFCE75  bl 0x830a7188
	ctx.lr = 0x830AA318;
	sub_830A7188(ctx, base);
	// 830AA318: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AA31C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AA320: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830AA324: 48106905  bl 0x831b0c28
	ctx.lr = 0x830AA328;
	sub_831B0C28(ctx, base);
	// 830AA328: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AA32C: 4BFFCFFD  bl 0x830a7328
	ctx.lr = 0x830AA330;
	sub_830A7328(ctx, base);
	// 830AA330: 48000040  b 0x830aa370
	pc = 0x830AA370; continue 'dispatch;
	// 830AA334: 2F090029  cmpwi cr6, r9, 0x29
	ctx.cr[6].compare_i32(ctx.r[9].s32, 41, &mut ctx.xer);
	// 830AA338: 409A0044  bne cr6, 0x830aa37c
	if !ctx.cr[6].eq {
	pc = 0x830AA37C; continue 'dispatch;
	}
	// 830AA33C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830AA340: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AA344: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830AA348: 4BFFCFE1  bl 0x830a7328
	ctx.lr = 0x830AA34C;
	sub_830A7328(ctx, base);
	// 830AA34C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AA350: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AA354: 4BFFF84D  bl 0x830a9ba0
	ctx.lr = 0x830AA358;
	sub_830A9BA0(ctx, base);
	// 830AA358: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830AA35C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AA360: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AA364: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 830AA368: 4BFF9771  bl 0x830a3ad8
	ctx.lr = 0x830AA36C;
	sub_830A3AD8(ctx, base);
	// 830AA36C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830AA370: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AA374: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830AA378: 480FDE44  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 830AA37C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AA380: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AA384: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 830AA388: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830AA38C: 38A002EC  li r5, 0x2ec
	ctx.r[5].s64 = 748;
	// 830AA390: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AA394: 4BFFCDF5  bl 0x830a7188
	ctx.lr = 0x830AA398;
	sub_830A7188(ctx, base);
	// 830AA398: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AA39C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AA3A0: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830AA3A4: 48106885  bl 0x831b0c28
	ctx.lr = 0x830AA3A8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AA3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AA3A8 size=152
    let mut pc: u32 = 0x830AA3A8;
    'dispatch: loop {
        match pc {
            0x830AA3A8 => {
    //   block [0x830AA3A8..0x830AA440)
	// 830AA3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AA3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AA3B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830AA3B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AA3B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AA3BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AA3C0: 4BFFCF69  bl 0x830a7328
	ctx.lr = 0x830AA3C4;
	sub_830A7328(ctx, base);
	// 830AA3C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AA3C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AA3CC: 4BFFF7D5  bl 0x830a9ba0
	ctx.lr = 0x830AA3D0;
	sub_830A9BA0(ctx, base);
	// 830AA3D0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AA3D4: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	// 830AA3D8: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AA3DC: 4BFF8F85  bl 0x830a3360
	ctx.lr = 0x830AA3E0;
	sub_830A3360(ctx, base);
	// 830AA3E0: A17F0020  lhz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830AA3E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830AA3E8: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 830AA3EC: 419A0030  beq cr6, 0x830aa41c
	if ctx.cr[6].eq {
	pc = 0x830AA41C; continue 'dispatch;
	}
	// 830AA3F0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AA3F4: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AA3F8: 38C0007E  li r6, 0x7e
	ctx.r[6].s64 = 126;
	// 830AA3FC: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830AA400: 38A002FA  li r5, 0x2fa
	ctx.r[5].s64 = 762;
	// 830AA404: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AA408: 4BFFCD81  bl 0x830a7188
	ctx.lr = 0x830AA40C;
	sub_830A7188(ctx, base);
	// 830AA40C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AA410: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AA414: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830AA418: 48106811  bl 0x831b0c28
	ctx.lr = 0x830AA41C;
	sub_831B0C28(ctx, base);
	// 830AA41C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AA420: 4BFFCF09  bl 0x830a7328
	ctx.lr = 0x830AA424;
	sub_830A7328(ctx, base);
	// 830AA424: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AA428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830AA42C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AA430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AA434: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830AA438: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AA43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AA440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AA440 size=8
    let mut pc: u32 = 0x830AA440;
    'dispatch: loop {
        match pc {
            0x830AA440 => {
    //   block [0x830AA440..0x830AA448)
	// 830AA440: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830AA444: 821802F0  lwz r16, 0x2f0(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(752 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AA448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AA448 size=516
    let mut pc: u32 = 0x830AA448;
    'dispatch: loop {
        match pc {
            0x830AA448 => {
    //   block [0x830AA448..0x830AA64C)
	// 830AA448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AA44C: 480FDD19  bl 0x831a8164
	ctx.lr = 0x830AA450;
	sub_831A8130(ctx, base);
	// 830AA450: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 830AA454: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AA458: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830AA45C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 830AA460: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830AA464: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AA468: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830AA46C: 409A000C  bne cr6, 0x830aa478
	if !ctx.cr[6].eq {
	pc = 0x830AA478; continue 'dispatch;
	}
	// 830AA470: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830AA474: 480001A4  b 0x830aa618
	pc = 0x830AA618; continue 'dispatch;
	// 830AA478: 389E0034  addi r4, r30, 0x34
	ctx.r[4].s64 = ctx.r[30].s64 + 52;
	// 830AA47C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830AA480: 4BF4B359  bl 0x82ff57d8
	ctx.lr = 0x830AA484;
	sub_82FF57D8(ctx, base);
	// 830AA484: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830AA488: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830AA48C: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830AA490: 939E000C  stw r28, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 830AA494: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AA498: 9BBE0008  stb r29, 8(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u8 ) };
	// 830AA49C: B3BE0018  sth r29, 0x18(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[29].u16 ) };
	// 830AA4A0: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 830AA4A4: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830AA4A8: 41820018  beq 0x830aa4c0
	if ctx.cr[0].eq {
	pc = 0x830AA4C0; continue 'dispatch;
	}
	// 830AA4AC: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AA4B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AA4B4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AA4B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AA4BC: 4E800421  bctrl
	ctx.lr = 0x830AA4C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AA4C0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830AA4C4: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AA4C8: 4BF266B9  bl 0x82fd0b80
	ctx.lr = 0x830AA4CC;
	sub_82FD0B80(ctx, base);
	// 830AA4CC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830AA4D0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830AA4D4: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830AA4D8: 909E0028  stw r4, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[4].u32 ) };
	// 830AA4DC: 816BACA4  lwz r11, -0x535c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21340 as u32) ) } as u64;
	// 830AA4E0: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 830AA4E4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 830AA4E8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830AA4EC: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AA4F0: 41820030  beq 0x830aa520
	if ctx.cr[0].eq {
	pc = 0x830AA520; continue 'dispatch;
	}
	// 830AA4F4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830AA4F8: 419A0018  beq cr6, 0x830aa510
	if ctx.cr[6].eq {
	pc = 0x830AA510; continue 'dispatch;
	}
	// 830AA4FC: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AA500: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AA504: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AA508: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AA50C: 4E800421  bctrl
	ctx.lr = 0x830AA510;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AA510: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830AA514: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AA518: 4BFFCB21  bl 0x830a7038
	ctx.lr = 0x830AA51C;
	sub_830A7038(ctx, base);
	// 830AA51C: 907E0028  stw r3, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 830AA520: 815E0028  lwz r10, 0x28(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830AA524: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AA528: 41820034  beq 0x830aa55c
	if ctx.cr[0].eq {
	pc = 0x830AA55C; continue 'dispatch;
	}
	// 830AA52C: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AA530: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AA534: 41820028  beq 0x830aa55c
	if ctx.cr[0].eq {
	pc = 0x830AA55C; continue 'dispatch;
	}
	// 830AA538: 396A0002  addi r11, r10, 2
	ctx.r[11].s64 = ctx.r[10].s64 + 2;
	// 830AA53C: 48000008  b 0x830aa544
	pc = 0x830AA544; continue 'dispatch;
	// 830AA540: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830AA544: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AA548: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AA54C: 4082FFF4  bne 0x830aa540
	if !ctx.cr[0].eq {
	pc = 0x830AA540; continue 'dispatch;
	}
	// 830AA550: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 830AA554: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830AA558: 48000008  b 0x830aa560
	pc = 0x830AA560; continue 'dispatch;
	// 830AA55C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 830AA560: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AA564: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 830AA568: 4BFFCDC1  bl 0x830a7328
	ctx.lr = 0x830AA56C;
	sub_830A7328(ctx, base);
	// 830AA56C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AA570: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AA574: 4BFFF62D  bl 0x830a9ba0
	ctx.lr = 0x830AA578;
	sub_830A9BA0(ctx, base);
	// 830AA578: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AA57C: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 830AA580: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 830AA584: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 830AA588: 419A0030  beq cr6, 0x830aa5b8
	if ctx.cr[6].eq {
	pc = 0x830AA5B8; continue 'dispatch;
	}
	// 830AA58C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AA590: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AA594: 38C00078  li r6, 0x78
	ctx.r[6].s64 = 120;
	// 830AA598: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830AA59C: 38A000BF  li r5, 0xbf
	ctx.r[5].s64 = 191;
	// 830AA5A0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830AA5A4: 4BFFCBE5  bl 0x830a7188
	ctx.lr = 0x830AA5A8;
	sub_830A7188(ctx, base);
	// 830AA5A8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AA5AC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830AA5B0: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830AA5B4: 48106675  bl 0x831b0c28
	ctx.lr = 0x830AA5B8;
	sub_831B0C28(ctx, base);
	// 830AA5B8: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AA5BC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AA5C0: 4182004C  beq 0x830aa60c
	if ctx.cr[0].eq {
	pc = 0x830AA60C; continue 'dispatch;
	}
	// 830AA5C4: 838B0008  lwz r28, 8(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AA5C8: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AA5CC: 4182002C  beq 0x830aa5f8
	if ctx.cr[0].eq {
	pc = 0x830AA5F8; continue 'dispatch;
	}
	// 830AA5D0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830AA5D4: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AA5D8: 4BF82299  bl 0x8302c870
	ctx.lr = 0x830AA5DC;
	sub_8302C870(ctx, base);
	// 830AA5DC: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830AA5E0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AA5E4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 830AA5E8: 40990038  ble cr6, 0x830aa620
	if !ctx.cr[6].gt {
	pc = 0x830AA620; continue 'dispatch;
	}
	// 830AA5EC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830AA5F0: 7F1DE040  cmplw cr6, r29, r28
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[28].u32, &mut ctx.xer);
	// 830AA5F4: 4198FFDC  blt cr6, 0x830aa5d0
	if ctx.cr[6].lt {
	pc = 0x830AA5D0; continue 'dispatch;
	}
	// 830AA5F8: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AA5FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AA600: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AA604: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AA608: 4E800421  bctrl
	ctx.lr = 0x830AA60C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AA60C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830AA610: 4BF4B201  bl 0x82ff5810
	ctx.lr = 0x830AA614;
	sub_82FF5810(ctx, base);
	// 830AA614: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830AA618: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 830AA61C: 480FDB98  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 830AA620: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AA624: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AA628: 38C00079  li r6, 0x79
	ctx.r[6].s64 = 121;
	// 830AA62C: 388B00C8  addi r4, r11, 0xc8
	ctx.r[4].s64 = ctx.r[11].s64 + 200;
	// 830AA630: 38A000C8  li r5, 0xc8
	ctx.r[5].s64 = 200;
	// 830AA634: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830AA638: 4BFFCB51  bl 0x830a7188
	ctx.lr = 0x830AA63C;
	sub_830A7188(ctx, base);
	// 830AA63C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AA640: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830AA644: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830AA648: 481065E1  bl 0x831b0c28
	ctx.lr = 0x830AA64C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AA64C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AA64C size=40
    let mut pc: u32 = 0x830AA64C;
    'dispatch: loop {
        match pc {
            0x830AA64C => {
    //   block [0x830AA64C..0x830AA674)
	// 830AA64C: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 830AA650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AA654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AA658: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AA65C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830AA660: 4BF4B1B1  bl 0x82ff5810
	ctx.lr = 0x830AA664;
	sub_82FF5810(ctx, base);
	// 830AA664: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AA668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AA66C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AA670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


