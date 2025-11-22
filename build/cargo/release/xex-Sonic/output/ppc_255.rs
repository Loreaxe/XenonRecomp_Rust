pub fn sub_8309C37C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309C37C size=44
    let mut pc: u32 = 0x8309C37C;
    'dispatch: loop {
        match pc {
            0x8309C37C => {
    //   block [0x8309C37C..0x8309C3A8)
	// 8309C37C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8309C380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309C384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309C388: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309C38C: 809F00AC  lwz r4, 0xac(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 8309C390: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8309C394: 4BF3BF4D  bl 0x82fd82e0
	ctx.lr = 0x8309C398;
	sub_82FD82E0(ctx, base);
	// 8309C398: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309C39C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309C3A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309C3A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309C3A8 size=168
    let mut pc: u32 = 0x8309C3A8;
    'dispatch: loop {
        match pc {
            0x8309C3A8 => {
    //   block [0x8309C3A8..0x8309C450)
	// 8309C3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309C3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309C3B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309C3B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309C3B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309C3BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309C3C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309C3C4: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 8309C3C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C3CC: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309C3D0: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309C3D4: 4BFCC82D  bl 0x83068c00
	ctx.lr = 0x8309C3D8;
	sub_83068C00(ctx, base);
	// 8309C3D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309C3DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8309C3E0: 41820008  beq 0x8309c3e8
	if ctx.cr[0].eq {
	pc = 0x8309C3E8; continue 'dispatch;
	}
	// 8309C3E4: 80A30000  lwz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C3E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8309C3EC: 80DF000C  lwz r6, 0xc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309C3F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309C3F4: 4BFF8F1D  bl 0x83095310
	ctx.lr = 0x8309C3F8;
	sub_83095310(ctx, base);
	// 8309C3F8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8309C3FC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8309C400: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8309C404: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309C408: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309C40C: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 8309C410: 4BF9C1D9  bl 0x830385e8
	ctx.lr = 0x8309C414;
	sub_830385E8(ctx, base);
	// 8309C414: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8309C418: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309C41C: 4BFB7DC5  bl 0x830541e0
	ctx.lr = 0x8309C420;
	sub_830541E0(ctx, base);
	// 8309C420: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C424: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309C428: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309C42C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309C430: 4E800421  bctrl
	ctx.lr = 0x8309C434;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309C434: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309C438: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309C43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309C440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309C444: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309C448: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309C44C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309C450 size=148
    let mut pc: u32 = 0x8309C450;
    'dispatch: loop {
        match pc {
            0x8309C450 => {
    //   block [0x8309C450..0x8309C4E4)
	// 8309C450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309C454: 4810BD11  bl 0x831a8164
	ctx.lr = 0x8309C458;
	sub_831A8130(ctx, base);
	// 8309C458: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309C45C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8309C460: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8309C464: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8309C468: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309C46C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309C470: 4182000C  beq 0x8309c47c
	if ctx.cr[0].eq {
	pc = 0x8309C47C; continue 'dispatch;
	}
	// 8309C474: 83CB0008  lwz r30, 8(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309C478: 48000008  b 0x8309c480
	pc = 0x8309C480; continue 'dispatch;
	// 8309C47C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8309C480: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8309C484: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8309C488: 419A0054  beq cr6, 0x8309c4dc
	if ctx.cr[6].eq {
	pc = 0x8309C4DC; continue 'dispatch;
	}
	// 8309C48C: 807B0010  lwz r3, 0x10(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309C490: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309C494: 41820010  beq 0x8309c4a4
	if ctx.cr[0].eq {
	pc = 0x8309C4A4; continue 'dispatch;
	}
	// 8309C498: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8309C49C: 4BF903D5  bl 0x8302c870
	ctx.lr = 0x8309C4A0;
	sub_8302C870(ctx, base);
	// 8309C4A0: 48000008  b 0x8309c4a8
	pc = 0x8309C4A8; continue 'dispatch;
	// 8309C4A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309C4A8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C4AC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8309C4B0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8309C4B4: 80830008  lwz r4, 8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309C4B8: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309C4BC: 4BFCC745  bl 0x83068c00
	ctx.lr = 0x8309C4C0;
	sub_83068C00(ctx, base);
	// 8309C4C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309C4C4: 41820008  beq 0x8309c4cc
	if ctx.cr[0].eq {
	pc = 0x8309C4CC; continue 'dispatch;
	}
	// 8309C4C8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C4CC: 480010E5  bl 0x8309d5b0
	ctx.lr = 0x8309C4D0;
	sub_8309D5B0(ctx, base);
	// 8309C4D0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8309C4D4: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8309C4D8: 4198FFB4  blt cr6, 0x8309c48c
	if ctx.cr[6].lt {
	pc = 0x8309C48C; continue 'dispatch;
	}
	// 8309C4DC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8309C4E0: 4810BCD4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309C4E8 size=60
    let mut pc: u32 = 0x8309C4E8;
    'dispatch: loop {
        match pc {
            0x8309C4E8 => {
    //   block [0x8309C4E8..0x8309C524)
	// 8309C4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309C4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309C4F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309C4F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C4F8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8309C4FC: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309C500: 4BFCC701  bl 0x83068c00
	ctx.lr = 0x8309C504;
	sub_83068C00(ctx, base);
	// 8309C504: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309C508: 41820008  beq 0x8309c510
	if ctx.cr[0].eq {
	pc = 0x8309C510; continue 'dispatch;
	}
	// 8309C50C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C510: 48000451  bl 0x8309c960
	ctx.lr = 0x8309C514;
	sub_8309C960(ctx, base);
	// 8309C514: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309C518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309C51C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309C520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309C528 size=64
    let mut pc: u32 = 0x8309C528;
    'dispatch: loop {
        match pc {
            0x8309C528 => {
    //   block [0x8309C528..0x8309C568)
	// 8309C528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309C52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309C530: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309C534: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309C538: 83E30008  lwz r31, 8(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309C53C: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309C540: 41820014  beq 0x8309c554
	if ctx.cr[0].eq {
	pc = 0x8309C554; continue 'dispatch;
	}
	// 8309C544: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309C548: 4BF9C039  bl 0x83038580
	ctx.lr = 0x8309C54C;
	sub_83038580(ctx, base);
	// 8309C54C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309C550: 4BF3BD91  bl 0x82fd82e0
	ctx.lr = 0x8309C554;
	sub_82FD82E0(ctx, base);
	// 8309C554: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309C558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309C55C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309C560: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309C564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309C568 size=112
    let mut pc: u32 = 0x8309C568;
    'dispatch: loop {
        match pc {
            0x8309C568 => {
    //   block [0x8309C568..0x8309C5D8)
	// 8309C568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309C56C: 4810BBFD  bl 0x831a8168
	ctx.lr = 0x8309C570;
	sub_831A8130(ctx, base);
	// 8309C570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309C574: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309C578: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8309C57C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8309C580: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8309C584: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8309C588: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8309C58C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8309C590: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8309C594: 9BDF0000  stb r30, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 8309C598: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8309C59C: 48012C95  bl 0x830af230
	ctx.lr = 0x8309C5A0;
	sub_830AF230(ctx, base);
	// 8309C5A0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8309C5A4: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 8309C5A8: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 8309C5AC: 93BF0024  stw r29, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[29].u32 ) };
	// 8309C5B0: 939F0028  stw r28, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[28].u32 ) };
	// 8309C5B4: 419A0014  beq cr6, 0x8309c5c8
	if ctx.cr[6].eq {
	pc = 0x8309C5C8; continue 'dispatch;
	}
	// 8309C5B8: 897D0010  lbz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309C5BC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309C5C0: 41820008  beq 0x8309c5c8
	if ctx.cr[0].eq {
	pc = 0x8309C5C8; continue 'dispatch;
	}
	// 8309C5C4: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8309C5C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309C5CC: 9BDF0000  stb r30, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 8309C5D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8309C5D4: 4810BBE4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309C5D8 size=8
    let mut pc: u32 = 0x8309C5D8;
    'dispatch: loop {
        match pc {
            0x8309C5D8 => {
    //   block [0x8309C5D8..0x8309C5E0)
	// 8309C5D8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309C5DC: 8216E278  lwz r16, -0x1d88(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-7560 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309C5E0 size=96
    let mut pc: u32 = 0x8309C5E0;
    'dispatch: loop {
        match pc {
            0x8309C5E0 => {
    //   block [0x8309C5E0..0x8309C640)
	// 8309C5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309C5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309C5E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309C5EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309C5F0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8309C5F4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309C5F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309C5FC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8309C600: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8309C604: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309C608: 41820018  beq 0x8309c620
	if ctx.cr[0].eq {
	pc = 0x8309C620; continue 'dispatch;
	}
	// 8309C60C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C610: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8309C614: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C618: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309C61C: 4E800421  bctrl
	ctx.lr = 0x8309C620;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309C620: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 8309C624: 48012CED  bl 0x830af310
	ctx.lr = 0x8309C628;
	sub_830AF310(ctx, base);
	// 8309C628: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8309C62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309C630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309C634: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309C638: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309C63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309C640 size=44
    let mut pc: u32 = 0x8309C640;
    'dispatch: loop {
        match pc {
            0x8309C640 => {
    //   block [0x8309C640..0x8309C66C)
	// 8309C640: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8309C644: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309C648: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309C64C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309C650: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8309C654: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 8309C658: 48012CB9  bl 0x830af310
	ctx.lr = 0x8309C65C;
	sub_830AF310(ctx, base);
	// 8309C65C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309C660: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309C664: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309C668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309C670 size=340
    let mut pc: u32 = 0x8309C670;
    'dispatch: loop {
        match pc {
            0x8309C670 => {
    //   block [0x8309C670..0x8309C7C4)
	// 8309C670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309C674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309C678: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309C67C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309C680: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 8309C684: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309C688: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 8309C68C: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 8309C690: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 8309C694: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8309C698: 419A010C  beq cr6, 0x8309c7a4
	if ctx.cr[6].eq {
	pc = 0x8309C7A4; continue 'dispatch;
	}
	// 8309C69C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8309C6A0: 419A0104  beq cr6, 0x8309c7a4
	if ctx.cr[6].eq {
	pc = 0x8309C7A4; continue 'dispatch;
	}
	// 8309C6A4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8309C6A8: 419A0034  beq cr6, 0x8309c6dc
	if ctx.cr[6].eq {
	pc = 0x8309C6DC; continue 'dispatch;
	}
	// 8309C6AC: A1690000  lhz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C6B0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309C6B4: 41820028  beq 0x8309c6dc
	if ctx.cr[0].eq {
	pc = 0x8309C6DC; continue 'dispatch;
	}
	// 8309C6B8: 39690002  addi r11, r9, 2
	ctx.r[11].s64 = ctx.r[9].s64 + 2;
	// 8309C6BC: 48000008  b 0x8309c6c4
	pc = 0x8309C6C4; continue 'dispatch;
	// 8309C6C0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8309C6C4: A10B0000  lhz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C6C8: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309C6CC: 4082FFF4  bne 0x8309c6c0
	if !ctx.cr[0].eq {
	pc = 0x8309C6C0; continue 'dispatch;
	}
	// 8309C6D0: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8309C6D4: 7D680E70  srawi r8, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8309C6D8: 48000008  b 0x8309c6e0
	pc = 0x8309C6E0; continue 'dispatch;
	// 8309C6DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8309C6E0: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8309C6E4: 419A0034  beq cr6, 0x8309c718
	if ctx.cr[6].eq {
	pc = 0x8309C718; continue 'dispatch;
	}
	// 8309C6E8: A1650000  lhz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C6EC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309C6F0: 41820028  beq 0x8309c718
	if ctx.cr[0].eq {
	pc = 0x8309C718; continue 'dispatch;
	}
	// 8309C6F4: 39650002  addi r11, r5, 2
	ctx.r[11].s64 = ctx.r[5].s64 + 2;
	// 8309C6F8: 48000008  b 0x8309c700
	pc = 0x8309C700; continue 'dispatch;
	// 8309C6FC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8309C700: A0EB0000  lhz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C704: 28070000  cmplwi r7, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309C708: 4082FFF4  bne 0x8309c6fc
	if !ctx.cr[0].eq {
	pc = 0x8309C6FC; continue 'dispatch;
	}
	// 8309C70C: 7D655850  subf r11, r5, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 8309C710: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8309C714: 48000008  b 0x8309c71c
	pc = 0x8309C71C; continue 'dispatch;
	// 8309C718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309C71C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8309C720: 409A001C  bne cr6, 0x8309c73c
	if !ctx.cr[6].eq {
	pc = 0x8309C73C; continue 'dispatch;
	}
	// 8309C724: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309C728: 409A0074  bne cr6, 0x8309c79c
	if !ctx.cr[6].eq {
	pc = 0x8309C79C; continue 'dispatch;
	}
	// 8309C72C: 7D6A1850  subf r11, r10, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[10].s64;
	// 8309C730: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8309C734: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8309C738: 48000078  b 0x8309c7b0
	pc = 0x8309C7B0; continue 'dispatch;
	// 8309C73C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309C740: 419A005C  beq cr6, 0x8309c79c
	if ctx.cr[6].eq {
	pc = 0x8309C79C; continue 'dispatch;
	}
	// 8309C744: 7F0A1840  cmplw cr6, r10, r3
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[3].u32, &mut ctx.xer);
	// 8309C748: 409A0030  bne cr6, 0x8309c778
	if !ctx.cr[6].eq {
	pc = 0x8309C778; continue 'dispatch;
	}
	// 8309C74C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C750: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 8309C754: 80DF0028  lwz r6, 0x28(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8309C758: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 8309C75C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8309C760: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309C764: 4E800421  bctrl
	ctx.lr = 0x8309C768;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309C768: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 8309C76C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8309C770: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8309C774: 4800003C  b 0x8309c7b0
	pc = 0x8309C7B0; continue 'dispatch;
	// 8309C778: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8309C77C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309C780: 419A000C  beq cr6, 0x8309c78c
	if ctx.cr[6].eq {
	pc = 0x8309C78C; continue 'dispatch;
	}
	// 8309C784: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 8309C788: 409A000C  bne cr6, 0x8309c794
	if !ctx.cr[6].eq {
	pc = 0x8309C794; continue 'dispatch;
	}
	// 8309C78C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8309C790: 4BFFFFEC  b 0x8309c77c
	pc = 0x8309C77C; continue 'dispatch;
	// 8309C794: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C798: 4BFFFFBC  b 0x8309c754
	pc = 0x8309C754; continue 'dispatch;
	// 8309C79C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309C7A0: 48000010  b 0x8309c7b0
	pc = 0x8309C7B0; continue 'dispatch;
	// 8309C7A4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8309C7A8: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 8309C7AC: 4BF37495  bl 0x82fd3c40
	ctx.lr = 0x8309C7B0;
	sub_82FD3C40(ctx, base);
	// 8309C7B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309C7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309C7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309C7BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309C7C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309C7C8 size=128
    let mut pc: u32 = 0x8309C7C8;
    'dispatch: loop {
        match pc {
            0x8309C7C8 => {
    //   block [0x8309C7C8..0x8309C848)
	// 8309C7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309C7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309C7D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309C7D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309C7D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309C7DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309C7E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8309C7E4: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C7E8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309C7EC: 41820044  beq 0x8309c830
	if ctx.cr[0].eq {
	pc = 0x8309C830; continue 'dispatch;
	}
	// 8309C7F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C7F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309C7F8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309C7FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309C800: 4E800421  bctrl
	ctx.lr = 0x8309C804;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309C804: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 8309C808: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8309C80C: 409A0024  bne cr6, 0x8309c830
	if !ctx.cr[6].eq {
	pc = 0x8309C830; continue 'dispatch;
	}
	// 8309C810: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 8309C814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8309C818: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8309C81C: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309C820: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8309C824: 38800067  li r4, 0x67
	ctx.r[4].s64 = 103;
	// 8309C828: 806B00A8  lwz r3, 0xa8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 8309C82C: 4BF7AD2D  bl 0x83017558
	ctx.lr = 0x8309C830;
	sub_83017558(ctx, base);
	// 8309C830: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309C834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309C838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309C83C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309C840: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309C844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309C848 size=132
    let mut pc: u32 = 0x8309C848;
    'dispatch: loop {
        match pc {
            0x8309C848 => {
    //   block [0x8309C848..0x8309C8CC)
	// 8309C848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309C84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309C850: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309C854: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309C858: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309C85C: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C860: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309C864: 41820054  beq 0x8309c8b8
	if ctx.cr[0].eq {
	pc = 0x8309C8B8; continue 'dispatch;
	}
	// 8309C868: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309C86C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C870: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309C874: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309C878: 4E800421  bctrl
	ctx.lr = 0x8309C87C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309C87C: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 8309C880: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8309C884: 41980010  blt cr6, 0x8309c894
	if ctx.cr[6].lt {
	pc = 0x8309C894; continue 'dispatch;
	}
	// 8309C888: 409A0030  bne cr6, 0x8309c8b8
	if !ctx.cr[6].eq {
	pc = 0x8309C8B8; continue 'dispatch;
	}
	// 8309C88C: 38800069  li r4, 0x69
	ctx.r[4].s64 = 105;
	// 8309C890: 48000008  b 0x8309c898
	pc = 0x8309C898; continue 'dispatch;
	// 8309C894: 38800068  li r4, 0x68
	ctx.r[4].s64 = 104;
	// 8309C898: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309C89C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8309C8A0: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8309C8A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8309C8A8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8309C8AC: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309C8B0: 806A00A8  lwz r3, 0xa8(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(168 as u32) ) } as u64;
	// 8309C8B4: 4BF7ACA5  bl 0x83017558
	ctx.lr = 0x8309C8B8;
	sub_83017558(ctx, base);
	// 8309C8B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309C8BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309C8C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309C8C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309C8C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309C8D0 size=16
    let mut pc: u32 = 0x8309C8D0;
    'dispatch: loop {
        match pc {
            0x8309C8D0 => {
    //   block [0x8309C8D0..0x8309C8E0)
	// 8309C8D0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309C8D4: 396BE2B0  addi r11, r11, -0x1d50
	ctx.r[11].s64 = ctx.r[11].s64 + -7504;
	// 8309C8D8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309C8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309C8E0 size=128
    let mut pc: u32 = 0x8309C8E0;
    'dispatch: loop {
        match pc {
            0x8309C8E0 => {
    //   block [0x8309C8E0..0x8309C960)
	// 8309C8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309C8E4: 4810B889  bl 0x831a816c
	ctx.lr = 0x8309C8E8;
	sub_831A8130(ctx, base);
	// 8309C8E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309C8EC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309C8F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309C8F4: 396BE2B0  addi r11, r11, -0x1d50
	ctx.r[11].s64 = ctx.r[11].s64 + -7504;
	// 8309C8F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309C8FC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8309C900: 57C4103A  slwi r4, r30, 2
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8309C904: 98BF0004  stb r5, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 8309C908: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 8309C90C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309C910: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8309C914: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8309C918: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 8309C91C: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 8309C920: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C924: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309C928: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309C92C: 4E800421  bctrl
	ctx.lr = 0x8309C930;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309C930: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8309C934: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8309C938: 419A001C  beq cr6, 0x8309c954
	if ctx.cr[6].eq {
	pc = 0x8309C954; continue 'dispatch;
	}
	// 8309C93C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8309C940: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309C944: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8309C948: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 8309C94C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8309C950: 4082FFF0  bne 0x8309c940
	if !ctx.cr[0].eq {
	pc = 0x8309C940; continue 'dispatch;
	}
	// 8309C954: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309C958: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309C95C: 4810B860  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309C960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309C960 size=256
    let mut pc: u32 = 0x8309C960;
    'dispatch: loop {
        match pc {
            0x8309C960 => {
    //   block [0x8309C960..0x8309CA60)
	// 8309C960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309C964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309C968: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309C96C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309C970: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309C974: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309C978: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8309C97C: 40820038  bne 0x8309c9b4
	if !ctx.cr[0].eq {
	pc = 0x8309C9B4; continue 'dispatch;
	}
	// 8309C980: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309C984: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C988: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309C98C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309C990: 4E800421  bctrl
	ctx.lr = 0x8309C994;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309C994: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 8309C998: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8309C99C: 409A00B0  bne cr6, 0x8309ca4c
	if !ctx.cr[6].eq {
	pc = 0x8309CA4C; continue 'dispatch;
	}
	// 8309C9A0: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C9A4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309C9A8: 418200A4  beq 0x8309ca4c
	if ctx.cr[0].eq {
	pc = 0x8309CA4C; continue 'dispatch;
	}
	// 8309C9AC: 38800063  li r4, 0x63
	ctx.r[4].s64 = 99;
	// 8309C9B0: 4800007C  b 0x8309ca2c
	pc = 0x8309CA2C; continue 'dispatch;
	// 8309C9B4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309C9B8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309C9BC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309C9C0: 4182000C  beq 0x8309c9cc
	if ctx.cr[0].eq {
	pc = 0x8309C9CC; continue 'dispatch;
	}
	// 8309C9C4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309C9C8: 48000008  b 0x8309c9d0
	pc = 0x8309C9D0; continue 'dispatch;
	// 8309C9CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309C9D0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8309C9D4: 419A0078  beq cr6, 0x8309ca4c
	if ctx.cr[6].eq {
	pc = 0x8309CA4C; continue 'dispatch;
	}
	// 8309C9D8: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C9DC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309C9E0: 4182006C  beq 0x8309ca4c
	if ctx.cr[0].eq {
	pc = 0x8309CA4C; continue 'dispatch;
	}
	// 8309C9E4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309C9E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309C9EC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309C9F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309C9F4: 4E800421  bctrl
	ctx.lr = 0x8309C9F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309C9F8: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 8309C9FC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8309CA00: 41980028  blt cr6, 0x8309ca28
	if ctx.cr[6].lt {
	pc = 0x8309CA28; continue 'dispatch;
	}
	// 8309CA04: 419A001C  beq cr6, 0x8309ca20
	if ctx.cr[6].eq {
	pc = 0x8309CA20; continue 'dispatch;
	}
	// 8309CA08: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 8309CA0C: 40980040  bge cr6, 0x8309ca4c
	if !ctx.cr[6].lt {
	pc = 0x8309CA4C; continue 'dispatch;
	}
	// 8309CA10: 38800066  li r4, 0x66
	ctx.r[4].s64 = 102;
	// 8309CA14: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309CA18: 80CB0004  lwz r6, 4(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309CA1C: 48000018  b 0x8309ca34
	pc = 0x8309CA34; continue 'dispatch;
	// 8309CA20: 38800065  li r4, 0x65
	ctx.r[4].s64 = 101;
	// 8309CA24: 4BFFFFF0  b 0x8309ca14
	pc = 0x8309CA14; continue 'dispatch;
	// 8309CA28: 38800064  li r4, 0x64
	ctx.r[4].s64 = 100;
	// 8309CA2C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309CA30: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8309CA34: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8309CA38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8309CA3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8309CA40: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309CA44: 806A00A8  lwz r3, 0xa8(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(168 as u32) ) } as u64;
	// 8309CA48: 4BF7AB11  bl 0x83017558
	ctx.lr = 0x8309CA4C;
	sub_83017558(ctx, base);
	// 8309CA4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309CA50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309CA54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309CA58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309CA5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309CA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309CA60 size=352
    let mut pc: u32 = 0x8309CA60;
    'dispatch: loop {
        match pc {
            0x8309CA60 => {
    //   block [0x8309CA60..0x8309CBC0)
	// 8309CA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309CA64: 4810B6E9  bl 0x831a814c
	ctx.lr = 0x8309CA68;
	sub_831A8130(ctx, base);
	// 8309CA68: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309CA6C: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 8309CA70: 7C952378  mr r21, r4
	ctx.r[21].u64 = ctx.r[4].u64;
	// 8309CA74: 8157001C  lwz r10, 0x1c(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(28 as u32) ) } as u64;
	// 8309CA78: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309CA7C: 41820130  beq 0x8309cbac
	if ctx.cr[0].eq {
	pc = 0x8309CBAC; continue 'dispatch;
	}
	// 8309CA80: 81750000  lwz r11, 0(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309CA84: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309CA88: 4182000C  beq 0x8309ca94
	if ctx.cr[0].eq {
	pc = 0x8309CA94; continue 'dispatch;
	}
	// 8309CA8C: 830B0004  lwz r24, 4(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309CA90: 48000008  b 0x8309ca98
	pc = 0x8309CA98; continue 'dispatch;
	// 8309CA94: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 8309CA98: 82CA0008  lwz r22, 8(r10)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309CA9C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8309CAA0: 28160000  cmplwi r22, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309CAA4: 41820108  beq 0x8309cbac
	if ctx.cr[0].eq {
	pc = 0x8309CBAC; continue 'dispatch;
	}
	// 8309CAA8: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8309CAAC: 8077001C  lwz r3, 0x1c(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(28 as u32) ) } as u64;
	// 8309CAB0: 4BF8FDC1  bl 0x8302c870
	ctx.lr = 0x8309CAB4;
	sub_8302C870(ctx, base);
	// 8309CAB4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8309CAB8: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309CABC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309CAC0: 4182000C  beq 0x8309cacc
	if ctx.cr[0].eq {
	pc = 0x8309CACC; continue 'dispatch;
	}
	// 8309CAC4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309CAC8: 48000008  b 0x8309cad0
	pc = 0x8309CAD0; continue 'dispatch;
	// 8309CACC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309CAD0: 7F185840  cmplw cr6, r24, r11
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309CAD4: 409A00CC  bne cr6, 0x8309cba0
	if !ctx.cr[6].eq {
	pc = 0x8309CBA0; continue 'dispatch;
	}
	// 8309CAD8: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 8309CADC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8309CAE0: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 8309CAE4: 419A00B4  beq cr6, 0x8309cb98
	if ctx.cr[6].eq {
	pc = 0x8309CB98; continue 'dispatch;
	}
	// 8309CAE8: 80750008  lwz r3, 8(r21)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309CAEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309CAF0: 41820014  beq 0x8309cb04
	if ctx.cr[0].eq {
	pc = 0x8309CB04; continue 'dispatch;
	}
	// 8309CAF4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8309CAF8: 4BF8FD79  bl 0x8302c870
	ctx.lr = 0x8309CAFC;
	sub_8302C870(ctx, base);
	// 8309CAFC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8309CB00: 48000008  b 0x8309cb08
	pc = 0x8309CB08; continue 'dispatch;
	// 8309CB04: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8309CB08: 80750004  lwz r3, 4(r21)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309CB0C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309CB10: 41820014  beq 0x8309cb24
	if ctx.cr[0].eq {
	pc = 0x8309CB24; continue 'dispatch;
	}
	// 8309CB14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8309CB18: 4BF5B5B9  bl 0x82ff80d0
	ctx.lr = 0x8309CB1C;
	sub_82FF80D0(ctx, base);
	// 8309CB1C: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309CB20: 48000008  b 0x8309cb28
	pc = 0x8309CB28; continue 'dispatch;
	// 8309CB24: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8309CB28: 807A0008  lwz r3, 8(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309CB2C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309CB30: 41820014  beq 0x8309cb44
	if ctx.cr[0].eq {
	pc = 0x8309CB44; continue 'dispatch;
	}
	// 8309CB34: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8309CB38: 4BF8FD39  bl 0x8302c870
	ctx.lr = 0x8309CB3C;
	sub_8302C870(ctx, base);
	// 8309CB3C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309CB40: 48000008  b 0x8309cb48
	pc = 0x8309CB48; continue 'dispatch;
	// 8309CB44: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8309CB48: 807A0004  lwz r3, 4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309CB4C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309CB50: 41820014  beq 0x8309cb64
	if ctx.cr[0].eq {
	pc = 0x8309CB64; continue 'dispatch;
	}
	// 8309CB54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8309CB58: 4BF5B579  bl 0x82ff80d0
	ctx.lr = 0x8309CB5C;
	sub_82FF80D0(ctx, base);
	// 8309CB5C: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309CB60: 48000008  b 0x8309cb68
	pc = 0x8309CB68; continue 'dispatch;
	// 8309CB64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8309CB68: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 8309CB6C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8309CB70: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8309CB74: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 8309CB78: 4BFFFAF9  bl 0x8309c670
	ctx.lr = 0x8309CB7C;
	sub_8309C670(ctx, base);
	// 8309CB7C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309CB80: 41820014  beq 0x8309cb94
	if ctx.cr[0].eq {
	pc = 0x8309CB94; continue 'dispatch;
	}
	// 8309CB84: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8309CB88: 7F1FC040  cmplw cr6, r31, r24
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[24].u32, &mut ctx.xer);
	// 8309CB8C: 4198FF5C  blt cr6, 0x8309cae8
	if ctx.cr[6].lt {
	pc = 0x8309CAE8; continue 'dispatch;
	}
	// 8309CB90: 48000008  b 0x8309cb98
	pc = 0x8309CB98; continue 'dispatch;
	// 8309CB94: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8309CB98: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309CB9C: 4082001C  bne 0x8309cbb8
	if !ctx.cr[0].eq {
	pc = 0x8309CBB8; continue 'dispatch;
	}
	// 8309CBA0: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 8309CBA4: 7F19B040  cmplw cr6, r25, r22
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[22].u32, &mut ctx.xer);
	// 8309CBA8: 4198FF00  blt cr6, 0x8309caa8
	if ctx.cr[6].lt {
	pc = 0x8309CAA8; continue 'dispatch;
	}
	// 8309CBAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309CBB0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8309CBB4: 4810B5E8  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
	// 8309CBB8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8309CBBC: 4BFFFFF4  b 0x8309cbb0
	pc = 0x8309CBB0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309CBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309CBC0 size=8
    let mut pc: u32 = 0x8309CBC0;
    'dispatch: loop {
        match pc {
            0x8309CBC0 => {
    //   block [0x8309CBC0..0x8309CBC8)
	// 8309CBC0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309CBC4: 8216E2E8  lwz r16, -0x1d18(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-7448 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309CBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309CBC8 size=164
    let mut pc: u32 = 0x8309CBC8;
    'dispatch: loop {
        match pc {
            0x8309CBC8 => {
    //   block [0x8309CBC8..0x8309CC6C)
	// 8309CBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309CBCC: 4810B599  bl 0x831a8164
	ctx.lr = 0x8309CBD0;
	sub_831A8130(ctx, base);
	// 8309CBD0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8309CBD4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309CBD8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309CBDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309CBE0: 396BE2C8  addi r11, r11, -0x1d38
	ctx.r[11].s64 = ctx.r[11].s64 + -7480;
	// 8309CBE4: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8309CBE8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309CBEC: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309CBF0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309CBF4: 4182004C  beq 0x8309cc40
	if ctx.cr[0].eq {
	pc = 0x8309CC40; continue 'dispatch;
	}
	// 8309CBF8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309CBFC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8309CC00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309CC04: 4099003C  ble cr6, 0x8309cc40
	if !ctx.cr[6].gt {
	pc = 0x8309CC40; continue 'dispatch;
	}
	// 8309CC08: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8309CC0C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309CC10: 7F8BE82E  lwzx r28, r11, r29
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8309CC14: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309CC18: 41820014  beq 0x8309cc2c
	if ctx.cr[0].eq {
	pc = 0x8309CC2C; continue 'dispatch;
	}
	// 8309CC1C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309CC20: 480126F1  bl 0x830af310
	ctx.lr = 0x8309CC24;
	sub_830AF310(ctx, base);
	// 8309CC24: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309CC28: 4BF3B6B9  bl 0x82fd82e0
	ctx.lr = 0x8309CC2C;
	sub_82FD82E0(ctx, base);
	// 8309CC2C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309CC30: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 8309CC34: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8309CC38: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309CC3C: 4198FFD0  blt cr6, 0x8309cc0c
	if ctx.cr[6].lt {
	pc = 0x8309CC0C; continue 'dispatch;
	}
	// 8309CC40: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309CC44: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309CC48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309CC4C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309CC50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309CC54: 4E800421  bctrl
	ctx.lr = 0x8309CC58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309CC58: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309CC5C: 396BE2B0  addi r11, r11, -0x1d50
	ctx.r[11].s64 = ctx.r[11].s64 + -7504;
	// 8309CC60: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309CC64: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8309CC68: 4810B54C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309CC6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309CC6C size=40
    let mut pc: u32 = 0x8309CC6C;
    'dispatch: loop {
        match pc {
            0x8309CC6C => {
    //   block [0x8309CC6C..0x8309CC94)
	// 8309CC6C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8309CC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309CC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309CC78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309CC7C: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8309CC80: 4BFFFC51  bl 0x8309c8d0
	ctx.lr = 0x8309CC84;
	sub_8309C8D0(ctx, base);
	// 8309CC84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309CC88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309CC8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309CC90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309CC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309CC98 size=148
    let mut pc: u32 = 0x8309CC98;
    'dispatch: loop {
        match pc {
            0x8309CC98 => {
    //   block [0x8309CC98..0x8309CD2C)
	// 8309CC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309CC9C: 4810B4CD  bl 0x831a8168
	ctx.lr = 0x8309CCA0;
	sub_831A8130(ctx, base);
	// 8309CCA0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309CCA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309CCA8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8309CCAC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8309CCB0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309CCB4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309CCB8: 41980030  blt cr6, 0x8309cce8
	if ctx.cr[6].lt {
	pc = 0x8309CCE8; continue 'dispatch;
	}
	// 8309CCBC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8309CCC0: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309CCC4: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 8309CCC8: 388B6B80  addi r4, r11, 0x6b80
	ctx.r[4].s64 = ctx.r[11].s64 + 27520;
	// 8309CCCC: 38A00043  li r5, 0x43
	ctx.r[5].s64 = 67;
	// 8309CCD0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8309CCD4: 4BF33C85  bl 0x82fd0958
	ctx.lr = 0x8309CCD8;
	sub_82FD0958(ctx, base);
	// 8309CCD8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8309CCDC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8309CCE0: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 8309CCE4: 48113F45  bl 0x831b0c28
	ctx.lr = 0x8309CCE8;
	sub_831B0C28(ctx, base);
	// 8309CCE8: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309CCEC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309CCF0: 41820028  beq 0x8309cd18
	if ctx.cr[0].eq {
	pc = 0x8309CD18; continue 'dispatch;
	}
	// 8309CCF4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309CCF8: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8309CCFC: 7FAA582E  lwzx r29, r10, r11
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8309CD00: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309CD04: 41820014  beq 0x8309cd18
	if ctx.cr[0].eq {
	pc = 0x8309CD18; continue 'dispatch;
	}
	// 8309CD08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309CD0C: 48012605  bl 0x830af310
	ctx.lr = 0x8309CD10;
	sub_830AF310(ctx, base);
	// 8309CD10: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309CD14: 4BF3B5CD  bl 0x82fd82e0
	ctx.lr = 0x8309CD18;
	sub_82FD82E0(ctx, base);
	// 8309CD18: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309CD1C: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8309CD20: 7F8A592E  stwx r28, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[28].u32) };
	// 8309CD24: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8309CD28: 4810B490  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309CD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309CD30 size=124
    let mut pc: u32 = 0x8309CD30;
    'dispatch: loop {
        match pc {
            0x8309CD30 => {
    //   block [0x8309CD30..0x8309CDAC)
	// 8309CD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309CD34: 4810B431  bl 0x831a8164
	ctx.lr = 0x8309CD38;
	sub_831A8130(ctx, base);
	// 8309CD38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309CD3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309CD40: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8309CD44: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 8309CD48: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309CD4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309CD50: 40990050  ble cr6, 0x8309cda0
	if !ctx.cr[6].gt {
	pc = 0x8309CDA0; continue 'dispatch;
	}
	// 8309CD54: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 8309CD58: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309CD5C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309CD60: 41820024  beq 0x8309cd84
	if ctx.cr[0].eq {
	pc = 0x8309CD84; continue 'dispatch;
	}
	// 8309CD64: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309CD68: 7F8BF02E  lwzx r28, r11, r30
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8309CD6C: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309CD70: 41820014  beq 0x8309cd84
	if ctx.cr[0].eq {
	pc = 0x8309CD84; continue 'dispatch;
	}
	// 8309CD74: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309CD78: 48012599  bl 0x830af310
	ctx.lr = 0x8309CD7C;
	sub_830AF310(ctx, base);
	// 8309CD7C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309CD80: 4BF3B561  bl 0x82fd82e0
	ctx.lr = 0x8309CD84;
	sub_82FD82E0(ctx, base);
	// 8309CD84: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309CD88: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8309CD8C: 7F6BF12E  stwx r27, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[27].u32) };
	// 8309CD90: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8309CD94: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309CD98: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309CD9C: 4198FFBC  blt cr6, 0x8309cd58
	if ctx.cr[6].lt {
	pc = 0x8309CD58; continue 'dispatch;
	}
	// 8309CDA0: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 8309CDA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8309CDA8: 4810B40C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309CDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309CDB0 size=260
    let mut pc: u32 = 0x8309CDB0;
    'dispatch: loop {
        match pc {
            0x8309CDB0 => {
    //   block [0x8309CDB0..0x8309CEB4)
	// 8309CDB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309CDB4: 4810B3B9  bl 0x831a816c
	ctx.lr = 0x8309CDB8;
	sub_831A8130(ctx, base);
	// 8309CDB8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309CDBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309CDC0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309CDC4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309CDC8: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309CDCC: 41980030  blt cr6, 0x8309cdfc
	if ctx.cr[6].lt {
	pc = 0x8309CDFC; continue 'dispatch;
	}
	// 8309CDD0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8309CDD4: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309CDD8: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 8309CDDC: 388B6B80  addi r4, r11, 0x6b80
	ctx.r[4].s64 = ctx.r[11].s64 + 27520;
	// 8309CDE0: 38A00090  li r5, 0x90
	ctx.r[5].s64 = 144;
	// 8309CDE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8309CDE8: 4BF33B71  bl 0x82fd0958
	ctx.lr = 0x8309CDEC;
	sub_82FD0958(ctx, base);
	// 8309CDEC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8309CDF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8309CDF4: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 8309CDF8: 48113E31  bl 0x831b0c28
	ctx.lr = 0x8309CDFC;
	sub_831B0C28(ctx, base);
	// 8309CDFC: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309CE00: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309CE04: 41820028  beq 0x8309ce2c
	if ctx.cr[0].eq {
	pc = 0x8309CE2C; continue 'dispatch;
	}
	// 8309CE08: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309CE0C: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8309CE10: 7FAA582E  lwzx r29, r10, r11
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8309CE14: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309CE18: 41820014  beq 0x8309ce2c
	if ctx.cr[0].eq {
	pc = 0x8309CE2C; continue 'dispatch;
	}
	// 8309CE1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309CE20: 480124F1  bl 0x830af310
	ctx.lr = 0x8309CE24;
	sub_830AF310(ctx, base);
	// 8309CE24: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309CE28: 4BF3B4B9  bl 0x82fd82e0
	ctx.lr = 0x8309CE2C;
	sub_82FD82E0(ctx, base);
	// 8309CE2C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309CE30: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8309CE34: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309CE38: 409A0018  bne cr6, 0x8309ce50
	if !ctx.cr[6].eq {
	pc = 0x8309CE50; continue 'dispatch;
	}
	// 8309CE3C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309CE40: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8309CE44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8309CE48: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 8309CE4C: 48000054  b 0x8309cea0
	pc = 0x8309CEA0; continue 'dispatch;
	// 8309CE50: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8309CE54: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309CE58: 40980030  bge cr6, 0x8309ce88
	if !ctx.cr[6].lt {
	pc = 0x8309CE88; continue 'dispatch;
	}
	// 8309CE5C: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8309CE60: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309CE64: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8309CE68: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8309CE6C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8309CE70: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309CE74: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8309CE78: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309CE7C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 8309CE80: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8309CE84: 4198FFDC  blt cr6, 0x8309ce60
	if ctx.cr[6].lt {
	pc = 0x8309CE60; continue 'dispatch;
	}
	// 8309CE88: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309CE8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8309CE90: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309CE94: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8309CE98: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8309CE9C: 912BFFFC  stw r9, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[9].u32 ) };
	// 8309CEA0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309CEA4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8309CEA8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8309CEAC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8309CEB0: 4810B30C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309CEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309CEB8 size=104
    let mut pc: u32 = 0x8309CEB8;
    'dispatch: loop {
        match pc {
            0x8309CEB8 => {
    //   block [0x8309CEB8..0x8309CF20)
	// 8309CEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309CEBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309CEC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309CEC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309CEC8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309CECC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309CED0: 4182003C  beq 0x8309cf0c
	if ctx.cr[0].eq {
	pc = 0x8309CF0C; continue 'dispatch;
	}
	// 8309CED4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8309CED8: 89430004  lbz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309CEDC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309CEE0: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8309CEE4: 41820028  beq 0x8309cf0c
	if ctx.cr[0].eq {
	pc = 0x8309CF0C; continue 'dispatch;
	}
	// 8309CEE8: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309CEEC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8309CEF0: 7FEB502E  lwzx r31, r11, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8309CEF4: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309CEF8: 41820014  beq 0x8309cf0c
	if ctx.cr[0].eq {
	pc = 0x8309CF0C; continue 'dispatch;
	}
	// 8309CEFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309CF00: 48012411  bl 0x830af310
	ctx.lr = 0x8309CF04;
	sub_830AF310(ctx, base);
	// 8309CF04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309CF08: 4BF3B3D9  bl 0x82fd82e0
	ctx.lr = 0x8309CF0C;
	sub_82FD82E0(ctx, base);
	// 8309CF0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309CF10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309CF14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309CF18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309CF1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309CF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309CF20 size=132
    let mut pc: u32 = 0x8309CF20;
    'dispatch: loop {
        match pc {
            0x8309CF20 => {
    //   block [0x8309CF20..0x8309CFA4)
	// 8309CF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309CF24: 4810B245  bl 0x831a8168
	ctx.lr = 0x8309CF28;
	sub_831A8130(ctx, base);
	// 8309CF28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309CF2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309CF30: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309CF34: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309CF38: 4182004C  beq 0x8309cf84
	if ctx.cr[0].eq {
	pc = 0x8309CF84; continue 'dispatch;
	}
	// 8309CF3C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309CF40: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8309CF44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309CF48: 4099003C  ble cr6, 0x8309cf84
	if !ctx.cr[6].gt {
	pc = 0x8309CF84; continue 'dispatch;
	}
	// 8309CF4C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8309CF50: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309CF54: 7FABF02E  lwzx r29, r11, r30
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8309CF58: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309CF5C: 41820014  beq 0x8309cf70
	if ctx.cr[0].eq {
	pc = 0x8309CF70; continue 'dispatch;
	}
	// 8309CF60: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309CF64: 480123AD  bl 0x830af310
	ctx.lr = 0x8309CF68;
	sub_830AF310(ctx, base);
	// 8309CF68: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309CF6C: 4BF3B375  bl 0x82fd82e0
	ctx.lr = 0x8309CF70;
	sub_82FD82E0(ctx, base);
	// 8309CF70: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309CF74: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8309CF78: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8309CF7C: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309CF80: 4198FFD0  blt cr6, 0x8309cf50
	if ctx.cr[6].lt {
	pc = 0x8309CF50; continue 'dispatch;
	}
	// 8309CF84: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309CF88: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309CF8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309CF90: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309CF94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309CF98: 4E800421  bctrl
	ctx.lr = 0x8309CF9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309CF9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8309CFA0: 4810B218  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309CFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309CFA8 size=76
    let mut pc: u32 = 0x8309CFA8;
    'dispatch: loop {
        match pc {
            0x8309CFA8 => {
    //   block [0x8309CFA8..0x8309CFF4)
	// 8309CFA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309CFAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309CFB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309CFB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309CFB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309CFBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309CFC0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309CFC4: 4BFFFC05  bl 0x8309cbc8
	ctx.lr = 0x8309CFC8;
	sub_8309CBC8(ctx, base);
	// 8309CFC8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309CFCC: 4182000C  beq 0x8309cfd8
	if ctx.cr[0].eq {
	pc = 0x8309CFD8; continue 'dispatch;
	}
	// 8309CFD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309CFD4: 4BF3B30D  bl 0x82fd82e0
	ctx.lr = 0x8309CFD8;
	sub_82FD82E0(ctx, base);
	// 8309CFD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309CFDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309CFE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309CFE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309CFE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309CFEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309CFF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309CFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309CFF8 size=8
    let mut pc: u32 = 0x8309CFF8;
    'dispatch: loop {
        match pc {
            0x8309CFF8 => {
    //   block [0x8309CFF8..0x8309D000)
	// 8309CFF8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309CFFC: 8216E330  lwz r16, -0x1cd0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-7376 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309D000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309D000 size=380
    let mut pc: u32 = 0x8309D000;
    'dispatch: loop {
        match pc {
            0x8309D000 => {
    //   block [0x8309D000..0x8309D17C)
	// 8309D000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309D004: 4810B161  bl 0x831a8164
	ctx.lr = 0x8309D008;
	sub_831A8130(ctx, base);
	// 8309D008: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8309D00C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309D010: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309D014: 90BF00B4  stw r5, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[5].u32 ) };
	// 8309D018: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8309D01C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8309D020: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309D024: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 8309D028: 939F00AC  stw r28, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[28].u32 ) };
	// 8309D02C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309D030: 409A00BC  bne cr6, 0x8309d0ec
	if !ctx.cr[6].eq {
	pc = 0x8309D0EC; continue 'dispatch;
	}
	// 8309D034: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 8309D038: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309D03C: 4BF3B25D  bl 0x82fd8298
	ctx.lr = 0x8309D040;
	sub_82FD8298(ctx, base);
	// 8309D040: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8309D044: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309D048: 4182001C  beq 0x8309d064
	if ctx.cr[0].eq {
	pc = 0x8309D064; continue 'dispatch;
	}
	// 8309D04C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8309D050: 80BE000C  lwz r5, 0xc(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309D054: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8309D058: 4BFCB749  bl 0x830687a0
	ctx.lr = 0x8309D05C;
	sub_830687A0(ctx, base);
	// 8309D05C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8309D060: 48000008  b 0x8309d068
	pc = 0x8309D068; continue 'dispatch;
	// 8309D064: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309D068: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 8309D06C: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309D070: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309D074: 4BF3B225  bl 0x82fd8298
	ctx.lr = 0x8309D078;
	sub_82FD8298(ctx, base);
	// 8309D078: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8309D07C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309D080: 4182001C  beq 0x8309d09c
	if ctx.cr[0].eq {
	pc = 0x8309D09C; continue 'dispatch;
	}
	// 8309D084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8309D088: 80BE000C  lwz r5, 0xc(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309D08C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8309D090: 4BFCB711  bl 0x830687a0
	ctx.lr = 0x8309D094;
	sub_830687A0(ctx, base);
	// 8309D094: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8309D098: 48000008  b 0x8309d0a0
	pc = 0x8309D0A0; continue 'dispatch;
	// 8309D09C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309D0A0: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8309D0A4: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309D0A8: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8309D0AC: 4BF3B1ED  bl 0x82fd8298
	ctx.lr = 0x8309D0B0;
	sub_82FD8298(ctx, base);
	// 8309D0B0: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8309D0B4: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8309D0B8: 4182002C  beq 0x8309d0e4
	if ctx.cr[0].eq {
	pc = 0x8309D0E4; continue 'dispatch;
	}
	// 8309D0BC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8309D0C0: 80DE000C  lwz r6, 0xc(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309D0C4: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8309D0C8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309D0CC: 4BF35975  bl 0x82fd2a40
	ctx.lr = 0x8309D0D0;
	sub_82FD2A40(ctx, base);
	// 8309D0D0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8309D0D4: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 8309D0D8: 396B6C68  addi r11, r11, 0x6c68
	ctx.r[11].s64 = ctx.r[11].s64 + 27752;
	// 8309D0DC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309D0E0: 48000008  b 0x8309d0e8
	pc = 0x8309D0E8; continue 'dispatch;
	// 8309D0E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8309D0E8: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8309D0EC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8309D0F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309D0F4: 4801228D  bl 0x830af380
	ctx.lr = 0x8309D0F8;
	sub_830AF380(ctx, base);
	// 8309D0F8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8309D0FC: 2F1DFFFF  cmpwi cr6, r29, -1
	ctx.cr[6].compare_i32(ctx.r[29].s32, -1, &mut ctx.xer);
	// 8309D100: 409A0038  bne cr6, 0x8309d138
	if !ctx.cr[6].eq {
	pc = 0x8309D138; continue 'dispatch;
	}
	// 8309D104: 389F00AC  addi r4, r31, 0xac
	ctx.r[4].s64 = ctx.r[31].s64 + 172;
	// 8309D108: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309D10C: 4BF49AFD  bl 0x82fe6c08
	ctx.lr = 0x8309D110;
	sub_82FE6C08(ctx, base);
	// 8309D110: 389F00B4  addi r4, r31, 0xb4
	ctx.r[4].s64 = ctx.r[31].s64 + 180;
	// 8309D114: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309D118: 4BF49AF1  bl 0x82fe6c08
	ctx.lr = 0x8309D11C;
	sub_82FE6C08(ctx, base);
	// 8309D11C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8309D120: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309D124: 4BF33A5D  bl 0x82fd0b80
	ctx.lr = 0x8309D128;
	sub_82FD0B80(ctx, base);
	// 8309D128: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8309D12C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309D130: 4BF9E021  bl 0x8303b150
	ctx.lr = 0x8309D134;
	sub_8303B150(ctx, base);
	// 8309D134: 48000040  b 0x8309d174
	pc = 0x8309D174; continue 'dispatch;
	// 8309D138: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8309D13C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309D140: 389F00B4  addi r4, r31, 0xb4
	ctx.r[4].s64 = ctx.r[31].s64 + 180;
	// 8309D144: 4BF8F6C5  bl 0x8302c808
	ctx.lr = 0x8309D148;
	sub_8302C808(ctx, base);
	// 8309D148: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309D14C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8309D150: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309D154: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309D158: 4BF33A29  bl 0x82fd0b80
	ctx.lr = 0x8309D15C;
	sub_82FD0B80(ctx, base);
	// 8309D15C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8309D160: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309D164: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309D168: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8309D16C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309D170: 4E800421  bctrl
	ctx.lr = 0x8309D174;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309D174: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8309D178: 4810B03C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309D17C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309D17C size=48
    let mut pc: u32 = 0x8309D17C;
    'dispatch: loop {
        match pc {
            0x8309D17C => {
    //   block [0x8309D17C..0x8309D1AC)
	// 8309D17C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8309D180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309D184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309D188: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309D18C: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8309D190: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309D194: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309D198: 4BF3B149  bl 0x82fd82e0
	ctx.lr = 0x8309D19C;
	sub_82FD82E0(ctx, base);
	// 8309D19C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309D1A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309D1A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309D1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309D1AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309D1AC size=48
    let mut pc: u32 = 0x8309D1AC;
    'dispatch: loop {
        match pc {
            0x8309D1AC => {
    //   block [0x8309D1AC..0x8309D1DC)
	// 8309D1AC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8309D1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309D1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309D1B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309D1BC: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8309D1C0: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309D1C4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309D1C8: 4BF3B119  bl 0x82fd82e0
	ctx.lr = 0x8309D1CC;
	sub_82FD82E0(ctx, base);
	// 8309D1CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309D1D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309D1D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309D1D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309D1DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309D1DC size=48
    let mut pc: u32 = 0x8309D1DC;
    'dispatch: loop {
        match pc {
            0x8309D1DC => {
    //   block [0x8309D1DC..0x8309D20C)
	// 8309D1DC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8309D1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309D1E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309D1E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309D1EC: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8309D1F0: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309D1F4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309D1F8: 4BF3B0E9  bl 0x82fd82e0
	ctx.lr = 0x8309D1FC;
	sub_82FD82E0(ctx, base);
	// 8309D1FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309D200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309D204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309D208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309D210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309D210 size=8
    let mut pc: u32 = 0x8309D210;
    'dispatch: loop {
        match pc {
            0x8309D210 => {
    //   block [0x8309D210..0x8309D218)
	// 8309D210: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309D214: 8216E3A0  lwz r16, -0x1c60(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-7264 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309D218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309D218 size=472
    let mut pc: u32 = 0x8309D218;
    'dispatch: loop {
        match pc {
            0x8309D218 => {
    //   block [0x8309D218..0x8309D3F0)
	// 8309D218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309D21C: 4810AF41  bl 0x831a815c
	ctx.lr = 0x8309D220;
	sub_831A8130(ctx, base);
	// 8309D220: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8309D224: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309D228: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8309D22C: 80640008  lwz r3, 8(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309D230: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8309D234: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8309D238: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8309D23C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8309D240: 93BF00B4  stw r29, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[29].u32 ) };
	// 8309D244: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 8309D248: 80ABB7E8  lwz r5, -0x4818(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8309D24C: 4BF891A5  bl 0x830263f0
	ctx.lr = 0x8309D250;
	sub_830263F0(ctx, base);
	// 8309D250: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309D254: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309D258: 40820020  bne 0x8309d278
	if !ctx.cr[0].eq {
	pc = 0x8309D278; continue 'dispatch;
	}
	// 8309D25C: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309D260: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309D264: 41820014  beq 0x8309d278
	if ctx.cr[0].eq {
	pc = 0x8309D278; continue 'dispatch;
	}
	// 8309D268: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8309D26C: 38800061  li r4, 0x61
	ctx.r[4].s64 = 97;
	// 8309D270: 806B00A8  lwz r3, 0xa8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 8309D274: 4BF7A175  bl 0x830173e8
	ctx.lr = 0x8309D278;
	sub_830173E8(ctx, base);
	// 8309D278: 3B7D000C  addi r27, r29, 0xc
	ctx.r[27].s64 = ctx.r[29].s64 + 12;
	// 8309D27C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8309D280: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8309D284: 480120FD  bl 0x830af380
	ctx.lr = 0x8309D288;
	sub_830AF380(ctx, base);
	// 8309D288: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309D28C: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 8309D290: 409A0024  bne cr6, 0x8309d2b4
	if !ctx.cr[6].eq {
	pc = 0x8309D2B4; continue 'dispatch;
	}
	// 8309D294: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309D298: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309D29C: 4182014C  beq 0x8309d3e8
	if ctx.cr[0].eq {
	pc = 0x8309D3E8; continue 'dispatch;
	}
	// 8309D2A0: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8309D2A4: 38800062  li r4, 0x62
	ctx.r[4].s64 = 98;
	// 8309D2A8: 806B00A8  lwz r3, 0xa8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 8309D2AC: 4BF7A13D  bl 0x830173e8
	ctx.lr = 0x8309D2B0;
	sub_830173E8(ctx, base);
	// 8309D2B0: 48000138  b 0x8309d3e8
	pc = 0x8309D3E8; continue 'dispatch;
	// 8309D2B4: 807B0004  lwz r3, 4(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309D2B8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309D2BC: 41820014  beq 0x8309d2d0
	if ctx.cr[0].eq {
	pc = 0x8309D2D0; continue 'dispatch;
	}
	// 8309D2C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8309D2C4: 4BF5AE0D  bl 0x82ff80d0
	ctx.lr = 0x8309D2C8;
	sub_82FF80D0(ctx, base);
	// 8309D2C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309D2CC: 48000008  b 0x8309d2d4
	pc = 0x8309D2D4; continue 'dispatch;
	// 8309D2D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309D2D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309D2D8: 409A0034  bne cr6, 0x8309d30c
	if !ctx.cr[6].eq {
	pc = 0x8309D30C; continue 'dispatch;
	}
	// 8309D2DC: 807B0008  lwz r3, 8(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309D2E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309D2E4: 41820010  beq 0x8309d2f4
	if ctx.cr[0].eq {
	pc = 0x8309D2F4; continue 'dispatch;
	}
	// 8309D2E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8309D2EC: 4BF8F585  bl 0x8302c870
	ctx.lr = 0x8309D2F0;
	sub_8302C870(ctx, base);
	// 8309D2F0: 48000008  b 0x8309d2f8
	pc = 0x8309D2F8; continue 'dispatch;
	// 8309D2F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309D2F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8309D2FC: 409A0010  bne cr6, 0x8309d30c
	if !ctx.cr[6].eq {
	pc = 0x8309D30C; continue 'dispatch;
	}
	// 8309D300: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309D304: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8309D308: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8309D30C: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 8309D310: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8309D314: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8309D318: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8309D31C: 4BFFFCE5  bl 0x8309d000
	ctx.lr = 0x8309D320;
	sub_8309D000(ctx, base);
	// 8309D320: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309D324: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309D328: 4182000C  beq 0x8309d334
	if ctx.cr[0].eq {
	pc = 0x8309D334; continue 'dispatch;
	}
	// 8309D32C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309D330: 48000008  b 0x8309d338
	pc = 0x8309D338; continue 'dispatch;
	// 8309D334: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309D338: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309D33C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8309D340: 409A00A8  bne cr6, 0x8309d3e8
	if !ctx.cr[6].eq {
	pc = 0x8309D3E8; continue 'dispatch;
	}
	// 8309D344: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8309D348: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309D34C: 4BFFF715  bl 0x8309ca60
	ctx.lr = 0x8309D350;
	sub_8309CA60(ctx, base);
	// 8309D350: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309D354: 4182000C  beq 0x8309d360
	if ctx.cr[0].eq {
	pc = 0x8309D360; continue 'dispatch;
	}
	// 8309D358: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309D35C: 4BFFF4ED  bl 0x8309c848
	ctx.lr = 0x8309D360;
	sub_8309C848(ctx, base);
	// 8309D360: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 8309D364: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309D368: 409A004C  bne cr6, 0x8309d3b4
	if !ctx.cr[6].eq {
	pc = 0x8309D3B4; continue 'dispatch;
	}
	// 8309D36C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8309D370: 809D0028  lwz r4, 0x28(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 8309D374: 4BF3AF25  bl 0x82fd8298
	ctx.lr = 0x8309D378;
	sub_82FD8298(ctx, base);
	// 8309D378: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8309D37C: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8309D380: 4182002C  beq 0x8309d3ac
	if ctx.cr[0].eq {
	pc = 0x8309D3AC; continue 'dispatch;
	}
	// 8309D384: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8309D388: 80DD0028  lwz r6, 0x28(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 8309D38C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8309D390: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309D394: 4BFFF54D  bl 0x8309c8e0
	ctx.lr = 0x8309D398;
	sub_8309C8E0(ctx, base);
	// 8309D398: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309D39C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8309D3A0: 396BE2C8  addi r11, r11, -0x1d38
	ctx.r[11].s64 = ctx.r[11].s64 + -7480;
	// 8309D3A4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309D3A8: 48000008  b 0x8309d3b0
	pc = 0x8309D3B0; continue 'dispatch;
	// 8309D3AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8309D3B0: 915D001C  stw r10, 0x1c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8309D3B4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8309D3B8: 809D0028  lwz r4, 0x28(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 8309D3BC: 4BF3AEDD  bl 0x82fd8298
	ctx.lr = 0x8309D3C0;
	sub_82FD8298(ctx, base);
	// 8309D3C0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8309D3C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309D3C8: 41820014  beq 0x8309d3dc
	if ctx.cr[0].eq {
	pc = 0x8309D3DC; continue 'dispatch;
	}
	// 8309D3CC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8309D3D0: 48012021  bl 0x830af3f0
	ctx.lr = 0x8309D3D4;
	sub_830AF3F0(ctx, base);
	// 8309D3D4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8309D3D8: 48000008  b 0x8309d3e0
	pc = 0x8309D3E0; continue 'dispatch;
	// 8309D3DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8309D3E0: 807D001C  lwz r3, 0x1c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 8309D3E4: 4BF9DD6D  bl 0x8303b150
	ctx.lr = 0x8309D3E8;
	sub_8303B150(ctx, base);
	// 8309D3E8: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8309D3EC: 4810ADC0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309D3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309D3F0 size=48
    let mut pc: u32 = 0x8309D3F0;
    'dispatch: loop {
        match pc {
            0x8309D3F0 => {
    //   block [0x8309D3F0..0x8309D420)
	// 8309D3F0: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8309D3F4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309D3F8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309D3FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309D400: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8309D404: 808B0028  lwz r4, 0x28(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8309D408: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309D40C: 4BF3AED5  bl 0x82fd82e0
	ctx.lr = 0x8309D410;
	sub_82FD82E0(ctx, base);
	// 8309D410: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309D414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309D418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309D41C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309D420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309D420 size=48
    let mut pc: u32 = 0x8309D420;
    'dispatch: loop {
        match pc {
            0x8309D420 => {
    //   block [0x8309D420..0x8309D450)
	// 8309D420: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8309D424: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309D428: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309D42C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309D430: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8309D434: 808B0028  lwz r4, 0x28(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8309D438: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309D43C: 4BF3AEA5  bl 0x82fd82e0
	ctx.lr = 0x8309D440;
	sub_82FD82E0(ctx, base);
	// 8309D440: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309D444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309D448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309D44C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309D450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309D450 size=8
    let mut pc: u32 = 0x8309D450;
    'dispatch: loop {
        match pc {
            0x8309D450 => {
    //   block [0x8309D450..0x8309D458)
	// 8309D450: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309D454: 8216E400  lwz r16, -0x1c00(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-7168 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309D458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309D458 size=248
    let mut pc: u32 = 0x8309D458;
    'dispatch: loop {
        match pc {
            0x8309D458 => {
    //   block [0x8309D458..0x8309D550)
	// 8309D458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309D45C: 4810ACFD  bl 0x831a8158
	ctx.lr = 0x8309D460;
	sub_831A8130(ctx, base);
	// 8309D460: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8309D464: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309D468: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 8309D46C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8309D470: 8178001C  lwz r11, 0x1c(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(28 as u32) ) } as u64;
	// 8309D474: 93BF00B4  stw r29, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[29].u32 ) };
	// 8309D478: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309D47C: 418200CC  beq 0x8309d548
	if ctx.cr[0].eq {
	pc = 0x8309D548; continue 'dispatch;
	}
	// 8309D480: 832B0008  lwz r25, 8(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309D484: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8309D488: 28190000  cmplwi r25, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309D48C: 418200BC  beq 0x8309d548
	if ctx.cr[0].eq {
	pc = 0x8309D548; continue 'dispatch;
	}
	// 8309D490: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309D494: 3B4BE2C8  addi r26, r11, -0x1d38
	ctx.r[26].s64 = ctx.r[11].s64 + -7480;
	// 8309D498: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8309D49C: 8078001C  lwz r3, 0x1c(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(28 as u32) ) } as u64;
	// 8309D4A0: 4BF8F3D1  bl 0x8302c870
	ctx.lr = 0x8309D4A4;
	sub_8302C870(ctx, base);
	// 8309D4A4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8309D4A8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309D4AC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8309D4B0: 4BFFF5B1  bl 0x8309ca60
	ctx.lr = 0x8309D4B4;
	sub_8309CA60(ctx, base);
	// 8309D4B4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309D4B8: 40820084  bne 0x8309d53c
	if !ctx.cr[0].eq {
	pc = 0x8309D53C; continue 'dispatch;
	}
	// 8309D4BC: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 8309D4C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309D4C4: 409A0044  bne cr6, 0x8309d508
	if !ctx.cr[6].eq {
	pc = 0x8309D508; continue 'dispatch;
	}
	// 8309D4C8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8309D4CC: 809D0028  lwz r4, 0x28(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 8309D4D0: 4BF3ADC9  bl 0x82fd8298
	ctx.lr = 0x8309D4D4;
	sub_82FD8298(ctx, base);
	// 8309D4D4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8309D4D8: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8309D4DC: 41820024  beq 0x8309d500
	if ctx.cr[0].eq {
	pc = 0x8309D500; continue 'dispatch;
	}
	// 8309D4E0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8309D4E4: 80DD0028  lwz r6, 0x28(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 8309D4E8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8309D4EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309D4F0: 4BFFF3F1  bl 0x8309c8e0
	ctx.lr = 0x8309D4F4;
	sub_8309C8E0(ctx, base);
	// 8309D4F4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8309D4F8: 935E0000  stw r26, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 8309D4FC: 48000008  b 0x8309d504
	pc = 0x8309D504; continue 'dispatch;
	// 8309D500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309D504: 917D001C  stw r11, 0x1c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8309D508: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8309D50C: 809D0028  lwz r4, 0x28(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 8309D510: 4BF3AD89  bl 0x82fd8298
	ctx.lr = 0x8309D514;
	sub_82FD8298(ctx, base);
	// 8309D514: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8309D518: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309D51C: 41820014  beq 0x8309d530
	if ctx.cr[0].eq {
	pc = 0x8309D530; continue 'dispatch;
	}
	// 8309D520: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8309D524: 48011ECD  bl 0x830af3f0
	ctx.lr = 0x8309D528;
	sub_830AF3F0(ctx, base);
	// 8309D528: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8309D52C: 48000008  b 0x8309d534
	pc = 0x8309D534; continue 'dispatch;
	// 8309D530: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8309D534: 807D001C  lwz r3, 0x1c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 8309D538: 4BF9DC19  bl 0x8303b150
	ctx.lr = 0x8309D53C;
	sub_8303B150(ctx, base);
	// 8309D53C: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 8309D540: 7F1BC840  cmplw cr6, r27, r25
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[25].u32, &mut ctx.xer);
	// 8309D544: 4198FF54  blt cr6, 0x8309d498
	if ctx.cr[6].lt {
	pc = 0x8309D498; continue 'dispatch;
	}
	// 8309D548: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8309D54C: 4810AC5C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309D550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309D550 size=48
    let mut pc: u32 = 0x8309D550;
    'dispatch: loop {
        match pc {
            0x8309D550 => {
    //   block [0x8309D550..0x8309D580)
	// 8309D550: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8309D554: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309D558: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309D55C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309D560: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8309D564: 808B0028  lwz r4, 0x28(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8309D568: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309D56C: 4BF3AD75  bl 0x82fd82e0
	ctx.lr = 0x8309D570;
	sub_82FD82E0(ctx, base);
	// 8309D570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309D574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309D578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309D57C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309D580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309D580 size=48
    let mut pc: u32 = 0x8309D580;
    'dispatch: loop {
        match pc {
            0x8309D580 => {
    //   block [0x8309D580..0x8309D5B0)
	// 8309D580: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8309D584: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309D588: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309D58C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309D590: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8309D594: 808B0028  lwz r4, 0x28(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8309D598: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309D59C: 4BF3AD45  bl 0x82fd82e0
	ctx.lr = 0x8309D5A0;
	sub_82FD82E0(ctx, base);
	// 8309D5A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309D5A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309D5A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309D5AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309D5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309D5B0 size=140
    let mut pc: u32 = 0x8309D5B0;
    'dispatch: loop {
        match pc {
            0x8309D5B0 => {
    //   block [0x8309D5B0..0x8309D63C)
	// 8309D5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309D5B4: 4810ABB5  bl 0x831a8168
	ctx.lr = 0x8309D5B8;
	sub_831A8130(ctx, base);
	// 8309D5B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309D5BC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8309D5C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309D5C4: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8309D5C8: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309D5CC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309D5D0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309D5D4: 4182000C  beq 0x8309d5e0
	if ctx.cr[0].eq {
	pc = 0x8309D5E0; continue 'dispatch;
	}
	// 8309D5D8: 83CB0008  lwz r30, 8(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309D5DC: 48000008  b 0x8309d5e4
	pc = 0x8309D5E4; continue 'dispatch;
	// 8309D5E0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8309D5E4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8309D5E8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8309D5EC: 40990048  ble cr6, 0x8309d634
	if !ctx.cr[6].gt {
	pc = 0x8309D634; continue 'dispatch;
	}
	// 8309D5F0: 3B9D000C  addi r28, r29, 0xc
	ctx.r[28].s64 = ctx.r[29].s64 + 12;
	// 8309D5F4: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309D5F8: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309D5FC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309D600: 41820014  beq 0x8309d614
	if ctx.cr[0].eq {
	pc = 0x8309D614; continue 'dispatch;
	}
	// 8309D604: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8309D608: 4BF8F269  bl 0x8302c870
	ctx.lr = 0x8309D60C;
	sub_8302C870(ctx, base);
	// 8309D60C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8309D610: 48000008  b 0x8309d618
	pc = 0x8309D618; continue 'dispatch;
	// 8309D614: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8309D618: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8309D61C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8309D620: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309D624: 4BFFF9DD  bl 0x8309d000
	ctx.lr = 0x8309D628;
	sub_8309D000(ctx, base);
	// 8309D628: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8309D62C: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8309D630: 4198FFC4  blt cr6, 0x8309d5f4
	if ctx.cr[6].lt {
	pc = 0x8309D5F4; continue 'dispatch;
	}
	// 8309D634: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8309D638: 4810AB80  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309D640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309D640 size=288
    let mut pc: u32 = 0x8309D640;
    'dispatch: loop {
        match pc {
            0x8309D640 => {
    //   block [0x8309D640..0x8309D760)
	// 8309D640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309D644: 4810AB29  bl 0x831a816c
	ctx.lr = 0x8309D648;
	sub_831A8130(ctx, base);
	// 8309D648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309D64C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309D650: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309D654: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309D658: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309D65C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309D660: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309D664: 4E800421  bctrl
	ctx.lr = 0x8309D668;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309D668: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 8309D66C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8309D670: 409A00E8  bne cr6, 0x8309d758
	if !ctx.cr[6].eq {
	pc = 0x8309D758; continue 'dispatch;
	}
	// 8309D674: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309D678: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8309D67C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309D680: 808B001C  lwz r4, 0x1c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8309D684: 4BF5CDDD  bl 0x82ffa460
	ctx.lr = 0x8309D688;
	sub_82FFA460(ctx, base);
	// 8309D688: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309D68C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309D690: 41820008  beq 0x8309d698
	if ctx.cr[0].eq {
	pc = 0x8309D698; continue 'dispatch;
	}
	// 8309D694: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309D698: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309D69C: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8309D6A0: 409A0038  bne cr6, 0x8309d6d8
	if !ctx.cr[6].eq {
	pc = 0x8309D6D8; continue 'dispatch;
	}
	// 8309D6A4: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309D6A8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309D6AC: 418200AC  beq 0x8309d758
	if ctx.cr[0].eq {
	pc = 0x8309D758; continue 'dispatch;
	}
	// 8309D6B0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309D6B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8309D6B8: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8309D6BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8309D6C0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8309D6C4: 3880006A  li r4, 0x6a
	ctx.r[4].s64 = 106;
	// 8309D6C8: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309D6CC: 806A00A8  lwz r3, 0xa8(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(168 as u32) ) } as u64;
	// 8309D6D0: 4BF79E89  bl 0x83017558
	ctx.lr = 0x8309D6D4;
	sub_83017558(ctx, base);
	// 8309D6D4: 48000084  b 0x8309d758
	pc = 0x8309D758; continue 'dispatch;
	// 8309D6D8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8309D6DC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309D6E0: 4182000C  beq 0x8309d6ec
	if ctx.cr[0].eq {
	pc = 0x8309D6EC; continue 'dispatch;
	}
	// 8309D6E4: 83AB0008  lwz r29, 8(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309D6E8: 48000008  b 0x8309d6f0
	pc = 0x8309D6F0; continue 'dispatch;
	// 8309D6EC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8309D6F0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8309D6F4: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309D6F8: 41820060  beq 0x8309d758
	if ctx.cr[0].eq {
	pc = 0x8309D758; continue 'dispatch;
	}
	// 8309D6FC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8309D700: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8309D704: 4BF8F16D  bl 0x8302c870
	ctx.lr = 0x8309D708;
	sub_8302C870(ctx, base);
	// 8309D708: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8309D70C: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8309D710: 4BFFF351  bl 0x8309ca60
	ctx.lr = 0x8309D714;
	sub_8309CA60(ctx, base);
	// 8309D714: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309D718: 40820034  bne 0x8309d74c
	if !ctx.cr[0].eq {
	pc = 0x8309D74C; continue 'dispatch;
	}
	// 8309D71C: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309D720: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309D724: 41820028  beq 0x8309d74c
	if ctx.cr[0].eq {
	pc = 0x8309D74C; continue 'dispatch;
	}
	// 8309D728: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309D72C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8309D730: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8309D734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8309D738: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8309D73C: 3880006B  li r4, 0x6b
	ctx.r[4].s64 = 107;
	// 8309D740: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309D744: 806A00A8  lwz r3, 0xa8(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(168 as u32) ) } as u64;
	// 8309D748: 4BF79E11  bl 0x83017558
	ctx.lr = 0x8309D74C;
	sub_83017558(ctx, base);
	// 8309D74C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8309D750: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8309D754: 4198FFA8  blt cr6, 0x8309d6fc
	if ctx.cr[6].lt {
	pc = 0x8309D6FC; continue 'dispatch;
	}
	// 8309D758: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8309D75C: 4810AA60  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309D760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309D760 size=92
    let mut pc: u32 = 0x8309D760;
    'dispatch: loop {
        match pc {
            0x8309D760 => {
    //   block [0x8309D760..0x8309D7BC)
	// 8309D760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309D764: 4810AA05  bl 0x831a8168
	ctx.lr = 0x8309D768;
	sub_831A8130(ctx, base);
	// 8309D768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309D76C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8309D770: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8309D774: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 8309D778: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309D77C: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8309D780: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309D784: 4800E5D5  bl 0x830abd58
	ctx.lr = 0x8309D788;
	sub_830ABD58(ctx, base);
	// 8309D788: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309D78C: 939F0024  stw r28, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[28].u32 ) };
	// 8309D790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8309D794: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 8309D798: 396BE45C  addi r11, r11, -0x1ba4
	ctx.r[11].s64 = ctx.r[11].s64 + -7076;
	// 8309D79C: 93BF0034  stw r29, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 8309D7A0: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 8309D7A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309D7A8: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 8309D7AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309D7B0: 913F002C  stw r9, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[9].u32 ) };
	// 8309D7B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8309D7B8: 4810AA00  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309D7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309D7C0 size=88
    let mut pc: u32 = 0x8309D7C0;
    'dispatch: loop {
        match pc {
            0x8309D7C0 => {
    //   block [0x8309D7C0..0x8309D818)
	// 8309D7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309D7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309D7C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309D7CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309D7D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309D7D4: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309D7D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309D7DC: 396BE45C  addi r11, r11, -0x1ba4
	ctx.r[11].s64 = ctx.r[11].s64 + -7076;
	// 8309D7E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309D7E4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309D7E8: 4800E0A1  bl 0x830ab888
	ctx.lr = 0x8309D7EC;
	sub_830AB888(ctx, base);
	// 8309D7EC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309D7F0: 4182000C  beq 0x8309d7fc
	if ctx.cr[0].eq {
	pc = 0x8309D7FC; continue 'dispatch;
	}
	// 8309D7F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309D7F8: 4BF3AAE9  bl 0x82fd82e0
	ctx.lr = 0x8309D7FC;
	sub_82FD82E0(ctx, base);
	// 8309D7FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309D800: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309D804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309D808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309D80C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309D810: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309D814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309D818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309D818 size=60
    let mut pc: u32 = 0x8309D818;
    'dispatch: loop {
        match pc {
            0x8309D818 => {
    //   block [0x8309D818..0x8309D854)
	// 8309D818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309D81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309D820: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309D824: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309D828: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309D82C: 4800E3AD  bl 0x830abbd8
	ctx.lr = 0x8309D830;
	sub_830ABBD8(ctx, base);
	// 8309D830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309D834: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8309D838: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8309D83C: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 8309D840: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309D844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309D848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309D84C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309D850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309D858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309D858 size=96
    let mut pc: u32 = 0x8309D858;
    'dispatch: loop {
        match pc {
            0x8309D858 => {
    //   block [0x8309D858..0x8309D8B8)
	// 8309D858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309D85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309D860: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309D864: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309D868: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309D86C: 4800E3DD  bl 0x830abc48
	ctx.lr = 0x8309D870;
	sub_830ABC48(ctx, base);
	// 8309D870: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8309D874: 815F002C  lwz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8309D878: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 8309D87C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8309D880: 913F0028  stw r9, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 8309D884: 409A0020  bne cr6, 0x8309d8a4
	if !ctx.cr[6].eq {
	pc = 0x8309D8A4; continue 'dispatch;
	}
	// 8309D888: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8309D88C: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8309D890: 80BF0024  lwz r5, 0x24(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8309D894: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8309D898: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8309D89C: 808A0008  lwz r4, 8(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309D8A0: 4BFFEC49  bl 0x8309c4e8
	ctx.lr = 0x8309D8A4;
	sub_8309C4E8(ctx, base);
	// 8309D8A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309D8A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309D8AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309D8B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309D8B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309D8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309D8B8 size=24
    let mut pc: u32 = 0x8309D8B8;
    'dispatch: loop {
        match pc {
            0x8309D8B8 => {
    //   block [0x8309D8B8..0x8309D8D0)
	// 8309D8B8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309D8BC: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 8309D8C0: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 8309D8C4: 396BE474  addi r11, r11, -0x1b8c
	ctx.r[11].s64 = ctx.r[11].s64 + -7052;
	// 8309D8C8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309D8CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309D8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309D8D0 size=8
    let mut pc: u32 = 0x8309D8D0;
    'dispatch: loop {
        match pc {
            0x8309D8D0 => {
    //   block [0x8309D8D0..0x8309D8D8)
	// 8309D8D0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309D8D4: 8216E490  lwz r16, -0x1b70(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-7024 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309D8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309D8D8 size=112
    let mut pc: u32 = 0x8309D8D8;
    'dispatch: loop {
        match pc {
            0x8309D8D8 => {
    //   block [0x8309D8D8..0x8309D948)
	// 8309D8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309D8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309D8E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309D8E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309D8E8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8309D8EC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309D8F0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309D8F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309D8F8: 396BE474  addi r11, r11, -0x1b8c
	ctx.r[11].s64 = ctx.r[11].s64 + -7052;
	// 8309D8FC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8309D900: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309D904: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309D908: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309D90C: 41820018  beq 0x8309d924
	if ctx.cr[0].eq {
	pc = 0x8309D924; continue 'dispatch;
	}
	// 8309D910: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309D914: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8309D918: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309D91C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309D920: 4E800421  bctrl
	ctx.lr = 0x8309D924;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309D924: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309D928: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 8309D92C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309D930: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8309D934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309D938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309D93C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309D940: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309D944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309D948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309D948 size=40
    let mut pc: u32 = 0x8309D948;
    'dispatch: loop {
        match pc {
            0x8309D948 => {
    //   block [0x8309D948..0x8309D970)
	// 8309D948: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8309D94C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309D950: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309D954: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309D958: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8309D95C: 4BFAEE05  bl 0x8304c760
	ctx.lr = 0x8309D960;
	sub_8304C760(ctx, base);
	// 8309D960: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309D964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309D968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309D96C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309D970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309D970 size=8
    let mut pc: u32 = 0x8309D970;
    'dispatch: loop {
        match pc {
            0x8309D970 => {
    //   block [0x8309D970..0x8309D978)
	// 8309D970: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309D974: 8216E4C8  lwz r16, -0x1b38(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-6968 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309D978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309D978 size=100
    let mut pc: u32 = 0x8309D978;
    'dispatch: loop {
        match pc {
            0x8309D978 => {
    //   block [0x8309D978..0x8309D9DC)
	// 8309D978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309D97C: 4810A7E9  bl 0x831a8164
	ctx.lr = 0x8309D980;
	sub_831A8130(ctx, base);
	// 8309D980: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8309D984: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309D988: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8309D98C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309D990: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8309D994: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8309D998: 38600038  li r3, 0x38
	ctx.r[3].s64 = 56;
	// 8309D99C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8309D9A0: 93BF00BC  stw r29, 0xbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[29].u32 ) };
	// 8309D9A4: 4BF3A8F5  bl 0x82fd8298
	ctx.lr = 0x8309D9A8;
	sub_82FD8298(ctx, base);
	// 8309D9A8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8309D9AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309D9B0: 41820020  beq 0x8309d9d0
	if ctx.cr[0].eq {
	pc = 0x8309D9D0; continue 'dispatch;
	}
	// 8309D9B4: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 8309D9B8: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309D9BC: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8309D9C0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8309D9C4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8309D9C8: 4BFFFD99  bl 0x8309d760
	ctx.lr = 0x8309D9CC;
	sub_8309D760(ctx, base);
	// 8309D9CC: 48000008  b 0x8309d9d4
	pc = 0x8309D9D4; continue 'dispatch;
	// 8309D9D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309D9D4: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8309D9D8: 4810A7DC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309D9DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309D9DC size=44
    let mut pc: u32 = 0x8309D9DC;
    'dispatch: loop {
        match pc {
            0x8309D9DC => {
    //   block [0x8309D9DC..0x8309DA08)
	// 8309D9DC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8309D9E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309D9E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309D9E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309D9EC: 809F00BC  lwz r4, 0xbc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 8309D9F0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309D9F4: 4BF3A8ED  bl 0x82fd82e0
	ctx.lr = 0x8309D9F8;
	sub_82FD82E0(ctx, base);
	// 8309D9F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309D9FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309DA00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309DA04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309DA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309DA08 size=12
    let mut pc: u32 = 0x8309DA08;
    'dispatch: loop {
        match pc {
            0x8309DA08 => {
    //   block [0x8309DA08..0x8309DA14)
	// 8309DA08: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8309DA0C: 386B3600  addi r3, r11, 0x3600
	ctx.r[3].s64 = ctx.r[11].s64 + 13824;
	// 8309DA10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309DA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309DA18 size=128
    let mut pc: u32 = 0x8309DA18;
    'dispatch: loop {
        match pc {
            0x8309DA18 => {
    //   block [0x8309DA18..0x8309DA98)
	// 8309DA18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309DA1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309DA20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309DA24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309DA28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309DA2C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8309DA30: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309DA34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309DA38: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 8309DA3C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8309DA40: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309DA44: 4182001C  beq 0x8309da60
	if ctx.cr[0].eq {
	pc = 0x8309DA60; continue 'dispatch;
	}
	// 8309DA48: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309DA4C: 4BF5C1B5  bl 0x82ff9c00
	ctx.lr = 0x8309DA50;
	sub_82FF9C00(ctx, base);
	// 8309DA50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309DA54: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309DA58: 4BFFB649  bl 0x830990a0
	ctx.lr = 0x8309DA5C;
	sub_830990A0(ctx, base);
	// 8309DA5C: 48000024  b 0x8309da80
	pc = 0x8309DA80; continue 'dispatch;
	// 8309DA60: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8309DA64: 388B35C0  addi r4, r11, 0x35c0
	ctx.r[4].s64 = ctx.r[11].s64 + 13760;
	// 8309DA68: 4BF5C259  bl 0x82ff9cc0
	ctx.lr = 0x8309DA6C;
	sub_82FF9CC0(ctx, base);
	// 8309DA6C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8309DA70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309DA74: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8309DA78: 4BFFB6A9  bl 0x83099120
	ctx.lr = 0x8309DA7C;
	sub_83099120(ctx, base);
	// 8309DA7C: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8309DA80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309DA84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309DA88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309DA8C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309DA90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309DA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309DA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309DA98 size=264
    let mut pc: u32 = 0x8309DA98;
    'dispatch: loop {
        match pc {
            0x8309DA98 => {
    //   block [0x8309DA98..0x8309DBA0)
	// 8309DA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309DA9C: 4810A6B9  bl 0x831a8154
	ctx.lr = 0x8309DAA0;
	sub_831A8130(ctx, base);
	// 8309DAA0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309DAA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309DAA8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8309DAAC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8309DAB0: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8309DAB4: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 8309DAB8: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 8309DABC: 4800E375  bl 0x830abe30
	ctx.lr = 0x8309DAC0;
	sub_830ABE30(ctx, base);
	// 8309DAC0: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8309DAC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309DAC8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8309DACC: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8309DAD0: 4800DDC9  bl 0x830ab898
	ctx.lr = 0x8309DAD4;
	sub_830AB898(ctx, base);
	// 8309DAD4: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8309DAD8: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8309DADC: 409A0010  bne cr6, 0x8309daec
	if !ctx.cr[6].eq {
	pc = 0x8309DAEC; continue 'dispatch;
	}
	// 8309DAE0: 546B07FE  clrlwi r11, r3, 0x1f
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000001u64;
	// 8309DAE4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8309DAE8: 419A0010  beq cr6, 0x8309daf8
	if ctx.cr[6].eq {
	pc = 0x8309DAF8; continue 'dispatch;
	}
	// 8309DAEC: 706B0005  andi. r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u64 & 5;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309DAF0: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 8309DAF4: 409A00A4  bne cr6, 0x8309db98
	if !ctx.cr[6].eq {
	pc = 0x8309DB98; continue 'dispatch;
	}
	// 8309DAF8: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8309DAFC: 82EB0008  lwz r23, 8(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309DB00: 81770010  lwz r11, 0x10(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309DB04: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309DB08: 4182000C  beq 0x8309db14
	if ctx.cr[0].eq {
	pc = 0x8309DB14; continue 'dispatch;
	}
	// 8309DB0C: 83AB0008  lwz r29, 8(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309DB10: 48000008  b 0x8309db18
	pc = 0x8309DB18; continue 'dispatch;
	// 8309DB14: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8309DB18: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8309DB1C: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8309DB20: 80BF0024  lwz r5, 0x24(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8309DB24: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8309DB28: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8309DB2C: 4BFFE925  bl 0x8309c450
	ctx.lr = 0x8309DB30;
	sub_8309C450(ctx, base);
	// 8309DB30: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8309DB34: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8309DB38: 40990060  ble cr6, 0x8309db98
	if !ctx.cr[6].gt {
	pc = 0x8309DB98; continue 'dispatch;
	}
	// 8309DB3C: 80770010  lwz r3, 0x10(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309DB40: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309DB44: 41820014  beq 0x8309db58
	if ctx.cr[0].eq {
	pc = 0x8309DB58; continue 'dispatch;
	}
	// 8309DB48: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8309DB4C: 4BF8ED25  bl 0x8302c870
	ctx.lr = 0x8309DB50;
	sub_8302C870(ctx, base);
	// 8309DB50: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8309DB54: 48000008  b 0x8309db5c
	pc = 0x8309DB5C; continue 'dispatch;
	// 8309DB58: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8309DB5C: 80BF0024  lwz r5, 0x24(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8309DB60: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8309DB64: 4BFFE845  bl 0x8309c3a8
	ctx.lr = 0x8309DB68;
	sub_8309C3A8(ctx, base);
	// 8309DB68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309DB6C: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 8309DB70: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 8309DB74: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8309DB78: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8309DB7C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8309DB80: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309DB84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309DB88: 4E800421  bctrl
	ctx.lr = 0x8309DB8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309DB8C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8309DB90: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8309DB94: 4198FFA8  blt cr6, 0x8309db3c
	if ctx.cr[6].lt {
	pc = 0x8309DB3C; continue 'dispatch;
	}
	// 8309DB98: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8309DB9C: 4810A608  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309DBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309DBA0 size=76
    let mut pc: u32 = 0x8309DBA0;
    'dispatch: loop {
        match pc {
            0x8309DBA0 => {
    //   block [0x8309DBA0..0x8309DBEC)
	// 8309DBA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309DBA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309DBA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309DBAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309DBB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309DBB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309DBB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309DBBC: 4BFFFD1D  bl 0x8309d8d8
	ctx.lr = 0x8309DBC0;
	sub_8309D8D8(ctx, base);
	// 8309DBC0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309DBC4: 4182000C  beq 0x8309dbd0
	if ctx.cr[0].eq {
	pc = 0x8309DBD0; continue 'dispatch;
	}
	// 8309DBC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309DBCC: 4BF3A715  bl 0x82fd82e0
	ctx.lr = 0x8309DBD0;
	sub_82FD82E0(ctx, base);
	// 8309DBD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309DBD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309DBD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309DBDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309DBE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309DBE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309DBE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309DBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309DBF0 size=80
    let mut pc: u32 = 0x8309DBF0;
    'dispatch: loop {
        match pc {
            0x8309DBF0 => {
    //   block [0x8309DBF0..0x8309DC40)
	// 8309DBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309DBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309DBF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309DBFC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8309DC00: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8309DC04: 4BF3A695  bl 0x82fd8298
	ctx.lr = 0x8309DC08;
	sub_82FD8298(ctx, base);
	// 8309DC08: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309DC0C: 41820020  beq 0x8309dc2c
	if ctx.cr[0].eq {
	pc = 0x8309DC2C; continue 'dispatch;
	}
	// 8309DC10: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309DC14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8309DC18: 396BE474  addi r11, r11, -0x1b8c
	ctx.r[11].s64 = ctx.r[11].s64 + -7052;
	// 8309DC1C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8309DC20: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309DC24: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8309DC28: 48000008  b 0x8309dc30
	pc = 0x8309DC30; continue 'dispatch;
	// 8309DC2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309DC30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309DC34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309DC38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309DC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309DC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309DC40 size=80
    let mut pc: u32 = 0x8309DC40;
    'dispatch: loop {
        match pc {
            0x8309DC40 => {
    //   block [0x8309DC40..0x8309DC90)
	// 8309DC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309DC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309DC48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309DC4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309DC50: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 8309DC54: 807FBBE0  lwz r3, -0x4420(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-17440 as u32) ) } as u64;
	// 8309DC58: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8309DC5C: 419A0018  beq cr6, 0x8309dc74
	if ctx.cr[6].eq {
	pc = 0x8309DC74; continue 'dispatch;
	}
	// 8309DC60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309DC64: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8309DC68: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309DC6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309DC70: 4E800421  bctrl
	ctx.lr = 0x8309DC74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309DC74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309DC78: 917FBBE0  stw r11, -0x4420(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-17440 as u32), ctx.r[11].u32 ) };
	// 8309DC7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309DC80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309DC84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309DC88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309DC8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309DC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309DC90 size=80
    let mut pc: u32 = 0x8309DC90;
    'dispatch: loop {
        match pc {
            0x8309DC90 => {
    //   block [0x8309DC90..0x8309DCE0)
	// 8309DC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309DC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309DC98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309DC9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309DCA0: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 8309DCA4: 807FBBE4  lwz r3, -0x441c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-17436 as u32) ) } as u64;
	// 8309DCA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8309DCAC: 419A0018  beq cr6, 0x8309dcc4
	if ctx.cr[6].eq {
	pc = 0x8309DCC4; continue 'dispatch;
	}
	// 8309DCB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309DCB4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8309DCB8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309DCBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309DCC0: 4E800421  bctrl
	ctx.lr = 0x8309DCC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309DCC4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309DCC8: 917FBBE4  stw r11, -0x441c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-17436 as u32), ctx.r[11].u32 ) };
	// 8309DCCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309DCD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309DCD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309DCD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309DCDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309DCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309DCE0 size=28
    let mut pc: u32 = 0x8309DCE0;
    'dispatch: loop {
        match pc {
            0x8309DCE0 => {
    //   block [0x8309DCE0..0x8309DCFC)
	// 8309DCE0: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8309DCE4: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 8309DCE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8309DCEC: 396B43A8  addi r11, r11, 0x43a8
	ctx.r[11].s64 = ctx.r[11].s64 + 17320;
	// 8309DCF0: 99430004  stb r10, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 8309DCF4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309DCF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309DD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309DD00 size=84
    let mut pc: u32 = 0x8309DD00;
    'dispatch: loop {
        match pc {
            0x8309DD00 => {
    //   block [0x8309DD00..0x8309DD54)
	// 8309DD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309DD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309DD08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309DD0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309DD10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309DD14: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 8309DD18: 83FEBBE8  lwz r31, -0x4418(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17432 as u32) ) } as u64;
	// 8309DD1C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8309DD20: 419A0014  beq cr6, 0x8309dd34
	if ctx.cr[6].eq {
	pc = 0x8309DD34; continue 'dispatch;
	}
	// 8309DD24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309DD28: 4BF57A61  bl 0x82ff5788
	ctx.lr = 0x8309DD2C;
	sub_82FF5788(ctx, base);
	// 8309DD2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309DD30: 4BF3A5B1  bl 0x82fd82e0
	ctx.lr = 0x8309DD34;
	sub_82FD82E0(ctx, base);
	// 8309DD34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309DD38: 917EBBE8  stw r11, -0x4418(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-17432 as u32), ctx.r[11].u32 ) };
	// 8309DD3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309DD40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309DD44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309DD48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309DD4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309DD50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309DD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309DD58 size=8
    let mut pc: u32 = 0x8309DD58;
    'dispatch: loop {
        match pc {
            0x8309DD58 => {
    //   block [0x8309DD58..0x8309DD60)
	// 8309DD58: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309DD5C: 8216E508  lwz r16, -0x1af8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-6904 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309DD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309DD60 size=164
    let mut pc: u32 = 0x8309DD60;
    'dispatch: loop {
        match pc {
            0x8309DD60 => {
    //   block [0x8309DD60..0x8309DE04)
	// 8309DD60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309DD64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309DD68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309DD6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309DD70: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8309DD74: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309DD78: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 8309DD7C: 807EBBE8  lwz r3, -0x4418(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17432 as u32) ) } as u64;
	// 8309DD80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8309DD84: 409A0068  bne cr6, 0x8309ddec
	if !ctx.cr[6].eq {
	pc = 0x8309DDEC; continue 'dispatch;
	}
	// 8309DD88: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8309DD8C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8309DD90: 808BB7EC  lwz r4, -0x4814(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18452 as u32) ) } as u64;
	// 8309DD94: 4BF57A45  bl 0x82ff57d8
	ctx.lr = 0x8309DD98;
	sub_82FF57D8(ctx, base);
	// 8309DD98: 817EBBE8  lwz r11, -0x4418(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17432 as u32) ) } as u64;
	// 8309DD9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309DDA0: 409A0040  bne cr6, 0x8309dde0
	if !ctx.cr[6].eq {
	pc = 0x8309DDE0; continue 'dispatch;
	}
	// 8309DDA4: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8309DDA8: 4BF3A4A1  bl 0x82fd8248
	ctx.lr = 0x8309DDAC;
	sub_82FD8248(ctx, base);
	// 8309DDAC: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8309DDB0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309DDB4: 41820010  beq 0x8309ddc4
	if ctx.cr[0].eq {
	pc = 0x8309DDC4; continue 'dispatch;
	}
	// 8309DDB8: 4BF57991  bl 0x82ff5748
	ctx.lr = 0x8309DDBC;
	sub_82FF5748(ctx, base);
	// 8309DDBC: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 8309DDC0: 48000008  b 0x8309ddc8
	pc = 0x8309DDC8; continue 'dispatch;
	// 8309DDC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8309DDC8: 3D60830A  lis r11, -0x7cf6
	ctx.r[11].s64 = -2096496640;
	// 8309DDCC: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 8309DDD0: 913EBBE8  stw r9, -0x4418(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-17432 as u32), ctx.r[9].u32 ) };
	// 8309DDD4: 388BDD00  addi r4, r11, -0x2300
	ctx.r[4].s64 = ctx.r[11].s64 + -8960;
	// 8309DDD8: 386ABBEC  addi r3, r10, -0x4414
	ctx.r[3].s64 = ctx.r[10].s64 + -17428;
	// 8309DDDC: 4BF59D5D  bl 0x82ff7b38
	ctx.lr = 0x8309DDE0;
	sub_82FF7B38(ctx, base);
	// 8309DDE0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8309DDE4: 4BF57A2D  bl 0x82ff5810
	ctx.lr = 0x8309DDE8;
	sub_82FF5810(ctx, base);
	// 8309DDE8: 807EBBE8  lwz r3, -0x4418(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17432 as u32) ) } as u64;
	// 8309DDEC: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8309DDF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309DDF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309DDF8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309DDFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309DE00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309DE04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309DE04 size=40
    let mut pc: u32 = 0x8309DE04;
    'dispatch: loop {
        match pc {
            0x8309DE04 => {
    //   block [0x8309DE04..0x8309DE2C)
	// 8309DE04: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8309DE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309DE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309DE10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309DE14: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8309DE18: 4BF579F9  bl 0x82ff5810
	ctx.lr = 0x8309DE1C;
	sub_82FF5810(ctx, base);
	// 8309DE1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309DE20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309DE24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309DE28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309DE2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309DE2C size=40
    let mut pc: u32 = 0x8309DE2C;
    'dispatch: loop {
        match pc {
            0x8309DE2C => {
    //   block [0x8309DE2C..0x8309DE54)
	// 8309DE2C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8309DE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309DE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309DE38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309DE3C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8309DE40: 4BF3A4A1  bl 0x82fd82e0
	ctx.lr = 0x8309DE44;
	sub_82FD82E0(ctx, base);
	// 8309DE44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309DE48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309DE4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309DE50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309DE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309DE58 size=8
    let mut pc: u32 = 0x8309DE58;
    'dispatch: loop {
        match pc {
            0x8309DE58 => {
    //   block [0x8309DE58..0x8309DE60)
	// 8309DE58: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309DE5C: 8216E558  lwz r16, -0x1aa8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-6824 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309DE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309DE60 size=160
    let mut pc: u32 = 0x8309DE60;
    'dispatch: loop {
        match pc {
            0x8309DE60 => {
    //   block [0x8309DE60..0x8309DF00)
	// 8309DE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309DE64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309DE68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309DE6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309DE70: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8309DE74: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309DE78: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 8309DE7C: 807EBBE0  lwz r3, -0x4420(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17440 as u32) ) } as u64;
	// 8309DE80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8309DE84: 409A0064  bne cr6, 0x8309dee8
	if !ctx.cr[6].eq {
	pc = 0x8309DEE8; continue 'dispatch;
	}
	// 8309DE88: 4BFFFED9  bl 0x8309dd60
	ctx.lr = 0x8309DE8C;
	sub_8309DD60(ctx, base);
	// 8309DE8C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8309DE90: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8309DE94: 4BF57945  bl 0x82ff57d8
	ctx.lr = 0x8309DE98;
	sub_82FF57D8(ctx, base);
	// 8309DE98: 817EBBE0  lwz r11, -0x4420(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17440 as u32) ) } as u64;
	// 8309DE9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309DEA0: 409A003C  bne cr6, 0x8309dedc
	if !ctx.cr[6].eq {
	pc = 0x8309DEDC; continue 'dispatch;
	}
	// 8309DEA4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309DEA8: 386B8070  addi r3, r11, -0x7f90
	ctx.r[3].s64 = ctx.r[11].s64 + -32656;
	// 8309DEAC: 4BF3A1E5  bl 0x82fd8090
	ctx.lr = 0x8309DEB0;
	sub_82FD8090(ctx, base);
	// 8309DEB0: 907EBBE0  stw r3, -0x4420(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-17440 as u32), ctx.r[3].u32 ) };
	// 8309DEB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309DEB8: 40820010  bne 0x8309dec8
	if !ctx.cr[0].eq {
	pc = 0x8309DEC8; continue 'dispatch;
	}
	// 8309DEBC: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8309DEC0: 4BF56B89  bl 0x82ff4a48
	ctx.lr = 0x8309DEC4;
	sub_82FF4A48(ctx, base);
	// 8309DEC4: 48000018  b 0x8309dedc
	pc = 0x8309DEDC; continue 'dispatch;
	// 8309DEC8: 3D60830A  lis r11, -0x7cf6
	ctx.r[11].s64 = -2096496640;
	// 8309DECC: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 8309DED0: 388BDC40  addi r4, r11, -0x23c0
	ctx.r[4].s64 = ctx.r[11].s64 + -9152;
	// 8309DED4: 386ABC04  addi r3, r10, -0x43fc
	ctx.r[3].s64 = ctx.r[10].s64 + -17404;
	// 8309DED8: 4BF59C61  bl 0x82ff7b38
	ctx.lr = 0x8309DEDC;
	sub_82FF7B38(ctx, base);
	// 8309DEDC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8309DEE0: 4BF57931  bl 0x82ff5810
	ctx.lr = 0x8309DEE4;
	sub_82FF5810(ctx, base);
	// 8309DEE4: 807EBBE0  lwz r3, -0x4420(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17440 as u32) ) } as u64;
	// 8309DEE8: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8309DEEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309DEF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309DEF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309DEF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309DEFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309DF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309DF00 size=40
    let mut pc: u32 = 0x8309DF00;
    'dispatch: loop {
        match pc {
            0x8309DF00 => {
    //   block [0x8309DF00..0x8309DF28)
	// 8309DF00: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8309DF04: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309DF08: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309DF0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309DF10: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8309DF14: 4BF578FD  bl 0x82ff5810
	ctx.lr = 0x8309DF18;
	sub_82FF5810(ctx, base);
	// 8309DF18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309DF1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309DF20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309DF24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309DF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309DF28 size=8
    let mut pc: u32 = 0x8309DF28;
    'dispatch: loop {
        match pc {
            0x8309DF28 => {
    //   block [0x8309DF28..0x8309DF30)
	// 8309DF28: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309DF2C: 8216E598  lwz r16, -0x1a68(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-6760 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309DF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309DF30 size=160
    let mut pc: u32 = 0x8309DF30;
    'dispatch: loop {
        match pc {
            0x8309DF30 => {
    //   block [0x8309DF30..0x8309DFD0)
	// 8309DF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309DF34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309DF38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309DF3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309DF40: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8309DF44: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309DF48: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 8309DF4C: 807EBBE4  lwz r3, -0x441c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17436 as u32) ) } as u64;
	// 8309DF50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8309DF54: 409A0064  bne cr6, 0x8309dfb8
	if !ctx.cr[6].eq {
	pc = 0x8309DFB8; continue 'dispatch;
	}
	// 8309DF58: 4BFFFE09  bl 0x8309dd60
	ctx.lr = 0x8309DF5C;
	sub_8309DD60(ctx, base);
	// 8309DF5C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8309DF60: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8309DF64: 4BF57875  bl 0x82ff57d8
	ctx.lr = 0x8309DF68;
	sub_82FF57D8(ctx, base);
	// 8309DF68: 817EBBE4  lwz r11, -0x441c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17436 as u32) ) } as u64;
	// 8309DF6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309DF70: 409A003C  bne cr6, 0x8309dfac
	if !ctx.cr[6].eq {
	pc = 0x8309DFAC; continue 'dispatch;
	}
	// 8309DF74: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8309DF78: 386B7EB0  addi r3, r11, 0x7eb0
	ctx.r[3].s64 = ctx.r[11].s64 + 32432;
	// 8309DF7C: 4BF3A115  bl 0x82fd8090
	ctx.lr = 0x8309DF80;
	sub_82FD8090(ctx, base);
	// 8309DF80: 907EBBE4  stw r3, -0x441c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-17436 as u32), ctx.r[3].u32 ) };
	// 8309DF84: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309DF88: 40820010  bne 0x8309df98
	if !ctx.cr[0].eq {
	pc = 0x8309DF98; continue 'dispatch;
	}
	// 8309DF8C: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8309DF90: 4BF56AB9  bl 0x82ff4a48
	ctx.lr = 0x8309DF94;
	sub_82FF4A48(ctx, base);
	// 8309DF94: 48000018  b 0x8309dfac
	pc = 0x8309DFAC; continue 'dispatch;
	// 8309DF98: 3D60830A  lis r11, -0x7cf6
	ctx.r[11].s64 = -2096496640;
	// 8309DF9C: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 8309DFA0: 388BDC90  addi r4, r11, -0x2370
	ctx.r[4].s64 = ctx.r[11].s64 + -9072;
	// 8309DFA4: 386ABBF8  addi r3, r10, -0x4408
	ctx.r[3].s64 = ctx.r[10].s64 + -17416;
	// 8309DFA8: 4BF59B91  bl 0x82ff7b38
	ctx.lr = 0x8309DFAC;
	sub_82FF7B38(ctx, base);
	// 8309DFAC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8309DFB0: 4BF57861  bl 0x82ff5810
	ctx.lr = 0x8309DFB4;
	sub_82FF5810(ctx, base);
	// 8309DFB4: 807EBBE4  lwz r3, -0x441c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17436 as u32) ) } as u64;
	// 8309DFB8: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8309DFBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309DFC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309DFC4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309DFC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309DFCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309DFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309DFD0 size=40
    let mut pc: u32 = 0x8309DFD0;
    'dispatch: loop {
        match pc {
            0x8309DFD0 => {
    //   block [0x8309DFD0..0x8309DFF8)
	// 8309DFD0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8309DFD4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309DFD8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309DFDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309DFE0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8309DFE4: 4BF5782D  bl 0x82ff5810
	ctx.lr = 0x8309DFE8;
	sub_82FF5810(ctx, base);
	// 8309DFE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309DFEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309DFF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309DFF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309DFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8309DFF8 size=428
    let mut pc: u32 = 0x8309DFF8;
    'dispatch: loop {
        match pc {
            0x8309DFF8 => {
    //   block [0x8309DFF8..0x8309E1A4)
	// 8309DFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309DFFC: 4810A159  bl 0x831a8154
	ctx.lr = 0x8309E000;
	sub_831A8130(ctx, base);
	// 8309E000: 9421F740  stwu r1, -0x8c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-2240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309E004: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8309E008: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309E00C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8309E010: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8309E014: 4BFFFE4D  bl 0x8309de60
	ctx.lr = 0x8309E018;
	sub_8309DE60(ctx, base);
	// 8309E018: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 8309E01C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8309E020: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 8309E024: 4199000C  bgt cr6, 0x8309e030
	if ctx.cr[6].gt {
	pc = 0x8309E030; continue 'dispatch;
	}
	// 8309E028: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8309E02C: 4800002C  b 0x8309e058
	pc = 0x8309E058; continue 'dispatch;
	// 8309E030: 397EFF55  addi r11, r30, -0xab
	ctx.r[11].s64 = ctx.r[30].s64 + -171;
	// 8309E034: 2B0B0088  cmplwi cr6, r11, 0x88
	ctx.cr[6].compare_u32(ctx.r[11].u32, 136 as u32, &mut ctx.xer);
	// 8309E038: 4199000C  bgt cr6, 0x8309e044
	if ctx.cr[6].gt {
	pc = 0x8309E044; continue 'dispatch;
	}
	// 8309E03C: 3BA00002  li r29, 2
	ctx.r[29].s64 = 2;
	// 8309E040: 48000018  b 0x8309e058
	pc = 0x8309E058; continue 'dispatch;
	// 8309E044: 397EFFF7  addi r11, r30, -9
	ctx.r[11].s64 = ctx.r[30].s64 + -9;
	// 8309E048: 216B00A1  subfic r11, r11, 0xa1
	ctx.xer.ca = ctx.r[11].u32 <= 161 as u32;
	ctx.r[11].s64 = (161 as i64) - ctx.r[11].s64;
	// 8309E04C: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8309E050: 556B07BC  rlwinm r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8309E054: 3BAB0001  addi r29, r11, 1
	ctx.r[29].s64 = ctx.r[11].s64 + 1;
	// 8309E058: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8309E05C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8309E060: 388B7EB0  addi r4, r11, 0x7eb0
	ctx.r[4].s64 = ctx.r[11].s64 + 32432;
	// 8309E064: 4BF35BDD  bl 0x82fd3c40
	ctx.lr = 0x8309E068;
	sub_82FD3C40(ctx, base);
	// 8309E068: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309E06C: 41820048  beq 0x8309e0b4
	if ctx.cr[0].eq {
	pc = 0x8309E0B4; continue 'dispatch;
	}
	// 8309E070: 397EFF8D  addi r11, r30, -0x73
	ctx.r[11].s64 = ctx.r[30].s64 + -115;
	// 8309E074: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8309E078: 4199000C  bgt cr6, 0x8309e084
	if ctx.cr[6].gt {
	pc = 0x8309E084; continue 'dispatch;
	}
	// 8309E07C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8309E080: 4800002C  b 0x8309e0ac
	pc = 0x8309E0AC; continue 'dispatch;
	// 8309E084: 397EFF8B  addi r11, r30, -0x75
	ctx.r[11].s64 = ctx.r[30].s64 + -117;
	// 8309E088: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8309E08C: 4199000C  bgt cr6, 0x8309e098
	if ctx.cr[6].gt {
	pc = 0x8309E098; continue 'dispatch;
	}
	// 8309E090: 3BA00002  li r29, 2
	ctx.r[29].s64 = 2;
	// 8309E094: 48000018  b 0x8309e0ac
	pc = 0x8309E0AC; continue 'dispatch;
	// 8309E098: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 8309E09C: 216B0071  subfic r11, r11, 0x71
	ctx.xer.ca = ctx.r[11].u32 <= 113 as u32;
	ctx.r[11].s64 = (113 as i64) - ctx.r[11].s64;
	// 8309E0A0: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8309E0A4: 556B07BC  rlwinm r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8309E0A8: 3BAB0001  addi r29, r11, 1
	ctx.r[29].s64 = ctx.r[11].s64 + 1;
	// 8309E0AC: 4BFFFE85  bl 0x8309df30
	ctx.lr = 0x8309E0B0;
	sub_8309DF30(ctx, base);
	// 8309E0B0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8309E0B4: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E0B8: 38C003FF  li r6, 0x3ff
	ctx.r[6].s64 = 1023;
	// 8309E0BC: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 8309E0C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8309E0C4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309E0C8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309E0CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309E0D0: 4E800421  bctrl
	ctx.lr = 0x8309E0D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309E0D4: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309E0D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309E0DC: 419A0098  beq cr6, 0x8309e174
	if ctx.cr[6].eq {
	pc = 0x8309E174; continue 'dispatch;
	}
	// 8309E0E0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8309E0E4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E0E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309E0EC: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E0F0: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309E0F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309E0F8: 4E800421  bctrl
	ctx.lr = 0x8309E0FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309E0FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E100: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8309E104: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309E108: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309E10C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309E110: 4E800421  bctrl
	ctx.lr = 0x8309E114;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309E114: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E118: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 8309E11C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309E120: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309E124: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309E128: 4E800421  bctrl
	ctx.lr = 0x8309E12C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309E12C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E130: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 8309E134: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309E138: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309E13C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309E140: 4E800421  bctrl
	ctx.lr = 0x8309E144;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309E144: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 8309E148: 807B0008  lwz r3, 8(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309E14C: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 8309E150: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309E154: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8309E158: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8309E15C: 93210054  stw r25, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[25].u32 ) };
	// 8309E160: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8309E164: 7EE9BB78  mr r9, r23
	ctx.r[9].u64 = ctx.r[23].u64;
	// 8309E168: 7F0AC378  mr r10, r24
	ctx.r[10].u64 = ctx.r[24].u64;
	// 8309E16C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309E170: 4E800421  bctrl
	ctx.lr = 0x8309E174;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309E174: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 8309E178: 409A0024  bne cr6, 0x8309e19c
	if !ctx.cr[6].eq {
	pc = 0x8309E19C; continue 'dispatch;
	}
	// 8309E17C: 897B0004  lbz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309E180: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309E184: 41820018  beq 0x8309e19c
	if ctx.cr[0].eq {
	pc = 0x8309E19C; continue 'dispatch;
	}
	// 8309E188: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8309E18C: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 8309E190: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8309E194: 388BC6CC  addi r4, r11, -0x3934
	ctx.r[4].s64 = ctx.r[11].s64 + -14644;
	// 8309E198: 48112A91  bl 0x831b0c28
	ctx.lr = 0x8309E19C;
	sub_831B0C28(ctx, base);
	// 8309E19C: 382108C0  addi r1, r1, 0x8c0
	ctx.r[1].s64 = ctx.r[1].s64 + 2240;
	// 8309E1A0: 4810A004  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309E1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8309E1A8 size=472
    let mut pc: u32 = 0x8309E1A8;
    'dispatch: loop {
        match pc {
            0x8309E1A8 => {
    //   block [0x8309E1A8..0x8309E380)
	// 8309E1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309E1AC: 48109FA5  bl 0x831a8150
	ctx.lr = 0x8309E1B0;
	sub_831A8130(ctx, base);
	// 8309E1B0: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 8309E1B4: 9421EF30  stwu r1, -0x10d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-4304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309E1B8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8309E1BC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8309E1C0: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 8309E1C4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8309E1C8: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 8309E1CC: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 8309E1D0: 7D384B78  mr r24, r9
	ctx.r[24].u64 = ctx.r[9].u64;
	// 8309E1D4: 7D575378  mr r23, r10
	ctx.r[23].u64 = ctx.r[10].u64;
	// 8309E1D8: 4BFFFC89  bl 0x8309de60
	ctx.lr = 0x8309E1DC;
	sub_8309DE60(ctx, base);
	// 8309E1DC: 397FFFFF  addi r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	// 8309E1E0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8309E1E4: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 8309E1E8: 4199000C  bgt cr6, 0x8309e1f4
	if ctx.cr[6].gt {
	pc = 0x8309E1F4; continue 'dispatch;
	}
	// 8309E1EC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8309E1F0: 4800002C  b 0x8309e21c
	pc = 0x8309E21C; continue 'dispatch;
	// 8309E1F4: 397FFF55  addi r11, r31, -0xab
	ctx.r[11].s64 = ctx.r[31].s64 + -171;
	// 8309E1F8: 2B0B0088  cmplwi cr6, r11, 0x88
	ctx.cr[6].compare_u32(ctx.r[11].u32, 136 as u32, &mut ctx.xer);
	// 8309E1FC: 4199000C  bgt cr6, 0x8309e208
	if ctx.cr[6].gt {
	pc = 0x8309E208; continue 'dispatch;
	}
	// 8309E200: 3B800002  li r28, 2
	ctx.r[28].s64 = 2;
	// 8309E204: 48000018  b 0x8309e21c
	pc = 0x8309E21C; continue 'dispatch;
	// 8309E208: 397FFFF7  addi r11, r31, -9
	ctx.r[11].s64 = ctx.r[31].s64 + -9;
	// 8309E20C: 216B00A1  subfic r11, r11, 0xa1
	ctx.xer.ca = ctx.r[11].u32 <= 161 as u32;
	ctx.r[11].s64 = (161 as i64) - ctx.r[11].s64;
	// 8309E210: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8309E214: 556B07BC  rlwinm r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8309E218: 3B8B0001  addi r28, r11, 1
	ctx.r[28].s64 = ctx.r[11].s64 + 1;
	// 8309E21C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8309E220: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 8309E224: 388B7EB0  addi r4, r11, 0x7eb0
	ctx.r[4].s64 = ctx.r[11].s64 + 32432;
	// 8309E228: 4BF35A19  bl 0x82fd3c40
	ctx.lr = 0x8309E22C;
	sub_82FD3C40(ctx, base);
	// 8309E22C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309E230: 41820048  beq 0x8309e278
	if ctx.cr[0].eq {
	pc = 0x8309E278; continue 'dispatch;
	}
	// 8309E234: 397FFF8D  addi r11, r31, -0x73
	ctx.r[11].s64 = ctx.r[31].s64 + -115;
	// 8309E238: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8309E23C: 4199000C  bgt cr6, 0x8309e248
	if ctx.cr[6].gt {
	pc = 0x8309E248; continue 'dispatch;
	}
	// 8309E240: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8309E244: 4800002C  b 0x8309e270
	pc = 0x8309E270; continue 'dispatch;
	// 8309E248: 397FFF8B  addi r11, r31, -0x75
	ctx.r[11].s64 = ctx.r[31].s64 + -117;
	// 8309E24C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8309E250: 4199000C  bgt cr6, 0x8309e25c
	if ctx.cr[6].gt {
	pc = 0x8309E25C; continue 'dispatch;
	}
	// 8309E254: 3B800002  li r28, 2
	ctx.r[28].s64 = 2;
	// 8309E258: 48000018  b 0x8309e270
	pc = 0x8309E270; continue 'dispatch;
	// 8309E25C: 397FFFFF  addi r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	// 8309E260: 216B0071  subfic r11, r11, 0x71
	ctx.xer.ca = ctx.r[11].u32 <= 113 as u32;
	ctx.r[11].s64 = (113 as i64) - ctx.r[11].s64;
	// 8309E264: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8309E268: 556B07BC  rlwinm r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8309E26C: 3B8B0001  addi r28, r11, 1
	ctx.r[28].s64 = ctx.r[11].s64 + 1;
	// 8309E270: 4BFFFCC1  bl 0x8309df30
	ctx.lr = 0x8309E274;
	sub_8309DF30(ctx, base);
	// 8309E274: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8309E278: 81211124  lwz r9, 0x1124(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(4388 as u32) ) } as u64;
	// 8309E27C: 7EEABB78  mr r10, r23
	ctx.r[10].u64 = ctx.r[23].u64;
	// 8309E280: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E284: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 8309E288: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 8309E28C: 38C007FF  li r6, 0x7ff
	ctx.r[6].s64 = 2047;
	// 8309E290: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 8309E294: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 8309E298: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8309E29C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309E2A0: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	// 8309E2A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309E2A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309E2AC: 4E800421  bctrl
	ctx.lr = 0x8309E2B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309E2B0: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309E2B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309E2B8: 419A0098  beq cr6, 0x8309e350
	if ctx.cr[6].eq {
	pc = 0x8309E350; continue 'dispatch;
	}
	// 8309E2BC: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8309E2C0: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E2C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309E2C8: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E2CC: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309E2D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309E2D4: 4E800421  bctrl
	ctx.lr = 0x8309E2D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309E2D8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E2DC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8309E2E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309E2E4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309E2E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309E2EC: 4E800421  bctrl
	ctx.lr = 0x8309E2F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309E2F0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E2F4: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8309E2F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309E2FC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309E300: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309E304: 4E800421  bctrl
	ctx.lr = 0x8309E308;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309E308: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E30C: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 8309E310: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309E314: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309E318: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309E31C: 4E800421  bctrl
	ctx.lr = 0x8309E320;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309E320: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 8309E324: 807B0008  lwz r3, 8(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309E328: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 8309E32C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309E330: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8309E334: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 8309E338: 93410054  stw r26, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[26].u32 ) };
	// 8309E33C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8309E340: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	// 8309E344: 7F2ACB78  mr r10, r25
	ctx.r[10].u64 = ctx.r[25].u64;
	// 8309E348: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309E34C: 4E800421  bctrl
	ctx.lr = 0x8309E350;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309E350: 2F1C0002  cmpwi cr6, r28, 2
	ctx.cr[6].compare_i32(ctx.r[28].s32, 2, &mut ctx.xer);
	// 8309E354: 409A0024  bne cr6, 0x8309e378
	if !ctx.cr[6].eq {
	pc = 0x8309E378; continue 'dispatch;
	}
	// 8309E358: 897B0004  lbz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309E35C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309E360: 41820018  beq 0x8309e378
	if ctx.cr[0].eq {
	pc = 0x8309E378; continue 'dispatch;
	}
	// 8309E364: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8309E368: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 8309E36C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8309E370: 388BC6CC  addi r4, r11, -0x3934
	ctx.r[4].s64 = ctx.r[11].s64 + -14644;
	// 8309E374: 481128B5  bl 0x831b0c28
	ctx.lr = 0x8309E378;
	sub_831B0C28(ctx, base);
	// 8309E378: 382110D0  addi r1, r1, 0x10d0
	ctx.r[1].s64 = ctx.r[1].s64 + 4304;
	// 8309E37C: 48109E24  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309E380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309E380 size=8
    let mut pc: u32 = 0x8309E380;
    'dispatch: loop {
        match pc {
            0x8309E380 => {
    //   block [0x8309E380..0x8309E388)
	// 8309E380: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309E384: 8216E628  lwz r16, -0x19d8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-6616 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309E388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309E388 size=136
    let mut pc: u32 = 0x8309E388;
    'dispatch: loop {
        match pc {
            0x8309E388 => {
    //   block [0x8309E388..0x8309E410)
	// 8309E388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309E38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309E390: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309E394: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309E398: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8309E39C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309E3A0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309E3A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309E3A8: 396BE5E0  addi r11, r11, -0x1a20
	ctx.r[11].s64 = ctx.r[11].s64 + -6688;
	// 8309E3AC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8309E3B0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309E3B4: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309E3B8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309E3BC: 41820018  beq 0x8309e3d4
	if ctx.cr[0].eq {
	pc = 0x8309E3D4; continue 'dispatch;
	}
	// 8309E3C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E3C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8309E3C8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E3CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309E3D0: 4E800421  bctrl
	ctx.lr = 0x8309E3D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309E3D4: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309E3D8: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309E3DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E3E0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309E3E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309E3E8: 4E800421  bctrl
	ctx.lr = 0x8309E3EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309E3EC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309E3F0: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 8309E3F4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309E3F8: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8309E3FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309E400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309E404: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309E408: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309E40C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309E410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309E410 size=40
    let mut pc: u32 = 0x8309E410;
    'dispatch: loop {
        match pc {
            0x8309E410 => {
    //   block [0x8309E410..0x8309E438)
	// 8309E410: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8309E414: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309E418: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309E41C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309E420: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8309E424: 4BFAE33D  bl 0x8304c760
	ctx.lr = 0x8309E428;
	sub_8304C760(ctx, base);
	// 8309E428: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309E42C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309E430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309E434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309E438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309E438 size=20
    let mut pc: u32 = 0x8309E438;
    'dispatch: loop {
        match pc {
            0x8309E438 => {
    //   block [0x8309E438..0x8309E44C)
	// 8309E438: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309E43C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E440: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309E444: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309E448: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309E450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309E450 size=20
    let mut pc: u32 = 0x8309E450;
    'dispatch: loop {
        match pc {
            0x8309E450 => {
    //   block [0x8309E450..0x8309E464)
	// 8309E450: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309E454: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E458: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309E45C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309E460: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309E468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309E468 size=20
    let mut pc: u32 = 0x8309E468;
    'dispatch: loop {
        match pc {
            0x8309E468 => {
    //   block [0x8309E468..0x8309E47C)
	// 8309E468: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309E46C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E470: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309E474: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309E478: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309E480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309E480 size=96
    let mut pc: u32 = 0x8309E480;
    'dispatch: loop {
        match pc {
            0x8309E480 => {
    //   block [0x8309E480..0x8309E4E0)
	// 8309E480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309E484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309E488: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309E48C: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8309E490: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309E494: 41980030  blt cr6, 0x8309e4c4
	if ctx.cr[6].lt {
	pc = 0x8309E4C4; continue 'dispatch;
	}
	// 8309E498: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309E49C: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309E4A0: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8309E4A4: 388BE658  addi r4, r11, -0x19a8
	ctx.r[4].s64 = ctx.r[11].s64 + -6568;
	// 8309E4A8: 38A000AC  li r5, 0xac
	ctx.r[5].s64 = 172;
	// 8309E4AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8309E4B0: 4BF324A9  bl 0x82fd0958
	ctx.lr = 0x8309E4B4;
	sub_82FD0958(ctx, base);
	// 8309E4B4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8309E4B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8309E4BC: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 8309E4C0: 48112769  bl 0x831b0c28
	ctx.lr = 0x8309E4C4;
	sub_831B0C28(ctx, base);
	// 8309E4C4: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309E4C8: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8309E4CC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8309E4D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309E4D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309E4D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309E4DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309E4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309E4E0 size=96
    let mut pc: u32 = 0x8309E4E0;
    'dispatch: loop {
        match pc {
            0x8309E4E0 => {
    //   block [0x8309E4E0..0x8309E540)
	// 8309E4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309E4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309E4E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309E4EC: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8309E4F0: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309E4F4: 41980030  blt cr6, 0x8309e524
	if ctx.cr[6].lt {
	pc = 0x8309E524; continue 'dispatch;
	}
	// 8309E4F8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309E4FC: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309E500: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8309E504: 388BE658  addi r4, r11, -0x19a8
	ctx.r[4].s64 = ctx.r[11].s64 + -6568;
	// 8309E508: 38A000B6  li r5, 0xb6
	ctx.r[5].s64 = 182;
	// 8309E50C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8309E510: 4BF32449  bl 0x82fd0958
	ctx.lr = 0x8309E514;
	sub_82FD0958(ctx, base);
	// 8309E514: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8309E518: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8309E51C: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 8309E520: 48112709  bl 0x831b0c28
	ctx.lr = 0x8309E524;
	sub_831B0C28(ctx, base);
	// 8309E524: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309E528: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8309E52C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8309E530: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309E534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309E538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309E53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309E540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309E540 size=12
    let mut pc: u32 = 0x8309E540;
    'dispatch: loop {
        match pc {
            0x8309E540 => {
    //   block [0x8309E540..0x8309E54C)
	// 8309E540: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8309E544: 386B3608  addi r3, r11, 0x3608
	ctx.r[3].s64 = ctx.r[11].s64 + 13832;
	// 8309E548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309E550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309E550 size=76
    let mut pc: u32 = 0x8309E550;
    'dispatch: loop {
        match pc {
            0x8309E550 => {
    //   block [0x8309E550..0x8309E59C)
	// 8309E550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309E554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309E558: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309E55C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309E560: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309E564: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309E568: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309E56C: 4BFFFE1D  bl 0x8309e388
	ctx.lr = 0x8309E570;
	sub_8309E388(ctx, base);
	// 8309E570: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309E574: 4182000C  beq 0x8309e580
	if ctx.cr[0].eq {
	pc = 0x8309E580; continue 'dispatch;
	}
	// 8309E578: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309E57C: 4BF39D65  bl 0x82fd82e0
	ctx.lr = 0x8309E580;
	sub_82FD82E0(ctx, base);
	// 8309E580: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309E584: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309E588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309E58C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309E590: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309E594: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309E598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309E5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309E5A0 size=108
    let mut pc: u32 = 0x8309E5A0;
    'dispatch: loop {
        match pc {
            0x8309E5A0 => {
    //   block [0x8309E5A0..0x8309E60C)
	// 8309E5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309E5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309E5A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309E5AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309E5B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309E5B4: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8309E5B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8309E5BC: 4BF39CDD  bl 0x82fd8298
	ctx.lr = 0x8309E5C0;
	sub_82FD8298(ctx, base);
	// 8309E5C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309E5C4: 41820030  beq 0x8309e5f4
	if ctx.cr[0].eq {
	pc = 0x8309E5F4; continue 'dispatch;
	}
	// 8309E5C8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309E5CC: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8309E5D0: 394BE5E0  addi r10, r11, -0x1a20
	ctx.r[10].s64 = ctx.r[11].s64 + -6688;
	// 8309E5D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309E5D8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8309E5DC: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8309E5E0: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8309E5E4: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8309E5E8: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8309E5EC: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8309E5F0: 48000008  b 0x8309e5f8
	pc = 0x8309E5F8; continue 'dispatch;
	// 8309E5F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309E5F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309E5FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309E600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309E604: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309E608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309E610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309E610 size=8
    let mut pc: u32 = 0x8309E610;
    'dispatch: loop {
        match pc {
            0x8309E610 => {
    //   block [0x8309E610..0x8309E618)
	// 8309E610: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309E614: 8216E6A8  lwz r16, -0x1958(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-6488 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309E618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309E618 size=172
    let mut pc: u32 = 0x8309E618;
    'dispatch: loop {
        match pc {
            0x8309E618 => {
    //   block [0x8309E618..0x8309E6C4)
	// 8309E618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309E61C: 48109B49  bl 0x831a8164
	ctx.lr = 0x8309E620;
	sub_831A8130(ctx, base);
	// 8309E620: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8309E624: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309E628: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309E62C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8309E630: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8309E634: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 8309E638: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8309E63C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309E640: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8309E644: 937E000C  stw r27, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 8309E648: 396BE5E0  addi r11, r11, -0x1a20
	ctx.r[11].s64 = ctx.r[11].s64 + -6688;
	// 8309E64C: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8309E650: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8309E654: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8309E658: 939E0018  stw r28, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[28].u32 ) };
	// 8309E65C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309E660: 939E0008  stw r28, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 8309E664: 939E0010  stw r28, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 8309E668: 939E0014  stw r28, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 8309E66C: 4BF39C2D  bl 0x82fd8298
	ctx.lr = 0x8309E670;
	sub_82FD8298(ctx, base);
	// 8309E670: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8309E674: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309E678: 41820018  beq 0x8309e690
	if ctx.cr[0].eq {
	pc = 0x8309E690; continue 'dispatch;
	}
	// 8309E67C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8309E680: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8309E684: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8309E688: 4BF484B9  bl 0x82fe6b40
	ctx.lr = 0x8309E68C;
	sub_82FE6B40(ctx, base);
	// 8309E68C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8309E690: 939E0008  stw r28, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 8309E694: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E698: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 8309E69C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309E6A0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309E6A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309E6A8: 4E800421  bctrl
	ctx.lr = 0x8309E6AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309E6AC: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8309E6B0: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8309E6B4: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8309E6B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309E6BC: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8309E6C0: 48109AF4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309E6C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309E6C4 size=40
    let mut pc: u32 = 0x8309E6C4;
    'dispatch: loop {
        match pc {
            0x8309E6C4 => {
    //   block [0x8309E6C4..0x8309E6EC)
	// 8309E6C4: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8309E6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309E6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309E6D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309E6D4: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8309E6D8: 4BFAE089  bl 0x8304c760
	ctx.lr = 0x8309E6DC;
	sub_8304C760(ctx, base);
	// 8309E6DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309E6E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309E6E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309E6E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309E6EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309E6EC size=44
    let mut pc: u32 = 0x8309E6EC;
    'dispatch: loop {
        match pc {
            0x8309E6EC => {
    //   block [0x8309E6EC..0x8309E718)
	// 8309E6EC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8309E6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309E6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309E6F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309E6FC: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309E700: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8309E704: 4BF39BDD  bl 0x82fd82e0
	ctx.lr = 0x8309E708;
	sub_82FD82E0(ctx, base);
	// 8309E708: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309E70C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309E710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309E714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309E718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309E718 size=56
    let mut pc: u32 = 0x8309E718;
    'dispatch: loop {
        match pc {
            0x8309E718 => {
    //   block [0x8309E718..0x8309E750)
	// 8309E718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309E71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309E720: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309E724: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8309E728: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309E72C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8309E730: 4BF5BD31  bl 0x82ffa460
	ctx.lr = 0x8309E734;
	sub_82FFA460(ctx, base);
	// 8309E734: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309E738: 41820008  beq 0x8309e740
	if ctx.cr[0].eq {
	pc = 0x8309E740; continue 'dispatch;
	}
	// 8309E73C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E740: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309E744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309E748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309E74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309E750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309E750 size=8
    let mut pc: u32 = 0x8309E750;
    'dispatch: loop {
        match pc {
            0x8309E750 => {
    //   block [0x8309E750..0x8309E758)
	// 8309E750: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309E754: 8216E6F0  lwz r16, -0x1910(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-6416 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309E758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309E758 size=372
    let mut pc: u32 = 0x8309E758;
    'dispatch: loop {
        match pc {
            0x8309E758 => {
    //   block [0x8309E758..0x8309E8CC)
	// 8309E758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309E75C: 48109A09  bl 0x831a8164
	ctx.lr = 0x8309E760;
	sub_831A8130(ctx, base);
	// 8309E760: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8309E764: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309E768: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309E76C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8309E770: 48029371  bl 0x830c7ae0
	ctx.lr = 0x8309E774;
	sub_830C7AE0(ctx, base);
	// 8309E774: A97D0000  lha r11, 0(r29)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 8309E778: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8309E77C: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309E780: 41820020  beq 0x8309e7a0
	if ctx.cr[0].eq {
	pc = 0x8309E7A0; continue 'dispatch;
	}
	// 8309E784: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8309E788: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309E78C: 4BFB1455  bl 0x8304fbe0
	ctx.lr = 0x8309E790;
	sub_8304FBE0(ctx, base);
	// 8309E790: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309E794: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8309E798: 4BF5AB61  bl 0x82ff92f8
	ctx.lr = 0x8309E79C;
	sub_82FF92F8(ctx, base);
	// 8309E79C: 48000128  b 0x8309e8c4
	pc = 0x8309E8C4; continue 'dispatch;
	// 8309E7A0: 3B9E000C  addi r28, r30, 0xc
	ctx.r[28].s64 = ctx.r[30].s64 + 12;
	// 8309E7A4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8309E7A8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8309E7AC: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 8309E7B0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309E7B4: 4BFAFB95  bl 0x8304e348
	ctx.lr = 0x8309E7B8;
	sub_8304E348(ctx, base);
	// 8309E7B8: 3B7E0014  addi r27, r30, 0x14
	ctx.r[27].s64 = ctx.r[30].s64 + 20;
	// 8309E7BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309E7C0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8309E7C4: 4BF5ADB5  bl 0x82ff9578
	ctx.lr = 0x8309E7C8;
	sub_82FF9578(ctx, base);
	// 8309E7C8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309E7CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309E7D0: 409A0048  bne cr6, 0x8309e818
	if !ctx.cr[6].eq {
	pc = 0x8309E818; continue 'dispatch;
	}
	// 8309E7D4: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E7D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309E7DC: 419A003C  beq cr6, 0x8309e818
	if ctx.cr[6].eq {
	pc = 0x8309E818; continue 'dispatch;
	}
	// 8309E7E0: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309E7E4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8309E7E8: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 8309E7EC: 4BF39AAD  bl 0x82fd8298
	ctx.lr = 0x8309E7F0;
	sub_82FD8298(ctx, base);
	// 8309E7F0: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8309E7F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309E7F8: 41820018  beq 0x8309e810
	if ctx.cr[0].eq {
	pc = 0x8309E810; continue 'dispatch;
	}
	// 8309E7FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8309E800: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309E804: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E808: 4BF48339  bl 0x82fe6b40
	ctx.lr = 0x8309E80C;
	sub_82FE6B40(ctx, base);
	// 8309E80C: 48000008  b 0x8309e814
	pc = 0x8309E814; continue 'dispatch;
	// 8309E810: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309E814: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8309E818: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E81C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309E820: 419A00A4  beq cr6, 0x8309e8c4
	if ctx.cr[6].eq {
	pc = 0x8309E8C4; continue 'dispatch;
	}
	// 8309E824: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309E828: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309E82C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E830: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309E834: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309E838: 4E800421  bctrl
	ctx.lr = 0x8309E83C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309E83C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309E840: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E844: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8309E848: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E84C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309E850: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309E854: 4E800421  bctrl
	ctx.lr = 0x8309E858;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309E858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8309E85C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309E860: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8309E864: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8309E868: 915E0018  stw r10, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 8309E86C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E870: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309E874: 48000040  b 0x8309e8b4
	pc = 0x8309E8B4; continue 'dispatch;
	// 8309E878: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309E87C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E880: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309E884: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309E888: 4E800421  bctrl
	ctx.lr = 0x8309E88C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309E88C: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8309E890: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309E894: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8309E898: 7C6B512E  stwx r3, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[3].u32) };
	// 8309E89C: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8309E8A0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8309E8A4: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8309E8A8: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309E8AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309E8B0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309E8B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309E8B8: 4E800421  bctrl
	ctx.lr = 0x8309E8BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309E8BC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309E8C0: 4082FFB8  bne 0x8309e878
	if !ctx.cr[0].eq {
	pc = 0x8309E878; continue 'dispatch;
	}
	// 8309E8C4: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8309E8C8: 481098EC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309E8CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309E8CC size=44
    let mut pc: u32 = 0x8309E8CC;
    'dispatch: loop {
        match pc {
            0x8309E8CC => {
    //   block [0x8309E8CC..0x8309E8F8)
	// 8309E8CC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8309E8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309E8D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309E8D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309E8DC: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309E8E0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8309E8E4: 4BF399FD  bl 0x82fd82e0
	ctx.lr = 0x8309E8E8;
	sub_82FD82E0(ctx, base);
	// 8309E8E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309E8EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309E8F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309E8F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309E8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309E8F8 size=36
    let mut pc: u32 = 0x8309E8F8;
    'dispatch: loop {
        match pc {
            0x8309E8F8 => {
    //   block [0x8309E8F8..0x8309E91C)
	// 8309E8F8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309E8FC: 394BE740  addi r10, r11, -0x18c0
	ctx.r[10].s64 = ctx.r[11].s64 + -6336;
	// 8309E900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309E904: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8309E908: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8309E90C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8309E910: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8309E914: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8309E918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309E920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309E920 size=68
    let mut pc: u32 = 0x8309E920;
    'dispatch: loop {
        match pc {
            0x8309E920 => {
    //   block [0x8309E920..0x8309E964)
	// 8309E920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309E924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309E928: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309E92C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309E930: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309E934: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309E938: 396BDA9C  addi r11, r11, -0x2564
	ctx.r[11].s64 = ctx.r[11].s64 + -9572;
	// 8309E93C: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8309E940: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309E944: 41820008  beq 0x8309e94c
	if ctx.cr[0].eq {
	pc = 0x8309E94C; continue 'dispatch;
	}
	// 8309E948: 4BF39999  bl 0x82fd82e0
	ctx.lr = 0x8309E94C;
	sub_82FD82E0(ctx, base);
	// 8309E94C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309E950: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309E954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309E958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309E95C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309E960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309E968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309E968 size=20
    let mut pc: u32 = 0x8309E968;
    'dispatch: loop {
        match pc {
            0x8309E968 => {
    //   block [0x8309E968..0x8309E97C)
	// 8309E968: 90C30004  stw r6, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8309E96C: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 8309E970: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 8309E974: 90A30010  stw r5, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 8309E978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309E980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309E980 size=8
    let mut pc: u32 = 0x8309E980;
    'dispatch: loop {
        match pc {
            0x8309E980 => {
    //   block [0x8309E980..0x8309E988)
	// 8309E980: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309E984: 8216E770  lwz r16, -0x1890(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-6288 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309E988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309E988 size=328
    let mut pc: u32 = 0x8309E988;
    'dispatch: loop {
        match pc {
            0x8309E988 => {
    //   block [0x8309E988..0x8309EAD0)
	// 8309E988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309E98C: 481097DD  bl 0x831a8168
	ctx.lr = 0x8309E990;
	sub_831A8130(ctx, base);
	// 8309E990: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8309E994: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309E998: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309E99C: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 8309E9A0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8309E9A4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8309E9A8: B09E0002  sth r4, 2(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(2 as u32), ctx.r[4].u16 ) };
	// 8309E9AC: 90DE0008  stw r6, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 8309E9B0: 80DF00DC  lwz r6, 0xdc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 8309E9B4: 809F00E4  lwz r4, 0xe4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8309E9B8: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8309E9BC: 9BBE0000  stb r29, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 8309E9C0: 9BBE0001  stb r29, 1(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(1 as u32), ctx.r[29].u8 ) };
	// 8309E9C4: 90BE0004  stw r5, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 8309E9C8: 90FE000C  stw r7, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8309E9CC: 911E0010  stw r8, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 8309E9D0: 913E0014  stw r9, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8309E9D4: 915E0018  stw r10, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 8309E9D8: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8309E9DC: 90DE0020  stw r6, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 8309E9E0: 93BE0024  stw r29, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[29].u32 ) };
	// 8309E9E4: 93BE0028  stw r29, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 8309E9E8: 93BE0030  stw r29, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 8309E9EC: 93BE0034  stw r29, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 8309E9F0: 93BE0038  stw r29, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 8309E9F4: 93BE003C  stw r29, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[29].u32 ) };
	// 8309E9F8: 93BE002C  stw r29, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 8309E9FC: 93BE005C  stw r29, 0x5c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 8309EA00: 93BE0060  stw r29, 0x60(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 8309EA04: 909E0064  stw r4, 0x64(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(100 as u32), ctx.r[4].u32 ) };
	// 8309EA08: 4BF39891  bl 0x82fd8298
	ctx.lr = 0x8309EA0C;
	sub_82FD8298(ctx, base);
	// 8309EA0C: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8309EA10: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 8309EA14: 4182002C  beq 0x8309ea40
	if ctx.cr[0].eq {
	pc = 0x8309EA40; continue 'dispatch;
	}
	// 8309EA18: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8309EA1C: 80DE0064  lwz r6, 0x64(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 8309EA20: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8309EA24: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309EA28: 4BFC9EA1  bl 0x830688c8
	ctx.lr = 0x8309EA2C;
	sub_830688C8(ctx, base);
	// 8309EA2C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8309EA30: 394B7190  addi r10, r11, 0x7190
	ctx.r[10].s64 = ctx.r[11].s64 + 29072;
	// 8309EA34: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 8309EA38: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8309EA3C: 48000008  b 0x8309ea44
	pc = 0x8309EA44; continue 'dispatch;
	// 8309EA40: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8309EA44: 395E0040  addi r10, r30, 0x40
	ctx.r[10].s64 = ctx.r[30].s64 + 64;
	// 8309EA48: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 8309EA4C: 917E002C  stw r11, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8309EA50: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8309EA54: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8309EA58: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8309EA5C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8309EA60: 4200FFF8  bdnz 0x8309ea58
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8309EA58; continue 'dispatch;
	}
	// 8309EA64: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 8309EA68: 809E0064  lwz r4, 0x64(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 8309EA6C: 4BF3982D  bl 0x82fd8298
	ctx.lr = 0x8309EA70;
	sub_82FD8298(ctx, base);
	// 8309EA70: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8309EA74: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309EA78: 4182001C  beq 0x8309ea94
	if ctx.cr[0].eq {
	pc = 0x8309EA94; continue 'dispatch;
	}
	// 8309EA7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8309EA80: 80BE0064  lwz r5, 0x64(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 8309EA84: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8309EA88: 4BFC9D19  bl 0x830687a0
	ctx.lr = 0x8309EA8C;
	sub_830687A0(ctx, base);
	// 8309EA8C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8309EA90: 48000008  b 0x8309ea98
	pc = 0x8309EA98; continue 'dispatch;
	// 8309EA94: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8309EA98: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8309EA9C: 809E0064  lwz r4, 0x64(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 8309EAA0: 917E005C  stw r11, 0x5c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8309EAA4: 4BF397F5  bl 0x82fd8298
	ctx.lr = 0x8309EAA8;
	sub_82FD8298(ctx, base);
	// 8309EAA8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8309EAAC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309EAB0: 41820010  beq 0x8309eac0
	if ctx.cr[0].eq {
	pc = 0x8309EAC0; continue 'dispatch;
	}
	// 8309EAB4: 809E0064  lwz r4, 0x64(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 8309EAB8: 4BF773E1  bl 0x83015e98
	ctx.lr = 0x8309EABC;
	sub_83015E98(ctx, base);
	// 8309EABC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8309EAC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309EAC4: 93BE0060  stw r29, 0x60(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 8309EAC8: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8309EACC: 481096EC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309EAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309EAD0 size=48
    let mut pc: u32 = 0x8309EAD0;
    'dispatch: loop {
        match pc {
            0x8309EAD0 => {
    //   block [0x8309EAD0..0x8309EB00)
	// 8309EAD0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8309EAD4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309EAD8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309EADC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309EAE0: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8309EAE4: 808B0064  lwz r4, 0x64(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 8309EAE8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309EAEC: 4BF397F5  bl 0x82fd82e0
	ctx.lr = 0x8309EAF0;
	sub_82FD82E0(ctx, base);
	// 8309EAF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309EAF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309EAF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309EAFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309EB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309EB00 size=48
    let mut pc: u32 = 0x8309EB00;
    'dispatch: loop {
        match pc {
            0x8309EB00 => {
    //   block [0x8309EB00..0x8309EB30)
	// 8309EB00: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8309EB04: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309EB08: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309EB0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309EB10: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8309EB14: 808B0064  lwz r4, 0x64(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 8309EB18: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309EB1C: 4BF397C5  bl 0x82fd82e0
	ctx.lr = 0x8309EB20;
	sub_82FD82E0(ctx, base);
	// 8309EB20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309EB24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309EB28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309EB2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309EB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309EB30 size=48
    let mut pc: u32 = 0x8309EB30;
    'dispatch: loop {
        match pc {
            0x8309EB30 => {
    //   block [0x8309EB30..0x8309EB60)
	// 8309EB30: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8309EB34: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309EB38: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309EB3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309EB40: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8309EB44: 808B0064  lwz r4, 0x64(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 8309EB48: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309EB4C: 4BF39795  bl 0x82fd82e0
	ctx.lr = 0x8309EB50;
	sub_82FD82E0(ctx, base);
	// 8309EB50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309EB54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309EB58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309EB5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309EB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309EB60 size=372
    let mut pc: u32 = 0x8309EB60;
    'dispatch: loop {
        match pc {
            0x8309EB60 => {
    //   block [0x8309EB60..0x8309ECD4)
	// 8309EB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309EB64: 48109605  bl 0x831a8168
	ctx.lr = 0x8309EB68;
	sub_831A8130(ctx, base);
	// 8309EB68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309EB6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309EB70: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 8309EB74: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8309EB78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309EB7C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309EB80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309EB84: 4E800421  bctrl
	ctx.lr = 0x8309EB88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309EB88: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8309EB8C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309EB90: 41820018  beq 0x8309eba8
	if ctx.cr[0].eq {
	pc = 0x8309EBA8; continue 'dispatch;
	}
	// 8309EB94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309EB98: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8309EB9C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309EBA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309EBA4: 4E800421  bctrl
	ctx.lr = 0x8309EBA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309EBA8: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309EBAC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309EBB0: 41820024  beq 0x8309ebd4
	if ctx.cr[0].eq {
	pc = 0x8309EBD4; continue 'dispatch;
	}
	// 8309EBB4: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8309EBB8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309EBBC: 41820018  beq 0x8309ebd4
	if ctx.cr[0].eq {
	pc = 0x8309EBD4; continue 'dispatch;
	}
	// 8309EBC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309EBC4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8309EBC8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309EBCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309EBD0: 4E800421  bctrl
	ctx.lr = 0x8309EBD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309EBD4: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8309EBD8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309EBDC: 41820018  beq 0x8309ebf4
	if ctx.cr[0].eq {
	pc = 0x8309EBF4; continue 'dispatch;
	}
	// 8309EBE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309EBE4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8309EBE8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309EBEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309EBF0: 4E800421  bctrl
	ctx.lr = 0x8309EBF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309EBF4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8309EBF8: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8309EBFC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309EC00: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 8309EC04: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 8309EC08: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 8309EC0C: 4182000C  beq 0x8309ec18
	if ctx.cr[0].eq {
	pc = 0x8309EC18; continue 'dispatch;
	}
	// 8309EC10: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8309EC14: 4BFFBB05  bl 0x8309a718
	ctx.lr = 0x8309EC18;
	sub_8309A718(ctx, base);
	// 8309EC18: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8309EC1C: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 8309EC20: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309EC24: 4182000C  beq 0x8309ec30
	if ctx.cr[0].eq {
	pc = 0x8309EC30; continue 'dispatch;
	}
	// 8309EC28: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8309EC2C: 4BFFBAED  bl 0x8309a718
	ctx.lr = 0x8309EC30;
	sub_8309A718(ctx, base);
	// 8309EC30: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8309EC34: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 8309EC38: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309EC3C: 4182000C  beq 0x8309ec48
	if ctx.cr[0].eq {
	pc = 0x8309EC48; continue 'dispatch;
	}
	// 8309EC40: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8309EC44: 4BFFBAD5  bl 0x8309a718
	ctx.lr = 0x8309EC48;
	sub_8309A718(ctx, base);
	// 8309EC48: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 8309EC4C: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 8309EC50: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309EC54: 4182000C  beq 0x8309ec60
	if ctx.cr[0].eq {
	pc = 0x8309EC60; continue 'dispatch;
	}
	// 8309EC58: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8309EC5C: 4BFFBABD  bl 0x8309a718
	ctx.lr = 0x8309EC60;
	sub_8309A718(ctx, base);
	// 8309EC60: 3BBF0040  addi r29, r31, 0x40
	ctx.r[29].s64 = ctx.r[31].s64 + 64;
	// 8309EC64: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 8309EC68: 3B800007  li r28, 7
	ctx.r[28].s64 = 7;
	// 8309EC6C: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309EC70: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309EC74: 4182000C  beq 0x8309ec80
	if ctx.cr[0].eq {
	pc = 0x8309EC80; continue 'dispatch;
	}
	// 8309EC78: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8309EC7C: 4BFFBA9D  bl 0x8309a718
	ctx.lr = 0x8309EC80;
	sub_8309A718(ctx, base);
	// 8309EC80: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8309EC84: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8309EC88: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8309EC8C: 4082FFE0  bne 0x8309ec6c
	if !ctx.cr[0].eq {
	pc = 0x8309EC6C; continue 'dispatch;
	}
	// 8309EC90: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8309EC94: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309EC98: 4182000C  beq 0x8309eca4
	if ctx.cr[0].eq {
	pc = 0x8309ECA4; continue 'dispatch;
	}
	// 8309EC9C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8309ECA0: 4BFFBA79  bl 0x8309a718
	ctx.lr = 0x8309ECA4;
	sub_8309A718(ctx, base);
	// 8309ECA4: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8309ECA8: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8309ECAC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309ECB0: 41820018  beq 0x8309ecc8
	if ctx.cr[0].eq {
	pc = 0x8309ECC8; continue 'dispatch;
	}
	// 8309ECB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309ECB8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8309ECBC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309ECC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309ECC4: 4E800421  bctrl
	ctx.lr = 0x8309ECC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309ECC8: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 8309ECCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8309ECD0: 481094E8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309ECD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309ECD8 size=8
    let mut pc: u32 = 0x8309ECD8;
    'dispatch: loop {
        match pc {
            0x8309ECD8 => {
    //   block [0x8309ECD8..0x8309ECE0)
	// 8309ECD8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309ECDC: 8216E7D0  lwz r16, -0x1830(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-6192 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309ECE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309ECE0 size=696
    let mut pc: u32 = 0x8309ECE0;
    'dispatch: loop {
        match pc {
            0x8309ECE0 => {
    //   block [0x8309ECE0..0x8309EF98)
	// 8309ECE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309ECE4: 4810946D  bl 0x831a8150
	ctx.lr = 0x8309ECE8;
	sub_831A8130(ctx, base);
	// 8309ECE8: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 8309ECEC: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309ECF0: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8309ECF4: 549D043E  clrlwi r29, r4, 0x10
	ctx.r[29].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 8309ECF8: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 8309ECFC: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 8309ED00: 2B1D0007  cmplwi cr6, r29, 7
	ctx.cr[6].compare_u32(ctx.r[29].u32, 7 as u32, &mut ctx.xer);
	// 8309ED04: 935F00C4  stw r26, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[26].u32 ) };
	// 8309ED08: 4098027C  bge cr6, 0x8309ef84
	if !ctx.cr[6].lt {
	pc = 0x8309EF84; continue 'dispatch;
	}
	// 8309ED0C: 807A0020  lwz r3, 0x20(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(32 as u32) ) } as u64;
	// 8309ED10: 48000451  bl 0x8309f160
	ctx.lr = 0x8309ED14;
	sub_8309F160(ctx, base);
	// 8309ED14: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8309ED18: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8309ED1C: 41820268  beq 0x8309ef84
	if ctx.cr[0].eq {
	pc = 0x8309EF84; continue 'dispatch;
	}
	// 8309ED20: 397D0010  addi r11, r29, 0x10
	ctx.r[11].s64 = ctx.r[29].s64 + 16;
	// 8309ED24: 557D103A  slwi r29, r11, 2
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 8309ED28: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309ED2C: 3B0BD218  addi r24, r11, -0x2de8
	ctx.r[24].s64 = ctx.r[11].s64 + -11752;
	// 8309ED30: 7F9DD02E  lwzx r28, r29, r26
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 8309ED34: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309ED38: 408200E0  bne 0x8309ee18
	if !ctx.cr[0].eq {
	pc = 0x8309EE18; continue 'dispatch;
	}
	// 8309ED3C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 8309ED40: 809A0064  lwz r4, 0x64(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(100 as u32) ) } as u64;
	// 8309ED44: 4BF39555  bl 0x82fd8298
	ctx.lr = 0x8309ED48;
	sub_82FD8298(ctx, base);
	// 8309ED48: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8309ED4C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309ED50: 41820018  beq 0x8309ed68
	if ctx.cr[0].eq {
	pc = 0x8309ED68; continue 'dispatch;
	}
	// 8309ED54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8309ED58: 80BA0064  lwz r5, 0x64(r26)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(100 as u32) ) } as u64;
	// 8309ED5C: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 8309ED60: 4BFC9A41  bl 0x830687a0
	ctx.lr = 0x8309ED64;
	sub_830687A0(ctx, base);
	// 8309ED64: 48000008  b 0x8309ed6c
	pc = 0x8309ED6C; continue 'dispatch;
	// 8309ED68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309ED6C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8309ED70: 7C7DD12E  stwx r3, r29, r26
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[26].u32), ctx.r[3].u32) };
	// 8309ED74: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309ED78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309ED7C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309ED80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309ED84: 4E800421  bctrl
	ctx.lr = 0x8309ED88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309ED88: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8309ED8C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309ED90: 3B2BCE8C  addi r25, r11, -0x3174
	ctx.r[25].s64 = ctx.r[11].s64 + -12660;
	// 8309ED94: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309ED98: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 8309ED9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309EDA0: 4E800421  bctrl
	ctx.lr = 0x8309EDA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309EDA4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8309EDA8: 4BF34E99  bl 0x82fd3c40
	ctx.lr = 0x8309EDAC;
	sub_82FD3C40(ctx, base);
	// 8309EDAC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309EDB0: 40820008  bne 0x8309edb8
	if !ctx.cr[0].eq {
	pc = 0x8309EDB8; continue 'dispatch;
	}
	// 8309EDB4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8309EDB8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309EDBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309EDC0: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 8309EDC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309EDC8: 4E800421  bctrl
	ctx.lr = 0x8309EDCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309EDCC: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8309EDD0: 4BF34E71  bl 0x82fd3c40
	ctx.lr = 0x8309EDD4;
	sub_82FD3C40(ctx, base);
	// 8309EDD4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309EDD8: 4182009C  beq 0x8309ee74
	if ctx.cr[0].eq {
	pc = 0x8309EE74; continue 'dispatch;
	}
	// 8309EDDC: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8309EDE0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309EDE4: 4BF47E25  bl 0x82fe6c08
	ctx.lr = 0x8309EDE8;
	sub_82FE6C08(ctx, base);
	// 8309EDE8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309EDEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309EDF0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8309EDF4: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 8309EDF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309EDFC: 4E800421  bctrl
	ctx.lr = 0x8309EE00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309EE00: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8309EE04: 4BF34E3D  bl 0x82fd3c40
	ctx.lr = 0x8309EE08;
	sub_82FD3C40(ctx, base);
	// 8309EE08: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309EE0C: 41820144  beq 0x8309ef50
	if ctx.cr[0].eq {
	pc = 0x8309EF50; continue 'dispatch;
	}
	// 8309EE10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309EE14: 48000174  b 0x8309ef88
	pc = 0x8309EF88; continue 'dispatch;
	// 8309EE18: 837C0004  lwz r27, 4(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309EE1C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8309EE20: 281B0000  cmplwi r27, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309EE24: 4182FF50  beq 0x8309ed74
	if ctx.cr[0].eq {
	pc = 0x8309ED74; continue 'dispatch;
	}
	// 8309EE28: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8309EE2C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309EE30: 4BF592A1  bl 0x82ff80d0
	ctx.lr = 0x8309EE34;
	sub_82FF80D0(ctx, base);
	// 8309EE34: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309EE38: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8309EE3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309EE40: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309EE44: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8309EE48: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 8309EE4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309EE50: 4E800421  bctrl
	ctx.lr = 0x8309EE54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309EE54: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8309EE58: 4BF34DE9  bl 0x82fd3c40
	ctx.lr = 0x8309EE5C;
	sub_82FD3C40(ctx, base);
	// 8309EE5C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309EE60: 4082FFB0  bne 0x8309ee10
	if !ctx.cr[0].eq {
	pc = 0x8309EE10; continue 'dispatch;
	}
	// 8309EE64: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8309EE68: 7F1DD840  cmplw cr6, r29, r27
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8309EE6C: 4198FFBC  blt cr6, 0x8309ee28
	if ctx.cr[6].lt {
	pc = 0x8309EE28; continue 'dispatch;
	}
	// 8309EE70: 4BFFFF04  b 0x8309ed74
	pc = 0x8309ED74; continue 'dispatch;
	// 8309EE74: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309EE78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309EE7C: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 8309EE80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309EE84: 4E800421  bctrl
	ctx.lr = 0x8309EE88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309EE88: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8309EE8C: 4BF34DB5  bl 0x82fd3c40
	ctx.lr = 0x8309EE90;
	sub_82FD3C40(ctx, base);
	// 8309EE90: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309EE94: 418200BC  beq 0x8309ef50
	if ctx.cr[0].eq {
	pc = 0x8309EF50; continue 'dispatch;
	}
	// 8309EE98: 807A0030  lwz r3, 0x30(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(48 as u32) ) } as u64;
	// 8309EE9C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309EEA0: 41820018  beq 0x8309eeb8
	if ctx.cr[0].eq {
	pc = 0x8309EEB8; continue 'dispatch;
	}
	// 8309EEA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8309EEA8: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8309EEAC: 4BFC98A5  bl 0x83068750
	ctx.lr = 0x8309EEB0;
	sub_83068750(ctx, base);
	// 8309EEB0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309EEB4: 4082009C  bne 0x8309ef50
	if !ctx.cr[0].eq {
	pc = 0x8309EF50; continue 'dispatch;
	}
	// 8309EEB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309EEBC: 480002A5  bl 0x8309f160
	ctx.lr = 0x8309EEC0;
	sub_8309F160(ctx, base);
	// 8309EEC0: 48000084  b 0x8309ef44
	pc = 0x8309EF44; continue 'dispatch;
	// 8309EEC4: 807A0030  lwz r3, 0x30(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(48 as u32) ) } as u64;
	// 8309EEC8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309EECC: 41820018  beq 0x8309eee4
	if ctx.cr[0].eq {
	pc = 0x8309EEE4; continue 'dispatch;
	}
	// 8309EED0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8309EED4: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8309EED8: 4BFC9879  bl 0x83068750
	ctx.lr = 0x8309EEDC;
	sub_83068750(ctx, base);
	// 8309EEDC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309EEE0: 4082005C  bne 0x8309ef3c
	if !ctx.cr[0].eq {
	pc = 0x8309EF3C; continue 'dispatch;
	}
	// 8309EEE4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309EEE8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309EEEC: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 8309EEF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309EEF4: 4E800421  bctrl
	ctx.lr = 0x8309EEF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309EEF8: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8309EEFC: 4BF34D45  bl 0x82fd3c40
	ctx.lr = 0x8309EF00;
	sub_82FD3C40(ctx, base);
	// 8309EF00: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309EF04: 41820038  beq 0x8309ef3c
	if ctx.cr[0].eq {
	pc = 0x8309EF3C; continue 'dispatch;
	}
	// 8309EF08: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8309EF0C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309EF10: 4BF47CF9  bl 0x82fe6c08
	ctx.lr = 0x8309EF14;
	sub_82FE6C08(ctx, base);
	// 8309EF14: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309EF18: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309EF1C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8309EF20: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 8309EF24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309EF28: 4E800421  bctrl
	ctx.lr = 0x8309EF2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309EF2C: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8309EF30: 4BF34D11  bl 0x82fd3c40
	ctx.lr = 0x8309EF34;
	sub_82FD3C40(ctx, base);
	// 8309EF34: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309EF38: 40820058  bne 0x8309ef90
	if !ctx.cr[0].eq {
	pc = 0x8309EF90; continue 'dispatch;
	}
	// 8309EF3C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309EF40: 48000371  bl 0x8309f2b0
	ctx.lr = 0x8309EF44;
	sub_8309F2B0(ctx, base);
	// 8309EF44: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8309EF48: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8309EF4C: 4082FF78  bne 0x8309eec4
	if !ctx.cr[0].eq {
	pc = 0x8309EEC4; continue 'dispatch;
	}
	// 8309EF50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309EF54: 4800035D  bl 0x8309f2b0
	ctx.lr = 0x8309EF58;
	sub_8309F2B0(ctx, base);
	// 8309EF58: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8309EF5C: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8309EF60: 4082FE58  bne 0x8309edb8
	if !ctx.cr[0].eq {
	pc = 0x8309EDB8; continue 'dispatch;
	}
	// 8309EF64: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8309EF68: 419A001C  beq cr6, 0x8309ef84
	if ctx.cr[6].eq {
	pc = 0x8309EF84; continue 'dispatch;
	}
	// 8309EF6C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8309EF70: 48000341  bl 0x8309f2b0
	ctx.lr = 0x8309EF74;
	sub_8309F2B0(ctx, base);
	// 8309EF74: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8309EF78: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8309EF7C: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8309EF80: 4082FE38  bne 0x8309edb8
	if !ctx.cr[0].eq {
	pc = 0x8309EDB8; continue 'dispatch;
	}
	// 8309EF84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309EF88: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 8309EF8C: 48109214  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
	// 8309EF90: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309EF94: 4BFFFFF4  b 0x8309ef88
	pc = 0x8309EF88; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309EF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309EF98 size=48
    let mut pc: u32 = 0x8309EF98;
    'dispatch: loop {
        match pc {
            0x8309EF98 => {
    //   block [0x8309EF98..0x8309EFC8)
	// 8309EF98: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 8309EF9C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309EFA0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309EFA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309EFA8: 817F00C4  lwz r11, 0xc4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8309EFAC: 808B0064  lwz r4, 0x64(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 8309EFB0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8309EFB4: 4BF3932D  bl 0x82fd82e0
	ctx.lr = 0x8309EFB8;
	sub_82FD82E0(ctx, base);
	// 8309EFB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309EFBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309EFC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309EFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309EFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309EFC8 size=244
    let mut pc: u32 = 0x8309EFC8;
    'dispatch: loop {
        match pc {
            0x8309EFC8 => {
    //   block [0x8309EFC8..0x8309F0BC)
	// 8309EFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309EFCC: 4810919D  bl 0x831a8168
	ctx.lr = 0x8309EFD0;
	sub_831A8130(ctx, base);
	// 8309EFD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309EFD4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309EFD8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8309EFDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8309EFE0: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 8309EFE4: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309EFE8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309EFEC: 41820024  beq 0x8309f010
	if ctx.cr[0].eq {
	pc = 0x8309F010; continue 'dispatch;
	}
	// 8309EFF0: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309EFF4: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309EFF8: 7F08E840  cmplw cr6, r8, r29
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8309EFFC: 419A00B0  beq cr6, 0x8309f0ac
	if ctx.cr[6].eq {
	pc = 0x8309F0AC; continue 'dispatch;
	}
	// 8309F000: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8309F004: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8309F008: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8309F00C: 4198FFE8  blt cr6, 0x8309eff4
	if ctx.cr[6].lt {
	pc = 0x8309EFF4; continue 'dispatch;
	}
	// 8309F010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309F014: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309F018: 4082000C  bne 0x8309f024
	if !ctx.cr[0].eq {
	pc = 0x8309F024; continue 'dispatch;
	}
	// 8309F01C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8309F020: 4BF9C131  bl 0x8303b150
	ctx.lr = 0x8309F024;
	sub_8303B150(ctx, base);
	// 8309F024: 817D002C  lwz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8309F028: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8309F02C: 838B0008  lwz r28, 8(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309F030: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309F034: 41820070  beq 0x8309f0a4
	if ctx.cr[0].eq {
	pc = 0x8309F0A4; continue 'dispatch;
	}
	// 8309F038: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8309F03C: 807D002C  lwz r3, 0x2c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 8309F040: 4BF8D831  bl 0x8302c870
	ctx.lr = 0x8309F044;
	sub_8302C870(ctx, base);
	// 8309F044: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8309F048: 7F04F040  cmplw cr6, r4, r30
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8309F04C: 419A004C  beq cr6, 0x8309f098
	if ctx.cr[6].eq {
	pc = 0x8309F098; continue 'dispatch;
	}
	// 8309F050: 811E002C  lwz r8, 0x2c(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 8309F054: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309F058: 81280008  lwz r9, 8(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309F05C: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309F060: 41820024  beq 0x8309f084
	if ctx.cr[0].eq {
	pc = 0x8309F084; continue 'dispatch;
	}
	// 8309F064: 81480010  lwz r10, 0x10(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309F068: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309F06C: 7F072040  cmplw cr6, r7, r4
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[4].u32, &mut ctx.xer);
	// 8309F070: 419A0044  beq cr6, 0x8309f0b4
	if ctx.cr[6].eq {
	pc = 0x8309F0B4; continue 'dispatch;
	}
	// 8309F074: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8309F078: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8309F07C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8309F080: 4198FFE8  blt cr6, 0x8309f068
	if ctx.cr[6].lt {
	pc = 0x8309F068; continue 'dispatch;
	}
	// 8309F084: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309F088: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309F08C: 4082000C  bne 0x8309f098
	if !ctx.cr[0].eq {
	pc = 0x8309F098; continue 'dispatch;
	}
	// 8309F090: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 8309F094: 4BF9C0BD  bl 0x8303b150
	ctx.lr = 0x8309F098;
	sub_8303B150(ctx, base);
	// 8309F098: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8309F09C: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8309F0A0: 4198FF98  blt cr6, 0x8309f038
	if ctx.cr[6].lt {
	pc = 0x8309F038; continue 'dispatch;
	}
	// 8309F0A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8309F0A8: 48109110  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8309F0AC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8309F0B0: 4BFFFF64  b 0x8309f014
	pc = 0x8309F014; continue 'dispatch;
	// 8309F0B4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8309F0B8: 4BFFFFD0  b 0x8309f088
	pc = 0x8309F088; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309F0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309F0C0 size=160
    let mut pc: u32 = 0x8309F0C0;
    'dispatch: loop {
        match pc {
            0x8309F0C0 => {
    //   block [0x8309F0C0..0x8309F160)
	// 8309F0C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309F0C4: 48109091  bl 0x831a8154
	ctx.lr = 0x8309F0C8;
	sub_831A8130(ctx, base);
	// 8309F0C8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309F0CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8309F0D0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8309F0D4: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8309F0D8: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8309F0DC: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 8309F0E0: 4BFFFC01  bl 0x8309ece0
	ctx.lr = 0x8309F0E4;
	sub_8309ECE0(ctx, base);
	// 8309F0E4: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8309F0E8: 4082006C  bne 0x8309f154
	if !ctx.cr[0].eq {
	pc = 0x8309F154; continue 'dispatch;
	}
	// 8309F0EC: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8309F0F0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309F0F4: 4182000C  beq 0x8309f100
	if ctx.cr[0].eq {
	pc = 0x8309F100; continue 'dispatch;
	}
	// 8309F0F8: 830B0008  lwz r24, 8(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309F0FC: 48000008  b 0x8309f104
	pc = 0x8309F104; continue 'dispatch;
	// 8309F100: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 8309F104: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8309F108: 28180000  cmplwi r24, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309F10C: 41820048  beq 0x8309f154
	if ctx.cr[0].eq {
	pc = 0x8309F154; continue 'dispatch;
	}
	// 8309F110: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8309F114: 807D0024  lwz r3, 0x24(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8309F118: 4BF8D759  bl 0x8302c870
	ctx.lr = 0x8309F11C;
	sub_8302C870(ctx, base);
	// 8309F11C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309F120: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8309F124: 419A001C  beq cr6, 0x8309f140
	if ctx.cr[6].eq {
	pc = 0x8309F140; continue 'dispatch;
	}
	// 8309F128: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 8309F12C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8309F130: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8309F134: 4BFFFBAD  bl 0x8309ece0
	ctx.lr = 0x8309F138;
	sub_8309ECE0(ctx, base);
	// 8309F138: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8309F13C: 40820014  bne 0x8309f150
	if !ctx.cr[0].eq {
	pc = 0x8309F150; continue 'dispatch;
	}
	// 8309F140: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8309F144: 7F1EC040  cmplw cr6, r30, r24
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[24].u32, &mut ctx.xer);
	// 8309F148: 4198FFC8  blt cr6, 0x8309f110
	if ctx.cr[6].lt {
	pc = 0x8309F110; continue 'dispatch;
	}
	// 8309F14C: 48000008  b 0x8309f154
	pc = 0x8309F154; continue 'dispatch;
	// 8309F150: 93F70000  stw r31, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8309F154: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309F158: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8309F15C: 48109048  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309F160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309F160 size=112
    let mut pc: u32 = 0x8309F160;
    'dispatch: loop {
        match pc {
            0x8309F160 => {
    //   block [0x8309F160..0x8309F1D0)
	// 8309F160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309F164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309F168: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309F16C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309F170: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309F174: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8309F178: 48000030  b 0x8309f1a8
	pc = 0x8309F1A8; continue 'dispatch;
	// 8309F17C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309F180: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309F184: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309F188: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309F18C: 4E800421  bctrl
	ctx.lr = 0x8309F190;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309F190: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 8309F194: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309F198: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8309F19C: 419A0020  beq cr6, 0x8309f1bc
	if ctx.cr[6].eq {
	pc = 0x8309F1BC; continue 'dispatch;
	}
	// 8309F1A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309F1A4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8309F1A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309F1AC: 4E800421  bctrl
	ctx.lr = 0x8309F1B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309F1B0: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8309F1B4: 4082FFC8  bne 0x8309f17c
	if !ctx.cr[0].eq {
	pc = 0x8309F17C; continue 'dispatch;
	}
	// 8309F1B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309F1BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309F1C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309F1C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309F1C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309F1CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309F1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309F1D0 size=224
    let mut pc: u32 = 0x8309F1D0;
    'dispatch: loop {
        match pc {
            0x8309F1D0 => {
    //   block [0x8309F1D0..0x8309F2B0)
	// 8309F1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309F1D4: 48108F89  bl 0x831a815c
	ctx.lr = 0x8309F1D8;
	sub_831A8130(ctx, base);
	// 8309F1D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309F1DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309F1E0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8309F1E4: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8309F1E8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8309F1EC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8309F1F0: 4800009C  b 0x8309f28c
	pc = 0x8309F28C; continue 'dispatch;
	// 8309F1F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309F1F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309F1FC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309F200: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309F204: 4E800421  bctrl
	ctx.lr = 0x8309F208;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309F208: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 8309F20C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8309F210: 409A0070  bne cr6, 0x8309f280
	if !ctx.cr[6].eq {
	pc = 0x8309F280; continue 'dispatch;
	}
	// 8309F214: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8309F218: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8309F21C: 419A0064  beq cr6, 0x8309f280
	if ctx.cr[6].eq {
	pc = 0x8309F280; continue 'dispatch;
	}
	// 8309F220: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 8309F224: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309F228: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309F22C: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 8309F230: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309F234: 4E800421  bctrl
	ctx.lr = 0x8309F238;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309F238: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8309F23C: 4BF34A05  bl 0x82fd3c40
	ctx.lr = 0x8309F240;
	sub_82FD3C40(ctx, base);
	// 8309F240: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309F244: 4182002C  beq 0x8309f270
	if ctx.cr[0].eq {
	pc = 0x8309F270; continue 'dispatch;
	}
	// 8309F248: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309F24C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309F250: 833E0000  lwz r25, 0(r30)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309F254: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 8309F258: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309F25C: 4E800421  bctrl
	ctx.lr = 0x8309F260;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309F260: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8309F264: 4BF349DD  bl 0x82fd3c40
	ctx.lr = 0x8309F268;
	sub_82FD3C40(ctx, base);
	// 8309F268: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309F26C: 4082003C  bne 0x8309f2a8
	if !ctx.cr[0].eq {
	pc = 0x8309F2A8; continue 'dispatch;
	}
	// 8309F270: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8309F274: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8309F278: 7F1DE040  cmplw cr6, r29, r28
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8309F27C: 4198FFA8  blt cr6, 0x8309f224
	if ctx.cr[6].lt {
	pc = 0x8309F224; continue 'dispatch;
	}
	// 8309F280: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309F284: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309F288: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8309F28C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309F290: 4E800421  bctrl
	ctx.lr = 0x8309F294;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309F294: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8309F298: 4082FF5C  bne 0x8309f1f4
	if !ctx.cr[0].eq {
	pc = 0x8309F1F4; continue 'dispatch;
	}
	// 8309F29C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309F2A0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8309F2A4: 48108F08  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 8309F2A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309F2AC: 4BFFFFF4  b 0x8309f2a0
	pc = 0x8309F2A0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309F2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309F2B0 size=108
    let mut pc: u32 = 0x8309F2B0;
    'dispatch: loop {
        match pc {
            0x8309F2B0 => {
    //   block [0x8309F2B0..0x8309F31C)
	// 8309F2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309F2B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309F2B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309F2BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309F2C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309F2C4: 4800002C  b 0x8309f2f0
	pc = 0x8309F2F0; continue 'dispatch;
	// 8309F2C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309F2CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309F2D0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309F2D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309F2D8: 4E800421  bctrl
	ctx.lr = 0x8309F2DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309F2DC: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 8309F2E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309F2E4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8309F2E8: 419A0020  beq cr6, 0x8309f308
	if ctx.cr[6].eq {
	pc = 0x8309F308; continue 'dispatch;
	}
	// 8309F2EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309F2F0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8309F2F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309F2F8: 4E800421  bctrl
	ctx.lr = 0x8309F2FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309F2FC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8309F300: 4082FFC8  bne 0x8309f2c8
	if !ctx.cr[0].eq {
	pc = 0x8309F2C8; continue 'dispatch;
	}
	// 8309F304: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309F308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309F30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309F310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309F314: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309F318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309F320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309F320 size=220
    let mut pc: u32 = 0x8309F320;
    'dispatch: loop {
        match pc {
            0x8309F320 => {
    //   block [0x8309F320..0x8309F3FC)
	// 8309F320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309F324: 48108E39  bl 0x831a815c
	ctx.lr = 0x8309F328;
	sub_831A8130(ctx, base);
	// 8309F328: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309F32C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309F330: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8309F334: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8309F338: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8309F33C: 48000098  b 0x8309f3d4
	pc = 0x8309F3D4; continue 'dispatch;
	// 8309F340: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309F344: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309F348: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309F34C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309F350: 4E800421  bctrl
	ctx.lr = 0x8309F354;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309F354: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 8309F358: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8309F35C: 409A0070  bne cr6, 0x8309f3cc
	if !ctx.cr[6].eq {
	pc = 0x8309F3CC; continue 'dispatch;
	}
	// 8309F360: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8309F364: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8309F368: 419A0064  beq cr6, 0x8309f3cc
	if ctx.cr[6].eq {
	pc = 0x8309F3CC; continue 'dispatch;
	}
	// 8309F36C: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 8309F370: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309F374: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309F378: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 8309F37C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309F380: 4E800421  bctrl
	ctx.lr = 0x8309F384;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309F384: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8309F388: 4BF348B9  bl 0x82fd3c40
	ctx.lr = 0x8309F38C;
	sub_82FD3C40(ctx, base);
	// 8309F38C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309F390: 4182002C  beq 0x8309f3bc
	if ctx.cr[0].eq {
	pc = 0x8309F3BC; continue 'dispatch;
	}
	// 8309F394: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309F398: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309F39C: 833E0000  lwz r25, 0(r30)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309F3A0: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 8309F3A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309F3A8: 4E800421  bctrl
	ctx.lr = 0x8309F3AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309F3AC: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8309F3B0: 4BF34891  bl 0x82fd3c40
	ctx.lr = 0x8309F3B4;
	sub_82FD3C40(ctx, base);
	// 8309F3B4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309F3B8: 4082003C  bne 0x8309f3f4
	if !ctx.cr[0].eq {
	pc = 0x8309F3F4; continue 'dispatch;
	}
	// 8309F3BC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8309F3C0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8309F3C4: 7F1DE040  cmplw cr6, r29, r28
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8309F3C8: 4198FFA8  blt cr6, 0x8309f370
	if ctx.cr[6].lt {
	pc = 0x8309F370; continue 'dispatch;
	}
	// 8309F3CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309F3D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309F3D4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8309F3D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309F3DC: 4E800421  bctrl
	ctx.lr = 0x8309F3E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309F3E0: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8309F3E4: 4082FF5C  bne 0x8309f340
	if !ctx.cr[0].eq {
	pc = 0x8309F340; continue 'dispatch;
	}
	// 8309F3E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309F3EC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8309F3F0: 48108DBC  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 8309F3F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309F3F8: 4BFFFFF4  b 0x8309f3ec
	pc = 0x8309F3EC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309F400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309F400 size=8
    let mut pc: u32 = 0x8309F400;
    'dispatch: loop {
        match pc {
            0x8309F400 => {
    //   block [0x8309F400..0x8309F408)
	// 8309F400: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 8309F404: 4BFA5974  b 0x83044d78
	sub_83044D78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309F408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309F408 size=8
    let mut pc: u32 = 0x8309F408;
    'dispatch: loop {
        match pc {
            0x8309F408 => {
    //   block [0x8309F408..0x8309F410)
	// 8309F408: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309F40C: 8216E8DC  lwz r16, -0x1724(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-5924 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309F410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309F410 size=680
    let mut pc: u32 = 0x8309F410;
    'dispatch: loop {
        match pc {
            0x8309F410 => {
    //   block [0x8309F410..0x8309F6B8)
	// 8309F410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309F414: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 8309F418: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 8309F41C: 48108D49  bl 0x831a8164
	ctx.lr = 0x8309F420;
	sub_831A8130(ctx, base);
	// 8309F420: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8309F424: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309F428: 81480078  lwz r10, 0x78(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(120 as u32) ) } as u64;
	// 8309F42C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8309F430: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8309F434: 909F00AC  stw r4, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[4].u32 ) };
	// 8309F438: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8309F43C: 90BF00B4  stw r5, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[5].u32 ) };
	// 8309F440: 7CEB0734  extsh r11, r7
	ctx.r[11].s64 = ctx.r[7].s16 as i64;
	// 8309F444: 911F00CC  stw r8, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[8].u32 ) };
	// 8309F448: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8309F44C: 814A0060  lwz r10, 0x60(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(96 as u32) ) } as u64;
	// 8309F450: 2F0B0080  cmpwi cr6, r11, 0x80
	ctx.cr[6].compare_i32(ctx.r[11].s32, 128, &mut ctx.xer);
	// 8309F454: 93DF00BC  stw r30, 0xbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[30].u32 ) };
	// 8309F458: 9B7F0050  stb r27, 0x50(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[27].u8 ) };
	// 8309F45C: 915C0004  stw r10, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8309F460: 419900B8  bgt cr6, 0x8309f518
	if ctx.cr[6].gt {
	pc = 0x8309F518; continue 'dispatch;
	}
	// 8309F464: 419A008C  beq cr6, 0x8309f4f0
	if ctx.cr[6].eq {
	pc = 0x8309F4F0; continue 'dispatch;
	}
	// 8309F468: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8309F46C: 419A0078  beq cr6, 0x8309f4e4
	if ctx.cr[6].eq {
	pc = 0x8309F4E4; continue 'dispatch;
	}
	// 8309F470: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 8309F474: 419A0098  beq cr6, 0x8309f50c
	if ctx.cr[6].eq {
	pc = 0x8309F50C; continue 'dispatch;
	}
	// 8309F478: 2F0B0010  cmpwi cr6, r11, 0x10
	ctx.cr[6].compare_i32(ctx.r[11].s32, 16, &mut ctx.xer);
	// 8309F47C: 419A005C  beq cr6, 0x8309f4d8
	if ctx.cr[6].eq {
	pc = 0x8309F4D8; continue 'dispatch;
	}
	// 8309F480: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 8309F484: 419A0044  beq cr6, 0x8309f4c8
	if ctx.cr[6].eq {
	pc = 0x8309F4C8; continue 'dispatch;
	}
	// 8309F488: 2F0B0040  cmpwi cr6, r11, 0x40
	ctx.cr[6].compare_i32(ctx.r[11].s32, 64, &mut ctx.xer);
	// 8309F48C: 409A01F4  bne cr6, 0x8309f680
	if !ctx.cr[6].eq {
	pc = 0x8309F680; continue 'dispatch;
	}
	// 8309F490: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F494: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309F498: 388BD4B0  addi r4, r11, -0x2b50
	ctx.r[4].s64 = ctx.r[11].s64 + -11088;
	// 8309F49C: 4BF347A5  bl 0x82fd3c40
	ctx.lr = 0x8309F4A0;
	sub_82FD3C40(ctx, base);
	// 8309F4A0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309F4A4: 408201DC  bne 0x8309f680
	if !ctx.cr[0].eq {
	pc = 0x8309F680; continue 'dispatch;
	}
	// 8309F4A8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F4AC: 388BD558  addi r4, r11, -0x2aa8
	ctx.r[4].s64 = ctx.r[11].s64 + -10920;
	// 8309F4B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309F4B4: 4BF3478D  bl 0x82fd3c40
	ctx.lr = 0x8309F4B8;
	sub_82FD3C40(ctx, base);
	// 8309F4B8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309F4BC: 408201C4  bne 0x8309f680
	if !ctx.cr[0].eq {
	pc = 0x8309F680; continue 'dispatch;
	}
	// 8309F4C0: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 8309F4C4: 480001BC  b 0x8309f680
	pc = 0x8309F680; continue 'dispatch;
	// 8309F4C8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8309F4CC: 419A01B4  beq cr6, 0x8309f680
	if ctx.cr[6].eq {
	pc = 0x8309F680; continue 'dispatch;
	}
	// 8309F4D0: 3BBC0008  addi r29, r28, 8
	ctx.r[29].s64 = ctx.r[28].s64 + 8;
	// 8309F4D4: 48000168  b 0x8309f63c
	pc = 0x8309F63C; continue 'dispatch;
	// 8309F4D8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8309F4DC: 83ABBC24  lwz r29, -0x43dc(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17372 as u32) ) } as u64;
	// 8309F4E0: 4800015C  b 0x8309f63c
	pc = 0x8309F63C; continue 'dispatch;
	// 8309F4E4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8309F4E8: 83ABBC28  lwz r29, -0x43d8(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17368 as u32) ) } as u64;
	// 8309F4EC: 48000150  b 0x8309f63c
	pc = 0x8309F63C; continue 'dispatch;
	// 8309F4F0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309F4F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309F4F8: 396BE850  addi r11, r11, -0x17b0
	ctx.r[11].s64 = ctx.r[11].s64 + -6064;
	// 8309F4FC: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 8309F500: 4BF34741  bl 0x82fd3c40
	ctx.lr = 0x8309F504;
	sub_82FD3C40(ctx, base);
	// 8309F504: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309F508: 40820178  bne 0x8309f680
	if !ctx.cr[0].eq {
	pc = 0x8309F680; continue 'dispatch;
	}
	// 8309F50C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8309F510: 83ABBC20  lwz r29, -0x43e0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17376 as u32) ) } as u64;
	// 8309F514: 48000128  b 0x8309f63c
	pc = 0x8309F63C; continue 'dispatch;
	// 8309F518: 2F0B0100  cmpwi cr6, r11, 0x100
	ctx.cr[6].compare_i32(ctx.r[11].s32, 256, &mut ctx.xer);
	// 8309F51C: 419A00FC  beq cr6, 0x8309f618
	if ctx.cr[6].eq {
	pc = 0x8309F618; continue 'dispatch;
	}
	// 8309F520: 2F0B0200  cmpwi cr6, r11, 0x200
	ctx.cr[6].compare_i32(ctx.r[11].s32, 512, &mut ctx.xer);
	// 8309F524: 419A00D0  beq cr6, 0x8309f5f4
	if ctx.cr[6].eq {
	pc = 0x8309F5F4; continue 'dispatch;
	}
	// 8309F528: 2F0B0400  cmpwi cr6, r11, 0x400
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1024, &mut ctx.xer);
	// 8309F52C: 419A008C  beq cr6, 0x8309f5b8
	if ctx.cr[6].eq {
	pc = 0x8309F5B8; continue 'dispatch;
	}
	// 8309F530: 2F0B0800  cmpwi cr6, r11, 0x800
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2048, &mut ctx.xer);
	// 8309F534: 419A0048  beq cr6, 0x8309f57c
	if ctx.cr[6].eq {
	pc = 0x8309F57C; continue 'dispatch;
	}
	// 8309F538: 2F0B1000  cmpwi cr6, r11, 0x1000
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4096, &mut ctx.xer);
	// 8309F53C: 409A0144  bne cr6, 0x8309f680
	if !ctx.cr[6].eq {
	pc = 0x8309F680; continue 'dispatch;
	}
	// 8309F540: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F544: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309F548: 388BD5E4  addi r4, r11, -0x2a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -10780;
	// 8309F54C: 4BF346F5  bl 0x82fd3c40
	ctx.lr = 0x8309F550;
	sub_82FD3C40(ctx, base);
	// 8309F550: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309F554: 4082012C  bne 0x8309f680
	if !ctx.cr[0].eq {
	pc = 0x8309F680; continue 'dispatch;
	}
	// 8309F558: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F55C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309F560: 388BD60C  addi r4, r11, -0x29f4
	ctx.r[4].s64 = ctx.r[11].s64 + -10740;
	// 8309F564: 4BF346DD  bl 0x82fd3c40
	ctx.lr = 0x8309F568;
	sub_82FD3C40(ctx, base);
	// 8309F568: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309F56C: 40820114  bne 0x8309f680
	if !ctx.cr[0].eq {
	pc = 0x8309F680; continue 'dispatch;
	}
	// 8309F570: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F574: 388BD5F8  addi r4, r11, -0x2a08
	ctx.r[4].s64 = ctx.r[11].s64 + -10760;
	// 8309F578: 4BFFFF38  b 0x8309f4b0
	pc = 0x8309F4B0; continue 'dispatch;
	// 8309F57C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F580: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309F584: 388BD478  addi r4, r11, -0x2b88
	ctx.r[4].s64 = ctx.r[11].s64 + -11144;
	// 8309F588: 4BF346B9  bl 0x82fd3c40
	ctx.lr = 0x8309F58C;
	sub_82FD3C40(ctx, base);
	// 8309F58C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309F590: 408200F0  bne 0x8309f680
	if !ctx.cr[0].eq {
	pc = 0x8309F680; continue 'dispatch;
	}
	// 8309F594: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F598: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309F59C: 388BD48C  addi r4, r11, -0x2b74
	ctx.r[4].s64 = ctx.r[11].s64 + -11124;
	// 8309F5A0: 4BF346A1  bl 0x82fd3c40
	ctx.lr = 0x8309F5A4;
	sub_82FD3C40(ctx, base);
	// 8309F5A4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309F5A8: 408200D8  bne 0x8309f680
	if !ctx.cr[0].eq {
	pc = 0x8309F680; continue 'dispatch;
	}
	// 8309F5AC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F5B0: 388BD4C4  addi r4, r11, -0x2b3c
	ctx.r[4].s64 = ctx.r[11].s64 + -11068;
	// 8309F5B4: 4BFFFEFC  b 0x8309f4b0
	pc = 0x8309F4B0; continue 'dispatch;
	// 8309F5B8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F5BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309F5C0: 388BD4F0  addi r4, r11, -0x2b10
	ctx.r[4].s64 = ctx.r[11].s64 + -11024;
	// 8309F5C4: 4BF3467D  bl 0x82fd3c40
	ctx.lr = 0x8309F5C8;
	sub_82FD3C40(ctx, base);
	// 8309F5C8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309F5CC: 408200B4  bne 0x8309f680
	if !ctx.cr[0].eq {
	pc = 0x8309F680; continue 'dispatch;
	}
	// 8309F5D0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F5D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309F5D8: 388BD42C  addi r4, r11, -0x2bd4
	ctx.r[4].s64 = ctx.r[11].s64 + -11220;
	// 8309F5DC: 4BF34665  bl 0x82fd3c40
	ctx.lr = 0x8309F5E0;
	sub_82FD3C40(ctx, base);
	// 8309F5E0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309F5E4: 4082009C  bne 0x8309f680
	if !ctx.cr[0].eq {
	pc = 0x8309F680; continue 'dispatch;
	}
	// 8309F5E8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F5EC: 388BD4FC  addi r4, r11, -0x2b04
	ctx.r[4].s64 = ctx.r[11].s64 + -11012;
	// 8309F5F0: 4BFFFEC0  b 0x8309f4b0
	pc = 0x8309F4B0; continue 'dispatch;
	// 8309F5F4: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309F5F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309F5FC: 3BABE850  addi r29, r11, -0x17b0
	ctx.r[29].s64 = ctx.r[11].s64 + -6064;
	// 8309F600: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8309F604: 4BF3463D  bl 0x82fd3c40
	ctx.lr = 0x8309F608;
	sub_82FD3C40(ctx, base);
	// 8309F608: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309F60C: 40820074  bne 0x8309f680
	if !ctx.cr[0].eq {
	pc = 0x8309F680; continue 'dispatch;
	}
	// 8309F610: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 8309F614: 4BFFFE9C  b 0x8309f4b0
	pc = 0x8309F4B0; continue 'dispatch;
	// 8309F618: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309F61C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309F620: 396BE850  addi r11, r11, -0x17b0
	ctx.r[11].s64 = ctx.r[11].s64 + -6064;
	// 8309F624: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 8309F628: 4BF34619  bl 0x82fd3c40
	ctx.lr = 0x8309F62C;
	sub_82FD3C40(ctx, base);
	// 8309F62C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309F630: 40820050  bne 0x8309f680
	if !ctx.cr[0].eq {
	pc = 0x8309F680; continue 'dispatch;
	}
	// 8309F634: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 8309F638: 9B7F0050  stb r27, 0x50(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[27].u8 ) };
	// 8309F63C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8309F640: 419A0040  beq cr6, 0x8309f680
	if ctx.cr[6].eq {
	pc = 0x8309F680; continue 'dispatch;
	}
	// 8309F644: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309F648: 80DC0000  lwz r6, 0(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309F64C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8309F650: 80BC0004  lwz r5, 4(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309F654: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309F658: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8309F65C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309F660: 4E800421  bctrl
	ctx.lr = 0x8309F664;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309F664: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8309F668: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8309F66C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8309F670: 48000010  b 0x8309f680
	pc = 0x8309F680; continue 'dispatch;
	// 8309F674: 48000004  b 0x8309f678
	pc = 0x8309F678; continue 'dispatch;
	// 8309F678: 8B7F0050  lbz r27, 0x50(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309F67C: 83DF00BC  lwz r30, 0xbc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 8309F680: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309F684: 4182002C  beq 0x8309f6b0
	if ctx.cr[0].eq {
	pc = 0x8309F6B0; continue 'dispatch;
	}
	// 8309F688: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F68C: 811F00B4  lwz r8, 0xb4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8309F690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8309F694: 809F00AC  lwz r4, 0xac(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 8309F698: 38AB8070  addi r5, r11, -0x7f90
	ctx.r[5].s64 = ctx.r[11].s64 + -32656;
	// 8309F69C: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 8309F6A0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8309F6A4: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8309F6A8: 38C00052  li r6, 0x52
	ctx.r[6].s64 = 82;
	// 8309F6AC: 4BFC8B7D  bl 0x83068228
	ctx.lr = 0x8309F6B0;
	sub_83068228(ctx, base);
	// 8309F6B0: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8309F6B4: 48108B00  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309F6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309F6B8 size=8
    let mut pc: u32 = 0x8309F6B8;
    'dispatch: loop {
        match pc {
            0x8309F6B8 => {
    //   block [0x8309F6B8..0x8309F6C0)
	// 8309F6B8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309F6BC: 8216E8DC  lwz r16, -0x1724(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-5924 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309F6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309F6C0 size=84
    let mut pc: u32 = 0x8309F6C0;
    'dispatch: loop {
        match pc {
            0x8309F6C0 => {
    //   block [0x8309F6C0..0x8309F714)
	// 8309F6C0: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8309F6C4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309F6C8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309F6CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309F6D0: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8309F6D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8309F6D8: 809F00AC  lwz r4, 0xac(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 8309F6DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8309F6E0: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 8309F6E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8309F6E8: 38C0009C  li r6, 0x9c
	ctx.r[6].s64 = 156;
	// 8309F6EC: 80EB0010  lwz r7, 0x10(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309F6F0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F6F4: 38AB8070  addi r5, r11, -0x7f90
	ctx.r[5].s64 = ctx.r[11].s64 + -32656;
	// 8309F6F8: 4BFC8B31  bl 0x83068228
	ctx.lr = 0x8309F6FC;
	sub_83068228(ctx, base);
	// 8309F6FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309F700: 3C60830A  lis r3, -0x7cf6
	ctx.r[3].s64 = -2096496640;
	// 8309F704: 3863F674  addi r3, r3, -0x98c
	ctx.r[3].s64 = ctx.r[3].s64 + -2444;
	// 8309F708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309F70C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309F710: 48108AA4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309F714(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309F714 size=8
    let mut pc: u32 = 0x8309F714;
    'dispatch: loop {
        match pc {
            0x8309F714 => {
    //   block [0x8309F714..0x8309F71C)
	// 8309F714: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309F718: 8216E8DC  lwz r16, -0x1724(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-5924 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309F71C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309F71C size=24
    let mut pc: u32 = 0x8309F71C;
    'dispatch: loop {
        match pc {
            0x8309F71C => {
    //   block [0x8309F71C..0x8309F734)
	// 8309F71C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309F720: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309F724: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309F728: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8309F72C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309F730: 481114F9  bl 0x831b0c28
	ctx.lr = 0x8309F734;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309F73C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309F73C size=32
    let mut pc: u32 = 0x8309F73C;
    'dispatch: loop {
        match pc {
            0x8309F73C => {
    //   block [0x8309F73C..0x8309F75C)
	// 8309F73C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8309F740: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309F744: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8309F748: 3C60830A  lis r3, -0x7cf6
	ctx.r[3].s64 = -2096496640;
	// 8309F74C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309F750: 3863F678  addi r3, r3, -0x988
	ctx.r[3].s64 = ctx.r[3].s64 + -2440;
	// 8309F754: 997F0050  stb r11, 0x50(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 8309F758: 48108A5C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309F760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309F760 size=8
    let mut pc: u32 = 0x8309F760;
    'dispatch: loop {
        match pc {
            0x8309F760 => {
    //   block [0x8309F760..0x8309F768)
	// 8309F760: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309F764: 8216E948  lwz r16, -0x16b8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-5816 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309F768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309F768 size=144
    let mut pc: u32 = 0x8309F768;
    'dispatch: loop {
        match pc {
            0x8309F768 => {
    //   block [0x8309F768..0x8309F7F8)
	// 8309F768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309F76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309F770: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309F774: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8309F778: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309F77C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8309F780: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8309F784: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309F788: 4BF70359  bl 0x8300fae0
	ctx.lr = 0x8309F78C;
	sub_8300FAE0(ctx, base);
	// 8309F78C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8309F790: 4BF71881  bl 0x83011010
	ctx.lr = 0x8309F794;
	sub_83011010(ctx, base);
	// 8309F794: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F798: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8309F79C: 388BD70C  addi r4, r11, -0x28f4
	ctx.r[4].s64 = ctx.r[11].s64 + -10484;
	// 8309F7A0: 4BF47531  bl 0x82fe6cd0
	ctx.lr = 0x8309F7A4;
	sub_82FE6CD0(ctx, base);
	// 8309F7A4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F7A8: 388BD688  addi r4, r11, -0x2978
	ctx.r[4].s64 = ctx.r[11].s64 + -10616;
	// 8309F7AC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8309F7B0: 906BBC20  stw r3, -0x43e0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-17376 as u32), ctx.r[3].u32 ) };
	// 8309F7B4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8309F7B8: 4BF47519  bl 0x82fe6cd0
	ctx.lr = 0x8309F7BC;
	sub_82FE6CD0(ctx, base);
	// 8309F7BC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F7C0: 388BD8BC  addi r4, r11, -0x2744
	ctx.r[4].s64 = ctx.r[11].s64 + -10052;
	// 8309F7C4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8309F7C8: 906BBC24  stw r3, -0x43dc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-17372 as u32), ctx.r[3].u32 ) };
	// 8309F7CC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8309F7D0: 4BF47501  bl 0x82fe6cd0
	ctx.lr = 0x8309F7D4;
	sub_82FE6CD0(ctx, base);
	// 8309F7D4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8309F7D8: 906BBC28  stw r3, -0x43d8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-17368 as u32), ctx.r[3].u32 ) };
	// 8309F7DC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8309F7E0: 4BF711B9  bl 0x83010998
	ctx.lr = 0x8309F7E4;
	sub_83010998(ctx, base);
	// 8309F7E4: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8309F7E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309F7EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309F7F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309F7F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309F7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309F7F8 size=40
    let mut pc: u32 = 0x8309F7F8;
    'dispatch: loop {
        match pc {
            0x8309F7F8 => {
    //   block [0x8309F7F8..0x8309F820)
	// 8309F7F8: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8309F7FC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309F800: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309F804: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309F808: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8309F80C: 4BF7118D  bl 0x83010998
	ctx.lr = 0x8309F810;
	sub_83010998(ctx, base);
	// 8309F810: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309F814: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309F818: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309F81C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309F820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309F820 size=8
    let mut pc: u32 = 0x8309F820;
    'dispatch: loop {
        match pc {
            0x8309F820 => {
    //   block [0x8309F820..0x8309F828)
	// 8309F820: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309F824: 8216E990  lwz r16, -0x1670(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-5744 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309F828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309F828 size=1376
    let mut pc: u32 = 0x8309F828;
    'dispatch: loop {
        match pc {
            0x8309F828 => {
    //   block [0x8309F828..0x8309FD88)
	// 8309F828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309F82C: 48108925  bl 0x831a8150
	ctx.lr = 0x8309F830;
	sub_831A8130(ctx, base);
	// 8309F830: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 8309F834: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309F838: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8309F83C: 4BF38A0D  bl 0x82fd8248
	ctx.lr = 0x8309F840;
	sub_82FD8248(ctx, base);
	// 8309F840: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8309F844: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309F848: 3EC08339  lis r22, -0x7cc7
	ctx.r[22].s64 = -2093416448;
	// 8309F84C: 41820014  beq 0x8309f860
	if ctx.cr[0].eq {
	pc = 0x8309F860; continue 'dispatch;
	}
	// 8309F850: 38800022  li r4, 0x22
	ctx.r[4].s64 = 34;
	// 8309F854: 80B6B7E8  lwz r5, -0x4818(r22)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8309F858: 4BF86B09  bl 0x83026360
	ctx.lr = 0x8309F85C;
	sub_83026360(ctx, base);
	// 8309F85C: 48000008  b 0x8309f864
	pc = 0x8309F864; continue 'dispatch;
	// 8309F860: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309F864: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F868: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 8309F86C: 388BD0AC  addi r4, r11, -0x2f54
	ctx.r[4].s64 = ctx.r[11].s64 + -12116;
	// 8309F870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309F874: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309F878: 907EBC18  stw r3, -0x43e8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-17384 as u32), ctx.r[3].u32 ) };
	// 8309F87C: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309F880: 4BF86BD9  bl 0x83026458
	ctx.lr = 0x8309F884;
	sub_83026458(ctx, base);
	// 8309F884: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F888: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309F88C: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309F890: 388BD0C0  addi r4, r11, -0x2f40
	ctx.r[4].s64 = ctx.r[11].s64 + -12096;
	// 8309F894: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8309F898: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309F89C: 4BF86BBD  bl 0x83026458
	ctx.lr = 0x8309F8A0;
	sub_83026458(ctx, base);
	// 8309F8A0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F8A4: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309F8A8: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309F8AC: 388BD0EC  addi r4, r11, -0x2f14
	ctx.r[4].s64 = ctx.r[11].s64 + -12052;
	// 8309F8B0: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8309F8B4: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309F8B8: 4BF86BA1  bl 0x83026458
	ctx.lr = 0x8309F8BC;
	sub_83026458(ctx, base);
	// 8309F8BC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F8C0: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309F8C4: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309F8C8: 388BD124  addi r4, r11, -0x2edc
	ctx.r[4].s64 = ctx.r[11].s64 + -11996;
	// 8309F8CC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8309F8D0: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309F8D4: 4BF86B85  bl 0x83026458
	ctx.lr = 0x8309F8D8;
	sub_83026458(ctx, base);
	// 8309F8D8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F8DC: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309F8E0: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309F8E4: 388BD130  addi r4, r11, -0x2ed0
	ctx.r[4].s64 = ctx.r[11].s64 + -11984;
	// 8309F8E8: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8309F8EC: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309F8F0: 4BF86B69  bl 0x83026458
	ctx.lr = 0x8309F8F4;
	sub_83026458(ctx, base);
	// 8309F8F4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F8F8: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309F8FC: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309F900: 388BD14C  addi r4, r11, -0x2eb4
	ctx.r[4].s64 = ctx.r[11].s64 + -11956;
	// 8309F904: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 8309F908: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309F90C: 4BF86B4D  bl 0x83026458
	ctx.lr = 0x8309F910;
	sub_83026458(ctx, base);
	// 8309F910: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F914: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309F918: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309F91C: 388BD15C  addi r4, r11, -0x2ea4
	ctx.r[4].s64 = ctx.r[11].s64 + -11940;
	// 8309F920: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 8309F924: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309F928: 4BF86B31  bl 0x83026458
	ctx.lr = 0x8309F92C;
	sub_83026458(ctx, base);
	// 8309F92C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F930: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309F934: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309F938: 388BD1A8  addi r4, r11, -0x2e58
	ctx.r[4].s64 = ctx.r[11].s64 + -11864;
	// 8309F93C: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 8309F940: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309F944: 4BF86B15  bl 0x83026458
	ctx.lr = 0x8309F948;
	sub_83026458(ctx, base);
	// 8309F948: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F94C: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309F950: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309F954: 388BD1B4  addi r4, r11, -0x2e4c
	ctx.r[4].s64 = ctx.r[11].s64 + -11852;
	// 8309F958: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 8309F95C: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309F960: 4BF86AF9  bl 0x83026458
	ctx.lr = 0x8309F964;
	sub_83026458(ctx, base);
	// 8309F964: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F968: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309F96C: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309F970: 388BD1D0  addi r4, r11, -0x2e30
	ctx.r[4].s64 = ctx.r[11].s64 + -11824;
	// 8309F974: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	// 8309F978: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309F97C: 4BF86ADD  bl 0x83026458
	ctx.lr = 0x8309F980;
	sub_83026458(ctx, base);
	// 8309F980: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F984: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309F988: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309F98C: 388BD1DC  addi r4, r11, -0x2e24
	ctx.r[4].s64 = ctx.r[11].s64 + -11812;
	// 8309F990: 3960000A  li r11, 0xa
	ctx.r[11].s64 = 10;
	// 8309F994: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309F998: 4BF86AC1  bl 0x83026458
	ctx.lr = 0x8309F99C;
	sub_83026458(ctx, base);
	// 8309F99C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F9A0: 388BD1E8  addi r4, r11, -0x2e18
	ctx.r[4].s64 = ctx.r[11].s64 + -11800;
	// 8309F9A4: 3960000B  li r11, 0xb
	ctx.r[11].s64 = 11;
	// 8309F9A8: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309F9AC: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309F9B0: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309F9B4: 4BF86AA5  bl 0x83026458
	ctx.lr = 0x8309F9B8;
	sub_83026458(ctx, base);
	// 8309F9B8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F9BC: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309F9C0: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309F9C4: 388BD0F8  addi r4, r11, -0x2f08
	ctx.r[4].s64 = ctx.r[11].s64 + -12040;
	// 8309F9C8: 3960000C  li r11, 0xc
	ctx.r[11].s64 = 12;
	// 8309F9CC: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309F9D0: 4BF86A89  bl 0x83026458
	ctx.lr = 0x8309F9D4;
	sub_83026458(ctx, base);
	// 8309F9D4: 3960000D  li r11, 0xd
	ctx.r[11].s64 = 13;
	// 8309F9D8: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309F9DC: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309F9E0: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309F9E4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F9E8: 388BD1F0  addi r4, r11, -0x2e10
	ctx.r[4].s64 = ctx.r[11].s64 + -11792;
	// 8309F9EC: 4BF86A6D  bl 0x83026458
	ctx.lr = 0x8309F9F0;
	sub_83026458(ctx, base);
	// 8309F9F0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309F9F4: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309F9F8: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309F9FC: 388BD10C  addi r4, r11, -0x2ef4
	ctx.r[4].s64 = ctx.r[11].s64 + -12020;
	// 8309FA00: 3960000E  li r11, 0xe
	ctx.r[11].s64 = 14;
	// 8309FA04: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309FA08: 4BF86A51  bl 0x83026458
	ctx.lr = 0x8309FA0C;
	sub_83026458(ctx, base);
	// 8309FA0C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FA10: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309FA14: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FA18: 388BD204  addi r4, r11, -0x2dfc
	ctx.r[4].s64 = ctx.r[11].s64 + -11772;
	// 8309FA1C: 3960000F  li r11, 0xf
	ctx.r[11].s64 = 15;
	// 8309FA20: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309FA24: 4BF86A35  bl 0x83026458
	ctx.lr = 0x8309FA28;
	sub_83026458(ctx, base);
	// 8309FA28: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FA2C: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309FA30: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FA34: 388BD318  addi r4, r11, -0x2ce8
	ctx.r[4].s64 = ctx.r[11].s64 + -11496;
	// 8309FA38: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 8309FA3C: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309FA40: 4BF86A19  bl 0x83026458
	ctx.lr = 0x8309FA44;
	sub_83026458(ctx, base);
	// 8309FA44: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FA48: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309FA4C: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FA50: 388BD218  addi r4, r11, -0x2de8
	ctx.r[4].s64 = ctx.r[11].s64 + -11752;
	// 8309FA54: 39600011  li r11, 0x11
	ctx.r[11].s64 = 17;
	// 8309FA58: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309FA5C: 4BF869FD  bl 0x83026458
	ctx.lr = 0x8309FA60;
	sub_83026458(ctx, base);
	// 8309FA60: 3AE00012  li r23, 0x12
	ctx.r[23].s64 = 18;
	// 8309FA64: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309FA68: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FA6C: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FA70: 388BD224  addi r4, r11, -0x2ddc
	ctx.r[4].s64 = ctx.r[11].s64 + -11740;
	// 8309FA74: B2FF0050  sth r23, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[23].u16 ) };
	// 8309FA78: 4BF869E1  bl 0x83026458
	ctx.lr = 0x8309FA7C;
	sub_83026458(ctx, base);
	// 8309FA7C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FA80: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309FA84: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FA88: 388BD240  addi r4, r11, -0x2dc0
	ctx.r[4].s64 = ctx.r[11].s64 + -11712;
	// 8309FA8C: 39600013  li r11, 0x13
	ctx.r[11].s64 = 19;
	// 8309FA90: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309FA94: 4BF869C5  bl 0x83026458
	ctx.lr = 0x8309FA98;
	sub_83026458(ctx, base);
	// 8309FA98: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FA9C: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309FAA0: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FAA4: 388BD254  addi r4, r11, -0x2dac
	ctx.r[4].s64 = ctx.r[11].s64 + -11692;
	// 8309FAA8: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 8309FAAC: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309FAB0: 4BF869A9  bl 0x83026458
	ctx.lr = 0x8309FAB4;
	sub_83026458(ctx, base);
	// 8309FAB4: 3B000015  li r24, 0x15
	ctx.r[24].s64 = 21;
	// 8309FAB8: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309FABC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FAC0: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FAC4: 388BD2C8  addi r4, r11, -0x2d38
	ctx.r[4].s64 = ctx.r[11].s64 + -11576;
	// 8309FAC8: B31F0050  sth r24, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[24].u16 ) };
	// 8309FACC: 4BF8698D  bl 0x83026458
	ctx.lr = 0x8309FAD0;
	sub_83026458(ctx, base);
	// 8309FAD0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FAD4: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309FAD8: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FADC: 388BD274  addi r4, r11, -0x2d8c
	ctx.r[4].s64 = ctx.r[11].s64 + -11660;
	// 8309FAE0: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 8309FAE4: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309FAE8: 4BF86971  bl 0x83026458
	ctx.lr = 0x8309FAEC;
	sub_83026458(ctx, base);
	// 8309FAEC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FAF0: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FAF4: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309FAF8: 388BD27C  addi r4, r11, -0x2d84
	ctx.r[4].s64 = ctx.r[11].s64 + -11652;
	// 8309FAFC: 39600017  li r11, 0x17
	ctx.r[11].s64 = 23;
	// 8309FB00: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309FB04: 4BF86955  bl 0x83026458
	ctx.lr = 0x8309FB08;
	sub_83026458(ctx, base);
	// 8309FB08: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FB0C: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309FB10: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FB14: 388BD288  addi r4, r11, -0x2d78
	ctx.r[4].s64 = ctx.r[11].s64 + -11640;
	// 8309FB18: 39600018  li r11, 0x18
	ctx.r[11].s64 = 24;
	// 8309FB1C: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309FB20: 4BF86939  bl 0x83026458
	ctx.lr = 0x8309FB24;
	sub_83026458(ctx, base);
	// 8309FB24: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FB28: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309FB2C: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FB30: 388BD2A8  addi r4, r11, -0x2d58
	ctx.r[4].s64 = ctx.r[11].s64 + -11608;
	// 8309FB34: 39600019  li r11, 0x19
	ctx.r[11].s64 = 25;
	// 8309FB38: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309FB3C: 4BF8691D  bl 0x83026458
	ctx.lr = 0x8309FB40;
	sub_83026458(ctx, base);
	// 8309FB40: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FB44: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309FB48: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FB4C: 388BD184  addi r4, r11, -0x2e7c
	ctx.r[4].s64 = ctx.r[11].s64 + -11900;
	// 8309FB50: 3960001A  li r11, 0x1a
	ctx.r[11].s64 = 26;
	// 8309FB54: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309FB58: 4BF86901  bl 0x83026458
	ctx.lr = 0x8309FB5C;
	sub_83026458(ctx, base);
	// 8309FB5C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FB60: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309FB64: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FB68: 388BD2B8  addi r4, r11, -0x2d48
	ctx.r[4].s64 = ctx.r[11].s64 + -11592;
	// 8309FB6C: 3960001B  li r11, 0x1b
	ctx.r[11].s64 = 27;
	// 8309FB70: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309FB74: 4BF868E5  bl 0x83026458
	ctx.lr = 0x8309FB78;
	sub_83026458(ctx, base);
	// 8309FB78: 3B20001C  li r25, 0x1c
	ctx.r[25].s64 = 28;
	// 8309FB7C: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309FB80: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FB84: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FB88: 388BD2D8  addi r4, r11, -0x2d28
	ctx.r[4].s64 = ctx.r[11].s64 + -11560;
	// 8309FB8C: B33F0050  sth r25, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[25].u16 ) };
	// 8309FB90: 4BF868C9  bl 0x83026458
	ctx.lr = 0x8309FB94;
	sub_83026458(ctx, base);
	// 8309FB94: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FB98: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309FB9C: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FBA0: 388BD2F8  addi r4, r11, -0x2d08
	ctx.r[4].s64 = ctx.r[11].s64 + -11528;
	// 8309FBA4: 3960001D  li r11, 0x1d
	ctx.r[11].s64 = 29;
	// 8309FBA8: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309FBAC: 4BF868AD  bl 0x83026458
	ctx.lr = 0x8309FBB0;
	sub_83026458(ctx, base);
	// 8309FBB0: 3B40001E  li r26, 0x1e
	ctx.r[26].s64 = 30;
	// 8309FBB4: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309FBB8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FBBC: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FBC0: 388BD304  addi r4, r11, -0x2cfc
	ctx.r[4].s64 = ctx.r[11].s64 + -11516;
	// 8309FBC4: B35F0050  sth r26, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[26].u16 ) };
	// 8309FBC8: 4BF86891  bl 0x83026458
	ctx.lr = 0x8309FBCC;
	sub_83026458(ctx, base);
	// 8309FBCC: 3B60001F  li r27, 0x1f
	ctx.r[27].s64 = 31;
	// 8309FBD0: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309FBD4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FBD8: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FBDC: 388BD30C  addi r4, r11, -0x2cf4
	ctx.r[4].s64 = ctx.r[11].s64 + -11508;
	// 8309FBE0: B37F0050  sth r27, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[27].u16 ) };
	// 8309FBE4: 4BF86875  bl 0x83026458
	ctx.lr = 0x8309FBE8;
	sub_83026458(ctx, base);
	// 8309FBE8: 3B800020  li r28, 0x20
	ctx.r[28].s64 = 32;
	// 8309FBEC: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309FBF0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FBF4: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FBF8: 388BD324  addi r4, r11, -0x2cdc
	ctx.r[4].s64 = ctx.r[11].s64 + -11484;
	// 8309FBFC: B39F0050  sth r28, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u16 ) };
	// 8309FC00: 4BF86859  bl 0x83026458
	ctx.lr = 0x8309FC04;
	sub_83026458(ctx, base);
	// 8309FC04: 3BA00021  li r29, 0x21
	ctx.r[29].s64 = 33;
	// 8309FC08: 807EBC18  lwz r3, -0x43e8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 8309FC0C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FC10: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FC14: 388BD334  addi r4, r11, -0x2ccc
	ctx.r[4].s64 = ctx.r[11].s64 + -11468;
	// 8309FC18: B3BF0050  sth r29, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u16 ) };
	// 8309FC1C: 4BF8683D  bl 0x83026458
	ctx.lr = 0x8309FC20;
	sub_83026458(ctx, base);
	// 8309FC20: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8309FC24: 4BF38625  bl 0x82fd8248
	ctx.lr = 0x8309FC28;
	sub_82FD8248(ctx, base);
	// 8309FC28: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8309FC2C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309FC30: 41820014  beq 0x8309fc44
	if ctx.cr[0].eq {
	pc = 0x8309FC44; continue 'dispatch;
	}
	// 8309FC34: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 8309FC38: 80B6B7E8  lwz r5, -0x4818(r22)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8309FC3C: 4BF86725  bl 0x83026360
	ctx.lr = 0x8309FC40;
	sub_83026360(ctx, base);
	// 8309FC40: 48000008  b 0x8309fc48
	pc = 0x8309FC48; continue 'dispatch;
	// 8309FC44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309FC48: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 8309FC4C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FC50: B3BF0050  sth r29, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u16 ) };
	// 8309FC54: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FC58: 388BCF14  addi r4, r11, -0x30ec
	ctx.r[4].s64 = ctx.r[11].s64 + -12524;
	// 8309FC5C: 907EBC1C  stw r3, -0x43e4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-17380 as u32), ctx.r[3].u32 ) };
	// 8309FC60: 4BF867F9  bl 0x83026458
	ctx.lr = 0x8309FC64;
	sub_83026458(ctx, base);
	// 8309FC64: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FC68: 807EBC1C  lwz r3, -0x43e4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17380 as u32) ) } as u64;
	// 8309FC6C: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FC70: 388BCF30  addi r4, r11, -0x30d0
	ctx.r[4].s64 = ctx.r[11].s64 + -12496;
	// 8309FC74: 39600022  li r11, 0x22
	ctx.r[11].s64 = 34;
	// 8309FC78: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309FC7C: 4BF867DD  bl 0x83026458
	ctx.lr = 0x8309FC80;
	sub_83026458(ctx, base);
	// 8309FC80: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FC84: 807EBC1C  lwz r3, -0x43e4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17380 as u32) ) } as u64;
	// 8309FC88: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FC8C: B35F0050  sth r26, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[26].u16 ) };
	// 8309FC90: 388BCEC8  addi r4, r11, -0x3138
	ctx.r[4].s64 = ctx.r[11].s64 + -12600;
	// 8309FC94: 4BF867C5  bl 0x83026458
	ctx.lr = 0x8309FC98;
	sub_83026458(ctx, base);
	// 8309FC98: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FC9C: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FCA0: 807EBC1C  lwz r3, -0x43e4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17380 as u32) ) } as u64;
	// 8309FCA4: 388BCEE4  addi r4, r11, -0x311c
	ctx.r[4].s64 = ctx.r[11].s64 + -12572;
	// 8309FCA8: B37F0050  sth r27, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[27].u16 ) };
	// 8309FCAC: 4BF867AD  bl 0x83026458
	ctx.lr = 0x8309FCB0;
	sub_83026458(ctx, base);
	// 8309FCB0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FCB4: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FCB8: 807EBC1C  lwz r3, -0x43e4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17380 as u32) ) } as u64;
	// 8309FCBC: 388BCF94  addi r4, r11, -0x306c
	ctx.r[4].s64 = ctx.r[11].s64 + -12396;
	// 8309FCC0: 3960002E  li r11, 0x2e
	ctx.r[11].s64 = 46;
	// 8309FCC4: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309FCC8: 4BF86791  bl 0x83026458
	ctx.lr = 0x8309FCCC;
	sub_83026458(ctx, base);
	// 8309FCCC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FCD0: 807EBC1C  lwz r3, -0x43e4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17380 as u32) ) } as u64;
	// 8309FCD4: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FCD8: B31F0050  sth r24, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[24].u16 ) };
	// 8309FCDC: 388BCFAC  addi r4, r11, -0x3054
	ctx.r[4].s64 = ctx.r[11].s64 + -12372;
	// 8309FCE0: 4BF86779  bl 0x83026458
	ctx.lr = 0x8309FCE4;
	sub_83026458(ctx, base);
	// 8309FCE4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FCE8: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FCEC: 807EBC1C  lwz r3, -0x43e4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17380 as u32) ) } as u64;
	// 8309FCF0: 388BCEB8  addi r4, r11, -0x3148
	ctx.r[4].s64 = ctx.r[11].s64 + -12616;
	// 8309FCF4: B33F0050  sth r25, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[25].u16 ) };
	// 8309FCF8: 4BF86761  bl 0x83026458
	ctx.lr = 0x8309FCFC;
	sub_83026458(ctx, base);
	// 8309FCFC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FD00: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FD04: 807EBC1C  lwz r3, -0x43e4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17380 as u32) ) } as u64;
	// 8309FD08: 388BCF4C  addi r4, r11, -0x30b4
	ctx.r[4].s64 = ctx.r[11].s64 + -12468;
	// 8309FD0C: 39600023  li r11, 0x23
	ctx.r[11].s64 = 35;
	// 8309FD10: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309FD14: 4BF86745  bl 0x83026458
	ctx.lr = 0x8309FD18;
	sub_83026458(ctx, base);
	// 8309FD18: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FD1C: 807EBC1C  lwz r3, -0x43e4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17380 as u32) ) } as u64;
	// 8309FD20: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FD24: B39F0050  sth r28, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u16 ) };
	// 8309FD28: 388BCF00  addi r4, r11, -0x3100
	ctx.r[4].s64 = ctx.r[11].s64 + -12544;
	// 8309FD2C: 4BF8672D  bl 0x83026458
	ctx.lr = 0x8309FD30;
	sub_83026458(ctx, base);
	// 8309FD30: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FD34: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FD38: 807EBC1C  lwz r3, -0x43e4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17380 as u32) ) } as u64;
	// 8309FD3C: 388BCE24  addi r4, r11, -0x31dc
	ctx.r[4].s64 = ctx.r[11].s64 + -12764;
	// 8309FD40: B2FF0050  sth r23, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[23].u16 ) };
	// 8309FD44: 4BF86715  bl 0x83026458
	ctx.lr = 0x8309FD48;
	sub_83026458(ctx, base);
	// 8309FD48: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FD4C: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FD50: 807EBC1C  lwz r3, -0x43e4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17380 as u32) ) } as u64;
	// 8309FD54: 388BCE48  addi r4, r11, -0x31b8
	ctx.r[4].s64 = ctx.r[11].s64 + -12728;
	// 8309FD58: 39600031  li r11, 0x31
	ctx.r[11].s64 = 49;
	// 8309FD5C: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309FD60: 4BF866F9  bl 0x83026458
	ctx.lr = 0x8309FD64;
	sub_83026458(ctx, base);
	// 8309FD64: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FD68: 807EBC1C  lwz r3, -0x43e4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17380 as u32) ) } as u64;
	// 8309FD6C: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8309FD70: 388BCF74  addi r4, r11, -0x308c
	ctx.r[4].s64 = ctx.r[11].s64 + -12428;
	// 8309FD74: 39600025  li r11, 0x25
	ctx.r[11].s64 = 37;
	// 8309FD78: B17F0050  sth r11, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8309FD7C: 4BF866DD  bl 0x83026458
	ctx.lr = 0x8309FD80;
	sub_83026458(ctx, base);
	// 8309FD80: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 8309FD84: 4810841C  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309FD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309FD88 size=40
    let mut pc: u32 = 0x8309FD88;
    'dispatch: loop {
        match pc {
            0x8309FD88 => {
    //   block [0x8309FD88..0x8309FDB0)
	// 8309FD88: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 8309FD8C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309FD90: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309FD94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309FD98: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309FD9C: 4BF38545  bl 0x82fd82e0
	ctx.lr = 0x8309FDA0;
	sub_82FD82E0(ctx, base);
	// 8309FDA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309FDA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309FDA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309FDAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309FDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309FDB0 size=40
    let mut pc: u32 = 0x8309FDB0;
    'dispatch: loop {
        match pc {
            0x8309FDB0 => {
    //   block [0x8309FDB0..0x8309FDD8)
	// 8309FDB0: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 8309FDB4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309FDB8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309FDBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309FDC0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309FDC4: 4BF3851D  bl 0x82fd82e0
	ctx.lr = 0x8309FDC8;
	sub_82FD82E0(ctx, base);
	// 8309FDC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309FDCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309FDD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309FDD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309FDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8309FDD8 size=8
    let mut pc: u32 = 0x8309FDD8;
    'dispatch: loop {
        match pc {
            0x8309FDD8 => {
    //   block [0x8309FDD8..0x8309FDE0)
	// 8309FDD8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309FDDC: 8216EA24  lwz r16, -0x15dc(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-5596 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309FDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309FDE0 size=1176
    let mut pc: u32 = 0x8309FDE0;
    'dispatch: loop {
        match pc {
            0x8309FDE0 => {
    //   block [0x8309FDE0..0x830A0278)
	// 8309FDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309FDE4: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 8309FDE8: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 8309FDEC: 48108345  bl 0x831a8130
	ctx.lr = 0x8309FDF0;
	sub_831A8130(ctx, base);
	// 8309FDF0: 3BE1FEC0  addi r31, r1, -0x140
	ctx.r[31].s64 = ctx.r[1].s64 + -320;
	// 8309FDF4: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309FDF8: 7D1C4378  mr r28, r8
	ctx.r[28].u64 = ctx.r[8].u64;
	// 8309FDFC: 907F0154  stw r3, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[3].u32 ) };
	// 8309FE00: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 8309FE04: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8309FE08: 7CCF3378  mr r15, r6
	ctx.r[15].u64 = ctx.r[6].u64;
	// 8309FE0C: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8309FE10: 939F017C  stw r28, 0x17c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(380 as u32), ctx.r[28].u32 ) };
	// 8309FE14: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8309FE18: 92DF015C  stw r22, 0x15c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(348 as u32), ctx.r[22].u32 ) };
	// 8309FE1C: B3BF0166  sth r29, 0x166(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(358 as u32), ctx.r[29].u16 ) };
	// 8309FE20: 91FF016C  stw r15, 0x16c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(364 as u32), ctx.r[15].u32 ) };
	// 8309FE24: 419A000C  beq cr6, 0x8309fe30
	if ctx.cr[6].eq {
	pc = 0x8309FE30; continue 'dispatch;
	}
	// 8309FE28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309FE2C: 917C0004  stw r11, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8309FE30: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 8309FE34: 419A043C  beq cr6, 0x830a0270
	if ctx.cr[6].eq {
	pc = 0x830A0270; continue 'dispatch;
	}
	// 8309FE38: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8309FE3C: 3A2BBC18  addi r17, r11, -0x43e8
	ctx.r[17].s64 = ctx.r[11].s64 + -17384;
	// 8309FE40: 81710000  lwz r11, 0(r17)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309FE44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309FE48: 419A0428  beq cr6, 0x830a0270
	if ctx.cr[6].eq {
	pc = 0x830A0270; continue 'dispatch;
	}
	// 8309FE4C: 57AB043E  clrlwi r11, r29, 0x10
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x0000FFFFu64;
	// 8309FE50: 2B0B0032  cmplwi cr6, r11, 0x32
	ctx.cr[6].compare_u32(ctx.r[11].u32, 50 as u32, &mut ctx.xer);
	// 8309FE54: 4098041C  bge cr6, 0x830a0270
	if !ctx.cr[6].lt {
	pc = 0x830A0270; continue 'dispatch;
	}
	// 8309FE58: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309FE5C: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 8309FE60: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 8309FE64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309FE68: 4E800421  bctrl
	ctx.lr = 0x8309FE6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309FE6C: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309FE70: 7C701B78  mr r16, r3
	ctx.r[16].u64 = ctx.r[3].u64;
	// 8309FE74: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 8309FE78: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 8309FE7C: 921F005C  stw r16, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[16].u32 ) };
	// 8309FE80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309FE84: 4E800421  bctrl
	ctx.lr = 0x8309FE88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309FE88: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FE8C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8309FE90: 3A4BCC98  addi r18, r11, -0x3368
	ctx.r[18].s64 = ctx.r[11].s64 + -13160;
	// 8309FE94: 7E439378  mr r3, r18
	ctx.r[3].u64 = ctx.r[18].u64;
	// 8309FE98: 4BF33DA9  bl 0x82fd3c40
	ctx.lr = 0x8309FE9C;
	sub_82FD3C40(ctx, base);
	// 8309FE9C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309FEA0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FEA4: 3A8B8070  addi r20, r11, -0x7f90
	ctx.r[20].s64 = ctx.r[11].s64 + -32656;
	// 8309FEA8: 40820028  bne 0x8309fed0
	if !ctx.cr[0].eq {
	pc = 0x8309FED0; continue 'dispatch;
	}
	// 8309FEAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8309FEB0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8309FEB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8309FEB8: 7E078378  mr r7, r16
	ctx.r[7].u64 = ctx.r[16].u64;
	// 8309FEBC: 38C000A6  li r6, 0xa6
	ctx.r[6].s64 = 166;
	// 8309FEC0: 7E85A378  mr r5, r20
	ctx.r[5].u64 = ctx.r[20].u64;
	// 8309FEC4: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8309FEC8: 7DE37B78  mr r3, r15
	ctx.r[3].u64 = ctx.r[15].u64;
	// 8309FECC: 4BFC835D  bl 0x83068228
	ctx.lr = 0x8309FED0;
	sub_83068228(ctx, base);
	// 8309FED0: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309FED4: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309FED8: 41820010  beq 0x8309fee8
	if ctx.cr[0].eq {
	pc = 0x8309FEE8; continue 'dispatch;
	}
	// 8309FEDC: 396BE86C  addi r11, r11, -0x1794
	ctx.r[11].s64 = ctx.r[11].s64 + -6036;
	// 8309FEE0: 39CB000C  addi r14, r11, 0xc
	ctx.r[14].s64 = ctx.r[11].s64 + 12;
	// 8309FEE4: 48000008  b 0x8309feec
	pc = 0x8309FEEC; continue 'dispatch;
	// 8309FEE8: 39CBE86C  addi r14, r11, -0x1794
	ctx.r[14].s64 = ctx.r[11].s64 + -6036;
	// 8309FEEC: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309FEF0: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 8309FEF4: 91DF0054  stw r14, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[14].u32 ) };
	// 8309FEF8: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8309FEFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309FF00: 4E800421  bctrl
	ctx.lr = 0x8309FF04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309FF04: 7C731B78  mr r19, r3
	ctx.r[19].u64 = ctx.r[3].u64;
	// 8309FF08: 81730000  lwz r11, 0(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309FF0C: 927F006C  stw r19, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[19].u32 ) };
	// 8309FF10: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309FF14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309FF18: 4E800421  bctrl
	ctx.lr = 0x8309FF1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309FF1C: 907F0068  stw r3, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 8309FF20: 397F0080  addi r11, r31, 0x80
	ctx.r[11].s64 = ctx.r[31].s64 + 128;
	// 8309FF24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8309FF28: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8309FF2C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8309FF30: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8309FF34: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8309FF38: 4200FFF8  bdnz 0x8309ff30
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8309FF30; continue 'dispatch;
	}
	// 8309FF3C: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 8309FF40: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8309FF44: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 8309FF48: 3B6B3610  addi r27, r11, 0x3610
	ctx.r[27].s64 = ctx.r[11].s64 + 13840;
	// 8309FF4C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FF50: 3B2BCDD0  addi r25, r11, -0x3230
	ctx.r[25].s64 = ctx.r[11].s64 + -12848;
	// 8309FF54: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FF58: 92BF0064  stw r21, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[21].u32 ) };
	// 8309FF5C: 3B0BCD54  addi r24, r11, -0x32ac
	ctx.r[24].s64 = ctx.r[11].s64 + -12972;
	// 8309FF60: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FF64: 3AEB8024  addi r23, r11, -0x7fdc
	ctx.r[23].s64 = ctx.r[11].s64 + -32732;
	// 8309FF68: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309FF6C: 3B4B8018  addi r26, r11, -0x7fe8
	ctx.r[26].s64 = ctx.r[11].s64 + -32744;
	// 8309FF70: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8309FF74: 7F155800  cmpw cr6, r21, r11
	ctx.cr[6].compare_i32(ctx.r[21].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8309FF78: 4098028C  bge cr6, 0x830a0204
	if !ctx.cr[6].lt {
	pc = 0x830A0204; continue 'dispatch;
	}
	// 8309FF7C: 81730000  lwz r11, 0(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309FF80: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 8309FF84: 7E639B78  mr r3, r19
	ctx.r[3].u64 = ctx.r[19].u64;
	// 8309FF88: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309FF8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309FF90: 4E800421  bctrl
	ctx.lr = 0x8309FF94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309FF94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309FF98: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309FF9C: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 8309FFA0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309FFA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309FFA8: 4E800421  bctrl
	ctx.lr = 0x8309FFAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309FFAC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8309FFB0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8309FFB4: 4BF33C8D  bl 0x82fd3c40
	ctx.lr = 0x8309FFB8;
	sub_82FD3C40(ctx, base);
	// 8309FFB8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309FFBC: 40820238  bne 0x830a01f4
	if !ctx.cr[0].eq {
	pc = 0x830A01F4; continue 'dispatch;
	}
	// 8309FFC0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309FFC4: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8309FFC8: 4BF39DA9  bl 0x82fd9d70
	ctx.lr = 0x8309FFCC;
	sub_82FD9D70(ctx, base);
	// 8309FFCC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309FFD0: 40820224  bne 0x830a01f4
	if !ctx.cr[0].eq {
	pc = 0x830A01F4; continue 'dispatch;
	}
	// 8309FFD4: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309FFD8: 2B0B0058  cmplwi cr6, r11, 0x58
	ctx.cr[6].compare_u32(ctx.r[11].u32, 88 as u32, &mut ctx.xer);
	// 8309FFDC: 419A000C  beq cr6, 0x8309ffe8
	if ctx.cr[6].eq {
	pc = 0x8309FFE8; continue 'dispatch;
	}
	// 8309FFE0: 2B0B0078  cmplwi cr6, r11, 0x78
	ctx.cr[6].compare_u32(ctx.r[11].u32, 120 as u32, &mut ctx.xer);
	// 8309FFE4: 409A002C  bne cr6, 0x830a0010
	if !ctx.cr[6].eq {
	pc = 0x830A0010; continue 'dispatch;
	}
	// 8309FFE8: A17D0002  lhz r11, 2(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(2 as u32) ) } as u64;
	// 8309FFEC: 2B0B004D  cmplwi cr6, r11, 0x4d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 77 as u32, &mut ctx.xer);
	// 8309FFF0: 419A000C  beq cr6, 0x8309fffc
	if ctx.cr[6].eq {
	pc = 0x8309FFFC; continue 'dispatch;
	}
	// 8309FFF4: 2B0B006D  cmplwi cr6, r11, 0x6d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 109 as u32, &mut ctx.xer);
	// 8309FFF8: 409A0018  bne cr6, 0x830a0010
	if !ctx.cr[6].eq {
	pc = 0x830A0010; continue 'dispatch;
	}
	// 8309FFFC: A17D0004  lhz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A0000: 2B0B004C  cmplwi cr6, r11, 0x4c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 76 as u32, &mut ctx.xer);
	// 830A0004: 419A006C  beq cr6, 0x830a0070
	if ctx.cr[6].eq {
	pc = 0x830A0070; continue 'dispatch;
	}
	// 830A0008: 2B0B006C  cmplwi cr6, r11, 0x6c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 108 as u32, &mut ctx.xer);
	// 830A000C: 419A0064  beq cr6, 0x830a0070
	if ctx.cr[6].eq {
	pc = 0x830A0070; continue 'dispatch;
	}
	// 830A0010: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A0014: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A0018: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 830A001C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A0020: 4E800421  bctrl
	ctx.lr = 0x830A0024;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A0024: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A0028: 41820088  beq 0x830a00b0
	if ctx.cr[0].eq {
	pc = 0x830A00B0; continue 'dispatch;
	}
	// 830A002C: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A0030: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A0034: 4182007C  beq 0x830a00b0
	if ctx.cr[0].eq {
	pc = 0x830A00B0; continue 'dispatch;
	}
	// 830A0038: 7E449378  mr r4, r18
	ctx.r[4].u64 = ctx.r[18].u64;
	// 830A003C: 4BF33C05  bl 0x82fd3c40
	ctx.lr = 0x830A0040;
	sub_82FD3C40(ctx, base);
	// 830A0040: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A0044: 40820044  bne 0x830a0088
	if !ctx.cr[0].eq {
	pc = 0x830A0088; continue 'dispatch;
	}
	// 830A0048: 7E038378  mr r3, r16
	ctx.r[3].u64 = ctx.r[16].u64;
	// 830A004C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830A0050: 4BF33BF1  bl 0x82fd3c40
	ctx.lr = 0x830A0054;
	sub_82FD3C40(ctx, base);
	// 830A0054: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A0058: 40820030  bne 0x830a0088
	if !ctx.cr[0].eq {
	pc = 0x830A0088; continue 'dispatch;
	}
	// 830A005C: 7E038378  mr r3, r16
	ctx.r[3].u64 = ctx.r[16].u64;
	// 830A0060: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830A0064: 4BF33BDD  bl 0x82fd3c40
	ctx.lr = 0x830A0068;
	sub_82FD3C40(ctx, base);
	// 830A0068: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A006C: 4082001C  bne 0x830a0088
	if !ctx.cr[0].eq {
	pc = 0x830A0088; continue 'dispatch;
	}
	// 830A0070: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 830A0074: 419A0180  beq cr6, 0x830a01f4
	if ctx.cr[6].eq {
	pc = 0x830A01F4; continue 'dispatch;
	}
	// 830A0078: 389F0058  addi r4, r31, 0x58
	ctx.r[4].s64 = ctx.r[31].s64 + 88;
	// 830A007C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830A0080: 4BF46B89  bl 0x82fe6c08
	ctx.lr = 0x830A0084;
	sub_82FE6C08(ctx, base);
	// 830A0084: 48000170  b 0x830a01f4
	pc = 0x830A01F4; continue 'dispatch;
	// 830A0088: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A008C: 7E098378  mr r9, r16
	ctx.r[9].u64 = ctx.r[16].u64;
	// 830A0090: 7DC87378  mr r8, r14
	ctx.r[8].u64 = ctx.r[14].u64;
	// 830A0094: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 830A0098: 38C0005F  li r6, 0x5f
	ctx.r[6].s64 = 95;
	// 830A009C: 7E85A378  mr r5, r20
	ctx.r[5].u64 = ctx.r[20].u64;
	// 830A00A0: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 830A00A4: 7DE37B78  mr r3, r15
	ctx.r[3].u64 = ctx.r[15].u64;
	// 830A00A8: 4BFC8181  bl 0x83068228
	ctx.lr = 0x830A00AC;
	sub_83068228(ctx, base);
	// 830A00AC: 48000148  b 0x830a01f4
	pc = 0x830A01F4; continue 'dispatch;
	// 830A00B0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A00B4: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 830A00B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A00BC: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 830A00C0: 915F0070  stw r10, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 830A00C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A00C8: 4E800421  bctrl
	ctx.lr = 0x830A00CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A00CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A00D0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830A00D4: 93BF0060  stw r29, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 830A00D8: 817F0154  lwz r11, 0x154(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 830A00DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A00E0: 80710000  lwz r3, 0(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A00E4: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A00E8: 4BF86309  bl 0x830263f0
	ctx.lr = 0x830A00EC;
	sub_830263F0(ctx, base);
	// 830A00EC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830A00F0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830A00F4: A1430000  lhz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A00F8: 4800006C  b 0x830a0164
	pc = 0x830A0164; continue 'dispatch;
	// 830A00FC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830A0100: 81FF016C  lwz r15, 0x16c(r31)
	ctx.r[15].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(364 as u32) ) } as u64;
	// 830A0104: 3B6B3610  addi r27, r11, 0x3610
	ctx.r[27].s64 = ctx.r[11].s64 + 13840;
	// 830A0108: 82DF015C  lwz r22, 0x15c(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(348 as u32) ) } as u64;
	// 830A010C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830A0110: 821F005C  lwz r16, 0x5c(r31)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 830A0114: 81DF0054  lwz r14, 0x54(r31)
	ctx.r[14].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830A0118: 3B2BCDD0  addi r25, r11, -0x3230
	ctx.r[25].s64 = ctx.r[11].s64 + -12848;
	// 830A011C: 827F006C  lwz r19, 0x6c(r31)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 830A0120: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830A0124: 82BF0064  lwz r21, 0x64(r31)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 830A0128: 83DF0058  lwz r30, 0x58(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 830A012C: 3B0BCD54  addi r24, r11, -0x32ac
	ctx.r[24].s64 = ctx.r[11].s64 + -12972;
	// 830A0130: 83BF0060  lwz r29, 0x60(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 830A0134: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830A0138: 815F0070  lwz r10, 0x70(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 830A013C: 8B9F0050  lbz r28, 0x50(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A0140: 3AEB8024  addi r23, r11, -0x7fdc
	ctx.r[23].s64 = ctx.r[11].s64 + -32732;
	// 830A0144: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830A0148: 3B4B8018  addi r26, r11, -0x7fe8
	ctx.r[26].s64 = ctx.r[11].s64 + -32744;
	// 830A014C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830A0150: 3A8B8070  addi r20, r11, -0x7f90
	ctx.r[20].s64 = ctx.r[11].s64 + -32656;
	// 830A0154: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830A0158: 3A4BCC98  addi r18, r11, -0x3368
	ctx.r[18].s64 = ctx.r[11].s64 + -13160;
	// 830A015C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830A0160: 3A2BBC18  addi r17, r11, -0x43e8
	ctx.r[17].s64 = ctx.r[11].s64 + -17384;
	// 830A0164: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A0168: 40820088  bne 0x830a01f0
	if !ctx.cr[0].eq {
	pc = 0x830A01F0; continue 'dispatch;
	}
	// 830A016C: A17F0166  lhz r11, 0x166(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(358 as u32) ) } as u64;
	// 830A0170: 1D6B0022  mulli r11, r11, 0x22
	ctx.r[11].s64 = ctx.r[11].s64 * 34;
	// 830A0174: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830A0178: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A017C: 7D6BDA2E  lhzx r11, r11, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 830A0180: 556907BF  clrlwi. r9, r11, 0x1e
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 830A0184: 41820048  beq 0x830a01cc
	if ctx.cr[0].eq {
	pc = 0x830A01CC; continue 'dispatch;
	}
	// 830A0188: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A018C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A0190: 557E04FA  rlwinm r30, r11, 0, 0x13, 0x1d
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 830A0194: 391F0080  addi r8, r31, 0x80
	ctx.r[8].s64 = ctx.r[31].s64 + 128;
	// 830A0198: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830A019C: 81290008  lwz r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A01A0: 7D6A41AE  stbx r11, r10, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[11].u8) };
	// 830A01A4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 830A01A8: 4E800421  bctrl
	ctx.lr = 0x830A01AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A01AC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 830A01B0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A01B4: 807F0154  lwz r3, 0x154(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 830A01B8: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 830A01BC: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 830A01C0: 7DE87B78  mr r8, r15
	ctx.r[8].u64 = ctx.r[15].u64;
	// 830A01C4: 4BFFF24D  bl 0x8309f410
	ctx.lr = 0x830A01C8;
	sub_8309F410(ctx, base);
	// 830A01C8: 48000028  b 0x830a01f0
	pc = 0x830A01F0; continue 'dispatch;
	// 830A01CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A01D0: 7E098378  mr r9, r16
	ctx.r[9].u64 = ctx.r[16].u64;
	// 830A01D4: 7DC87378  mr r8, r14
	ctx.r[8].u64 = ctx.r[14].u64;
	// 830A01D8: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 830A01DC: 38C0005F  li r6, 0x5f
	ctx.r[6].s64 = 95;
	// 830A01E0: 7E85A378  mr r5, r20
	ctx.r[5].u64 = ctx.r[20].u64;
	// 830A01E4: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 830A01E8: 7DE37B78  mr r3, r15
	ctx.r[3].u64 = ctx.r[15].u64;
	// 830A01EC: 4BFC803D  bl 0x83068228
	ctx.lr = 0x830A01F0;
	sub_83068228(ctx, base);
	// 830A01F0: 839F017C  lwz r28, 0x17c(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(380 as u32) ) } as u64;
	// 830A01F4: 3AB50001  addi r21, r21, 1
	ctx.r[21].s64 = ctx.r[21].s64 + 1;
	// 830A01F8: 92BF0064  stw r21, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[21].u32 ) };
	// 830A01FC: A3BF0166  lhz r29, 0x166(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(358 as u32) ) } as u64;
	// 830A0200: 4BFFFD70  b 0x8309ff70
	pc = 0x8309FF70; continue 'dispatch;
	// 830A0204: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830A0208: 57AA043E  clrlwi r10, r29, 0x10
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x0000FFFFu64;
	// 830A020C: 3BCB4358  addi r30, r11, 0x4358
	ctx.r[30].s64 = ctx.r[11].s64 + 17240;
	// 830A0210: 1D6A0044  mulli r11, r10, 0x44
	ctx.r[11].s64 = ctx.r[10].s64 * 68;
	// 830A0214: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A0218: 7F8BDA14  add r28, r11, r27
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 830A021C: A17C0000  lhz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A0220: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A0224: 41820038  beq 0x830a025c
	if ctx.cr[0].eq {
	pc = 0x830A025C; continue 'dispatch;
	}
	// 830A0228: 397F0080  addi r11, r31, 0x80
	ctx.r[11].s64 = ctx.r[31].s64 + 128;
	// 830A022C: 7D7D58AE  lbzx r11, r29, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830A0230: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A0234: 40820028  bne 0x830a025c
	if !ctx.cr[0].eq {
	pc = 0x830A025C; continue 'dispatch;
	}
	// 830A0238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A023C: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A0240: 7E098378  mr r9, r16
	ctx.r[9].u64 = ctx.r[16].u64;
	// 830A0244: 7DC87378  mr r8, r14
	ctx.r[8].u64 = ctx.r[14].u64;
	// 830A0248: 38C0005E  li r6, 0x5e
	ctx.r[6].s64 = 94;
	// 830A024C: 7E85A378  mr r5, r20
	ctx.r[5].u64 = ctx.r[20].u64;
	// 830A0250: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 830A0254: 7DE37B78  mr r3, r15
	ctx.r[3].u64 = ctx.r[15].u64;
	// 830A0258: 4BFC7FD1  bl 0x83068228
	ctx.lr = 0x830A025C;
	sub_83068228(ctx, base);
	// 830A025C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830A0260: 3B9C0002  addi r28, r28, 2
	ctx.r[28].s64 = ctx.r[28].s64 + 2;
	// 830A0264: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 830A0268: 2F1D0022  cmpwi cr6, r29, 0x22
	ctx.cr[6].compare_i32(ctx.r[29].s32, 34, &mut ctx.xer);
	// 830A026C: 4198FFB0  blt cr6, 0x830a021c
	if ctx.cr[6].lt {
	pc = 0x830A021C; continue 'dispatch;
	}
	// 830A0270: 383F0140  addi r1, r31, 0x140
	ctx.r[1].s64 = ctx.r[31].s64 + 320;
	// 830A0274: 48107F0C  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A0278 size=8
    let mut pc: u32 = 0x830A0278;
    'dispatch: loop {
        match pc {
            0x830A0278 => {
    //   block [0x830A0278..0x830A0280)
	// 830A0278: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A027C: 8216EA24  lwz r16, -0x15dc(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-5596 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A0280 size=24
    let mut pc: u32 = 0x830A0280;
    'dispatch: loop {
        match pc {
            0x830A0280 => {
    //   block [0x830A0280..0x830A0298)
	// 830A0280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A0284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A0288: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A028C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A0290: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A0294: 48110995  bl 0x831b0c28
	ctx.lr = 0x830A0298;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A02A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A02A0 size=88
    let mut pc: u32 = 0x830A02A0;
    'dispatch: loop {
        match pc {
            0x830A02A0 => {
    //   block [0x830A02A0..0x830A02F8)
	// 830A02A0: 3BECFEC0  addi r31, r12, -0x140
	ctx.r[31].s64 = ctx.r[12].s64 + -320;
	// 830A02A4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A02A8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A02AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A02B0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830A02B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A02B8: 813F005C  lwz r9, 0x5c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 830A02BC: 38AB8070  addi r5, r11, -0x7f90
	ctx.r[5].s64 = ctx.r[11].s64 + -32656;
	// 830A02C0: 811F0054  lwz r8, 0x54(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830A02C4: 38C0005F  li r6, 0x5f
	ctx.r[6].s64 = 95;
	// 830A02C8: 80FF0060  lwz r7, 0x60(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 830A02CC: 809F015C  lwz r4, 0x15c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(348 as u32) ) } as u64;
	// 830A02D0: 807F016C  lwz r3, 0x16c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(364 as u32) ) } as u64;
	// 830A02D4: 4BFC7F55  bl 0x83068228
	ctx.lr = 0x830A02D8;
	sub_83068228(ctx, base);
	// 830A02D8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830A02DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A02E0: 3C60830A  lis r3, -0x7cf6
	ctx.r[3].s64 = -2096496640;
	// 830A02E4: 386300FC  addi r3, r3, 0xfc
	ctx.r[3].s64 = ctx.r[3].s64 + 252;
	// 830A02E8: 997F0050  stb r11, 0x50(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 830A02EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A02F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A02F4: 48107E8C  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A02F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A02F8 size=168
    let mut pc: u32 = 0x830A02F8;
    'dispatch: loop {
        match pc {
            0x830A02F8 => {
    //   block [0x830A02F8..0x830A03A0)
	// 830A02F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A02FC: 48107E71  bl 0x831a816c
	ctx.lr = 0x830A0300;
	sub_831A8130(ctx, base);
	// 830A0300: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A0304: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830A0308: 3BEBBC10  addi r31, r11, -0x43f0
	ctx.r[31].s64 = ctx.r[11].s64 + -17392;
	// 830A030C: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A0310: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830A0314: 419A0014  beq cr6, 0x830a0328
	if ctx.cr[6].eq {
	pc = 0x830A0328; continue 'dispatch;
	}
	// 830A0318: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A031C: 4BF5546D  bl 0x82ff5788
	ctx.lr = 0x830A0320;
	sub_82FF5788(ctx, base);
	// 830A0320: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A0324: 4BF37FBD  bl 0x82fd82e0
	ctx.lr = 0x830A0328;
	sub_82FD82E0(ctx, base);
	// 830A0328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830A032C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830A0330: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 830A0334: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 830A0338: 83DFBC18  lwz r30, -0x43e8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-17384 as u32) ) } as u64;
	// 830A033C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830A0340: 419A0014  beq cr6, 0x830a0354
	if ctx.cr[6].eq {
	pc = 0x830A0354; continue 'dispatch;
	}
	// 830A0344: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A0348: 4BF98239  bl 0x83038580
	ctx.lr = 0x830A034C;
	sub_83038580(ctx, base);
	// 830A034C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A0350: 4BF37F91  bl 0x82fd82e0
	ctx.lr = 0x830A0354;
	sub_82FD82E0(ctx, base);
	// 830A0354: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 830A0358: 83BEBC1C  lwz r29, -0x43e4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17380 as u32) ) } as u64;
	// 830A035C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830A0360: 419A0014  beq cr6, 0x830a0374
	if ctx.cr[6].eq {
	pc = 0x830A0374; continue 'dispatch;
	}
	// 830A0364: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A0368: 4BF98219  bl 0x83038580
	ctx.lr = 0x830A036C;
	sub_83038580(ctx, base);
	// 830A036C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A0370: 4BF37F71  bl 0x82fd82e0
	ctx.lr = 0x830A0374;
	sub_82FD82E0(ctx, base);
	// 830A0374: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830A0378: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830A037C: 917EBC1C  stw r11, -0x43e4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-17380 as u32), ctx.r[11].u32 ) };
	// 830A0380: 916ABC28  stw r11, -0x43d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17368 as u32), ctx.r[11].u32 ) };
	// 830A0384: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830A0388: 917FBC18  stw r11, -0x43e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-17384 as u32), ctx.r[11].u32 ) };
	// 830A038C: 916ABC24  stw r11, -0x43dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17372 as u32), ctx.r[11].u32 ) };
	// 830A0390: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830A0394: 916ABC20  stw r11, -0x43e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17376 as u32), ctx.r[11].u32 ) };
	// 830A0398: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A039C: 48107E20  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A03A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A03A0 size=8
    let mut pc: u32 = 0x830A03A0;
    'dispatch: loop {
        match pc {
            0x830A03A0 => {
    //   block [0x830A03A0..0x830A03A8)
	// 830A03A0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A03A4: 8216EA90  lwz r16, -0x1570(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-5488 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A03A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A03A8 size=208
    let mut pc: u32 = 0x830A03A8;
    'dispatch: loop {
        match pc {
            0x830A03A8 => {
    //   block [0x830A03A8..0x830A0478)
	// 830A03A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A03AC: 48107DC1  bl 0x831a816c
	ctx.lr = 0x830A03B0;
	sub_831A8130(ctx, base);
	// 830A03B0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A03B4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A03B8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830A03BC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A03C0: 3BCBBC10  addi r30, r11, -0x43f0
	ctx.r[30].s64 = ctx.r[11].s64 + -17392;
	// 830A03C4: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A03C8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A03CC: 408200A4  bne 0x830a0470
	if !ctx.cr[0].eq {
	pc = 0x830A0470; continue 'dispatch;
	}
	// 830A03D0: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A03D4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830A03D8: 409A0050  bne cr6, 0x830a0428
	if !ctx.cr[6].eq {
	pc = 0x830A0428; continue 'dispatch;
	}
	// 830A03DC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830A03E0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830A03E4: 808BB7EC  lwz r4, -0x4814(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18452 as u32) ) } as u64;
	// 830A03E8: 4BF553F1  bl 0x82ff57d8
	ctx.lr = 0x830A03EC;
	sub_82FF57D8(ctx, base);
	// 830A03EC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A03F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A03F4: 409A0028  bne cr6, 0x830a041c
	if !ctx.cr[6].eq {
	pc = 0x830A041C; continue 'dispatch;
	}
	// 830A03F8: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 830A03FC: 4BF37E4D  bl 0x82fd8248
	ctx.lr = 0x830A0400;
	sub_82FD8248(ctx, base);
	// 830A0400: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830A0404: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A0408: 4182000C  beq 0x830a0414
	if ctx.cr[0].eq {
	pc = 0x830A0414; continue 'dispatch;
	}
	// 830A040C: 4BF5533D  bl 0x82ff5748
	ctx.lr = 0x830A0410;
	sub_82FF5748(ctx, base);
	// 830A0410: 48000008  b 0x830a0418
	pc = 0x830A0418; continue 'dispatch;
	// 830A0414: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A0418: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 830A041C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830A0420: 4BF553F1  bl 0x82ff5810
	ctx.lr = 0x830A0424;
	sub_82FF5810(ctx, base);
	// 830A0424: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A0428: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 830A042C: 4BF553AD  bl 0x82ff57d8
	ctx.lr = 0x830A0430;
	sub_82FF57D8(ctx, base);
	// 830A0430: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A0434: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A0438: 40820030  bne 0x830a0468
	if !ctx.cr[0].eq {
	pc = 0x830A0468; continue 'dispatch;
	}
	// 830A043C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A0440: 4BFFF329  bl 0x8309f768
	ctx.lr = 0x830A0444;
	sub_8309F768(ctx, base);
	// 830A0444: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A0448: 4BFFF3E1  bl 0x8309f828
	ctx.lr = 0x830A044C;
	sub_8309F828(ctx, base);
	// 830A044C: 3D60830A  lis r11, -0x7cf6
	ctx.r[11].s64 = -2096496640;
	// 830A0450: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830A0454: 388B02F8  addi r4, r11, 0x2f8
	ctx.r[4].s64 = ctx.r[11].s64 + 760;
	// 830A0458: 386ABC2C  addi r3, r10, -0x43d4
	ctx.r[3].s64 = ctx.r[10].s64 + -17364;
	// 830A045C: 4BF576DD  bl 0x82ff7b38
	ctx.lr = 0x830A0460;
	sub_82FF7B38(ctx, base);
	// 830A0460: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830A0464: 997E0000  stb r11, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 830A0468: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 830A046C: 4BF553A5  bl 0x82ff5810
	ctx.lr = 0x830A0470;
	sub_82FF5810(ctx, base);
	// 830A0470: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A0474: 48107D48  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A0478 size=40
    let mut pc: u32 = 0x830A0478;
    'dispatch: loop {
        match pc {
            0x830A0478 => {
    //   block [0x830A0478..0x830A04A0)
	// 830A0478: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A047C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A0480: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A0484: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A0488: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830A048C: 4BF55385  bl 0x82ff5810
	ctx.lr = 0x830A0490;
	sub_82FF5810(ctx, base);
	// 830A0490: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A0494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A0498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A049C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A04A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A04A0 size=40
    let mut pc: u32 = 0x830A04A0;
    'dispatch: loop {
        match pc {
            0x830A04A0 => {
    //   block [0x830A04A0..0x830A04C8)
	// 830A04A0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A04A4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A04A8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A04AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A04B0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830A04B4: 4BF37E2D  bl 0x82fd82e0
	ctx.lr = 0x830A04B8;
	sub_82FD82E0(ctx, base);
	// 830A04B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A04BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A04C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A04C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A04C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A04C8 size=40
    let mut pc: u32 = 0x830A04C8;
    'dispatch: loop {
        match pc {
            0x830A04C8 => {
    //   block [0x830A04C8..0x830A04F0)
	// 830A04C8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A04CC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A04D0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A04D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A04D8: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 830A04DC: 4BF55335  bl 0x82ff5810
	ctx.lr = 0x830A04E0;
	sub_82FF5810(ctx, base);
	// 830A04E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A04E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A04E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A04EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A04F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A04F0 size=8
    let mut pc: u32 = 0x830A04F0;
    'dispatch: loop {
        match pc {
            0x830A04F0 => {
    //   block [0x830A04F0..0x830A04F8)
	// 830A04F0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A04F4: 8216EAF0  lwz r16, -0x1510(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-5392 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A04F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A04F8 size=88
    let mut pc: u32 = 0x830A04F8;
    'dispatch: loop {
        match pc {
            0x830A04F8 => {
    //   block [0x830A04F8..0x830A0550)
	// 830A04F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A04FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A0500: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A0504: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A0508: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830A050C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A0510: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A0514: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830A0518: 387E0008  addi r3, r30, 8
	ctx.r[3].s64 = ctx.r[30].s64 + 8;
	// 830A051C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830A0520: 909E0000  stw r4, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 830A0524: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830A0528: 4BFA4779  bl 0x83044ca0
	ctx.lr = 0x830A052C;
	sub_83044CA0(ctx, base);
	// 830A052C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A0530: 4BFFFE79  bl 0x830a03a8
	ctx.lr = 0x830A0534;
	sub_830A03A8(ctx, base);
	// 830A0534: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A0538: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830A053C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A0540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A0544: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A0548: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A054C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A0550 size=44
    let mut pc: u32 = 0x830A0550;
    'dispatch: loop {
        match pc {
            0x830A0550 => {
    //   block [0x830A0550..0x830A057C)
	// 830A0550: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830A0554: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A0558: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A055C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A0560: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830A0564: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 830A0568: 4BFA4811  bl 0x83044d78
	ctx.lr = 0x830A056C;
	sub_83044D78(ctx, base);
	// 830A056C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A0570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A0574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A0578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A0580 size=60
    let mut pc: u32 = 0x830A0580;
    'dispatch: loop {
        match pc {
            0x830A0580 => {
    //   block [0x830A0580..0x830A05BC)
	// 830A0580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A0584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A0588: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A058C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A0590: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A0594: 4BFF8C0D  bl 0x830991a0
	ctx.lr = 0x830A0598;
	sub_830991A0(ctx, base);
	// 830A0598: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A059C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A05A0: 396BEB28  addi r11, r11, -0x14d8
	ctx.r[11].s64 = ctx.r[11].s64 + -5336;
	// 830A05A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A05A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A05AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A05B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A05B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A05B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A05C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A05C0 size=12
    let mut pc: u32 = 0x830A05C0;
    'dispatch: loop {
        match pc {
            0x830A05C0 => {
    //   block [0x830A05C0..0x830A05CC)
	// 830A05C0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830A05C4: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 830A05C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A05D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A05D0 size=4
    let mut pc: u32 = 0x830A05D0;
    'dispatch: loop {
        match pc {
            0x830A05D0 => {
    //   block [0x830A05D0..0x830A05D4)
	// 830A05D0: 4BFF89D8  b 0x83098fa8
	sub_83098FA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A05D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A05D8 size=72
    let mut pc: u32 = 0x830A05D8;
    'dispatch: loop {
        match pc {
            0x830A05D8 => {
    //   block [0x830A05D8..0x830A0620)
	// 830A05D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A05DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A05E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A05E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A05E8: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 830A05EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A05F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A05F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A05F8: 4BFF8BA9  bl 0x830991a0
	ctx.lr = 0x830A05FC;
	sub_830991A0(ctx, base);
	// 830A05FC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A0600: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A0604: 396BEB28  addi r11, r11, -0x14d8
	ctx.r[11].s64 = ctx.r[11].s64 + -5336;
	// 830A0608: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A060C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A0610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A0614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A0618: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A061C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A0620 size=88
    let mut pc: u32 = 0x830A0620;
    'dispatch: loop {
        match pc {
            0x830A0620 => {
    //   block [0x830A0620..0x830A0678)
	// 830A0620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A0624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A0628: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A062C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A0630: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A0634: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A0638: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A063C: 396BEB28  addi r11, r11, -0x14d8
	ctx.r[11].s64 = ctx.r[11].s64 + -5336;
	// 830A0640: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A0644: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A0648: 4BFF8C69  bl 0x830992b0
	ctx.lr = 0x830A064C;
	sub_830992B0(ctx, base);
	// 830A064C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A0650: 4182000C  beq 0x830a065c
	if ctx.cr[0].eq {
	pc = 0x830A065C; continue 'dispatch;
	}
	// 830A0654: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A0658: 4BF37C89  bl 0x82fd82e0
	ctx.lr = 0x830A065C;
	sub_82FD82E0(ctx, base);
	// 830A065C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A0660: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A0664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A0668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A066C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A0670: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A0674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A0678 size=8
    let mut pc: u32 = 0x830A0678;
    'dispatch: loop {
        match pc {
            0x830A0678 => {
    //   block [0x830A0678..0x830A0680)
	// 830A0678: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A067C: 8216EB48  lwz r16, -0x14b8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-5304 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A0680 size=96
    let mut pc: u32 = 0x830A0680;
    'dispatch: loop {
        match pc {
            0x830A0680 => {
    //   block [0x830A0680..0x830A06E0)
	// 830A0680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A0684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A0688: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A068C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A0690: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830A0694: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A0698: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A069C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 830A06A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A06A4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830A06A8: 4BF37BF1  bl 0x82fd8298
	ctx.lr = 0x830A06AC;
	sub_82FD8298(ctx, base);
	// 830A06AC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A06B0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A06B4: 41820010  beq 0x830a06c4
	if ctx.cr[0].eq {
	pc = 0x830A06C4; continue 'dispatch;
	}
	// 830A06B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A06BC: 4BFFFF1D  bl 0x830a05d8
	ctx.lr = 0x830A06C0;
	sub_830A05D8(ctx, base);
	// 830A06C0: 48000008  b 0x830a06c8
	pc = 0x830A06C8; continue 'dispatch;
	// 830A06C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A06C8: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830A06CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A06D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A06D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A06D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A06DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A06E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A06E0 size=44
    let mut pc: u32 = 0x830A06E0;
    'dispatch: loop {
        match pc {
            0x830A06E0 => {
    //   block [0x830A06E0..0x830A070C)
	// 830A06E0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830A06E4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A06E8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A06EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A06F0: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830A06F4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A06F8: 4BF37BE9  bl 0x82fd82e0
	ctx.lr = 0x830A06FC;
	sub_82FD82E0(ctx, base);
	// 830A06FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A0700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A0704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A0708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A0710 size=60
    let mut pc: u32 = 0x830A0710;
    'dispatch: loop {
        match pc {
            0x830A0710 => {
    //   block [0x830A0710..0x830A074C)
	// 830A0710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A0714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A0718: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A071C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A0720: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A0724: 4BFF8A7D  bl 0x830991a0
	ctx.lr = 0x830A0728;
	sub_830991A0(ctx, base);
	// 830A0728: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A072C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A0730: 396BEB84  addi r11, r11, -0x147c
	ctx.r[11].s64 = ctx.r[11].s64 + -5244;
	// 830A0734: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A0738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A073C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A0740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A0744: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A0748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A0750 size=12
    let mut pc: u32 = 0x830A0750;
    'dispatch: loop {
        match pc {
            0x830A0750 => {
    //   block [0x830A0750..0x830A075C)
	// 830A0750: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830A0754: 386B43E8  addi r3, r11, 0x43e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17384;
	// 830A0758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A0760 size=72
    let mut pc: u32 = 0x830A0760;
    'dispatch: loop {
        match pc {
            0x830A0760 => {
    //   block [0x830A0760..0x830A07A8)
	// 830A0760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A0764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A0768: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A076C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A0770: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 830A0774: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A0778: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A077C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A0780: 4BFF8A21  bl 0x830991a0
	ctx.lr = 0x830A0784;
	sub_830991A0(ctx, base);
	// 830A0784: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A0788: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A078C: 396BEB84  addi r11, r11, -0x147c
	ctx.r[11].s64 = ctx.r[11].s64 + -5244;
	// 830A0790: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A0794: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A0798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A079C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A07A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A07A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A07A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A07A8 size=88
    let mut pc: u32 = 0x830A07A8;
    'dispatch: loop {
        match pc {
            0x830A07A8 => {
    //   block [0x830A07A8..0x830A0800)
	// 830A07A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A07AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A07B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A07B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A07B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A07BC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A07C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A07C4: 396BEB84  addi r11, r11, -0x147c
	ctx.r[11].s64 = ctx.r[11].s64 + -5244;
	// 830A07C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A07CC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A07D0: 4BFF8AE1  bl 0x830992b0
	ctx.lr = 0x830A07D4;
	sub_830992B0(ctx, base);
	// 830A07D4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A07D8: 4182000C  beq 0x830a07e4
	if ctx.cr[0].eq {
	pc = 0x830A07E4; continue 'dispatch;
	}
	// 830A07DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A07E0: 4BF37B01  bl 0x82fd82e0
	ctx.lr = 0x830A07E4;
	sub_82FD82E0(ctx, base);
	// 830A07E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A07E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A07EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A07F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A07F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A07F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A07FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A0800 size=8
    let mut pc: u32 = 0x830A0800;
    'dispatch: loop {
        match pc {
            0x830A0800 => {
    //   block [0x830A0800..0x830A0808)
	// 830A0800: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A0804: 8216EBA0  lwz r16, -0x1460(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-5216 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A0808 size=96
    let mut pc: u32 = 0x830A0808;
    'dispatch: loop {
        match pc {
            0x830A0808 => {
    //   block [0x830A0808..0x830A0868)
	// 830A0808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A080C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A0810: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A0814: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A0818: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830A081C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A0820: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A0824: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 830A0828: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A082C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830A0830: 4BF37A69  bl 0x82fd8298
	ctx.lr = 0x830A0834;
	sub_82FD8298(ctx, base);
	// 830A0834: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A0838: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A083C: 41820010  beq 0x830a084c
	if ctx.cr[0].eq {
	pc = 0x830A084C; continue 'dispatch;
	}
	// 830A0840: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A0844: 4BFFFF1D  bl 0x830a0760
	ctx.lr = 0x830A0848;
	sub_830A0760(ctx, base);
	// 830A0848: 48000008  b 0x830a0850
	pc = 0x830A0850; continue 'dispatch;
	// 830A084C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A0850: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830A0854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A0858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A085C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A0860: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A0864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A0868 size=44
    let mut pc: u32 = 0x830A0868;
    'dispatch: loop {
        match pc {
            0x830A0868 => {
    //   block [0x830A0868..0x830A0894)
	// 830A0868: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830A086C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A0870: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A0874: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A0878: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830A087C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A0880: 4BF37A61  bl 0x82fd82e0
	ctx.lr = 0x830A0884;
	sub_82FD82E0(ctx, base);
	// 830A0884: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A0888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A088C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A0890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A0898 size=80
    let mut pc: u32 = 0x830A0898;
    'dispatch: loop {
        match pc {
            0x830A0898 => {
    //   block [0x830A0898..0x830A08E8)
	// 830A0898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A089C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A08A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A08A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A08A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A08AC: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 830A08B0: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 830A08B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A08B8: 4BFF88E9  bl 0x830991a0
	ctx.lr = 0x830A08BC;
	sub_830991A0(ctx, base);
	// 830A08BC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A08C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A08C4: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 830A08C8: 396BEBDC  addi r11, r11, -0x1424
	ctx.r[11].s64 = ctx.r[11].s64 + -5156;
	// 830A08CC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A08D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A08D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A08D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A08DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A08E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A08E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A08E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A08E8 size=12
    let mut pc: u32 = 0x830A08E8;
    'dispatch: loop {
        match pc {
            0x830A08E8 => {
    //   block [0x830A08E8..0x830A08F4)
	// 830A08E8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830A08EC: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 830A08F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A08F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A08F8 size=96
    let mut pc: u32 = 0x830A08F8;
    'dispatch: loop {
        match pc {
            0x830A08F8 => {
    //   block [0x830A08F8..0x830A0958)
	// 830A08F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A08FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A0900: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A0904: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A0908: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A090C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A0910: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830A0914: 4BFF8695  bl 0x83098fa8
	ctx.lr = 0x830A0918;
	sub_83098FA8(ctx, base);
	// 830A0918: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A091C: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 830A0920: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830A0924: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A0928: 41820010  beq 0x830a0938
	if ctx.cr[0].eq {
	pc = 0x830A0938; continue 'dispatch;
	}
	// 830A092C: 809E001C  lwz r4, 0x1c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A0930: 4BFF8771  bl 0x830990a0
	ctx.lr = 0x830A0934;
	sub_830990A0(ctx, base);
	// 830A0934: 4800000C  b 0x830a0940
	pc = 0x830A0940; continue 'dispatch;
	// 830A0938: 4BFF87E9  bl 0x83099120
	ctx.lr = 0x830A093C;
	sub_83099120(ctx, base);
	// 830A093C: 907E001C  stw r3, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 830A0940: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A0944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A0948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A094C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A0950: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A0954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A0958 size=80
    let mut pc: u32 = 0x830A0958;
    'dispatch: loop {
        match pc {
            0x830A0958 => {
    //   block [0x830A0958..0x830A09A8)
	// 830A0958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A095C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A0960: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A0964: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A0968: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 830A096C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A0970: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A0974: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A0978: 4BFF8829  bl 0x830991a0
	ctx.lr = 0x830A097C;
	sub_830991A0(ctx, base);
	// 830A097C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A0980: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A0984: 396BEBDC  addi r11, r11, -0x1424
	ctx.r[11].s64 = ctx.r[11].s64 + -5156;
	// 830A0988: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A098C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 830A0990: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A0994: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A0998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A099C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A09A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A09A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A09A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A09A8 size=88
    let mut pc: u32 = 0x830A09A8;
    'dispatch: loop {
        match pc {
            0x830A09A8 => {
    //   block [0x830A09A8..0x830A0A00)
	// 830A09A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A09AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A09B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A09B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A09B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A09BC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A09C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A09C4: 396BEBDC  addi r11, r11, -0x1424
	ctx.r[11].s64 = ctx.r[11].s64 + -5156;
	// 830A09C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A09CC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A09D0: 4BFF88E1  bl 0x830992b0
	ctx.lr = 0x830A09D4;
	sub_830992B0(ctx, base);
	// 830A09D4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A09D8: 4182000C  beq 0x830a09e4
	if ctx.cr[0].eq {
	pc = 0x830A09E4; continue 'dispatch;
	}
	// 830A09DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A09E0: 4BF37901  bl 0x82fd82e0
	ctx.lr = 0x830A09E4;
	sub_82FD82E0(ctx, base);
	// 830A09E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A09E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A09EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A09F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A09F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A09F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A09FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A0A00 size=8
    let mut pc: u32 = 0x830A0A00;
    'dispatch: loop {
        match pc {
            0x830A0A00 => {
    //   block [0x830A0A00..0x830A0A08)
	// 830A0A00: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A0A04: 8216EBF8  lwz r16, -0x1408(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-5128 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A0A08 size=96
    let mut pc: u32 = 0x830A0A08;
    'dispatch: loop {
        match pc {
            0x830A0A08 => {
    //   block [0x830A0A08..0x830A0A68)
	// 830A0A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A0A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A0A10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A0A14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A0A18: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830A0A1C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A0A20: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A0A24: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 830A0A28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A0A2C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830A0A30: 4BF37869  bl 0x82fd8298
	ctx.lr = 0x830A0A34;
	sub_82FD8298(ctx, base);
	// 830A0A34: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A0A38: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A0A3C: 41820010  beq 0x830a0a4c
	if ctx.cr[0].eq {
	pc = 0x830A0A4C; continue 'dispatch;
	}
	// 830A0A40: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A0A44: 4BFFFF15  bl 0x830a0958
	ctx.lr = 0x830A0A48;
	sub_830A0958(ctx, base);
	// 830A0A48: 48000008  b 0x830a0a50
	pc = 0x830A0A50; continue 'dispatch;
	// 830A0A4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A0A50: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830A0A54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A0A58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A0A5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A0A60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A0A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A0A68 size=44
    let mut pc: u32 = 0x830A0A68;
    'dispatch: loop {
        match pc {
            0x830A0A68 => {
    //   block [0x830A0A68..0x830A0A94)
	// 830A0A68: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830A0A6C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A0A70: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A0A74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A0A78: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830A0A7C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A0A80: 4BF37861  bl 0x82fd82e0
	ctx.lr = 0x830A0A84;
	sub_82FD82E0(ctx, base);
	// 830A0A84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A0A88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A0A8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A0A90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A0A98 size=8
    let mut pc: u32 = 0x830A0A98;
    'dispatch: loop {
        match pc {
            0x830A0A98 => {
    //   block [0x830A0A98..0x830A0AA0)
	// 830A0A98: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A0A9C: 8216EC78  lwz r16, -0x1388(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-5000 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A0AA0 size=88
    let mut pc: u32 = 0x830A0AA0;
    'dispatch: loop {
        match pc {
            0x830A0AA0 => {
    //   block [0x830A0AA0..0x830A0AF8)
	// 830A0AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A0AA4: 481076C5  bl 0x831a8168
	ctx.lr = 0x830A0AA8;
	sub_831A8130(ctx, base);
	// 830A0AA8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A0AAC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A0AB0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A0AB4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830A0AB8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 830A0ABC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 830A0AC0: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 830A0AC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A0AC8: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830A0ACC: 4BF8B29D  bl 0x8302bd68
	ctx.lr = 0x830A0AD0;
	sub_8302BD68(ctx, base);
	// 830A0AD0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A0AD4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A0AD8: 939E020C  stw r28, 0x20c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(524 as u32), ctx.r[28].u32 ) };
	// 830A0ADC: 396BEC28  addi r11, r11, -0x13d8
	ctx.r[11].s64 = ctx.r[11].s64 + -5080;
	// 830A0AE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A0AE4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A0AE8: 4BF47669  bl 0x82fe8150
	ctx.lr = 0x830A0AEC;
	sub_82FE8150(ctx, base);
	// 830A0AEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A0AF0: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A0AF4: 481076C4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A0AF8 size=40
    let mut pc: u32 = 0x830A0AF8;
    'dispatch: loop {
        match pc {
            0x830A0AF8 => {
    //   block [0x830A0AF8..0x830A0B20)
	// 830A0AF8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A0AFC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A0B00: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A0B04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A0B08: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A0B0C: 4BF8B43D  bl 0x8302bf48
	ctx.lr = 0x830A0B10;
	sub_8302BF48(ctx, base);
	// 830A0B10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A0B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A0B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A0B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A0B20 size=12
    let mut pc: u32 = 0x830A0B20;
    'dispatch: loop {
        match pc {
            0x830A0B20 => {
    //   block [0x830A0B20..0x830A0B2C)
	// 830A0B20: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830A0B24: 386B8208  addi r3, r11, -0x7df8
	ctx.r[3].s64 = ctx.r[11].s64 + -32248;
	// 830A0B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A0B30 size=772
    let mut pc: u32 = 0x830A0B30;
    'dispatch: loop {
        match pc {
            0x830A0B30 => {
    //   block [0x830A0B30..0x830A0E34)
	// 830A0B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A0B34: 48107629  bl 0x831a815c
	ctx.lr = 0x830A0B38;
	sub_831A8130(ctx, base);
	// 830A0B38: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A0B3C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830A0B40: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 830A0B44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A0B48: 99790000  stb r11, 0(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 830A0B4C: 817F01B8  lwz r11, 0x1b8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(440 as u32) ) } as u64;
	// 830A0B50: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830A0B54: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A0B58: 41820044  beq 0x830a0b9c
	if ctx.cr[0].eq {
	pc = 0x830A0B9C; continue 'dispatch;
	}
	// 830A0B5C: 388000F5  li r4, 0xf5
	ctx.r[4].s64 = 245;
	// 830A0B60: 4BF47D29  bl 0x82fe8888
	ctx.lr = 0x830A0B64;
	sub_82FE8888(ctx, base);
	// 830A0B64: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 830A0B68: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 830A0B6C: 4BF76CA5  bl 0x83017810
	ctx.lr = 0x830A0B70;
	sub_83017810(ctx, base);
	// 830A0B70: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A0B74: 38C0004D  li r6, 0x4d
	ctx.r[6].s64 = 77;
	// 830A0B78: 80FF00D8  lwz r7, 0xd8(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 830A0B7C: 388BECA8  addi r4, r11, -0x1358
	ctx.r[4].s64 = ctx.r[11].s64 + -4952;
	// 830A0B80: 38A00046  li r5, 0x46
	ctx.r[5].s64 = 70;
	// 830A0B84: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A0B88: 4BF304D1  bl 0x82fd1058
	ctx.lr = 0x830A0B8C;
	sub_82FD1058(ctx, base);
	// 830A0B8C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A0B90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A0B94: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830A0B98: 48110091  bl 0x831b0c28
	ctx.lr = 0x830A0B9C;
	sub_831B0C28(ctx, base);
	// 830A0B9C: 3BDF0190  addi r30, r31, 0x190
	ctx.r[30].s64 = ctx.r[31].s64 + 400;
	// 830A0BA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A0BA4: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A0BA8: 811E0020  lwz r8, 0x20(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A0BAC: 7D675B78  mr r7, r11
	ctx.r[7].u64 = ctx.r[11].u64;
	// 830A0BB0: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830A0BB4: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 830A0BB8: 54EB103A  slwi r11, r7, 2
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A0BBC: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 830A0BC0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830A0BC4: 8149FFFC  lwz r10, -4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-4 as u32) ) } as u64;
	// 830A0BC8: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 830A0BCC: 834A002C  lwz r26, 0x2c(r10)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A0BD0: 838B0030  lwz r28, 0x30(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A0BD4: 4BF757B5  bl 0x83016388
	ctx.lr = 0x830A0BD8;
	sub_83016388(ctx, base);
	// 830A0BD8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830A0BDC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A0BE0: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830A0BE4: 4BF528ED  bl 0x82ff34d0
	ctx.lr = 0x830A0BE8;
	sub_82FF34D0(ctx, base);
	// 830A0BE8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A0BEC: 40820030  bne 0x830a0c1c
	if !ctx.cr[0].eq {
	pc = 0x830A0C1C; continue 'dispatch;
	}
	// 830A0BF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830A0BF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830A0BF8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830A0BFC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830A0C00: 388000CA  li r4, 0xca
	ctx.r[4].s64 = 202;
	// 830A0C04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A0C08: 4BF47DD1  bl 0x82fe89d8
	ctx.lr = 0x830A0C0C;
	sub_82FE89D8(ctx, base);
	// 830A0C0C: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 830A0C10: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 830A0C14: 4BF76BFD  bl 0x83017810
	ctx.lr = 0x830A0C18;
	sub_83017810(ctx, base);
	// 830A0C18: 48000214  b 0x830a0e2c
	pc = 0x830A0E2C; continue 'dispatch;
	// 830A0C1C: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 830A0C20: 817F01B8  lwz r11, 0x1b8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(440 as u32) ) } as u64;
	// 830A0C24: 813F0084  lwz r9, 0x84(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830A0C28: 614A8028  ori r10, r10, 0x8028
	ctx.r[10].u64 = ctx.r[10].u64 | 32808;
	// 830A0C2C: 811D0004  lwz r8, 4(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A0C30: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830A0C34: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830A0C38: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830A0C3C: 557B063E  clrlwi r27, r11, 0x18
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830A0C40: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830A0C44: 419A0010  beq cr6, 0x830a0c54
	if ctx.cr[6].eq {
	pc = 0x830A0C54; continue 'dispatch;
	}
	// 830A0C48: 38800116  li r4, 0x116
	ctx.r[4].s64 = 278;
	// 830A0C4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A0C50: 4BF47C39  bl 0x82fe8888
	ctx.lr = 0x830A0C54;
	sub_82FE8888(ctx, base);
	// 830A0C54: 3B9F007C  addi r28, r31, 0x7c
	ctx.r[28].s64 = ctx.r[31].s64 + 124;
	// 830A0C58: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A0C5C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830A0C60: 4BF4D7C1  bl 0x82fee420
	ctx.lr = 0x830A0C64;
	sub_82FEE420(ctx, base);
	// 830A0C64: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 830A0C68: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830A0C6C: 4BF4D645  bl 0x82fee2b0
	ctx.lr = 0x830A0C70;
	sub_82FEE2B0(ctx, base);
	// 830A0C70: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A0C74: 4082002C  bne 0x830a0ca0
	if !ctx.cr[0].eq {
	pc = 0x830A0CA0; continue 'dispatch;
	}
	// 830A0C78: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A0C7C: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A0C80: 4BF3D6E9  bl 0x82fde368
	ctx.lr = 0x830A0C84;
	sub_82FDE368(ctx, base);
	// 830A0C84: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830A0C88: 388000C8  li r4, 0xc8
	ctx.r[4].s64 = 200;
	// 830A0C8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A0C90: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830A0C94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830A0C98: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830A0C9C: 4BF47D3D  bl 0x82fe89d8
	ctx.lr = 0x830A0CA0;
	sub_82FE89D8(ctx, base);
	// 830A0CA0: 897F0010  lbz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A0CA4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A0CA8: 418200AC  beq 0x830a0d54
	if ctx.cr[0].eq {
	pc = 0x830A0D54; continue 'dispatch;
	}
	// 830A0CAC: 807F00A8  lwz r3, 0xa8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 830A0CB0: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A0CB4: 80DD000C  lwz r6, 0xc(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 830A0CB8: 80BD0010  lwz r5, 0x10(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A0CBC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A0CC0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A0CC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A0CC8: 4E800421  bctrl
	ctx.lr = 0x830A0CCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A0CCC: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830A0CD0: 41800084  blt 0x830a0d54
	if ctx.cr[0].lt {
	pc = 0x830A0D54; continue 'dispatch;
	}
	// 830A0CD4: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 830A0CD8: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A0CDC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A0CE0: 40820020  bne 0x830a0d00
	if !ctx.cr[0].eq {
	pc = 0x830A0D00; continue 'dispatch;
	}
	// 830A0CE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A0CE8: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 830A0CEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A0CF0: 4E800421  bctrl
	ctx.lr = 0x830A0CF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A0CF4: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 830A0CF8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830A0CFC: 48000044  b 0x830a0d40
	pc = 0x830A0D40; continue 'dispatch;
	// 830A0D00: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830A0D04: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A0D08: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 830A0D0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A0D10: 41980010  blt cr6, 0x830a0d20
	if ctx.cr[6].lt {
	pc = 0x830A0D20; continue 'dispatch;
	}
	// 830A0D14: 4E800421  bctrl
	ctx.lr = 0x830A0D18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A0D18: 38800015  li r4, 0x15
	ctx.r[4].s64 = 21;
	// 830A0D1C: 4BFFFFDC  b 0x830a0cf8
	pc = 0x830A0CF8; continue 'dispatch;
	// 830A0D20: 4E800421  bctrl
	ctx.lr = 0x830A0D24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A0D24: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A0D28: 578A103A  slwi r10, r28, 2
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A0D2C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830A0D30: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830A0D34: 4BF3D635  bl 0x82fde368
	ctx.lr = 0x830A0D38;
	sub_82FDE368(ctx, base);
	// 830A0D38: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 830A0D3C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 830A0D40: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830A0D44: 807F00A8  lwz r3, 0xa8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 830A0D48: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830A0D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830A0D50: 4BF76809  bl 0x83017558
	ctx.lr = 0x830A0D54;
	sub_83017558(ctx, base);
	// 830A0D54: 817F00A8  lwz r11, 0xa8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 830A0D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A0D5C: 812B0048  lwz r9, 0x48(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 830A0D60: 914B0034  stw r10, 0x34(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 830A0D64: B1490000  sth r10, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 830A0D68: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 830A0D6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A0D70: 419A003C  beq cr6, 0x830a0dac
	if ctx.cr[6].eq {
	pc = 0x830A0DAC; continue 'dispatch;
	}
	// 830A0D74: 397F0158  addi r11, r31, 0x158
	ctx.r[11].s64 = ctx.r[31].s64 + 344;
	// 830A0D78: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 830A0D7C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 830A0D80: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A0D84: 810B0018  lwz r8, 0x18(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A0D88: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830A0D8C: 7D49432E  sthx r10, r9, r8
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u16) };
	// 830A0D90: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 830A0D94: 80EB0018  lwz r7, 0x18(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A0D98: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A0D9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A0DA0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A0DA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A0DA8: 4E800421  bctrl
	ctx.lr = 0x830A0DAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A0DAC: 576B063E  clrlwi r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	// 830A0DB0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830A0DB4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830A0DB8: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A0DBC: 99790000  stb r11, 0(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 830A0DC0: 4182006C  beq 0x830a0e2c
	if ctx.cr[0].eq {
	pc = 0x830A0E2C; continue 'dispatch;
	}
	// 830A0DC4: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A0DC8: 815E0020  lwz r10, 0x20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A0DCC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A0DD0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830A0DD4: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 830A0DD8: 806B0028  lwz r3, 0x28(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A0DDC: 907F00B8  stw r3, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[3].u32 ) };
	// 830A0DE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A0DE4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A0DE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A0DEC: 4E800421  bctrl
	ctx.lr = 0x830A0DF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A0DF0: 817F00A8  lwz r11, 0xa8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 830A0DF4: 907F01DC  stw r3, 0x1dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(476 as u32), ctx.r[3].u32 ) };
	// 830A0DF8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830A0DFC: 809F00B8  lwz r4, 0xb8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 830A0E00: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A0E04: 816A0028  lwz r11, 0x28(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A0E08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A0E0C: 4E800421  bctrl
	ctx.lr = 0x830A0E10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A0E10: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A0E14: 815E0020  lwz r10, 0x20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A0E18: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A0E1C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830A0E20: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 830A0E24: 896B0020  lbz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A0E28: 997F0010  stb r11, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 830A0E2C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 830A0E30: 4810737C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A0E38 size=192
    let mut pc: u32 = 0x830A0E38;
    'dispatch: loop {
        match pc {
            0x830A0E38 => {
    //   block [0x830A0E38..0x830A0EF8)
	// 830A0E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A0E3C: 4810732D  bl 0x831a8168
	ctx.lr = 0x830A0E40;
	sub_831A8130(ctx, base);
	// 830A0E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A0E44: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830A0E48: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830A0E4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A0E50: 388BCC98  addi r4, r11, -0x3368
	ctx.r[4].s64 = ctx.r[11].s64 + -13160;
	// 830A0E54: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A0E58: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830A0E5C: 4BF32DE5  bl 0x82fd3c40
	ctx.lr = 0x830A0E60;
	sub_82FD3C40(ctx, base);
	// 830A0E60: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A0E64: 4182000C  beq 0x830a0e70
	if ctx.cr[0].eq {
	pc = 0x830A0E70; continue 'dispatch;
	}
	// 830A0E68: 83DF020C  lwz r30, 0x20c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(524 as u32) ) } as u64;
	// 830A0E6C: 48000014  b 0x830a0e80
	pc = 0x830A0E80; continue 'dispatch;
	// 830A0E70: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A0E74: 807F00B0  lwz r3, 0xb0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 830A0E78: 4BF46111  bl 0x82fe6f88
	ctx.lr = 0x830A0E7C;
	sub_82FE6F88(ctx, base);
	// 830A0E7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A0E80: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830A0E84: 419A0048  beq cr6, 0x830a0ecc
	if ctx.cr[6].eq {
	pc = 0x830A0ECC; continue 'dispatch;
	}
	// 830A0E88: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A0E8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A0E90: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A0E94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A0E98: 4E800421  bctrl
	ctx.lr = 0x830A0E9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A0E9C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 830A0EA0: 409A002C  bne cr6, 0x830a0ecc
	if !ctx.cr[6].eq {
	pc = 0x830A0ECC; continue 'dispatch;
	}
	// 830A0EA4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830A0EA8: 807F00A8  lwz r3, 0xa8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 830A0EAC: 93DF00B8  stw r30, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[30].u32 ) };
	// 830A0EB0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A0EB4: 917F01DC  stw r11, 0x1dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(476 as u32), ctx.r[11].u32 ) };
	// 830A0EB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A0EBC: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A0EC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A0EC4: 4E800421  bctrl
	ctx.lr = 0x830A0EC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A0EC8: 48000028  b 0x830a0ef0
	pc = 0x830A0EF0; continue 'dispatch;
	// 830A0ECC: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A0ED0: 40820020  bne 0x830a0ef0
	if !ctx.cr[0].eq {
	pc = 0x830A0EF0; continue 'dispatch;
	}
	// 830A0ED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830A0ED8: 807F00A8  lwz r3, 0xa8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 830A0EDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830A0EE0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830A0EE4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A0EE8: 38800048  li r4, 0x48
	ctx.r[4].s64 = 72;
	// 830A0EEC: 4BF7666D  bl 0x83017558
	ctx.lr = 0x830A0EF0;
	sub_83017558(ctx, base);
	// 830A0EF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830A0EF4: 481072C4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A0EF8 size=88
    let mut pc: u32 = 0x830A0EF8;
    'dispatch: loop {
        match pc {
            0x830A0EF8 => {
    //   block [0x830A0EF8..0x830A0F50)
	// 830A0EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A0EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A0F00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A0F04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A0F08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A0F0C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A0F10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A0F14: 396BEC28  addi r11, r11, -0x13d8
	ctx.r[11].s64 = ctx.r[11].s64 + -5080;
	// 830A0F18: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A0F1C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A0F20: 4BF8B029  bl 0x8302bf48
	ctx.lr = 0x830A0F24;
	sub_8302BF48(ctx, base);
	// 830A0F24: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A0F28: 4182000C  beq 0x830a0f34
	if ctx.cr[0].eq {
	pc = 0x830A0F34; continue 'dispatch;
	}
	// 830A0F2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A0F30: 4BF373B1  bl 0x82fd82e0
	ctx.lr = 0x830A0F34;
	sub_82FD82E0(ctx, base);
	// 830A0F34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A0F38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A0F3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A0F40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A0F44: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A0F48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A0F4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A0F50 size=8
    let mut pc: u32 = 0x830A0F50;
    'dispatch: loop {
        match pc {
            0x830A0F50 => {
    //   block [0x830A0F50..0x830A0F58)
	// 830A0F50: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A0F54: 8216ECF0  lwz r16, -0x1310(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-4880 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A0F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A0F58 size=616
    let mut pc: u32 = 0x830A0F58;
    'dispatch: loop {
        match pc {
            0x830A0F58 => {
    //   block [0x830A0F58..0x830A11C0)
	// 830A0F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A0F5C: 481071E9  bl 0x831a8144
	ctx.lr = 0x830A0F60;
	sub_831A8130(ctx, base);
	// 830A0F60: 3BE1FF00  addi r31, r1, -0x100
	ctx.r[31].s64 = ctx.r[1].s64 + -256;
	// 830A0F64: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A0F68: 7C942378  mr r20, r4
	ctx.r[20].u64 = ctx.r[4].u64;
	// 830A0F6C: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 830A0F70: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A0F74: 7EDBB378  mr r27, r22
	ctx.r[27].u64 = ctx.r[22].u64;
	// 830A0F78: 3A600001  li r19, 1
	ctx.r[19].s64 = 1;
	// 830A0F7C: 2F140000  cmpwi cr6, r20, 0
	ctx.cr[6].compare_i32(ctx.r[20].s32, 0, &mut ctx.xer);
	// 830A0F80: 40990094  ble cr6, 0x830a1014
	if !ctx.cr[6].gt {
	pc = 0x830A1014; continue 'dispatch;
	}
	// 830A0F84: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830A0F88: 3B4BCC10  addi r26, r11, -0x33f0
	ctx.r[26].s64 = ctx.r[11].s64 + -13296;
	// 830A0F8C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830A0F90: 3B0B8018  addi r24, r11, -0x7fe8
	ctx.r[24].s64 = ctx.r[11].s64 + -32744;
	// 830A0F94: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830A0F98: 3B2B8024  addi r25, r11, -0x7fdc
	ctx.r[25].s64 = ctx.r[11].s64 + -32732;
	// 830A0F9C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830A0FA0: 807E0208  lwz r3, 0x208(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(520 as u32) ) } as u64;
	// 830A0FA4: 4BF8B8CD  bl 0x8302c870
	ctx.lr = 0x830A0FA8;
	sub_8302C870(ctx, base);
	// 830A0FA8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830A0FAC: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 830A0FB0: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830A0FB4: 83BC0008  lwz r29, 8(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A0FB8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A0FBC: 4BF30985  bl 0x82fd1940
	ctx.lr = 0x830A0FC0;
	sub_82FD1940(ctx, base);
	// 830A0FC0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830A0FC4: 41820018  beq 0x830a0fdc
	if ctx.cr[0].eq {
	pc = 0x830A0FDC; continue 'dispatch;
	}
	// 830A0FC8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A0FCC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830A0FD0: 4BF32C71  bl 0x82fd3c40
	ctx.lr = 0x830A0FD4;
	sub_82FD3C40(ctx, base);
	// 830A0FD4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A0FD8: 41820030  beq 0x830a1008
	if ctx.cr[0].eq {
	pc = 0x830A1008; continue 'dispatch;
	}
	// 830A0FDC: 839C0010  lwz r28, 0x10(r28)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A0FE0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A0FE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A0FE8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830A0FEC: 4BF83A9D  bl 0x83024a88
	ctx.lr = 0x830A0FF0;
	sub_83024A88(ctx, base);
	// 830A0FF0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830A0FF4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830A0FF8: 4BF32C49  bl 0x82fd3c40
	ctx.lr = 0x830A0FFC;
	sub_82FD3C40(ctx, base);
	// 830A0FFC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A1000: 41820008  beq 0x830a1008
	if ctx.cr[0].eq {
	pc = 0x830A1008; continue 'dispatch;
	}
	// 830A1004: 9A7E01D8  stb r19, 0x1d8(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(472 as u32), ctx.r[19].u8 ) };
	// 830A1008: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 830A100C: 7F1BA000  cmpw cr6, r27, r20
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[20].s32, &mut ctx.xer);
	// 830A1010: 4198FF8C  blt cr6, 0x830a0f9c
	if ctx.cr[6].lt {
	pc = 0x830A0F9C; continue 'dispatch;
	}
	// 830A1014: 897E01D8  lbz r11, 0x1d8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(472 as u32) ) } as u64;
	// 830A1018: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A101C: 4182019C  beq 0x830a11b8
	if ctx.cr[0].eq {
	pc = 0x830A11B8; continue 'dispatch;
	}
	// 830A1020: 3ABE00DC  addi r21, r30, 0xdc
	ctx.r[21].s64 = ctx.r[30].s64 + 220;
	// 830A1024: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 830A1028: 4BF3DFD1  bl 0x82fdeff8
	ctx.lr = 0x830A102C;
	sub_82FDEFF8(ctx, base);
	// 830A102C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830A1030: 92BF005C  stw r21, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[21].u32 ) };
	// 830A1034: 939F0058  stw r28, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 830A1038: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A103C: 809E00D8  lwz r4, 0xd8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 830A1040: 4BF3D171  bl 0x82fde1b0
	ctx.lr = 0x830A1044;
	sub_82FDE1B0(ctx, base);
	// 830A1044: 7EDAB378  mr r26, r22
	ctx.r[26].u64 = ctx.r[22].u64;
	// 830A1048: 2F140000  cmpwi cr6, r20, 0
	ctx.cr[6].compare_i32(ctx.r[20].s32, 0, &mut ctx.xer);
	// 830A104C: 409900C8  ble cr6, 0x830a1114
	if !ctx.cr[6].gt {
	pc = 0x830A1114; continue 'dispatch;
	}
	// 830A1050: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830A1054: 3B0BD54C  addi r24, r11, -0x2ab4
	ctx.r[24].s64 = ctx.r[11].s64 + -10932;
	// 830A1058: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830A105C: 3AEBD238  addi r23, r11, -0x2dc8
	ctx.r[23].s64 = ctx.r[11].s64 + -11720;
	// 830A1060: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830A1064: 3B2BCCFC  addi r25, r11, -0x3304
	ctx.r[25].s64 = ctx.r[11].s64 + -13060;
	// 830A1068: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830A106C: 807E0208  lwz r3, 0x208(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(520 as u32) ) } as u64;
	// 830A1070: 4BF8B801  bl 0x8302c870
	ctx.lr = 0x830A1074;
	sub_8302C870(ctx, base);
	// 830A1074: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A1078: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A107C: 80BE0028  lwz r5, 0x28(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A1080: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A1084: 4BF3DA35  bl 0x82fdeab8
	ctx.lr = 0x830A1088;
	sub_82FDEAB8(ctx, base);
	// 830A1088: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A108C: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 830A1090: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A1094: 4BF7D62D  bl 0x8301e6c0
	ctx.lr = 0x830A1098;
	sub_8301E6C0(ctx, base);
	// 830A1098: 817E0038  lwz r11, 0x38(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 830A109C: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830A10A0: 409A0068  bne cr6, 0x830a1108
	if !ctx.cr[6].eq {
	pc = 0x830A1108; continue 'dispatch;
	}
	// 830A10A4: 837F0070  lwz r27, 0x70(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 830A10A8: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830A10AC: 83BD0010  lwz r29, 0x10(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A10B0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830A10B4: 4BF32B8D  bl 0x82fd3c40
	ctx.lr = 0x830A10B8;
	sub_82FD3C40(ctx, base);
	// 830A10B8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A10BC: 4182001C  beq 0x830a10d8
	if ctx.cr[0].eq {
	pc = 0x830A10D8; continue 'dispatch;
	}
	// 830A10C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A10C4: 92DC0004  stw r22, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[22].u32 ) };
	// 830A10C8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A10CC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830A10D0: 4BF384A1  bl 0x82fd9570
	ctx.lr = 0x830A10D4;
	sub_82FD9570(ctx, base);
	// 830A10D4: 48000034  b 0x830a1108
	pc = 0x830A1108; continue 'dispatch;
	// 830A10D8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830A10DC: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 830A10E0: 4BF32B61  bl 0x82fd3c40
	ctx.lr = 0x830A10E4;
	sub_82FD3C40(ctx, base);
	// 830A10E4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A10E8: 41820020  beq 0x830a1108
	if ctx.cr[0].eq {
	pc = 0x830A1108; continue 'dispatch;
	}
	// 830A10EC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A10F0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830A10F4: 4BF32B4D  bl 0x82fd3c40
	ctx.lr = 0x830A10F8;
	sub_82FD3C40(ctx, base);
	// 830A10F8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A10FC: 4182000C  beq 0x830a1108
	if ctx.cr[0].eq {
	pc = 0x830A1108; continue 'dispatch;
	}
	// 830A1100: 817E00A8  lwz r11, 0xa8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) } as u64;
	// 830A1104: 9A6B0024  stb r19, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[19].u8 ) };
	// 830A1108: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 830A110C: 7F1AA000  cmpw cr6, r26, r20
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[20].s32, &mut ctx.xer);
	// 830A1110: 4198FF58  blt cr6, 0x830a1068
	if ctx.cr[6].lt {
	pc = 0x830A1068; continue 'dispatch;
	}
	// 830A1114: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A1118: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830A111C: 554ADFFF  rlwinm. r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830A1120: 40820084  bne 0x830a11a4
	if !ctx.cr[0].eq {
	pc = 0x830A11A4; continue 'dispatch;
	}
	// 830A1124: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 830A1128: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A112C: 3BBE0158  addi r29, r30, 0x158
	ctx.r[29].s64 = ctx.r[30].s64 + 344;
	// 830A1130: 38FF0050  addi r7, r31, 0x50
	ctx.r[7].s64 = ctx.r[31].s64 + 80;
	// 830A1134: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 830A1138: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 830A113C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A1140: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A1144: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A1148: 7ECB532E  sthx r22, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[22].u16) };
	// 830A114C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A1150: 809C0018  lwz r4, 0x18(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A1154: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A1158: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A115C: 4E800421  bctrl
	ctx.lr = 0x830A1160;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A1160: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A1164: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A1168: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 830A116C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A1170: 7ECB532E  sthx r22, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[22].u16) };
	// 830A1174: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A1178: 813D0018  lwz r9, 0x18(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A117C: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A1180: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A1184: 7ECA4B2E  sthx r22, r10, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[22].u16) };
	// 830A1188: 815F0050  lwz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A118C: 809D0018  lwz r4, 0x18(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A1190: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830A1194: 807E00A8  lwz r3, 0xa8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) } as u64;
	// 830A1198: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A119C: 7CAA5A14  add r5, r10, r11
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 830A11A0: 4BF82BD1  bl 0x83023d70
	ctx.lr = 0x830A11A4;
	sub_83023D70(ctx, base);
	// 830A11A4: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A11A8: 4BF3D849  bl 0x82fde9f0
	ctx.lr = 0x830A11AC;
	sub_82FDE9F0(ctx, base);
	// 830A11AC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830A11B0: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 830A11B4: 4BF3DF75  bl 0x82fdf128
	ctx.lr = 0x830A11B8;
	sub_82FDF128(ctx, base);
	// 830A11B8: 383F0100  addi r1, r31, 0x100
	ctx.r[1].s64 = ctx.r[31].s64 + 256;
	// 830A11BC: 48106FD8  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A11C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A11C0 size=40
    let mut pc: u32 = 0x830A11C0;
    'dispatch: loop {
        match pc {
            0x830A11C0 => {
    //   block [0x830A11C0..0x830A11E8)
	// 830A11C0: 3BECFF00  addi r31, r12, -0x100
	ctx.r[31].s64 = ctx.r[12].s64 + -256;
	// 830A11C4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A11C8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A11CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A11D0: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 830A11D4: 4BF32DB5  bl 0x82fd3f88
	ctx.lr = 0x830A11D8;
	sub_82FD3F88(ctx, base);
	// 830A11D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A11DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A11E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A11E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A11E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A11E8 size=40
    let mut pc: u32 = 0x830A11E8;
    'dispatch: loop {
        match pc {
            0x830A11E8 => {
    //   block [0x830A11E8..0x830A1210)
	// 830A11E8: 3BECFF00  addi r31, r12, -0x100
	ctx.r[31].s64 = ctx.r[12].s64 + -256;
	// 830A11EC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A11F0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A11F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A11F8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A11FC: 4BF3D7F5  bl 0x82fde9f0
	ctx.lr = 0x830A1200;
	sub_82FDE9F0(ctx, base);
	// 830A1200: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A1204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A1208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A120C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A1210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A1210 size=8
    let mut pc: u32 = 0x830A1210;
    'dispatch: loop {
        match pc {
            0x830A1210 => {
    //   block [0x830A1210..0x830A1218)
	// 830A1210: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A1214: 8216ED58  lwz r16, -0x12a8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-4776 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A1218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A1218 size=2032
    let mut pc: u32 = 0x830A1218;
    'dispatch: loop {
        match pc {
            0x830A1218 => {
    //   block [0x830A1218..0x830A1A08)
	// 830A1218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A121C: 48106F15  bl 0x831a8130
	ctx.lr = 0x830A1220;
	sub_831A8130(ctx, base);
	// 830A1220: 3BE1FEE0  addi r31, r1, -0x120
	ctx.r[31].s64 = ctx.r[1].s64 + -288;
	// 830A1224: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A1228: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830A122C: 909F013C  stw r4, 0x13c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(316 as u32), ctx.r[4].u32 ) };
	// 830A1230: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A1234: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 830A1238: 3A9E013C  addi r20, r30, 0x13c
	ctx.r[20].s64 = ctx.r[30].s64 + 316;
	// 830A123C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A1240: 99640000  stb r11, 0(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 830A1244: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 830A1248: 817E0200  lwz r11, 0x200(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(512 as u32) ) } as u64;
	// 830A124C: 92BE01EC  stw r21, 0x1ec(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(492 as u32), ctx.r[21].u32 ) };
	// 830A1250: 93DF0134  stw r30, 0x134(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), ctx.r[30].u32 ) };
	// 830A1254: B2AB0000  sth r21, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[21].u16 ) };
	// 830A1258: 81740018  lwz r11, 0x18(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A125C: 92B40004  stw r21, 4(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(4 as u32), ctx.r[21].u32 ) };
	// 830A1260: B2AB0000  sth r21, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[21].u16 ) };
	// 830A1264: 807E0084  lwz r3, 0x84(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 830A1268: 4BF51EA1  bl 0x82ff3108
	ctx.lr = 0x830A126C;
	sub_82FF3108(ctx, base);
	// 830A126C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A1270: 40820024  bne 0x830a1294
	if !ctx.cr[0].eq {
	pc = 0x830A1294; continue 'dispatch;
	}
	// 830A1274: 388000B7  li r4, 0xb7
	ctx.r[4].s64 = 183;
	// 830A1278: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A127C: 4BF4760D  bl 0x82fe8888
	ctx.lr = 0x830A1280;
	sub_82FE8888(ctx, base);
	// 830A1280: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 830A1284: 387E007C  addi r3, r30, 0x7c
	ctx.r[3].s64 = ctx.r[30].s64 + 124;
	// 830A1288: 4BF76589  bl 0x83017810
	ctx.lr = 0x830A128C;
	sub_83017810(ctx, base);
	// 830A128C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A1290: 48000770  b 0x830a1a00
	pc = 0x830A1A00; continue 'dispatch;
	// 830A1294: 817E01B8  lwz r11, 0x1b8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(440 as u32) ) } as u64;
	// 830A1298: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A129C: 387E007C  addi r3, r30, 0x7c
	ctx.r[3].s64 = ctx.r[30].s64 + 124;
	// 830A12A0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830A12A4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830A12A8: 557D063E  clrlwi r29, r11, 0x18
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830A12AC: 9BBF0050  stb r29, 0x50(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u8 ) };
	// 830A12B0: 4BF4D171  bl 0x82fee420
	ctx.lr = 0x830A12B4;
	sub_82FEE420(ctx, base);
	// 830A12B4: 81740004  lwz r11, 4(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A12B8: 38DF0051  addi r6, r31, 0x51
	ctx.r[6].s64 = ctx.r[31].s64 + 81;
	// 830A12BC: 81540018  lwz r10, 0x18(r20)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A12C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A12C4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A12C8: 7EAB532E  sthx r21, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[21].u16) };
	// 830A12CC: 83340018  lwz r25, 0x18(r20)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A12D0: 80BE0208  lwz r5, 0x208(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(520 as u32) ) } as u64;
	// 830A12D4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830A12D8: 4BF85339  bl 0x83026610
	ctx.lr = 0x830A12DC;
	sub_83026610(ctx, base);
	// 830A12DC: 7C6F1B78  mr r15, r3
	ctx.r[15].u64 = ctx.r[3].u64;
	// 830A12E0: 7EBBAB78  mr r27, r21
	ctx.r[27].u64 = ctx.r[21].u64;
	// 830A12E4: 7EBCAB78  mr r28, r21
	ctx.r[28].u64 = ctx.r[21].u64;
	// 830A12E8: 3A40FFFF  li r18, -1
	ctx.r[18].s64 = -1;
	// 830A12EC: 7EB1AB78  mr r17, r21
	ctx.r[17].u64 = ctx.r[21].u64;
	// 830A12F0: 7FAEEB79  or. r14, r29, r29
	ctx.r[14].u64 = ctx.r[29].u64 | ctx.r[29].u64;
	ctx.cr[0].compare_i32(ctx.r[14].s32, 0, &mut ctx.xer);
	// 830A12F4: 408200F4  bne 0x830a13e8
	if !ctx.cr[0].eq {
	pc = 0x830A13E8; continue 'dispatch;
	}
	// 830A12F8: 3B5E0190  addi r26, r30, 0x190
	ctx.r[26].s64 = ctx.r[30].s64 + 400;
	// 830A12FC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830A1300: 4BF75319  bl 0x83016618
	ctx.lr = 0x830A1304;
	sub_83016618(ctx, base);
	// 830A1304: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A1308: 83AB0038  lwz r29, 0x38(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 830A130C: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A1310: 4182000C  beq 0x830a131c
	if ctx.cr[0].eq {
	pc = 0x830A131C; continue 'dispatch;
	}
	// 830A1314: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A1318: 48000008  b 0x830a1320
	pc = 0x830A1320; continue 'dispatch;
	// 830A131C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A1320: 895E0010  lbz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A1324: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A1328: 41820024  beq 0x830a134c
	if ctx.cr[0].eq {
	pc = 0x830A134C; continue 'dispatch;
	}
	// 830A132C: 817E00A8  lwz r11, 0xa8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) } as u64;
	// 830A1330: 806B005C  lwz r3, 0x5c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 830A1334: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A1338: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830A133C: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A1340: 41820034  beq 0x830a1374
	if ctx.cr[0].eq {
	pc = 0x830A1374; continue 'dispatch;
	}
	// 830A1344: 7EBDAB78  mr r29, r21
	ctx.r[29].u64 = ctx.r[21].u64;
	// 830A1348: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830A134C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 830A1350: 419A003C  beq cr6, 0x830a138c
	if ctx.cr[6].eq {
	pc = 0x830A138C; continue 'dispatch;
	}
	// 830A1354: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830A1358: 419A0034  beq cr6, 0x830a138c
	if ctx.cr[6].eq {
	pc = 0x830A138C; continue 'dispatch;
	}
	// 830A135C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 830A1360: 419A002C  beq cr6, 0x830a138c
	if ctx.cr[6].eq {
	pc = 0x830A138C; continue 'dispatch;
	}
	// 830A1364: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 830A1368: 409A0080  bne cr6, 0x830a13e8
	if !ctx.cr[6].eq {
	pc = 0x830A13E8; continue 'dispatch;
	}
	// 830A136C: 3A200001  li r17, 1
	ctx.r[17].s64 = 1;
	// 830A1370: 48000078  b 0x830a13e8
	pc = 0x830A13E8; continue 'dispatch;
	// 830A1374: 4BF773B5  bl 0x83018728
	ctx.lr = 0x830A1378;
	sub_83018728(ctx, base);
	// 830A1378: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A137C: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A1380: 4182FFC8  beq 0x830a1348
	if ctx.cr[0].eq {
	pc = 0x830A1348; continue 'dispatch;
	}
	// 830A1384: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A1388: 4BFFFFC4  b 0x830a134c
	pc = 0x830A134C; continue 'dispatch;
	// 830A138C: 817D0050  lwz r11, 0x50(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A1390: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A1394: 409A0020  bne cr6, 0x830a13b4
	if !ctx.cr[6].eq {
	pc = 0x830A13B4; continue 'dispatch;
	}
	// 830A1398: 817D003C  lwz r11, 0x3c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A139C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A13A0: 419A0014  beq cr6, 0x830a13b4
	if ctx.cr[6].eq {
	pc = 0x830A13B4; continue 'dispatch;
	}
	// 830A13A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A13A8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A13AC: 4BF9EC15  bl 0x8303ffc0
	ctx.lr = 0x830A13B0;
	sub_8303FFC0(ctx, base);
	// 830A13B0: 907D0050  stw r3, 0x50(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A13B4: 839D0050  lwz r28, 0x50(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A13B8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830A13BC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A13C0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A13C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A13C8: 4E800421  bctrl
	ctx.lr = 0x830A13CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A13CC: 817A0028  lwz r11, 0x28(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A13D0: 815A0020  lwz r10, 0x20(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A13D4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 830A13D8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A13DC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830A13E0: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 830A13E4: 824B0024  lwz r18, 0x24(r11)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A13E8: 3ADE0190  addi r22, r30, 0x190
	ctx.r[22].s64 = ctx.r[30].s64 + 400;
	// 830A13EC: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 830A13F0: 4BF756F1  bl 0x83016ae0
	ctx.lr = 0x830A13F4;
	sub_83016AE0(ctx, base);
	// 830A13F4: 81560028  lwz r10, 0x28(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A13F8: 81360020  lwz r9, 0x20(r22)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A13FC: 7C731B78  mr r19, r3
	ctx.r[19].u64 = ctx.r[3].u64;
	// 830A1400: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A1404: 897E0010  lbz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A1408: 2B0F0000  cmplwi cr6, r15, 0
	ctx.cr[6].compare_u32(ctx.r[15].u32, 0 as u32, &mut ctx.xer);
	// 830A140C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 830A1410: 814AFFFC  lwz r10, -4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) } as u64;
	// 830A1414: 996A0020  stb r11, 0x20(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u8 ) };
	// 830A1418: 419A0010  beq cr6, 0x830a1428
	if ctx.cr[6].eq {
	pc = 0x830A1428; continue 'dispatch;
	}
	// 830A141C: 7DE47B78  mr r4, r15
	ctx.r[4].u64 = ctx.r[15].u64;
	// 830A1420: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A1424: 4BFFFB35  bl 0x830a0f58
	ctx.lr = 0x830A1428;
	sub_830A0F58(ctx, base);
	// 830A1428: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A142C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 830A1430: 3AFE0158  addi r23, r30, 0x158
	ctx.r[23].s64 = ctx.r[30].s64 + 344;
	// 830A1434: 38FF0054  addi r7, r31, 0x54
	ctx.r[7].s64 = ctx.r[31].s64 + 84;
	// 830A1438: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 830A143C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830A1440: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A1444: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A1448: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 830A144C: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 830A1450: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A1454: 4E800421  bctrl
	ctx.lr = 0x830A1458;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A1458: 8A1E0010  lbz r16, 0x10(r30)
	ctx.r[16].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A145C: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 830A1460: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 830A1464: 419A005C  beq cr6, 0x830a14c0
	if ctx.cr[6].eq {
	pc = 0x830A14C0; continue 'dispatch;
	}
	// 830A1468: 81770004  lwz r11, 4(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A146C: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 830A1470: 81570018  lwz r10, 0x18(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A1474: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A1478: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A147C: 7EAB532E  sthx r21, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[21].u16) };
	// 830A1480: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830A1484: 80970018  lwz r4, 0x18(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A1488: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830A148C: 80FE00D8  lwz r7, 0xd8(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 830A1490: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A1494: 7CABCA14  add r5, r11, r25
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 830A1498: 4BF3D791  bl 0x82fdec28
	ctx.lr = 0x830A149C;
	sub_82FDEC28(ctx, base);
	// 830A149C: 38F3FFFF  addi r7, r19, -1
	ctx.r[7].s64 = ctx.r[19].s64 + -1;
	// 830A14A0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 830A14A4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 830A14A8: 389F0060  addi r4, r31, 0x60
	ctx.r[4].s64 = ctx.r[31].s64 + 96;
	// 830A14AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A14B0: 4BFB8E89  bl 0x8305a338
	ctx.lr = 0x830A14B4;
	sub_8305A338(ctx, base);
	// 830A14B4: 7C711B78  mr r17, r3
	ctx.r[17].u64 = ctx.r[3].u64;
	// 830A14B8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A14BC: 4BF3D535  bl 0x82fde9f0
	ctx.lr = 0x830A14C0;
	sub_82FDE9F0(ctx, base);
	// 830A14C0: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830A14C4: 7EBBAB78  mr r27, r21
	ctx.r[27].u64 = ctx.r[21].u64;
	// 830A14C8: 807E00B8  lwz r3, 0xb8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(184 as u32) ) } as u64;
	// 830A14CC: 7E479378  mr r7, r18
	ctx.r[7].u64 = ctx.r[18].u64;
	// 830A14D0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830A14D4: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 830A14D8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A14DC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830A14E0: 7FABCA14  add r29, r11, r25
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 830A14E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A14E8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A14EC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A14F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A14F4: 4E800421  bctrl
	ctx.lr = 0x830A14F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A14F8: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830A14FC: 40820124  bne 0x830a1620
	if !ctx.cr[0].eq {
	pc = 0x830A1620; continue 'dispatch;
	}
	// 830A1500: 807E00B8  lwz r3, 0xb8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(184 as u32) ) } as u64;
	// 830A1504: 817E00C0  lwz r11, 0xc0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(192 as u32) ) } as u64;
	// 830A1508: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A150C: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A1510: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A1514: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A1518: 4E800421  bctrl
	ctx.lr = 0x830A151C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A151C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A1520: 807E00C0  lwz r3, 0xc0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(192 as u32) ) } as u64;
	// 830A1524: 817C0020  lwz r11, 0x20(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A1528: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A152C: 4E800421  bctrl
	ctx.lr = 0x830A1530;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A1530: 7F181840  cmplw cr6, r24, r3
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[3].u32, &mut ctx.xer);
	// 830A1534: 419A0020  beq cr6, 0x830a1554
	if ctx.cr[6].eq {
	pc = 0x830A1554; continue 'dispatch;
	}
	// 830A1538: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830A153C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A1540: 4BF46B19  bl 0x82fe8058
	ctx.lr = 0x830A1544;
	sub_82FE8058(ctx, base);
	// 830A1544: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A1548: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A154C: 7E258B78  mr r5, r17
	ctx.r[5].u64 = ctx.r[17].u64;
	// 830A1550: 4BFFF8E9  bl 0x830a0e38
	ctx.lr = 0x830A1554;
	sub_830A0E38(ctx, base);
	// 830A1554: 807E00B8  lwz r3, 0xb8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(184 as u32) ) } as u64;
	// 830A1558: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 830A155C: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 830A1560: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A1564: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830A1568: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A156C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A1570: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A1574: 4E800421  bctrl
	ctx.lr = 0x830A1578;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A1578: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830A157C: 408200A4  bne 0x830a1620
	if !ctx.cr[0].eq {
	pc = 0x830A1620; continue 'dispatch;
	}
	// 830A1580: 38FF0058  addi r7, r31, 0x58
	ctx.r[7].s64 = ctx.r[31].s64 + 88;
	// 830A1584: 807E0218  lwz r3, 0x218(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(536 as u32) ) } as u64;
	// 830A1588: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 830A158C: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 830A1590: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A1594: 4BF7691D  bl 0x83017eb0
	ctx.lr = 0x830A1598;
	sub_83017EB0(ctx, base);
	// 830A1598: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A159C: 41820010  beq 0x830a15ac
	if ctx.cr[0].eq {
	pc = 0x830A15AC; continue 'dispatch;
	}
	// 830A15A0: 83830000  lwz r28, 0(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A15A4: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A15A8: 40820078  bne 0x830a1620
	if !ctx.cr[0].eq {
	pc = 0x830A1620; continue 'dispatch;
	}
	// 830A15AC: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 830A15B0: 809E00D8  lwz r4, 0xd8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 830A15B4: 4BF36CE5  bl 0x82fd8298
	ctx.lr = 0x830A15B8;
	sub_82FD8298(ctx, base);
	// 830A15B8: 907F0058  stw r3, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 830A15BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A15C0: 41820038  beq 0x830a15f8
	if ctx.cr[0].eq {
	pc = 0x830A15F8; continue 'dispatch;
	}
	// 830A15C4: 81770004  lwz r11, 4(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A15C8: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 830A15CC: 81570018  lwz r10, 0x18(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A15D0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 830A15D4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A15D8: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 830A15DC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A15E0: 7EAB532E  sthx r21, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[21].u16) };
	// 830A15E4: 80970018  lwz r4, 0x18(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A15E8: 813E00D8  lwz r9, 0xd8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 830A15EC: 4BFC452D  bl 0x83065b18
	ctx.lr = 0x830A15F0;
	sub_83065B18(ctx, base);
	// 830A15F0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830A15F4: 48000008  b 0x830a15fc
	pc = 0x830A15FC; continue 'dispatch;
	// 830A15F8: 7EBCAB78  mr r28, r21
	ctx.r[28].u64 = ctx.r[21].u64;
	// 830A15FC: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A1600: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 830A1604: 807E0218  lwz r3, 0x218(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(536 as u32) ) } as u64;
	// 830A1608: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 830A160C: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 830A1610: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A1614: 4BF76F15  bl 0x83018528
	ctx.lr = 0x830A1618;
	sub_83018528(ctx, base);
	// 830A1618: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 830A161C: 907C0010  stw r3, 0x10(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 830A1620: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A1624: 40820018  bne 0x830a163c
	if !ctx.cr[0].eq {
	pc = 0x830A163C; continue 'dispatch;
	}
	// 830A1628: 817C000C  lwz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 830A162C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 830A1630: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830A1634: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A1638: 40820060  bne 0x830a1698
	if !ctx.cr[0].eq {
	pc = 0x830A1698; continue 'dispatch;
	}
	// 830A163C: 562B063F  clrlwi. r11, r17, 0x18
	ctx.r[11].u64 = ctx.r[17].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A1640: 41820020  beq 0x830a1660
	if ctx.cr[0].eq {
	pc = 0x830A1660; continue 'dispatch;
	}
	// 830A1644: 9ABE0010  stb r21, 0x10(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[21].u8 ) };
	// 830A1648: 81760028  lwz r11, 0x28(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A164C: 81560020  lwz r10, 0x20(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A1650: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A1654: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830A1658: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 830A165C: 9AAB0020  stb r21, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[21].u8 ) };
	// 830A1660: 897E0010  lbz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A1664: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A1668: 41820030  beq 0x830a1698
	if ctx.cr[0].eq {
	pc = 0x830A1698; continue 'dispatch;
	}
	// 830A166C: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 830A1670: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A1674: 917C000C  stw r11, 0xc(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830A1678: 4BF3CCF1  bl 0x82fde368
	ctx.lr = 0x830A167C;
	sub_82FDE368(ctx, base);
	// 830A167C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830A1680: 807E00A8  lwz r3, 0xa8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) } as u64;
	// 830A1684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830A1688: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830A168C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830A1690: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 830A1694: 4BF75EC5  bl 0x83017558
	ctx.lr = 0x830A1698;
	sub_83017558(ctx, base);
	// 830A1698: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 830A169C: 817E0084  lwz r11, 0x84(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 830A16A0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830A16A4: 614A8028  ori r10, r10, 0x8028
	ctx.r[10].u64 = ctx.r[10].u64 | 32808;
	// 830A16A8: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 830A16AC: 7CAB502E  lwzx r5, r11, r10
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830A16B0: 4BF74D41  bl 0x830163f0
	ctx.lr = 0x830A16B4;
	sub_830163F0(ctx, base);
	// 830A16B4: 81760028  lwz r11, 0x28(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A16B8: 2B0E0000  cmplwi cr6, r14, 0
	ctx.cr[6].compare_u32(ctx.r[14].u32, 0 as u32, &mut ctx.xer);
	// 830A16BC: 81560020  lwz r10, 0x20(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A16C0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A16C4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830A16C8: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 830A16CC: 930B002C  stw r24, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[24].u32 ) };
	// 830A16D0: 419A0014  beq cr6, 0x830a16e4
	if ctx.cr[6].eq {
	pc = 0x830A16E4; continue 'dispatch;
	}
	// 830A16D4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 830A16D8: 809E00D8  lwz r4, 0xd8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 830A16DC: 4BF2F4A5  bl 0x82fd0b80
	ctx.lr = 0x830A16E0;
	sub_82FD0B80(ctx, base);
	// 830A16E0: 907E00C4  stw r3, 0xc4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(196 as u32), ctx.r[3].u32 ) };
	// 830A16E4: 897E0010  lbz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A16E8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A16EC: 4182001C  beq 0x830a1708
	if ctx.cr[0].eq {
	pc = 0x830A1708; continue 'dispatch;
	}
	// 830A16F0: 807E00A8  lwz r3, 0xa8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) } as u64;
	// 830A16F4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830A16F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A16FC: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A1700: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A1704: 4E800421  bctrl
	ctx.lr = 0x830A1708;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A1708: 81740004  lwz r11, 4(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A170C: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 830A1710: 81540018  lwz r10, 0x18(r20)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A1714: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A1718: 7EAB532E  sthx r21, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[21].u16) };
	// 830A171C: 80940018  lwz r4, 0x18(r20)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A1720: 4BF76151  bl 0x83017870
	ctx.lr = 0x830A1724;
	sub_83017870(ctx, base);
	// 830A1724: 897E0010  lbz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A1728: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A172C: 41820030  beq 0x830a175c
	if ctx.cr[0].eq {
	pc = 0x830A175C; continue 'dispatch;
	}
	// 830A1730: 817E00A8  lwz r11, 0xa8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) } as u64;
	// 830A1734: 806B005C  lwz r3, 0x5c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 830A1738: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A173C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830A1740: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A1744: 4182000C  beq 0x830a1750
	if ctx.cr[0].eq {
	pc = 0x830A1750; continue 'dispatch;
	}
	// 830A1748: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 830A174C: 48000014  b 0x830a1760
	pc = 0x830A1760; continue 'dispatch;
	// 830A1750: 4BF76FD9  bl 0x83018728
	ctx.lr = 0x830A1754;
	sub_83018728(ctx, base);
	// 830A1754: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A1758: 48000008  b 0x830a1760
	pc = 0x830A1760; continue 'dispatch;
	// 830A175C: 817C0038  lwz r11, 0x38(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(56 as u32) ) } as u64;
	// 830A1760: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A1764: 41820070  beq 0x830a17d4
	if ctx.cr[0].eq {
	pc = 0x830A17D4; continue 'dispatch;
	}
	// 830A1768: 834B0024  lwz r26, 0x24(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 830A176C: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 830A1770: 824B0018  lwz r18, 0x18(r11)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A1774: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830A1778: 4BF30639  bl 0x82fd1db0
	ctx.lr = 0x830A177C;
	sub_82FD1DB0(ctx, base);
	// 830A177C: 7C791B79  or. r25, r3, r3
	ctx.r[25].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 830A1780: 40810054  ble 0x830a17d4
	if !ctx.cr[0].gt {
	pc = 0x830A17D4; continue 'dispatch;
	}
	// 830A1784: 3B7E00DC  addi r27, r30, 0xdc
	ctx.r[27].s64 = ctx.r[30].s64 + 220;
	// 830A1788: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830A178C: 4BF3D86D  bl 0x82fdeff8
	ctx.lr = 0x830A1790;
	sub_82FDEFF8(ctx, base);
	// 830A1790: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A1794: 937F005C  stw r27, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[27].u32 ) };
	// 830A1798: 93BF0058  stw r29, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 830A179C: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 830A17A0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830A17A4: 4BF37DCD  bl 0x82fd9570
	ctx.lr = 0x830A17A8;
	sub_82FD9570(ctx, base);
	// 830A17A8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A17AC: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A17B0: 7E258B78  mr r5, r17
	ctx.r[5].u64 = ctx.r[17].u64;
	// 830A17B4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A17B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A17BC: 7EAB532E  sthx r21, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[21].u16) };
	// 830A17C0: 809D0018  lwz r4, 0x18(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A17C4: 4BFFF675  bl 0x830a0e38
	ctx.lr = 0x830A17C8;
	sub_830A0E38(ctx, base);
	// 830A17C8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A17CC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830A17D0: 4BF3D959  bl 0x82fdf128
	ctx.lr = 0x830A17D4;
	sub_82FDF128(ctx, base);
	// 830A17D4: 81760028  lwz r11, 0x28(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A17D8: 81560020  lwz r10, 0x20(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A17DC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A17E0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830A17E4: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 830A17E8: 924B0024  stw r18, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[18].u32 ) };
	// 830A17EC: 817E01E0  lwz r11, 0x1e0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(480 as u32) ) } as u64;
	// 830A17F0: 7F135840  cmplw cr6, r19, r11
	ctx.cr[6].compare_u32(ctx.r[19].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830A17F4: 4198000C  blt cr6, 0x830a1800
	if ctx.cr[6].lt {
	pc = 0x830A1800; continue 'dispatch;
	}
	// 830A17F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A17FC: 4BF82E8D  bl 0x83024688
	ctx.lr = 0x830A1800;
	sub_83024688(ctx, base);
	// 830A1800: 817E01E4  lwz r11, 0x1e4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(484 as u32) ) } as u64;
	// 830A1804: 566A103A  slwi r10, r19, 2
	ctx.r[10].u32 = ctx.r[19].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A1808: 2B0E0000  cmplwi cr6, r14, 0
	ctx.cr[6].compare_u32(ctx.r[14].u32, 0 as u32, &mut ctx.xer);
	// 830A180C: 7EAB512E  stwx r21, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[21].u32) };
	// 830A1810: 81760028  lwz r11, 0x28(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A1814: 81560020  lwz r10, 0x20(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A1818: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A181C: 813E00B8  lwz r9, 0xb8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(184 as u32) ) } as u64;
	// 830A1820: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830A1824: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 830A1828: 912B0028  stw r9, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 830A182C: 409A001C  bne cr6, 0x830a1848
	if !ctx.cr[6].eq {
	pc = 0x830A1848; continue 'dispatch;
	}
	// 830A1830: 560B063F  clrlwi. r11, r16, 0x18
	ctx.r[11].u64 = ctx.r[16].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A1834: 41820014  beq 0x830a1848
	if ctx.cr[0].eq {
	pc = 0x830A1848; continue 'dispatch;
	}
	// 830A1838: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830A183C: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A1840: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 830A1844: 4BF74C35  bl 0x83016478
	ctx.lr = 0x830A1848;
	sub_83016478(ctx, base);
	// 830A1848: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 830A184C: 809E0208  lwz r4, 0x208(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(520 as u32) ) } as u64;
	// 830A1850: 7DE57B78  mr r5, r15
	ctx.r[5].u64 = ctx.r[15].u64;
	// 830A1854: 80FE0054  lwz r7, 0x54(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) } as u64;
	// 830A1858: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A185C: 4BF856D5  bl 0x83026f30
	ctx.lr = 0x830A1860;
	sub_83026F30(ctx, base);
	// 830A1860: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830A1864: 4182000C  beq 0x830a1870
	if ctx.cr[0].eq {
	pc = 0x830A1870; continue 'dispatch;
	}
	// 830A1868: 807E0224  lwz r3, 0x224(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(548 as u32) ) } as u64;
	// 830A186C: 4BFCE0CD  bl 0x8306f938
	ctx.lr = 0x830A1870;
	sub_8306F938(ctx, base);
	// 830A1870: 817E005C  lwz r11, 0x5c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 830A1874: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A1878: 419A0044  beq cr6, 0x830a18bc
	if ctx.cr[6].eq {
	pc = 0x830A18BC; continue 'dispatch;
	}
	// 830A187C: 81770004  lwz r11, 4(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A1880: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830A1884: 80F70018  lwz r7, 0x18(r23)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A1888: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 830A188C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A1890: 895F0050  lbz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A1894: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 830A1898: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830A189C: 7EAB3B2E  sthx r21, r11, r7
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), ctx.r[21].u16) };
	// 830A18A0: 807E005C  lwz r3, 0x5c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 830A18A4: 80FE0054  lwz r7, 0x54(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) } as u64;
	// 830A18A8: 80D70018  lwz r6, 0x18(r23)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A18AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A18B0: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A18B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A18B8: 4E800421  bctrl
	ctx.lr = 0x830A18BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A18BC: 897F0051  lbz r11, 0x51(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(81 as u32) ) } as u64;
	// 830A18C0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A18C4: 41820138  beq 0x830a19fc
	if ctx.cr[0].eq {
	pc = 0x830A19FC; continue 'dispatch;
	}
	// 830A18C8: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 830A18CC: 4BF74ABD  bl 0x83016388
	ctx.lr = 0x830A18D0;
	sub_83016388(ctx, base);
	// 830A18D0: 897E0010  lbz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A18D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A18D8: 41820068  beq 0x830a1940
	if ctx.cr[0].eq {
	pc = 0x830A1940; continue 'dispatch;
	}
	// 830A18DC: 807E00A8  lwz r3, 0xa8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) } as u64;
	// 830A18E0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830A18E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A18E8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830A18EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A18F0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A18F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A18F8: 4E800421  bctrl
	ctx.lr = 0x830A18FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A18FC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830A1900: 41800040  blt 0x830a1940
	if ctx.cr[0].lt {
	pc = 0x830A1940; continue 'dispatch;
	}
	// 830A1904: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A1908: 4BF3CA61  bl 0x82fde368
	ctx.lr = 0x830A190C;
	sub_82FDE368(ctx, base);
	// 830A190C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A1910: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A1914: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830A1918: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 830A191C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A1920: 4E800421  bctrl
	ctx.lr = 0x830A1924;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A1924: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 830A1928: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A192C: 807E00A8  lwz r3, 0xa8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) } as u64;
	// 830A1930: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 830A1934: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830A1938: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830A193C: 4BF75C1D  bl 0x83017558
	ctx.lr = 0x830A1940;
	sub_83017558(ctx, base);
	// 830A1940: 817E005C  lwz r11, 0x5c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 830A1944: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A1948: 419A0038  beq cr6, 0x830a1980
	if ctx.cr[6].eq {
	pc = 0x830A1980; continue 'dispatch;
	}
	// 830A194C: 81770004  lwz r11, 4(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A1950: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 830A1954: 81570018  lwz r10, 0x18(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A1958: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830A195C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A1960: 88DF0050  lbz r6, 0x50(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A1964: 7EAB532E  sthx r21, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[21].u16) };
	// 830A1968: 807E005C  lwz r3, 0x5c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 830A196C: 80F70018  lwz r7, 0x18(r23)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A1970: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A1974: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A1978: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A197C: 4E800421  bctrl
	ctx.lr = 0x830A1980;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A1980: 2B0E0000  cmplwi cr6, r14, 0
	ctx.cr[6].compare_u32(ctx.r[14].u32, 0 as u32, &mut ctx.xer);
	// 830A1984: 419A0010  beq cr6, 0x830a1994
	if ctx.cr[6].eq {
	pc = 0x830A1994; continue 'dispatch;
	}
	// 830A1988: 817F013C  lwz r11, 0x13c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(316 as u32) ) } as u64;
	// 830A198C: 9AAB0000  stb r21, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[21].u8 ) };
	// 830A1990: 4800006C  b 0x830a19fc
	pc = 0x830A19FC; continue 'dispatch;
	// 830A1994: 81760028  lwz r11, 0x28(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A1998: 81560020  lwz r10, 0x20(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A199C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A19A0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830A19A4: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 830A19A8: 806B0028  lwz r3, 0x28(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A19AC: 907E00B8  stw r3, 0xb8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(184 as u32), ctx.r[3].u32 ) };
	// 830A19B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A19B4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A19B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A19BC: 4E800421  bctrl
	ctx.lr = 0x830A19C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A19C0: 817E00A8  lwz r11, 0xa8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) } as u64;
	// 830A19C4: 907E01DC  stw r3, 0x1dc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(476 as u32), ctx.r[3].u32 ) };
	// 830A19C8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830A19CC: 809E00B8  lwz r4, 0xb8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(184 as u32) ) } as u64;
	// 830A19D0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A19D4: 816A0028  lwz r11, 0x28(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A19D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A19DC: 4E800421  bctrl
	ctx.lr = 0x830A19E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A19E0: 81760028  lwz r11, 0x28(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A19E4: 81560020  lwz r10, 0x20(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A19E8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A19EC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830A19F0: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 830A19F4: 896B0020  lbz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A19F8: 997E0010  stb r11, 0x10(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 830A19FC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830A1A00: 383F0120  addi r1, r31, 0x120
	ctx.r[1].s64 = ctx.r[31].s64 + 288;
	// 830A1A04: 4810677C  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A1A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A1A08 size=40
    let mut pc: u32 = 0x830A1A08;
    'dispatch: loop {
        match pc {
            0x830A1A08 => {
    //   block [0x830A1A08..0x830A1A30)
	// 830A1A08: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 830A1A0C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A1A10: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A1A14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A1A18: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A1A1C: 4BF3CFD5  bl 0x82fde9f0
	ctx.lr = 0x830A1A20;
	sub_82FDE9F0(ctx, base);
	// 830A1A20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A1A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A1A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A1A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A1A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A1A30 size=48
    let mut pc: u32 = 0x830A1A30;
    'dispatch: loop {
        match pc {
            0x830A1A30 => {
    //   block [0x830A1A30..0x830A1A60)
	// 830A1A30: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 830A1A34: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A1A38: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A1A3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A1A40: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 830A1A44: 808B00D8  lwz r4, 0xd8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(216 as u32) ) } as u64;
	// 830A1A48: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 830A1A4C: 4BF36895  bl 0x82fd82e0
	ctx.lr = 0x830A1A50;
	sub_82FD82E0(ctx, base);
	// 830A1A50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A1A54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A1A58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A1A5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A1A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A1A60 size=40
    let mut pc: u32 = 0x830A1A60;
    'dispatch: loop {
        match pc {
            0x830A1A60 => {
    //   block [0x830A1A60..0x830A1A88)
	// 830A1A60: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 830A1A64: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A1A68: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A1A6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A1A70: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 830A1A74: 4BF32515  bl 0x82fd3f88
	ctx.lr = 0x830A1A78;
	sub_82FD3F88(ctx, base);
	// 830A1A78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A1A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A1A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A1A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A1A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A1A88 size=680
    let mut pc: u32 = 0x830A1A88;
    'dispatch: loop {
        match pc {
            0x830A1A88 => {
    //   block [0x830A1A88..0x830A1D30)
	// 830A1A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A1A8C: 481066DD  bl 0x831a8168
	ctx.lr = 0x830A1A90;
	sub_831A8130(ctx, base);
	// 830A1A90: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A1A94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A1A98: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830A1A9C: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 830A1AA0: 809F020C  lwz r4, 0x20c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(524 as u32) ) } as u64;
	// 830A1AA4: 807F00A8  lwz r3, 0xa8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 830A1AA8: 939F01DC  stw r28, 0x1dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(476 as u32), ctx.r[28].u32 ) };
	// 830A1AAC: 909F00B8  stw r4, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[4].u32 ) };
	// 830A1AB0: 909F00BC  stw r4, 0xbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[4].u32 ) };
	// 830A1AB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A1AB8: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A1ABC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A1AC0: 4E800421  bctrl
	ctx.lr = 0x830A1AC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A1AC4: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 830A1AC8: 9B9F0010  stb r28, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[28].u8 ) };
	// 830A1ACC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A1AD0: 41820014  beq 0x830a1ae4
	if ctx.cr[0].eq {
	pc = 0x830A1AE4; continue 'dispatch;
	}
	// 830A1AD4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A1AD8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A1ADC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A1AE0: 4E800421  bctrl
	ctx.lr = 0x830A1AE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A1AE4: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 830A1AE8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A1AEC: 41820014  beq 0x830a1b00
	if ctx.cr[0].eq {
	pc = 0x830A1B00; continue 'dispatch;
	}
	// 830A1AF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A1AF4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830A1AF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A1AFC: 4E800421  bctrl
	ctx.lr = 0x830A1B00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A1B00: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 830A1B04: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A1B08: 41820014  beq 0x830a1b1c
	if ctx.cr[0].eq {
	pc = 0x830A1B1C; continue 'dispatch;
	}
	// 830A1B0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A1B10: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A1B14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A1B18: 4E800421  bctrl
	ctx.lr = 0x830A1B1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A1B1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A1B20: 4BF75F79  bl 0x83017a98
	ctx.lr = 0x830A1B24;
	sub_83017A98(ctx, base);
	// 830A1B24: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 830A1B28: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A1B2C: 41820018  beq 0x830a1b44
	if ctx.cr[0].eq {
	pc = 0x830A1B44; continue 'dispatch;
	}
	// 830A1B30: 807F00D8  lwz r3, 0xd8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 830A1B34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A1B38: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A1B3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A1B40: 4E800421  bctrl
	ctx.lr = 0x830A1B44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A1B44: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830A1B48: 80FF0034  lwz r7, 0x34(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830A1B4C: 387F0190  addi r3, r31, 0x190
	ctx.r[3].s64 = ctx.r[31].s64 + 400;
	// 830A1B50: 80DF0030  lwz r6, 0x30(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830A1B54: 80BF002C  lwz r5, 0x2c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 830A1B58: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830A1B5C: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 830A1B60: 4BF74C39  bl 0x83016798
	ctx.lr = 0x830A1B64;
	sub_83016798(ctx, base);
	// 830A1B64: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 830A1B68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830A1B6C: 409A0024  bne cr6, 0x830a1b90
	if !ctx.cr[6].eq {
	pc = 0x830A1B90; continue 'dispatch;
	}
	// 830A1B70: 807F00C0  lwz r3, 0xc0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 830A1B74: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830A1B78: 388BCC10  addi r4, r11, -0x33f0
	ctx.r[4].s64 = ctx.r[11].s64 + -13296;
	// 830A1B7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A1B80: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A1B84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A1B88: 4E800421  bctrl
	ctx.lr = 0x830A1B8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A1B8C: 907F0038  stw r3, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[3].u32 ) };
	// 830A1B90: 807F0210  lwz r3, 0x210(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(528 as u32) ) } as u64;
	// 830A1B94: 9BDF000D  stb r30, 0xd(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(13 as u32), ctx.r[30].u8 ) };
	// 830A1B98: 9BDF000E  stb r30, 0xe(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(14 as u32), ctx.r[30].u8 ) };
	// 830A1B9C: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 830A1BA0: 9B9F000F  stb r28, 0xf(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(15 as u32), ctx.r[28].u8 ) };
	// 830A1BA4: 9BDF01D8  stb r30, 0x1d8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(472 as u32), ctx.r[30].u8 ) };
	// 830A1BA8: 9B9F000A  stb r28, 0xa(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(10 as u32), ctx.r[28].u8 ) };
	// 830A1BAC: 9B9F0012  stb r28, 0x12(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(18 as u32), ctx.r[28].u8 ) };
	// 830A1BB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A1BB4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A1BB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A1BBC: 4E800421  bctrl
	ctx.lr = 0x830A1BC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A1BC0: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 830A1BC4: 815F0210  lwz r10, 0x210(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(528 as u32) ) } as u64;
	// 830A1BC8: 3B9F007C  addi r28, r31, 0x7c
	ctx.r[28].s64 = ctx.r[31].s64 + 124;
	// 830A1BCC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 830A1BD0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 830A1BD4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 830A1BD8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830A1BDC: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830A1BE0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A1BE4: 916A0058  stw r11, 0x58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 830A1BE8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830A1BEC: 817F0210  lwz r11, 0x210(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(528 as u32) ) } as u64;
	// 830A1BF0: 895F000B  lbz r10, 0xb(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(11 as u32) ) } as u64;
	// 830A1BF4: 994B0054  stb r10, 0x54(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 830A1BF8: 817F0210  lwz r11, 0x210(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(528 as u32) ) } as u64;
	// 830A1BFC: 815F00B0  lwz r10, 0xb0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 830A1C00: 914B001C  stw r10, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 830A1C04: 893F0009  lbz r9, 9(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(9 as u32) ) } as u64;
	// 830A1C08: 4BF4B5D1  bl 0x82fed1d8
	ctx.lr = 0x830A1C0C;
	sub_82FED1D8(ctx, base);
	// 830A1C0C: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 830A1C10: 408200B4  bne 0x830a1cc4
	if !ctx.cr[0].eq {
	pc = 0x830A1CC4; continue 'dispatch;
	}
	// 830A1C14: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A1C18: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A1C1C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A1C20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A1C24: 4E800421  bctrl
	ctx.lr = 0x830A1C28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A1C28: 83FF00D8  lwz r31, 0xd8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 830A1C2C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A1C30: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A1C34: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A1C38: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A1C3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A1C40: 41820044  beq 0x830a1c84
	if ctx.cr[0].eq {
	pc = 0x830A1C84; continue 'dispatch;
	}
	// 830A1C44: 4E800421  bctrl
	ctx.lr = 0x830A1C48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A1C48: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A1C4C: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 830A1C50: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 830A1C54: 388BECA8  addi r4, r11, -0x1358
	ctx.r[4].s64 = ctx.r[11].s64 + -4952;
	// 830A1C58: 38C0004C  li r6, 0x4c
	ctx.r[6].s64 = 76;
	// 830A1C5C: 38A00233  li r5, 0x233
	ctx.r[5].s64 = 563;
	// 830A1C60: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830A1C64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830A1C68: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830A1C6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A1C70: 4BF75B11  bl 0x83017780
	ctx.lr = 0x830A1C74;
	sub_83017780(ctx, base);
	// 830A1C74: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A1C78: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830A1C7C: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830A1C80: 4810EFA9  bl 0x831b0c28
	ctx.lr = 0x830A1C84;
	sub_831B0C28(ctx, base);
	// 830A1C84: 4E800421  bctrl
	ctx.lr = 0x830A1C88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A1C88: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A1C8C: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 830A1C90: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 830A1C94: 388BECA8  addi r4, r11, -0x1358
	ctx.r[4].s64 = ctx.r[11].s64 + -4952;
	// 830A1C98: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 830A1C9C: 38A00235  li r5, 0x235
	ctx.r[5].s64 = 565;
	// 830A1CA0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830A1CA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830A1CA8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830A1CAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A1CB0: 4BF75AD1  bl 0x83017780
	ctx.lr = 0x830A1CB4;
	sub_83017780(ctx, base);
	// 830A1CB4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A1CB8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830A1CBC: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830A1CC0: 4810EF69  bl 0x831b0c28
	ctx.lr = 0x830A1CC4;
	sub_831B0C28(ctx, base);
	// 830A1CC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830A1CC8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830A1CCC: 4BF4C9DD  bl 0x82fee6a8
	ctx.lr = 0x830A1CD0;
	sub_82FEE6A8(ctx, base);
	// 830A1CD0: 807F00D0  lwz r3, 0xd0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 830A1CD4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A1CD8: 4182001C  beq 0x830a1cf4
	if ctx.cr[0].eq {
	pc = 0x830A1CF4; continue 'dispatch;
	}
	// 830A1CDC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A1CE0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A1CE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A1CE8: 4E800421  bctrl
	ctx.lr = 0x830A1CEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A1CEC: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 830A1CF0: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 830A1CF4: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 830A1CF8: 93DF021C  stw r30, 0x21c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(540 as u32), ctx.r[30].u32 ) };
	// 830A1CFC: 2B0B0020  cmplwi cr6, r11, 0x20
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32 as u32, &mut ctx.xer);
	// 830A1D00: 41980020  blt cr6, 0x830a1d20
	if ctx.cr[6].lt {
	pc = 0x830A1D20; continue 'dispatch;
	}
	// 830A1D04: 807F0220  lwz r3, 0x220(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(544 as u32) ) } as u64;
	// 830A1D08: 4BF586C9  bl 0x82ffa3d0
	ctx.lr = 0x830A1D0C;
	sub_82FFA3D0(ctx, base);
	// 830A1D0C: 807F0224  lwz r3, 0x224(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(548 as u32) ) } as u64;
	// 830A1D10: 4BFCDC29  bl 0x8306f938
	ctx.lr = 0x830A1D14;
	sub_8306F938(ctx, base);
	// 830A1D14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A1D18: 4BF467C9  bl 0x82fe84e0
	ctx.lr = 0x830A1D1C;
	sub_82FE84E0(ctx, base);
	// 830A1D1C: 4800000C  b 0x830a1d28
	pc = 0x830A1D28; continue 'dispatch;
	// 830A1D20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A1D24: 4BF46775  bl 0x82fe8498
	ctx.lr = 0x830A1D28;
	sub_82FE8498(ctx, base);
	// 830A1D28: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 830A1D2C: 4810648C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A1D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A1D30 size=80
    let mut pc: u32 = 0x830A1D30;
    'dispatch: loop {
        match pc {
            0x830A1D30 => {
    //   block [0x830A1D30..0x830A1D80)
	// 830A1D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A1D34: 48106439  bl 0x831a816c
	ctx.lr = 0x830A1D38;
	sub_831A8130(ctx, base);
	// 830A1D38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A1D3C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830A1D40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A1D44: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 830A1D48: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 830A1D4C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 830A1D50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A1D54: 4BF8D75D  bl 0x8302f4b0
	ctx.lr = 0x830A1D58;
	sub_8302F4B0(ctx, base);
	// 830A1D58: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A1D5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A1D60: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 830A1D64: 396BEDB8  addi r11, r11, -0x1248
	ctx.r[11].s64 = ctx.r[11].s64 + -4680;
	// 830A1D68: 93BF001C  stw r29, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 830A1D6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A1D70: B15F0018  sth r10, 0x18(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u16 ) };
	// 830A1D74: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A1D78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A1D7C: 48106440  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A1D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A1D80 size=16
    let mut pc: u32 = 0x830A1D80;
    'dispatch: loop {
        match pc {
            0x830A1D80 => {
    //   block [0x830A1D80..0x830A1D90)
	// 830A1D80: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A1D84: 396BEDB8  addi r11, r11, -0x1248
	ctx.r[11].s64 = ctx.r[11].s64 + -4680;
	// 830A1D88: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A1D8C: 4BF8D78C  b 0x8302f518
	sub_8302F518(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A1D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A1D90 size=88
    let mut pc: u32 = 0x830A1D90;
    'dispatch: loop {
        match pc {
            0x830A1D90 => {
    //   block [0x830A1D90..0x830A1DE8)
	// 830A1D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A1D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A1D98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A1D9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A1DA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A1DA4: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A1DA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A1DAC: 396BEDB8  addi r11, r11, -0x1248
	ctx.r[11].s64 = ctx.r[11].s64 + -4680;
	// 830A1DB0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A1DB4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A1DB8: 4BF8D761  bl 0x8302f518
	ctx.lr = 0x830A1DBC;
	sub_8302F518(ctx, base);
	// 830A1DBC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A1DC0: 4182000C  beq 0x830a1dcc
	if ctx.cr[0].eq {
	pc = 0x830A1DCC; continue 'dispatch;
	}
	// 830A1DC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A1DC8: 4BF36519  bl 0x82fd82e0
	ctx.lr = 0x830A1DCC;
	sub_82FD82E0(ctx, base);
	// 830A1DCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A1DD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A1DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A1DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A1DDC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A1DE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A1DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A1DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A1DE8 size=28
    let mut pc: u32 = 0x830A1DE8;
    'dispatch: loop {
        match pc {
            0x830A1DE8 => {
    //   block [0x830A1DE8..0x830A1E04)
	// 830A1DE8: 54CB073E  clrlwi r11, r6, 0x1c
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x0000000Fu64;
	// 830A1DEC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 830A1DF0: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 830A1DF4: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 830A1DF8: 409A000C  bne cr6, 0x830a1e04
	if !ctx.cr[6].eq {
		sub_830A1E04(ctx, base);
		return;
	}
	// 830A1DFC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830A1E00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A1E04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A1E04 size=28
    let mut pc: u32 = 0x830A1E04;
    'dispatch: loop {
        match pc {
            0x830A1E04 => {
    //   block [0x830A1E04..0x830A1E20)
	// 830A1E04: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 830A1E08: 409A0018  bne cr6, 0x830a1e20
	if !ctx.cr[6].eq {
		sub_830A1E20(ctx, base);
		return;
	}
	// 830A1E0C: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 830A1E10: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 830A1E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830A1E18: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 830A1E1C: 4BFDC0F4  b 0x8307df10
	sub_8307DF10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A1E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A1E20 size=28
    let mut pc: u32 = 0x830A1E20;
    'dispatch: loop {
        match pc {
            0x830A1E20 => {
    //   block [0x830A1E20..0x830A1E3C)
	// 830A1E20: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 830A1E24: 409A0018  bne cr6, 0x830a1e3c
	if !ctx.cr[6].eq {
		sub_830A1E3C(ctx, base);
		return;
	}
	// 830A1E28: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 830A1E2C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 830A1E30: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 830A1E34: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 830A1E38: 4BFDC0D8  b 0x8307df10
	sub_8307DF10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A1E3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A1E3C size=8
    let mut pc: u32 = 0x830A1E3C;
    'dispatch: loop {
        match pc {
            0x830A1E3C => {
    //   block [0x830A1E3C..0x830A1E44)
	// 830A1E3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A1E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A1E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A1E48 size=112
    let mut pc: u32 = 0x830A1E48;
    'dispatch: loop {
        match pc {
            0x830A1E48 => {
    //   block [0x830A1E48..0x830A1EB8)
	// 830A1E48: 546B073E  clrlwi r11, r3, 0x1c
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000000Fu64;
	// 830A1E4C: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 830A1E50: 419A0068  beq cr6, 0x830a1eb8
	if ctx.cr[6].eq {
		sub_830A1EB8(ctx, base);
		return;
	}
	// 830A1E54: 54AA073E  clrlwi r10, r5, 0x1c
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x0000000Fu64;
	// 830A1E58: 2F0A0006  cmpwi cr6, r10, 6
	ctx.cr[6].compare_i32(ctx.r[10].s32, 6, &mut ctx.xer);
	// 830A1E5C: 419A005C  beq cr6, 0x830a1eb8
	if ctx.cr[6].eq {
		sub_830A1EB8(ctx, base);
		return;
	}
	// 830A1E60: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 830A1E64: 409A0014  bne cr6, 0x830a1e78
	if !ctx.cr[6].eq {
	pc = 0x830A1E78; continue 'dispatch;
	}
	// 830A1E68: 2F0A0008  cmpwi cr6, r10, 8
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8, &mut ctx.xer);
	// 830A1E6C: 409A000C  bne cr6, 0x830a1e78
	if !ctx.cr[6].eq {
	pc = 0x830A1E78; continue 'dispatch;
	}
	// 830A1E70: 7F043040  cmplw cr6, r4, r6
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[6].u32, &mut ctx.xer);
	// 830A1E74: 419A0044  beq cr6, 0x830a1eb8
	if ctx.cr[6].eq {
		sub_830A1EB8(ctx, base);
		return;
	}
	// 830A1E78: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 830A1E7C: 409A000C  bne cr6, 0x830a1e88
	if !ctx.cr[6].eq {
	pc = 0x830A1E88; continue 'dispatch;
	}
	// 830A1E80: 2F0A0007  cmpwi cr6, r10, 7
	ctx.cr[6].compare_i32(ctx.r[10].s32, 7, &mut ctx.xer);
	// 830A1E84: 419A0034  beq cr6, 0x830a1eb8
	if ctx.cr[6].eq {
		sub_830A1EB8(ctx, base);
		return;
	}
	// 830A1E88: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 830A1E8C: 409A000C  bne cr6, 0x830a1e98
	if !ctx.cr[6].eq {
	pc = 0x830A1E98; continue 'dispatch;
	}
	// 830A1E90: 2F0A0007  cmpwi cr6, r10, 7
	ctx.cr[6].compare_i32(ctx.r[10].s32, 7, &mut ctx.xer);
	// 830A1E94: 419A0014  beq cr6, 0x830a1ea8
	if ctx.cr[6].eq {
	pc = 0x830A1EA8; continue 'dispatch;
	}
	// 830A1E98: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 830A1E9C: 409A0014  bne cr6, 0x830a1eb0
	if !ctx.cr[6].eq {
	pc = 0x830A1EB0; continue 'dispatch;
	}
	// 830A1EA0: 2F0A0008  cmpwi cr6, r10, 8
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8, &mut ctx.xer);
	// 830A1EA4: 409A000C  bne cr6, 0x830a1eb0
	if !ctx.cr[6].eq {
	pc = 0x830A1EB0; continue 'dispatch;
	}
	// 830A1EA8: 7F043040  cmplw cr6, r4, r6
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[6].u32, &mut ctx.xer);
	// 830A1EAC: 409A000C  bne cr6, 0x830a1eb8
	if !ctx.cr[6].eq {
		sub_830A1EB8(ctx, base);
		return;
	}
	// 830A1EB0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A1EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A1EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A1EB8 size=8
    let mut pc: u32 = 0x830A1EB8;
    'dispatch: loop {
        match pc {
            0x830A1EB8 => {
    //   block [0x830A1EB8..0x830A1EC0)
	// 830A1EB8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830A1EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A1EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A1EC0 size=172
    let mut pc: u32 = 0x830A1EC0;
    'dispatch: loop {
        match pc {
            0x830A1EC0 => {
    //   block [0x830A1EC0..0x830A1F6C)
	// 830A1EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A1EC4: 481062A9  bl 0x831a816c
	ctx.lr = 0x830A1EC8;
	sub_831A8130(ctx, base);
	// 830A1EC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A1ECC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 830A1ED0: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 830A1ED4: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 830A1ED8: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 830A1EDC: 409A005C  bne cr6, 0x830a1f38
	if !ctx.cr[6].eq {
	pc = 0x830A1F38; continue 'dispatch;
	}
	// 830A1EE0: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 830A1EE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830A1EE8: 409A0040  bne cr6, 0x830a1f28
	if !ctx.cr[6].eq {
	pc = 0x830A1F28; continue 'dispatch;
	}
	// 830A1EEC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830A1EF0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A1EF4: 4BFDBDD5  bl 0x8307dcc8
	ctx.lr = 0x830A1EF8;
	sub_8307DCC8(ctx, base);
	// 830A1EF8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A1EFC: 40820024  bne 0x830a1f20
	if !ctx.cr[0].eq {
	pc = 0x830A1F20; continue 'dispatch;
	}
	// 830A1F00: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830A1F04: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A1F08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A1F0C: 4BFDBDBD  bl 0x8307dcc8
	ctx.lr = 0x830A1F10;
	sub_8307DCC8(ctx, base);
	// 830A1F10: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A1F14: 4082000C  bne 0x830a1f20
	if !ctx.cr[0].eq {
	pc = 0x830A1F20; continue 'dispatch;
	}
	// 830A1F18: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A1F1C: 48000048  b 0x830a1f64
	pc = 0x830A1F64; continue 'dispatch;
	// 830A1F20: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830A1F24: 48000040  b 0x830a1f64
	pc = 0x830A1F64; continue 'dispatch;
	// 830A1F28: 80BE0020  lwz r5, 0x20(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A1F2C: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 830A1F30: 4BFFFEB9  bl 0x830a1de8
	ctx.lr = 0x830A1F34;
	sub_830A1DE8(ctx, base);
	// 830A1F34: 48000030  b 0x830a1f64
	pc = 0x830A1F64; continue 'dispatch;
	// 830A1F38: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 830A1F3C: 409A0014  bne cr6, 0x830a1f50
	if !ctx.cr[6].eq {
	pc = 0x830A1F50; continue 'dispatch;
	}
	// 830A1F40: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 830A1F44: 80BF0020  lwz r5, 0x20(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A1F48: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A1F4C: 4BFFFFE0  b 0x830a1f2c
	pc = 0x830A1F2C; continue 'dispatch;
	// 830A1F50: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 830A1F54: 80DE0020  lwz r6, 0x20(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A1F58: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830A1F5C: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830A1F60: 4BFFFEE9  bl 0x830a1e48
	ctx.lr = 0x830A1F64;
	sub_830A1E48(ctx, base);
	// 830A1F64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A1F68: 48106254  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A1F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A1F70 size=8
    let mut pc: u32 = 0x830A1F70;
    'dispatch: loop {
        match pc {
            0x830A1F70 => {
    //   block [0x830A1F70..0x830A1F78)
	// 830A1F70: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A1F74: 8216EDF0  lwz r16, -0x1210(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-4624 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A1F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A1F78 size=104
    let mut pc: u32 = 0x830A1F78;
    'dispatch: loop {
        match pc {
            0x830A1F78 => {
    //   block [0x830A1F78..0x830A1FE0)
	// 830A1F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A1F7C: 481061E1  bl 0x831a815c
	ctx.lr = 0x830A1F80;
	sub_831A8130(ctx, base);
	// 830A1F80: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830A1F84: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A1F88: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A1F8C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 830A1F90: 80DF00E4  lwz r6, 0xe4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 830A1F94: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 830A1F98: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 830A1F9C: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 830A1FA0: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 830A1FA4: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 830A1FA8: 4BF36F89  bl 0x82fd8f30
	ctx.lr = 0x830A1FAC;
	sub_82FD8F30(ctx, base);
	// 830A1FAC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 830A1FB0: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 830A1FB4: 396B6A98  addi r11, r11, 0x6a98
	ctx.r[11].s64 = ctx.r[11].s64 + 27288;
	// 830A1FB8: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 830A1FBC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 830A1FC0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830A1FC4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A1FC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A1FCC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A1FD0: 4BF37401  bl 0x82fd93d0
	ctx.lr = 0x830A1FD4;
	sub_82FD93D0(ctx, base);
	// 830A1FD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A1FD8: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 830A1FDC: 481061D0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A1FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A1FE0 size=40
    let mut pc: u32 = 0x830A1FE0;
    'dispatch: loop {
        match pc {
            0x830A1FE0 => {
    //   block [0x830A1FE0..0x830A2008)
	// 830A1FE0: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830A1FE4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A1FE8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A1FEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A1FF0: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830A1FF4: 4BF36E85  bl 0x82fd8e78
	ctx.lr = 0x830A1FF8;
	sub_82FD8E78(ctx, base);
	// 830A1FF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A1FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A2000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A2004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A2008 size=28
    let mut pc: u32 = 0x830A2008;
    'dispatch: loop {
        match pc {
            0x830A2008 => {
    //   block [0x830A2008..0x830A2024)
	// 830A2008: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A200C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 830A2010: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 830A2014: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830A2018: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830A201C: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830A2020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A2028 size=8
    let mut pc: u32 = 0x830A2028;
    'dispatch: loop {
        match pc {
            0x830A2028 => {
    //   block [0x830A2028..0x830A2030)
	// 830A2028: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A202C: 8216EE78  lwz r16, -0x1188(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-4488 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A2030 size=188
    let mut pc: u32 = 0x830A2030;
    'dispatch: loop {
        match pc {
            0x830A2030 => {
    //   block [0x830A2030..0x830A20EC)
	// 830A2030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A2034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A2038: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A203C: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830A2040: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A2044: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A2048: 907F00A4  stw r3, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[3].u32 ) };
	// 830A204C: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 830A2050: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 830A2054: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 830A2058: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830A205C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830A2060: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 830A2064: 91230014  stw r9, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 830A2068: 3D208217  lis r9, -0x7de9
	ctx.r[9].s64 = -2112421888;
	// 830A206C: 548A073E  clrlwi r10, r4, 0x1c
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x0000000Fu64;
	// 830A2070: 90A30018  stw r5, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[5].u32 ) };
	// 830A2074: 3929EE60  addi r9, r9, -0x11a0
	ctx.r[9].s64 = ctx.r[9].s64 + -4512;
	// 830A2078: 90C3001C  stw r6, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[6].u32 ) };
	// 830A207C: 2F0A0006  cmpwi cr6, r10, 6
	ctx.cr[6].compare_i32(ctx.r[10].s32, 6, &mut ctx.xer);
	// 830A2080: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 830A2084: 419A0054  beq cr6, 0x830a20d8
	if ctx.cr[6].eq {
	pc = 0x830A20D8; continue 'dispatch;
	}
	// 830A2088: 2F0A0007  cmpwi cr6, r10, 7
	ctx.cr[6].compare_i32(ctx.r[10].s32, 7, &mut ctx.xer);
	// 830A208C: 419A004C  beq cr6, 0x830a20d8
	if ctx.cr[6].eq {
	pc = 0x830A20D8; continue 'dispatch;
	}
	// 830A2090: 2F0A0008  cmpwi cr6, r10, 8
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8, &mut ctx.xer);
	// 830A2094: 419A0044  beq cr6, 0x830a20d8
	if ctx.cr[6].eq {
	pc = 0x830A20D8; continue 'dispatch;
	}
	// 830A2098: 3D408217  lis r10, -0x7de9
	ctx.r[10].s64 = -2112421888;
	// 830A209C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 830A20A0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830A20A4: 38EAEE58  addi r7, r10, -0x11a8
	ctx.r[7].s64 = ctx.r[10].s64 + -4520;
	// 830A20A8: 3D408217  lis r10, -0x7de9
	ctx.r[10].s64 = -2112421888;
	// 830A20AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830A20B0: 388AEE20  addi r4, r10, -0x11e0
	ctx.r[4].s64 = ctx.r[10].s64 + -4576;
	// 830A20B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A20B8: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 830A20BC: 38A00051  li r5, 0x51
	ctx.r[5].s64 = 81;
	// 830A20C0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A20C4: 4BFFFEB5  bl 0x830a1f78
	ctx.lr = 0x830A20C8;
	sub_830A1F78(ctx, base);
	// 830A20C8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A20CC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830A20D0: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830A20D4: 4810EB55  bl 0x831b0c28
	ctx.lr = 0x830A20D8;
	sub_831B0C28(ctx, base);
	// 830A20D8: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 830A20DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A20E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A20E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A20E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A20EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A20EC size=40
    let mut pc: u32 = 0x830A20EC;
    'dispatch: loop {
        match pc {
            0x830A20EC => {
    //   block [0x830A20EC..0x830A2114)
	// 830A20EC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830A20F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A20F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A20F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A20FC: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830A2100: 4BFE23C9  bl 0x830844c8
	ctx.lr = 0x830A2104;
	sub_830844C8(ctx, base);
	// 830A2104: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A2108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A210C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A2110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A2118 size=88
    let mut pc: u32 = 0x830A2118;
    'dispatch: loop {
        match pc {
            0x830A2118 => {
    //   block [0x830A2118..0x830A2170)
	// 830A2118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A211C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A2120: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A2124: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A2128: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A212C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A2130: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A2134: 396BEE60  addi r11, r11, -0x11a0
	ctx.r[11].s64 = ctx.r[11].s64 + -4512;
	// 830A2138: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A213C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A2140: 4BFE2389  bl 0x830844c8
	ctx.lr = 0x830A2144;
	sub_830844C8(ctx, base);
	// 830A2144: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A2148: 4182000C  beq 0x830a2154
	if ctx.cr[0].eq {
	pc = 0x830A2154; continue 'dispatch;
	}
	// 830A214C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A2150: 4BF36191  bl 0x82fd82e0
	ctx.lr = 0x830A2154;
	sub_82FD82E0(ctx, base);
	// 830A2154: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A2158: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A215C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A2160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A2164: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A2168: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A216C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A2170 size=172
    let mut pc: u32 = 0x830A2170;
    'dispatch: loop {
        match pc {
            0x830A2170 => {
    //   block [0x830A2170..0x830A221C)
	// 830A2170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A2174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A2178: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A217C: 7F032040  cmplw cr6, r3, r4
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[4].u32, &mut ctx.xer);
	// 830A2180: 419A008C  beq cr6, 0x830a220c
	if ctx.cr[6].eq {
	pc = 0x830A220C; continue 'dispatch;
	}
	// 830A2184: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A2188: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A218C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830A2190: 419A0030  beq cr6, 0x830a21c0
	if ctx.cr[6].eq {
	pc = 0x830A21C0; continue 'dispatch;
	}
	// 830A2194: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A2198: 80E30014  lwz r7, 0x14(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A219C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 830A21A0: 388BA4A4  addi r4, r11, -0x5b5c
	ctx.r[4].s64 = ctx.r[11].s64 + -23388;
	// 830A21A4: 38A000E2  li r5, 0xe2
	ctx.r[5].s64 = 226;
	// 830A21A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A21AC: 4BF2EEAD  bl 0x82fd1058
	ctx.lr = 0x830A21B0;
	sub_82FD1058(ctx, base);
	// 830A21B0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A21B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A21B8: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830A21BC: 4810EA6D  bl 0x831b0c28
	ctx.lr = 0x830A21C0;
	sub_831B0C28(ctx, base);
	// 830A21C0: 2B0B0041  cmplwi cr6, r11, 0x41
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65 as u32, &mut ctx.xer);
	// 830A21C4: 40980018  bge cr6, 0x830a21dc
	if !ctx.cr[6].lt {
	pc = 0x830A21DC; continue 'dispatch;
	}
	// 830A21C8: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A21CC: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830A21D0: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 830A21D4: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830A21D8: 48000034  b 0x830a220c
	pc = 0x830A220C; continue 'dispatch;
	// 830A21DC: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A21E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830A21E4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830A21E8: 40990024  ble cr6, 0x830a220c
	if !ctx.cr[6].gt {
	pc = 0x830A220C; continue 'dispatch;
	}
	// 830A21EC: 81440010  lwz r10, 0x10(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A21F0: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A21F4: 7D4B50AE  lbzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830A21F8: 7D4B49AE  stbx r10, r11, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u8) };
	// 830A21FC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830A2200: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A2204: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830A2208: 4198FFE4  blt cr6, 0x830a21ec
	if ctx.cr[6].lt {
	pc = 0x830A21EC; continue 'dispatch;
	}
	// 830A220C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A2210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A2214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A2218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A2220 size=176
    let mut pc: u32 = 0x830A2220;
    'dispatch: loop {
        match pc {
            0x830A2220 => {
    //   block [0x830A2220..0x830A22D0)
	// 830A2220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A2224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A2228: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A222C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A2230: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A2234: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A2238: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A223C: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 830A2240: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 830A2244: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A2248: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A224C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A2250: 409A0038  bne cr6, 0x830a2288
	if !ctx.cr[6].eq {
	pc = 0x830A2288; continue 'dispatch;
	}
	// 830A2254: 4E800421  bctrl
	ctx.lr = 0x830A2258;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A2258: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A225C: 40820024  bne 0x830a2280
	if !ctx.cr[0].eq {
	pc = 0x830A2280; continue 'dispatch;
	}
	// 830A2260: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A2264: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A2268: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A226C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A2270: 4E800421  bctrl
	ctx.lr = 0x830A2274;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A2274: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A2278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830A227C: 4182003C  beq 0x830a22b8
	if ctx.cr[0].eq {
	pc = 0x830A22B8; continue 'dispatch;
	}
	// 830A2280: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830A2284: 48000034  b 0x830a22b8
	pc = 0x830A22B8; continue 'dispatch;
	// 830A2288: 4E800421  bctrl
	ctx.lr = 0x830A228C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A228C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A2290: 41820024  beq 0x830a22b4
	if ctx.cr[0].eq {
	pc = 0x830A22B4; continue 'dispatch;
	}
	// 830A2294: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A2298: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A229C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A22A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A22A4: 4E800421  bctrl
	ctx.lr = 0x830A22A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A22A8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A22AC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830A22B0: 40820008  bne 0x830a22b8
	if !ctx.cr[0].eq {
	pc = 0x830A22B8; continue 'dispatch;
	}
	// 830A22B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830A22B8: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830A22BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A22C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A22C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A22C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A22CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A22D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A22D0 size=8
    let mut pc: u32 = 0x830A22D0;
    'dispatch: loop {
        match pc {
            0x830A22D0 => {
    //   block [0x830A22D0..0x830A22D8)
	// 830A22D0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A22D4: 8216EF00  lwz r16, -0x1100(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-4352 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A22D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A22D8 size=152
    let mut pc: u32 = 0x830A22D8;
    'dispatch: loop {
        match pc {
            0x830A22D8 => {
    //   block [0x830A22D8..0x830A2370)
	// 830A22D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A22DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A22E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A22E4: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A22E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A22EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830A22F0: 907F0094  stw r3, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[3].u32 ) };
	// 830A22F4: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 830A22F8: 90E30004  stw r7, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 830A22FC: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 830A2300: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830A2304: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830A2308: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 830A230C: 3D408217  lis r10, -0x7de9
	ctx.r[10].s64 = -2112421888;
	// 830A2310: 548B073E  clrlwi r11, r4, 0x1c
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x0000000Fu64;
	// 830A2314: 90A30018  stw r5, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[5].u32 ) };
	// 830A2318: 394AEEE4  addi r10, r10, -0x111c
	ctx.r[10].s64 = ctx.r[10].s64 + -4380;
	// 830A231C: 90C3001C  stw r6, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[6].u32 ) };
	// 830A2320: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 830A2324: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830A2328: 419A0034  beq cr6, 0x830a235c
	if ctx.cr[6].eq {
	pc = 0x830A235C; continue 'dispatch;
	}
	// 830A232C: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 830A2330: 419A002C  beq cr6, 0x830a235c
	if ctx.cr[6].eq {
	pc = 0x830A235C; continue 'dispatch;
	}
	// 830A2334: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A2338: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 830A233C: 388BEEA8  addi r4, r11, -0x1158
	ctx.r[4].s64 = ctx.r[11].s64 + -4440;
	// 830A2340: 38A0005E  li r5, 0x5e
	ctx.r[5].s64 = 94;
	// 830A2344: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830A2348: 4BF2ED11  bl 0x82fd1058
	ctx.lr = 0x830A234C;
	sub_82FD1058(ctx, base);
	// 830A234C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A2350: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830A2354: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830A2358: 4810E8D1  bl 0x831b0c28
	ctx.lr = 0x830A235C;
	sub_831B0C28(ctx, base);
	// 830A235C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A2360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A2364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A2368: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A236C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A2370 size=40
    let mut pc: u32 = 0x830A2370;
    'dispatch: loop {
        match pc {
            0x830A2370 => {
    //   block [0x830A2370..0x830A2398)
	// 830A2370: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A2374: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A2378: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A237C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A2380: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A2384: 4BFE2145  bl 0x830844c8
	ctx.lr = 0x830A2388;
	sub_830844C8(ctx, base);
	// 830A2388: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A238C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A2390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A2394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A2398 size=8
    let mut pc: u32 = 0x830A2398;
    'dispatch: loop {
        match pc {
            0x830A2398 => {
    //   block [0x830A2398..0x830A23A0)
	// 830A2398: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A239C: 8216EF38  lwz r16, -0x10c8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-4296 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A23A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A23A0 size=140
    let mut pc: u32 = 0x830A23A0;
    'dispatch: loop {
        match pc {
            0x830A23A0 => {
    //   block [0x830A23A0..0x830A242C)
	// 830A23A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A23A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A23A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A23AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A23B0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830A23B4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A23B8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A23BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A23C0: 396BEEE4  addi r11, r11, -0x111c
	ctx.r[11].s64 = ctx.r[11].s64 + -4380;
	// 830A23C4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830A23C8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A23CC: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A23D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A23D4: 41820018  beq 0x830a23ec
	if ctx.cr[0].eq {
	pc = 0x830A23EC; continue 'dispatch;
	}
	// 830A23D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A23DC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830A23E0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A23E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A23E8: 4E800421  bctrl
	ctx.lr = 0x830A23EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A23EC: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A23F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A23F4: 41820018  beq 0x830a240c
	if ctx.cr[0].eq {
	pc = 0x830A240C; continue 'dispatch;
	}
	// 830A23F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A23FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830A2400: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A2404: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A2408: 4E800421  bctrl
	ctx.lr = 0x830A240C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A240C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A2410: 4BFE20B9  bl 0x830844c8
	ctx.lr = 0x830A2414;
	sub_830844C8(ctx, base);
	// 830A2414: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830A2418: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A241C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A2420: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A2424: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A2428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A242C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A242C size=40
    let mut pc: u32 = 0x830A242C;
    'dispatch: loop {
        match pc {
            0x830A242C => {
    //   block [0x830A242C..0x830A2454)
	// 830A242C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830A2430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A2434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A2438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A243C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830A2440: 4BFE2089  bl 0x830844c8
	ctx.lr = 0x830A2444;
	sub_830844C8(ctx, base);
	// 830A2444: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A2448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A244C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A2450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A2458 size=168
    let mut pc: u32 = 0x830A2458;
    'dispatch: loop {
        match pc {
            0x830A2458 => {
    //   block [0x830A2458..0x830A2500)
	// 830A2458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A245C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A2460: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A2464: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A2468: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A246C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A2470: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A2474: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A2478: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 830A247C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 830A2480: 409A001C  bne cr6, 0x830a249c
	if !ctx.cr[6].eq {
	pc = 0x830A249C; continue 'dispatch;
	}
	// 830A2484: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A2488: 4BFE20A9  bl 0x83084530
	ctx.lr = 0x830A248C;
	sub_83084530(ctx, base);
	// 830A248C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A2490: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A2494: 4BFFFCDD  bl 0x830a2170
	ctx.lr = 0x830A2498;
	sub_830A2170(ctx, base);
	// 830A2498: 4800003C  b 0x830a24d4
	pc = 0x830A24D4; continue 'dispatch;
	// 830A249C: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 830A24A0: 409A0048  bne cr6, 0x830a24e8
	if !ctx.cr[6].eq {
	pc = 0x830A24E8; continue 'dispatch;
	}
	// 830A24A4: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A24A8: 4BFE2089  bl 0x83084530
	ctx.lr = 0x830A24AC;
	sub_83084530(ctx, base);
	// 830A24AC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A24B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A24B4: 4BFFFCBD  bl 0x830a2170
	ctx.lr = 0x830A24B8;
	sub_830A2170(ctx, base);
	// 830A24B8: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A24BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A24C0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A24C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A24C8: 4E800421  bctrl
	ctx.lr = 0x830A24CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A24CC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A24D0: 41820018  beq 0x830a24e8
	if ctx.cr[0].eq {
	pc = 0x830A24E8; continue 'dispatch;
	}
	// 830A24D4: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A24D8: 4BFE2059  bl 0x83084530
	ctx.lr = 0x830A24DC;
	sub_83084530(ctx, base);
	// 830A24DC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A24E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A24E4: 4BFE1305  bl 0x830837e8
	ctx.lr = 0x830A24E8;
	sub_830837E8(ctx, base);
	// 830A24E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A24EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A24F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A24F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A24F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A24FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A2500 size=172
    let mut pc: u32 = 0x830A2500;
    'dispatch: loop {
        match pc {
            0x830A2500 => {
    //   block [0x830A2500..0x830A25AC)
	// 830A2500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A2504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A2508: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A250C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A2510: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A2514: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A2518: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A251C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A2520: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 830A2524: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 830A2528: 409A0020  bne cr6, 0x830a2548
	if !ctx.cr[6].eq {
	pc = 0x830A2548; continue 'dispatch;
	}
	// 830A252C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A2530: 4BFE20C9  bl 0x830845f8
	ctx.lr = 0x830A2534;
	sub_830845F8(ctx, base);
	// 830A2534: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A2538: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A253C: 4BFFFC35  bl 0x830a2170
	ctx.lr = 0x830A2540;
	sub_830A2170(ctx, base);
	// 830A2540: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A2544: 48000040  b 0x830a2584
	pc = 0x830A2584; continue 'dispatch;
	// 830A2548: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 830A254C: 409A0048  bne cr6, 0x830a2594
	if !ctx.cr[6].eq {
	pc = 0x830A2594; continue 'dispatch;
	}
	// 830A2550: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A2554: 4BFE20A5  bl 0x830845f8
	ctx.lr = 0x830A2558;
	sub_830845F8(ctx, base);
	// 830A2558: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A255C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A2560: 4BFFFC11  bl 0x830a2170
	ctx.lr = 0x830A2564;
	sub_830A2170(ctx, base);
	// 830A2564: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830A2568: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A256C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A2570: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A2574: 4E800421  bctrl
	ctx.lr = 0x830A2578;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A2578: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A257C: 41820018  beq 0x830a2594
	if ctx.cr[0].eq {
	pc = 0x830A2594; continue 'dispatch;
	}
	// 830A2580: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A2584: 4BFE2075  bl 0x830845f8
	ctx.lr = 0x830A2588;
	sub_830845F8(ctx, base);
	// 830A2588: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A258C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A2590: 4BFE1259  bl 0x830837e8
	ctx.lr = 0x830A2594;
	sub_830837E8(ctx, base);
	// 830A2594: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A2598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A259C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A25A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A25A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A25A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A25B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A25B0 size=76
    let mut pc: u32 = 0x830A25B0;
    'dispatch: loop {
        match pc {
            0x830A25B0 => {
    //   block [0x830A25B0..0x830A25FC)
	// 830A25B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A25B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A25B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A25BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A25C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A25C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A25C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A25CC: 4BFFFDD5  bl 0x830a23a0
	ctx.lr = 0x830A25D0;
	sub_830A23A0(ctx, base);
	// 830A25D0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A25D4: 4182000C  beq 0x830a25e0
	if ctx.cr[0].eq {
	pc = 0x830A25E0; continue 'dispatch;
	}
	// 830A25D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A25DC: 4BF35D05  bl 0x82fd82e0
	ctx.lr = 0x830A25E0;
	sub_82FD82E0(ctx, base);
	// 830A25E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A25E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A25E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A25EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A25F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A25F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A25F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A2600 size=32
    let mut pc: u32 = 0x830A2600;
    'dispatch: loop {
        match pc {
            0x830A2600 => {
    //   block [0x830A2600..0x830A2620)
	// 830A2600: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A2604: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830A2608: 409A0018  bne cr6, 0x830a2620
	if !ctx.cr[6].eq {
		sub_830A2620(ctx, base);
		return;
	}
	// 830A260C: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A2610: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A2614: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A2618: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A261C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A2620 size=8
    let mut pc: u32 = 0x830A2620;
    'dispatch: loop {
        match pc {
            0x830A2620 => {
    //   block [0x830A2620..0x830A2628)
	// 830A2620: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830A2624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A2628 size=8
    let mut pc: u32 = 0x830A2628;
    'dispatch: loop {
        match pc {
            0x830A2628 => {
    //   block [0x830A2628..0x830A2630)
	// 830A2628: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A262C: 8216EFC8  lwz r16, -0x1038(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-4152 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A2630 size=156
    let mut pc: u32 = 0x830A2630;
    'dispatch: loop {
        match pc {
            0x830A2630 => {
    //   block [0x830A2630..0x830A26CC)
	// 830A2630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A2634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A2638: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A263C: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A2640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A2644: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830A2648: 907F0094  stw r3, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[3].u32 ) };
	// 830A264C: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 830A2650: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 830A2654: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 830A2658: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830A265C: 90E30004  stw r7, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 830A2660: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830A2664: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 830A2668: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A266C: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 830A2670: 90A30018  stw r5, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[5].u32 ) };
	// 830A2674: 396BEFAC  addi r11, r11, -0x1054
	ctx.r[11].s64 = ctx.r[11].s64 + -4180;
	// 830A2678: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A267C: 419A003C  beq cr6, 0x830a26b8
	if ctx.cr[6].eq {
	pc = 0x830A26B8; continue 'dispatch;
	}
	// 830A2680: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 830A2684: 419A0034  beq cr6, 0x830a26b8
	if ctx.cr[6].eq {
	pc = 0x830A26B8; continue 'dispatch;
	}
	// 830A2688: 2F040003  cmpwi cr6, r4, 3
	ctx.cr[6].compare_i32(ctx.r[4].s32, 3, &mut ctx.xer);
	// 830A268C: 419A002C  beq cr6, 0x830a26b8
	if ctx.cr[6].eq {
	pc = 0x830A26B8; continue 'dispatch;
	}
	// 830A2690: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A2694: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 830A2698: 388BEF70  addi r4, r11, -0x1090
	ctx.r[4].s64 = ctx.r[11].s64 + -4240;
	// 830A269C: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 830A26A0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830A26A4: 4BF2E9B5  bl 0x82fd1058
	ctx.lr = 0x830A26A8;
	sub_82FD1058(ctx, base);
	// 830A26A8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830A26AC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830A26B0: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830A26B4: 4810E575  bl 0x831b0c28
	ctx.lr = 0x830A26B8;
	sub_831B0C28(ctx, base);
	// 830A26B8: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A26BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A26C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A26C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A26C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A26CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A26CC size=40
    let mut pc: u32 = 0x830A26CC;
    'dispatch: loop {
        match pc {
            0x830A26CC => {
    //   block [0x830A26CC..0x830A26F4)
	// 830A26CC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A26D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A26D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A26D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A26DC: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A26E0: 4BFE1DE9  bl 0x830844c8
	ctx.lr = 0x830A26E4;
	sub_830844C8(ctx, base);
	// 830A26E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A26E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A26EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A26F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A26F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A26F8 size=8
    let mut pc: u32 = 0x830A26F8;
    'dispatch: loop {
        match pc {
            0x830A26F8 => {
    //   block [0x830A26F8..0x830A2700)
	// 830A26F8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A26FC: 8216F000  lwz r16, -0x1000(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-4096 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A2700 size=108
    let mut pc: u32 = 0x830A2700;
    'dispatch: loop {
        match pc {
            0x830A2700 => {
    //   block [0x830A2700..0x830A276C)
	// 830A2700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A2704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A2708: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A270C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A2710: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830A2714: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A2718: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A271C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A2720: 396BEFAC  addi r11, r11, -0x1054
	ctx.r[11].s64 = ctx.r[11].s64 + -4180;
	// 830A2724: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830A2728: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A272C: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A2730: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A2734: 41820018  beq 0x830a274c
	if ctx.cr[0].eq {
	pc = 0x830A274C; continue 'dispatch;
	}
	// 830A2738: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A273C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830A2740: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A2744: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A2748: 4E800421  bctrl
	ctx.lr = 0x830A274C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A274C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A2750: 4BFE1D79  bl 0x830844c8
	ctx.lr = 0x830A2754;
	sub_830844C8(ctx, base);
	// 830A2754: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830A2758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A275C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A2760: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A2764: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A2768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A276C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A276C size=40
    let mut pc: u32 = 0x830A276C;
    'dispatch: loop {
        match pc {
            0x830A276C => {
    //   block [0x830A276C..0x830A2794)
	// 830A276C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830A2770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A2774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A2778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A277C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830A2780: 4BFE1D49  bl 0x830844c8
	ctx.lr = 0x830A2784;
	sub_830844C8(ctx, base);
	// 830A2784: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A2788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A278C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A2790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A2798 size=60
    let mut pc: u32 = 0x830A2798;
    'dispatch: loop {
        match pc {
            0x830A2798 => {
    //   block [0x830A2798..0x830A27D4)
	// 830A2798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A279C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A27A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A27A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A27A8: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A27AC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830A27B0: 4BFE1D81  bl 0x83084530
	ctx.lr = 0x830A27B4;
	sub_83084530(ctx, base);
	// 830A27B4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A27B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A27BC: 4BFFF9B5  bl 0x830a2170
	ctx.lr = 0x830A27C0;
	sub_830A2170(ctx, base);
	// 830A27C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A27C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A27C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A27CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A27D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A27D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A27D8 size=60
    let mut pc: u32 = 0x830A27D8;
    'dispatch: loop {
        match pc {
            0x830A27D8 => {
    //   block [0x830A27D8..0x830A2814)
	// 830A27D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A27DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A27E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A27E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A27E8: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 830A27EC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830A27F0: 4BFE1E09  bl 0x830845f8
	ctx.lr = 0x830A27F4;
	sub_830845F8(ctx, base);
	// 830A27F4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A27F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A27FC: 4BFFF975  bl 0x830a2170
	ctx.lr = 0x830A2800;
	sub_830A2170(ctx, base);
	// 830A2800: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A2804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A2808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A280C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A2810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A2818 size=76
    let mut pc: u32 = 0x830A2818;
    'dispatch: loop {
        match pc {
            0x830A2818 => {
    //   block [0x830A2818..0x830A2864)
	// 830A2818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A281C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A2820: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A2824: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A2828: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A282C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A2830: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830A2834: 4BFFFECD  bl 0x830a2700
	ctx.lr = 0x830A2838;
	sub_830A2700(ctx, base);
	// 830A2838: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830A283C: 4182000C  beq 0x830a2848
	if ctx.cr[0].eq {
	pc = 0x830A2848; continue 'dispatch;
	}
	// 830A2840: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A2844: 4BF35A9D  bl 0x82fd82e0
	ctx.lr = 0x830A2848;
	sub_82FD82E0(ctx, base);
	// 830A2848: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830A284C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A2850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A2854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A2858: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A285C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A2860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A2868 size=112
    let mut pc: u32 = 0x830A2868;
    'dispatch: loop {
        match pc {
            0x830A2868 => {
    //   block [0x830A2868..0x830A28D8)
	// 830A2868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A286C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A2870: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A2874: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A2878: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A287C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A2880: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830A2884: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A2888: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A288C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A2890: 4E800421  bctrl
	ctx.lr = 0x830A2894;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A2894: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A2898: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A289C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A28A0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A28A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A28A8: 4E800421  bctrl
	ctx.lr = 0x830A28AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A28AC: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A28B0: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A28B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A28B8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A28BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A28C0: 4E800421  bctrl
	ctx.lr = 0x830A28C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A28C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A28C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A28CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A28D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A28D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A28D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A28D8 size=4
    let mut pc: u32 = 0x830A28D8;
    'dispatch: loop {
        match pc {
            0x830A28D8 => {
    //   block [0x830A28D8..0x830A28DC)
	// 830A28D8: 4BFFFF90  b 0x830a2868
	sub_830A2868(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A28E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A28E0 size=8
    let mut pc: u32 = 0x830A28E0;
    'dispatch: loop {
        match pc {
            0x830A28E0 => {
    //   block [0x830A28E0..0x830A28E8)
	// 830A28E0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A28E4: 8216F058  lwz r16, -0xfa8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-4008 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A28E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A28E8 size=392
    let mut pc: u32 = 0x830A28E8;
    'dispatch: loop {
        match pc {
            0x830A28E8 => {
    //   block [0x830A28E8..0x830A2A70)
	// 830A28E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A28EC: 48105871  bl 0x831a815c
	ctx.lr = 0x830A28F0;
	sub_831A8130(ctx, base);
	// 830A28F0: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 830A28F4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A28F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A28FC: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 830A2900: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830A2904: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 830A2908: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830A290C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A2910: 41820034  beq 0x830a2944
	if ctx.cr[0].eq {
	pc = 0x830A2944; continue 'dispatch;
	}
	// 830A2914: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A2918: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A291C: 41820028  beq 0x830a2944
	if ctx.cr[0].eq {
	pc = 0x830A2944; continue 'dispatch;
	}
	// 830A2920: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 830A2924: 48000008  b 0x830a292c
	pc = 0x830A292C; continue 'dispatch;
	// 830A2928: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 830A292C: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A2930: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A2934: 4082FFF4  bne 0x830a2928
	if !ctx.cr[0].eq {
	pc = 0x830A2928; continue 'dispatch;
	}
	// 830A2938: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 830A293C: 7D7C0E70  srawi r28, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830A2940: 48000008  b 0x830a2948
	pc = 0x830A2948; continue 'dispatch;
	// 830A2944: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830A2948: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A294C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 830A2950: 409A0010  bne cr6, 0x830a2960
	if !ctx.cr[6].eq {
	pc = 0x830A2960; continue 'dispatch;
	}
	// 830A2954: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830A2958: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 830A295C: 48105850  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 830A2960: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A2964: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A2968: 41820018  beq 0x830a2980
	if ctx.cr[0].eq {
	pc = 0x830A2980; continue 'dispatch;
	}
	// 830A296C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830A2970: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A2974: 4BF2E20D  bl 0x82fd0b80
	ctx.lr = 0x830A2978;
	sub_82FD0B80(ctx, base);
	// 830A2978: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A297C: 4BF2F6CD  bl 0x82fd2048
	ctx.lr = 0x830A2980;
	sub_82FD2048(ctx, base);
	// 830A2980: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A2984: 93BF0058  stw r29, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 830A2988: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 830A298C: 7D7CDA14  add r11, r28, r27
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[27].u64;
	// 830A2990: 7F0BC800  cmpw cr6, r11, r25
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[25].s32, &mut ctx.xer);
	// 830A2994: 419900BC  bgt cr6, 0x830a2a50
	if ctx.cr[6].gt {
	pc = 0x830A2A50; continue 'dispatch;
	}
	// 830A2998: A0BF0050  lhz r5, 0x50(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A299C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 830A29A0: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	// 830A29A4: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830A29A8: 40990068  ble cr6, 0x830a2a10
	if !ctx.cr[6].gt {
	pc = 0x830A2A10; continue 'dispatch;
	}
	// 830A29AC: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A29B0: 811E000C  lwz r8, 0xc(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830A29B4: 5789083C  slwi r9, r28, 1
	ctx.r[9].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830A29B8: 7D4AEA14  add r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 830A29BC: 7CFDD050  subf r7, r29, r26
	ctx.r[7].s64 = ctx.r[26].s64 - ctx.r[29].s64;
	// 830A29C0: 394AFFFE  addi r10, r10, -2
	ctx.r[10].s64 = ctx.r[10].s64 + -2;
	// 830A29C4: 3929FFFE  addi r9, r9, -2
	ctx.r[9].s64 = ctx.r[9].s64 + -2;
	// 830A29C8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 830A29CC: 38C6FFFF  addi r6, r6, -1
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	// 830A29D0: 7CA7522E  lhzx r5, r7, r10
	ctx.r[5].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830A29D4: 7C69422E  lhzx r3, r9, r8
	ctx.r[3].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 830A29D8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830A29DC: 7F1B1840  cmplw cr6, r27, r3
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[3].u32, &mut ctx.xer);
	// 830A29E0: 419A0024  beq cr6, 0x830a2a04
	if ctx.cr[6].eq {
	pc = 0x830A2A04; continue 'dispatch;
	}
	// 830A29E4: 887E0000  lbz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A29E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A29EC: 41820024  beq 0x830a2a10
	if ctx.cr[0].eq {
	pc = 0x830A2A10; continue 'dispatch;
	}
	// 830A29F0: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A29F4: A36A0000  lhz r27, 0(r10)
	ctx.r[27].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A29F8: 7C634A2E  lhzx r3, r3, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 830A29FC: 7F03D840  cmplw cr6, r3, r27
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[27].u32, &mut ctx.xer);
	// 830A2A00: 409A0010  bne cr6, 0x830a2a10
	if !ctx.cr[6].eq {
	pc = 0x830A2A10; continue 'dispatch;
	}
	// 830A2A04: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 830A2A08: 419A0050  beq cr6, 0x830a2a58
	if ctx.cr[6].eq {
	pc = 0x830A2A58; continue 'dispatch;
	}
	// 830A2A0C: 4199FFB4  bgt cr6, 0x830a29c0
	if ctx.cr[6].gt {
	pc = 0x830A29C0; continue 'dispatch;
	}
	// 830A2A10: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A2A14: 54A9043E  clrlwi r9, r5, 0x10
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0x0000FFFFu64;
	// 830A2A18: 811E0008  lwz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A2A1C: 7CE95396  divwu r7, r9, r10
	ctx.r[7].u32 = ctx.r[9].u32 / ctx.r[10].u32;
	// 830A2A20: 0CCA0000  twi 6, r10, 0
	// 830A2A24: 7D4751D6  mullw r10, r7, r10
	ctx.r[10].s64 = (ctx.r[7].s32 as i64) * (ctx.r[10].s32 as i64);
	// 830A2A28: 7D4A4850  subf r10, r10, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 830A2A2C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A2A30: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 830A2A34: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 830A2A38: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830A2A3C: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 830A2A40: 40980008  bge cr6, 0x830a2a48
	if !ctx.cr[6].lt {
	pc = 0x830A2A48; continue 'dispatch;
	}
	// 830A2A44: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 830A2A48: 7F0BC800  cmpw cr6, r11, r25
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[25].s32, &mut ctx.xer);
	// 830A2A4C: 4099FF50  ble cr6, 0x830a299c
	if !ctx.cr[6].gt {
	pc = 0x830A299C; continue 'dispatch;
	}
	// 830A2A50: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 830A2A54: 48000008  b 0x830a2a5c
	pc = 0x830A2A5C; continue 'dispatch;
	// 830A2A58: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 830A2A5C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A2A60: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 830A2A64: 4BF3005D  bl 0x82fd2ac0
	ctx.lr = 0x830A2A68;
	sub_82FD2AC0(ctx, base);
	// 830A2A68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A2A6C: 4BFFFEEC  b 0x830a2958
	pc = 0x830A2958; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A2A70 size=40
    let mut pc: u32 = 0x830A2A70;
    'dispatch: loop {
        match pc {
            0x830A2A70 => {
    //   block [0x830A2A70..0x830A2A98)
	// 830A2A70: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 830A2A74: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A2A78: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A2A7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A2A80: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 830A2A84: 4BF303D5  bl 0x82fd2e58
	ctx.lr = 0x830A2A88;
	sub_82FD2E58(ctx, base);
	// 830A2A88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A2A8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A2A90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A2A94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A2A98 size=444
    let mut pc: u32 = 0x830A2A98;
    'dispatch: loop {
        match pc {
            0x830A2A98 => {
    //   block [0x830A2A98..0x830A2C54)
	// 830A2A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A2A9C: 481056D1  bl 0x831a816c
	ctx.lr = 0x830A2AA0;
	sub_831A8130(ctx, base);
	// 830A2AA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A2AA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A2AA8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830A2AAC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A2AB0: 41820034  beq 0x830a2ae4
	if ctx.cr[0].eq {
	pc = 0x830A2AE4; continue 'dispatch;
	}
	// 830A2AB4: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A2AB8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A2ABC: 41820028  beq 0x830a2ae4
	if ctx.cr[0].eq {
	pc = 0x830A2AE4; continue 'dispatch;
	}
	// 830A2AC0: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 830A2AC4: 48000008  b 0x830a2acc
	pc = 0x830A2ACC; continue 'dispatch;
	// 830A2AC8: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 830A2ACC: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A2AD0: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A2AD4: 4082FFF4  bne 0x830a2ac8
	if !ctx.cr[0].eq {
	pc = 0x830A2AC8; continue 'dispatch;
	}
	// 830A2AD8: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 830A2ADC: 7D7E0E70  srawi r30, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830A2AE0: 48000008  b 0x830a2ae8
	pc = 0x830A2AE8; continue 'dispatch;
	// 830A2AE4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830A2AE8: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A2AEC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A2AF0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A2AF4: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830A2AF8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A2AFC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A2B00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A2B04: 4E800421  bctrl
	ctx.lr = 0x830A2B08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A2B08: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A2B0C: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 830A2B10: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A2B14: 41820038  beq 0x830a2b4c
	if ctx.cr[0].eq {
	pc = 0x830A2B4C; continue 'dispatch;
	}
	// 830A2B18: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A2B1C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830A2B20: 4BF2E061  bl 0x82fd0b80
	ctx.lr = 0x830A2B24;
	sub_82FD0B80(ctx, base);
	// 830A2B24: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830A2B28: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A2B2C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830A2B30: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830A2B34: 4BF2E04D  bl 0x82fd0b80
	ctx.lr = 0x830A2B38;
	sub_82FD0B80(ctx, base);
	// 830A2B38: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A2B3C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A2B40: 4BF2F509  bl 0x82fd2048
	ctx.lr = 0x830A2B44;
	sub_82FD2048(ctx, base);
	// 830A2B44: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A2B48: 4BF2F521  bl 0x82fd2068
	ctx.lr = 0x830A2B4C;
	sub_82FD2068(ctx, base);
	// 830A2B4C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830A2B50: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A2B54: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 830A2B58: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830A2B5C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 830A2B60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830A2B64: 40990024  ble cr6, 0x830a2b88
	if !ctx.cr[6].gt {
	pc = 0x830A2B88; continue 'dispatch;
	}
	// 830A2B68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830A2B6C: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A2B70: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830A2B74: 7FCA492E  stwx r30, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[30].u32) };
	// 830A2B78: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 830A2B7C: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A2B80: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830A2B84: 4198FFE8  blt cr6, 0x830a2b6c
	if ctx.cr[6].lt {
	pc = 0x830A2B6C; continue 'dispatch;
	}
	// 830A2B88: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830A2B8C: 419A00B4  beq cr6, 0x830a2c40
	if ctx.cr[6].eq {
	pc = 0x830A2C40; continue 'dispatch;
	}
	// 830A2B90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830A2B94: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 830A2B98: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830A2B9C: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A2BA0: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A2BA4: 0CC70000  twi 6, r7, 0
	// 830A2BA8: 7D48522E  lhzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830A2BAC: 7CCA3B96  divwu r6, r10, r7
	ctx.r[6].u32 = ctx.r[10].u32 / ctx.r[7].u32;
	// 830A2BB0: 7CE639D6  mullw r7, r6, r7
	ctx.r[7].s64 = (ctx.r[6].s32 as i64) * (ctx.r[7].s32 as i64);
	// 830A2BB4: 7D475050  subf r10, r7, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	// 830A2BB8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A2BBC: 7CEA482E  lwzx r7, r10, r9
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 830A2BC0: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 830A2BC4: 40980008  bge cr6, 0x830a2bcc
	if !ctx.cr[6].lt {
	pc = 0x830A2BCC; continue 'dispatch;
	}
	// 830A2BC8: 7D6A492E  stwx r11, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u32) };
	// 830A2BCC: 895F0000  lbz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A2BD0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A2BD4: 4182005C  beq 0x830a2c30
	if ctx.cr[0].eq {
	pc = 0x830A2C30; continue 'dispatch;
	}
	// 830A2BD8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830A2BDC: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 830A2BE0: 409A0010  bne cr6, 0x830a2bf0
	if !ctx.cr[6].eq {
	pc = 0x830A2BF0; continue 'dispatch;
	}
	// 830A2BE4: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830A2BE8: 7D4A422E  lhzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 830A2BEC: 48000008  b 0x830a2bf4
	pc = 0x830A2BF4; continue 'dispatch;
	// 830A2BF0: 7D48EA2E  lhzx r10, r8, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 830A2BF4: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A2BF8: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 830A2BFC: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830A2C00: 7CAA3396  divwu r5, r10, r6
	ctx.r[5].u32 = ctx.r[10].u32 / ctx.r[6].u32;
	// 830A2C04: 0CC60000  twi 6, r6, 0
	// 830A2C08: 7CC531D6  mullw r6, r5, r6
	ctx.r[6].s64 = (ctx.r[5].s32 as i64) * (ctx.r[6].s32 as i64);
	// 830A2C0C: 7D465050  subf r10, r6, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[6].s64;
	// 830A2C10: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830A2C14: 7CCA482E  lwzx r6, r10, r9
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 830A2C18: 7F0B3000  cmpw cr6, r11, r6
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[6].s32, &mut ctx.xer);
	// 830A2C1C: 40980008  bge cr6, 0x830a2c24
	if !ctx.cr[6].lt {
	pc = 0x830A2C24; continue 'dispatch;
	}
	// 830A2C20: 7D6A492E  stwx r11, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u32) };
	// 830A2C24: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 830A2C28: 2F070002  cmpwi cr6, r7, 2
	ctx.cr[6].compare_i32(ctx.r[7].s32, 2, &mut ctx.xer);
	// 830A2C2C: 4198FFB0  blt cr6, 0x830a2bdc
	if ctx.cr[6].lt {
	pc = 0x830A2BDC; continue 'dispatch;
	}
	// 830A2C30: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830A2C34: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 830A2C38: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 830A2C3C: 4082FF5C  bne 0x830a2b98
	if !ctx.cr[0].eq {
	pc = 0x830A2B98; continue 'dispatch;
	}
	// 830A2C40: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A2C44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830A2C48: 4BF2FE79  bl 0x82fd2ac0
	ctx.lr = 0x830A2C4C;
	sub_82FD2AC0(ctx, base);
	// 830A2C4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830A2C50: 4810556C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A2C58 size=8
    let mut pc: u32 = 0x830A2C58;
    'dispatch: loop {
        match pc {
            0x830A2C58 => {
    //   block [0x830A2C58..0x830A2C60)
	// 830A2C58: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A2C5C: 8216F0E4  lwz r16, -0xf1c(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-3868 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A2C60 size=140
    let mut pc: u32 = 0x830A2C60;
    'dispatch: loop {
        match pc {
            0x830A2C60 => {
    //   block [0x830A2C60..0x830A2CEC)
	// 830A2C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A2C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A2C68: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 830A2C6C: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 830A2C70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A2C74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A2C78: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830A2C7C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A2C80: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A2C84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830A2C88: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830A2C8C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830A2C90: 98DE0000  stb r6, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[6].u8 ) };
	// 830A2C94: 90BE0004  stw r5, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 830A2C98: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830A2C9C: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830A2CA0: 90FE0014  stw r7, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 830A2CA4: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830A2CA8: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 830A2CAC: 4BF2DED5  bl 0x82fd0b80
	ctx.lr = 0x830A2CB0;
	sub_82FD0B80(ctx, base);
	// 830A2CB0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830A2CB4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830A2CB8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830A2CBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A2CC0: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830A2CC4: 4BFFFDD5  bl 0x830a2a98
	ctx.lr = 0x830A2CC8;
	sub_830A2A98(ctx, base);
	// 830A2CC8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830A2CCC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830A2CD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A2CD4: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830A2CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A2CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A2CE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A2CE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A2CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2CEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A2CEC size=8
    let mut pc: u32 = 0x830A2CEC;
    'dispatch: loop {
        match pc {
            0x830A2CEC => {
    //   block [0x830A2CEC..0x830A2CF4)
	// 830A2CEC: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A2CF0: 8216F0E4  lwz r16, -0xf1c(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-3868 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2CF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A2CF4 size=24
    let mut pc: u32 = 0x830A2CF4;
    'dispatch: loop {
        match pc {
            0x830A2CF4 => {
    //   block [0x830A2CF4..0x830A2D0C)
	// 830A2CF4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A2CF8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A2CFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A2D00: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A2D04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A2D08: 4810DF21  bl 0x831b0c28
	ctx.lr = 0x830A2D0C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2D14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A2D14 size=36
    let mut pc: u32 = 0x830A2D14;
    'dispatch: loop {
        match pc {
            0x830A2D14 => {
    //   block [0x830A2D14..0x830A2D38)
	// 830A2D14: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830A2D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A2D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A2D20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A2D24: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830A2D28: 4BFFFB41  bl 0x830a2868
	ctx.lr = 0x830A2D2C;
	sub_830A2868(ctx, base);
	// 830A2D2C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830A2D30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830A2D34: 4810DEF5  bl 0x831b0c28
	ctx.lr = 0x830A2D38;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A2D38 size=80
    let mut pc: u32 = 0x830A2D38;
    'dispatch: loop {
        match pc {
            0x830A2D38 => {
    //   block [0x830A2D38..0x830A2D88)
	// 830A2D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A2D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A2D40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A2D44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A2D48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830A2D4C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A2D50: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A2D54: 41820018  beq 0x830a2d6c
	if ctx.cr[0].eq {
	pc = 0x830A2D6C; continue 'dispatch;
	}
	// 830A2D58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A2D5C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830A2D60: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A2D64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A2D68: 4E800421  bctrl
	ctx.lr = 0x830A2D6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A2D6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830A2D70: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A2D74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A2D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A2D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A2D80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A2D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A2D88 size=132
    let mut pc: u32 = 0x830A2D88;
    'dispatch: loop {
        match pc {
            0x830A2D88 => {
    //   block [0x830A2D88..0x830A2E0C)
	// 830A2D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A2D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A2D90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A2D94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A2D98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A2D9C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830A2DA0: 3BEBBC39  addi r31, r11, -0x43c7
	ctx.r[31].s64 = ctx.r[11].s64 + -17351;
	// 830A2DA4: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 830A2DA8: A15F0004  lhz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A2DAC: 556B803E  rotlwi r11, r11, 0x10
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(16)) as u64;
	// 830A2DB0: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 830A2DB4: 895F0006  lbz r10, 6(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 830A2DB8: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830A2DBC: 7D5E5B78  or r30, r10, r11
	ctx.r[30].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 830A2DC0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830A2DC4: 419A0014  beq cr6, 0x830a2dd8
	if ctx.cr[6].eq {
	pc = 0x830A2DD8; continue 'dispatch;
	}
	// 830A2DC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A2DCC: 4BF529BD  bl 0x82ff5788
	ctx.lr = 0x830A2DD0;
	sub_82FF5788(ctx, base);
	// 830A2DD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A2DD4: 4BF3550D  bl 0x82fd82e0
	ctx.lr = 0x830A2DD8;
	sub_82FD82E0(ctx, base);
	// 830A2DD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830A2DDC: 7D6AC670  srawi r10, r11, 0x18
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 24) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 24) as i64;
	// 830A2DE0: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 830A2DE4: 995F0003  stb r10, 3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3 as u32), ctx.r[10].u8 ) };
	// 830A2DE8: 796AC202  rldicl r10, r11, 0x38, 8
	ctx.r[10].u64 = ctx.r[11].u64 & 0x00000000000000FFu64;
	// 830A2DEC: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 830A2DF0: 997F0006  stb r11, 6(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u8 ) };
	// 830A2DF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830A2DF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A2DFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A2E00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A2E04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A2E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A2E10 size=8
    let mut pc: u32 = 0x830A2E10;
    'dispatch: loop {
        match pc {
            0x830A2E10 => {
    //   block [0x830A2E10..0x830A2E18)
	// 830A2E10: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A2E14: 8216F178  lwz r16, -0xe88(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-3720 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A2E18 size=176
    let mut pc: u32 = 0x830A2E18;
    'dispatch: loop {
        match pc {
            0x830A2E18 => {
    //   block [0x830A2E18..0x830A2EC8)
	// 830A2E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A2E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A2E20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830A2E24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830A2E28: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830A2E2C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A2E30: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830A2E34: 3BCBBC3C  addi r30, r11, -0x43c4
	ctx.r[30].s64 = ctx.r[11].s64 + -17348;
	// 830A2E38: 897EFFFD  lbz r11, -3(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(-3 as u32) ) } as u64;
	// 830A2E3C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A2E40: 4082006C  bne 0x830a2eac
	if !ctx.cr[0].eq {
	pc = 0x830A2EAC; continue 'dispatch;
	}
	// 830A2E44: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830A2E48: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830A2E4C: 808BB7EC  lwz r4, -0x4814(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18452 as u32) ) } as u64;
	// 830A2E50: 4BF52989  bl 0x82ff57d8
	ctx.lr = 0x830A2E54;
	sub_82FF57D8(ctx, base);
	// 830A2E54: 897EFFFD  lbz r11, -3(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(-3 as u32) ) } as u64;
	// 830A2E58: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A2E5C: 40820048  bne 0x830a2ea4
	if !ctx.cr[0].eq {
	pc = 0x830A2EA4; continue 'dispatch;
	}
	// 830A2E60: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 830A2E64: 4BF353E5  bl 0x82fd8248
	ctx.lr = 0x830A2E68;
	sub_82FD8248(ctx, base);
	// 830A2E68: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830A2E6C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A2E70: 41820010  beq 0x830a2e80
	if ctx.cr[0].eq {
	pc = 0x830A2E80; continue 'dispatch;
	}
	// 830A2E74: 4BF528D5  bl 0x82ff5748
	ctx.lr = 0x830A2E78;
	sub_82FF5748(ctx, base);
	// 830A2E78: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830A2E7C: 48000008  b 0x830a2e84
	pc = 0x830A2E84; continue 'dispatch;
	// 830A2E80: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830A2E84: 3D60830A  lis r11, -0x7cf6
	ctx.r[11].s64 = -2096496640;
	// 830A2E88: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830A2E8C: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 830A2E90: 388B2D88  addi r4, r11, 0x2d88
	ctx.r[4].s64 = ctx.r[11].s64 + 11656;
	// 830A2E94: 386ABC40  addi r3, r10, -0x43c0
	ctx.r[3].s64 = ctx.r[10].s64 + -17344;
	// 830A2E98: 4BF54CA1  bl 0x82ff7b38
	ctx.lr = 0x830A2E9C;
	sub_82FF7B38(ctx, base);
	// 830A2E9C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830A2EA0: 997EFFFD  stb r11, -3(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(-3 as u32), ctx.r[11].u8 ) };
	// 830A2EA4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830A2EA8: 4BF52969  bl 0x82ff5810
	ctx.lr = 0x830A2EAC;
	sub_82FF5810(ctx, base);
	// 830A2EAC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A2EB0: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830A2EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A2EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A2EBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830A2EC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830A2EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A2EC8 size=40
    let mut pc: u32 = 0x830A2EC8;
    'dispatch: loop {
        match pc {
            0x830A2EC8 => {
    //   block [0x830A2EC8..0x830A2EF0)
	// 830A2EC8: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830A2ECC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A2ED0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A2ED4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A2ED8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830A2EDC: 4BF52935  bl 0x82ff5810
	ctx.lr = 0x830A2EE0;
	sub_82FF5810(ctx, base);
	// 830A2EE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A2EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A2EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A2EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A2EF0 size=40
    let mut pc: u32 = 0x830A2EF0;
    'dispatch: loop {
        match pc {
            0x830A2EF0 => {
    //   block [0x830A2EF0..0x830A2F18)
	// 830A2EF0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830A2EF4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A2EF8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A2EFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A2F00: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830A2F04: 4BF353DD  bl 0x82fd82e0
	ctx.lr = 0x830A2F08;
	sub_82FD82E0(ctx, base);
	// 830A2F08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A2F0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A2F10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A2F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A2F18 size=8
    let mut pc: u32 = 0x830A2F18;
    'dispatch: loop {
        match pc {
            0x830A2F18 => {
    //   block [0x830A2F18..0x830A2F20)
	// 830A2F18: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A2F1C: 8216F1E8  lwz r16, -0xe18(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-3608 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A2F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A2F20 size=460
    let mut pc: u32 = 0x830A2F20;
    'dispatch: loop {
        match pc {
            0x830A2F20 => {
    //   block [0x830A2F20..0x830A30EC)
	// 830A2F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A2F24: 48105241  bl 0x831a8164
	ctx.lr = 0x830A2F28;
	sub_831A8130(ctx, base);
	// 830A2F28: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830A2F2C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A2F30: 3F808339  lis r28, -0x7cc7
	ctx.r[28].s64 = -2093416448;
	// 830A2F34: 897CBC38  lbz r11, -0x43c8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(-17352 as u32) ) } as u64;
	// 830A2F38: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A2F3C: 408201A8  bne 0x830a30e4
	if !ctx.cr[0].eq {
	pc = 0x830A30E4; continue 'dispatch;
	}
	// 830A2F40: 4BFFFED9  bl 0x830a2e18
	ctx.lr = 0x830A2F44;
	sub_830A2E18(ctx, base);
	// 830A2F44: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830A2F48: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830A2F4C: 4BF5288D  bl 0x82ff57d8
	ctx.lr = 0x830A2F50;
	sub_82FF57D8(ctx, base);
	// 830A2F50: 897CBC38  lbz r11, -0x43c8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(-17352 as u32) ) } as u64;
	// 830A2F54: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A2F58: 40820184  bne 0x830a30dc
	if !ctx.cr[0].eq {
	pc = 0x830A30DC; continue 'dispatch;
	}
	// 830A2F5C: 4800DB55  bl 0x830b0ab0
	ctx.lr = 0x830A2F60;
	sub_830B0AB0(ctx, base);
	// 830A2F60: 4800D829  bl 0x830b0788
	ctx.lr = 0x830A2F64;
	sub_830B0788(ctx, base);
	// 830A2F64: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830A2F68: 3BCBF118  addi r30, r11, -0xee8
	ctx.r[30].s64 = ctx.r[11].s64 + -3816;
	// 830A2F6C: 4800DB45  bl 0x830b0ab0
	ctx.lr = 0x830A2F70;
	sub_830B0AB0(ctx, base);
	// 830A2F70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A2F74: 4800D215  bl 0x830b0188
	ctx.lr = 0x830A2F78;
	sub_830B0188(ctx, base);
	// 830A2F78: 3BBE0008  addi r29, r30, 8
	ctx.r[29].s64 = ctx.r[30].s64 + 8;
	// 830A2F7C: 4800DB35  bl 0x830b0ab0
	ctx.lr = 0x830A2F80;
	sub_830B0AB0(ctx, base);
	// 830A2F80: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A2F84: 4800D205  bl 0x830b0188
	ctx.lr = 0x830A2F88;
	sub_830B0188(ctx, base);
	// 830A2F88: 3BBE0014  addi r29, r30, 0x14
	ctx.r[29].s64 = ctx.r[30].s64 + 20;
	// 830A2F8C: 4800DB25  bl 0x830b0ab0
	ctx.lr = 0x830A2F90;
	sub_830B0AB0(ctx, base);
	// 830A2F90: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A2F94: 4800D1F5  bl 0x830b0188
	ctx.lr = 0x830A2F98;
	sub_830B0188(ctx, base);
	// 830A2F98: 3BBE0024  addi r29, r30, 0x24
	ctx.r[29].s64 = ctx.r[30].s64 + 36;
	// 830A2F9C: 4800DB15  bl 0x830b0ab0
	ctx.lr = 0x830A2FA0;
	sub_830B0AB0(ctx, base);
	// 830A2FA0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A2FA4: 4800D1E5  bl 0x830b0188
	ctx.lr = 0x830A2FA8;
	sub_830B0188(ctx, base);
	// 830A2FA8: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 830A2FAC: 4BF3529D  bl 0x82fd8248
	ctx.lr = 0x830A2FB0;
	sub_82FD8248(ctx, base);
	// 830A2FB0: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830A2FB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A2FB8: 41820010  beq 0x830a2fc8
	if ctx.cr[0].eq {
	pc = 0x830A2FC8; continue 'dispatch;
	}
	// 830A2FBC: 4800DCED  bl 0x830b0ca8
	ctx.lr = 0x830A2FC0;
	sub_830B0CA8(ctx, base);
	// 830A2FC0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A2FC4: 48000008  b 0x830a2fcc
	pc = 0x830A2FCC; continue 'dispatch;
	// 830A2FC8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A2FCC: 4800DAE5  bl 0x830b0ab0
	ctx.lr = 0x830A2FD0;
	sub_830B0AB0(ctx, base);
	// 830A2FD0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A2FD4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A2FD8: 4800D599  bl 0x830b0570
	ctx.lr = 0x830A2FDC;
	sub_830B0570(ctx, base);
	// 830A2FDC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A2FE0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A2FE4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A2FE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A2FEC: 4E800421  bctrl
	ctx.lr = 0x830A2FF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A2FF0: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 830A2FF4: 4BF35255  bl 0x82fd8248
	ctx.lr = 0x830A2FF8;
	sub_82FD8248(ctx, base);
	// 830A2FF8: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830A2FFC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A3000: 41820010  beq 0x830a3010
	if ctx.cr[0].eq {
	pc = 0x830A3010; continue 'dispatch;
	}
	// 830A3004: 4800CD85  bl 0x830afd88
	ctx.lr = 0x830A3008;
	sub_830AFD88(ctx, base);
	// 830A3008: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A300C: 48000008  b 0x830a3014
	pc = 0x830A3014; continue 'dispatch;
	// 830A3010: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A3014: 3B7E0008  addi r27, r30, 8
	ctx.r[27].s64 = ctx.r[30].s64 + 8;
	// 830A3018: 4800DA99  bl 0x830b0ab0
	ctx.lr = 0x830A301C;
	sub_830B0AB0(ctx, base);
	// 830A301C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830A3020: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A3024: 4800D54D  bl 0x830b0570
	ctx.lr = 0x830A3028;
	sub_830B0570(ctx, base);
	// 830A3028: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A302C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A3030: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A3034: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A3038: 4E800421  bctrl
	ctx.lr = 0x830A303C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A303C: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 830A3040: 4BF35209  bl 0x82fd8248
	ctx.lr = 0x830A3044;
	sub_82FD8248(ctx, base);
	// 830A3044: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830A3048: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A304C: 41820010  beq 0x830a305c
	if ctx.cr[0].eq {
	pc = 0x830A305C; continue 'dispatch;
	}
	// 830A3050: 4800C8B1  bl 0x830af900
	ctx.lr = 0x830A3054;
	sub_830AF900(ctx, base);
	// 830A3054: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A3058: 48000008  b 0x830a3060
	pc = 0x830A3060; continue 'dispatch;
	// 830A305C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A3060: 3B7E0014  addi r27, r30, 0x14
	ctx.r[27].s64 = ctx.r[30].s64 + 20;
	// 830A3064: 4800DA4D  bl 0x830b0ab0
	ctx.lr = 0x830A3068;
	sub_830B0AB0(ctx, base);
	// 830A3068: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830A306C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A3070: 4800D501  bl 0x830b0570
	ctx.lr = 0x830A3074;
	sub_830B0570(ctx, base);
	// 830A3074: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A3078: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A307C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A3080: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A3084: 4E800421  bctrl
	ctx.lr = 0x830A3088;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A3088: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 830A308C: 4BF351BD  bl 0x82fd8248
	ctx.lr = 0x830A3090;
	sub_82FD8248(ctx, base);
	// 830A3090: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830A3094: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A3098: 41820010  beq 0x830a30a8
	if ctx.cr[0].eq {
	pc = 0x830A30A8; continue 'dispatch;
	}
	// 830A309C: 4800C5F5  bl 0x830af690
	ctx.lr = 0x830A30A0;
	sub_830AF690(ctx, base);
	// 830A30A0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A30A4: 48000008  b 0x830a30ac
	pc = 0x830A30AC; continue 'dispatch;
	// 830A30A8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A30AC: 3BDE0024  addi r30, r30, 0x24
	ctx.r[30].s64 = ctx.r[30].s64 + 36;
	// 830A30B0: 4800DA01  bl 0x830b0ab0
	ctx.lr = 0x830A30B4;
	sub_830B0AB0(ctx, base);
	// 830A30B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830A30B8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830A30BC: 4800D4B5  bl 0x830b0570
	ctx.lr = 0x830A30C0;
	sub_830B0570(ctx, base);
	// 830A30C0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A30C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A30C8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A30CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830A30D0: 4E800421  bctrl
	ctx.lr = 0x830A30D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830A30D4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830A30D8: 997CBC38  stb r11, -0x43c8(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(-17352 as u32), ctx.r[11].u8 ) };
	// 830A30DC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830A30E0: 4BF52731  bl 0x82ff5810
	ctx.lr = 0x830A30E4;
	sub_82FF5810(ctx, base);
	// 830A30E4: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 830A30E8: 481050CC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A30EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A30EC size=40
    let mut pc: u32 = 0x830A30EC;
    'dispatch: loop {
        match pc {
            0x830A30EC => {
    //   block [0x830A30EC..0x830A3114)
	// 830A30EC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830A30F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A30F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A30F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A30FC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830A3100: 4BF52711  bl 0x82ff5810
	ctx.lr = 0x830A3104;
	sub_82FF5810(ctx, base);
	// 830A3104: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A3108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A310C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3114(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3114 size=40
    let mut pc: u32 = 0x830A3114;
    'dispatch: loop {
        match pc {
            0x830A3114 => {
    //   block [0x830A3114..0x830A313C)
	// 830A3114: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830A3118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A311C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3120: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3124: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830A3128: 4BF351B9  bl 0x82fd82e0
	ctx.lr = 0x830A312C;
	sub_82FD82E0(ctx, base);
	// 830A312C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A3130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A3134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A313C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A313C size=40
    let mut pc: u32 = 0x830A313C;
    'dispatch: loop {
        match pc {
            0x830A313C => {
    //   block [0x830A313C..0x830A3164)
	// 830A313C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830A3140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3148: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A314C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830A3150: 4BF35191  bl 0x82fd82e0
	ctx.lr = 0x830A3154;
	sub_82FD82E0(ctx, base);
	// 830A3154: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A3158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A315C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3164(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3164 size=40
    let mut pc: u32 = 0x830A3164;
    'dispatch: loop {
        match pc {
            0x830A3164 => {
    //   block [0x830A3164..0x830A318C)
	// 830A3164: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830A3168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A316C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3170: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3174: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830A3178: 4BF35169  bl 0x82fd82e0
	ctx.lr = 0x830A317C;
	sub_82FD82E0(ctx, base);
	// 830A317C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A3180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A3184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A318C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A318C size=40
    let mut pc: u32 = 0x830A318C;
    'dispatch: loop {
        match pc {
            0x830A318C => {
    //   block [0x830A318C..0x830A31B4)
	// 830A318C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830A3190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A319C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830A31A0: 4BF35141  bl 0x82fd82e0
	ctx.lr = 0x830A31A4;
	sub_82FD82E0(ctx, base);
	// 830A31A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A31A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A31AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A31B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A31B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A31B8 size=8
    let mut pc: u32 = 0x830A31B8;
    'dispatch: loop {
        match pc {
            0x830A31B8 => {
    //   block [0x830A31B8..0x830A31C0)
	// 830A31B8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A31BC: 8216F270  lwz r16, -0xd90(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-3472 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A31C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A31C0 size=172
    let mut pc: u32 = 0x830A31C0;
    'dispatch: loop {
        match pc {
            0x830A31C0 => {
    //   block [0x830A31C0..0x830A326C)
	// 830A31C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A31C4: 48104FA1  bl 0x831a8164
	ctx.lr = 0x830A31C8;
	sub_831A8130(ctx, base);
	// 830A31C8: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830A31CC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A31D0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 830A31D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A31D8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830A31DC: 937F00AC  stw r27, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[27].u32 ) };
	// 830A31E0: 4BF350B9  bl 0x82fd8298
	ctx.lr = 0x830A31E4;
	sub_82FD8298(ctx, base);
	// 830A31E4: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830A31E8: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 830A31EC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A31F0: 4182002C  beq 0x830a321c
	if ctx.cr[0].eq {
	pc = 0x830A321C; continue 'dispatch;
	}
	// 830A31F4: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 830A31F8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830A31FC: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 830A3200: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830A3204: 4BFA95ED  bl 0x8304c7f0
	ctx.lr = 0x830A3208;
	sub_8304C7F0(ctx, base);
	// 830A3208: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830A320C: 394B2990  addi r10, r11, 0x2990
	ctx.r[10].s64 = ctx.r[11].s64 + 10640;
	// 830A3210: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 830A3214: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830A3218: 48000008  b 0x830a3220
	pc = 0x830A3220; continue 'dispatch;
	// 830A321C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 830A3220: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830A3224: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 830A3228: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830A322C: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 830A3230: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 830A3234: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 830A3238: 93BE0014  stw r29, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 830A323C: 93BE0018  stw r29, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 830A3240: 93BE001C  stw r29, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 830A3244: 93BE0020  stw r29, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 830A3248: 93BE0024  stw r29, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[29].u32 ) };
	// 830A324C: 93BE0028  stw r29, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 830A3250: 93BE002C  stw r29, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 830A3254: 93BE0030  stw r29, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 830A3258: 93BE0034  stw r29, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 830A325C: 93BE0038  stw r29, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 830A3260: 937E003C  stw r27, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[27].u32 ) };
	// 830A3264: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 830A3268: 48104F4C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A326C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A326C size=44
    let mut pc: u32 = 0x830A326C;
    'dispatch: loop {
        match pc {
            0x830A326C => {
    //   block [0x830A326C..0x830A3298)
	// 830A326C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830A3270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A327C: 809F00AC  lwz r4, 0xac(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 830A3280: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A3284: 4BF3505D  bl 0x82fd82e0
	ctx.lr = 0x830A3288;
	sub_82FD82E0(ctx, base);
	// 830A3288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A328C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A3290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A3298 size=8
    let mut pc: u32 = 0x830A3298;
    'dispatch: loop {
        match pc {
            0x830A3298 => {
    //   block [0x830A3298..0x830A32A0)
	// 830A3298: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A329C: 8216F2A8  lwz r16, -0xd58(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-3416 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A32A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A32A0 size=136
    let mut pc: u32 = 0x830A32A0;
    'dispatch: loop {
        match pc {
            0x830A32A0 => {
    //   block [0x830A32A0..0x830A3328)
	// 830A32A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A32A4: 48104EC5  bl 0x831a8168
	ctx.lr = 0x830A32A8;
	sub_831A8130(ctx, base);
	// 830A32A8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A32AC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A32B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A32B4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830A32B8: 57BC043E  clrlwi r28, r29, 0x10
	ctx.r[28].u64 = ctx.r[29].u32 as u64 & 0x0000FFFFu64;
	// 830A32BC: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830A32C0: 2B1C0007  cmplwi cr6, r28, 7
	ctx.cr[6].compare_u32(ctx.r[28].u32, 7 as u32, &mut ctx.xer);
	// 830A32C4: 409A0010  bne cr6, 0x830a32d4
	if !ctx.cr[6].eq {
	pc = 0x830A32D4; continue 'dispatch;
	}
	// 830A32C8: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830A32CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A32D0: 40820050  bne 0x830a3320
	if !ctx.cr[0].eq {
	pc = 0x830A3320; continue 'dispatch;
	}
	// 830A32D4: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 830A32D8: 809E003C  lwz r4, 0x3c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A32DC: 4BF34FBD  bl 0x82fd8298
	ctx.lr = 0x830A32E0;
	sub_82FD8298(ctx, base);
	// 830A32E0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A32E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A32E8: 41820018  beq 0x830a3300
	if ctx.cr[0].eq {
	pc = 0x830A3300; continue 'dispatch;
	}
	// 830A32EC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A32F0: 80BE003C  lwz r5, 0x3c(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A32F4: 480021D5  bl 0x830a54c8
	ctx.lr = 0x830A32F8;
	sub_830A54C8(ctx, base);
	// 830A32F8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A32FC: 48000008  b 0x830a3304
	pc = 0x830A3304; continue 'dispatch;
	// 830A3300: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A3304: 2B1C0007  cmplwi cr6, r28, 7
	ctx.cr[6].compare_u32(ctx.r[28].u32, 7 as u32, &mut ctx.xer);
	// 830A3308: 409A0008  bne cr6, 0x830a3310
	if !ctx.cr[6].eq {
	pc = 0x830A3310; continue 'dispatch;
	}
	// 830A330C: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 830A3310: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A3314: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A3318: 4BF97E39  bl 0x8303b150
	ctx.lr = 0x830A331C;
	sub_8303B150(ctx, base);
	// 830A331C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A3320: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A3324: 48104E94  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3328 size=48
    let mut pc: u32 = 0x830A3328;
    'dispatch: loop {
        match pc {
            0x830A3328 => {
    //   block [0x830A3328..0x830A3358)
	// 830A3328: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830A332C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3330: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830A3334: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3338: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830A333C: 808B003C  lwz r4, 0x3c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A3340: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830A3344: 4BF34F9D  bl 0x82fd82e0
	ctx.lr = 0x830A3348;
	sub_82FD82E0(ctx, base);
	// 830A3348: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830A334C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830A3350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830A3354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830A3358 size=8
    let mut pc: u32 = 0x830A3358;
    'dispatch: loop {
        match pc {
            0x830A3358 => {
    //   block [0x830A3358..0x830A3360)
	// 830A3358: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830A335C: 8216F2F0  lwz r16, -0xd10(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-3344 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830A3360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830A3360 size=112
    let mut pc: u32 = 0x830A3360;
    'dispatch: loop {
        match pc {
            0x830A3360 => {
    //   block [0x830A3360..0x830A33D0)
	// 830A3360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830A3364: 48104E05  bl 0x831a8168
	ctx.lr = 0x830A3368;
	sub_831A8130(ctx, base);
	// 830A3368: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830A336C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830A3370: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830A3374: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 830A3378: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830A337C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830A3380: 809E003C  lwz r4, 0x3c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A3384: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830A3388: 4BF34F11  bl 0x82fd8298
	ctx.lr = 0x830A338C;
	sub_82FD8298(ctx, base);
	// 830A338C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830A3390: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830A3394: 41820020  beq 0x830a33b4
	if ctx.cr[0].eq {
	pc = 0x830A33B4; continue 'dispatch;
	}
	// 830A3398: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830A339C: 80FE003C  lwz r7, 0x3c(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830A33A0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830A33A4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A33A8: 4800DE81  bl 0x830b1228
	ctx.lr = 0x830A33AC;
	sub_830B1228(ctx, base);
	// 830A33AC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830A33B0: 48000008  b 0x830a33b8
	pc = 0x830A33B8; continue 'dispatch;
	// 830A33B4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830A33B8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830A33BC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830A33C0: 4BF97D91  bl 0x8303b150
	ctx.lr = 0x830A33C4;
	sub_8303B150(ctx, base);
	// 830A33C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830A33C8: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830A33CC: 48104DEC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


