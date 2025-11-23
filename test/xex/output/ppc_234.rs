pub fn sub_832AB2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB2B8 size=104
    let mut pc: u32 = 0x832AB2B8;
    'dispatch: loop {
        match pc {
            0x832AB2B8 => {
    //   block [0x832AB2B8..0x832AB2E0)
	// 832AB2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB2BC: 4B9FE14D  bl 0x82ca9408
	ctx.lr = 0x832AB2C0;
	sub_82CA93D0(ctx, base);
	// 832AB2C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB2C4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB2C8: 3BC0001F  li r30, 0x1f
	ctx.r[30].s64 = 31;
	// 832AB2CC: 396B8A38  addi r11, r11, -0x75c8
	ctx.r[11].s64 = ctx.r[11].s64 + -30152;
	// 832AB2D0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832AB2D4: 3BEB0080  addi r31, r11, 0x80
	ctx.r[31].s64 = ctx.r[11].s64 + 128;
	// 832AB2D8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832AB2DC: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	pc = 0x832AB2E0; continue 'dispatch;
            }
            0x832AB2E0 => {
    //   block [0x832AB2E0..0x832AB2F0)
	// 832AB2E0: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832AB2E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AB2E8: 4AF1B481  bl 0x821c6768
	ctx.lr = 0x832AB2EC;
	sub_821C6768(ctx, base);
	// 832AB2EC: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	pc = 0x832AB2F0; continue 'dispatch;
            }
            0x832AB2F0 => {
    //   block [0x832AB2F0..0x832AB320)
	// 832AB2F0: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832AB2F4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AB2F8: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832AB2FC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832AB300: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832AB304: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AB308: 4082FFE8  bne 0x832ab2f0
	if !ctx.cr[0].eq {
	pc = 0x832AB2F0; continue 'dispatch;
	}
	// 832AB30C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832AB310: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832AB314: 4080FFCC  bge 0x832ab2e0
	if !ctx.cr[0].lt {
	pc = 0x832AB2E0; continue 'dispatch;
	}
	// 832AB318: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832AB31C: 4B9FE13C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB320 size=84
    let mut pc: u32 = 0x832AB320;
    'dispatch: loop {
        match pc {
            0x832AB320 => {
    //   block [0x832AB320..0x832AB348)
	// 832AB320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AB328: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AB32C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB330: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB334: 3BEB8AB8  addi r31, r11, -0x7548
	ctx.r[31].s64 = ctx.r[11].s64 + -30024;
	// 832AB338: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AB33C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832AB340: 419A0008  beq cr6, 0x832ab348
	if ctx.cr[6].eq {
	pc = 0x832AB348; continue 'dispatch;
	}
	// 832AB344: 4AF709F5  bl 0x8221bd38
	ctx.lr = 0x832AB348;
	sub_8221BD38(ctx, base);
	pc = 0x832AB348; continue 'dispatch;
            }
            0x832AB348 => {
    //   block [0x832AB348..0x832AB374)
	// 832AB348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB34C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AB350: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832AB354: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AB358: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832AB35C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832AB360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AB364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AB368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AB36C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AB370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB378 size=12
    let mut pc: u32 = 0x832AB378;
    'dispatch: loop {
        match pc {
            0x832AB378 => {
    //   block [0x832AB378..0x832AB384)
	// 832AB378: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB37C: 386B8AD0  addi r3, r11, -0x7530
	ctx.r[3].s64 = ctx.r[11].s64 + -30000;
	// 832AB380: 4AF69A58  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB388 size=12
    let mut pc: u32 = 0x832AB388;
    'dispatch: loop {
        match pc {
            0x832AB388 => {
    //   block [0x832AB388..0x832AB394)
	// 832AB388: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB38C: 386B8AD4  addi r3, r11, -0x752c
	ctx.r[3].s64 = ctx.r[11].s64 + -29996;
	// 832AB390: 4AF69A48  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB398 size=12
    let mut pc: u32 = 0x832AB398;
    'dispatch: loop {
        match pc {
            0x832AB398 => {
    //   block [0x832AB398..0x832AB3A4)
	// 832AB398: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB39C: 386B8AD8  addi r3, r11, -0x7528
	ctx.r[3].s64 = ctx.r[11].s64 + -29992;
	// 832AB3A0: 4AF69A38  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB3A8 size=12
    let mut pc: u32 = 0x832AB3A8;
    'dispatch: loop {
        match pc {
            0x832AB3A8 => {
    //   block [0x832AB3A8..0x832AB3B4)
	// 832AB3A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB3AC: 386B8ADC  addi r3, r11, -0x7524
	ctx.r[3].s64 = ctx.r[11].s64 + -29988;
	// 832AB3B0: 4AF69A28  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB3B8 size=12
    let mut pc: u32 = 0x832AB3B8;
    'dispatch: loop {
        match pc {
            0x832AB3B8 => {
    //   block [0x832AB3B8..0x832AB3C4)
	// 832AB3B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB3BC: 386B8AE0  addi r3, r11, -0x7520
	ctx.r[3].s64 = ctx.r[11].s64 + -29984;
	// 832AB3C0: 4AF69A18  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB3C8 size=12
    let mut pc: u32 = 0x832AB3C8;
    'dispatch: loop {
        match pc {
            0x832AB3C8 => {
    //   block [0x832AB3C8..0x832AB3D4)
	// 832AB3C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB3CC: 386B8AE4  addi r3, r11, -0x751c
	ctx.r[3].s64 = ctx.r[11].s64 + -29980;
	// 832AB3D0: 4AF69A08  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB3D8 size=12
    let mut pc: u32 = 0x832AB3D8;
    'dispatch: loop {
        match pc {
            0x832AB3D8 => {
    //   block [0x832AB3D8..0x832AB3E4)
	// 832AB3D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB3DC: 386B8B08  addi r3, r11, -0x74f8
	ctx.r[3].s64 = ctx.r[11].s64 + -29944;
	// 832AB3E0: 4AF699F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB3E8 size=12
    let mut pc: u32 = 0x832AB3E8;
    'dispatch: loop {
        match pc {
            0x832AB3E8 => {
    //   block [0x832AB3E8..0x832AB3F4)
	// 832AB3E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB3EC: 386B8B0C  addi r3, r11, -0x74f4
	ctx.r[3].s64 = ctx.r[11].s64 + -29940;
	// 832AB3F0: 4AF699E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB3F8 size=12
    let mut pc: u32 = 0x832AB3F8;
    'dispatch: loop {
        match pc {
            0x832AB3F8 => {
    //   block [0x832AB3F8..0x832AB404)
	// 832AB3F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB3FC: 386B8B10  addi r3, r11, -0x74f0
	ctx.r[3].s64 = ctx.r[11].s64 + -29936;
	// 832AB400: 4AF699D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB408 size=12
    let mut pc: u32 = 0x832AB408;
    'dispatch: loop {
        match pc {
            0x832AB408 => {
    //   block [0x832AB408..0x832AB414)
	// 832AB408: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB40C: 386B8B14  addi r3, r11, -0x74ec
	ctx.r[3].s64 = ctx.r[11].s64 + -29932;
	// 832AB410: 4AF699C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB418 size=12
    let mut pc: u32 = 0x832AB418;
    'dispatch: loop {
        match pc {
            0x832AB418 => {
    //   block [0x832AB418..0x832AB424)
	// 832AB418: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB41C: 386B8B18  addi r3, r11, -0x74e8
	ctx.r[3].s64 = ctx.r[11].s64 + -29928;
	// 832AB420: 4AF699B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB428 size=12
    let mut pc: u32 = 0x832AB428;
    'dispatch: loop {
        match pc {
            0x832AB428 => {
    //   block [0x832AB428..0x832AB434)
	// 832AB428: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB42C: 386B8B1C  addi r3, r11, -0x74e4
	ctx.r[3].s64 = ctx.r[11].s64 + -29924;
	// 832AB430: 4AF699A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB438 size=12
    let mut pc: u32 = 0x832AB438;
    'dispatch: loop {
        match pc {
            0x832AB438 => {
    //   block [0x832AB438..0x832AB444)
	// 832AB438: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB43C: 386B8B20  addi r3, r11, -0x74e0
	ctx.r[3].s64 = ctx.r[11].s64 + -29920;
	// 832AB440: 4AF69998  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB448 size=12
    let mut pc: u32 = 0x832AB448;
    'dispatch: loop {
        match pc {
            0x832AB448 => {
    //   block [0x832AB448..0x832AB454)
	// 832AB448: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB44C: 386B8B24  addi r3, r11, -0x74dc
	ctx.r[3].s64 = ctx.r[11].s64 + -29916;
	// 832AB450: 4AF69988  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB458 size=104
    let mut pc: u32 = 0x832AB458;
    'dispatch: loop {
        match pc {
            0x832AB458 => {
    //   block [0x832AB458..0x832AB480)
	// 832AB458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB45C: 4B9FDFAD  bl 0x82ca9408
	ctx.lr = 0x832AB460;
	sub_82CA93D0(ctx, base);
	// 832AB460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB464: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB468: 3BC00007  li r30, 7
	ctx.r[30].s64 = 7;
	// 832AB46C: 396B8B28  addi r11, r11, -0x74d8
	ctx.r[11].s64 = ctx.r[11].s64 + -29912;
	// 832AB470: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832AB474: 3BEB0020  addi r31, r11, 0x20
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	// 832AB478: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832AB47C: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	pc = 0x832AB480; continue 'dispatch;
            }
            0x832AB480 => {
    //   block [0x832AB480..0x832AB490)
	// 832AB480: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832AB484: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AB488: 4AF1B2E1  bl 0x821c6768
	ctx.lr = 0x832AB48C;
	sub_821C6768(ctx, base);
	// 832AB48C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	pc = 0x832AB490; continue 'dispatch;
            }
            0x832AB490 => {
    //   block [0x832AB490..0x832AB4C0)
	// 832AB490: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832AB494: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AB498: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832AB49C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832AB4A0: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832AB4A4: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AB4A8: 4082FFE8  bne 0x832ab490
	if !ctx.cr[0].eq {
	pc = 0x832AB490; continue 'dispatch;
	}
	// 832AB4AC: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832AB4B0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832AB4B4: 4080FFCC  bge 0x832ab480
	if !ctx.cr[0].lt {
	pc = 0x832AB480; continue 'dispatch;
	}
	// 832AB4B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832AB4BC: 4B9FDF9C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB4C0 size=68
    let mut pc: u32 = 0x832AB4C0;
    'dispatch: loop {
        match pc {
            0x832AB4C0 => {
    //   block [0x832AB4C0..0x832AB504)
	// 832AB4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB4C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AB4C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AB4CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB4D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB4D4: 3BEB8B90  addi r31, r11, -0x7470
	ctx.r[31].s64 = ctx.r[11].s64 + -29808;
	// 832AB4D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AB4DC: 4B481E5D  bl 0x8272d338
	ctx.lr = 0x832AB4E0;
	sub_8272D338(ctx, base);
	// 832AB4E0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AB4E4: 4AF70855  bl 0x8221bd38
	ctx.lr = 0x832AB4E8;
	sub_8221BD38(ctx, base);
	// 832AB4E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB4EC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AB4F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AB4F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AB4F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AB4FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AB500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB508 size=12
    let mut pc: u32 = 0x832AB508;
    'dispatch: loop {
        match pc {
            0x832AB508 => {
    //   block [0x832AB508..0x832AB514)
	// 832AB508: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB50C: 386B8B9C  addi r3, r11, -0x7464
	ctx.r[3].s64 = ctx.r[11].s64 + -29796;
	// 832AB510: 4AF698C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB518 size=12
    let mut pc: u32 = 0x832AB518;
    'dispatch: loop {
        match pc {
            0x832AB518 => {
    //   block [0x832AB518..0x832AB524)
	// 832AB518: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB51C: 386B8BA0  addi r3, r11, -0x7460
	ctx.r[3].s64 = ctx.r[11].s64 + -29792;
	// 832AB520: 4AF698B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB528 size=12
    let mut pc: u32 = 0x832AB528;
    'dispatch: loop {
        match pc {
            0x832AB528 => {
    //   block [0x832AB528..0x832AB534)
	// 832AB528: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB52C: 386B8BA4  addi r3, r11, -0x745c
	ctx.r[3].s64 = ctx.r[11].s64 + -29788;
	// 832AB530: 4AF698A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB538 size=12
    let mut pc: u32 = 0x832AB538;
    'dispatch: loop {
        match pc {
            0x832AB538 => {
    //   block [0x832AB538..0x832AB544)
	// 832AB538: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB53C: 386B8BA8  addi r3, r11, -0x7458
	ctx.r[3].s64 = ctx.r[11].s64 + -29784;
	// 832AB540: 4AF69898  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB548 size=12
    let mut pc: u32 = 0x832AB548;
    'dispatch: loop {
        match pc {
            0x832AB548 => {
    //   block [0x832AB548..0x832AB554)
	// 832AB548: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB54C: 386B8BAC  addi r3, r11, -0x7454
	ctx.r[3].s64 = ctx.r[11].s64 + -29780;
	// 832AB550: 4AF69888  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB558 size=12
    let mut pc: u32 = 0x832AB558;
    'dispatch: loop {
        match pc {
            0x832AB558 => {
    //   block [0x832AB558..0x832AB564)
	// 832AB558: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB55C: 386B8BB0  addi r3, r11, -0x7450
	ctx.r[3].s64 = ctx.r[11].s64 + -29776;
	// 832AB560: 4AF69878  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB568 size=104
    let mut pc: u32 = 0x832AB568;
    'dispatch: loop {
        match pc {
            0x832AB568 => {
    //   block [0x832AB568..0x832AB590)
	// 832AB568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB56C: 4B9FDE9D  bl 0x82ca9408
	ctx.lr = 0x832AB570;
	sub_82CA93D0(ctx, base);
	// 832AB570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB574: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832AB578: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 832AB57C: 396B3B04  addi r11, r11, 0x3b04
	ctx.r[11].s64 = ctx.r[11].s64 + 15108;
	// 832AB580: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832AB584: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 832AB588: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832AB58C: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	pc = 0x832AB590; continue 'dispatch;
            }
            0x832AB590 => {
    //   block [0x832AB590..0x832AB5A0)
	// 832AB590: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832AB594: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AB598: 4AF1B1D1  bl 0x821c6768
	ctx.lr = 0x832AB59C;
	sub_821C6768(ctx, base);
	// 832AB59C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	pc = 0x832AB5A0; continue 'dispatch;
            }
            0x832AB5A0 => {
    //   block [0x832AB5A0..0x832AB5D0)
	// 832AB5A0: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832AB5A4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AB5A8: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832AB5AC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832AB5B0: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832AB5B4: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AB5B8: 4082FFE8  bne 0x832ab5a0
	if !ctx.cr[0].eq {
	pc = 0x832AB5A0; continue 'dispatch;
	}
	// 832AB5BC: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832AB5C0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832AB5C4: 4080FFCC  bge 0x832ab590
	if !ctx.cr[0].lt {
	pc = 0x832AB590; continue 'dispatch;
	}
	// 832AB5C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832AB5CC: 4B9FDE8C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB5E0 size=84
    let mut pc: u32 = 0x832AB5E0;
    'dispatch: loop {
        match pc {
            0x832AB5E0 => {
    //   block [0x832AB5E0..0x832AB608)
	// 832AB5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AB5E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AB5EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB5F0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB5F4: 3BEB8BD0  addi r31, r11, -0x7430
	ctx.r[31].s64 = ctx.r[11].s64 + -29744;
	// 832AB5F8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AB5FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832AB600: 419A0008  beq cr6, 0x832ab608
	if ctx.cr[6].eq {
	pc = 0x832AB608; continue 'dispatch;
	}
	// 832AB604: 4AF70735  bl 0x8221bd38
	ctx.lr = 0x832AB608;
	sub_8221BD38(ctx, base);
	pc = 0x832AB608; continue 'dispatch;
            }
            0x832AB608 => {
    //   block [0x832AB608..0x832AB634)
	// 832AB608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB60C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AB610: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832AB614: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AB618: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832AB61C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832AB620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AB624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AB628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AB62C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AB630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB638 size=12
    let mut pc: u32 = 0x832AB638;
    'dispatch: loop {
        match pc {
            0x832AB638 => {
    //   block [0x832AB638..0x832AB644)
	// 832AB638: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB63C: 386B8BE4  addi r3, r11, -0x741c
	ctx.r[3].s64 = ctx.r[11].s64 + -29724;
	// 832AB640: 4AF69798  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB648 size=12
    let mut pc: u32 = 0x832AB648;
    'dispatch: loop {
        match pc {
            0x832AB648 => {
    //   block [0x832AB648..0x832AB654)
	// 832AB648: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB64C: 386B8BE8  addi r3, r11, -0x7418
	ctx.r[3].s64 = ctx.r[11].s64 + -29720;
	// 832AB650: 4AF69788  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB658 size=12
    let mut pc: u32 = 0x832AB658;
    'dispatch: loop {
        match pc {
            0x832AB658 => {
    //   block [0x832AB658..0x832AB664)
	// 832AB658: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB65C: 386B8BEC  addi r3, r11, -0x7414
	ctx.r[3].s64 = ctx.r[11].s64 + -29716;
	// 832AB660: 4AF69778  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB668 size=12
    let mut pc: u32 = 0x832AB668;
    'dispatch: loop {
        match pc {
            0x832AB668 => {
    //   block [0x832AB668..0x832AB674)
	// 832AB668: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB66C: 386B8BF0  addi r3, r11, -0x7410
	ctx.r[3].s64 = ctx.r[11].s64 + -29712;
	// 832AB670: 4AF69768  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB678 size=12
    let mut pc: u32 = 0x832AB678;
    'dispatch: loop {
        match pc {
            0x832AB678 => {
    //   block [0x832AB678..0x832AB684)
	// 832AB678: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB67C: 386B8BF4  addi r3, r11, -0x740c
	ctx.r[3].s64 = ctx.r[11].s64 + -29708;
	// 832AB680: 4B366EB8  b 0x82612538
	sub_82612538(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB688 size=20
    let mut pc: u32 = 0x832AB688;
    'dispatch: loop {
        match pc {
            0x832AB688 => {
    //   block [0x832AB688..0x832AB69C)
	// 832AB688: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB68C: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 832AB690: 396B2B90  addi r11, r11, 0x2b90
	ctx.r[11].s64 = ctx.r[11].s64 + 11152;
	// 832AB694: 916AA0F4  stw r11, -0x5f0c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24332 as u32), ctx.r[11].u32 ) };
	// 832AB698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB6A0 size=20
    let mut pc: u32 = 0x832AB6A0;
    'dispatch: loop {
        match pc {
            0x832AB6A0 => {
    //   block [0x832AB6A0..0x832AB6B4)
	// 832AB6A0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832AB6A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB6A8: 392A1C70  addi r9, r10, 0x1c70
	ctx.r[9].s64 = ctx.r[10].s64 + 7280;
	// 832AB6AC: 91690400  stw r11, 0x400(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(1024 as u32), ctx.r[11].u32 ) };
	// 832AB6B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB6B8 size=20
    let mut pc: u32 = 0x832AB6B8;
    'dispatch: loop {
        match pc {
            0x832AB6B8 => {
    //   block [0x832AB6B8..0x832AB6CC)
	// 832AB6B8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB6BC: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB6C0: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB6C4: 916A8C00  stw r11, -0x7400(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29696 as u32), ctx.r[11].u32 ) };
	// 832AB6C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB6D0 size=4
    let mut pc: u32 = 0x832AB6D0;
    'dispatch: loop {
        match pc {
            0x832AB6D0 => {
    //   block [0x832AB6D0..0x832AB6D4)
	// 832AB6D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB6D8 size=4
    let mut pc: u32 = 0x832AB6D8;
    'dispatch: loop {
        match pc {
            0x832AB6D8 => {
    //   block [0x832AB6D8..0x832AB6DC)
	// 832AB6D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB6E0 size=20
    let mut pc: u32 = 0x832AB6E0;
    'dispatch: loop {
        match pc {
            0x832AB6E0 => {
    //   block [0x832AB6E0..0x832AB6F4)
	// 832AB6E0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB6E4: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB6E8: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB6EC: 916A8D4C  stw r11, -0x72b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29364 as u32), ctx.r[11].u32 ) };
	// 832AB6F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB6F8 size=20
    let mut pc: u32 = 0x832AB6F8;
    'dispatch: loop {
        match pc {
            0x832AB6F8 => {
    //   block [0x832AB6F8..0x832AB70C)
	// 832AB6F8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB6FC: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB700: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB704: 916A8E60  stw r11, -0x71a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29088 as u32), ctx.r[11].u32 ) };
	// 832AB708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB710 size=20
    let mut pc: u32 = 0x832AB710;
    'dispatch: loop {
        match pc {
            0x832AB710 => {
    //   block [0x832AB710..0x832AB724)
	// 832AB710: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB714: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB718: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB71C: 916A8F74  stw r11, -0x708c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28812 as u32), ctx.r[11].u32 ) };
	// 832AB720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB728 size=20
    let mut pc: u32 = 0x832AB728;
    'dispatch: loop {
        match pc {
            0x832AB728 => {
    //   block [0x832AB728..0x832AB73C)
	// 832AB728: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB72C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB730: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB734: 916A9088  stw r11, -0x6f78(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28536 as u32), ctx.r[11].u32 ) };
	// 832AB738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB740 size=20
    let mut pc: u32 = 0x832AB740;
    'dispatch: loop {
        match pc {
            0x832AB740 => {
    //   block [0x832AB740..0x832AB754)
	// 832AB740: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB744: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB748: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB74C: 916A919C  stw r11, -0x6e64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28260 as u32), ctx.r[11].u32 ) };
	// 832AB750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB758 size=68
    let mut pc: u32 = 0x832AB758;
    'dispatch: loop {
        match pc {
            0x832AB758 => {
    //   block [0x832AB758..0x832AB79C)
	// 832AB758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AB760: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AB764: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB768: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB76C: 3BEB92B0  addi r31, r11, -0x6d50
	ctx.r[31].s64 = ctx.r[11].s64 + -27984;
	// 832AB770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AB774: 4B077075  bl 0x823227e8
	ctx.lr = 0x832AB778;
	sub_823227E8(ctx, base);
	// 832AB778: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AB77C: 4AF705BD  bl 0x8221bd38
	ctx.lr = 0x832AB780;
	sub_8221BD38(ctx, base);
	// 832AB780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB784: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AB788: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AB78C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AB790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AB794: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AB798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB7A0 size=20
    let mut pc: u32 = 0x832AB7A0;
    'dispatch: loop {
        match pc {
            0x832AB7A0 => {
    //   block [0x832AB7A0..0x832AB7B4)
	// 832AB7A0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB7A4: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB7A8: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB7AC: 916A92BC  stw r11, -0x6d44(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-27972 as u32), ctx.r[11].u32 ) };
	// 832AB7B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB7B8 size=20
    let mut pc: u32 = 0x832AB7B8;
    'dispatch: loop {
        match pc {
            0x832AB7B8 => {
    //   block [0x832AB7B8..0x832AB7CC)
	// 832AB7B8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB7BC: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB7C0: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB7C4: 916A93D0  stw r11, -0x6c30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-27696 as u32), ctx.r[11].u32 ) };
	// 832AB7C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB7D0 size=20
    let mut pc: u32 = 0x832AB7D0;
    'dispatch: loop {
        match pc {
            0x832AB7D0 => {
    //   block [0x832AB7D0..0x832AB7E4)
	// 832AB7D0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB7D4: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB7D8: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB7DC: 916A94E4  stw r11, -0x6b1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-27420 as u32), ctx.r[11].u32 ) };
	// 832AB7E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB7E8 size=20
    let mut pc: u32 = 0x832AB7E8;
    'dispatch: loop {
        match pc {
            0x832AB7E8 => {
    //   block [0x832AB7E8..0x832AB7FC)
	// 832AB7E8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB7EC: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB7F0: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB7F4: 916A95F8  stw r11, -0x6a08(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-27144 as u32), ctx.r[11].u32 ) };
	// 832AB7F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB800 size=20
    let mut pc: u32 = 0x832AB800;
    'dispatch: loop {
        match pc {
            0x832AB800 => {
    //   block [0x832AB800..0x832AB814)
	// 832AB800: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB804: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB808: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB80C: 916A970C  stw r11, -0x68f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26868 as u32), ctx.r[11].u32 ) };
	// 832AB810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB818 size=20
    let mut pc: u32 = 0x832AB818;
    'dispatch: loop {
        match pc {
            0x832AB818 => {
    //   block [0x832AB818..0x832AB82C)
	// 832AB818: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB81C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB820: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB824: 916A9824  stw r11, -0x67dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26588 as u32), ctx.r[11].u32 ) };
	// 832AB828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB830 size=20
    let mut pc: u32 = 0x832AB830;
    'dispatch: loop {
        match pc {
            0x832AB830 => {
    //   block [0x832AB830..0x832AB844)
	// 832AB830: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB834: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB838: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB83C: 916A9938  stw r11, -0x66c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26312 as u32), ctx.r[11].u32 ) };
	// 832AB840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB848 size=20
    let mut pc: u32 = 0x832AB848;
    'dispatch: loop {
        match pc {
            0x832AB848 => {
    //   block [0x832AB848..0x832AB85C)
	// 832AB848: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB84C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB850: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB854: 916A9A4C  stw r11, -0x65b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26036 as u32), ctx.r[11].u32 ) };
	// 832AB858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB860 size=20
    let mut pc: u32 = 0x832AB860;
    'dispatch: loop {
        match pc {
            0x832AB860 => {
    //   block [0x832AB860..0x832AB874)
	// 832AB860: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB864: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB868: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB86C: 916A9B60  stw r11, -0x64a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-25760 as u32), ctx.r[11].u32 ) };
	// 832AB870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB878 size=12
    let mut pc: u32 = 0x832AB878;
    'dispatch: loop {
        match pc {
            0x832AB878 => {
    //   block [0x832AB878..0x832AB884)
	// 832AB878: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB87C: 386B9C78  addi r3, r11, -0x6388
	ctx.r[3].s64 = ctx.r[11].s64 + -25480;
	// 832AB880: 4B37F990  b 0x8262b210
	sub_8262B210(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB888 size=12
    let mut pc: u32 = 0x832AB888;
    'dispatch: loop {
        match pc {
            0x832AB888 => {
    //   block [0x832AB888..0x832AB894)
	// 832AB888: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832AB88C: 386B3BD4  addi r3, r11, 0x3bd4
	ctx.r[3].s64 = ctx.r[11].s64 + 15316;
	// 832AB890: 4B130F98  b 0x823dc828
	sub_823DC828(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB898 size=20
    let mut pc: u32 = 0x832AB898;
    'dispatch: loop {
        match pc {
            0x832AB898 => {
    //   block [0x832AB898..0x832AB8AC)
	// 832AB898: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB89C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB8A0: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB8A4: 916A9C84  stw r11, -0x637c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-25468 as u32), ctx.r[11].u32 ) };
	// 832AB8A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB8B0 size=20
    let mut pc: u32 = 0x832AB8B0;
    'dispatch: loop {
        match pc {
            0x832AB8B0 => {
    //   block [0x832AB8B0..0x832AB8C4)
	// 832AB8B0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB8B4: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB8B8: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB8BC: 916A9D98  stw r11, -0x6268(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-25192 as u32), ctx.r[11].u32 ) };
	// 832AB8C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AB8C8 size=68
    let mut pc: u32 = 0x832AB8C8;
    'dispatch: loop {
        match pc {
            0x832AB8C8 => {
    //   block [0x832AB8C8..0x832AB90C)
	// 832AB8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AB8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832AB8D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832AB8D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AB8D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AB8DC: 3BEB9EAC  addi r31, r11, -0x6154
	ctx.r[31].s64 = ctx.r[11].s64 + -24916;
	// 832AB8E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AB8E4: 4B37FC65  bl 0x8262b548
	ctx.lr = 0x832AB8E8;
	sub_8262B548(ctx, base);
	// 832AB8E8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832AB8EC: 4AF7044D  bl 0x8221bd38
	ctx.lr = 0x832AB8F0;
	sub_8221BD38(ctx, base);
	// 832AB8F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB8F4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832AB8F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832AB8FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832AB900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832AB904: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832AB908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB910 size=20
    let mut pc: u32 = 0x832AB910;
    'dispatch: loop {
        match pc {
            0x832AB910 => {
    //   block [0x832AB910..0x832AB924)
	// 832AB910: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832AB914: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB918: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832AB91C: 916A9EB8  stw r11, -0x6148(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24904 as u32), ctx.r[11].u32 ) };
	// 832AB920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB928 size=20
    let mut pc: u32 = 0x832AB928;
    'dispatch: loop {
        match pc {
            0x832AB928 => {
    //   block [0x832AB928..0x832AB93C)
	// 832AB928: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832AB92C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB930: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832AB934: 916A9FC4  stw r11, -0x603c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24636 as u32), ctx.r[11].u32 ) };
	// 832AB938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB940 size=20
    let mut pc: u32 = 0x832AB940;
    'dispatch: loop {
        match pc {
            0x832AB940 => {
    //   block [0x832AB940..0x832AB954)
	// 832AB940: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832AB944: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB948: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832AB94C: 916AA0D0  stw r11, -0x5f30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24368 as u32), ctx.r[11].u32 ) };
	// 832AB950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB958 size=20
    let mut pc: u32 = 0x832AB958;
    'dispatch: loop {
        match pc {
            0x832AB958 => {
    //   block [0x832AB958..0x832AB96C)
	// 832AB958: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832AB95C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB960: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832AB964: 916AA1E4  stw r11, -0x5e1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24092 as u32), ctx.r[11].u32 ) };
	// 832AB968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB970 size=20
    let mut pc: u32 = 0x832AB970;
    'dispatch: loop {
        match pc {
            0x832AB970 => {
    //   block [0x832AB970..0x832AB984)
	// 832AB970: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832AB974: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB978: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832AB97C: 916AA2F0  stw r11, -0x5d10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-23824 as u32), ctx.r[11].u32 ) };
	// 832AB980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB988 size=20
    let mut pc: u32 = 0x832AB988;
    'dispatch: loop {
        match pc {
            0x832AB988 => {
    //   block [0x832AB988..0x832AB99C)
	// 832AB988: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832AB98C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832AB990: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832AB994: 916AA3FC  stw r11, -0x5c04(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-23556 as u32), ctx.r[11].u32 ) };
	// 832AB998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB9A0 size=28
    let mut pc: u32 = 0x832AB9A0;
    'dispatch: loop {
        match pc {
            0x832AB9A0 => {
    //   block [0x832AB9A0..0x832AB9BC)
	// 832AB9A0: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 832AB9A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB9A8: 392A1C70  addi r9, r10, 0x1c70
	ctx.r[9].s64 = ctx.r[10].s64 + 7280;
	// 832AB9AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AB9B0: 91690040  stw r11, 0x40(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 832AB9B4: 99490044  stb r10, 0x44(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[10].u8 ) };
	// 832AB9B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB9C0 size=28
    let mut pc: u32 = 0x832AB9C0;
    'dispatch: loop {
        match pc {
            0x832AB9C0 => {
    //   block [0x832AB9C0..0x832AB9DC)
	// 832AB9C0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832AB9C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB9C8: 392A1B30  addi r9, r10, 0x1b30
	ctx.r[9].s64 = ctx.r[10].s64 + 6960;
	// 832AB9CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AB9D0: 91690040  stw r11, 0x40(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 832AB9D4: 99490044  stb r10, 0x44(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[10].u8 ) };
	// 832AB9D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AB9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AB9E0 size=28
    let mut pc: u32 = 0x832AB9E0;
    'dispatch: loop {
        match pc {
            0x832AB9E0 => {
    //   block [0x832AB9E0..0x832AB9FC)
	// 832AB9E0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832AB9E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832AB9E8: 392A1B80  addi r9, r10, 0x1b80
	ctx.r[9].s64 = ctx.r[10].s64 + 7040;
	// 832AB9EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832AB9F0: 91690040  stw r11, 0x40(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 832AB9F4: 99490044  stb r10, 0x44(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[10].u8 ) };
	// 832AB9F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABA00 size=28
    let mut pc: u32 = 0x832ABA00;
    'dispatch: loop {
        match pc {
            0x832ABA00 => {
    //   block [0x832ABA00..0x832ABA1C)
	// 832ABA00: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832ABA04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ABA08: 392A1BD0  addi r9, r10, 0x1bd0
	ctx.r[9].s64 = ctx.r[10].s64 + 7120;
	// 832ABA0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832ABA10: 91690040  stw r11, 0x40(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 832ABA14: 99490044  stb r10, 0x44(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[10].u8 ) };
	// 832ABA18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABA20 size=28
    let mut pc: u32 = 0x832ABA20;
    'dispatch: loop {
        match pc {
            0x832ABA20 => {
    //   block [0x832ABA20..0x832ABA3C)
	// 832ABA20: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832ABA24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ABA28: 392A1C20  addi r9, r10, 0x1c20
	ctx.r[9].s64 = ctx.r[10].s64 + 7200;
	// 832ABA2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832ABA30: 91690040  stw r11, 0x40(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 832ABA34: 99490044  stb r10, 0x44(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[10].u8 ) };
	// 832ABA38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABA40 size=20
    let mut pc: u32 = 0x832ABA40;
    'dispatch: loop {
        match pc {
            0x832ABA40 => {
    //   block [0x832ABA40..0x832ABA54)
	// 832ABA40: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832ABA44: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832ABA48: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832ABA4C: 916AA508  stw r11, -0x5af8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-23288 as u32), ctx.r[11].u32 ) };
	// 832ABA50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABA58 size=12
    let mut pc: u32 = 0x832ABA58;
    'dispatch: loop {
        match pc {
            0x832ABA58 => {
    //   block [0x832ABA58..0x832ABA64)
	// 832ABA58: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABA5C: 386BA61C  addi r3, r11, -0x59e4
	ctx.r[3].s64 = ctx.r[11].s64 + -23012;
	// 832ABA60: 4AF69378  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABA68 size=12
    let mut pc: u32 = 0x832ABA68;
    'dispatch: loop {
        match pc {
            0x832ABA68 => {
    //   block [0x832ABA68..0x832ABA74)
	// 832ABA68: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABA6C: 386BA620  addi r3, r11, -0x59e0
	ctx.r[3].s64 = ctx.r[11].s64 + -23008;
	// 832ABA70: 4AF69368  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABA78 size=12
    let mut pc: u32 = 0x832ABA78;
    'dispatch: loop {
        match pc {
            0x832ABA78 => {
    //   block [0x832ABA78..0x832ABA84)
	// 832ABA78: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABA7C: 386BA624  addi r3, r11, -0x59dc
	ctx.r[3].s64 = ctx.r[11].s64 + -23004;
	// 832ABA80: 4AF69358  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABA88 size=12
    let mut pc: u32 = 0x832ABA88;
    'dispatch: loop {
        match pc {
            0x832ABA88 => {
    //   block [0x832ABA88..0x832ABA94)
	// 832ABA88: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABA8C: 386BA628  addi r3, r11, -0x59d8
	ctx.r[3].s64 = ctx.r[11].s64 + -23000;
	// 832ABA90: 4AF69348  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABA98 size=12
    let mut pc: u32 = 0x832ABA98;
    'dispatch: loop {
        match pc {
            0x832ABA98 => {
    //   block [0x832ABA98..0x832ABAA4)
	// 832ABA98: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABA9C: 386BA62C  addi r3, r11, -0x59d4
	ctx.r[3].s64 = ctx.r[11].s64 + -22996;
	// 832ABAA0: 4AF69338  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABAA8 size=12
    let mut pc: u32 = 0x832ABAA8;
    'dispatch: loop {
        match pc {
            0x832ABAA8 => {
    //   block [0x832ABAA8..0x832ABAB4)
	// 832ABAA8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABAAC: 386BA630  addi r3, r11, -0x59d0
	ctx.r[3].s64 = ctx.r[11].s64 + -22992;
	// 832ABAB0: 4AF69328  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABAB8 size=80
    let mut pc: u32 = 0x832ABAB8;
    'dispatch: loop {
        match pc {
            0x832ABAB8 => {
    //   block [0x832ABAB8..0x832ABADC)
	// 832ABAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832ABAC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832ABAC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832ABAC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABACC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABAD0: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 832ABAD4: 396BA638  addi r11, r11, -0x59c8
	ctx.r[11].s64 = ctx.r[11].s64 + -22984;
	// 832ABAD8: 3BEB02C0  addi r31, r11, 0x2c0
	ctx.r[31].s64 = ctx.r[11].s64 + 704;
	pc = 0x832ABADC; continue 'dispatch;
            }
            0x832ABADC => {
    //   block [0x832ABADC..0x832ABB08)
	// 832ABADC: 3BFFFF50  addi r31, r31, -0xb0
	ctx.r[31].s64 = ctx.r[31].s64 + -176;
	// 832ABAE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ABAE4: 4B3AF085  bl 0x8265ab68
	ctx.lr = 0x832ABAE8;
	sub_8265AB68(ctx, base);
	// 832ABAE8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832ABAEC: 4080FFF0  bge 0x832abadc
	if !ctx.cr[0].lt {
	pc = 0x832ABADC; continue 'dispatch;
	}
	// 832ABAF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832ABAF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832ABAF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832ABAFC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832ABB00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832ABB04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABB28 size=84
    let mut pc: u32 = 0x832ABB28;
    'dispatch: loop {
        match pc {
            0x832ABB28 => {
    //   block [0x832ABB28..0x832ABB50)
	// 832ABB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832ABB30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832ABB34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABB38: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABB3C: 3BEBA920  addi r31, r11, -0x56e0
	ctx.r[31].s64 = ctx.r[11].s64 + -22240;
	// 832ABB40: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832ABB44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832ABB48: 419A0008  beq cr6, 0x832abb50
	if ctx.cr[6].eq {
	pc = 0x832ABB50; continue 'dispatch;
	}
	// 832ABB4C: 4AF701ED  bl 0x8221bd38
	ctx.lr = 0x832ABB50;
	sub_8221BD38(ctx, base);
	pc = 0x832ABB50; continue 'dispatch;
            }
            0x832ABB50 => {
    //   block [0x832ABB50..0x832ABB7C)
	// 832ABB50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ABB54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832ABB58: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832ABB5C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832ABB60: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832ABB64: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832ABB68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832ABB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832ABB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832ABB74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832ABB78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABB80 size=68
    let mut pc: u32 = 0x832ABB80;
    'dispatch: loop {
        match pc {
            0x832ABB80 => {
    //   block [0x832ABB80..0x832ABBC4)
	// 832ABB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832ABB88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832ABB8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABB90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABB94: 3BEBA930  addi r31, r11, -0x56d0
	ctx.r[31].s64 = ctx.r[11].s64 + -22224;
	// 832ABB98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ABB9C: 4B3BC5BD  bl 0x82668158
	ctx.lr = 0x832ABBA0;
	sub_82668158(ctx, base);
	// 832ABBA0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832ABBA4: 4AF70195  bl 0x8221bd38
	ctx.lr = 0x832ABBA8;
	sub_8221BD38(ctx, base);
	// 832ABBA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ABBAC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832ABBB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832ABBB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832ABBB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832ABBBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832ABBC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABBC8 size=12
    let mut pc: u32 = 0x832ABBC8;
    'dispatch: loop {
        match pc {
            0x832ABBC8 => {
    //   block [0x832ABBC8..0x832ABBD4)
	// 832ABBC8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABBCC: 386BA93C  addi r3, r11, -0x56c4
	ctx.r[3].s64 = ctx.r[11].s64 + -22212;
	// 832ABBD0: 4AF69208  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABBD8 size=12
    let mut pc: u32 = 0x832ABBD8;
    'dispatch: loop {
        match pc {
            0x832ABBD8 => {
    //   block [0x832ABBD8..0x832ABBE4)
	// 832ABBD8: 3D60834F  lis r11, -0x7cb1
	ctx.r[11].s64 = -2091974656;
	// 832ABBDC: 386B755C  addi r3, r11, 0x755c
	ctx.r[3].s64 = ctx.r[11].s64 + 30044;
	// 832ABBE0: 4AF691F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABBE8 size=12
    let mut pc: u32 = 0x832ABBE8;
    'dispatch: loop {
        match pc {
            0x832ABBE8 => {
    //   block [0x832ABBE8..0x832ABBF4)
	// 832ABBE8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABBEC: 386BA960  addi r3, r11, -0x56a0
	ctx.r[3].s64 = ctx.r[11].s64 + -22176;
	// 832ABBF0: 4AF691E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABBF8 size=12
    let mut pc: u32 = 0x832ABBF8;
    'dispatch: loop {
        match pc {
            0x832ABBF8 => {
    //   block [0x832ABBF8..0x832ABC04)
	// 832ABBF8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABBFC: 386BA964  addi r3, r11, -0x569c
	ctx.r[3].s64 = ctx.r[11].s64 + -22172;
	// 832ABC00: 4AF691D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABC08 size=12
    let mut pc: u32 = 0x832ABC08;
    'dispatch: loop {
        match pc {
            0x832ABC08 => {
    //   block [0x832ABC08..0x832ABC14)
	// 832ABC08: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABC0C: 386BA968  addi r3, r11, -0x5698
	ctx.r[3].s64 = ctx.r[11].s64 + -22168;
	// 832ABC10: 4AF691C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABC18 size=12
    let mut pc: u32 = 0x832ABC18;
    'dispatch: loop {
        match pc {
            0x832ABC18 => {
    //   block [0x832ABC18..0x832ABC24)
	// 832ABC18: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABC1C: 386BA96C  addi r3, r11, -0x5694
	ctx.r[3].s64 = ctx.r[11].s64 + -22164;
	// 832ABC20: 4AF691B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABC28 size=12
    let mut pc: u32 = 0x832ABC28;
    'dispatch: loop {
        match pc {
            0x832ABC28 => {
    //   block [0x832ABC28..0x832ABC34)
	// 832ABC28: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABC2C: 386BA970  addi r3, r11, -0x5690
	ctx.r[3].s64 = ctx.r[11].s64 + -22160;
	// 832ABC30: 4AF691A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABC38 size=12
    let mut pc: u32 = 0x832ABC38;
    'dispatch: loop {
        match pc {
            0x832ABC38 => {
    //   block [0x832ABC38..0x832ABC44)
	// 832ABC38: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABC3C: 386BA974  addi r3, r11, -0x568c
	ctx.r[3].s64 = ctx.r[11].s64 + -22156;
	// 832ABC40: 4AF69198  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABC48 size=12
    let mut pc: u32 = 0x832ABC48;
    'dispatch: loop {
        match pc {
            0x832ABC48 => {
    //   block [0x832ABC48..0x832ABC54)
	// 832ABC48: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABC4C: 386BA978  addi r3, r11, -0x5688
	ctx.r[3].s64 = ctx.r[11].s64 + -22152;
	// 832ABC50: 4AF69188  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABC58 size=12
    let mut pc: u32 = 0x832ABC58;
    'dispatch: loop {
        match pc {
            0x832ABC58 => {
    //   block [0x832ABC58..0x832ABC64)
	// 832ABC58: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABC5C: 386BA97C  addi r3, r11, -0x5684
	ctx.r[3].s64 = ctx.r[11].s64 + -22148;
	// 832ABC60: 4AF69178  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABC68 size=12
    let mut pc: u32 = 0x832ABC68;
    'dispatch: loop {
        match pc {
            0x832ABC68 => {
    //   block [0x832ABC68..0x832ABC74)
	// 832ABC68: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABC6C: 386BA980  addi r3, r11, -0x5680
	ctx.r[3].s64 = ctx.r[11].s64 + -22144;
	// 832ABC70: 4B3C1EE0  b 0x8266db50
	sub_8266DB50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABC78 size=12
    let mut pc: u32 = 0x832ABC78;
    'dispatch: loop {
        match pc {
            0x832ABC78 => {
    //   block [0x832ABC78..0x832ABC84)
	// 832ABC78: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABC7C: 386BA98C  addi r3, r11, -0x5674
	ctx.r[3].s64 = ctx.r[11].s64 + -22132;
	// 832ABC80: 4B3C21E0  b 0x8266de60
	sub_8266DE60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABC88 size=12
    let mut pc: u32 = 0x832ABC88;
    'dispatch: loop {
        match pc {
            0x832ABC88 => {
    //   block [0x832ABC88..0x832ABC94)
	// 832ABC88: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABC8C: 386BA998  addi r3, r11, -0x5668
	ctx.r[3].s64 = ctx.r[11].s64 + -22120;
	// 832ABC90: 4B3C7F50  b 0x82673be0
	sub_82673BE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABC98 size=68
    let mut pc: u32 = 0x832ABC98;
    'dispatch: loop {
        match pc {
            0x832ABC98 => {
    //   block [0x832ABC98..0x832ABCDC)
	// 832ABC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832ABCA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832ABCA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABCA8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABCAC: 3BEBA9A4  addi r31, r11, -0x565c
	ctx.r[31].s64 = ctx.r[11].s64 + -22108;
	// 832ABCB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ABCB4: 4B3C80E5  bl 0x82673d98
	ctx.lr = 0x832ABCB8;
	sub_82673D98(ctx, base);
	// 832ABCB8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832ABCBC: 4AF7007D  bl 0x8221bd38
	ctx.lr = 0x832ABCC0;
	sub_8221BD38(ctx, base);
	// 832ABCC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ABCC4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832ABCC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832ABCCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832ABCD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832ABCD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832ABCD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABCE0 size=84
    let mut pc: u32 = 0x832ABCE0;
    'dispatch: loop {
        match pc {
            0x832ABCE0 => {
    //   block [0x832ABCE0..0x832ABD08)
	// 832ABCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832ABCE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832ABCEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABCF0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABCF4: 3BEBA9B0  addi r31, r11, -0x5650
	ctx.r[31].s64 = ctx.r[11].s64 + -22096;
	// 832ABCF8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832ABCFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832ABD00: 419A0008  beq cr6, 0x832abd08
	if ctx.cr[6].eq {
	pc = 0x832ABD08; continue 'dispatch;
	}
	// 832ABD04: 4AF70035  bl 0x8221bd38
	ctx.lr = 0x832ABD08;
	sub_8221BD38(ctx, base);
	pc = 0x832ABD08; continue 'dispatch;
            }
            0x832ABD08 => {
    //   block [0x832ABD08..0x832ABD34)
	// 832ABD08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ABD0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832ABD10: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832ABD14: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832ABD18: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832ABD1C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832ABD20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832ABD24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832ABD28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832ABD2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832ABD30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABD38 size=68
    let mut pc: u32 = 0x832ABD38;
    'dispatch: loop {
        match pc {
            0x832ABD38 => {
    //   block [0x832ABD38..0x832ABD7C)
	// 832ABD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832ABD40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832ABD44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABD48: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABD4C: 3BEBA9D0  addi r31, r11, -0x5630
	ctx.r[31].s64 = ctx.r[11].s64 + -22064;
	// 832ABD50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ABD54: 4AEC7185  bl 0x82172ed8
	ctx.lr = 0x832ABD58;
	sub_82172ED8(ctx, base);
	// 832ABD58: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832ABD5C: 4AF6FFDD  bl 0x8221bd38
	ctx.lr = 0x832ABD60;
	sub_8221BD38(ctx, base);
	// 832ABD60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ABD64: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832ABD68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832ABD6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832ABD70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832ABD74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832ABD78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABD80 size=68
    let mut pc: u32 = 0x832ABD80;
    'dispatch: loop {
        match pc {
            0x832ABD80 => {
    //   block [0x832ABD80..0x832ABDC4)
	// 832ABD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832ABD88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832ABD8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABD90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABD94: 3BEBA9DC  addi r31, r11, -0x5624
	ctx.r[31].s64 = ctx.r[11].s64 + -22052;
	// 832ABD98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ABD9C: 4B029055  bl 0x822d4df0
	ctx.lr = 0x832ABDA0;
	sub_822D4DF0(ctx, base);
	// 832ABDA0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832ABDA4: 4AF6FF95  bl 0x8221bd38
	ctx.lr = 0x832ABDA8;
	sub_8221BD38(ctx, base);
	// 832ABDA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ABDAC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832ABDB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832ABDB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832ABDB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832ABDBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832ABDC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABDF0 size=104
    let mut pc: u32 = 0x832ABDF0;
    'dispatch: loop {
        match pc {
            0x832ABDF0 => {
    //   block [0x832ABDF0..0x832ABE18)
	// 832ABDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABDF4: 4B9FD615  bl 0x82ca9408
	ctx.lr = 0x832ABDF8;
	sub_82CA93D0(ctx, base);
	// 832ABDF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABDFC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABE00: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 832ABE04: 396BAA08  addi r11, r11, -0x55f8
	ctx.r[11].s64 = ctx.r[11].s64 + -22008;
	// 832ABE08: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832ABE0C: 3BEB0088  addi r31, r11, 0x88
	ctx.r[31].s64 = ctx.r[11].s64 + 136;
	// 832ABE10: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832ABE14: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	pc = 0x832ABE18; continue 'dispatch;
            }
            0x832ABE18 => {
    //   block [0x832ABE18..0x832ABE28)
	// 832ABE18: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832ABE1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ABE20: 4AF1A949  bl 0x821c6768
	ctx.lr = 0x832ABE24;
	sub_821C6768(ctx, base);
	// 832ABE24: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	pc = 0x832ABE28; continue 'dispatch;
            }
            0x832ABE28 => {
    //   block [0x832ABE28..0x832ABE58)
	// 832ABE28: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832ABE2C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832ABE30: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832ABE34: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832ABE38: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832ABE3C: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832ABE40: 4082FFE8  bne 0x832abe28
	if !ctx.cr[0].eq {
	pc = 0x832ABE28; continue 'dispatch;
	}
	// 832ABE44: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832ABE48: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832ABE4C: 4080FFCC  bge 0x832abe18
	if !ctx.cr[0].lt {
	pc = 0x832ABE18; continue 'dispatch;
	}
	// 832ABE50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832ABE54: 4B9FD604  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832ABE88 size=100
    let mut pc: u32 = 0x832ABE88;
    'dispatch: loop {
        match pc {
            0x832ABE88 => {
    //   block [0x832ABE88..0x832ABEC0)
	// 832ABE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832ABE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832ABE90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832ABE94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832ABE98: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABE9C: 3BEBAA9C  addi r31, r11, -0x5564
	ctx.r[31].s64 = ctx.r[11].s64 + -21860;
	// 832ABEA0: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832ABEA4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832ABEA8: 419A0018  beq cr6, 0x832abec0
	if ctx.cr[6].eq {
	pc = 0x832ABEC0; continue 'dispatch;
	}
	// 832ABEAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832ABEB0: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832ABEB4: 4B134015  bl 0x823dfec8
	ctx.lr = 0x832ABEB8;
	sub_823DFEC8(ctx, base);
	// 832ABEB8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832ABEBC: 4AF6FE7D  bl 0x8221bd38
	ctx.lr = 0x832ABEC0;
	sub_8221BD38(ctx, base);
	pc = 0x832ABEC0; continue 'dispatch;
            }
            0x832ABEC0 => {
    //   block [0x832ABEC0..0x832ABEEC)
	// 832ABEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832ABEC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832ABEC8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832ABECC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832ABED0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832ABED4: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832ABED8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832ABEDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832ABEE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832ABEE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832ABEE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABEF0 size=12
    let mut pc: u32 = 0x832ABEF0;
    'dispatch: loop {
        match pc {
            0x832ABEF0 => {
    //   block [0x832ABEF0..0x832ABEFC)
	// 832ABEF0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABEF4: 386BAAB0  addi r3, r11, -0x5550
	ctx.r[3].s64 = ctx.r[11].s64 + -21840;
	// 832ABEF8: 4AF68EE0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF00 size=12
    let mut pc: u32 = 0x832ABF00;
    'dispatch: loop {
        match pc {
            0x832ABF00 => {
    //   block [0x832ABF00..0x832ABF0C)
	// 832ABF00: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF04: 386BAAB4  addi r3, r11, -0x554c
	ctx.r[3].s64 = ctx.r[11].s64 + -21836;
	// 832ABF08: 4AF68ED0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF10 size=12
    let mut pc: u32 = 0x832ABF10;
    'dispatch: loop {
        match pc {
            0x832ABF10 => {
    //   block [0x832ABF10..0x832ABF1C)
	// 832ABF10: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF14: 386BAAB8  addi r3, r11, -0x5548
	ctx.r[3].s64 = ctx.r[11].s64 + -21832;
	// 832ABF18: 4AF68EC0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF20 size=12
    let mut pc: u32 = 0x832ABF20;
    'dispatch: loop {
        match pc {
            0x832ABF20 => {
    //   block [0x832ABF20..0x832ABF2C)
	// 832ABF20: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF24: 386BAABC  addi r3, r11, -0x5544
	ctx.r[3].s64 = ctx.r[11].s64 + -21828;
	// 832ABF28: 4AF68EB0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF30 size=12
    let mut pc: u32 = 0x832ABF30;
    'dispatch: loop {
        match pc {
            0x832ABF30 => {
    //   block [0x832ABF30..0x832ABF3C)
	// 832ABF30: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF34: 386BAAC0  addi r3, r11, -0x5540
	ctx.r[3].s64 = ctx.r[11].s64 + -21824;
	// 832ABF38: 4AF68EA0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF40 size=12
    let mut pc: u32 = 0x832ABF40;
    'dispatch: loop {
        match pc {
            0x832ABF40 => {
    //   block [0x832ABF40..0x832ABF4C)
	// 832ABF40: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF44: 386BAAC4  addi r3, r11, -0x553c
	ctx.r[3].s64 = ctx.r[11].s64 + -21820;
	// 832ABF48: 4AF68E90  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF50 size=12
    let mut pc: u32 = 0x832ABF50;
    'dispatch: loop {
        match pc {
            0x832ABF50 => {
    //   block [0x832ABF50..0x832ABF5C)
	// 832ABF50: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF54: 386BAAC8  addi r3, r11, -0x5538
	ctx.r[3].s64 = ctx.r[11].s64 + -21816;
	// 832ABF58: 4AF68E80  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF60 size=12
    let mut pc: u32 = 0x832ABF60;
    'dispatch: loop {
        match pc {
            0x832ABF60 => {
    //   block [0x832ABF60..0x832ABF6C)
	// 832ABF60: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF64: 386BAACC  addi r3, r11, -0x5534
	ctx.r[3].s64 = ctx.r[11].s64 + -21812;
	// 832ABF68: 4AF68E70  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF70 size=12
    let mut pc: u32 = 0x832ABF70;
    'dispatch: loop {
        match pc {
            0x832ABF70 => {
    //   block [0x832ABF70..0x832ABF7C)
	// 832ABF70: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF74: 386BAAD0  addi r3, r11, -0x5530
	ctx.r[3].s64 = ctx.r[11].s64 + -21808;
	// 832ABF78: 4AF68E60  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF80 size=12
    let mut pc: u32 = 0x832ABF80;
    'dispatch: loop {
        match pc {
            0x832ABF80 => {
    //   block [0x832ABF80..0x832ABF8C)
	// 832ABF80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF84: 386BAAD4  addi r3, r11, -0x552c
	ctx.r[3].s64 = ctx.r[11].s64 + -21804;
	// 832ABF88: 4AF68E50  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABF90 size=12
    let mut pc: u32 = 0x832ABF90;
    'dispatch: loop {
        match pc {
            0x832ABF90 => {
    //   block [0x832ABF90..0x832ABF9C)
	// 832ABF90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABF94: 386BAAD8  addi r3, r11, -0x5528
	ctx.r[3].s64 = ctx.r[11].s64 + -21800;
	// 832ABF98: 4AF68E40  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABFA0 size=12
    let mut pc: u32 = 0x832ABFA0;
    'dispatch: loop {
        match pc {
            0x832ABFA0 => {
    //   block [0x832ABFA0..0x832ABFAC)
	// 832ABFA0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABFA4: 386BAADC  addi r3, r11, -0x5524
	ctx.r[3].s64 = ctx.r[11].s64 + -21796;
	// 832ABFA8: 4AF68E30  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABFB0 size=12
    let mut pc: u32 = 0x832ABFB0;
    'dispatch: loop {
        match pc {
            0x832ABFB0 => {
    //   block [0x832ABFB0..0x832ABFBC)
	// 832ABFB0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABFB4: 386BAAE0  addi r3, r11, -0x5520
	ctx.r[3].s64 = ctx.r[11].s64 + -21792;
	// 832ABFB8: 4AF68E20  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABFC0 size=12
    let mut pc: u32 = 0x832ABFC0;
    'dispatch: loop {
        match pc {
            0x832ABFC0 => {
    //   block [0x832ABFC0..0x832ABFCC)
	// 832ABFC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABFC4: 386BAAE4  addi r3, r11, -0x551c
	ctx.r[3].s64 = ctx.r[11].s64 + -21788;
	// 832ABFC8: 4AF68E10  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABFD0 size=12
    let mut pc: u32 = 0x832ABFD0;
    'dispatch: loop {
        match pc {
            0x832ABFD0 => {
    //   block [0x832ABFD0..0x832ABFDC)
	// 832ABFD0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABFD4: 386BAAE8  addi r3, r11, -0x5518
	ctx.r[3].s64 = ctx.r[11].s64 + -21784;
	// 832ABFD8: 4AF68E00  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABFE0 size=12
    let mut pc: u32 = 0x832ABFE0;
    'dispatch: loop {
        match pc {
            0x832ABFE0 => {
    //   block [0x832ABFE0..0x832ABFEC)
	// 832ABFE0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABFE4: 386BAAEC  addi r3, r11, -0x5514
	ctx.r[3].s64 = ctx.r[11].s64 + -21780;
	// 832ABFE8: 4AF68DF0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832ABFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832ABFF0 size=12
    let mut pc: u32 = 0x832ABFF0;
    'dispatch: loop {
        match pc {
            0x832ABFF0 => {
    //   block [0x832ABFF0..0x832ABFFC)
	// 832ABFF0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832ABFF4: 386BAAF0  addi r3, r11, -0x5510
	ctx.r[3].s64 = ctx.r[11].s64 + -21776;
	// 832ABFF8: 4AF68DE0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC000 size=12
    let mut pc: u32 = 0x832AC000;
    'dispatch: loop {
        match pc {
            0x832AC000 => {
    //   block [0x832AC000..0x832AC00C)
	// 832AC000: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC004: 386BAAF4  addi r3, r11, -0x550c
	ctx.r[3].s64 = ctx.r[11].s64 + -21772;
	// 832AC008: 4AF68DD0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC010 size=12
    let mut pc: u32 = 0x832AC010;
    'dispatch: loop {
        match pc {
            0x832AC010 => {
    //   block [0x832AC010..0x832AC01C)
	// 832AC010: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC014: 386BAAF8  addi r3, r11, -0x5508
	ctx.r[3].s64 = ctx.r[11].s64 + -21768;
	// 832AC018: 4AF68DC0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC020 size=12
    let mut pc: u32 = 0x832AC020;
    'dispatch: loop {
        match pc {
            0x832AC020 => {
    //   block [0x832AC020..0x832AC02C)
	// 832AC020: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC024: 386BAAFC  addi r3, r11, -0x5504
	ctx.r[3].s64 = ctx.r[11].s64 + -21764;
	// 832AC028: 4AF68DB0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC030 size=12
    let mut pc: u32 = 0x832AC030;
    'dispatch: loop {
        match pc {
            0x832AC030 => {
    //   block [0x832AC030..0x832AC03C)
	// 832AC030: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC034: 386BAB00  addi r3, r11, -0x5500
	ctx.r[3].s64 = ctx.r[11].s64 + -21760;
	// 832AC038: 4AF68DA0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC040 size=12
    let mut pc: u32 = 0x832AC040;
    'dispatch: loop {
        match pc {
            0x832AC040 => {
    //   block [0x832AC040..0x832AC04C)
	// 832AC040: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC044: 386BAB04  addi r3, r11, -0x54fc
	ctx.r[3].s64 = ctx.r[11].s64 + -21756;
	// 832AC048: 4AF68D90  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC050 size=12
    let mut pc: u32 = 0x832AC050;
    'dispatch: loop {
        match pc {
            0x832AC050 => {
    //   block [0x832AC050..0x832AC05C)
	// 832AC050: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC054: 386BAB08  addi r3, r11, -0x54f8
	ctx.r[3].s64 = ctx.r[11].s64 + -21752;
	// 832AC058: 4AF68D80  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC060 size=12
    let mut pc: u32 = 0x832AC060;
    'dispatch: loop {
        match pc {
            0x832AC060 => {
    //   block [0x832AC060..0x832AC06C)
	// 832AC060: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC064: 386BAB0C  addi r3, r11, -0x54f4
	ctx.r[3].s64 = ctx.r[11].s64 + -21748;
	// 832AC068: 4B075678  b 0x823216e0
	sub_823216E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC070 size=12
    let mut pc: u32 = 0x832AC070;
    'dispatch: loop {
        match pc {
            0x832AC070 => {
    //   block [0x832AC070..0x832AC07C)
	// 832AC070: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC074: 386BAB18  addi r3, r11, -0x54e8
	ctx.r[3].s64 = ctx.r[11].s64 + -21736;
	// 832AC078: 4AF68D60  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC080 size=12
    let mut pc: u32 = 0x832AC080;
    'dispatch: loop {
        match pc {
            0x832AC080 => {
    //   block [0x832AC080..0x832AC08C)
	// 832AC080: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC084: 386BAB1C  addi r3, r11, -0x54e4
	ctx.r[3].s64 = ctx.r[11].s64 + -21732;
	// 832AC088: 4AF68D50  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC090 size=12
    let mut pc: u32 = 0x832AC090;
    'dispatch: loop {
        match pc {
            0x832AC090 => {
    //   block [0x832AC090..0x832AC09C)
	// 832AC090: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC094: 386BAB20  addi r3, r11, -0x54e0
	ctx.r[3].s64 = ctx.r[11].s64 + -21728;
	// 832AC098: 4AF68D40  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC0A0 size=12
    let mut pc: u32 = 0x832AC0A0;
    'dispatch: loop {
        match pc {
            0x832AC0A0 => {
    //   block [0x832AC0A0..0x832AC0AC)
	// 832AC0A0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC0A4: 386BAB24  addi r3, r11, -0x54dc
	ctx.r[3].s64 = ctx.r[11].s64 + -21724;
	// 832AC0A8: 4B76BCB8  b 0x82a17d60
	sub_82A17D60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832AC0B0 size=104
    let mut pc: u32 = 0x832AC0B0;
    'dispatch: loop {
        match pc {
            0x832AC0B0 => {
    //   block [0x832AC0B0..0x832AC0D8)
	// 832AC0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832AC0B4: 4B9FD355  bl 0x82ca9408
	ctx.lr = 0x832AC0B8;
	sub_82CA93D0(ctx, base);
	// 832AC0B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832AC0BC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC0C0: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 832AC0C4: 396BAB30  addi r11, r11, -0x54d0
	ctx.r[11].s64 = ctx.r[11].s64 + -21712;
	// 832AC0C8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832AC0CC: 3BEB0024  addi r31, r11, 0x24
	ctx.r[31].s64 = ctx.r[11].s64 + 36;
	// 832AC0D0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832AC0D4: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	pc = 0x832AC0D8; continue 'dispatch;
            }
            0x832AC0D8 => {
    //   block [0x832AC0D8..0x832AC0E8)
	// 832AC0D8: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832AC0DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832AC0E0: 4AF1A689  bl 0x821c6768
	ctx.lr = 0x832AC0E4;
	sub_821C6768(ctx, base);
	// 832AC0E4: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	pc = 0x832AC0E8; continue 'dispatch;
            }
            0x832AC0E8 => {
    //   block [0x832AC0E8..0x832AC118)
	// 832AC0E8: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832AC0EC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AC0F0: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832AC0F4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832AC0F8: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832AC0FC: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832AC100: 4082FFE8  bne 0x832ac0e8
	if !ctx.cr[0].eq {
	pc = 0x832AC0E8; continue 'dispatch;
	}
	// 832AC104: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832AC108: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832AC10C: 4080FFCC  bge 0x832ac0d8
	if !ctx.cr[0].lt {
	pc = 0x832AC0D8; continue 'dispatch;
	}
	// 832AC110: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832AC114: 4B9FD344  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC118 size=4
    let mut pc: u32 = 0x832AC118;
    'dispatch: loop {
        match pc {
            0x832AC118 => {
    //   block [0x832AC118..0x832AC11C)
	// 832AC118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC120 size=4
    let mut pc: u32 = 0x832AC120;
    'dispatch: loop {
        match pc {
            0x832AC120 => {
    //   block [0x832AC120..0x832AC124)
	// 832AC120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC128 size=12
    let mut pc: u32 = 0x832AC128;
    'dispatch: loop {
        match pc {
            0x832AC128 => {
    //   block [0x832AC128..0x832AC134)
	// 832AC128: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832AC12C: 386B3D2C  addi r3, r11, 0x3d2c
	ctx.r[3].s64 = ctx.r[11].s64 + 15660;
	// 832AC130: 4AF68CA8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC138 size=12
    let mut pc: u32 = 0x832AC138;
    'dispatch: loop {
        match pc {
            0x832AC138 => {
    //   block [0x832AC138..0x832AC144)
	// 832AC138: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC13C: 386BAB54  addi r3, r11, -0x54ac
	ctx.r[3].s64 = ctx.r[11].s64 + -21676;
	// 832AC140: 4AF68C98  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC148 size=12
    let mut pc: u32 = 0x832AC148;
    'dispatch: loop {
        match pc {
            0x832AC148 => {
    //   block [0x832AC148..0x832AC154)
	// 832AC148: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC14C: 386BAB58  addi r3, r11, -0x54a8
	ctx.r[3].s64 = ctx.r[11].s64 + -21672;
	// 832AC150: 4AF68C88  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC158 size=12
    let mut pc: u32 = 0x832AC158;
    'dispatch: loop {
        match pc {
            0x832AC158 => {
    //   block [0x832AC158..0x832AC164)
	// 832AC158: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC15C: 386BAB5C  addi r3, r11, -0x54a4
	ctx.r[3].s64 = ctx.r[11].s64 + -21668;
	// 832AC160: 4AF68C78  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC168 size=12
    let mut pc: u32 = 0x832AC168;
    'dispatch: loop {
        match pc {
            0x832AC168 => {
    //   block [0x832AC168..0x832AC174)
	// 832AC168: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC16C: 386BABB4  addi r3, r11, -0x544c
	ctx.r[3].s64 = ctx.r[11].s64 + -21580;
	// 832AC170: 4AF68C68  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC178 size=12
    let mut pc: u32 = 0x832AC178;
    'dispatch: loop {
        match pc {
            0x832AC178 => {
    //   block [0x832AC178..0x832AC184)
	// 832AC178: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC17C: 386BABB8  addi r3, r11, -0x5448
	ctx.r[3].s64 = ctx.r[11].s64 + -21576;
	// 832AC180: 4AF68C58  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC188 size=12
    let mut pc: u32 = 0x832AC188;
    'dispatch: loop {
        match pc {
            0x832AC188 => {
    //   block [0x832AC188..0x832AC194)
	// 832AC188: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC18C: 386BABBC  addi r3, r11, -0x5444
	ctx.r[3].s64 = ctx.r[11].s64 + -21572;
	// 832AC190: 4AF68C48  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC198 size=12
    let mut pc: u32 = 0x832AC198;
    'dispatch: loop {
        match pc {
            0x832AC198 => {
    //   block [0x832AC198..0x832AC1A4)
	// 832AC198: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC19C: 386BABC0  addi r3, r11, -0x5440
	ctx.r[3].s64 = ctx.r[11].s64 + -21568;
	// 832AC1A0: 4AF68C38  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC1A8 size=12
    let mut pc: u32 = 0x832AC1A8;
    'dispatch: loop {
        match pc {
            0x832AC1A8 => {
    //   block [0x832AC1A8..0x832AC1B4)
	// 832AC1A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC1AC: 386BABC4  addi r3, r11, -0x543c
	ctx.r[3].s64 = ctx.r[11].s64 + -21564;
	// 832AC1B0: 4AF68C28  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC1B8 size=12
    let mut pc: u32 = 0x832AC1B8;
    'dispatch: loop {
        match pc {
            0x832AC1B8 => {
    //   block [0x832AC1B8..0x832AC1C4)
	// 832AC1B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC1BC: 386BABC8  addi r3, r11, -0x5438
	ctx.r[3].s64 = ctx.r[11].s64 + -21560;
	// 832AC1C0: 4AF68C18  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC1C8 size=12
    let mut pc: u32 = 0x832AC1C8;
    'dispatch: loop {
        match pc {
            0x832AC1C8 => {
    //   block [0x832AC1C8..0x832AC1D4)
	// 832AC1C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC1CC: 386BABCC  addi r3, r11, -0x5434
	ctx.r[3].s64 = ctx.r[11].s64 + -21556;
	// 832AC1D0: 4AF68C08  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC1D8 size=12
    let mut pc: u32 = 0x832AC1D8;
    'dispatch: loop {
        match pc {
            0x832AC1D8 => {
    //   block [0x832AC1D8..0x832AC1E4)
	// 832AC1D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC1DC: 386BABD0  addi r3, r11, -0x5430
	ctx.r[3].s64 = ctx.r[11].s64 + -21552;
	// 832AC1E0: 4AF68BF8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC1E8 size=12
    let mut pc: u32 = 0x832AC1E8;
    'dispatch: loop {
        match pc {
            0x832AC1E8 => {
    //   block [0x832AC1E8..0x832AC1F4)
	// 832AC1E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC1EC: 386BABD4  addi r3, r11, -0x542c
	ctx.r[3].s64 = ctx.r[11].s64 + -21548;
	// 832AC1F0: 4AF68BE8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC1F8 size=12
    let mut pc: u32 = 0x832AC1F8;
    'dispatch: loop {
        match pc {
            0x832AC1F8 => {
    //   block [0x832AC1F8..0x832AC204)
	// 832AC1F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC1FC: 386BABD8  addi r3, r11, -0x5428
	ctx.r[3].s64 = ctx.r[11].s64 + -21544;
	// 832AC200: 4AF68BD8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC208 size=12
    let mut pc: u32 = 0x832AC208;
    'dispatch: loop {
        match pc {
            0x832AC208 => {
    //   block [0x832AC208..0x832AC214)
	// 832AC208: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC20C: 386BABDC  addi r3, r11, -0x5424
	ctx.r[3].s64 = ctx.r[11].s64 + -21540;
	// 832AC210: 4AF68BC8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC218 size=12
    let mut pc: u32 = 0x832AC218;
    'dispatch: loop {
        match pc {
            0x832AC218 => {
    //   block [0x832AC218..0x832AC224)
	// 832AC218: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC21C: 386BABE0  addi r3, r11, -0x5420
	ctx.r[3].s64 = ctx.r[11].s64 + -21536;
	// 832AC220: 4AF68BB8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC228 size=12
    let mut pc: u32 = 0x832AC228;
    'dispatch: loop {
        match pc {
            0x832AC228 => {
    //   block [0x832AC228..0x832AC234)
	// 832AC228: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC22C: 386BABE4  addi r3, r11, -0x541c
	ctx.r[3].s64 = ctx.r[11].s64 + -21532;
	// 832AC230: 4AF68BA8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC238 size=12
    let mut pc: u32 = 0x832AC238;
    'dispatch: loop {
        match pc {
            0x832AC238 => {
    //   block [0x832AC238..0x832AC244)
	// 832AC238: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC23C: 386BABE8  addi r3, r11, -0x5418
	ctx.r[3].s64 = ctx.r[11].s64 + -21528;
	// 832AC240: 4AF68B98  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC248 size=12
    let mut pc: u32 = 0x832AC248;
    'dispatch: loop {
        match pc {
            0x832AC248 => {
    //   block [0x832AC248..0x832AC254)
	// 832AC248: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC24C: 386BABEC  addi r3, r11, -0x5414
	ctx.r[3].s64 = ctx.r[11].s64 + -21524;
	// 832AC250: 4AF68B88  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC258 size=12
    let mut pc: u32 = 0x832AC258;
    'dispatch: loop {
        match pc {
            0x832AC258 => {
    //   block [0x832AC258..0x832AC264)
	// 832AC258: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC25C: 386BABF0  addi r3, r11, -0x5410
	ctx.r[3].s64 = ctx.r[11].s64 + -21520;
	// 832AC260: 4AF68B78  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC268 size=12
    let mut pc: u32 = 0x832AC268;
    'dispatch: loop {
        match pc {
            0x832AC268 => {
    //   block [0x832AC268..0x832AC274)
	// 832AC268: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC26C: 386BABF4  addi r3, r11, -0x540c
	ctx.r[3].s64 = ctx.r[11].s64 + -21516;
	// 832AC270: 4AF68B68  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC278 size=12
    let mut pc: u32 = 0x832AC278;
    'dispatch: loop {
        match pc {
            0x832AC278 => {
    //   block [0x832AC278..0x832AC284)
	// 832AC278: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC27C: 386BABF8  addi r3, r11, -0x5408
	ctx.r[3].s64 = ctx.r[11].s64 + -21512;
	// 832AC280: 4AF68B58  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC288 size=12
    let mut pc: u32 = 0x832AC288;
    'dispatch: loop {
        match pc {
            0x832AC288 => {
    //   block [0x832AC288..0x832AC294)
	// 832AC288: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC28C: 386BABFC  addi r3, r11, -0x5404
	ctx.r[3].s64 = ctx.r[11].s64 + -21508;
	// 832AC290: 4AF68B48  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC298 size=12
    let mut pc: u32 = 0x832AC298;
    'dispatch: loop {
        match pc {
            0x832AC298 => {
    //   block [0x832AC298..0x832AC2A4)
	// 832AC298: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC29C: 386BAC00  addi r3, r11, -0x5400
	ctx.r[3].s64 = ctx.r[11].s64 + -21504;
	// 832AC2A0: 4AF68B38  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC2A8 size=12
    let mut pc: u32 = 0x832AC2A8;
    'dispatch: loop {
        match pc {
            0x832AC2A8 => {
    //   block [0x832AC2A8..0x832AC2B4)
	// 832AC2A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC2AC: 386BAC04  addi r3, r11, -0x53fc
	ctx.r[3].s64 = ctx.r[11].s64 + -21500;
	// 832AC2B0: 4AF68B28  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC2B8 size=12
    let mut pc: u32 = 0x832AC2B8;
    'dispatch: loop {
        match pc {
            0x832AC2B8 => {
    //   block [0x832AC2B8..0x832AC2C4)
	// 832AC2B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC2BC: 386BAC08  addi r3, r11, -0x53f8
	ctx.r[3].s64 = ctx.r[11].s64 + -21496;
	// 832AC2C0: 4AF68B18  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC2C8 size=12
    let mut pc: u32 = 0x832AC2C8;
    'dispatch: loop {
        match pc {
            0x832AC2C8 => {
    //   block [0x832AC2C8..0x832AC2D4)
	// 832AC2C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC2CC: 386BAC0C  addi r3, r11, -0x53f4
	ctx.r[3].s64 = ctx.r[11].s64 + -21492;
	// 832AC2D0: 4AF68B08  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC2D8 size=12
    let mut pc: u32 = 0x832AC2D8;
    'dispatch: loop {
        match pc {
            0x832AC2D8 => {
    //   block [0x832AC2D8..0x832AC2E4)
	// 832AC2D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC2DC: 386BAC10  addi r3, r11, -0x53f0
	ctx.r[3].s64 = ctx.r[11].s64 + -21488;
	// 832AC2E0: 4AF68AF8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC2E8 size=4
    let mut pc: u32 = 0x832AC2E8;
    'dispatch: loop {
        match pc {
            0x832AC2E8 => {
    //   block [0x832AC2E8..0x832AC2EC)
	// 832AC2E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC2F0 size=4
    let mut pc: u32 = 0x832AC2F0;
    'dispatch: loop {
        match pc {
            0x832AC2F0 => {
    //   block [0x832AC2F0..0x832AC2F4)
	// 832AC2F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC2F8 size=4
    let mut pc: u32 = 0x832AC2F8;
    'dispatch: loop {
        match pc {
            0x832AC2F8 => {
    //   block [0x832AC2F8..0x832AC2FC)
	// 832AC2F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC300 size=4
    let mut pc: u32 = 0x832AC300;
    'dispatch: loop {
        match pc {
            0x832AC300 => {
    //   block [0x832AC300..0x832AC304)
	// 832AC300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC308 size=4
    let mut pc: u32 = 0x832AC308;
    'dispatch: loop {
        match pc {
            0x832AC308 => {
    //   block [0x832AC308..0x832AC30C)
	// 832AC308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC310 size=4
    let mut pc: u32 = 0x832AC310;
    'dispatch: loop {
        match pc {
            0x832AC310 => {
    //   block [0x832AC310..0x832AC314)
	// 832AC310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC318 size=4
    let mut pc: u32 = 0x832AC318;
    'dispatch: loop {
        match pc {
            0x832AC318 => {
    //   block [0x832AC318..0x832AC31C)
	// 832AC318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC320 size=4
    let mut pc: u32 = 0x832AC320;
    'dispatch: loop {
        match pc {
            0x832AC320 => {
    //   block [0x832AC320..0x832AC324)
	// 832AC320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC328 size=4
    let mut pc: u32 = 0x832AC328;
    'dispatch: loop {
        match pc {
            0x832AC328 => {
    //   block [0x832AC328..0x832AC32C)
	// 832AC328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC330 size=4
    let mut pc: u32 = 0x832AC330;
    'dispatch: loop {
        match pc {
            0x832AC330 => {
    //   block [0x832AC330..0x832AC334)
	// 832AC330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC338 size=4
    let mut pc: u32 = 0x832AC338;
    'dispatch: loop {
        match pc {
            0x832AC338 => {
    //   block [0x832AC338..0x832AC33C)
	// 832AC338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC340 size=12
    let mut pc: u32 = 0x832AC340;
    'dispatch: loop {
        match pc {
            0x832AC340 => {
    //   block [0x832AC340..0x832AC34C)
	// 832AC340: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC344: 386BAC40  addi r3, r11, -0x53c0
	ctx.r[3].s64 = ctx.r[11].s64 + -21440;
	// 832AC348: 4AF68A90  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC350 size=12
    let mut pc: u32 = 0x832AC350;
    'dispatch: loop {
        match pc {
            0x832AC350 => {
    //   block [0x832AC350..0x832AC35C)
	// 832AC350: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC354: 386BAC44  addi r3, r11, -0x53bc
	ctx.r[3].s64 = ctx.r[11].s64 + -21436;
	// 832AC358: 4AF68A80  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC360 size=12
    let mut pc: u32 = 0x832AC360;
    'dispatch: loop {
        match pc {
            0x832AC360 => {
    //   block [0x832AC360..0x832AC36C)
	// 832AC360: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC364: 386BAC48  addi r3, r11, -0x53b8
	ctx.r[3].s64 = ctx.r[11].s64 + -21432;
	// 832AC368: 4AF68A70  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC370 size=12
    let mut pc: u32 = 0x832AC370;
    'dispatch: loop {
        match pc {
            0x832AC370 => {
    //   block [0x832AC370..0x832AC37C)
	// 832AC370: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC374: 386BACA0  addi r3, r11, -0x5360
	ctx.r[3].s64 = ctx.r[11].s64 + -21344;
	// 832AC378: 4AF68A60  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC380 size=12
    let mut pc: u32 = 0x832AC380;
    'dispatch: loop {
        match pc {
            0x832AC380 => {
    //   block [0x832AC380..0x832AC38C)
	// 832AC380: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC384: 386BACA4  addi r3, r11, -0x535c
	ctx.r[3].s64 = ctx.r[11].s64 + -21340;
	// 832AC388: 4AF68A50  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC390 size=12
    let mut pc: u32 = 0x832AC390;
    'dispatch: loop {
        match pc {
            0x832AC390 => {
    //   block [0x832AC390..0x832AC39C)
	// 832AC390: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC394: 386BACA8  addi r3, r11, -0x5358
	ctx.r[3].s64 = ctx.r[11].s64 + -21336;
	// 832AC398: 4AF68A40  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC3A0 size=12
    let mut pc: u32 = 0x832AC3A0;
    'dispatch: loop {
        match pc {
            0x832AC3A0 => {
    //   block [0x832AC3A0..0x832AC3AC)
	// 832AC3A0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC3A4: 386BAD00  addi r3, r11, -0x5300
	ctx.r[3].s64 = ctx.r[11].s64 + -21248;
	// 832AC3A8: 4AF68A30  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC3B0 size=12
    let mut pc: u32 = 0x832AC3B0;
    'dispatch: loop {
        match pc {
            0x832AC3B0 => {
    //   block [0x832AC3B0..0x832AC3BC)
	// 832AC3B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC3B4: 386BAD04  addi r3, r11, -0x52fc
	ctx.r[3].s64 = ctx.r[11].s64 + -21244;
	// 832AC3B8: 4AF68A20  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC3C0 size=12
    let mut pc: u32 = 0x832AC3C0;
    'dispatch: loop {
        match pc {
            0x832AC3C0 => {
    //   block [0x832AC3C0..0x832AC3CC)
	// 832AC3C0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC3C4: 386BAD08  addi r3, r11, -0x52f8
	ctx.r[3].s64 = ctx.r[11].s64 + -21240;
	// 832AC3C8: 4AF68A10  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC3D0 size=12
    let mut pc: u32 = 0x832AC3D0;
    'dispatch: loop {
        match pc {
            0x832AC3D0 => {
    //   block [0x832AC3D0..0x832AC3DC)
	// 832AC3D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC3D4: 386BAD0C  addi r3, r11, -0x52f4
	ctx.r[3].s64 = ctx.r[11].s64 + -21236;
	// 832AC3D8: 4AF68A00  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC3E0 size=12
    let mut pc: u32 = 0x832AC3E0;
    'dispatch: loop {
        match pc {
            0x832AC3E0 => {
    //   block [0x832AC3E0..0x832AC3EC)
	// 832AC3E0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC3E4: 386BAD10  addi r3, r11, -0x52f0
	ctx.r[3].s64 = ctx.r[11].s64 + -21232;
	// 832AC3E8: 4AF689F0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC3F0 size=12
    let mut pc: u32 = 0x832AC3F0;
    'dispatch: loop {
        match pc {
            0x832AC3F0 => {
    //   block [0x832AC3F0..0x832AC3FC)
	// 832AC3F0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC3F4: 386BAD14  addi r3, r11, -0x52ec
	ctx.r[3].s64 = ctx.r[11].s64 + -21228;
	// 832AC3F8: 4AF689E0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC400 size=12
    let mut pc: u32 = 0x832AC400;
    'dispatch: loop {
        match pc {
            0x832AC400 => {
    //   block [0x832AC400..0x832AC40C)
	// 832AC400: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC404: 386BAD18  addi r3, r11, -0x52e8
	ctx.r[3].s64 = ctx.r[11].s64 + -21224;
	// 832AC408: 4AF689D0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC410 size=12
    let mut pc: u32 = 0x832AC410;
    'dispatch: loop {
        match pc {
            0x832AC410 => {
    //   block [0x832AC410..0x832AC41C)
	// 832AC410: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC414: 386BAD1C  addi r3, r11, -0x52e4
	ctx.r[3].s64 = ctx.r[11].s64 + -21220;
	// 832AC418: 4AF689C0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC420 size=12
    let mut pc: u32 = 0x832AC420;
    'dispatch: loop {
        match pc {
            0x832AC420 => {
    //   block [0x832AC420..0x832AC42C)
	// 832AC420: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC424: 386BAD20  addi r3, r11, -0x52e0
	ctx.r[3].s64 = ctx.r[11].s64 + -21216;
	// 832AC428: 4AF689B0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC430 size=12
    let mut pc: u32 = 0x832AC430;
    'dispatch: loop {
        match pc {
            0x832AC430 => {
    //   block [0x832AC430..0x832AC43C)
	// 832AC430: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC434: 386BAD24  addi r3, r11, -0x52dc
	ctx.r[3].s64 = ctx.r[11].s64 + -21212;
	// 832AC438: 4AF689A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC440 size=12
    let mut pc: u32 = 0x832AC440;
    'dispatch: loop {
        match pc {
            0x832AC440 => {
    //   block [0x832AC440..0x832AC44C)
	// 832AC440: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC444: 386BAD28  addi r3, r11, -0x52d8
	ctx.r[3].s64 = ctx.r[11].s64 + -21208;
	// 832AC448: 4AF68990  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC450 size=12
    let mut pc: u32 = 0x832AC450;
    'dispatch: loop {
        match pc {
            0x832AC450 => {
    //   block [0x832AC450..0x832AC45C)
	// 832AC450: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC454: 386BAD2C  addi r3, r11, -0x52d4
	ctx.r[3].s64 = ctx.r[11].s64 + -21204;
	// 832AC458: 4AF68980  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC460 size=12
    let mut pc: u32 = 0x832AC460;
    'dispatch: loop {
        match pc {
            0x832AC460 => {
    //   block [0x832AC460..0x832AC46C)
	// 832AC460: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC464: 386BAD30  addi r3, r11, -0x52d0
	ctx.r[3].s64 = ctx.r[11].s64 + -21200;
	// 832AC468: 4AF68970  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC470 size=12
    let mut pc: u32 = 0x832AC470;
    'dispatch: loop {
        match pc {
            0x832AC470 => {
    //   block [0x832AC470..0x832AC47C)
	// 832AC470: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC474: 386BAD34  addi r3, r11, -0x52cc
	ctx.r[3].s64 = ctx.r[11].s64 + -21196;
	// 832AC478: 4AF68960  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC480 size=12
    let mut pc: u32 = 0x832AC480;
    'dispatch: loop {
        match pc {
            0x832AC480 => {
    //   block [0x832AC480..0x832AC48C)
	// 832AC480: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC484: 386BAD38  addi r3, r11, -0x52c8
	ctx.r[3].s64 = ctx.r[11].s64 + -21192;
	// 832AC488: 4AF68950  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC490 size=12
    let mut pc: u32 = 0x832AC490;
    'dispatch: loop {
        match pc {
            0x832AC490 => {
    //   block [0x832AC490..0x832AC49C)
	// 832AC490: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC494: 386BAD3C  addi r3, r11, -0x52c4
	ctx.r[3].s64 = ctx.r[11].s64 + -21188;
	// 832AC498: 4AF68940  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC4A0 size=12
    let mut pc: u32 = 0x832AC4A0;
    'dispatch: loop {
        match pc {
            0x832AC4A0 => {
    //   block [0x832AC4A0..0x832AC4AC)
	// 832AC4A0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC4A4: 386BAD40  addi r3, r11, -0x52c0
	ctx.r[3].s64 = ctx.r[11].s64 + -21184;
	// 832AC4A8: 4AF68930  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC4B0 size=12
    let mut pc: u32 = 0x832AC4B0;
    'dispatch: loop {
        match pc {
            0x832AC4B0 => {
    //   block [0x832AC4B0..0x832AC4BC)
	// 832AC4B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC4B4: 386BAD44  addi r3, r11, -0x52bc
	ctx.r[3].s64 = ctx.r[11].s64 + -21180;
	// 832AC4B8: 4AF68920  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC4C0 size=12
    let mut pc: u32 = 0x832AC4C0;
    'dispatch: loop {
        match pc {
            0x832AC4C0 => {
    //   block [0x832AC4C0..0x832AC4CC)
	// 832AC4C0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC4C4: 386BAD48  addi r3, r11, -0x52b8
	ctx.r[3].s64 = ctx.r[11].s64 + -21176;
	// 832AC4C8: 4AF68910  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC4D0 size=12
    let mut pc: u32 = 0x832AC4D0;
    'dispatch: loop {
        match pc {
            0x832AC4D0 => {
    //   block [0x832AC4D0..0x832AC4DC)
	// 832AC4D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC4D4: 386BAD4C  addi r3, r11, -0x52b4
	ctx.r[3].s64 = ctx.r[11].s64 + -21172;
	// 832AC4D8: 4AF68900  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC4E0 size=12
    let mut pc: u32 = 0x832AC4E0;
    'dispatch: loop {
        match pc {
            0x832AC4E0 => {
    //   block [0x832AC4E0..0x832AC4EC)
	// 832AC4E0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC4E4: 386BAD50  addi r3, r11, -0x52b0
	ctx.r[3].s64 = ctx.r[11].s64 + -21168;
	// 832AC4E8: 4AF688F0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC4F0 size=12
    let mut pc: u32 = 0x832AC4F0;
    'dispatch: loop {
        match pc {
            0x832AC4F0 => {
    //   block [0x832AC4F0..0x832AC4FC)
	// 832AC4F0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC4F4: 386BAD54  addi r3, r11, -0x52ac
	ctx.r[3].s64 = ctx.r[11].s64 + -21164;
	// 832AC4F8: 4AF688E0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC500 size=12
    let mut pc: u32 = 0x832AC500;
    'dispatch: loop {
        match pc {
            0x832AC500 => {
    //   block [0x832AC500..0x832AC50C)
	// 832AC500: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC504: 386BAD58  addi r3, r11, -0x52a8
	ctx.r[3].s64 = ctx.r[11].s64 + -21160;
	// 832AC508: 4AF688D0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC510 size=12
    let mut pc: u32 = 0x832AC510;
    'dispatch: loop {
        match pc {
            0x832AC510 => {
    //   block [0x832AC510..0x832AC51C)
	// 832AC510: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC514: 386BAD5C  addi r3, r11, -0x52a4
	ctx.r[3].s64 = ctx.r[11].s64 + -21156;
	// 832AC518: 4AF688C0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC520 size=12
    let mut pc: u32 = 0x832AC520;
    'dispatch: loop {
        match pc {
            0x832AC520 => {
    //   block [0x832AC520..0x832AC52C)
	// 832AC520: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC524: 386BAD60  addi r3, r11, -0x52a0
	ctx.r[3].s64 = ctx.r[11].s64 + -21152;
	// 832AC528: 4AF688B0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC530 size=12
    let mut pc: u32 = 0x832AC530;
    'dispatch: loop {
        match pc {
            0x832AC530 => {
    //   block [0x832AC530..0x832AC53C)
	// 832AC530: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC534: 386BAD64  addi r3, r11, -0x529c
	ctx.r[3].s64 = ctx.r[11].s64 + -21148;
	// 832AC538: 4AF688A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC540 size=12
    let mut pc: u32 = 0x832AC540;
    'dispatch: loop {
        match pc {
            0x832AC540 => {
    //   block [0x832AC540..0x832AC54C)
	// 832AC540: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC544: 386BAD68  addi r3, r11, -0x5298
	ctx.r[3].s64 = ctx.r[11].s64 + -21144;
	// 832AC548: 4AF68890  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC550 size=12
    let mut pc: u32 = 0x832AC550;
    'dispatch: loop {
        match pc {
            0x832AC550 => {
    //   block [0x832AC550..0x832AC55C)
	// 832AC550: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC554: 386BAD6C  addi r3, r11, -0x5294
	ctx.r[3].s64 = ctx.r[11].s64 + -21140;
	// 832AC558: 4AF68880  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC560 size=12
    let mut pc: u32 = 0x832AC560;
    'dispatch: loop {
        match pc {
            0x832AC560 => {
    //   block [0x832AC560..0x832AC56C)
	// 832AC560: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC564: 386BAD70  addi r3, r11, -0x5290
	ctx.r[3].s64 = ctx.r[11].s64 + -21136;
	// 832AC568: 4AF68870  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC570 size=12
    let mut pc: u32 = 0x832AC570;
    'dispatch: loop {
        match pc {
            0x832AC570 => {
    //   block [0x832AC570..0x832AC57C)
	// 832AC570: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC574: 386BAD74  addi r3, r11, -0x528c
	ctx.r[3].s64 = ctx.r[11].s64 + -21132;
	// 832AC578: 4AF68860  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC580 size=12
    let mut pc: u32 = 0x832AC580;
    'dispatch: loop {
        match pc {
            0x832AC580 => {
    //   block [0x832AC580..0x832AC58C)
	// 832AC580: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC584: 386BAD78  addi r3, r11, -0x5288
	ctx.r[3].s64 = ctx.r[11].s64 + -21128;
	// 832AC588: 4AF68850  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC590 size=12
    let mut pc: u32 = 0x832AC590;
    'dispatch: loop {
        match pc {
            0x832AC590 => {
    //   block [0x832AC590..0x832AC59C)
	// 832AC590: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC594: 386BAD7C  addi r3, r11, -0x5284
	ctx.r[3].s64 = ctx.r[11].s64 + -21124;
	// 832AC598: 4AF68840  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC5A0 size=12
    let mut pc: u32 = 0x832AC5A0;
    'dispatch: loop {
        match pc {
            0x832AC5A0 => {
    //   block [0x832AC5A0..0x832AC5AC)
	// 832AC5A0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC5A4: 386BAD80  addi r3, r11, -0x5280
	ctx.r[3].s64 = ctx.r[11].s64 + -21120;
	// 832AC5A8: 4AF68830  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC5B0 size=12
    let mut pc: u32 = 0x832AC5B0;
    'dispatch: loop {
        match pc {
            0x832AC5B0 => {
    //   block [0x832AC5B0..0x832AC5BC)
	// 832AC5B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC5B4: 386BADD8  addi r3, r11, -0x5228
	ctx.r[3].s64 = ctx.r[11].s64 + -21032;
	// 832AC5B8: 4AF68820  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC5C0 size=12
    let mut pc: u32 = 0x832AC5C0;
    'dispatch: loop {
        match pc {
            0x832AC5C0 => {
    //   block [0x832AC5C0..0x832AC5CC)
	// 832AC5C0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC5C4: 386BADDC  addi r3, r11, -0x5224
	ctx.r[3].s64 = ctx.r[11].s64 + -21028;
	// 832AC5C8: 4AF68810  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC5D0 size=12
    let mut pc: u32 = 0x832AC5D0;
    'dispatch: loop {
        match pc {
            0x832AC5D0 => {
    //   block [0x832AC5D0..0x832AC5DC)
	// 832AC5D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC5D4: 386BADE0  addi r3, r11, -0x5220
	ctx.r[3].s64 = ctx.r[11].s64 + -21024;
	// 832AC5D8: 4AF68800  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC5E0 size=12
    let mut pc: u32 = 0x832AC5E0;
    'dispatch: loop {
        match pc {
            0x832AC5E0 => {
    //   block [0x832AC5E0..0x832AC5EC)
	// 832AC5E0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC5E4: 386BAE38  addi r3, r11, -0x51c8
	ctx.r[3].s64 = ctx.r[11].s64 + -20936;
	// 832AC5E8: 4AF687F0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC5F0 size=12
    let mut pc: u32 = 0x832AC5F0;
    'dispatch: loop {
        match pc {
            0x832AC5F0 => {
    //   block [0x832AC5F0..0x832AC5FC)
	// 832AC5F0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC5F4: 386BAE3C  addi r3, r11, -0x51c4
	ctx.r[3].s64 = ctx.r[11].s64 + -20932;
	// 832AC5F8: 4AF687E0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC600 size=12
    let mut pc: u32 = 0x832AC600;
    'dispatch: loop {
        match pc {
            0x832AC600 => {
    //   block [0x832AC600..0x832AC60C)
	// 832AC600: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC604: 386BAE9C  addi r3, r11, -0x5164
	ctx.r[3].s64 = ctx.r[11].s64 + -20836;
	// 832AC608: 4AF687D0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC610 size=12
    let mut pc: u32 = 0x832AC610;
    'dispatch: loop {
        match pc {
            0x832AC610 => {
    //   block [0x832AC610..0x832AC61C)
	// 832AC610: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC614: 386BAEA0  addi r3, r11, -0x5160
	ctx.r[3].s64 = ctx.r[11].s64 + -20832;
	// 832AC618: 4AF687C0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC620 size=12
    let mut pc: u32 = 0x832AC620;
    'dispatch: loop {
        match pc {
            0x832AC620 => {
    //   block [0x832AC620..0x832AC62C)
	// 832AC620: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC624: 386BAEF8  addi r3, r11, -0x5108
	ctx.r[3].s64 = ctx.r[11].s64 + -20744;
	// 832AC628: 4AF687B0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC630 size=12
    let mut pc: u32 = 0x832AC630;
    'dispatch: loop {
        match pc {
            0x832AC630 => {
    //   block [0x832AC630..0x832AC63C)
	// 832AC630: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC634: 386BAEFC  addi r3, r11, -0x5104
	ctx.r[3].s64 = ctx.r[11].s64 + -20740;
	// 832AC638: 4AF687A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC640 size=12
    let mut pc: u32 = 0x832AC640;
    'dispatch: loop {
        match pc {
            0x832AC640 => {
    //   block [0x832AC640..0x832AC64C)
	// 832AC640: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC644: 386BAF00  addi r3, r11, -0x5100
	ctx.r[3].s64 = ctx.r[11].s64 + -20736;
	// 832AC648: 4AF68790  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC650 size=12
    let mut pc: u32 = 0x832AC650;
    'dispatch: loop {
        match pc {
            0x832AC650 => {
    //   block [0x832AC650..0x832AC65C)
	// 832AC650: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC654: 386BAF04  addi r3, r11, -0x50fc
	ctx.r[3].s64 = ctx.r[11].s64 + -20732;
	// 832AC658: 4AF68780  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC660 size=12
    let mut pc: u32 = 0x832AC660;
    'dispatch: loop {
        match pc {
            0x832AC660 => {
    //   block [0x832AC660..0x832AC66C)
	// 832AC660: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC664: 386BAF08  addi r3, r11, -0x50f8
	ctx.r[3].s64 = ctx.r[11].s64 + -20728;
	// 832AC668: 4AF68770  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC670 size=12
    let mut pc: u32 = 0x832AC670;
    'dispatch: loop {
        match pc {
            0x832AC670 => {
    //   block [0x832AC670..0x832AC67C)
	// 832AC670: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC674: 386BAF60  addi r3, r11, -0x50a0
	ctx.r[3].s64 = ctx.r[11].s64 + -20640;
	// 832AC678: 4AF68760  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC680 size=12
    let mut pc: u32 = 0x832AC680;
    'dispatch: loop {
        match pc {
            0x832AC680 => {
    //   block [0x832AC680..0x832AC68C)
	// 832AC680: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC684: 386BAF64  addi r3, r11, -0x509c
	ctx.r[3].s64 = ctx.r[11].s64 + -20636;
	// 832AC688: 4AF68750  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC690 size=12
    let mut pc: u32 = 0x832AC690;
    'dispatch: loop {
        match pc {
            0x832AC690 => {
    //   block [0x832AC690..0x832AC69C)
	// 832AC690: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC694: 386BAF68  addi r3, r11, -0x5098
	ctx.r[3].s64 = ctx.r[11].s64 + -20632;
	// 832AC698: 4AF1A160  b 0x821c67f8
	sub_821C67F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC6A0 size=12
    let mut pc: u32 = 0x832AC6A0;
    'dispatch: loop {
        match pc {
            0x832AC6A0 => {
    //   block [0x832AC6A0..0x832AC6AC)
	// 832AC6A0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC6A4: 386BAF6C  addi r3, r11, -0x5094
	ctx.r[3].s64 = ctx.r[11].s64 + -20628;
	// 832AC6A8: 4AF68730  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC6B0 size=12
    let mut pc: u32 = 0x832AC6B0;
    'dispatch: loop {
        match pc {
            0x832AC6B0 => {
    //   block [0x832AC6B0..0x832AC6BC)
	// 832AC6B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC6B4: 386BAF70  addi r3, r11, -0x5090
	ctx.r[3].s64 = ctx.r[11].s64 + -20624;
	// 832AC6B8: 4AF68720  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC6C0 size=12
    let mut pc: u32 = 0x832AC6C0;
    'dispatch: loop {
        match pc {
            0x832AC6C0 => {
    //   block [0x832AC6C0..0x832AC6CC)
	// 832AC6C0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC6C4: 386BAF74  addi r3, r11, -0x508c
	ctx.r[3].s64 = ctx.r[11].s64 + -20620;
	// 832AC6C8: 4AF68710  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC6D0 size=12
    let mut pc: u32 = 0x832AC6D0;
    'dispatch: loop {
        match pc {
            0x832AC6D0 => {
    //   block [0x832AC6D0..0x832AC6DC)
	// 832AC6D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC6D4: 386BAF78  addi r3, r11, -0x5088
	ctx.r[3].s64 = ctx.r[11].s64 + -20616;
	// 832AC6D8: 4AF68700  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC6E0 size=12
    let mut pc: u32 = 0x832AC6E0;
    'dispatch: loop {
        match pc {
            0x832AC6E0 => {
    //   block [0x832AC6E0..0x832AC6EC)
	// 832AC6E0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC6E4: 386BAF7C  addi r3, r11, -0x5084
	ctx.r[3].s64 = ctx.r[11].s64 + -20612;
	// 832AC6E8: 4AF686F0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC6F0 size=12
    let mut pc: u32 = 0x832AC6F0;
    'dispatch: loop {
        match pc {
            0x832AC6F0 => {
    //   block [0x832AC6F0..0x832AC6FC)
	// 832AC6F0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC6F4: 386BAF80  addi r3, r11, -0x5080
	ctx.r[3].s64 = ctx.r[11].s64 + -20608;
	// 832AC6F8: 4AF686E0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC700 size=12
    let mut pc: u32 = 0x832AC700;
    'dispatch: loop {
        match pc {
            0x832AC700 => {
    //   block [0x832AC700..0x832AC70C)
	// 832AC700: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC704: 386BAF84  addi r3, r11, -0x507c
	ctx.r[3].s64 = ctx.r[11].s64 + -20604;
	// 832AC708: 4AF686D0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC710 size=12
    let mut pc: u32 = 0x832AC710;
    'dispatch: loop {
        match pc {
            0x832AC710 => {
    //   block [0x832AC710..0x832AC71C)
	// 832AC710: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC714: 386BAF88  addi r3, r11, -0x5078
	ctx.r[3].s64 = ctx.r[11].s64 + -20600;
	// 832AC718: 4AF686C0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC720 size=12
    let mut pc: u32 = 0x832AC720;
    'dispatch: loop {
        match pc {
            0x832AC720 => {
    //   block [0x832AC720..0x832AC72C)
	// 832AC720: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC724: 386BAF8C  addi r3, r11, -0x5074
	ctx.r[3].s64 = ctx.r[11].s64 + -20596;
	// 832AC728: 4AF686B0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC730 size=12
    let mut pc: u32 = 0x832AC730;
    'dispatch: loop {
        match pc {
            0x832AC730 => {
    //   block [0x832AC730..0x832AC73C)
	// 832AC730: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC734: 386BAFF4  addi r3, r11, -0x500c
	ctx.r[3].s64 = ctx.r[11].s64 + -20492;
	// 832AC738: 4AF686A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC740 size=12
    let mut pc: u32 = 0x832AC740;
    'dispatch: loop {
        match pc {
            0x832AC740 => {
    //   block [0x832AC740..0x832AC74C)
	// 832AC740: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC744: 386BAFF8  addi r3, r11, -0x5008
	ctx.r[3].s64 = ctx.r[11].s64 + -20488;
	// 832AC748: 4AF68690  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC750 size=12
    let mut pc: u32 = 0x832AC750;
    'dispatch: loop {
        match pc {
            0x832AC750 => {
    //   block [0x832AC750..0x832AC75C)
	// 832AC750: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC754: 386BAFFC  addi r3, r11, -0x5004
	ctx.r[3].s64 = ctx.r[11].s64 + -20484;
	// 832AC758: 4AF68680  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC760 size=12
    let mut pc: u32 = 0x832AC760;
    'dispatch: loop {
        match pc {
            0x832AC760 => {
    //   block [0x832AC760..0x832AC76C)
	// 832AC760: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC764: 386BB000  addi r3, r11, -0x5000
	ctx.r[3].s64 = ctx.r[11].s64 + -20480;
	// 832AC768: 4AF68670  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC770 size=12
    let mut pc: u32 = 0x832AC770;
    'dispatch: loop {
        match pc {
            0x832AC770 => {
    //   block [0x832AC770..0x832AC77C)
	// 832AC770: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC774: 386BB004  addi r3, r11, -0x4ffc
	ctx.r[3].s64 = ctx.r[11].s64 + -20476;
	// 832AC778: 4AF68660  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC780 size=12
    let mut pc: u32 = 0x832AC780;
    'dispatch: loop {
        match pc {
            0x832AC780 => {
    //   block [0x832AC780..0x832AC78C)
	// 832AC780: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC784: 386BB008  addi r3, r11, -0x4ff8
	ctx.r[3].s64 = ctx.r[11].s64 + -20472;
	// 832AC788: 4AF68650  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC790 size=12
    let mut pc: u32 = 0x832AC790;
    'dispatch: loop {
        match pc {
            0x832AC790 => {
    //   block [0x832AC790..0x832AC79C)
	// 832AC790: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC794: 386BB00C  addi r3, r11, -0x4ff4
	ctx.r[3].s64 = ctx.r[11].s64 + -20468;
	// 832AC798: 4AF68640  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC7A0 size=12
    let mut pc: u32 = 0x832AC7A0;
    'dispatch: loop {
        match pc {
            0x832AC7A0 => {
    //   block [0x832AC7A0..0x832AC7AC)
	// 832AC7A0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC7A4: 386BB010  addi r3, r11, -0x4ff0
	ctx.r[3].s64 = ctx.r[11].s64 + -20464;
	// 832AC7A8: 4AF68630  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC7B0 size=12
    let mut pc: u32 = 0x832AC7B0;
    'dispatch: loop {
        match pc {
            0x832AC7B0 => {
    //   block [0x832AC7B0..0x832AC7BC)
	// 832AC7B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC7B4: 386BB014  addi r3, r11, -0x4fec
	ctx.r[3].s64 = ctx.r[11].s64 + -20460;
	// 832AC7B8: 4AF68620  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC7C0 size=12
    let mut pc: u32 = 0x832AC7C0;
    'dispatch: loop {
        match pc {
            0x832AC7C0 => {
    //   block [0x832AC7C0..0x832AC7CC)
	// 832AC7C0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC7C4: 386BB018  addi r3, r11, -0x4fe8
	ctx.r[3].s64 = ctx.r[11].s64 + -20456;
	// 832AC7C8: 4AF68610  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC7D0 size=12
    let mut pc: u32 = 0x832AC7D0;
    'dispatch: loop {
        match pc {
            0x832AC7D0 => {
    //   block [0x832AC7D0..0x832AC7DC)
	// 832AC7D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC7D4: 386BB01C  addi r3, r11, -0x4fe4
	ctx.r[3].s64 = ctx.r[11].s64 + -20452;
	// 832AC7D8: 4AF68600  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC7E0 size=12
    let mut pc: u32 = 0x832AC7E0;
    'dispatch: loop {
        match pc {
            0x832AC7E0 => {
    //   block [0x832AC7E0..0x832AC7EC)
	// 832AC7E0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC7E4: 386BB020  addi r3, r11, -0x4fe0
	ctx.r[3].s64 = ctx.r[11].s64 + -20448;
	// 832AC7E8: 4AF685F0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC7F0 size=12
    let mut pc: u32 = 0x832AC7F0;
    'dispatch: loop {
        match pc {
            0x832AC7F0 => {
    //   block [0x832AC7F0..0x832AC7FC)
	// 832AC7F0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC7F4: 386BB024  addi r3, r11, -0x4fdc
	ctx.r[3].s64 = ctx.r[11].s64 + -20444;
	// 832AC7F8: 4AF685E0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC800 size=12
    let mut pc: u32 = 0x832AC800;
    'dispatch: loop {
        match pc {
            0x832AC800 => {
    //   block [0x832AC800..0x832AC80C)
	// 832AC800: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC804: 386BB028  addi r3, r11, -0x4fd8
	ctx.r[3].s64 = ctx.r[11].s64 + -20440;
	// 832AC808: 4AF685D0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC810 size=12
    let mut pc: u32 = 0x832AC810;
    'dispatch: loop {
        match pc {
            0x832AC810 => {
    //   block [0x832AC810..0x832AC81C)
	// 832AC810: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC814: 386BB02C  addi r3, r11, -0x4fd4
	ctx.r[3].s64 = ctx.r[11].s64 + -20436;
	// 832AC818: 4AF685C0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC820 size=12
    let mut pc: u32 = 0x832AC820;
    'dispatch: loop {
        match pc {
            0x832AC820 => {
    //   block [0x832AC820..0x832AC82C)
	// 832AC820: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC824: 386BB030  addi r3, r11, -0x4fd0
	ctx.r[3].s64 = ctx.r[11].s64 + -20432;
	// 832AC828: 4AF685B0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC830 size=12
    let mut pc: u32 = 0x832AC830;
    'dispatch: loop {
        match pc {
            0x832AC830 => {
    //   block [0x832AC830..0x832AC83C)
	// 832AC830: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC834: 386BB088  addi r3, r11, -0x4f78
	ctx.r[3].s64 = ctx.r[11].s64 + -20344;
	// 832AC838: 4AF685A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC840 size=12
    let mut pc: u32 = 0x832AC840;
    'dispatch: loop {
        match pc {
            0x832AC840 => {
    //   block [0x832AC840..0x832AC84C)
	// 832AC840: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC844: 386BB0E0  addi r3, r11, -0x4f20
	ctx.r[3].s64 = ctx.r[11].s64 + -20256;
	// 832AC848: 4AF68590  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC850 size=12
    let mut pc: u32 = 0x832AC850;
    'dispatch: loop {
        match pc {
            0x832AC850 => {
    //   block [0x832AC850..0x832AC85C)
	// 832AC850: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC854: 386BB0E4  addi r3, r11, -0x4f1c
	ctx.r[3].s64 = ctx.r[11].s64 + -20252;
	// 832AC858: 4AF68580  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC860 size=12
    let mut pc: u32 = 0x832AC860;
    'dispatch: loop {
        match pc {
            0x832AC860 => {
    //   block [0x832AC860..0x832AC86C)
	// 832AC860: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC864: 386BB0E8  addi r3, r11, -0x4f18
	ctx.r[3].s64 = ctx.r[11].s64 + -20248;
	// 832AC868: 4AF68570  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AC870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AC870 size=12
    let mut pc: u32 = 0x832AC870;
    'dispatch: loop {
        match pc {
            0x832AC870 => {
    //   block [0x832AC870..0x832AC87C)
	// 832AC870: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AC874: 386BB0EC  addi r3, r11, -0x4f14
	ctx.r[3].s64 = ctx.r[11].s64 + -20244;
	// 832AC878: 4AF68560  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


